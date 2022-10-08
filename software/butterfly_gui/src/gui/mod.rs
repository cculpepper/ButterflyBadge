mod app;
mod layout;

use eframe::egui;

use crate::butterfly::vis2::{MyVis2Config, MyVis2};
use crate::butterfly::{BfContext, BfVis, Butterfly};
use crate::butterfly::vis::{BfVis1, SolidColorVis};

use layout::*;
pub use app::MyApp;

pub trait ContextCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut egui::Ui) -> bool;

    /// Create a Butterfly context.
    fn create(&self) -> BfContext;
}

pub trait VisCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut egui::Ui) -> bool;

    // Create a Butterfly visual driver
    fn create(&self, ctx: &BfContext) -> Box<dyn BfVis>;
}

pub trait ButterflyCreator {
    /// Display the creator widget and return whether the settings changed.
    fn show(&mut self, ui: &mut egui::Ui) -> bool;
    // Create a Butterfly visual driver
    fn create(&self) -> Option<Butterfly>;
}

struct SimpleContextCreator {
    time: f32,
    layout: Box<dyn LayoutCreator>,
}

impl Default for SimpleContextCreator {
    fn default() -> Self {
        Self {
            time: 0.,
            layout: Box::new(MultiLayoutCreator::default()),
        }
    }
}

impl ContextCreator for SimpleContextCreator {
    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.horizontal(|ui: &mut egui::Ui| {
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
    color: egui::Color32,
}

impl Default for SolidColorVisCreator {
    fn default() -> Self {
        Self {
            color: egui::Color32::LIGHT_BLUE,
        }
    }
}

struct FirstVisCreator {}

impl VisCreator for FirstVisCreator {
    fn show(&mut self, _ui: &mut egui::Ui) -> bool {
        false
    }

    fn create(&self, _ctx: &BfContext) -> Box<dyn BfVis> {
        Box::new(BfVis1)
    }
}

// todo viscreator should have type bounds
impl VisCreator for SolidColorVisCreator {
    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        ui.color_edit_button_srgba(&mut self.color).changed()
    }

    fn create(&self, _ctx: &BfContext) -> Box<dyn BfVis> {
        Box::new(SolidColorVis { color: self.color })
    }
}

impl VisCreator for MyVis2Config {

    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        ui.label("ring_count");

        let mut changed = false;

        changed |= ui.add(egui::DragValue::new(&mut self.ring_count).speed(1.)).changed();
        changed |= ui.add(egui::Slider::new(&mut self.time_scale, (0.)..=(2.) )).changed();


        changed
    }

    fn create(&self, ctx: &BfContext) -> Box<dyn BfVis> {
        Box::new(MyVis2::from_ctx_cfg(ctx, *self))
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
            String::from("Vis2"),
            Box::new(MyVis2Config::default()),
        ));
        
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

    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let last_idx = self.selected_idx;

        ui.horizontal_wrapped(|ui: &mut egui::Ui| {
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

    fn show(&mut self, ui: &mut egui::Ui) -> bool {
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

