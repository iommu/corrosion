use crate::{matrix::Matrix4F, vector::Vector4F};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vector4F,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector4F::new(x, y, 0.0, 1.0),
        }
    }

    pub fn x(&self) -> f32 {
        self.pos.x()
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        self.pos.x_mut()
    }

    pub fn y(&self) -> f32 {
        self.pos.y()
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        self.pos.y_mut()
    }

    pub fn new_tranfrom(&self, transform: Matrix4F) -> Vertex {
        Vertex {
            pos: transform.transform(self.pos),
        }
    }

    pub fn tri_area(&self, vert_b: &Vertex, vert_c: &Vertex) -> f32 {
        let x_1 = vert_b.x() - self.x();
        let y_1 = vert_b.y() - self.y();
        let x_2 = vert_c.x() - self.x();
        let y_2 = vert_c.y() - self.y();

        // 2D cross product
        return ((x_1 * y_2) - (x_2 * y_1)) / 2.0;
    }
}
