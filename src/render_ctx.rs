use std::{mem::swap, usize, vec};

use crate::{
    bitmap::Bitmap, edge::Edge, gradients::Gradients, matrix::Matrix4F, mesh::Mesh, vertex::Vertex,
};

pub fn gen_buffer(buffer: &Bitmap) -> Vec<f32> {
    vec![0.0; buffer.size()[0] * buffer.size()[1]]
}

pub fn clear_buffer(z_buffer: &mut Vec<f32>) {
    z_buffer.fill(f32::MAX);
}

impl Bitmap {
    pub fn draw_mesh(
        &mut self,
        mesh: &Mesh,
        view_projection: &Matrix4F,
        transform: &Matrix4F,
        texture: &Bitmap,
        z_buffer: &mut Vec<f32>,
    ) {
        let mvp = *view_projection * *transform;
        for chunk in mesh.indices().chunks_exact(3) {
            self.draw_tri(
                &mesh.vertices()[chunk[0] as usize].transform(&mvp, transform),
                &mesh.vertices()[chunk[1] as usize].transform(&mvp, transform),
                &mesh.vertices()[chunk[2] as usize].transform(&mvp, transform),
                texture,
                z_buffer,
            );
        }
    }

    fn draw_scan_line(
        &mut self,
        left: &Edge,
        right: &Edge,
        y: usize,
        gradients: &Gradients,
        texture: &Bitmap,
        z_buffer: &mut Vec<f32>,
    ) {
        let x_min = left.x().ceil() as i32;
        let x_max = right.x().ceil() as i32;
        let x_pre = x_min as f32 - left.x();

        let tex_coord_xx_step = gradients.tex_coord_xx_step;
        let tex_coord_yx_step = gradients.tex_coord_yx_step;
        let zx_step_inv = gradients.zx_step_inv;
        let depth_x_step = gradients.depth_x_step;
        let light_amount_step = gradients.light_amount_x_step * x_pre;

        let mut tex_coord_x = left.tex_coord_x() + tex_coord_xx_step * x_pre;
        let mut tex_coord_y = left.tex_coord_y() + tex_coord_yx_step * x_pre;
        let mut z_inv = left.z_inv() + zx_step_inv * x_pre;
        let mut depth = left.depth() + depth_x_step * x_pre;
        let mut light_amount = left.light_amount() + light_amount_step * x_pre;

        for x in x_min..x_max {
            let index = x as usize + y * self.size()[0];
            if depth < z_buffer[index] {
                z_buffer[index] = depth;
                let z = 1.0 / z_inv;
                let x_src = ((tex_coord_x * z) * (texture.size()[0] - 1) as f32 + 0.5) as usize;
                let y_src = ((tex_coord_y * z) * (texture.size()[1] - 1) as f32 + 0.5) as usize;
                self.copy_pixel(x as usize, y, x_src, y_src, texture, light_amount);
            }

            //
            tex_coord_x += tex_coord_xx_step;
            tex_coord_y += tex_coord_yx_step;
            z_inv += zx_step_inv;
            depth += depth_x_step;
            light_amount += light_amount_step;
        }
    }

