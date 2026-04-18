use std::{mem::swap, usize};

use crate::{
    bitmap::Bitmap,
    edge::Edge,
    gradients::{self, Gradients},
    matrix::Matrix4F,
    pixel::Pixel,
    vertex::Vertex,
};

impl Bitmap {
    fn draw_scan_line(&mut self, gradients: &Gradients, left: &Edge, right: &Edge, y: usize) {
        let x_min = left.x().ceil() as i32;
        let x_max = right.x().ceil() as i32;
        let x_pre = x_min as f32 - left.x();
        let min_color = left.color() + gradients.color_x_step() * x_pre;
        let max_color = right.color() + gradients.color_x_step() * x_pre;

        let mut lerp = 0.0;
        let lerp_step = 1.0 / (x_max - x_min) as f32;

        for x in x_min..x_max {
            let color = min_color.lerp(max_color, lerp);
            let color = Pixel::new(
                (color.x() * 255.0) as u8,
                (color.y() * 255.0) as u8,
                (color.z() * 255.0) as u8,
                0xff,
            );
            self.draw_pixel(x as usize, y, color);
            lerp += lerp_step;
        }
    }

    fn scan_edges(&mut self, gradients : &Gradients, a: &mut Edge, b: &mut Edge, handedness: bool) {
        let y_start = b.y_start();
        let y_end = b.y_end();

        let [left, right] = match handedness {
            true => [b, a],
            false => [a, b],
        };

        for y in y_start..y_end {
            self.draw_scan_line(gradients, left, right, y as usize);
            left.step();
            right.step();
        }
    }

    fn scan_tri(
        &mut self,
        min_y_vert: Vertex,
        mid_y_vert: Vertex,
        max_y_vert: Vertex,
        handedness: bool,
    ) {
        let gradients = Gradients::new(min_y_vert, mid_y_vert, max_y_vert);
        let mut top_to_bot = Edge::new(&gradients, min_y_vert, max_y_vert, 0);
        let mut top_to_mid = Edge::new(&gradients, min_y_vert, mid_y_vert, 0);
        let mut mid_to_bot = Edge::new(&gradients, mid_y_vert, max_y_vert, 1);

        self.scan_edges(&gradients,&mut top_to_bot, &mut top_to_mid, handedness);
        self.scan_edges(&gradients,&mut top_to_bot, &mut mid_to_bot, handedness);
    }
}

pub struct RenderCtx {}

impl RenderCtx {
    pub fn new_from_bitmap(bitmap: &Bitmap) -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        Self {}
    }

    pub fn fill_tri(
        &mut self,
        bitmap: &mut Bitmap,
        mut vert_1: Vertex,
        mut vert_2: Vertex,
        mut vert_3: Vertex,
    ) {
        let ss_transform =
            Matrix4F::new_ss_transform(bitmap.width() as f32 / 2.0, bitmap.height() as f32 / 2.0);
        let min_y_vert = &mut vert_1.transform(ss_transform).perspective_div();
        let mid_y_vert = &mut vert_2.transform(ss_transform).perspective_div();
        let max_y_vert = &mut vert_3.transform(ss_transform).perspective_div();

        if max_y_vert.y() < mid_y_vert.y() {
            swap(max_y_vert, mid_y_vert);
        }

        if mid_y_vert.y() < min_y_vert.y() {
            swap(mid_y_vert, min_y_vert);
        }

        if max_y_vert.y() < mid_y_vert.y() {
            swap(max_y_vert, mid_y_vert);
        }

        let area: f32 = min_y_vert.tri_area(max_y_vert, mid_y_vert);
        let handedness = if area >= 0.0 { true } else { false };

        bitmap.scan_tri(*min_y_vert, *mid_y_vert, *max_y_vert, handedness);
    }
}
