use eframe::{
    egui::{
        self, CollapsingHeader, Frame, RichText, ScrollArea, SidePanel, Slider, Style, TextEdit,
        TextStyle, Visuals,
    },
    epaint::{Color32, Vec2},
    App,
};
use egui_plot::{Legend, Line, Plot, PlotPoints};
use exmex::{Express, FlatEx};

mod web;

#[cfg(target_arch = "wasm32")]
pub use web::start_web;

pub const EULER: &'static str = "2.7182818284590452353602874713527";

const COLORS: &'static [Color32; 18] = &[
    Color32::RED,
    Color32::GREEN,
    Color32::YELLOW,
    Color32::BLUE,
    Color32::BROWN,
    Color32::GOLD,
    Color32::GRAY,
    Color32::WHITE,
    Color32::LIGHT_YELLOW,
    Color32::LIGHT_GREEN,
    Color32::LIGHT_BLUE,
    Color32::LIGHT_GRAY,
    Color32::LIGHT_RED,
    Color32::DARK_GRAY,
    Color32::DARK_RED,
    Color32::KHAKI,
    Color32::DARK_GREEN,
    Color32::DARK_BLUE,
];

#[derive(Clone, Debug)]
pub struct Grapher {
    data: Vec<FunctionEntry>,
    error: Option<String>,
    points: usize,
}

impl Grapher {
    pub fn new() -> Self {
        let mut data = Vec::new();

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let error: Option<String> = web::get_data_from_url(&mut data);
            } else {
                let error: Option<String> = None;
            }
        }

        if data.is_empty() {
            data.push(FunctionEntry::new());
        }

        Self {
            data,
            error,
            points: 500,
        }
    }

    fn side_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("left_panel").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(6.0);
                ui.heading("Grapher");
                ui.small("© 2022 Grant Handy");

                ui.separator();

                let mut outer_changed = false;

                ui.horizontal_top(|ui| {
                    if self.data.len() < 18 && ui.button("Add").clicked() {
                        self.data.push(FunctionEntry::new());
                        outer_changed = true;
                    }

                    if self.data.len() > 1 && ui.button("Delete").clicked() {
                        self.data.pop();
                        outer_changed = true;
                    }
                });

                ui.add_space(4.5);

                for (n, entry) in self.data.iter_mut().enumerate() {
                    let mut inner_changed = false;

                    let hint_text = match n {
                        0 => "x^2",
                        1 => "sin(x)",
                        2 => "x+2",
                        3 => "x*3",
                        4 => "abs(x)",
                        5 => "cos(x)",
                        // most people won't go past 5 so i'll be lazy
                        _ => "",
                    };

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(" ").strong().background_color(COLORS[n]));

                        if ui.add(TextEdit::singleline(&mut entry.text).hint_text(hint_text)).changed() {
                            if entry.text != "" {
                                inner_changed = true;
                            } else {
                                entry.func = None;
                            }

                            outer_changed = true;
                        }
                    });

                    if inner_changed {
                        self.error = None;

                        // for nathan
                        entry.func = match exmex::parse::<f64>(&entry.text.replace("e", EULER)) {
                            Ok(func) => Some(func),
                            Err(e) => {
                                self.error = Some(e.to_string());
                                continue;
                            }
                        };
                    }
                }

                #[cfg(target_arch = "wasm32")]
                if outer_changed {
                    web::update_url(&self.data);
                }

                ui.separator();
                ui.label("Grapher is a free and open source graphing calculator available online. Add functions on the left and they'll appear on the right in the graph.");
                ui.label("Hold control and scroll to zoom and drag to move around the graph.");
                ui.hyperlink_to("Source Code ", "https://github.com/grantshandy/grapher");
                #[cfg(not(target_arch = "wasm32"))]
                ui.hyperlink_to("View Graph Online", {
                    let mut base_url = "https://grantshandy.github.io/grapher/".to_string();
                    base_url.push_str(&web::url_string_from_data(&self.data));

                    base_url
                });
                #[cfg(target_arch = "wasm32")]
                ui.hyperlink_to("Download for Desktop", "https://github.com/grantshandy/grapher/releases");
                ui.separator();

                CollapsingHeader::new("Settings").show(ui, |ui| {
                    ui.add(Slider::new(&mut self.points, 10..=1000).text("Resolution"));
                    ui.label("Set to a lower resolution for better performance and a higher resolution for more accuracy. It's also pretty funny if you bring it down ridiculously low.");
                });
            });
        });
    }

    fn graph(&mut self, ctx: &egui::Context) {
        let mut lines: Vec<Line> = Vec::new();

        for (n, entry) in self.data.clone().into_iter().enumerate() {
            if let Some(func) = entry.func {
                let name = format!("y = {}", entry.text.clone());
                let values = PlotPoints::from_explicit_callback(
                    move |x| match func.eval(&[x]) {
                        Ok(y) => y,
                        Err(e) => {
                            // DIRTY HACK THEY DON'T WANT YOU TO KNOW ABOUT!
                            if e.to_string() == "parsed expression contains 0 vars but passed slice has 1 elements" {
                                entry.text.parse().unwrap_or(0.0)
                            } else {
                                0.0
                            }
                        }
                    },
                    ..,
                    self.points,
                );

                let line = Line::new(values).name(name).color(COLORS[n]);

                lines.push(line);
            }
        }

        let frame = Frame::window(&Style::default()).inner_margin(Vec2 { x: 0.0, y: 0.0 });

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            if let Some(error) = &self.error {
                ui.centered_and_justified(|ui| {
                    ui.heading(format!("Error: {}", error));
                });
            } else {
                Plot::new("grapher")
                    .legend(Legend::default().text_style(TextStyle::Body))
                    .data_aspect(1.0)
                    .show(ui, |plot_ui| {
                        for line in lines {
                            plot_ui.line(line);
                        }
                    });
            }
        });
    }
}

impl Default for Grapher {
    fn default() -> Self {
        Self::new()
    }
}

impl App for Grapher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());

        self.side_panel(ctx);
        self.graph(ctx);
    }
}

/// An entry in the sidebar
#[derive(Clone, Debug)]
pub struct FunctionEntry {
    pub text: String,
    pub func: Option<FlatEx<f64>>,
}

impl FunctionEntry {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            func: None,
        }
    }
}
