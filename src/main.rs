use libcorr::{
    bitmap::Bitmap,
    matrix::Matrix4F,
    mesh::Mesh,
    render_ctx::{clear_buffer, gen_buffer},
    transform::Transform,
    vector::Vector4F,
};

use corrosion::{camera::Camera, gui::DisplayTransform, stars_3d::Stars3D};

use macroquad::{
    color::{BLACK, WHITE},
    file::set_pc_assets_folder,
    input::{is_key_down, mouse_wheel},
    texture::draw_texture,
    time::{draw_fps, get_frame_time},
    window::next_frame,
};

use egui_macroquad::egui;

#[cfg(feature = "bench")]
fn main() {}

#[cfg(not(feature = "bench"))]
#[macroquad::main("Corrosion")]
async fn main() {
    set_pc_assets_folder("res");
    let mut bitmap = Bitmap::new([800, 600]);
    let mut z_buffer = gen_buffer(&bitmap);
    let mut camera = Camera::new(Matrix4F::new_perspective(
        (70.0_f32).to_radians(),
        800.0 / 600.0,
        0.1,
        1000.0,
    ));

    let texture_1 = Bitmap::new_from_bytes(include_bytes!("../res/bricks2.png"), None).unwrap();
    let texture_2 = Bitmap::new_from_bytes(include_bytes!("../res/bricks.png"), None).unwrap();

    let monkey_mesh = Mesh::new_from_obj_file("../res/smoothMonkey0.obj").unwrap();
    let terrain_mesh = Mesh::new_from_obj_file("../res/terrain2.obj").unwrap();

    let mut monkey_trans = DisplayTransform::from_pos(Vector4F::new(0.0, 0.0, 3.0, 1.0));
    let terrain_trans = Transform::from_pos(Vector4F::new(0.0, -1.0, 0.0, 1.0));

    // GUI
    let mut show_fps = false;

    loop {
        camera.update(is_key_down, get_frame_time(), mouse_wheel().1);

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

        draw_texture(&bitmap.to_texture(), 0.0, 0.0, WHITE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Settings").show(egui_ctx, |ui| {
                ui.label("Test");
                monkey_trans.ui(ui, "Monkey");
                ui.checkbox(&mut show_fps, "Show FPS");
            });
        });

        egui_macroquad::draw();

        if show_fps {
            draw_fps();
        }

        next_frame().await
    }
}
