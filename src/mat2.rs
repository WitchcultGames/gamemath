use std;
use vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Mat2 {
    pub rows: [Vec2; 2],
}

impl Mat2 {
    pub fn identity() -> Mat2 {
        Self::default()
    }

    pub fn transposed(&self) -> Mat2 {
        ((self[0][0], self[1][0]),
         (self[0][1], self[1][1])).into()
    }

    pub fn transpose(&mut self) {
        *self = self.transposed();
    }

    pub fn determinant(&self) -> f32 {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    pub fn inverted(&self) -> Mat2 {
        let determinant = self.determinant();

        if determinant > 0.0 {
            let determinant = 1.0 / determinant;

            ((determinant * self[1][1], determinant * -self[0][1]),
             (determinant * -self[1][0], determinant * self[0][0])).into()
        } else {
            ((0.0, 0.0),
             (0.0, 0.0)).into()
        }
    }

    pub fn rotation(radians: f32) -> Mat2 {
        let sin = radians.sin();
        let cos = radians.cos();

        ((cos, sin),
         (-sin, cos)).into()
    }

    pub fn rotate(&mut self, radians: f32) {
        *self *= Mat2::rotation(radians);
    }

    pub fn scale(&mut self, factor: Vec2) {
        self[0] *= factor.x;
        self[1] *= factor.y;
    }
}

impl Default for Mat2 {
    fn default() -> Mat2 {
        ((1.0, 0.0),
         (0.0, 1.0)).into()
    }
}

impl From<f32> for Mat2 {
    fn from(value: f32) -> Mat2 {
        Mat2 {
            rows: [value.into(), value.into()],
        }
    }
}

impl From<((f32, f32), (f32, f32))> for Mat2 {
    fn from(tuple: ((f32, f32), (f32, f32))) -> Mat2 {
        Mat2 {
            rows: [tuple.0.into(), tuple.1.into()],
        }
    }
}

impl std::ops::Index<usize> for Mat2 {
    type Output = Vec2;
    
    fn index(&self, index: usize) -> &Vec2 {
        match index {
            0 => &self.rows[0],
            1 => &self.rows[1],
            _ => panic!("Mat2 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Vec2 {
        match index {
            0 => &mut self.rows[0],
            1 => &mut self.rows[1],
            _ => panic!("Mat2 index out of range!"),
        }
    }
}

impl std::ops::Add for Mat2 {
    type Output = Mat2;

    fn add(self, right: Mat2) -> Mat2 {
        Mat2 {
            rows: [self[0] + right[0], self[1] + right[1]],
        }
    }
}

impl std::ops::AddAssign for Mat2 {
    fn add_assign(&mut self, right: Mat2) {
        *self = *self + right;
    }
}

impl std::ops::Sub for Mat2 {
    type Output = Mat2;

    fn sub(self, right: Mat2) -> Mat2 {
        Mat2 {
            rows: [self[0] - right[0], self[1] - right[1]],
        }
    }
}

impl std::ops::SubAssign for Mat2 {
    fn sub_assign(&mut self, right: Mat2) {
        *self = *self - right;
    }
}

impl std::ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, right: Mat2) -> Mat2 {
        ((self[0][0] * right[0][0] + self[0][1] * right[1][0],
          self[0][0] * right[0][1] + self[0][1] * right[1][1]),
         (self[1][0] * right[0][0] + self[1][1] * right[1][0],
          self[1][0] * right[0][1] + self[1][1] * right[1][1])).into()
    }
}

impl std::ops::MulAssign<Mat2> for Mat2 {
    fn mul_assign(&mut self, right: Mat2) {
        *self = *self * right;
    }
}
