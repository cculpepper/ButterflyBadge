
use std::cell::Cell;
use std::f64::consts::{PI, TAU, SQRT_2};
use std::sync::mpsc::channel;

use crate::butterfly::{BfContext, BfVis, Butterfly, Color32, egui, Led, Vec2};
use crate::butterfly::egui::{ColorImage, Response, Sense, Ui};
use crate::butterfly::vis::{BfVis1, SolidColorVis};

use crate::gui::LayoutCreator;
use crate::gui::util::*;

pub struct GridLayoutCreator {
    grid_size: [i32; 2],
    collide_detector: Box<dyn Fn([f64; 2]) -> bool>,
}

impl Default for GridLayoutCreator {

  fn default() -> Self {
      let image = egui_extras::image::load_svg_bytes(include_bytes!("butterfly.svg")).unwrap();
      Self {
          grid_size: [40,40],
          collide_detector: create_collide_detector(image),
      }
  }
}

impl LayoutCreator for GridLayoutCreator {

  fn show(&mut self, ui: &mut Ui) -> bool {
      let mut changed = false;
      ui.horizontal(|ui: &mut Ui| {
          ui.label("grid_size_x");
          changed |= ui.add(
              egui::Slider::new(&mut self.grid_size[0], 1..=100)
          ).changed();

          ui.label("grid_size_y");
          changed |= ui.add(
              egui::Slider::new(&mut self.grid_size[1], 1..=100)
          ).changed();
      });
      changed
  }

