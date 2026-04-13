use crate::{bitmap::Bitmap, pixel::Pixel};

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

    pub fn draw_scan_buffer(&mut self, y: usize,  x_min: usize, x_max: usize) {
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
}
