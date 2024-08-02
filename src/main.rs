#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result {
    eframe::run_native(
        "Grapher",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(grapher::Grapher::new()))),
    )
}
