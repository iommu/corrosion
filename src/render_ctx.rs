use std::{mem::swap, usize};

use crate::{bitmap::Bitmap, edge::Edge, matrix::Matrix4F, pixel::Pixel, vertex::Vertex};

impl Bitmap {
    fn draw_scan_line(&mut self, left: &Edge, right: &Edge, y: usize) {
        let x_min = left.x().ceil() as i32;
        let x_max = right.x().ceil() as i32;

        for x in x_min..x_max {
            self.draw_pixel(x as usize, y, Pixel::WHITE);
        }
    }

    fn scan_edges(&mut self, a: &mut Edge, b: &mut Edge, handedness: bool) {
        let y_start = b.y_start();
        let y_end = b.y_end();

        let [left, right] = match handedness {
            true => [b, a],
            false => [a, b],
        };

        for y in y_start..y_end {
            self.draw_scan_line(left, right, y as usize);
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
        let mut top_to_bot = Edge::new(min_y_vert, max_y_vert);
        let mut top_to_mid = Edge::new(min_y_vert, mid_y_vert);
        let mut mid_to_bot = Edge::new(mid_y_vert, max_y_vert);

        self.scan_edges(&mut top_to_bot, &mut top_to_mid, handedness);
        self.scan_edges(&mut top_to_bot, &mut mid_to_bot, handedness);
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
