// Todo : use generics

use crate::pixel::Pixel;

pub struct Bitmap {
    size: [usize; 2],
    components: Vec<[u8; 4]>,
}

impl Bitmap {
    pub fn new(size: [usize; 2]) -> Self {
        Self {
            size,
            components: vec![[0, 0, 0, 255]; size[0] * size[1] * 4],
        }
    }

    pub fn fill_pixel(&mut self, pixel: Pixel) {
        self.components.fill([pixel.r, pixel.g, pixel.b, pixel.a]);
    }

    pub fn fill(&mut self, shade: u8) {
        self.components.fill([shade, shade, shade, shade]);
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.components[y * self.size[0] + x]
            .copy_from_slice(&[pixel.r, pixel.g, pixel.b, pixel.a]);
    }

    pub fn get_buffer(&mut self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.components.as_ptr() as *const u8,
                self.components.len() * 4,
            )
        }
    }

    pub fn width(&self) -> usize {
        self.size[0]
    }

    pub fn height(&self) -> usize {
        self.size[1]
    }
}
