use egui::ColorImage;
use egui_extras::RetainedImage;
use pathtracer::save_buffer_to_ppm;

use crate::tracer::Tracer;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

pub struct App {
    tracer: Tracer,
}

impl App {
    fn save(&mut self) -> anyhow::Result<()> {
        let image = save_buffer_to_ppm(
            "./render.ppm",
            self.tracer.image_buffer(),
            self.tracer.width(),
            self.tracer.height(),
        )?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            tracer: Tracer::new(WIDTH, HEIGHT),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_down(egui::Key::Escape)) {
            frame.close()
        }
        if ctx.input(|i| i.key_down(egui::Key::Enter)) {
            self.tracer.trace();
        }

        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.label("Options:");
            ui.separator();

            ui.label(format!("Frame Time: {:?}", self.tracer.frame_time()));

            if ui.button("Trace").clicked() {
                self.tracer.trace();
            }
            if ui.button("Save").clicked() {
                self.save().unwrap();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width() as usize;
            let height = ui.available_height() as usize;
            if width != self.tracer.width() || height != self.tracer.height() {
                self.tracer
                    .resize(
                        ui.available_width() as usize,
                        ui.available_height() as usize,
                    )
                    .unwrap();
                self.tracer.trace();
            }

            dbg!(self.tracer.width());
            dbg!(self.tracer.height());
            dbg!(self.tracer.image_buffer().len());
            let render = RetainedImage::from_color_image(
                "render",
                ColorImage::from_rgb(
                    [self.tracer.width(), self.tracer.height()],
                    self.tracer.image_buffer(),
                ),
            );
            render.show(ui)
        });
    }
}
