use std::ops::{Mul, Sub};
use eframe::egui;
use eframe::egui::{Color32, ColorImage, Sense,Pos2};
use eframe::egui::CursorIcon::Text;
use eframe::epaint::CircleShape;
use egui_extras::RetainedImage;

use crate::butterfly::{BfContext, BfVis, Butterfly, Led, Vec2,};
use crate::butterfly::egui::Ui;
use crate::butterfly::vis::BfVis1;
use crate::gui::{ButterflyCreator, SimpleButterflyCreator};

pub struct MyApp {
    butterfly_retained_image: RetainedImage,
    butterfly_color_image: ColorImage,

    butterfly_creator: Box<dyn ButterflyCreator>,
    butterfly: Option<Butterfly>,

    frames_elapsed: i64,

    paused: bool,
    framerate: i32,
    update_frame_interval: i32,
    time_scale: f32,
    led_scale: f32,
}

impl Default for MyApp {

    fn default() -> Self {
        let image = egui_extras::image::load_svg_bytes(include_bytes!("butterfly.svg")).unwrap();

        let butterfly_creator: Box<dyn ButterflyCreator> = Box::new(SimpleButterflyCreator::default());
        let butterfly = butterfly_creator.create();

        Self {
            butterfly_retained_image: RetainedImage::from_color_image(
                "butterfly.svg",
                image.clone(),
            ),
            butterfly_color_image: image,
            butterfly_creator,
            butterfly,
            frames_elapsed: 0,
            paused: false,
            framerate: 60,
            update_frame_interval: 10,
            time_scale: 1.,
            led_scale: 8.,
        }
    }
}

impl MyApp {

    // fn set_leds_from_creator(&mut self) {
    //     self.butterfly_context = BfContext {
    //         leds: self.led_layout_creator.layout()
    //         ..self.butterfly_context
    //     };
    // }

    fn show_butterfly(&self, size: Vec2, ui: &mut Ui) {
        let PADDING: Vec2 = Vec2::new(20.,20.);
        let LED_SCALE_BASE = 1. / 1200.;

        if let Some(ref bf) = self.butterfly {
            let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
            
            let paint_region_size = rect.size() - PADDING.mul(Vec2::new(2.,2.));
            let paint_region_start = rect.min + PADDING;

            let uv_to_pos = |uv: &Vec2| -> Pos2 {
                Pos2 {
                    x: (uv.x * paint_region_size.x) + paint_region_start.x,
                    y: (uv.y * paint_region_size.y) + paint_region_start.y
                }
            };
 
            for led in &bf.ctx.leds {
                let circle = CircleShape {
                    center: uv_to_pos(&led.uv),
                    radius: rect.width() * self.led_scale * LED_SCALE_BASE,
                    fill: led.color.get(),
                    stroke: Default::default()
                };

                ui.painter().add(circle);
            }
        }
        //self.butterfly_retained_image.show_size(ui, cursor_rect.size());
    }

    fn show_app_settings(&mut self, ui: &mut Ui) {
        ui.label("App Settings");

        ui.horizontal_wrapped(|ui: &mut Ui| {
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
        });
    }

    fn update_context(&mut self, dt: f32) {
        if self.butterfly.is_some() {
            let mut bf = self.butterfly.as_mut().unwrap();

            bf.ctx.time += dt;
            bf.vis.as_mut()
                .update(&bf.ctx);
        }
    }

}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.paused {
            self.frames_elapsed = self.frames_elapsed.overflowing_add(1).0;

            if self.frames_elapsed % self.update_frame_interval as i64 == 0 {
                let dt = (self.update_frame_interval as f32 / self.framerate as f32) * self.time_scale;
                self.update_context(dt);
            }
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top_panel").min_height(280.).show(ctx, |ui: &mut egui::Ui| {

            ui.label(egui::RichText::new("Buterferlies are cool!")
                         .font(egui::FontId::monospace(20.))
                         .color(Color32::GOLD).italics());


            self.show_app_settings(ui);
            ui.separator();

            let changed = self.butterfly_creator.show(ui);

            if changed {
                self.butterfly = self.butterfly_creator.create();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            // todo calculate the constraining aspect ratio part
            let butterfly_size = Vec2 {
                x: ui.available_width(),
                y: ui.available_width() * (self.butterfly_color_image.height() as f32 / self.butterfly_color_image.width() as f32),
            };
            self.show_butterfly(butterfly_size, ui);
        });

        egui::TopBottomPanel::bottom("panel").show(ctx, |ui: &mut egui::Ui| {
            if let Some(bf) = self.butterfly.as_ref() {
                ui.horizontal_wrapped(|ui: &mut Ui| {
                    ui.label(format!("butterfly_time(s) {}", bf.ctx.time));
                    ui.label(format!("led_count {}", bf.ctx.leds.len()));
                });
            }
        });

    }

}
