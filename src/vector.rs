use std::ops::{self, Add, Index, IndexMut, Mul, Sub};

use derive_new::new;

#[derive(Default, new, Clone, Copy)]
pub struct Vector4F {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4F {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }

    pub fn w_mut(&mut self) -> &mut f32 {
        &mut self.w
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn max(&self) -> f32 {
        self.x.max(self.y.max(self.z.max(self.w)))
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x - rhs.z,
            z: self.x * rhs.y - self.y - rhs.x,
            w: 0.0,
        }
    }

    pub fn normalized(&self) -> Self {
        let len = self.len();
        *self / len
    }

    pub fn rotate(&self, axis: Vector4F, angle: f32) -> Self {
        let sin = (-angle).sin();
        let cos = (-angle).cos();

        //Rotation on local X
        self.cross(axis.mul(sin)).add(
            //Rotation on local Z
            self.mul(cos).add(
                //Rotation on local Y
                axis.mul(self.dot(axis.mul(1.0 - cos))),
            ),
        )
    }

    pub fn lerp(&self, dest: Vector4F, factor: f32) -> Self {
        dest.sub(*self).mul(factor).add(*self)
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }
}

// Addition

impl ops::Add for Vector4F {
    type Output = Vector4F;
    fn add(self, rhs: Self) -> Vector4F {
        Vector4F {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Add<f32> for Vector4F {
    type Output = Vector4F;
    fn add(self, val: f32) -> Vector4F {
        Vector4F {
            x: self.x + val,
            y: self.y + val,
            z: self.z + val,
            w: self.w + val,
        }
    }
}

// Subtraction

impl ops::Sub for Vector4F {
    type Output = Vector4F;
    fn sub(self, rhs: Self) -> Vector4F {
        Vector4F {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Sub<f32> for Vector4F {
    type Output = Vector4F;
    fn sub(self, val: f32) -> Vector4F {
        Vector4F {
            x: self.x - val,
            y: self.y - val,
            z: self.z - val,
            w: self.w - val,
        }
    }
}

// Multiplication

impl ops::Mul for Vector4F {
    type Output = Vector4F;
    #[inline]
    fn mul(self, rhs: Vector4F) -> Vector4F {
        Vector4F {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl ops::Mul<f32> for Vector4F {
    type Output = Vector4F;
    #[inline]
    fn mul(self, val: f32) -> Vector4F {
        Vector4F {
            x: self.x * val,
            y: self.y * val,
            z: self.z * val,
            w: self.w * val,
        }
    }
}

// Division

impl ops::Div for Vector4F {
    type Output = Vector4F;
    #[inline]
    fn div(self, rhs: Vector4F) -> Vector4F {
        Vector4F {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

impl ops::Div<f32> for Vector4F {
    type Output = Vector4F;
    #[inline]
    fn div(self, val: f32) -> Vector4F {
        Vector4F {
            x: self.x / val,
            y: self.y / val,
            z: self.z / val,
            w: self.w / val,
        }
    }
}

// Equals

impl PartialEq for Vector4F {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z) && (self.w == rhs.w)
    }
}


// Indexing

impl Index<usize> for Vector4F {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds")
        }
    }
}

impl IndexMut<usize> for Vector4F {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds")
        }
    }
}