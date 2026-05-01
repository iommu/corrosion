// Todo : use generics

use std::path::Path;

use macroquad::{
    Error,
    color::Color,
    prelude::ImageFormat,
    texture::{Image, Texture2D},
};

pub struct Bitmap(Image);

impl Bitmap {
    pub fn new(size: [usize; 2]) -> Self {
        Self(Image {
            bytes: vec![255u8; size[0] * size[1] * 4],
            width: size[0] as u16,
            height: size[1] as u16,
        })
    }

    pub fn resize(&mut self, size: [usize; 2]) {
        if size[0] * size[1] > (self.0.width * self.0.height) as usize {
            self.0.bytes.resize(size[0] * size[1] * 4, 255u8);
        }
        self.0.width = size[0] as u16;
        self.0.height = size[1] as u16;
    }

    pub fn new_from_bytes(bytes: &[u8], format: Option<ImageFormat>) -> Result<Self, Error> {
        let img = Image::from_file_with_format(bytes, format)?;
        Ok(Self::new_from_img(img))
    }

    #[inline]
    fn slice_mut(bytes: &mut Vec<u8>) -> &mut [[u8; 4]] {
        unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut [u8; 4], bytes.len()) }
    }

    #[inline]
    fn slice(bytes: &Vec<u8>) -> &[[u8; 4]] {
        unsafe { std::slice::from_raw_parts(bytes.as_ptr() as *mut [u8; 4], bytes.len()) }
    }

    pub fn new_from_img(img: Image) -> Self {
        Self(img)
    }

    pub fn size(&self) -> [usize; 2] {
        [self.0.width as usize, self.0.height as usize]
    }

    pub fn fill_pixel(&mut self, pixel: Color) {
        for buff in unsafe { self.0.bytes.as_chunks_unchecked_mut() } {
            *buff = [pixel.r as u8, pixel.g as u8, pixel.b as u8, pixel.a as u8];
        }
    }

    pub fn fill(&mut self, shade: u8) {
        for buff in unsafe { self.0.bytes.as_chunks_unchecked_mut() } {
            *buff = [shade, shade, shade, 255];
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, pixel: Color) {
        Self::slice_mut(&mut self.0.bytes)[y * self.0.width as usize + x] =
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
        light: [u32; 4],
    ) {
        let src_idx = y_src * bitmap.0.width as usize + x_src;
        let dst_idx = y_dest * self.0.width as usize + x_dest;
        let [r, g, b, a] = Self::slice(&bitmap.0.bytes)[src_idx];

        // min(65280) (255 * 256) we can allow for lights greater than 1.0 for a blown out look
        Self::slice_mut(&mut self.0.bytes)[dst_idx] = [
            ((r as u32 * light[0]).min(65280) >> 8) as u8,
            ((g as u32 * light[1]).min(65280) >> 8) as u8,
            ((b as u32 * light[2]).min(65280) >> 8) as u8,
            a,
        ];
    }

    pub fn width(&self) -> usize {
        self.0.width as usize
    }

    pub fn height(&self) -> usize {
        self.0.height as usize
    }

    #[cfg(not(feature = "bench"))]
    pub fn to_texture(&self) -> Texture2D {
        Texture2D::from_image(&self.0)
    }
}
