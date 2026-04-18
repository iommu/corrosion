use std::{ops::Mul, time::Instant};

use crate::{
    display::Display, matrix::Matrix4F, pixel::Pixel, render_ctx::RenderCtx, stars3D::Stars3D,
    vector::Vector4F, vertex::Vertex,
};

mod bitmap;
mod display;
mod edge;
mod matrix;
mod pixel;
mod render_ctx;
mod stars3D;
mod vector;
mod vertex;
mod gradients;

fn main() {
    let mut start = Instant::now();
    let mut disp = Display::new([800, 600], "Software rendering".to_owned());
    let mut rctx = RenderCtx::new_from_bitmap(&disp.bitmap);
    disp.start();

    let min_y_vert = Vertex::new(
        Vector4F::new(-1.0, -1.0, 0.0, 1.0),
        Vector4F::new(1.0, 0.0, 0.0, 1.0),
    );
    let mid_y_vert = Vertex::new(
        Vector4F::new(0.0, 1.0, 0.0, 1.0),
        Vector4F::new(0.0, 1.0, 0.0, 1.0),
    );
    let max_y_vert = Vertex::new(
        Vector4F::new(1.0, -1.0, 0.0, 1.0),
        Vector4F::new(0.0, 0.0, 1.0, 1.0),
    );

    let projection = Matrix4F::new_perspective((70.0_f32).to_radians(), 800.0 / 600.0, 0.1, 1000.0);

    //
    let mut rot_count: f32 = 0.0;
    while disp.run() {
        let delta = start.elapsed().as_nanos() as f32 / 1_000_000_000.0;
        start = Instant::now();

        //
        rot_count += delta;
        let translation = Matrix4F::new_translation(0.0, 0.0, 3.0);
        let rotation = Matrix4F::new_rotation(0.0, rot_count, 0.0);
        let transform = projection.mul(translation.mul(rotation));

        //
        disp.bitmap.fill_pixel(Pixel::BLACK);
        rctx.fill_tri(
            &mut disp.bitmap,
            min_y_vert.transform(transform),
            mid_y_vert.transform(transform),
            max_y_vert.transform(transform),
        );

        //
        disp.update();
    }
    disp.stop();
}
