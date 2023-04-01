use egui::{pos2, Color32, ColorImage, DragValue, Rect, Rounding};
use egui_extras::RetainedImage;
use pathtracer::save_buffer_to_ppm;

use crate::tracer::Tracer;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const SPACING: f32 = 8.0;

// TODO:
//   - fix: render disappearing when rendered and resized smaller than render dimensions
//     - this may be do to a bug in the calculate_display_size function
//   - add ability to change render size

pub struct App {
    tracer: Tracer,
    render: RetainedImage,
    fullscreen: bool,
    render_on_size_change: bool,
    fit_render_to_viewport: bool,
    render_continuously: bool,
    viewport_width: f32,
    viewport_heigth: f32,
    render_width: usize,
    render_height: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tracer: Tracer::new(WIDTH, HEIGHT),
            render: RetainedImage::from_color_image(
                "render",
                ColorImage {
                    size: [WIDTH, HEIGHT],
                    pixels: vec![Color32::from_gray(255); WIDTH * HEIGHT],
                },
            ),
            fullscreen: false,
            render_on_size_change: false,
            fit_render_to_viewport: false,
            render_continuously: false,
            viewport_width: WIDTH as f32,
            viewport_heigth: HEIGHT as f32,
            render_width: WIDTH,
            render_height: HEIGHT,
        }
    }
}

impl App {
    fn trace(&mut self) {
        self.tracer.trace();
        self.render = RetainedImage::from_color_image(
            "render",
            ColorImage::from_rgb(
                [self.tracer.width(), self.tracer.height()],
                self.tracer.image_buffer(),
            ),
        )
    }

    fn size_changed(&self, width: usize, height: usize) -> bool {
        self.tracer.width() != width || self.tracer.height() != height
    }

    // broken
    fn calculate_display_size(&self, available_width: f32, available_height: f32) -> egui::Vec2 {
        if self.render.width() as f32 > available_width {
            egui::vec2(
                available_width,
                available_width * (self.render.width() / self.render.height()) as f32,
            )
        } else if self.render.height() as f32 > available_height {
            egui::vec2(
                available_height * (self.render.height() / self.render.width()) as f32,
                available_height,
            )
        } else {
            egui::vec2(self.render.width() as f32, self.render.height() as f32)
        }
    }

    fn resize_tracer(&mut self, width: usize, height: usize) {
        if width <= 0 || height <= 0 {
            return;
        }

        let size_changed = self.size_changed(width, height);
        self.tracer.resize(width, height).unwrap();
        if self.render_on_size_change && size_changed {
            self.trace()
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
                self.viewport_width = ui.available_width();
                self.viewport_heigth = ui.available_height();
                self.resize_tracer(self.viewport_width as usize, self.viewport_heigth as usize);

                // kinda scuffed lol
                let render_dimension_guide_min = pos2(0.0, 0.0);
                let render_dimension_guide_max =
                    pos2(self.render_width as f32, self.render_height as f32);
                ui.painter().rect_filled(
                    Rect {
                        min: render_dimension_guide_min,
                        max: render_dimension_guide_max,
                    },
                    Rounding::none(),
                    egui::Color32::from_rgb(255, 0, 0),
                );
                ui.centered_and_justified(|ui| {
                    self.render.show_size(
                        ui,
                        self.calculate_display_size(self.viewport_width, self.viewport_heigth),
                    )
                })
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
                    self.trace();
                }
                if ui.button("Save").clicked() {
                    self.save().unwrap();
                }
            });

            ui.add_space(SPACING);

            ui.checkbox(&mut self.render_on_size_change, "Render on size change");
            ui.checkbox(&mut self.render_continuously, "Render continuously");
            ui.checkbox(&mut self.fit_render_to_viewport, "Fit render to viewport");

            ui.add_space(SPACING);

            ui.label(format!("Frame Time: {:?}", self.tracer.frame_time()));

            ui.add_space(SPACING);

            ui.label("Render dimensions:");
            ui.horizontal(|ui| {
                ui.label("Width:  ");
                ui.add_enabled(
                    !self.fit_render_to_viewport,
                    DragValue::new(&mut self.render_width)
                        .clamp_range(std::ops::RangeInclusive::new(1.0, self.viewport_width)),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Height: ");
                ui.add_enabled(
                    !self.fit_render_to_viewport,
                    DragValue::new(&mut self.render_height)
                        .clamp_range(std::ops::RangeInclusive::new(1.0, self.viewport_heigth)),
                );
            })
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
            self.trace();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.save().unwrap()
        }

        if self.render_continuously {
            self.trace()
        }
        frame.set_fullscreen(self.fullscreen);
        if !self.fullscreen {
            self.show_sidepanel(ctx);
            self.show_viewport(ctx)
        } else {
            self.show_viewport(ctx)
        }

        if self.render_continuously {
            ctx.request_repaint()
        }
    }
}
