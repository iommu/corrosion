
#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn tri_area(&self, vert_b : &Vertex, vert_c : &Vertex) -> f32 {
        let x_1 = vert_b.x - self.x;
        let y_1 = vert_b.y - self.y;
        let x_2 = vert_c.x - self.x;
        let y_2 = vert_c.y - self.y;

        // 2D cross product
        return (x_1 * y_2) - (x_2 * y_1);
    } 
}
