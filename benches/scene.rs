use std::time::Instant;

use image::ImageError;

use corrosion::{
    bitmap::Bitmap,
    camera::Camera,
    matrix::Matrix4F,
    mesh::Mesh,
    render_ctx::{clear_buffer, gen_buffer},
    stars_3d::Stars3D,
    transform::Transform,
    vector::Vector4F,
};
use macroquad::color::BLACK;

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 10, args = [0, 100, 500])]
fn render_scene(iterations : usize) {
    let mut bitmap = Bitmap::new([800, 600]);
    let mut z_buffer = gen_buffer(&bitmap);
    let mut camera = Camera::new(Matrix4F::new_perspective(
        (70.0_f32).to_radians(),
        800.0 / 600.0,
        0.1,
        1000.0,
    ));

    let texture_1 = Bitmap::new_from_file("res/bricks2.jpg").unwrap();
    let texture_2 = Bitmap::new_from_file("res/bricks.jpg").unwrap();

    let monkey_mesh = Mesh::new_from_obj_file("res/smoothMonkey0.obj").unwrap();
    let terrain_mesh = Mesh::new_from_obj_file("res/terrain2.obj").unwrap();

    let monkey_trans = Transform::from_pos(Vector4F::new(0.0, 0.0, 3.0, 1.0));
    let terrain_trans = Transform::from_pos(Vector4F::new(0.0, -1.0, 0.0, 1.0));
    //
    for _iter in 0..iterations {
        let vp = camera.get_view_projection();
        //
        bitmap.fill_pixel(BLACK);
        clear_buffer(&mut z_buffer);
        bitmap.draw_mesh(
            &monkey_mesh,
            &vp,
            &monkey_trans.transformation(),
            &texture_1,
            &mut z_buffer,
        );
        bitmap.draw_mesh(
            &terrain_mesh,
            &vp,
            &terrain_trans.transformation(),
            &texture_2,
            &mut z_buffer,
        );
    }

}
