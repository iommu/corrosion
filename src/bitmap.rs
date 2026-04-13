// Todo : use generics

use crate::pixel::Pixel;

pub struct Bitmap {
    width: usize,
    height: usize,
    components: Vec<Pixel>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            components: vec![Pixel::default(); width * height],
        }
    }

    pub fn clear(&mut self, shade: u8) {
        self.components.fill(Pixel::new(shade, shade, shade, shade));
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.components[x * self.width + y] = pixel;
    }
}
