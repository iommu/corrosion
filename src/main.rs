use std::time::Instant;

use crate::{display::Display, pixel::Pixel, stars3D::Stars3D};

mod bitmap;
mod display;
mod pixel;
mod stars3D;

fn main() {
    let mut start = Instant::now();
    let mut disp = Display::new([800, 600], "Software rendering".to_owned());
    disp.start();
    //
    let mut stars = Stars3D::new(4096, 25.0, 20.0);
    while disp.run() {
        let delta = start.elapsed().as_nanos() as f32 / 1_000_000_000.0;
        start = Instant::now();
        //
        stars.update_and_render(&mut disp.bitmap, delta);
        disp.update();
    }
    disp.stop();
}
