use derive_new::new;
use getset::{Getters, MutGetters};

use crate::{matrix::Matrix4F, quaternion::Quaternion, vector::Vector4F};

#[derive(new, Clone, Copy, Getters, MutGetters)]
pub struct Transform {
    #[getset(get = "pub", get_mut = "pub")]
    pos: Vector4F,
    #[getset(get = "pub", get_mut = "pub")]
    rot: Quaternion,
    #[getset(get = "pub", get_mut = "pub")]
    scale: Vector4F,
}

impl Transform {
    pub fn default() -> Self {
        Self::from_pos(Vector4F::new(0.0, 0.0, 0.0, 0.0))
    }

    pub fn from_pos(pos: Vector4F) -> Self {
        Self {
            pos,
            rot: Quaternion::new(0.0, 0.0, 0.0, 1.0),
            scale: Vector4F::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn rotate(&self, rotation: Quaternion) -> Self {
        Self {
            pos: self.pos,
            rot: (rotation * self.rot).normalized(),
            scale: self.scale,
        }
    }

    pub fn look_at(&self, point: Vector4F, up: Vector4F) -> Self {
        self.rotate(self.get_look_at_rot(point, up))
    }

    pub fn get_look_at_rot(&self, point: Vector4F, up: Vector4F) -> Quaternion {
        Quaternion::from_rot(&Matrix4F::new_roation_from_fu(
            (point - self.pos).normalized(),
            up,
        ))
    }

    pub fn transformation(&self) -> Matrix4F {
        let trans_mat = Matrix4F::new_translation(self.pos.x(), self.pos.y(), self.pos.z());
        let rot_mat = self.rot.to_rot_matrix();
        let scale_mat = Matrix4F::new_scale(self.scale.x(), self.scale.y(), self.scale.z());

        trans_mat * (rot_mat * scale_mat)
    }
}
