
use nalgebra::{ Vector2 };

use super::{ BfVis, BfContext, Vec2, Color32, Hsva };

pub struct MyVis2 {
  rings: Vec<Vec<usize>>,
  config: MyVis2Config,
}

#[derive(Clone, Copy)]
pub struct MyVis2Config {
    pub ring_count: i32,
    pub time_scale: f32,
}

impl Default for MyVis2Config {
    fn default() -> Self {
        Self {
            ring_count: 10,
            time_scale: 1./10.,
        }
    }
}

impl MyVis2 {

    pub fn from_ctx_cfg(ctx: &BfContext, config: MyVis2Config) -> Self {
      // let mut rings =  (0..10).into_iter()
      //   .map(|_| Vec::new() )
      //   .collect();
        let dist_to = |pos: Vec2| {
            let pos = Vector2::new(pos.x as f64, pos.y as f64);
            let my_pos = Vector2::new(0.5,0.5);

            (pos - my_pos).magnitude()
        };

        let mut sorted: Vec<(usize, f64)> = ctx.leds.iter()
            .enumerate()
            .map( |(idx, led)| {
                (idx, dist_to(led.uv))
            }).collect();

        sorted.sort_by(|(_, a), (_,b)| {
            a.partial_cmp(b).unwrap()
        });

        let rings: Vec<Vec<usize>> = if ctx.leds.len() < config.ring_count as usize {
            Vec::new()
        } else {
            sorted.chunks(sorted.len()/config.ring_count as usize)
                .map(|chunk| {
                    chunk.iter().map(|&(idx,_)| idx).collect()
                })
                .collect()
        };

        Self {
            rings,
            config,
        }
    }

}

impl BfVis for MyVis2 {
  fn update(&mut self, ctx: &BfContext) {
    let base_hue = ctx.time * self.config.time_scale;

    let hue_for_ring  = |idx: usize| {
      let ring_offset = idx as f32 * 1./(self.rings.len() as f32);

      base_hue + ring_offset
    };
    
      
    for (idx, ring) in self.rings.iter().rev().enumerate() {
      let color = Color32::from(
        Hsva {
          h: hue_for_ring(idx),
          s: 1.,
          v: 1.,
          a: 1.,
        }
      );
      
      for &led_idx in ring {
        ctx.leds[led_idx].color.set(color);
      }
    }

  }
}
