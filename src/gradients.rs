use crate::{vertex::Vertex};

#[derive(Clone)]
pub struct Gradients {
    pub tex_coords_x: Vec<f32>,
    pub tex_coords_y: Vec<f32>,
    pub tex_coord_xx_step: f32,
    pub tex_coord_xy_step: f32,
    pub tex_coord_yx_step: f32,
    pub tex_coord_yy_step: f32,
}

impl Gradients {
    pub fn new(min_y_vert: Vertex, mid_y_vert: Vertex, max_y_vert: Vertex) -> Self {
        let one_over_dx = 1.0
            / (((mid_y_vert.x() - max_y_vert.x()) * (min_y_vert.y() - max_y_vert.y()))
                - ((min_y_vert.x() - max_y_vert.x()) * (mid_y_vert.y() - max_y_vert.y())));

        let one_over_dy = -one_over_dx;

        let tex_coords_x = vec![min_y_vert.tex_coords().x(), mid_y_vert.tex_coords().x(), max_y_vert.tex_coords().x()];
        let tex_coords_y = vec![min_y_vert.tex_coords().y(), mid_y_vert.tex_coords().y(), max_y_vert.tex_coords().y()];

        let tex_coord_xx_step = (((tex_coords_x[1] - tex_coords_x[2])
            * (min_y_vert.y() - max_y_vert.y()))
            - ((tex_coords_x[0] - tex_coords_x[2]) * (mid_y_vert.y() - max_y_vert.y())))
            * one_over_dx;

        let tex_coord_xy_step = (((tex_coords_x[1] - tex_coords_x[2])
            * (min_y_vert.x() - max_y_vert.x()))
            - ((tex_coords_x[0] - tex_coords_x[2]) * (mid_y_vert.x() - max_y_vert.x())))
            * one_over_dy;

        let tex_coord_yx_step = (((tex_coords_y[1] - tex_coords_y[2])
            * (min_y_vert.y() - max_y_vert.y()))
            - ((tex_coords_y[0] - tex_coords_y[2]) * (mid_y_vert.y() - max_y_vert.y())))
            * one_over_dx;

        let tex_coord_yy_step = (((tex_coords_y[1] - tex_coords_y[2])
            * (min_y_vert.x() - max_y_vert.x()))
            - ((tex_coords_y[0] - tex_coords_y[2]) * (mid_y_vert.x() - max_y_vert.x())))
            * one_over_dy;

        Self {
            tex_coords_x,
            tex_coords_y,
            tex_coord_xx_step,
            tex_coord_xy_step,
            tex_coord_yx_step,
            tex_coord_yy_step,
        }
    }
}
