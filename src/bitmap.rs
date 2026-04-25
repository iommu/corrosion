// Todo : use generics

use std::{
    io::{self, BufRead, Read, Seek},
    path::Path,
};

use image::{DynamicImage, GenericImageView, ImageError, ImageReader};

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

    pub fn new_from_file<P : AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)?.decode()?;
        Ok(Self::new_from_img(img))
    }

    pub fn new_from_img(img: DynamicImage) -> Self {
        let dims = img.dimensions();
        let bytes = img.into_rgba8();
        Self {
            size: [dims.0 as usize, dims.1 as usize],
            components: bytes
                .into_raw()
                .chunks_exact(4)
                .map(|component| <[u8; 4]>::try_from(component).unwrap_or([0, 0, 0, 0]))
                .collect(),
        }
    }

    pub fn size(&self) -> [usize; 2] {
        self.size
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

    pub fn copy_pixel(
        &mut self,
        x_dest: usize,
        y_dest: usize,
        x_src: usize,
        y_src: usize,
        bitmap: &Bitmap,
    ) {
        self.components[y_dest * self.size[0] + x_dest] =
            bitmap.components[y_src * bitmap.size[0] + x_src];
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
