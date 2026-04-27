use crate::vector::Vector4F;
use getset::{Getters, MutGetters};

#[derive(Default, Clone, Getters, MutGetters)]
pub struct IndexedModel {
    #[getset(get = "pub", get_mut = "pub")]
    positions: Vec<Vector4F>,
    #[getset(get = "pub", get_mut = "pub")]
    tex_coords: Vec<Vector4F>,
    #[getset(get = "pub", get_mut = "pub")]
    normals: Vec<Vector4F>,
    #[getset(get = "pub", get_mut = "pub")]
    tangents: Vec<Vector4F>,
    #[getset(get = "pub", get_mut = "pub")]
    indices: Vec<i32>,
}

impl IndexedModel {
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

            let delta_u_1 = self.tex_coords[i_1].x() - self.tex_coords[i_0].x();
            let delta_v_1 = self.tex_coords[i_1].y() - self.tex_coords[i_0].y();
            let delta_u_2 = self.tex_coords[i_2].x() - self.tex_coords[i_0].x();
            let delta_v_2 = self.tex_coords[i_2].y() - self.tex_coords[i_0].y();

            let dividend = (delta_u_1 * delta_v_2 - delta_u_2 * delta_v_1);
            let f = if dividend == 0.0 { 0.0 } else { 1.0 / dividend };

            let tangent = Vector4F::new(
                f * (delta_v_2 * edge_1.x() - delta_v_1 * edge_2.x()),
                f * (delta_v_2 * edge_1.y() - delta_v_1 * edge_2.y()),
                f * (delta_v_2 * edge_1.z() - delta_v_1 * edge_2.z()),
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
