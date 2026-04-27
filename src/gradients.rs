use crate::{vector::Vector4F, vertex::Vertex};

fn saturate(val : f32) -> f32 {
    val.max(0.0).min(1.0)
}

#[derive(Clone)]
pub struct Gradients {
    pub tex_coords_x: Vec<f32>,
    pub tex_coords_y: Vec<f32>,
    pub z_inv: Vec<f32>,
    pub depth: Vec<f32>,
    pub light_amount : Vec<f32>,
    pub tex_coord_xx_step: f32,
    pub tex_coord_xy_step: f32,
    pub tex_coord_yx_step: f32,
    pub tex_coord_yy_step: f32,
    pub zx_step_inv: f32,
    pub zy_step_inv: f32,
    pub depth_x_step: f32,
    pub depth_y_step: f32,
    pub light_amount_x_step : f32,
    pub light_amount_y_step : f32,
}

impl Gradients {
    pub fn new(min_y_vert: Vertex, mid_y_vert: Vertex, max_y_vert: Vertex) -> Self {
        let dx_inv = 1.0
            / (((mid_y_vert.x() - max_y_vert.x()) * (min_y_vert.y() - max_y_vert.y()))
                - ((min_y_vert.x() - max_y_vert.x()) * (mid_y_vert.y() - max_y_vert.y())));

        let dy_inv = -dx_inv;

        let z_inv = vec![
            1.0 / min_y_vert.pos().w(),
            1.0 / mid_y_vert.pos().w(),
            1.0 / max_y_vert.pos().w(),
        ];
        let tex_coords_x = vec![
            min_y_vert.tex_coords().x() * z_inv[0],
            mid_y_vert.tex_coords().x() * z_inv[1],
            max_y_vert.tex_coords().x() * z_inv[2],
        ];
        let tex_coords_y = vec![
            min_y_vert.tex_coords().y() * z_inv[0],
            mid_y_vert.tex_coords().y() * z_inv[1],
            max_y_vert.tex_coords().y() * z_inv[2],
        ];

        let light_dir = Vector4F::new(0.0, 0.0, 1.0, 1.0);
        let light_amount = vec![    
            saturate(min_y_vert.normal().dot(light_dir)) * 0.9 + 0.1,
            saturate(mid_y_vert.normal().dot(light_dir)) * 0.9 + 0.1,
            saturate(max_y_vert.normal().dot(light_dir)) * 0.9 + 0.1,
        ];

        println!("normal {:?}", (min_y_vert.normal().dot(light_dir)));

        let depth = vec![min_y_vert.pos().z(), mid_y_vert.pos().z(), max_y_vert.pos().z()];

        let tex_coord_xx_step =
            Self::calc_x_step(&tex_coords_x, &min_y_vert, &mid_y_vert, &max_y_vert, dx_inv);
        let tex_coord_xy_step =
            Self::calc_y_step(&tex_coords_x, &min_y_vert, &mid_y_vert, &max_y_vert, dy_inv);

        let tex_coord_yx_step =
            Self::calc_x_step(&tex_coords_y, &min_y_vert, &mid_y_vert, &max_y_vert, dx_inv);
        let tex_coord_yy_step =
            Self::calc_y_step(&tex_coords_y, &min_y_vert, &mid_y_vert, &max_y_vert, dy_inv);

        let zx_step_inv = Self::calc_x_step(&z_inv, &min_y_vert, &mid_y_vert, &max_y_vert, dx_inv);
        let zy_step_inv = Self::calc_y_step(&z_inv, &min_y_vert, &mid_y_vert, &max_y_vert, dy_inv);

        let depth_x_step = Self::calc_x_step(&depth, &min_y_vert, &mid_y_vert, &max_y_vert, dx_inv);
        let depth_y_step = Self::calc_y_step(&depth, &min_y_vert, &mid_y_vert, &max_y_vert, dy_inv);

        let light_amount_x_step = Self::calc_x_step(&light_amount, &min_y_vert, &mid_y_vert, &max_y_vert, dx_inv);
        let light_amount_y_step = Self::calc_y_step(&light_amount, &min_y_vert, &mid_y_vert, &max_y_vert, dy_inv);

        Self {
            tex_coords_x,
            tex_coords_y,
            z_inv,
            tex_coord_xx_step,
            tex_coord_xy_step,
            tex_coord_yx_step,
            tex_coord_yy_step,
            zx_step_inv,
            zy_step_inv,
            depth,
            depth_x_step,
            depth_y_step,
            light_amount,
            light_amount_x_step,
            light_amount_y_step,
        }
    }

    fn calc_x_step(
        values: &Vec<f32>,
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
        values: &Vec<f32>,
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
