use getset::{Getters, MutGetters};
use std::ops::{Add, Div, Mul, Sub};

use crate::{matrix::Matrix4F, vector::Vector4F};

#[derive(Debug, Default, Clone, Copy)]
pub struct Quaternion(Vector4F);

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4F::new(x, y, z, w))
    }

    pub fn from_axis(axis: Vector4F, angle: f32) -> Self {
        let sin_half_angle = (angle / 2.0).sin();
        let cos_half_angle = (angle / 2.0).cos();
        Self(Vector4F::new(
            axis.x() * sin_half_angle,
            axis.y() * sin_half_angle,
            axis.z() * sin_half_angle,
            cos_half_angle,
        ))
    }

    pub fn len(&self) -> f32 {
        self.0.len()
    }

    pub fn conjugate(&self) -> Self {
        Self(Vector4F::new(
            -self.0.x(),
            -self.0.y(),
            -self.0.z(),
            self.0.w(),
        ))
    }

    pub fn to_rot_matrix(&self) -> Matrix4F {
        let forward = Vector4F::new(
            2.0 * (self.0.x() * self.0.z() - self.0.w() * self.0.y()),
            2.0 * (self.0.y() * self.0.z() + self.0.w() * self.0.x()),
            1.0 - 2.0 * (self.0.x() * self.0.x() + self.0.y() * self.0.y()),
            1.0,
        );
        let up = Vector4F::new(
            2.0 * (self.0.x() * self.0.y() + self.0.w() * self.0.z()),
            1.0 - 2.0 * (self.0.x() * self.0.x() + self.0.z() * self.0.z()),
            2.0 * (self.0.y() * self.0.z() - self.0.w() * self.0.x()),
            1.0,
        );
        let right = Vector4F::new(
            1.0 - 2.0 * (self.0.y() * self.0.y() + self.0.z() * self.0.z()),
            2.0 * (self.0.x() * self.0.y() - self.0.w() * self.0.z()),
            2.0 * (self.0.x() * self.0.z() + self.0.w() * self.0.y()),
            1.0,
        );

        Matrix4F::new_rotation_from_fur(forward, up, right)
    }

    pub fn dot(&self, rhs: Quaternion) -> f32 {
        self.0.dot(rhs.0)
    }

    pub fn n_lerp(&self, dest: Quaternion, factor: f32, shortest: bool) -> Self {
        const EPISILON: f32 = 1e3;

        let mut cos = self.dot(dest);
        let corrected_dest = if shortest && cos < 0.0 {
            cos = -cos;
            dest * -1.0
        } else {
            dest
        };

        if cos.abs() >= 1.0 - EPISILON {
            return self.n_lerp(corrected_dest, factor, false);
        }

        let sin = (1.0 - cos * cos).sqrt();
        let angle = f32::atan2(sin, cos);
        let sin_inv = 1.0 / sin;

        let src_factor = ((1.0 - factor) * angle).sin() * sin_inv;
        let dest_factor = ((factor) * angle).sin() * sin_inv;

        (*self * src_factor) + (corrected_dest * dest_factor)
    }

    //From Ken Shoemake's "Quaternion Calculus and Fast Animation" article
    pub fn from_rot(rot: &Matrix4F) -> Self {
        let trace = rot[0][0] + rot[1][1] + rot[2][2];

        let raw = if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            Vector4F::new(
                (rot[1][2] - rot[2][1]) * s,
                (rot[2][0] - rot[0][2]) * s,
                (rot[0][1] - rot[1][0]) * s,
                0.25 / s,
            )
        } else {
            if rot[0][0] > rot[1][1] && rot[0][0] > rot[2][2] {
                let s = 2.0 * (1.0 + rot[0][0] - rot[1][1] - rot[2][2]).sqrt();

                Vector4F::new(
                    0.25 * s,
                    (rot[1][0] + rot[0][1]) / s,
                    (rot[2][0] + rot[0][2]) / s,
                    (rot[1][2] - rot[2][1]) / s,
                )
            } else if rot[1][1] > rot[2][2] {
                let s = 2.0 * (1.0 + rot[1][1] - rot[0][0] - rot[2][2]).sqrt();

                Vector4F::new(
                    (rot[1][0] + rot[0][1]) / s,
                    0.25 * s,
                    (rot[2][1] + rot[1][2]) / s,
                    (rot[2][0] - rot[0][2]) / s,
                )
            } else {
                let s = 2.0 * (1.0 + rot[2][2] - rot[0][0] - rot[1][1]).sqrt();

                Vector4F::new(
                    (rot[2][0] + rot[0][2]) / s,
                    (rot[1][2] + rot[2][1]) / s,
                    0.25 * s,
                    (rot[0][1] - rot[1][0]) / s,
                )
            }
        };

        Quaternion(raw.normalized())
    }

    pub fn normalized(&self) -> Self {
        Quaternion(self.0.normalized())
    }

    pub fn forward(&self) -> Vector4F {
        Vector4F::new(0.0, 0.0, 1.0, 1.0).rotate_quaternion(*self)
    }

    pub fn back(&self) -> Vector4F {
        Vector4F::new(0.0, 0.0, -1.0, 1.0).rotate_quaternion(*self)
    }

    pub fn up(&self) -> Vector4F {
        Vector4F::new(0.0, 1.0, 0.0, 1.0).rotate_quaternion(*self)
    }

    pub fn down(&self) -> Vector4F {
        Vector4F::new(0.0, -1.0, 0.0, 1.0).rotate_quaternion(*self)
    }

    pub fn right(&self) -> Vector4F {
        Vector4F::new(1.0, 0.0, 0.0, 1.0).rotate_quaternion(*self)
    }

    pub fn left(&self) -> Vector4F {
        Vector4F::new(-1.0, 0.0, 0.0, 1.0).rotate_quaternion(*self)
    }

    pub fn x(&self) -> f32 {
        self.0.x()
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        self.0.x_mut()
    }

    pub fn y(&self) -> f32 {
        self.0.y()
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        self.0.y_mut()
    }

    pub fn z(&self) -> f32 {
        self.0.z()
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        self.0.z_mut()
    }

    pub fn w(&self) -> f32 {
        self.0.w()
    }

    pub fn w_mut(&mut self) -> &mut f32 {
        self.0.w_mut()
    }
}

