fn main() {
    eframe::run_native(
        Box::new(grapher::Grapher::new()),
        eframe::NativeOptions::default(),
    );
}
