use std;
use vec2::Vec2;
use vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Mat3 {
    pub rows: [Vec3; 3],
}

impl Mat3 {
    pub fn identity() -> Mat3 {
        Self::default()
    }

    pub fn transposed(&self) -> Mat3 {
        ((self[0][0], self[1][0], self[2][0]),
         (self[0][1], self[1][1], self[2][1]),
         (self[0][2], self[1][2], self[2][2])).into()
    }

    pub fn transpose(&mut self) {
        *self = self.transposed();
    }

    pub fn determinant(&self) -> f32 {
        let p1 = self[2][2] * self[1][1] - self[1][2] * self[2][1];
        let p2 = -self[2][2] * self[1][0] + self[1][2] * self[2][0];
        let p3 = self[2][1] * self[1][0] - self[1][1] * self[2][0];

        self[0][0] * p1 + self[0][1] * p2 + self[0][2] + p3
    }

    pub fn inverted(&self) -> Mat3 {
        let determinant = self.determinant();

        if determinant > 0.0 {
            let p1 =  self[2][2] * self[1][1] - self[1][2] * self[2][1];
            let p2 = -self[2][2] * self[1][0] + self[1][2] * self[2][0];
            let p3 =  self[2][1] * self[1][0] - self[1][1] * self[2][0];
            let determinant = 1.0 / determinant;

            let mut result: Mat3 = determinant.into();

            result[0][0] *= p1;
            result[1][0] *= p2;
            result[2][0] *= p3;
            result[0][1] *= -self[2][2] * self[0][1] + self[0][2] * self[2][1];
            result[0][2] *= self[1][2] * self[0][1] - self[0][2] * self[1][1];
            result[1][1] *= self[2][2] * self[0][0] - self[0][2] * self[2][0];
            result[1][2] *= -self[2][2] * self[0][0] + self[0][2] * self[1][0];
            result[2][1] *= -self[2][1] * self[0][0] + self[0][1] * self[2][0];
            result[2][2] *= self[1][1] * self[0][0] - self[0][1] * self[1][0];

            result
        } else {
            ((0.0, 0.0, 0.0),
             (0.0, 0.0, 0.0),
             (0.0, 0.0, 0.0)).into()
        }
    }

    pub fn rotation(radians: f32) -> Mat3 {
        let sin = radians.sin();
        let cos = radians.cos();

        ((cos, sin, 0.0),
         (-sin, cos, 0.0),
         (0.0, 0.0, 1.0)).into()
    }

    pub fn rotate(&mut self, radians: f32) {
        *self *= Mat3::rotation(radians);
    }

    pub fn scale(&mut self, factor: Vec2) {
        self[0] *= factor.x;
        self[1] *= factor.y;
    }

    pub fn translate(&mut self, translation: Vec2) {
        self[2][0] += self[0][0] * translation.x + self[1][0] * translation.y;
        self[2][1] += self[0][1] * translation.x + self[1][1] * translation.y;
        self[2][2] += self[0][2] * translation.x + self[1][2] * translation.y;
    }
}

impl Default for Mat3 {
    fn default() -> Mat3 {
        ((1.0, 0.0, 0.0),
         (0.0, 1.0, 0.0),
         (0.0, 0.0, 1.0)).into()
    }
}

impl From<f32> for Mat3 {
    fn from(value: f32) -> Mat3 {
        Mat3 {
            rows: [value.into(), value.into(), value.into()],
        }
    }
}

impl From<((f32, f32, f32), (f32, f32, f32), (f32, f32, f32))> for Mat3 {
    fn from(tuple: ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32))) -> Mat3 {
        Mat3 {
            rows: [tuple.0.into(), tuple.1.into(), tuple.2.into()],
        }
    }
}

impl std::ops::Index<usize> for Mat3 {
    type Output = Vec3;
    
    fn index(&self, index: usize) -> &Vec3 {
        match index {
            0 => &self.rows[0],
            1 => &self.rows[1],
            2 => &self.rows[2],
            _ => panic!("Mat3 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Vec3 {
        match index {
            0 => &mut self.rows[0],
            1 => &mut self.rows[1],
            2 => &mut self.rows[2],
            _ => panic!("Mat3 index out of range!"),
        }
    }
}

impl std::ops::Add for Mat3 {
    type Output = Mat3;

    fn add(self, right: Mat3) -> Mat3 {
        Mat3 {
            rows: [self[0] + right[0], self[1] + right[1], self[2] + right[2]],
        }
    }
}

impl std::ops::AddAssign for Mat3 {
    fn add_assign(&mut self, right: Mat3) {
        *self = *self + right;
    }
}

impl std::ops::Sub for Mat3 {
    type Output = Mat3;

    fn sub(self, right: Mat3) -> Mat3 {
        Mat3 {
            rows: [self[0] - right[0], self[1] - right[1], self[2] - right[2]],
        }
    }
}

impl std::ops::SubAssign for Mat3 {
    fn sub_assign(&mut self, right: Mat3) {
        *self = *self - right;
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, right: Mat3) -> Mat3 {
        ((self[0][0] * right[0][0] +
          self[0][1] * right[1][0] +
          self[0][2] * right[2][0],
          self[0][0] * right[0][1] +
          self[0][1] * right[1][1] +
          self[0][2] * right[2][1],
          self[0][0] * right[0][2] +
          self[0][1] * right[1][2] +
          self[0][2] * right[2][2]),
         (self[1][0] * right[0][0] +
          self[1][1] * right[1][0] +
          self[1][2] * right[2][0],
          self[1][0] * right[0][1] +
          self[1][1] * right[1][1] +
          self[1][2] * right[2][1],
          self[1][0] * right[0][2] +
          self[1][1] * right[1][2] +
          self[1][2] * right[2][2]),
         (self[2][0] * right[0][0] +
          self[2][1] * right[1][0] +
          self[2][2] * right[2][0],
          self[2][0] * right[0][1] +
          self[2][1] * right[1][1] +
          self[2][2] * right[2][1],
          self[2][0] * right[0][2] +
          self[2][1] * right[1][2] +
          self[2][2] * right[2][2])).into()
    }
}

impl std::ops::MulAssign<Mat3> for Mat3 {
    fn mul_assign(&mut self, right: Mat3) {
        *self = *self * right;
    }
}
