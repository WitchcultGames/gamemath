use std;
use vec3::Vec3;
use vec4::Vec4;
use quat::Quat;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Mat4 {
    pub rows: [Vec4; 4],
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        Self::default()
    }

    pub fn frustum(top: f32, left: f32, right: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
        let mut result: Mat4 = 0.0.into();

        let double_near = near * 2.0;
        let delta_x = right - left;
        let delta_y = top - bottom;
        let delta_z = far - near;

        result[0][0] = double_near / delta_x;
        result[1][1] = double_near / delta_y;
        result[2][0] = (right + left) / delta_x;
        result[2][1] = (top + bottom) / delta_y;
        result[2][2] = (-far - near) / delta_z;
        result[2][3] = -1.0;
        result[3][2] = (-double_near * far) / delta_z;

        result
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
        let y_max = near * (fov * (PI / 360.0)).tan();
        let x_max = y_max * aspect;

        Self::frustum(y_max, -x_max, x_max, -y_max, near, far)
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        let forward = (eye - target).normalized();
        let right = up.normalized().cross(forward).normalized();
        let up = forward.cross(right).normalized();

        ((right.x, right.y, right.z, 0.0),
         (up.x, up.y, up.z, 0.0),
         (forward.x, forward.y, forward.z, 0.0),
         (eye.x, eye.y, eye.z, 1.0)).into()
    }

    pub fn orthogonal(top: f32, left: f32, right: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
        let mut result: Mat4 = 0.0.into();

        let left_to_right = 1.0 / (left - right);
        let bottom_to_top = 1.0 / (bottom - top);
        let near_to_far = 1.0 / (near - far);

        result[0][0] = -2.0 * left_to_right;
        result[1][1] = -2.0 * bottom_to_top;
        result[2][2] = 2.0 * near_to_far;

        result[3][0] = left_to_right * (left + right);
        result[3][1] = bottom_to_top * (top + bottom);
        result[3][2] = near_to_far * (near + far);
        result[3][3] = 1.0;

        result
    }

    pub fn get_right_vector(&self) -> Vec3 {
        (self[0][0], self[1][0], self[2][0]).into()
    }

    pub fn get_left_vector(&self) -> Vec3 {
        -self.get_right_vector()
    }

    pub fn get_up_vector(&self) -> Vec3 {
        (self[0][1], self[1][1], self[2][1]).into()
    }

    pub fn get_down_vector(&self) -> Vec3 {
        -self.get_up_vector()
    }

    pub fn get_backward_vector(&self) -> Vec3 {
        (self[0][2], self[1][2], self[2][2]).into()
    }

    pub fn get_forward_vector(&self) -> Vec3 {
        -self.get_backward_vector()
    }

    pub fn transposed(&self) -> Mat4 {
        ((self[0][0], self[1][0], self[2][0], self[3][0]),
         (self[0][1], self[1][1], self[2][1], self[3][1]),
         (self[0][2], self[1][2], self[2][2], self[3][2]),
         (self[0][3], self[1][3], self[2][3], self[3][3])).into()
    }

    pub fn transpose(&mut self) {
        *self = self.transposed();
    }

    pub fn determinant(&self) -> f32 {
        self[3][0] * self[2][1] * self[1][2] * self[0][3] -
        self[2][0] * self[3][1] * self[1][2] * self[0][3] -
        self[3][0] * self[1][1] * self[2][2] * self[0][3] +
        self[1][0] * self[3][1] * self[2][2] * self[0][3] +
        self[2][0] * self[1][1] * self[3][2] * self[0][3] -
        self[1][0] * self[2][1] * self[3][2] * self[0][3] -
        self[3][0] * self[2][1] * self[0][2] * self[1][3] +
        self[2][0] * self[3][1] * self[0][2] * self[1][3] +
        self[3][0] * self[0][1] * self[2][2] * self[1][3] -
        self[0][0] * self[3][1] * self[2][2] * self[1][3] -
        self[2][0] * self[0][1] * self[3][2] * self[1][3] +
        self[0][0] * self[2][1] * self[3][2] * self[1][3] +
        self[3][0] * self[1][1] * self[0][2] * self[2][3] -
        self[1][0] * self[3][1] * self[0][2] * self[2][3] -
        self[3][0] * self[0][1] * self[1][2] * self[2][3] +
        self[0][0] * self[3][1] * self[1][2] * self[2][3] +
        self[1][0] * self[0][1] * self[3][2] * self[2][3] -
        self[0][0] * self[1][1] * self[3][2] * self[2][3] -
        self[2][0] * self[1][1] * self[0][2] * self[3][3] +
        self[1][0] * self[2][1] * self[0][2] * self[3][3] +
        self[2][0] * self[0][1] * self[1][2] * self[3][3] -
        self[0][0] * self[2][1] * self[1][2] * self[3][3] -
        self[1][0] * self[0][1] * self[2][2] * self[3][3] +
        self[0][0] * self[1][1] * self[2][2] * self[3][3]
    }

    pub fn adjointed(&self) -> Mat4 {
        let mut result: Mat4 = 0.0.into();

        result[0][0] =   self[1][1] * (self[2][2] * self[3][3] - self[2][3] * self[3][2]) -
                         self[2][1] * (self[1][2] * self[3][3] - self[1][3] * self[3][2]) +
                         self[3][1] * (self[1][2] * self[2][3] - self[1][3] * self[2][2]);
        result[0][1] = -(self[0][1] * (self[2][2] * self[3][3] - self[2][3] * self[3][2]) -
                         self[2][1] * (self[0][2] * self[3][3] - self[0][3] * self[3][2]) +
                         self[3][1] * (self[0][2] * self[2][3] - self[0][3] * self[2][2]));
        result[0][2] =   self[0][1] * (self[1][2] * self[3][3] - self[1][3] * self[3][2]) -
                         self[1][1] * (self[0][2] * self[3][3] - self[0][3] * self[3][2]) +
                         self[3][1] * (self[0][2] * self[1][3] - self[0][3] * self[1][2]);
        result[0][3] = -(self[0][1] * (self[1][2] * self[2][3] - self[1][3] * self[2][2]) -
                         self[1][1] * (self[0][2] * self[2][3] - self[0][3] * self[2][2]) +
                         self[2][1] * (self[0][2] * self[1][3] - self[0][3] * self[1][2]));
        result[1][0] = -(self[1][0] * (self[2][2] * self[3][3] - self[2][3] * self[3][2]) -
                         self[2][0] * (self[1][2] * self[3][3] - self[1][3] * self[3][2]) +
                         self[3][0] * (self[1][2] * self[2][3] - self[1][3] * self[2][2]));
        result[1][1] =   self[0][0] * (self[2][2] * self[3][3] - self[2][3] * self[3][2]) -
                         self[2][0] * (self[0][2] * self[3][3] - self[0][3] * self[3][2]) +
                         self[3][0] * (self[0][2] * self[2][3] - self[0][3] * self[2][2]);
        result[1][2] = -(self[0][0] * (self[1][2] * self[3][3] - self[1][3] * self[3][2]) -
                         self[1][0] * (self[0][2] * self[3][3] - self[0][3] * self[3][2]) +
                         self[3][0] * (self[0][2] * self[1][3] - self[0][3] * self[1][2]));
        result[1][3] =   self[0][0] * (self[1][2] * self[2][3] - self[1][3] * self[2][2]) -
                         self[1][0] * (self[0][2] * self[2][3] - self[0][3] * self[2][2]) +
                         self[2][0] * (self[0][2] * self[1][3] - self[0][3] * self[1][2]);
        result[2][0] =   self[1][0] * (self[2][1] * self[3][3] - self[2][3] * self[3][1]) -
                         self[2][0] * (self[1][1] * self[3][3] - self[1][3] * self[3][1]) +
                         self[3][0] * (self[1][1] * self[2][3] - self[1][3] * self[2][1]);
        result[2][1] = -(self[0][0] * (self[2][1] * self[3][3] - self[2][3] * self[3][1]) -
                         self[2][0] * (self[0][1] * self[3][3] - self[0][3] * self[3][1]) +
                         self[3][0] * (self[0][1] * self[2][3] - self[0][3] * self[2][1]));
        result[2][2] =   self[0][0] * (self[1][1] * self[3][3] - self[1][3] * self[3][1]) -
                         self[1][0] * (self[0][1] * self[3][3] - self[0][3] * self[3][1]) +
                         self[3][0] * (self[0][1] * self[1][3] - self[0][3] * self[1][1]);
        result[2][3] = -(self[0][0] * (self[1][1] * self[2][3] - self[1][3] * self[2][1]) -
                         self[1][0] * (self[0][1] * self[2][3] - self[0][3] * self[2][1]) +
                         self[2][0] * (self[0][1] * self[1][3] - self[0][3] * self[1][1]));
        result[3][0] = -(self[1][0] * (self[2][1] * self[3][2] - self[2][2] * self[3][1]) -
                         self[2][0] * (self[1][1] * self[3][2] - self[1][2] * self[3][1]) +
                         self[3][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1]));
        result[3][1] =   self[0][0] * (self[2][1] * self[3][2] - self[2][2] * self[3][1]) -
                         self[2][0] * (self[0][1] * self[3][2] - self[0][2] * self[3][1]) +
                         self[3][0] * (self[0][1] * self[2][2] - self[0][2] * self[2][1]);
        result[3][2] = -(self[0][0] * (self[1][1] * self[3][2] - self[1][2] * self[3][1]) -
                         self[1][0] * (self[0][1] * self[3][2] - self[0][2] * self[3][1]) +
                         self[3][0] * (self[0][1] * self[1][2] - self[0][2] * self[1][1]));
        result[3][3] =   self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1]) -
                         self[1][0] * (self[0][1] * self[2][2] - self[0][2] * self[2][1]) +
                         self[2][0] * (self[0][1] * self[1][2] - self[0][2] * self[1][1]);

        result
    }

    pub fn inverted(&self) -> Mat4 {
        let determinant = self.determinant();

        if determinant > 0.0 {
            let mut result: Mat4 = 0.0.into();
            let adjoint = self.adjointed();

            result[0][0] = adjoint[0][0] / determinant;
            result[0][1] = adjoint[0][1] / determinant;
            result[0][2] = adjoint[0][2] / determinant;
            result[0][3] = adjoint[0][3] / determinant;

            result[1][0] = adjoint[1][0] / determinant;
            result[1][1] = adjoint[1][1] / determinant;
            result[1][2] = adjoint[1][2] / determinant;
            result[1][3] = adjoint[1][3] / determinant;

            result[2][0] = adjoint[2][0] / determinant;
            result[2][1] = adjoint[2][1] / determinant;
            result[2][2] = adjoint[2][2] / determinant;
            result[2][3] = adjoint[2][3] / determinant;

            result[3][0] = adjoint[3][0] / determinant;
            result[3][1] = adjoint[3][1] / determinant;
            result[3][2] = adjoint[3][2] / determinant;
            result[3][3] = adjoint[3][3] / determinant;

            result
        } else {
            0.0.into()
        }
    }

    pub fn rotation(radians: f32, axis: Vec3) -> Mat4 {
        let sin = radians.sin();
        let cos = radians.cos();
        let cos_m1 = 1.0 - cos;
        let axis = axis.normalized();

        ((axis.x * axis.x * cos_m1 + cos,
          axis.x * axis.y * cos_m1 - axis.z * sin,
          axis.x * axis.z * cos_m1 + axis.y * sin,
          0.0),
         (axis.y * axis.x * cos_m1 + axis.z * sin,
          axis.y * axis.y * cos_m1 + cos,
          axis.y * axis.z * cos_m1 - axis.x * sin,
          0.0),
         (axis.z * axis.x * cos_m1 - axis.y * sin,
          axis.z * axis.y * cos_m1 + axis.x * sin,
          axis.z * axis.z * cos_m1 + cos,
          0.0),
         (0.0, 0.0, 0.0, 1.0)).into()
    }

    pub fn rotated(&mut self, radians: f32, axis: Vec3) -> Mat4 {
        *self * Mat4::rotation(radians, axis)
    }

    pub fn rotate(&mut self, radians: f32, axis: Vec3) {
        *self = self.rotated(radians, axis);
    }

    pub fn scale(&mut self, factor: Vec3) {
        *self = self.scaled(factor);
    }

    pub fn scaled(&mut self, factor: Vec3) -> Mat4 {
        let mut matrix = *self;

        matrix[0] *= factor.x;
        matrix[1] *= factor.y;
        matrix[2] *= factor.z;

        matrix
    }

    pub fn translate(&mut self, translation: Vec3) {
        *self = self.translated(translation);
    }

    pub fn translated(&mut self, translation: Vec3) -> Mat4 {
        let mut result = *self;

        result[3][0] += self[0][0] * translation.x +
                        self[1][0] * translation.y +
                        self[2][0] * translation.z;

        result[3][1] +=   self[0][1] * translation.x +
                        self[1][1] * translation.y +
                        self[2][1] * translation.z;

        result[3][2] += self[0][2] * translation.x +
                        self[1][2] * translation.y +
                        self[2][2] * translation.z;

        result[3][3] += self[0][3] * translation.x +
                        self[1][3] * translation.y +
                        self[2][3] * translation.z;

        result
    }
}

