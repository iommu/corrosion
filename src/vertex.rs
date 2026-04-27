use getset::{Getters, MutGetters};
use std::ops::{Index, IndexMut};

use crate::{matrix::Matrix4F, vector::Vector4F};

#[derive(Clone, Copy, Getters, MutGetters)]
pub struct Vertex {
    #[getset(get = "pub", get_mut = "pub")]
    pos: Vector4F,
    #[getset(get = "pub", get_mut = "pub")]
    tex_coords: Vector4F,
    #[getset(get = "pub", get_mut = "pub")]
    normal: Vector4F,
}

impl Vertex {
    pub fn new(pos: Vector4F, tex_coords: Vector4F, normal: Vector4F) -> Self {
        Self {
            pos,
            tex_coords,
            normal,
        }
    }

    pub fn lerp(&self, rhs: Self, factor: f32) -> Self {
        Self {
            pos: self.pos.lerp(rhs.pos, factor),
            tex_coords: self.tex_coords.lerp(rhs.tex_coords, factor),
            normal: self.normal.lerp(*rhs.normal(), factor),
        }
    }

    pub fn x(&self) -> f32 {
        self.pos.x()
    }

    pub fn y(&self) -> f32 {
        self.pos.y()
    }

    pub fn z(&self) -> f32 {
        self.pos.z()
    }

    pub fn w(&self) -> f32 {
        self.pos.w()
    }

    pub fn transform(&self, transform: &Matrix4F, normal_transform: &Matrix4F) -> Self {
        // Normaliztion is important for scaling
        Self::new(
            transform.transform(self.pos),
            self.tex_coords,
            normal_transform.transform(self.normal).normalized(),
        )
    }

    pub fn perspective_div(&self) -> Self {
        Self::new(
            Vector4F::new(
                self.pos.x() / self.pos.w(),
                self.pos.y() / self.pos.w(),
                self.pos.z() / self.pos.w(),
                self.pos.w(), // sneaky storage
            ),
            self.tex_coords,
            self.normal,
        )
    }

    pub fn tri_area(&self, vert_b: &Vertex, vert_c: &Vertex) -> f32 {
        let x_1 = vert_b.pos.x() - self.pos.x();
        let y_1 = vert_b.pos.y() - self.pos.y();
        let x_2 = vert_c.pos.x() - self.pos.x();
        let y_2 = vert_c.pos.y() - self.pos.y();

        // 2D cross product
        return ((x_1 * y_2) - (x_2 * y_1)) / 2.0;
    }

    pub fn is_inside_view_frustum(&self) -> bool {
        (self.pos.x().abs() <= self.pos.w().abs())
            && (self.pos.y().abs() <= self.pos.w().abs())
            && (self.pos.z().abs() <= self.pos.w().abs())
    }
}

// Indexing

impl Index<usize> for Vertex {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.pos[index]
    }
}

impl IndexMut<usize> for Vertex {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pos[index]
    }
}
