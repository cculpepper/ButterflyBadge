#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod vis;

use std::borrow::BorrowMut;
use std::cell::Cell;
use std::f32::consts::TAU;
use std::ops::{Deref, Div, RangeFull, RangeInclusive, Rem, Sub};
use std::time::Duration;

pub use eframe::egui;
pub use egui::color::{Color32, Hsva};
pub use egui::{Vec2};

use egui::{epaint, Sense, TextureHandle};
use egui::epaint::CircleShape;
use egui_extras::RetainedImage;
use emath::{remap_clamp, };
use egui::{ColorImage, Image, Response, Ui, Widget};

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