impl Default for Mat4 {
    fn default() -> Mat4 {
        ((1.0, 0.0, 0.0, 0.0),
         (0.0, 1.0, 0.0, 0.0),
         (0.0, 0.0, 1.0, 0.0),
         (0.0, 0.0, 0.0, 1.0)).into()
    }
}

impl From<f32> for Mat4 {
    fn from(value: f32) -> Mat4 {
        Mat4 {
            rows: [(value, 0.0, 0.0, 0.0).into(),
                   (0.0, value, 0.0, 0.0).into(),
                   (0.0, 0.0, value, 0.0).into(),
                   (0.0, 0.0, 0.0, value).into()],
        }
    }
}

impl From<((f32, f32, f32, f32),
           (f32, f32, f32, f32),
           (f32, f32, f32, f32),
           (f32, f32, f32, f32))> for Mat4 {
    fn from(tuple: ((f32, f32, f32, f32),
                    (f32, f32, f32, f32),
                    (f32, f32, f32, f32),
                    (f32, f32, f32, f32))) -> Mat4 {
        Mat4 {
            rows: [tuple.0.into(), tuple.1.into(), tuple.2.into(), tuple.3.into()],
        }
    }
}

impl From<(f32, f32, f32, f32,
           f32, f32, f32, f32,
           f32, f32, f32, f32,
           f32, f32, f32, f32)> for Mat4 {
    fn from(tuple: (f32, f32, f32, f32,
                    f32, f32, f32, f32,
                    f32, f32, f32, f32,
                    f32, f32, f32, f32)) -> Mat4 {
        Mat4 {
            rows: [(tuple.0, tuple.1, tuple.2, tuple.3).into(),
                   (tuple.4, tuple.5, tuple.6, tuple.7).into(),
                   (tuple.8, tuple.9, tuple.10, tuple.11).into(),
                   (tuple.12, tuple.13, tuple.14, tuple.15).into()],
        }
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(slice: [[f32; 4]; 4]) -> Mat4 {
        Mat4 {
            rows: [slice[0].into(), slice[1].into(), slice[2].into(), slice[3].into()],
        }
    }
}

impl From<[f32; 16]> for Mat4 {
    fn from(slice: [f32; 16]) -> Mat4 {
        Mat4 {
            rows: [(slice[ 0], slice[ 1], slice[ 2], slice[ 3]).into(),
                   (slice[ 4], slice[ 5], slice[ 6], slice[ 7]).into(),
                   (slice[ 8], slice[ 9], slice[10], slice[11]).into(),
                   (slice[12], slice[13], slice[14], slice[15]).into()],
        }
    }
}

impl From<[Vec4; 4]> for Mat4 {
    fn from(slice: [Vec4; 4]) -> Mat4 {
        Mat4 {
            rows: [slice[0], slice[1], slice[2], slice[3]],
        }
    }
}

impl From<(Vec4, Vec4, Vec4, Vec4)> for Mat4 {
    fn from(tuple: (Vec4, Vec4, Vec4, Vec4)) -> Mat4 {
        Mat4 {
            rows: [tuple.0, tuple.1, tuple.2, tuple.3],
        }
    }
}

impl From<Quat> for Mat4 {
    fn from(quat: Quat) -> Mat4 {
        quat.extract_matrix()
    }
}

impl std::ops::Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Vec4 {
        match index {
            0 => &self.rows[0],
            1 => &self.rows[1],
            2 => &self.rows[2],
            3 => &self.rows[3],
            _ => panic!("Mat4 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Vec4 {
        match index {
            0 => &mut self.rows[0],
            1 => &mut self.rows[1],
            2 => &mut self.rows[2],
            3 => &mut self.rows[3],
            _ => panic!("Mat4 index out of range!"),
        }
    }
}

impl std::ops::Index<(usize, usize)> for Mat4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        &self.rows[index.0][index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        &mut self.rows[index.0][index.1]
    }
}

impl std::ops::Add for Mat4 {
    type Output = Mat4;

    fn add(self, right: Mat4) -> Mat4 {
        Mat4 {
            rows: [self[0] + right[0], self[1] + right[1], self[2] + right[2], self[3] + right[3]],
        }
    }
}

impl std::ops::AddAssign for Mat4 {
    fn add_assign(&mut self, right: Mat4) {
        *self = *self + right;
    }
}

impl std::ops::Sub for Mat4 {
    type Output = Mat4;

    fn sub(self, right: Mat4) -> Mat4 {
        Mat4 {
            rows: [self[0] - right[0], self[1] - right[1], self[2] - right[2], self[3] - right[3]],
        }
    }
}

impl std::ops::SubAssign for Mat4 {
    fn sub_assign(&mut self, right: Mat4) {
        *self = *self - right;
    }
}

impl std::ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, vec: Vec4) -> Vec4 {
        (self[0].dot(vec),
         self[1].dot(vec),
         self[2].dot(vec),
         self[3].dot(vec)).into()
    }
}

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, right: Mat4) -> Mat4 {
        let mut result: Mat4 = 0.0.into();

        result[0][0] = self[0][0] * right[0][0] +
                     self[0][1] * right[1][0] +
                     self[0][2] * right[2][0] +
                     self[0][3] * right[3][0];
        result[0][1] = self[0][0] * right[0][1] +
                     self[0][1] * right[1][1] +
                     self[0][2] * right[2][1] +
                     self[0][3] * right[3][1];
        result[0][2] = self[0][0] * right[0][2] +
                     self[0][1] * right[1][2] +
                     self[0][2] * right[2][2] +
                     self[0][3] * right[3][2];
        result[0][3] = self[0][0] * right[0][3] +
                     self[0][1] * right[1][3] +
                     self[0][2] * right[2][3] +
                     self[0][3] * right[3][3];

        result[1][0] = self[1][0] * right[0][0] +
                     self[1][1] * right[1][0] +
                     self[1][2] * right[2][0] +
                     self[1][3] * right[3][0];
        result[1][1] = self[1][0] * right[0][1] +
                     self[1][1] * right[1][1] +
                     self[1][2] * right[2][1] +
                     self[1][3] * right[3][1];
        result[1][2] = self[1][0] * right[0][2] +
                     self[1][1] * right[1][2] +
                     self[1][2] * right[2][2] +
                     self[1][3] * right[3][2];
        result[1][3] = self[1][0] * right[0][3] +
                     self[1][1] * right[1][3] +
                     self[1][2] * right[2][3] +
                     self[1][3] * right[3][3];

        result[2][0] = self[2][0] * right[0][0] +
                     self[2][1] * right[1][0] +
                     self[2][2] * right[2][0] +
                     self[2][3] * right[3][0];
        result[2][1] = self[2][0] * right[0][1] +
                     self[2][1] * right[1][1] +
                     self[2][2] * right[2][1] +
                     self[2][3] * right[3][1];
        result[2][2] = self[2][0] * right[0][2] +
                     self[2][1] * right[1][2] +
                     self[2][2] * right[2][2] +
                     self[2][3] * right[3][2];
        result[2][3] = self[2][0] * right[0][3] +
                     self[2][1] * right[1][3] +
                     self[2][2] * right[2][3] +
                     self[2][3] * right[3][3];

        result[3][0] = self[3][0] * right[0][0] +
                     self[3][1] * right[1][0] +
                     self[3][2] * right[2][0] +
                     self[3][3] * right[3][0];
        result[3][1] = self[3][0] * right[0][1] +
                     self[3][1] * right[1][1] +
                     self[3][2] * right[2][1] +
                     self[3][3] * right[3][1];
        result[3][2] = self[3][0] * right[0][2] +
                     self[3][1] * right[1][2] +
                     self[3][2] * right[2][2] +
                     self[3][3] * right[3][2];
        result[3][3] = self[3][0] * right[0][3] +
                     self[3][1] * right[1][3] +
                     self[3][2] * right[2][3] +
                     self[3][3] * right[3][3];

        result
    }
}

impl std::ops::MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, right: Mat4) {
        *self = *self * right;
    }
}
