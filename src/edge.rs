use crate::vertex::Vertex;

#[derive(Clone, Copy)]
pub struct Edge {
    x: f32,
    x_step: f32,
    y_start: i32,
    y_end: i32,
}

impl Edge {
    pub fn new(min_y_vert: Vertex, max_y_vert: Vertex) -> Self {
        let y_dist = max_y_vert.y() - min_y_vert.y();
        let x_dist = max_y_vert.x() - min_y_vert.x();
        let y_pre = min_y_vert.y().ceil() - min_y_vert.y();
        let x_step = x_dist / y_dist;

        Self {
            x: min_y_vert.x() + y_pre * x_step,
            x_step: x_step,
            y_start: min_y_vert.y().ceil() as i32,
            y_end: max_y_vert.y().ceil() as i32,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn x_step(&self) -> f32 {
        self.x_step
    }

    pub fn y_start(&self) -> i32 {
        self.y_start
    }

    pub fn y_end(&self) -> i32 {
        self.y_end
    }

    pub fn step(&mut self) {
        self.x += self.x_step;
    }
}
