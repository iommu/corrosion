use crate::vector::Vector4F;

#[derive(Default, Clone)]
pub struct IndexedModel {
    positions: Vec<Vector4F>,
    tex_coords: Vec<Vector4F>,
    normals: Vec<Vector4F>,
    tangents: Vec<Vector4F>,
    indices: Vec<i32>,
}

impl IndexedModel {
    pub fn positions(&self) -> &Vec<Vector4F> {
        &self.positions
    }

    pub fn positions_mut(&mut self) -> &mut Vec<Vector4F> {
        &mut self.positions
    }

    pub fn tex_coords(&self) -> &Vec<Vector4F> {
        &self.tex_coords
    }

    pub fn tex_coords_mut(&mut self) -> &mut Vec<Vector4F> {
        &mut self.tex_coords
    }

    pub fn normals(&self) -> &Vec<Vector4F> {
        &self.normals
    }

     pub fn normals_mut(&mut self) -> &mut Vec<Vector4F> {
        &mut self.normals
    }

    pub fn tangents(&self) -> &Vec<Vector4F> {
        &self.tangents
    }

    pub fn tangents_mut(&mut self) -> &mut Vec<Vector4F> {
        &mut self.tangents
    }

    pub fn indices(&self) -> &Vec<i32> {
        &self.indices
    }

    pub fn indices_mut(&mut self) -> &mut Vec<i32> {
        &mut self.indices
    }

    pub fn calc_normals(&mut self) {
        for idx in (0..self.indices.len()).step_by(3) {
            let i_0 = self.indices[idx + 0] as usize;
            let i_1 = self.indices[idx + 1] as usize;
            let i_2 = self.indices[idx + 2] as usize;

            let v_1 = self.positions[i_1] - self.positions[i_0];
            let v_2 = self.positions[i_2] - self.positions[i_0];

            let normal = v_1.cross(v_2).normalized();

            self.normals[i_0] = self.normals[i_0] + normal;
            self.normals[i_1] = self.normals[i_1] + normal;
            self.normals[i_2] = self.normals[i_2] + normal;
        }

        for idx in 0..self.normals.len() {
            self.normals[idx] = self.normals[idx].normalized();
        }
    }

    pub fn calc_tangents(&mut self) {
        for idx in (0..self.indices.len()).step_by(3) {
            let i_0 = self.indices[idx + 0] as usize;
            let i_1 = self.indices[idx + 1] as usize;
            let i_2 = self.indices[idx + 2] as usize;

            let edge_1 = self.positions[i_1] - self.positions[i_0];
            let edge_2 = self.positions[i_2] - self.positions[i_0];

            let delta_U_1 = self.tex_coords[i_1].x() - self.tex_coords[i_0].x();
            let delta_V_1 = self.tex_coords[i_1].y() - self.tex_coords[i_0].y();
            let delta_U_2 = self.tex_coords[i_2].x() - self.tex_coords[i_0].x();
            let delta_V_2 = self.tex_coords[i_2].y() - self.tex_coords[i_0].y();

            let dividend = (delta_U_1 * delta_V_2 - delta_U_2 * delta_V_1);
            let f = if dividend == 0.0 { 0.0 } else { 1.0 / dividend };

            let tangent = Vector4F::new(
                f * (delta_V_2 * edge_1.x() - delta_V_1 * edge_2.x()),
                f * (delta_V_2 * edge_1.y() - delta_V_1 * edge_2.y()),
                f * (delta_V_2 * edge_1.z() - delta_V_1 * edge_2.z()),
                0.0,
            );

            self.tangents[i_0] = self.tangents[i_0] + tangent;
            self.tangents[i_1] = self.tangents[i_1] + tangent;
            self.tangents[i_2] = self.tangents[i_2] + tangent;
        }

        for idx in 0..self.tangents.len() {
            self.tangents[idx] = self.tangents[idx].normalized();
        }
    }
}
