use app::App;

mod app;
mod math;
mod tracer;

const MIN_WINDOW_SIZE: egui::Vec2 = egui::vec2(200.0, 200.0);

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        min_window_size: Some(MIN_WINDOW_SIZE),
        ..Default::default()
    };

    eframe::run_native(
        "Pathtracer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
