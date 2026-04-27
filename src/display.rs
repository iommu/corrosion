use std::{cell::RefCell, rc::Rc, sync::{Arc, mpsc::{Receiver, channel}}};

use fltk::{
    app::{self, App},
    enums::{Event, Key},
    frame::Frame,
    image::RgbImage,
    prelude::*,
    window::{DoubleWindow, Window},
};

use crate::bitmap::Bitmap;
pub struct Display {
    pub size: [usize; 2],
    pub bitmap: Bitmap,
    pub inputs : Receiver<Key>,
    app: App,
    window: DoubleWindow,
    frame: Frame,
}

impl Display {
    pub fn new(size: [usize; 2], title: String) -> Self {
        let (tx, rx) = channel::<Key>();
        let width = size[0] as i32;
        let height = size[1] as i32;
        let bitmap = Bitmap::new(size);
        let mut obj = Self {
            size,
            bitmap,
            inputs : rx,
            app: app::App::default(),
            window: Window::default()
                .with_size(width, height)
                .with_label(&title),
            // Note : frame + 19 or there's a weird grey bar for some unknown reason
            frame: Frame::new(0, 0, width, height + 19, ""),
        };

        obj.window.handle({
            let tx = tx.clone();
            move |_, ev| {
                match ev {
                    // we handle focus to be able to accept KeyDown events
                    Event::Focus => true,
                    Event::KeyDown => {
                        let key = app::event_key();
                        tx.send(key).unwrap();
                        true
                    }
                    _ => false,
                }
            }
        });

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

    pub fn drain_inputs<F>(&mut self, mut f: F)
    where
        F: FnMut(Key),
    {
        while let Ok(key) = self.inputs.try_recv() {
            f(key);
        }
    }
}
