#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod util;
pub mod vis;
pub mod vis2;

use std::cell::Cell;

pub use eframe::egui;

pub use egui::color::{Color32, Hsva};
pub use egui::{Vec2};

pub struct Butterfly {
    pub ctx: BfContext,
    pub vis: Box<dyn BfVis>
}

pub struct BfContext {
    pub time: f32,
    pub leds: Vec<Led>,
}

pub struct Led {
    pub idx: usize,
    pub uv: Vec2,
    pub color: Cell<Color32>,
}

pub trait BfVis {
    fn update(&mut self, ctx: &BfContext);
}


