
use std::cell::Cell;

use crate::butterfly::{BfContext, BfVis, Butterfly, Color32, egui, Led, Vec2};
use crate::butterfly::egui::{ColorImage, Response, Sense, Ui};
use crate::butterfly::vis::{BfVis1, SolidColorVis};
use crate::gui::util::*;

pub trait LayoutCreator {
  fn show(&mut self, ui: &mut Ui) -> bool;
  fn create(&self) -> Vec<Led>;
}

pub struct GridLayoutCreator {
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

