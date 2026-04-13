use std::time::Instant;

use crate::{display::Display, pixel::Pixel, render_ctx::RenderCtx, stars3D::Stars3D};

mod bitmap;
mod display;
mod pixel;
mod stars3D;
mod  render_ctx;

fn main() {
    let mut start = Instant::now();
    let mut disp = Display::new([800, 600], "Software rendering".to_owned());
    let mut rctx = RenderCtx::new_from_bitmap(&disp.bitmap);
    disp.start();
    //
    // let mut stars = Stars3D::new(4096, 25.0, 20.0);
    while disp.run() {
        let delta = start.elapsed().as_nanos() as f32 / 1_000_000_000.0;
        start = Instant::now();
        //
        disp.bitmap.fill_pixel(Pixel::BLACK);
        for j in 100..200 {
            rctx.draw_scan_buffer(j, 300-j, 300+j); 
        }
        rctx.fill_shape(&mut disp.bitmap, 100, 200);
        // stars.update_and_render(&mut disp.bitmap, delta);
        disp.update();
    }
    disp.stop();
}
