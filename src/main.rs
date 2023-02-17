use app::App;

mod app;
mod tracer;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Pathtracer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