    fn scan_edges(
        &mut self,
        a: &mut Edge,
        b: &mut Edge,
        handedness: bool,
        gradients: &Gradients,
        texture: &Bitmap,
        z_buffer: &mut Vec<f32>,
    ) {
        let y_start = b.y_start();
        let y_end = b.y_end();

        let [left, right] = match handedness {
            true => [b, a],
            false => [a, b],
        };

        for y in y_start..y_end {
            self.draw_scan_line(left, right, y as usize, gradients, texture, z_buffer);
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
        z_buffer: &mut Vec<f32>,
    ) {
        let gradients = Gradients::new(min_y_vert, mid_y_vert, max_y_vert);
        let mut top_to_bot = Edge::new(&gradients, min_y_vert, max_y_vert, 0);
        let mut top_to_mid = Edge::new(&gradients, min_y_vert, mid_y_vert, 0);
        let mut mid_to_bot = Edge::new(&gradients, mid_y_vert, max_y_vert, 1);

        self.scan_edges(
            &mut top_to_bot,
            &mut top_to_mid,
            handedness,
            &gradients,
            texture,
            z_buffer,
        );
        self.scan_edges(
            &mut top_to_bot,
            &mut mid_to_bot,
            handedness,
            &gradients,
            texture,
            z_buffer,
        );
    }

    pub fn draw_tri(
        &mut self,
        vert_1: &Vertex,
        vert_2: &Vertex,
        vert_3: &Vertex,
        texture: &Bitmap,
        z_buffer: &mut Vec<f32>,
    ) {
        let v_1_inside = vert_1.is_inside_view_frustum();
        let v_2_inside = vert_2.is_inside_view_frustum();
        let v_3_inside = vert_3.is_inside_view_frustum();

        if v_1_inside && v_2_inside && v_3_inside {
            self.fill_tri(vert_1, vert_2, vert_3, texture, z_buffer);
            return;
        }

        if !v_1_inside && !v_2_inside && !v_3_inside {
            return;
        }

        let mut vertices = vec![*vert_1, *vert_2, *vert_3];
        let mut aux_list = vec![*vert_1; 0];

        if Self::clip_poly_axis(&mut vertices, &mut aux_list, 0)
            && Self::clip_poly_axis(&mut vertices, &mut aux_list, 1)
            && Self::clip_poly_axis(&mut vertices, &mut aux_list, 2)
        {
            let initial_vert = vertices[0];
            for index in 1..vertices.len() - 1 {
                self.fill_tri(
                    &initial_vert,
                    &vertices[index],
                    &vertices[index + 1],
                    texture,
                    z_buffer,
                );
            }
        }
    }

    fn clip_poly_axis(
        vertices: &mut Vec<Vertex>,
        aux_list: &mut Vec<Vertex>,
        component_index: usize,
    ) -> bool {
        Self::clip_poly_component(vertices, component_index, 1.0, aux_list);
        vertices.clear();

        if aux_list.is_empty() {
            return false;
        }

        Self::clip_poly_component(aux_list, component_index, -1.0, vertices);
        aux_list.clear();

        return !vertices.is_empty();
    }

    fn clip_poly_component(
        vertices: &Vec<Vertex>,
        component_index: usize,
        component_factor: f32,
        result: &mut Vec<Vertex>,
    ) {
        let mut prev_vert = &vertices[vertices.len() - 1];
        let mut prev_comp = prev_vert[component_index] * component_factor;
        let mut prev_inside = prev_comp <= prev_vert.pos().w();

        for curr_vert in vertices {
            let curr_comp = curr_vert[component_index] * component_factor;
            let curr_inside = curr_comp <= curr_vert.pos().w();

            if curr_inside ^ prev_inside {
                let lerp_factor = (prev_vert.pos().w() - prev_comp)
                    / ((prev_vert.pos().w() - prev_comp) - (curr_vert.pos().w() - curr_comp));

                result.push(prev_vert.lerp(*curr_vert, lerp_factor));
            }

            if curr_inside {
                result.push(*curr_vert);
            }

            prev_vert = curr_vert;
            prev_comp = curr_comp;
            prev_inside = curr_inside;
        }
    }

    fn fill_tri(
        &mut self,
        vert_1: &Vertex,
        vert_2: &Vertex,
        vert_3: &Vertex,
        texture: &Bitmap,
        z_buffer: &mut Vec<f32>,
    ) {
        let ss_transform =
            Matrix4F::new_ss_transform(self.width() as f32 / 2.0, self.height() as f32 / 2.0);

        let identity = Matrix4F::new_identity();

        let min_y_vert = &mut vert_1.transform(&ss_transform, &identity).perspective_div();
        let mid_y_vert = &mut vert_2.transform(&ss_transform, &identity).perspective_div();
        let max_y_vert = &mut vert_3.transform(&ss_transform, &identity).perspective_div();

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

        self.scan_tri(
            *min_y_vert,
            *mid_y_vert,
            *max_y_vert,
            handedness,
            texture,
            z_buffer,
        );
    }
}
