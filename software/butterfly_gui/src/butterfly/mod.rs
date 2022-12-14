use std::cell::Cell;

use eframe::egui;

pub mod vis;
pub mod vis2;

pub type Vec2 = egui::Vec2;
pub type Color32 = egui::Color32;
pub type Hsva = egui::color::Hsva;

pub struct Butterfly {
    pub time: f32,
    pub leds: Vec<Led>,
    pub vis: Cell<Box<dyn BfVis>>,
}

pub struct Led {
    pub idx: usize,
    pub uv: Vec2,
    pub color: Cell<Color32>,
}

pub trait BfVis {
    fn update(&mut self, bf: &Butterfly);
}
