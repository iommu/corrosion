use crate::{gradients::Gradients, vector::Vector4F, vertex::Vertex};

#[derive(Clone, Copy)]
pub struct Edge {
    x: f32,
    x_step: f32,
    y_start: i32,
    y_end: i32,
    color: Vector4F,
    color_step: Vector4F,
}

impl Edge {
    pub fn new(
        gradients: &Gradients,
        min_y_vert: Vertex,
        max_y_vert: Vertex,
        min_y_vert_idx: usize,
    ) -> Self {
        let y_dist = max_y_vert.y() - min_y_vert.y();
        let x_dist = max_y_vert.x() - min_y_vert.x();

        let y_pre = min_y_vert.y().ceil() - min_y_vert.y();
        let x_step = x_dist / y_dist;

        let x = min_y_vert.x() + y_pre * x_step;
        let x_pre = x - min_y_vert.x();

        Self {
            x,
            x_step: x_step,
            y_start: min_y_vert.y().ceil() as i32,
            y_end: max_y_vert.y().ceil() as i32,
            color: gradients.color()[min_y_vert_idx] + gradients.color_y_step() * y_pre + gradients.color_x_step() * x_pre,
            color_step : gradients.color_y_step() + gradients.color_x_step() * x_step,
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

    pub fn step(&mut self) {
        self.x += self.x_step;
        self.color = self.color + self.color_step
    }

    pub fn color(&self) -> Vector4F {
        self.color
    }

    pub fn color_step(&self) -> Vector4F {
        self.color_step
    }
}
