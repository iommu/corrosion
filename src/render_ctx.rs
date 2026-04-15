use std::{mem::swap, usize};

use crate::{bitmap::Bitmap, pixel::Pixel, vertex::Vertex};

pub struct RenderCtx {
    scan_buffer: Vec<[usize; 2]>,
}

impl RenderCtx {
    pub fn new_from_bitmap(bitmap: &Bitmap) -> Self {
        Self::new(bitmap.height())
    }

    pub fn new(height: usize) -> Self {
        Self {
            scan_buffer: vec![[0; 2]; height],
        }
    }

    pub fn draw_scan_buffer(&mut self, y: usize, x_min: usize, x_max: usize) {
        self.scan_buffer[y] = [x_min, x_max];
    }

    pub fn fill_shape(&self, bitmap: &mut Bitmap, y_min: usize, y_max: usize) {
        for y in y_min..y_max {
            let [x_min, x_max] = self.scan_buffer[y];
            for x in x_min..x_max {
                bitmap.draw_pixel(x, y, Pixel::WHITE);
            }
        }
    }

    fn scan_convert_line(&mut self, min_y_vert: Vertex, max_y_vert: Vertex, side: usize) {
        let y_start = min_y_vert.y() as i32;
        let y_end = max_y_vert.y() as i32;
        let x_start = min_y_vert.x() as i32;
        let x_end = max_y_vert.x() as i32;

        let y_dist = y_end - y_start;
        let x_dist = x_end - x_start;

        if y_dist <= 0 {
            return;
        }

        let x_step = x_dist as f32 / y_dist as f32;
        let mut x_cur = x_start as f32;

        for y in y_start..y_end {
            self.scan_buffer[y as usize][side] = x_cur as usize;
            x_cur += x_step;
        }
    }

    pub fn scan_convert_tri(
        &mut self,
        min_y_vert: Vertex,
        mid_y_vert: Vertex,
        max_y_vert: Vertex,
        handedness: i32,
    ) {
        self.scan_convert_line(min_y_vert, max_y_vert, (0 + handedness) as usize);
        self.scan_convert_line(min_y_vert, mid_y_vert, (1 - handedness) as usize);
        self.scan_convert_line(mid_y_vert, max_y_vert, (1 - handedness) as usize);
    }

    pub fn fill_tri(&mut self, bitmap: &mut Bitmap, mut vert_1: Vertex, mut vert_2: Vertex, mut vert_3: Vertex) {
        let min_y_vert = &mut vert_1;
        let mid_y_vert = &mut vert_2;
        let max_y_vert = &mut vert_3;

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
        let handedness = if area >= 0.0 {
            1
        } else {
            0
        };

        self.scan_convert_tri(*min_y_vert, *mid_y_vert, *max_y_vert, handedness);
        self.fill_shape(bitmap, min_y_vert.y() as usize, max_y_vert.y() as usize);
    }
}