// Addition

impl Add for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Self) -> Quaternion {
        Quaternion(self.0 * rhs.0)
    }
}

impl Add<f32> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: f32) -> Self::Output {
        Quaternion(self.0 + rhs)
    }
}

// Subtraction

impl Sub for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Self) -> Self::Output {
        Quaternion(self.0 - rhs.0)
    }
}

impl Sub<f32> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: f32) -> Quaternion {
        Quaternion(self.0 - rhs)
    }
}

// Multiplication

impl Mul for Quaternion {
    type Output = Quaternion;
    #[inline]
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion(Vector4F::new(
            self.0.x() * rhs.w() + self.0.w() * rhs.x() + self.0.y() * rhs.z()
                - self.0.z() * rhs.y(),
            self.0.y() * rhs.w() + self.0.w() * rhs.y() + self.0.z() * rhs.x()
                - self.0.x() * rhs.z(),
            self.0.z() * rhs.w() + self.0.w() * rhs.z() + self.0.x() * rhs.y()
                - self.0.y() * rhs.x(),
            self.0.w() * rhs.w()
                - self.0.x() * rhs.x()
                - self.0.y() * rhs.y()
                - self.0.z() * rhs.z(),
        ))
    }
}

impl Mul<Vector4F> for Quaternion {
    type Output = Quaternion;
    #[inline]
    fn mul(self, rhs: Vector4F) -> Self::Output {
        Quaternion(Vector4F::new(
            self.0.w() * rhs.x() + self.0.y() * rhs.z() - self.0.z() * rhs.y(),
            self.0.w() * rhs.y() + self.0.z() * rhs.x() - self.0.x() * rhs.z(),
            self.0.w() * rhs.z() + self.0.x() * rhs.y() - self.0.y() * rhs.x(),
            -self.0.x() * rhs.x() - self.0.y() * rhs.y() - self.0.z() * rhs.z(),
        ))
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion(self.0 * rhs)
    }
}

// Division

impl Div for Quaternion {
    type Output = Quaternion;
    #[inline]
    fn div(self, rhs: Quaternion) -> Quaternion {
        Quaternion(self.0 / rhs.0)
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;
    #[inline]
    fn div(self, rhs: f32) -> Quaternion {
        Quaternion(self.0 / rhs)
    }
}

// Equals

impl PartialEq for Quaternion {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}
