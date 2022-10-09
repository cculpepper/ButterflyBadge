use eframe::egui;

use super::{VisCreator, Layout};

use crate::butterfly::{BfVis};
use crate::butterfly;

struct FirstVisCreator;

impl VisCreator for FirstVisCreator {
    fn show(&mut self, _ui: &mut egui::Ui) -> bool {
        false
    }

    fn create(&self, _ctx: &Layout) -> Box<dyn BfVis> {
        Box::new(butterfly::vis::BfVis1)
    }
}

impl VisCreator for butterfly::vis2::MyVis2Config {
    fn show(&mut self, ui: &mut egui::Ui) -> bool {
        ui.label("ring_count");

        let mut changed = false;

        changed |= ui
            .add(egui::DragValue::new(&mut self.ring_count).speed(1.))
            .changed();
        changed |= ui
            .add(egui::Slider::new(&mut self.time_scale, (0.)..=(2.)))
            .changed();

        changed
    }

    fn create(&self, layout: &Layout) -> Box<dyn BfVis> {
        Box::new(butterfly::vis2::MyVis2::from_layout_and_cfg(layout, *self))
    }
}

pub struct MultiVisCreator {
    selected_idx: usize,
    vis_creators: Vec<(String, Box<dyn VisCreator>)>,
}

impl Default for MultiVisCreator {
    fn default() -> Self {
        let mut vis_creators: Vec<(String, Box<dyn VisCreator>)> = Vec::new();

        vis_creators.push((
            String::from("Vis2"),
            Box::new(butterfly::vis2::MyVis2Config::default()),
        ));

        vis_creators.push((String::from("FirstVis"), Box::new(FirstVisCreator {})));

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
        (self.selected_idx != last_idx) | selected_tup.1.show(ui)
    }

    fn create(&self, layout: &Layout) -> Box<dyn BfVis> {
        self.vis_creators[self.selected_idx].1.create(layout)
    }
}
