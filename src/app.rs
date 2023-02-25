use egui::ColorImage;
use egui_extras::RetainedImage;
use pathtracer::save_buffer_to_ppm;

use crate::tracer::Tracer;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const SPACING: f32 = 8.0;

pub struct App {
    tracer: Tracer,
    fullscreen: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tracer: Tracer::new(WIDTH, HEIGHT),
            fullscreen: false,
        }
    }
}

impl App {
    fn resize_tracer(&mut self, width: usize, height: usize) {
        if width <= 0 || height <= 0 {
            return;
        }

        if width != self.tracer.width() || height != self.tracer.height() {
            self.tracer.resize(width, height).unwrap();
            self.tracer.trace();
        }
    }

    fn save(&mut self) -> anyhow::Result<()> {
        if let Some(path) = rfd::FileDialog::new()
            .set_directory("./")
            .set_file_name(".ppm")
            .save_file()
        {
            save_buffer_to_ppm(
                path,
                self.tracer.image_buffer(),
                self.tracer.width(),
                self.tracer.height(),
            )?;
        }
        Ok(())
    }

    fn show_viewport(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                outer_margin: egui::Margin::same(0.0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let width = ui.available_width() as usize;
                let height = ui.available_height() as usize;
                self.resize_tracer(width, height);

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

    fn show_sidepanel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("side_panel").show(ctx, |ui| {
            ui.add_space(SPACING);
            ui.label("Options:");
            ui.separator();

            ui.add_space(SPACING);

            ui.horizontal(|ui| {
                if ui.button("Trace").clicked() {
                    self.tracer.trace();
                }
                if ui.button("Save").clicked() {
                    self.save().unwrap();
                }
            });

            ui.add_space(SPACING);

            ui.label(format!("Frame Time: {:?}", self.tracer.frame_time()));

            ui.add_space(SPACING);

            ui.label("Render dimensions:");
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("Width: {}", self.tracer.width()));
                ui.label(format!("Height: {}", self.tracer.height()));
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_down(egui::Key::Escape)) {
            frame.close()
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
            self.fullscreen = !self.fullscreen
        }
        if ctx.input(|i| i.key_down(egui::Key::Enter)) {
            self.tracer.trace();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.save().unwrap()
        }

        frame.set_fullscreen(self.fullscreen);
        if !self.fullscreen {
            self.show_sidepanel(ctx);
            self.show_viewport(ctx)
        } else {
            self.show_viewport(ctx)
        }
    }
}
