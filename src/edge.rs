use crate::{gradients::Gradients, vector::Vector4F, vertex::Vertex};

#[derive(Clone, Copy)]
pub struct Edge {
    x: f32,
    x_step: f32,
    y_start: i32,
    y_end: i32,
    tex_coord_x: f32,
    tex_coord_x_step: f32,
    tex_coord_y: f32,
    tex_coord_y_step: f32,
    z_inv: f32,
    z_step_inv: f32,
}

impl Edge {
    pub fn new(
        gradients: &Gradients,
        min_y_vert: Vertex,
        max_y_vert: Vertex,
        min_y_vert_idx: usize,
    ) -> Self {
        let y_start = min_y_vert.y().ceil() as i32;
        let y_end = max_y_vert.y().ceil() as i32;

        let y_dist = max_y_vert.y() - min_y_vert.y();
        let x_dist = max_y_vert.x() - min_y_vert.x();

        let y_pre = min_y_vert.y().ceil() - min_y_vert.y();
        let x_step = x_dist / y_dist;

        let x = min_y_vert.x() + y_pre * x_step;
        let x_pre = x - min_y_vert.x();

        let tex_coord_x = gradients.tex_coords_x[min_y_vert_idx]
            + gradients.tex_coord_xx_step * x_pre
            + gradients.tex_coord_xy_step * y_pre;
        let tex_coord_y = gradients.tex_coords_y[min_y_vert_idx]
            + gradients.tex_coord_yx_step * x_pre
            + gradients.tex_coord_yy_step * y_pre;

        let tex_coord_x_step = gradients.tex_coord_xy_step + gradients.tex_coord_xx_step * x_step;
        let tex_coord_y_step = gradients.tex_coord_yy_step + gradients.tex_coord_yx_step * x_step;

        let z_inv = gradients.z_inv[min_y_vert_idx]
            + gradients.zx_step_inv * x_pre
            + gradients.zy_step_inv * y_pre;
        let z_step_inv = gradients.zy_step_inv + gradients.zx_step_inv * x_step;

        Self {
            x,
            x_step,
            y_start,
            y_end,
            tex_coord_x,
            tex_coord_x_step,
            tex_coord_y,
            tex_coord_y_step,
            z_inv,
            z_step_inv,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn x_step(&self) -> f32 {
        self.x_step
    }

    pub fn y_start(&self) -> i32 {
        self.y_start
    }

    pub fn y_end(&self) -> i32 {
        self.y_end
    }

    pub fn tex_coord_x(&self) -> f32 {
        self.tex_coord_x
    }

    pub fn tex_coord_y(&self) -> f32 {
        self.tex_coord_y
    }

    pub fn z_inv(&self) -> f32 {
        self.z_inv
    }

    pub fn step(&mut self) {
        self.x += self.x_step;
        self.tex_coord_x += self.tex_coord_x_step;
        self.tex_coord_y += self.tex_coord_y_step;
        self.z_inv += self.z_step_inv
    }
}