  fn create(&self) -> Vec<Led> {
      let filter_uv = |uv: [f64; 2]| -> Option<Vec2>{
          if (self.collide_detector)(uv) {
              Some(Vec2::new(uv[0] as f32, uv[1] as f32))
          } else {
              None
          }
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

fn create_collide_detector(image: ColorImage) -> Box<dyn Fn([f64; 2]) -> bool> {
    Box::new(move |p| {
        let xy = [image.size[0] as f64 * p[0], image.size[1] as f64 * p[1],];
        let pixel_idx = xy[1] as usize * image.size[0] + xy[0] as usize;
        image.pixels[pixel_idx].is_opaque()
    })
}

struct RadialLayoutCreator {
    radius: f64,
    step_angle: f64,
    cone_size: f64,
    start: [f64; 2],

    collide_detector: Box<dyn Fn([f64; 2]) -> bool>,
}

impl Default for RadialLayoutCreator {
    fn default() -> Self {
        let image = egui_extras::image::load_svg_bytes(include_bytes!("butterfly.svg")).unwrap();
        Self {
            radius: 0.04,
            step_angle: PI/30.,
            cone_size: 7./4.*PI,
            start: [0.5, 0.5],

            collide_detector: create_collide_detector(image),
        }
    }
}

impl RadialLayoutCreator {

    fn dist(a: [f64; 2], b: [f64; 2]) -> f64 {
        ((b[0]-a[0]) * (b[0]-a[0])
            + (b[1]-a[1]) * (b[1]-a[1])).sqrt()
    }

    fn angle_dist_to_vec(start: [f64; 2], angle: f64, dist: f64) -> [f64; 2] {
        let change_vec = [
            angle.cos(),
            angle.sin(),
        ];
        [
            start[0] + change_vec[0] * dist,
            start[1] + change_vec[1] * dist,
        ]
    }

    fn within_dist_of_other(candidate: [f64; 2], dist: f64, points: &[[f64;2]]) -> bool {
        points.iter().any(|p| {
            Self::dist(candidate, *p) <= dist
        })
    }

    fn test_candidate(&self, candidate: [f64; 2], points: &[[f64;2]]) -> bool {
        if candidate.iter().any(|v| {*v < 0.0 || *v > 1.0}) {
            return false;
        }

        (self.collide_detector)(candidate) &&
            !Self::within_dist_of_other(candidate, self.radius, points)
    }

    /// depth first
    fn visit_recursive(&self, p: [f64; 2], start_angle: f64, points: &mut Vec<[f64; 2]>) {

        let mut angle = start_angle;
        while angle < start_angle+self.cone_size {
            if points.len() > 1000 {
                return;
            }

            let candidate = RadialLayoutCreator::angle_dist_to_vec(p, angle, self.radius);

            if self.test_candidate(candidate, points.as_slice()) {
                points.push(candidate);
                self.visit_recursive(candidate, start_angle - (self.cone_size/2.), points);
            }

            angle += self.step_angle;
        }
    }

}

impl LayoutCreator for RadialLayoutCreator {

    fn show(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;
        ui.horizontal_wrapped(|ui: &mut Ui| {


            ui.label("radius");
            changed |= ui.add(
                egui::Slider::new(&mut self.radius, (0.001)..=0.2)
            ).changed();

            ui.label("step_angle");
            changed |= ui.add(
                egui::Slider::new(&mut self.step_angle, (0.01)..=(PI/4.))
            ).changed();

            ui.label("cone_size");
            changed |= ui.add(
                egui::Slider::new(&mut self.cone_size, (0.1)..=TAU)
            ).changed();


            ui.label("start_x");
            changed |= ui.add(
                egui::Slider::new(&mut self.start[0], (0.)..=1.)
            ).changed();

            ui.label("start_y");
            changed |= ui.add(
                egui::Slider::new(&mut self.start[1], (0.)..=1.)
            ).changed();

        });
        changed
    }

    fn create(&self) -> Vec<Led> {
        if !self.step_angle.is_finite() || self.step_angle <= 0. || !self.radius.is_finite() || self.radius <= 0. {
            return Vec::new();
        }

        let mut points = Vec::new();

        if self.test_candidate(self.start, points.as_slice()) {
            points.push(self.start);

            let mut angle = 0.;
            while angle < TAU {
                let candidate = RadialLayoutCreator::angle_dist_to_vec(self.start, angle, self.radius);

                if self.test_candidate(candidate, points.as_slice()) {
                    points.push(candidate);
                    self.visit_recursive(candidate, angle, &mut points);
                }
                angle += self.step_angle;
            }
        }

        points.into_iter().enumerate().map(|(idx, uv)| {
            Led {
                idx,
                uv: Vec2::new(uv[0] as f32, uv[1] as f32),
                color: Cell::new(Default::default())
            }
        }).collect()
    }
}



pub struct MultiLayoutCreator {
    selected_idx: usize,
    creators: Vec<(String, Box<dyn LayoutCreator>)>,
}

impl Default for MultiLayoutCreator {
    fn default() -> Self {
        let mut creators: Vec<(String, Box<dyn LayoutCreator>)> = Vec::new();
       
        creators.push((
            String::from("Radial"),
            Box::new(RadialLayoutCreator::default())
        ));

        creators.push((
            String::from("Grid"),
            Box::new(GridLayoutCreator::default())
        ));

        Self {
            selected_idx: 0,
            creators,
        }
    }
}

impl LayoutCreator for MultiLayoutCreator {
    fn show(&mut self, ui: &mut Ui) -> bool {
        let last_idx = self.selected_idx;

        ui.horizontal_wrapped(|ui: &mut Ui| {
            for (idx, vis) in self.creators.iter().enumerate() {
                ui.selectable_value(&mut self.selected_idx, idx, &vis.0);
            }
        });
        let selected_tup = &mut self.creators[self.selected_idx];

        if ui.button("Save Svg").clicked() {
            let layout = selected_tup.1.create();
            
            save_to_svg(layout.into_iter().map(|led| { 
                (led.uv.x,led.uv.y)
            }), "points.svg");

        } else if ui.button("Save Csv").clicked() {
            let layout = selected_tup.1.create();
            save_to_csv(layout.into_iter().map(|led| {
                (led.uv.x,led.uv.y)
            }), "points.csv");

        }
        ui.separator();

        (self.selected_idx != last_idx) |
            selected_tup.1.show(ui)

    }

    fn create(&self) -> Vec<Led> {
        self.creators[self.selected_idx].1.create()
    }
}




