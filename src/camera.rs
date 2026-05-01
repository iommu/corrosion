use macroquad::input::KeyCode;

use libcorr::{matrix::Matrix4F, quaternion::Quaternion, transform::Transform, vector::Vector4F};

pub struct Camera {
    transform: Transform,
    projection: Matrix4F,
}

impl Camera {
    pub fn new(projection: Matrix4F) -> Self {
        Self {
            transform: Transform::default(),
            projection,
        }
    }

    pub fn reproject(&mut self, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) {
        self.projection = Matrix4F::new_perspective(fov, aspect_ratio, z_near, z_far);
    }

    pub fn get_view_projection(&self) -> Matrix4F {
        let cam_rot = self.transform.rot().conjugate().to_rot_matrix();
        let cam_pos = *self.transform.pos() * -1.0;
        let cam_trans = Matrix4F::new_translation(cam_pos.x(), cam_pos.y(), cam_pos.z());

        self.projection * (cam_rot * cam_trans)
    }

    pub fn process_keys<F>(&mut self, mut is_key_down: F, delta: f32, scroll: f32)
    where
        F: FnMut(KeyCode) -> bool,
    {
        let y_axis = Vector4F::new(0.0, 1.0, 0.0, 1.0);

        // Speed and rotation amounts are hardcoded here.
        // In a more general system, you might want to have them as variables.

        let sens_x = 2.66 * delta;
        let sens_y = 2.0 * delta;
        let move_amount = 5.0 * delta;
        let scroll = if scroll == 0.0 { 0.0 } else { scroll.signum() };

        // Similarly, input keys are hardcoded here.
        // As before, in a more general system, you might want to have these as variables.
        //
        let forward = ((is_key_down(KeyCode::W) as i8 - is_key_down(KeyCode::S) as i8) as f32
            * move_amount)
            + (scroll * 3.0);
        let left =
            (is_key_down(KeyCode::A) as i8 - is_key_down(KeyCode::D) as i8) as f32 * move_amount;
        let up = (is_key_down(KeyCode::Space) as i8 - is_key_down(KeyCode::LeftShift) as i8) as f32
            * move_amount;
        let tilt =
            (is_key_down(KeyCode::Down) as i8 - is_key_down(KeyCode::Up) as i8) as f32 * sens_y;
        let azi =
            (is_key_down(KeyCode::Right) as i8 - is_key_down(KeyCode::Left) as i8) as f32 * sens_x;

        if forward != 0.0 {
            self.reposition(self.transform.rot().forward(), forward);
        }

        if left != 0.0 {
            self.reposition(self.transform.rot().left(), left);
        }

        if up != 0.0 {
            self.reposition(self.transform.rot().up(), up);
        }

        if tilt != 0.0 {
            self.rotate(self.transform.rot().right(), tilt);
        }

        if azi != 0.0 {
            self.rotate(y_axis, azi);
        }
    }

    pub fn reposition(&mut self, dir: Vector4F, amount: f32) {
        *self.transform.pos_mut() = *self.transform.pos() + dir * amount;
    }

    pub fn rotate(&mut self, axis: Vector4F, angle: f32) {
        self.transform = self.transform.rotate(Quaternion::from_axis(axis, angle))
    }
}
