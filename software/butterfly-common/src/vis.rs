
use core::f32::consts::{TAU, PI};

use crate::{Vec2, hsv2rgb_u8, butterfly_uvs};
use libm::{sinf, remainderf, fabsf, modff};


pub fn butterfly_uv_iter() -> impl Iterator<Item = Vec2> {
    butterfly_uvs().iter().copied().map(|f| {
        f.compwise_mul(Vec2::new(1./290., 1./200.))
    })
}

/// time in seconds since program start
// offset sin wave for saturation
// fixed color space
pub fn color_fn_1(uv: Vec2, time: f32) -> (u8,u8,u8) {
    const TIME_FACTOR: f32 = 1. / 5.;
    const WAVELENGTH: f32 = 300.;
    const SATURATION_RANGE: [f32; 2] = [0.2, 1.0];

    fn hue_for_uv(uv: Vec2) -> f32 {
        let (hue, _) = modff(uv.magnitude());
        hue
    }

    let time_t = remainderf(time * TIME_FACTOR, 1.0);

    let wave_height = {
        let wave_vec = Vec2::angled( PI * 0.25);
        let wave_t_offset = uv.dot(wave_vec) % WAVELENGTH;
        let t = (time_t * TAU) + wave_t_offset;
        
        fabsf(
            sinf(t)
        )
    };

    let saturation = wave_height * (SATURATION_RANGE[1] - SATURATION_RANGE[0]) + SATURATION_RANGE[0];

    let hue = {
        let offset_uv = Vec2::new(
            remainderf(uv.x() + time_t * 0.5, 1.0),
            uv.y(),
        );
        hue_for_uv(offset_uv)
    };

    let rgb = hsv2rgb_u8(hue * 255.0, saturation, 1.);
    rgb
}



