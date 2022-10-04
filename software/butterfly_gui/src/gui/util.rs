use crate::butterfly::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct SimpleGrid {
    pub size: [i32; 2],
}

impl SimpleGrid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            size: [width, height],
        }
    }

    pub fn uv_for_pos(&self, pos: [i32; 2]) -> [f64; 2] {
        [
            (pos[0] as f64 / self.size[0] as f64),
            (pos[1] as f64 / self.size[1] as f64),
        ]
    }

    pub fn iter(self) -> NumIterator2d {
        return NumIterator2d {
            cur: [0,0],
            bounds: self.size,
        }
    }
}

pub struct NumIterator2d {
    cur: [i32; 2],
    bounds: [i32; 2],
}

impl Iterator for NumIterator2d {
    type Item = [i32; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur[1] + 1 < self.bounds[1] {
            self.cur[1] += 1;
            Some(self.cur)

        } else if self.cur[0] + 1 < self.bounds[0] {
            self.cur[1] = 0;
            self.cur[0] += 1;
            Some(self.cur)

        } else {
            return None;
        }
    }
}

pub fn adjust_uvs_to_fill(uvs: &mut [Vec2]) {
    if uvs.is_empty() {
        return;
    }
    if uvs.len() == 1 {
        uvs[0] = Vec2::new(0.5, 0.5);
        return;
    }

    let mut min = Vec2::new(1.0, 1.0);
    let mut max = Vec2::new(0.0,0.0);

    let update_min_max = |uv: &Vec2| {
        if uv.x < min.x {
            min.x = uv.x;
        }
        if uv.y < min.y {
            min.y = uv.y;
        }
        if uv.x > max.x {
            max.x = uv.x;
        }
        if uv.y > max.y {
            max.y = uv.y;
        }
    };
    uvs.iter().for_each(update_min_max);

    let uv_for_new_min_max = |uv: Vec2| -> Vec2 {
        (uv - min) / (max - min)
    };

    uvs.iter_mut().for_each(|uv| {
        *uv = uv_for_new_min_max(*uv);
    });
}

pub fn save_to_svg(points: impl Iterator<Item =(f32,f32)>, file_name: &str) {
    use svg::Document;
    use svg::node::element::Circle;

    let view_box = (900,600);

    let mut document = Document::new()
        .set("viewBox", (0, 0, view_box.0, view_box.1));

    let iter = points.map(|(x,y)| {
        Circle::new()
            .set("fill", "red")
            .set("cx", view_box.0 as f32 * x)
            .set("cy", view_box.1 as f32 * y)
            .set("r", view_box.0 as f32 / 200.)

    }).into_iter();

    for c in iter {
        document = document.add(c);
    }

    // let bf = svg::read(include_str!("butterfly.svg")).unwrap();
    // bf.

    svg::save(file_name, &document).unwrap();
}

pub fn save_to_csv(points: impl Iterator<Item =(f32,f32)>, file_name: &str) {
    extern crate csv;


    let mut csv_writer = csv::Writer::from_path(file_name).unwrap();
    csv_writer.write_record(&["LED_NUM", "x", "y"]);
    for (i, pt) in points.enumerate(){
        csv_writer.serialize((
            i,
            pt.0,
            pt.1,
        ));
    }

    csv_writer.flush();
}
