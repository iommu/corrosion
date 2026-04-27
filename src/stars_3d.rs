use crate::{bitmap::Bitmap, pixel::Pixel};

#[derive(Default, Clone)]
pub struct Stars3D {
    spread: f32,
    speed: f32,
    num_stars: usize,
    half_tan_fov: f32,
    stars_x: Vec<f32>,
    stars_y: Vec<f32>,
    stars_z: Vec<f32>,
}

impl Stars3D {
    pub fn new(num_stars: usize, spread: f32, speed: f32) -> Self {
        let mut obj = Self {
            spread,
            speed,
            num_stars,
            half_tan_fov: (70.0 as f32 / 2.0).to_radians().tan(),
            stars_x: vec![0.0; num_stars],
            stars_y: vec![0.0; num_stars],
            stars_z: vec![0.0; num_stars],
        };

        for index in 0..num_stars {
            obj.init_star(index);
        }

        return obj;
    }

    fn init_star(&mut self, index: usize) {
        self.stars_x[index] = (rand::random::<f32>() * 2.0 - 1.0) * self.spread;
        self.stars_y[index] = (rand::random::<f32>() * 2.0 - 1.0) * self.spread;
        self.stars_z[index] = (rand::random::<f32>() + 0.00001) * self.spread;
    }

    pub fn update_and_render(&mut self, target: &mut Bitmap, delta: f32) {
        target.fill_pixel(Pixel::BLACK);

        let half_width = target.width() as f32 / 2.0;
        let half_height = target.height() as f32 / 2.0;

        for index in 0..self.num_stars {
            self.stars_z[index] -= delta * self.speed;

            if self.stars_z[index] <= 0.0 {
                self.init_star(index);
            }

            let x = ((self.stars_x[index] / (self.stars_z[index] * self.half_tan_fov)) * half_width
                + half_width) as i64;
            let y = ((self.stars_y[index] / (self.stars_z[index] * self.half_tan_fov))
                * half_height
                + half_height) as i64;

            if (x < 0 || x >= target.width() as i64) || (y < 0 || y >= target.height() as i64) {
                self.init_star(index);
            } else {
                target.draw_pixel(x as usize, y as usize, Pixel::WHITE);
            }
        }
    }
}
