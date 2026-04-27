use std::time::Instant;

use image::ImageError;

use corrosion::{
    bitmap::Bitmap,
    camera::Camera,
    display::Display,
    matrix::Matrix4F,
    mesh::Mesh,
    pixel::Pixel,
    render_ctx::{clear_buffer, gen_buffer},
    stars_3d::Stars3D,
    transform::Transform,
    vector::Vector4F,
};

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 10, args = [0, 100, 500])]
fn render_scene(iterations : usize) -> Result<(), ImageError> {
    let mut start = Instant::now();
    let mut disp = Display::new([800, 600], "Software rendering".to_owned());
    let mut z_buffer = gen_buffer(&disp.bitmap);
    let mut camera = Camera::new(Matrix4F::new_perspective(
        (70.0_f32).to_radians(),
        800.0 / 600.0,
        0.1,
        1000.0,
    ));

    let texture_1 = Bitmap::new_from_file("res/bricks2.jpg")?;
    let texture_2 = Bitmap::new_from_file("res/bricks.jpg")?;

    let monkey_mesh = Mesh::new_from_obj_file("res/smoothMonkey0.obj")?;
    let terrain_mesh = Mesh::new_from_obj_file("res/terrain2.obj")?;

    let monkey_trans = Transform::from_pos(Vector4F::new(0.0, 0.0, 3.0, 1.0));
    let terrain_trans = Transform::from_pos(Vector4F::new(0.0, -1.0, 0.0, 1.0));

    // let min_y_vert = Vertex::new(
    //     Vector4F::new(-1.0, -1.0, 0.0, 1.0),
    //     Vector4F::new(0.0, 0.0, 0.0, 0.0),
    // );
    // let mid_y_vert = Vertex::new(
    //     Vector4F::new(0.0, 1.0, 0.0, 1.0),
    //     Vector4F::new(0.5, 1.0, 0.0, 0.0),
    // );
    // let max_y_vert = Vertex::new(
    //     Vector4F::new(1.0, -1.0, 0.0, 1.0),
    //     Vector4F::new(1.0, 0.0, 1.0, 0.0),
    // );

    //
    for iter in 0..iterations {
        let vp = camera.get_view_projection();
        //
        disp.bitmap.fill_pixel(Pixel::BLACK);
        clear_buffer(&mut z_buffer);
        disp.bitmap.draw_mesh(
            &monkey_mesh,
            &vp,
            &monkey_trans.transformation(),
            &texture_1,
            &mut z_buffer,
        );
        disp.bitmap.draw_mesh(
            &terrain_mesh,
            &vp,
            &terrain_trans.transformation(),
            &texture_2,
            &mut z_buffer,
        );
    }

    Ok(())
}
