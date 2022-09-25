mod app;
mod util;

use std::cell::Cell;
use std::ops::{Mul, Sub};
use eframe::epaint::CircleShape;
use egui_extras::RetainedImage;
use emath::{Pos2};

use crate::butterfly::{BfContext, BfVis, Butterfly, Color32, egui, Led, Vec2};
use crate::butterfly::egui::{ColorImage, Response, Sense, Ui};
use crate::butterfly::vis::{BfVis1, SolidColorVis};

use util::*;
pub use app::MyApp;

pub trait LayoutCreator {
    fn show(&mut self, ui: &mut Ui) -> bool;
    fn create(&self) -> Vec<Led>;
}

pub trait ContextCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut Ui) -> bool;

    /// Create a Butterfly context.
    fn create(&self) -> BfContext;
}

pub trait VisCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut Ui) -> bool;

    // Create a Butterfly visual driver
    fn create(&self, ctx: &BfContext) -> Box<dyn BfVis>;
}

pub trait ButterflyCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut Ui) -> bool;
    // Create a Butterfly visual driver
    fn create(&self) -> Option<Butterfly>;
}

struct GridLayoutCreator {
    grid_size: [i32; 2],
    image: ColorImage,
}

impl Default for GridLayoutCreator {
    fn default() -> Self {
        Self {
            grid_size: [40,40],
            image: egui_extras::image::load_svg_bytes(include_bytes!("butterfly.svg")).unwrap(),
        }
    }
}

impl LayoutCreator for GridLayoutCreator {

    fn show(&mut self, ui: &mut Ui) -> bool {
        let last_grid_size = self.grid_size;

        ui.horizontal(|ui: &mut Ui| {
            ui.add(egui::Slider::new(&mut self.grid_size[1], 1..=100).text("grid_size_y"));
            ui.add(egui::Slider::new(&mut self.grid_size[0], 1..=100).text("grid_size_x"));
        });

        last_grid_size != self.grid_size
    }

    fn create(&self) -> Vec<Led> {
        let uv_to_image_xy = |uv: [f64; 2]| {
            [
                self.image.size[0] as f64 * uv[0],
                self.image.size[1] as f64 * uv[1],
            ]
        };

        let filter_uv = |uv: [f64; 2]| -> Option<Vec2>{
            let xy = uv_to_image_xy(uv);
            let pixel_idx = xy[1] as usize * self.image.size[0] + xy[0] as usize;

            let should_keep_point = self.image.pixels[pixel_idx].is_opaque();
            if !should_keep_point {
                return None;
            }

            Some(Vec2::new(uv[0] as f32, uv[1] as f32))
        };

        let grid = SimpleGrid { size: self.grid_size };

        let mut uvs: Vec<Vec2> = grid.iter()
            .map(|p| grid.uv_for_pos(p))
            .filter_map(filter_uv)
            .collect();

        adjust_uvs_to_fill(uvs.as_mut_slice());

        uvs.into_iter()
            .enumerate()
            .map(|(idx, uv)| {
                Led {
                    idx,
                    uv,
                    color: Cell::new(Color32::WHITE),
                }
            }).collect()
    }
}


struct SimpleContextCreator {
    time: f32,
    layout: Box<dyn LayoutCreator>,
}

impl Default for SimpleContextCreator {
    fn default() -> Self {
        Self {
            time: 0.,
            layout: Box::new(GridLayoutCreator::default()),
        }
    }
}

impl ContextCreator for SimpleContextCreator {
    fn show(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;

        ui.horizontal(|ui: &mut Ui| {
            ui.label("start_time");
            changed |= ui.add(egui::DragValue::new(&mut self.time).speed(1.)).changed();
        });

        ui.separator();

        ui.label("Layout Creator");
        changed |= self.layout.show(ui);

        changed
    }

    fn create(&self) -> BfContext {
        BfContext {
            time: self.time,
            leds: self.layout.create(),
        }
    }
}

struct SolidColorVisCreator {
    color: Color32,
}

impl Default for SolidColorVisCreator {
    fn default() -> Self {
        Self {
            color: Color32::LIGHT_BLUE,
        }
    }
}


struct FirstVisCreator {}

impl VisCreator for FirstVisCreator {
    fn show(&mut self, ui: &mut Ui) -> bool {
        false
    }

    fn create(&self, ctx: &BfContext) -> Box<dyn BfVis> {
        Box::new(BfVis1)
    }
}

impl VisCreator for SolidColorVisCreator {
    fn show(&mut self, ui: &mut Ui) -> bool {
        ui.color_edit_button_srgba(&mut self.color).changed()
    }

    fn create(&self, ctx: &BfContext) -> Box<dyn BfVis> {
        Box::new(SolidColorVis { color: self.color })
    }
}

struct MultiVisCreator {
    selected_idx: usize,
    vis_creators: Vec<(String, Box<dyn VisCreator>)>,
}

impl Default for MultiVisCreator {
    fn default() -> Self {
        let mut vis_creators: Vec<(String, Box<dyn VisCreator>)> = Vec::new();
        vis_creators.push((
            String::from("FirstVis"),
            Box::new(FirstVisCreator{})
        ));
        vis_creators.push((
            String::from("SolidColorVis"),
            Box::new(SolidColorVisCreator::default())
        ));

        Self {
            selected_idx: 0,
            vis_creators,
        }
    }
}

impl VisCreator for MultiVisCreator {
    fn show(&mut self, ui: &mut Ui) -> bool {
        let last_idx = self.selected_idx;

        ui.horizontal_wrapped(|ui: &mut Ui| {
            for (idx, vis) in self.vis_creators.iter().enumerate() {
                ui.selectable_value(&mut self.selected_idx, idx, &vis.0);
            }
        });

        ui.separator();

        let selected_tup = &mut self.vis_creators[self.selected_idx];
        (self.selected_idx != last_idx) |
            selected_tup.1.show(ui)

    }

    fn create(&self, ctx: &BfContext) -> Box<dyn BfVis> {
        self.vis_creators[self.selected_idx].1.create(ctx)
    }
}

pub struct SimpleButterflyCreator {
    context_creator: Box<dyn ContextCreator>,
    vis_creator: Box<dyn VisCreator>,
}

impl Default for SimpleButterflyCreator {
    fn default() -> Self {
        Self {
            context_creator: Box::new(SimpleContextCreator::default()),
            vis_creator: Box::new(MultiVisCreator::default()),
        }
    }
}

impl ButterflyCreator for SimpleButterflyCreator {
    fn show(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;

        ui.label("Context Creator");
        changed |= self.context_creator.show(ui);
        ui.separator();

        ui.label("Vis Creator");
        changed |= self.vis_creator.show(ui);

        changed
    }

    fn create(&self) -> Option<Butterfly> {
        let ctx = self.context_creator.create();
        let mut vis = self.vis_creator.create(&ctx);

        vis.update(&ctx);

        Some(Butterfly {
            ctx,
            vis,
        })
    }
}

