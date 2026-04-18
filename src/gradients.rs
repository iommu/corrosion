use std::ops;

use crate::{vector::Vector4F, vertex::Vertex};

#[derive(Clone)]
pub struct Gradients {
    color: Vec<Vector4F>,
    color_x_step: Vector4F,
    color_y_step: Vector4F,
}

impl Gradients {
    pub fn new(min_y_vert: Vertex, mid_y_vert: Vertex, max_y_vert: Vertex) -> Self {
        let color = vec![min_y_vert.color(), mid_y_vert.color(), max_y_vert.color()];

        let one_over_dx = 1.0
            / (((mid_y_vert.x() - max_y_vert.x())
                * (min_y_vert.y() - max_y_vert.y()))
                - ((min_y_vert.x() - max_y_vert.x())
                    * (mid_y_vert.y() - max_y_vert.y())));

        let one_over_dy = -one_over_dx;

        let d_c_x = ((color[1] - color[2]) * (min_y_vert.y() - max_y_vert.y()))
            - ((color[0] - color[2]) * (mid_y_vert.y() - max_y_vert.y()));

            let d_c_y = ((color[1] - color[2]) * (min_y_vert.x() - max_y_vert.x()))
            - ((color[0] - color[2]) * (mid_y_vert.x() - max_y_vert.x()));
        Self {
            color,
            color_x_step: d_c_x * one_over_dx,
            color_y_step: d_c_y * one_over_dy,
        }
    }

    pub fn color(&self) -> &Vec<Vector4F> {
        &self.color
    }

    pub fn color_x_step(&self) -> Vector4F {
        self.color_x_step
    }

    pub fn color_y_step(&self) -> Vector4F {
        self.color_y_step
    }
}
