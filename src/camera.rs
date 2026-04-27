use fltk::enums::Key;

use crate::{matrix::Matrix4F, quaternion::Quaternion, transform::Transform, vector::Vector4F};

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

    pub fn get_view_projection(&self) -> Matrix4F {
        let cam_rot = self.transform.rot().conjugate().to_rot_matrix();
        let cam_pos = *self.transform.pos() * -1.0;
        let cam_trans = Matrix4F::new_translation(cam_pos.x(), cam_pos.y(), cam_pos.z());

        self.projection * (cam_rot * cam_trans)
    }

    pub fn update(&mut self, key: Key, delta: f32) {
        const KEY_W: Key = Key::from_char('w');
        const KEY_A: Key = Key::from_char('a');
        const KEY_S: Key = Key::from_char('s');
        const KEY_D: Key = Key::from_char('d');

        let Y_AXIS = Vector4F::new(0.0, 1.0, 0.0, 1.0);

        // Speed and rotation amounts are hardcoded here.
        // In a more general system, you might want to have them as variables.

        let sens_x = 2.66 * delta;
        let sens_y = 2.0 * delta;
        let move_amount = 5.0 * delta;

        // Similarly, input keys are hardcoded here.
        // As before, in a more general system, you might want to have these as variables.
        match key {
            KEY_W => {
                self.reposition(self.transform.rot().forward(), move_amount);
            }
            KEY_S => {
                self.reposition(self.transform.rot().forward(), -move_amount);
            }
            KEY_A => {
                self.reposition(self.transform.rot().left(), move_amount);
            }
            KEY_D => {
                self.reposition(self.transform.rot().right(), move_amount);
            }
            Key::Right => {
                self.rotate(Y_AXIS, sens_x);
            }
            Key::Left => {
                self.rotate(Y_AXIS, -sens_x);
            }
            Key::Up => {
                self.rotate(self.transform.rot().right(), -sens_y);
            }
            Key::Down => {
                self.rotate(self.transform.rot().right(), sens_y);
            }
            _ => {}
        }
    }

    pub fn reposition(&mut self, dir: Vector4F, amount: f32) {
        *self.transform.pos_mut() = *self.transform.pos() + dir * amount;
    }

    pub fn rotate(&mut self, axis: Vector4F, angle: f32) {
        self.transform = self.transform.rotate(Quaternion::from_axis(axis, angle))
    }
}
