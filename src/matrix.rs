use std::ops::{Index, IndexMut, Mul};

use crate::vector::Vector4F;

#[derive(Default, Clone, Copy)]
pub struct Matrix4F {
    mat: [[f32; 4]; 4],
}

impl Matrix4F {
    pub fn new_identity() -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new_ss_transform(half_width: f32, half_height: f32) -> Self {
        Self {
            mat: [
                [half_width, 0.0, 0.0, half_width - 0.5],
                [0.0, -half_height, 0.0, half_height - 0.5],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new_translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new_rotation_with_angle(x: f32, y: f32, z: f32, angle: f32) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        Self {
            mat: [
                [
                    cos + x * x * (1.0 - cos),
                    x * y * (1.0 - cos) - z * sin,
                    x * z * (1.0 - cos) + y * sin,
                    0.0,
                ],
                [
                    y * x * (1.0 - cos) + z * sin,
                    cos + y * y * (1.0 - cos),
                    y * z * (1.0 - cos) - x * sin,
                    0.0,
                ],
                [
                    z * x * (1.0 - cos) - y * sin,
                    z * y * (1.0 - cos) + x * sin,
                    cos + z * z * (1.0 - cos),
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new_rotation(x: f32, y: f32, z: f32) -> Self {
        let r_z = Matrix4F {
            mat: [
                [z.cos(), -(z.sin()), 0.0, 0.0],
                [z.sin(), z.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let r_x = Matrix4F {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, x.cos(), -(x.sin()), 0.0],
                [0.0, x.sin(), x.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let r_y = Matrix4F {
            mat: [
                [y.cos(), 0.0, -(y.sin()), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [y.sin(), 0.0, y.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        r_z.mul(r_y.mul(r_x))
    }

    pub fn transform(&self, rhs: Vector4F) -> Vector4F {
        Vector4F::new(
            self.mat[0][0] * rhs.x()
                + self.mat[0][1] * rhs.y()
                + self.mat[0][2] * rhs.z()
                + self.mat[0][3] * rhs.w(),
            self.mat[1][0] * rhs.x()
                + self.mat[1][1] * rhs.y()
                + self.mat[1][2] * rhs.z()
                + self.mat[1][3] * rhs.w(),
            self.mat[2][0] * rhs.x()
                + self.mat[2][1] * rhs.y()
                + self.mat[2][2] * rhs.z()
                + self.mat[2][3] * rhs.w(),
            self.mat[3][0] * rhs.x()
                + self.mat[3][1] * rhs.y()
                + self.mat[3][2] * rhs.z()
                + self.mat[3][3] * rhs.w(),
        )
    }

    pub fn new_scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            mat: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new_perspective(fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        let tan_half_fov = (fov / 2.0).tan();
        let z_range = z_near - z_far;

        Self {
            mat: [
                [1.0 / (tan_half_fov * aspect_ratio), 0.0, 0.0, 0.0],
                [0.0, 1.0 / tan_half_fov, 0.0, 0.0],
                [
                    0.0,
                    0.0,
                    (-z_near - z_far) / z_range,
                    (2.0 * z_far * z_near) / z_range,
                ],
                [0.0, 0.0, 1.0, 0.0],
            ],
        }
    }

    //  pub fn new_rotation_from_nrml(fov : f32, aspect_ratio : f32, z_near : f32, z_far : f32) -> Self {
    //     Self {
    //         mat: [
    //             [1.0, 0.0, 0.0, 0.0],
    //             [0.0, 1.0, 0.0, 0.0],
    //             [0.0, 0.0, 1.0, 0.0],
    //             [0.0, 0.0, 0.0, 1.0],
    //         ],
    //     }
    // }

    // pub fn new_rotation_from_vecs(fov : f32, aspect_ratio : f32, z_near : f32, z_far : f32) -> Self {
    //     Self {
    //         mat: [
    //             [1.0, 0.0, 0.0, 0.0],
    //             [0.0, 1.0, 0.0, 0.0],
    //             [0.0, 0.0, 1.0, 0.0],
    //             [0.0, 0.0, 0.0, 1.0],
    //         ],
    //     }
    // }
}

impl Mul for Matrix4F {
    type Output = Matrix4F;

    fn mul(self, rhs: Matrix4F) -> Self {
        let mut res = Self::default();

        for i in 0..4 {
            for j in 0..4 {
                res[i][j] = (self.mat[i][0] * rhs.mat[0][j])
                    + (self.mat[i][1] * rhs.mat[1][j])
                    + (self.mat[i][2] * rhs.mat[2][j])
                    + (self.mat[i][3] * rhs.mat[3][j]);
            }
        }

        res
    }
}

impl Index<usize> for Matrix4F {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl IndexMut<usize> for Matrix4F {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mat[index]
    }
}
