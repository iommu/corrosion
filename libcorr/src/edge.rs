use crate::{
    gradients::{Gradient, Gradients},
    vertex::Vertex,
};

use getset::Getters;

impl Gradient {
    pub fn initial(&self, min_y_vert_idx: usize, x_pre: f32, y_pre: f32) -> f32 {
        self.values()[min_y_vert_idx] + self.x_step() * x_pre + self.y_step() * y_pre
    }

    pub fn step(&self, x_step: f32) -> f32 {
        self.y_step() + self.x_step() * x_step
    }
}

#[derive(Clone, Copy, Getters)]
pub struct Edge {
    #[getset(get = "pub")]
    x: f32,
    x_step: f32,
    #[getset(get = "pub")]
    y_start: i32,
    #[getset(get = "pub")]
    y_end: i32,
    #[getset(get = "pub")]
    tex_coord_x: f32,
    tex_coord_x_step: f32,
    #[getset(get = "pub")]
    tex_coord_y: f32,
    tex_coord_y_step: f32,
    #[getset(get = "pub")]
    z_inv: f32,
    z_inv_step: f32,
    #[getset(get = "pub")]
    depth: f32,
    depth_step: f32,
    #[getset(get = "pub")]
    light_amount: f32,
    light_amount_step: f32,
}

impl Edge {
    pub fn new(
        gradients: &Gradients,
        min_y_vert: &Vertex,
        max_y_vert: &Vertex,
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

        let tex_coord_x = gradients
            .tex_coord_x()
            .initial(min_y_vert_idx, x_pre, y_pre);
        let tex_coord_y = gradients
            .tex_coord_y()
            .initial(min_y_vert_idx, x_pre, y_pre);

        let tex_coord_x_step =
            gradients.tex_coord_x().y_step() + gradients.tex_coord_x().x_step() * x_step;
        let tex_coord_y_step =
            gradients.tex_coord_y().y_step() + gradients.tex_coord_y().x_step() * x_step;

        let z_inv = gradients.z_inv().initial(min_y_vert_idx, x_pre, y_pre);
        let z_inv_step = gradients.z_inv().step(x_step);

        let depth = gradients.depth().initial(min_y_vert_idx, x_pre, y_pre);
        let depth_step = gradients.depth().step(x_step);

        let light_amount = gradients
            .light_amount()
            .initial(min_y_vert_idx, x_pre, y_pre);
        let light_amount_step = gradients.light_amount().step(x_step);

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
            z_inv_step,
            depth,
            depth_step,
            light_amount,
            light_amount_step,
        }
    }

    pub fn step(&mut self) {
        self.x += self.x_step;
        self.tex_coord_x += self.tex_coord_x_step;
        self.tex_coord_y += self.tex_coord_y_step;
        self.z_inv += self.z_inv_step;
        self.depth += self.depth_step;
        self.light_amount += self.light_amount_step;
    }
}
