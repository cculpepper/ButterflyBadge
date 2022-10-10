use eframe::egui;
use eframe::emath;
use eframe::epaint;

use egui::Vec2;

use super::{ButterflyCreator, SimpleButterflyCreator};
use crate::butterfly::{Butterfly, Color32};
use crate::butterfly::vis::SolidColorVis;

pub struct MyApp {
    butterfly_color_image: egui::ColorImage,

    butterfly_creator: Box<dyn ButterflyCreator>,
    butterfly: Option<Butterfly>,

    frames_elapsed: i64,
    paused: bool,
    framerate: i32,
    update_frame_interval: i32,
    time_scale: f32,
    led_scale: f32,
    show_border: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let image = egui_extras::image::load_svg_bytes(include_bytes!("butterfly.svg")).unwrap();

        let butterfly_creator: Box<dyn ButterflyCreator> =
            Box::new(SimpleButterflyCreator::default());
        let butterfly = butterfly_creator.create();

        Self {
            butterfly_color_image: image,
            butterfly_creator,
            butterfly,
            frames_elapsed: 0,
            paused: false,
            framerate: 60,
            update_frame_interval: 10,
            time_scale: 1.,
            led_scale: 1.,
            show_border: true,
        }
    }
}

impl MyApp {
    fn show_butterfly(&self, size: Vec2, ui: &mut egui::Ui) {
        let padding: Vec2 = Vec2::new(20., 20.);
        let led_scale_base = 15. / 1200.;

        if let Some(ref bf) = self.butterfly {
            ui.vertical_centered(|ui: &mut egui::Ui| {
                let (rect, _resp) = ui.allocate_exact_size(size, egui::Sense::hover());

                let paint_region_size = rect.size() - (padding * Vec2::new(2., 2.));
                let paint_region_start = rect.min + padding;

                if self.show_border {
                    let paint_region_rect = epaint::Rect::from_two_pos(
                        paint_region_start,
                        epaint::Pos2::new(
                            paint_region_start.x + paint_region_size.x,
                            paint_region_start.y + paint_region_size.y,
                        ),
                    );
                    let border = egui::Stroke::new(2., Color32::GRAY);
                    ui.painter().rect(
                        paint_region_rect,
                        egui::Rounding::none(),
                        Color32::TRANSPARENT,
                        border,
                    );
                }

                let uv_to_pos = |uv: &Vec2| -> emath::Pos2 {
                    epaint::Pos2 {
                        x: (uv.x * paint_region_size.x) + paint_region_start.x,
                        y: (uv.y * paint_region_size.y) + paint_region_start.y,
                    }
                };

                for led in &bf.leds {
                    let circle = epaint::CircleShape {
                        center: uv_to_pos(&led.uv),
                        radius: rect.width() * self.led_scale * led_scale_base,
                        fill: led.color.get(),
                        stroke: Default::default(),
                    };

                    ui.painter().add(circle);
                }
            });
        }
        //self.butterfly_retained_image.show_size(ui, cursor_rect.size());
    }

    fn show_app_settings(&mut self, ui: &mut egui::Ui) {
        ui.label("App Settings");

        ui.horizontal_wrapped(|ui: &mut egui::Ui| {
            ui.label("paused:");
            ui.checkbox(&mut self.paused, "");

            ui.label("framerate?:");
            ui.add(egui::Slider::new(&mut self.framerate, 1..=144));

            ui.label("update_frame_interval:");
            ui.add(egui::Slider::new(&mut self.update_frame_interval, 1..=200));

            ui.label("time_scale:");
            ui.add(egui::Slider::new(&mut self.time_scale, (0.)..=10.));

            ui.label("led_scale:");
            ui.add(egui::Slider::new(&mut self.led_scale, (0.)..=20.));

            ui.label("show_border:");
            ui.checkbox(&mut self.show_border, "");
        });
    }

    fn update_butterfly(&mut self, dt: f32) {
        if self.butterfly.is_some() {
            let mut bf = self.butterfly.as_mut().unwrap();

            bf.time += dt;
            let mut vis = bf.vis.replace(Box::new(SolidColorVis{ color: Default::default() }));
            vis.update(&bf);
            bf.vis.set(vis);
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.paused {
            self.frames_elapsed = self.frames_elapsed.overflowing_add(1).0;

            if self.frames_elapsed % self.update_frame_interval as i64 == 0 {
                let dt =
                    (self.update_frame_interval as f32 / self.framerate as f32) * self.time_scale;
                self.update_butterfly(dt);
            }
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top_panel")
            .min_height(280.)
            .show(ctx, |ui: &mut egui::Ui| {
                ui.label(
                    egui::RichText::new("Buterferlies are cool!")
                        .font(egui::FontId::monospace(20.))
                        .color(egui::Color32::GOLD)
                        .italics(),
                );

                self.show_app_settings(ui);
                ui.separator();

                let changed = self.butterfly_creator.show(ui);

                if changed {
                    self.butterfly = self.butterfly_creator.create();
                }
            });

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let bf_size = Vec2 {
                x: self.butterfly_color_image.width() as f32,
                y: self.butterfly_color_image.height() as f32,
            };
            self.show_butterfly(
                calculate_constrained_paint_size(bf_size, ui.available_size()),
                ui,
            );
        });

        egui::TopBottomPanel::bottom("panel").show(ctx, |ui: &mut egui::Ui| {
            if let Some(bf) = self.butterfly.as_ref() {
                ui.horizontal_wrapped(|ui: &mut egui::Ui| {
                    ui.label(format!("butterfly_time(s) {}", bf.time));
                    ui.label(format!("led_count {}", bf.leds.len()));
                });
            }
        });
    }
}

fn calculate_constrained_paint_size(shape: Vec2, available: Vec2) -> Vec2 {
    let shape_aspect = shape.x / shape.y;
    let available_aspect = available.x / available.y;

    if available_aspect >= shape_aspect {
        Vec2 {
            x: available.y * shape_aspect,
            y: available.y,
        }
    } else {
        Vec2 {
            x: available.x,
            y: available.x / shape_aspect,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_constrained_paint_size() {
        assert_eq!(
            calculate_constrained_paint_size(Vec2::new(200., 100.), Vec2::new(100., 100.)),
            Vec2::new(100., 50.)
        );
        assert_eq!(
            calculate_constrained_paint_size(Vec2::new(200., 200.), Vec2::new(200., 100.)),
            Vec2::new(100., 100.)
        );
        assert_eq!(
            calculate_constrained_paint_size(Vec2::new(100., 200.), Vec2::new(100., 100.)),
            Vec2::new(50., 100.)
        );
        assert_eq!(
            calculate_constrained_paint_size(Vec2::new(25., 50.), Vec2::new(100., 100.)),
            Vec2::new(50., 100.)
        );
    }
}
