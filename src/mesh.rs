use std::{io, path::Path};

use crate::{obj_loader::OBJModel, vertex::Vertex};

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<i32>,
}

impl Mesh {
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<i32> {
        &self.indices
    }

    pub fn new_from_obj_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let model = OBJModel::new_from_file(path)?.to_indexedmodel();
        let mut vertices: Vec<Vertex> = vec![];

        for idx in 0..model.positions().len() {
            vertices.push(Vertex::new(model.positions()[idx], model.tex_coords()[idx]));
        }

        Ok(Self {
            vertices,
            indices: model.indices().clone(),
        })
    }
}
