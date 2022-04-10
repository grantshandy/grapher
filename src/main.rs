#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() {
    eframe::run_native(
        Box::new(grapher::Grapher::new()),
        eframe::NativeOptions::default(),
    );
}
