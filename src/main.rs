use std::time::Instant;

use crate::{
    display::Display, pixel::Pixel, render_ctx::RenderCtx, stars3D::Stars3D, vertex::Vertex,
};

mod bitmap;
mod display;
mod pixel;
mod render_ctx;
mod stars3D;
mod vertex;
mod matrix;
mod vector;

fn main() {
    let mut start = Instant::now();
    let mut disp = Display::new([800, 600], "Software rendering".to_owned());
    let mut rctx = RenderCtx::new_from_bitmap(&disp.bitmap);
    disp.start();

    let min_y_vert = Vertex::new(100.0, 100.0);
    let mid_y_vert = Vertex::new(150.0, 200.0);
    let max_y_vert = Vertex::new(80.0, 300.0);

    //
    // let mut stars = Stars3D::new(4096, 25.0, 20.0);
    while disp.run() {
        let delta = start.elapsed().as_nanos() as f32 / 1_000_000_000.0;
        start = Instant::now();
        //
        disp.bitmap.fill_pixel(Pixel::BLACK);
        // for j in 100..200 {
        //     rctx.draw_scan_buffer(j, 300-j, 300+j);
        // }

        // rctx.scan_convert_tri(min_y_vert, mid_y_vert, max_y_vert, 0);

        // rctx.fill_shape(&mut disp.bitmap, 100, 300);
        rctx.fill_tri(&mut disp.bitmap, min_y_vert, mid_y_vert, max_y_vert);

        // stars.update_and_render(&mut disp.bitmap, delta);
        disp.update();
    }
    disp.stop();
}
