use fltk::{
    app::{self, App},
    frame::Frame,
    image::RgbImage,
    prelude::*,
    window::{DoubleWindow, Window},
};

use crate::bitmap::Bitmap;
pub struct Display {
    pub size: [usize; 2],
    pub bitmap: Bitmap,
    app: App,
    window: DoubleWindow,
    frame: Frame,
}

impl Display {
    pub fn new(size: [usize; 2], title: String) -> Self {
        let width = size[0] as i32;
        let height = size[1] as i32;
        let bitmap = Bitmap::new(size);
        let mut obj = Self {
            size,
            bitmap,
            app: app::App::default(),
            window: Window::default()
                .with_size(width, height)
                .with_label(&title),
            // Note : frame + 19 or there's a weird grey bar for some unknown reason
            frame: Frame::new(0, 0, width, height + 19, ""),
        };

        return obj;
    }

    pub fn start(&mut self) {
        self.window.show();
        self.app.wait();
    }

    pub fn run(&mut self) -> bool {
        return self.app.wait();
    }

    pub fn stop(&mut self) {
        self.app.quit();
    }

    pub fn update(&mut self) {
        let mut image = RgbImage::new(
            &self.bitmap.get_buffer(),
            self.size[0] as i32,
            self.size[1] as i32,
            fltk::enums::ColorDepth::Rgba8,
        )
        .unwrap();

        self.frame.set_image(Some(image));
        self.window.redraw();
    }
}
