use std::f32::consts::TAU;
use std::ops::Rem;

use crate::butterfly::{BfContext, BfVis, Led, Vec2, Color32, Hsva};

pub struct SolidColorVis {
    pub color: Color32,
}

impl BfVis for SolidColorVis {
    fn update(&mut self, ctx: &BfContext) {
        for led in ctx.leds.iter() {
            led.color.set(self.color);
        }
    }
}

pub struct BfVis1;

impl BfVis for BfVis1 {
    fn update(&mut self, ctx: &BfContext) {
        for led in ctx.leds.iter() {
            led.color.set(color_fn_1(led.uv, ctx.time));
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

