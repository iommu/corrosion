use getset::{Getters, MutGetters};
use std::{io, path::Path};

use crate::{obj_loader::OBJModel, vertex::Vertex};

#[derive(Clone, Getters, MutGetters)]
pub struct Mesh {
    #[getset(get = "pub", get_mut = "pub")]
    vertices: Vec<Vertex>,
    #[getset(get = "pub", get_mut = "pub")]
    indices: Vec<i32>,
}

impl Mesh {
    pub fn new_from_obj_bytes(bytes: &[u8]) -> io::Result<Self> {
        let model = OBJModel::new_from_bytes(bytes)?.to_indexedmodel();
        let mut vertices: Vec<Vertex> = vec![];

        for idx in 0..model.positions().len() {
            vertices.push(Vertex::new(
                model.positions()[idx],
                model.tex_coords()[idx],
                model.normals()[idx],
            ));
        }

        Ok(Self {
            vertices,
            indices: model.indices().clone(),
        })
    }
}
