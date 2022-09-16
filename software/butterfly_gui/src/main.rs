#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::borrow::BorrowMut;
use std::f32::consts::TAU;
use std::ops::{RangeInclusive, Rem};
use std::time::Duration;
use eframe::egui;
use eframe::emath::Pos2;
use egui::{Color32, epaint, Sense, TextureHandle, vec2};
use egui::epaint::CircleShape;
use egui_extras::RetainedImage;
use emath::{remap_clamp, Vec2};
use crate::egui::{ColorImage, Image};
use crate::egui::color::Hsva;

fn main() {
    let options = eframe::NativeOptions {
        resizable: false,
        initial_window_pos: Some(Pos2{x: 1200.,y: 800.}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Butterfly Visualization",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct LedData {
    uv: Vec2,
    color: Color32,
}

struct MyApp {
    frames_elapsed: usize,
    leds: Vec<LedData>,
    butterfly_texture: RetainedImage,
    butterfly_image: ColorImage,
}

impl Default for MyApp {

    fn default() -> Self {
        let image = egui_extras::image::load_svg_bytes(include_bytes!("butterfly.svg")).unwrap();

        let grid_size = [40usize, 40usize];

        let mut leds = Vec::with_capacity(grid_size[0] * grid_size[1]);

        for row_idx in 0..grid_size[0] {
            for col_idx in 0..grid_size[1] {
                let uv = Vec2 {
                    x: (col_idx as f32 / grid_size[0] as f32),
                    y: (row_idx as f32 / grid_size[1] as f32),
                };

                let (x, y) = (
                    (image.size[0] as f32 * uv.x) as usize,
                    (image.size[1] as f32 * uv.y) as usize,
                );

                let pixel_idx = y * image.size[0] + x;
                let pixel = image.pixels[pixel_idx];

                if pixel.is_opaque() {
                    leds.push(LedData {
                        uv,
                        color: Color32::WHITE,
                    });
                }
            }
        }

        Self {
            frames_elapsed: 0,
            leds,
            butterfly_texture: RetainedImage::from_color_image(
                "butterfly.svg",
                image.clone(),
            ),
            butterfly_image: image,
        }
    }

}

fn hue_for_uv(uv: Vec2) -> f32 {
    let hue = uv.length();

    hue
}

/// time in seconds since program start
// offset sin wave for saturation
// fixed color space
fn color_fn_1(uv: Vec2, time: f32) -> Color32 {
    const TIME_FACTOR: f32 = 1./5.;

    const WAVELENGTH: f32 = 300.;
    const SATURATION_RANGE: [f32; 2] = [0.2, 1.0];

    let time_t = (time * TIME_FACTOR).rem(1.0);

    let wave_vec = Vec2::angled(TAU * 0.25);
    let projection = uv.dot(wave_vec) * wave_vec;

    let wave_t_offset = projection.length() % WAVELENGTH;
    let wave_height = ((time_t * TAU) + wave_t_offset).sin().abs();

    let x = wave_height * (SATURATION_RANGE[1] - SATURATION_RANGE[0]) + SATURATION_RANGE[0];

    let hue = {
        let offset_uv = Vec2 {
            x: (uv.x + time_t*0.5).rem(1.0),
            y: uv.y,
        };

        hue_for_uv(offset_uv)
    };

    let hsva = Hsva {
        h: hue,
        s: x,
        v: 1.0,
        a: 1.0
    };

    Color32::from(hsva)
}

impl MyApp {

    fn update_leds(&mut self) {
        let time = self.frames_elapsed as f32 / 60.0f32;
        for led in self.leds.iter_mut() {
            led.color = color_fn_1(led.uv, time);
        }
    }

}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.frames_elapsed = self.frames_elapsed.overflowing_add(1).0;
        if self.frames_elapsed % 10 == 0 {
            self.update_leds();
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let size = ui.available_size();
            self.butterfly_texture.show_size(ui, size);

            let painter = ui.painter();

            for led in &self.leds {

                let center = Pos2 {
                    x: led.uv.x * size.x,
                    y: led.uv.y * size.y,
                };

                let circle = CircleShape {
                    center: center,
                    radius: 5.,
                    fill: led.color,
                    stroke: Default::default()
                };

                painter.add(circle);
            }

            ctx.request_repaint();
        });

        // egui::TopBottomPanel::bottom("panel").show(ctx, |ui: &mut egui::Ui| {
        //     ui.label(format!("frame: {}", self.frames_elapsed));
        // });

    }

}
