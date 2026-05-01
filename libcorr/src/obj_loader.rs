use crate::{indexed_model::IndexedModel, vector::Vector4F};

use std::{
    collections::HashMap,
    fs::File,
    hash::Hash,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Hash, Clone, Copy)]

struct OBJIndex {
    pub vertex_idx: i32,
    pub tex_coord_idx: i32,
    pub nrml_idx: i32,
}

// Equals

impl PartialEq for OBJIndex {
    fn eq(&self, rhs: &Self) -> bool {
        self.vertex_idx == rhs.vertex_idx
            && self.tex_coord_idx == rhs.tex_coord_idx
            && self.nrml_idx == rhs.nrml_idx
    }
}

impl Eq for OBJIndex {}

pub struct OBJModel {
    positions: Vec<Vector4F>,
    tex_coords: Vec<Vector4F>,
    normals: Vec<Vector4F>,
    indices: Vec<OBJIndex>,
    has_tex_coords: bool,
    has_normals: bool,
}

impl OBJModel {
    fn strings_rm_empty(str: &mut Vec<&str>) {
        str.retain(|s| !s.is_empty());
    }

    fn idx_from_str(&mut self, s: &str) -> io::Result<OBJIndex> {
        let values: Vec<&str> = s.split("/").collect();
        let mut idx = OBJIndex {
            vertex_idx: 0,
            tex_coord_idx: 0,
            nrml_idx: 0,
        };
        idx.vertex_idx = values[0].parse::<f32>().unwrap() as i32 - 1;

        if values.len() > 1 && !values[1].is_empty() {
            self.has_tex_coords = true;
            idx.tex_coord_idx = values[1].parse::<f32>().unwrap() as i32 - 1;
        }

        if values.len() > 2 && !values[2].is_empty() {
            self.has_normals = true;
            idx.nrml_idx = values[2].parse::<f32>().unwrap() as i32 - 1;
        }

        Ok(idx)
    }

    pub fn new_from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut mdl = Self {
            positions: vec![],
            tex_coords: vec![],
            normals: vec![],
            indices: vec![],
            has_tex_coords: false,
            has_normals: false,
        };

        let reader = BufReader::new(bytes).lines();
        for line in reader.map_while(Result::ok) {
            let mut tokens: Vec<&str> = line.split(" ").collect();
            Self::strings_rm_empty(&mut tokens);

            if tokens.len() == 0 || tokens[0] == "#" {
                continue;
            }
            match tokens[0] {
                "v" => {
                    mdl.positions.push(Vector4F::new(
                        tokens[1].parse().unwrap(),
                        tokens[2].parse().unwrap(),
                        tokens[3].parse().unwrap(),
                        1.0,
                    ));
                }
                "vt" => {
                    mdl.tex_coords.push(Vector4F::new(
                        tokens[1].parse().unwrap(),
                        tokens[2].parse().unwrap(),
                        0.0,
                        0.0,
                    ));
                }
                "vn" => {
                    mdl.normals.push(Vector4F::new(
                        tokens[1].parse().unwrap(),
                        tokens[2].parse().unwrap(),
                        tokens[3].parse().unwrap(),
                        0.0,
                    ));
                }
                "f" => {
                    for index in 0..tokens.len().saturating_sub(3) {
                        let idx = mdl.idx_from_str(tokens[1])?;
                        mdl.indices.push(idx);
                        let idx = mdl.idx_from_str(tokens[2 + index])?;
                        mdl.indices.push(idx);
                        let idx = mdl.idx_from_str(tokens[3 + index])?;
                        mdl.indices.push(idx);
                    }
                }
                _ => {}
            }
        }

        Ok(mdl)
    }

    pub fn to_indexedmodel(&self) -> IndexedModel {
        let mut res_imdl = IndexedModel::default();
        let mut normal_imdl = IndexedModel::default();
        let mut res_idx_map: HashMap<OBJIndex, i32> = HashMap::new();
        let mut normal_idx_map: HashMap<i32, i32> = HashMap::new();
        let mut idx_map: HashMap<i32, i32> = HashMap::new();

        for idx in 0..self.indices.len() {
            let cur_idx = &self.indices[idx];
            let cur_pos = &self.positions[cur_idx.vertex_idx as usize];
            let cur_tex_coord = if self.has_tex_coords {
                &self.tex_coords[cur_idx.tex_coord_idx as usize]
            } else {
                &Vector4F::new(0.0, 0.0, 0.0, 0.0)
            };
            let cur_normal = if self.has_normals {
                &self.normals[cur_idx.nrml_idx as usize]
            } else {
                &Vector4F::new(0.0, 0.0, 0.0, 0.0)
            };

            let mdl_vertex_idx = res_idx_map.entry(*cur_idx).or_insert_with(|| {
                let mdl_vertex_idx = res_imdl.positions().len() as i32;
                //
                res_imdl.positions_mut().push(*cur_pos);
                res_imdl.tex_coords_mut().push(*cur_tex_coord);
                if self.has_normals {
                    res_imdl.normals_mut().push(*cur_normal);
                }
                //
                mdl_vertex_idx
            });

            let normal_imdl_idx = normal_idx_map.entry(cur_idx.vertex_idx).or_insert_with(|| {
                let normal_imdl_idx = normal_imdl.positions().len() as i32;
                //
                normal_imdl.positions_mut().push(*cur_pos);
                normal_imdl.tex_coords_mut().push(*cur_tex_coord);
                normal_imdl.normals_mut().push(*cur_normal);
                normal_imdl
                    .tangents_mut()
                    .push(Vector4F::new(0.0, 0.0, 0.0, 0.0));
                //
                normal_imdl_idx
            });

            res_imdl.indices_mut().push(*mdl_vertex_idx);
            normal_imdl.indices_mut().push(*normal_imdl_idx);
            idx_map.insert(*mdl_vertex_idx, *normal_imdl_idx);
        }

        if self.has_normals {
            normal_imdl.calc_normals();

            for idx in 0..res_imdl.positions().len() as i32 {
                res_imdl
                    .normals_mut()
                    .push(normal_imdl.normals()[idx_map[&idx] as usize]);
            }
        }

        normal_imdl.calc_tangents();

        for idx in 0..res_imdl.positions().len() as i32 {
            res_imdl
                .tangents_mut()
                .push(normal_imdl.tangents()[idx_map[&idx] as usize]);
        }

        res_imdl
    }
}
