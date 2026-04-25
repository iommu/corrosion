use std::{mem::swap, usize};

use crate::{
    bitmap::Bitmap,
    edge::Edge,
    gradients::{self, Gradients},
    matrix::Matrix4F,
    mesh::Mesh,
    pixel::Pixel,
    vertex::Vertex,
};

impl Bitmap {
    fn draw_scan_line(&mut self, left: &Edge, right: &Edge, y: usize, texture: &Bitmap) {
        let x_min = left.x().ceil() as i32;
        let x_max = right.x().ceil() as i32;
        let x_pre = x_min as f32 - left.x();

        let x_dist = right.x() - left.x();
        let tex_coord_xx_step = (right.tex_coord_x() - left.tex_coord_x()) / x_dist;
        let tex_coord_yx_step = (right.tex_coord_y() - left.tex_coord_y()) / x_dist;
        let zx_step_inv = (right.z_inv() - left.z_inv()) / x_dist;

        let mut tex_coord_x = left.tex_coord_x() + tex_coord_xx_step * x_pre;
        let mut tex_coord_y = left.tex_coord_y() + tex_coord_yx_step * x_pre;
        let mut z_inv = left.z_inv() + zx_step_inv * x_pre;

        for x in x_min..x_max {
            let z = 1.0 / z_inv;
            let x_src = ((tex_coord_x * z) * (texture.size()[0] - 1) as f32 + 0.5) as usize;
            let y_src = ((tex_coord_y * z) * (texture.size()[1] - 1) as f32 + 0.5) as usize;
            self.copy_pixel(x as usize, y, x_src, y_src, texture);
            //
            tex_coord_x += tex_coord_xx_step;
            tex_coord_y += tex_coord_yx_step;
            z_inv += zx_step_inv;
        }
    }

    fn scan_edges(&mut self, a: &mut Edge, b: &mut Edge, handedness: bool, texture: &Bitmap) {
        let y_start = b.y_start();
        let y_end = b.y_end();

        let [left, right] = match handedness {
            true => [b, a],
            false => [a, b],
        };

        for y in y_start..y_end {
            self.draw_scan_line(left, right, y as usize, texture);
            left.step();
            right.step();
        }
    }

    fn scan_tri(
        &mut self,
        min_y_vert: Vertex,
        mid_y_vert: Vertex,
        max_y_vert: Vertex,
        handedness: bool,
        texture: &Bitmap,
    ) {
        let gradients = Gradients::new(min_y_vert, mid_y_vert, max_y_vert);
        let mut top_to_bot = Edge::new(&gradients, min_y_vert, max_y_vert, 0);
        let mut top_to_mid = Edge::new(&gradients, min_y_vert, mid_y_vert, 0);
        let mut mid_to_bot = Edge::new(&gradients, mid_y_vert, max_y_vert, 1);

        self.scan_edges(&mut top_to_bot, &mut top_to_mid, handedness, texture);
        self.scan_edges(&mut top_to_bot, &mut mid_to_bot, handedness, texture);
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, transform: &Matrix4F, texture: &Bitmap) {
        for idx in (0..mesh.indices().len()).step_by(3) {
            self.fill_tri(
                mesh.vertices()[mesh.indices()[idx + 0] as usize].transform(*transform),
                mesh.vertices()[mesh.indices()[idx + 1] as usize].transform(*transform),
                mesh.vertices()[mesh.indices()[idx + 2] as usize].transform(*transform),
                texture,
            )
        }
    }

    pub fn fill_tri(
        &mut self, 
        mut vert_1: Vertex,
        mut vert_2: Vertex,
        mut vert_3: Vertex,
        texture: &Bitmap,
    ) {
        let ss_transform =
            Matrix4F::new_ss_transform(self.width() as f32 / 2.0, self.height() as f32 / 2.0);
        let min_y_vert = &mut vert_1.transform(ss_transform).perspective_div();
        let mid_y_vert = &mut vert_2.transform(ss_transform).perspective_div();
        let max_y_vert = &mut vert_3.transform(ss_transform).perspective_div();

        if min_y_vert.tri_area(max_y_vert, mid_y_vert) >= 0.0 {
            return;
        }

        if max_y_vert.y() < mid_y_vert.y() {
            swap(max_y_vert, mid_y_vert);
        }

        if mid_y_vert.y() < min_y_vert.y() {
            swap(mid_y_vert, min_y_vert);
        }

        if max_y_vert.y() < mid_y_vert.y() {
            swap(max_y_vert, mid_y_vert);
        }

        let area: f32 = min_y_vert.tri_area(max_y_vert, mid_y_vert);
        let handedness = if area >= 0.0 { true } else { false };

        self.scan_tri(*min_y_vert, *mid_y_vert, *max_y_vert, handedness, texture);
    }
}
