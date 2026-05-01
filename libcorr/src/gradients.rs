use std::ops::Index;

use crate::{lightsource::LightSource, vector::Vector4F, vertex::Vertex};

use getset::Getters;

fn saturate(val: f32) -> f32 {
    val.max(0.0).min(1.0)
}

#[derive(Clone, Copy, Getters)]
pub struct Gradient {
    #[getset(get = "pub")]
    values: [f32; 3],
    #[getset(get = "pub")]
    x_step: f32,
    #[getset(get = "pub")]
    y_step: f32,
}

impl Gradient {
    pub fn new(
        values: [f32; 3],
        min_y_vert: &Vertex,
        mid_y_vert: &Vertex,
        max_y_vert: &Vertex,
        dx_inv: f32,
        dy_inv: f32,
    ) -> Self {
        let x_step = Self::calc_x_step(&values, min_y_vert, mid_y_vert, max_y_vert, dx_inv);
        let y_step = Self::calc_y_step(&values, min_y_vert, mid_y_vert, max_y_vert, dy_inv);
        Self {
            values,
            x_step,
            y_step,
        }
    }

    fn calc_x_step(
        values: &[f32; 3],
        min_y_vert: &Vertex,
        mid_y_vert: &Vertex,
        max_y_vert: &Vertex,
        dx_inv: f32,
    ) -> f32 {
        (((values[1] - values[2]) * (min_y_vert.y() - max_y_vert.y()))
            - ((values[0] - values[2]) * (mid_y_vert.y() - max_y_vert.y())))
            * dx_inv
    }

    fn calc_y_step(
        values: &[f32; 3],
        min_y_vert: &Vertex,
        mid_y_vert: &Vertex,
        max_y_vert: &Vertex,
        dy_inv: f32,
    ) -> f32 {
        (((values[1] - values[2]) * (min_y_vert.x() - max_y_vert.x()))
            - ((values[0] - values[2]) * (mid_y_vert.x() - max_y_vert.x())))
            * dy_inv
    }
}

// Indexing

impl Index<usize> for Gradient {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

#[derive(Clone, Copy, Getters)]
pub struct Gradients {
    #[getset(get = "pub")]
    tex_coord_x: Gradient,
    #[getset(get = "pub")]
    tex_coord_y: Gradient,
    #[getset(get = "pub")]
    z_inv: Gradient,
    #[getset(get = "pub")]
    depth: Gradient,
    #[getset(get = "pub")]
    light_amount: Gradient,
}

impl Gradients {
    pub fn new(
        min_y_vert: &Vertex,
        mid_y_vert: &Vertex,
        max_y_vert: &Vertex,
        light: &LightSource,
    ) -> Self {
        let dx_inv = 1.0
            / (((mid_y_vert.x() - max_y_vert.x()) * (min_y_vert.y() - max_y_vert.y()))
                - ((min_y_vert.x() - max_y_vert.x()) * (mid_y_vert.y() - max_y_vert.y())));

        let dy_inv = -dx_inv;

        let z_inv = [
            1.0 / min_y_vert.pos().w(),
            1.0 / mid_y_vert.pos().w(),
            1.0 / max_y_vert.pos().w(),
        ];

        let tex_coord_x = Gradient::new(
            [
                min_y_vert.tex_coords().x() * z_inv[0],
                mid_y_vert.tex_coords().x() * z_inv[1],
                max_y_vert.tex_coords().x() * z_inv[2],
            ],
            &min_y_vert,
            &mid_y_vert,
            &max_y_vert,
            dx_inv,
            dy_inv,
        );

        let tex_coord_y = Gradient::new(
            [
                min_y_vert.tex_coords().y() * z_inv[0],
                mid_y_vert.tex_coords().y() * z_inv[1],
                max_y_vert.tex_coords().y() * z_inv[2],
            ],
            &min_y_vert,
            &mid_y_vert,
            &max_y_vert,
            dx_inv,
            dy_inv,
        );

        let z_inv = Gradient::new(z_inv, &min_y_vert, &mid_y_vert, &max_y_vert, dx_inv, dy_inv);

        let depth = Gradient::new(
            [
                min_y_vert.pos().z(),
                mid_y_vert.pos().z(),
                max_y_vert.pos().z(),
            ],
            &min_y_vert,
            &mid_y_vert,
            &max_y_vert,
            dx_inv,
            dy_inv,
        );

        let light_amount = Gradient::new(
            [
                saturate(min_y_vert.normal().dot(*light.direction())) * 0.9 + 0.1,
                saturate(mid_y_vert.normal().dot(*light.direction())) * 0.9 + 0.1,
                saturate(max_y_vert.normal().dot(*light.direction())) * 0.9 + 0.1,
            ],
            &min_y_vert,
            &mid_y_vert,
            &max_y_vert,
            dx_inv,
            dy_inv,
        );

        Self {
            tex_coord_x,
            tex_coord_y,
            z_inv,
            depth,
            light_amount,
        }
    }
}
