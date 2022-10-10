use eframe::egui;
use std::cell::Cell;

use crate::butterfly;

mod app;
mod layout;
mod vis;

pub use app::MyApp;
pub type Vec2 = egui::Vec2;

pub struct Layout {
    pub points: Vec<Vec2>,
}

impl Layout {
    pub fn from_points(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}

pub trait LayoutCreator {
    fn show(&mut self, ui: &mut egui::Ui) -> bool;
    fn create(&self) -> Layout;
}

pub trait VisCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut egui::Ui) -> bool;

    // Create a Butterfly visual driver
    fn create(&self, layout: &Layout) -> Box<dyn butterfly::BfVis>;
}

pub trait ButterflyCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut egui::Ui) -> bool;

    // Create a Butterfly visual driver
    fn create(&self) -> Option<butterfly::Butterfly>;
}

pub struct SimpleButterflyCreator {
    start_time: f32,
    layout_creator: Box<dyn layout::LayoutCreator>,
    vis_creator: Box<dyn VisCreator>,
}

impl Default for SimpleButterflyCreator {
    fn default() -> Self {
        Self {
            start_time: 0.,
            layout_creator: Box::new(layout::MultiLayoutCreator::default()),
            vis_creator: Box::new(vis::MultiVisCreator::default()),
        }
    }
}

impl ButterflyCreator for SimpleButterflyCreator {
    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label("start_time");
            changed |= ui
                .add(egui::DragValue::new(&mut self.start_time).speed(1.))
                .changed();
        });

        ui.separator();

        ui.label("Layout Creator");
        changed |= self.layout_creator.show(ui);

        ui.label("Vis Creator");
        changed |= self.vis_creator.show(ui);

        changed
    }

    fn create(&self) -> Option<butterfly::Butterfly> {
        let layout = self.layout_creator.create();

        let leds = layout
            .points
            .iter()
            .enumerate()
            .map(|(idx, &p)| butterfly::Led {
                idx,
                uv: p,
                color: Cell::new(Default::default()),
            })
            .collect();

        let vis = self.vis_creator.create(&layout);

        Some(butterfly::Butterfly {
            time: self.start_time,
            leds,
            vis: Cell::new(vis),
        })
    }
}
