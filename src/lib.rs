use eframe::{
    egui::{
        self,
        panel::TopBottomSide,
        plot::{Line, Plot, Values},
        Frame, Style, TextEdit,
    },
    epaint::Vec2,
    epi::{self},
};
use exmex::{Express, FlatEx, ExError};

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();

    eframe::start_web(canvas_id, Box::new(Grapher::new()))
}

const EULER: &'static str = "2.7182818284590452353602874713527";

#[derive(Clone, Debug)]
pub struct Grapher {
    data: Vec<FunctionEntry>,
    error: Option<String>,
}

impl Grapher {
    pub fn new() -> Self {
        let mut data = Vec::new();
        data.push(FunctionEntry::new());

        Self { data, error: None }
    }

    fn side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.add_space(6.0);

            for entry in self.data.iter_mut() {
                let mut changed = false;

                ui.horizontal(|ui| {
                    ui.label("Function:");

                    if ui.add(TextEdit::singleline(&mut entry.text)).changed() {
                        if entry.text != "" {
                            changed = true;
                        } else {
                            entry.func = None;
                        }
                    };
                });

                if changed {
                    self.error = None;

                    // for nathan
                    let text = &entry.text.replace("e", EULER);

                    entry.func = match exmex::parse::<f64>(text) {
                        Ok(func) => Some(func),
                        Err(e) => {
                            self.error = Some(e.to_string());
                            break;
                        }
                    };
                }
            }
        });
    }

    fn graph(&mut self, ctx: &egui::Context) {
        let mut lines: Vec<Line> = Vec::new();

        for entry in self.data.clone() {
            if let Some(func) = entry.func {
                let values = Values::from_explicit_callback(
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
                    512,
                );

                let line = Line::new(values);

                lines.push(line);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(error) = &self.error {
                ui.centered_and_justified(|ui| {
                    ui.heading(format!("Error: {}", error));
                });
            } else {
                Plot::new("grapher").show(ui, |plot_ui| {
                    for line in lines {
                        plot_ui.line(line);
                    }
                });
            }
        });
    }
}

impl epi::App for Grapher {
    fn name(&self) -> &str {
        "Grapher"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2 {
            x: 4096.0,
            y: 2160.0,
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let frame = Frame::window(&Style::default()).margin(Vec2 { x: 10.0, y: 10.0 });

        egui::TopBottomPanel::new(TopBottomSide::Top, "top_panel")
            .frame(frame)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Grapher");

                    if ui.button("Add Function").clicked() {
                        self.data.push(FunctionEntry::new());
                    }

                    if ui.button("Remove Function").clicked() {
                        self.data.pop();
                    }

                    ui.label("Copyright 2022 Grant Handy");
                })
            });

        if !self.data.is_empty() {
            self.side_panel(ctx);
        }

        self.graph(ctx);
    }
}

#[derive(Clone, Debug)]
struct FunctionEntry {
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
