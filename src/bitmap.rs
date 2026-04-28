// Todo : use generics

use std::path::{Component, Path};

use image::{DynamicImage, GenericImageView, ImageError, ImageReader};
use macroquad::{color::Color, texture::Texture2D};

pub struct Bitmap {
    size: [usize; 2],
    components: Vec<[u8; 4]>,
    #[cfg(not(feature = "bench"))]
    texture: Texture2D,
}

impl Bitmap {
    pub fn new(size: [usize; 2]) -> Self {
        let components: Vec<[u8; 4]> = vec![[0, 0, 255, 255]; size[0] * size[1]];
        #[cfg(not(feature = "bench"))]
        let texture = Texture2D::from_rgba8(
            size[0] as u16,
            size[1] as u16,
            bytemuck::cast_slice(&components),
        );
        Self {
            size,
            components,
            #[cfg(not(feature = "bench"))]
            texture,
        }
    }

    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let img = ImageReader::open(path)?.decode()?;
        Ok(Self::new_from_img(img))
    }

    pub fn new_from_img(img: DynamicImage) -> Self {
        let dims = img.dimensions();
        let size = [dims.0 as usize, dims.1 as usize];

        let components: Vec<[u8; 4]> = img
            .into_rgba8()
            .into_raw()
            .chunks_exact(4)
            .map(|component| <[u8; 4]>::try_from(component).unwrap_or([0, 0, 0, 0]))
            .collect();

        #[cfg(not(feature = "bench"))]
        let texture = Texture2D::from_rgba8(
            size[0] as u16,
            size[1] as u16,
            bytemuck::cast_slice(&components),
        );

        Self {
            size,
            components,
            #[cfg(not(feature = "bench"))]
            texture,
        }
    }

    pub fn size(&self) -> [usize; 2] {
        self.size
    }

    pub fn fill_pixel(&mut self, pixel: Color) {
        self.components
            .fill([pixel.r as u8, pixel.g as u8, pixel.b as u8, pixel.a as u8]);
    }

    pub fn fill(&mut self, shade: u8) {
        self.components.fill([shade, shade, shade, shade]);
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: Color) {
        self.components[y * self.size[0] + x] =
            [pixel.r as u8, pixel.g as u8, pixel.b as u8, pixel.a as u8];
    }

    #[inline]
    pub fn copy_pixel(
        &mut self,
        x_dest: usize,
        y_dest: usize,
        x_src: usize,
        y_src: usize,
        bitmap: &Bitmap,
        light_amount: f32,
    ) {
        let src_idx = y_src * bitmap.size[0] + x_src;
        let dst_idx = y_dest * self.size[0] + x_dest;
        let [r, g, b, a] = bitmap.components[src_idx];

        // By multiplying by [0, 256] (u32) and then dividing by 256 we achive the same result as multiplying by [0.0, 1.0] but without the repeated float multiplication
        let light_amount = (light_amount * 255.0) as u32;

        self.components[dst_idx] = [
            ((r as u32 * light_amount) >> 8) as u8,
            ((g as u32 * light_amount) >> 8) as u8,
            ((b as u32 * light_amount) >> 8) as u8,
            a,
        ];
    }

    pub fn width(&self) -> usize {
        self.size[0]
    }

    pub fn height(&self) -> usize {
        self.size[1]
    }

    #[cfg(not(feature = "bench"))]
    pub fn update(&self) {
        self.texture.update_from_bytes(
            self.size[0] as u32,
            self.size[1] as u32,
            bytemuck::cast_slice(&self.components),
        );
    }
}

#[cfg(not(feature = "bench"))]
impl std::ops::Deref for Bitmap {
    type Target = Texture2D;

    fn deref(&self) -> &Self::Target {
        self.update();
        &self.texture
    }
}
