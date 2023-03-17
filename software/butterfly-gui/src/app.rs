use std::ops::Rem;

use eframe::egui::{self, Sense, CentralPanel};
use eframe::egui::Ui;
use eframe::epaint::{
    Vec2, Pos2, Color32
};


use butterfly_common::{
    butterfly_uvs,
    vis::{
        color_fn_1,
        butterfly_uv_iter,
    }
};


// struct ButterflyVis<'a> {
//     uvs: &[Vec2],
//     leds: &[(u8,u8,u8)],
// }

pub struct ButterflyApp {
    leds: [[u8;3]; 512],
    time: f32,
}

impl Default for ButterflyApp {
    fn default() -> Self {
        Self {
            leds: [[0;3];512],
            time: 0.,
        }
    }
}

impl eframe::App for ButterflyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

            //self.time = std::time::SystemTime::now().elapsed().unwrap().as_secs_f32().rem(1000.,);
            self.time += 1./60.;
            self.step_vis();

            show_butterfly(
                ui,
                self.make_output())
        });
    }
}

impl ButterflyApp {

    fn step_vis(&mut self) {
        for (idx, uv) in butterfly_uv_iter().enumerate() {
            let (r,g,b) = butterfly_common::vis::color_fn_1(uv, self.time);
            self.leds[idx] = [r,g,b];
        }
    }
    
    fn make_output(&mut self, ) -> impl Iterator<Item = (Vec2, Color32)> + '_ {
        let color_iter = self.leds.iter().map(|&[r,g,b]| Color32::from_rgb(r, g, b));
        let uv_iter = butterfly_uv_iter().map(|v| Vec2::new(v.x(), v.y()));
        
        uv_iter.zip(color_iter)
    }

}


fn show_butterfly(ui: &mut egui::Ui, leds: impl Iterator<Item = (Vec2, Color32)>) {
    let painter = ui.painter();
    let rect = painter.clip_rect();

    let inner_pos = |uv: Vec2| -> Pos2 {
        const BORDER: f32 = 0.1;
        
        let scale = rect.size() * (1.0-BORDER);
        let offset = rect.size() * BORDER * 0.5;

        let value = (uv * scale) + offset;
        value.to_pos2()
    };


    for (uv, color) in leds {
        painter.circle_filled(
            inner_pos(uv),
            10.,
            color,
        );
    }
}