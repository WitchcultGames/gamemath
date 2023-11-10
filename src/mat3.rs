use std;
use vec2::Vec2;
use vec3::Vec3;

// TODO: Consider making Mat3 of a generic type instead of forcing f32.
//       But would any type other than f64 ever be useful?

type Row = (f32, f32, f32);
type InlineMat3 = (f32, f32, f32, f32, f32, f32, f32, f32, f32);

/// A 3x3-component Euclidean matrix useful for linear algebra computation in game development
/// and 2D rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    /// The three rows of the matrix, represented by an array of three `Vec3<f32>` objects.
    pub rows: [Vec3<f32>; 3],
}

impl Mat3 {
    /// Constructs a 3x3 identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat3, Vec3};
    ///
    /// let m = Mat3::identity();
    ///
    /// assert_eq!(m[0], Vec3::new(1.0, 0.0, 0.0));
    /// assert_eq!(m[1], Vec3::new(0.0, 1.0, 0.0));
    /// assert_eq!(m[2], Vec3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn identity() -> Mat3 {
        Self::default()
    }

    /// Extracts and returns a transposed representation of the calling `Mat3` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat3;
    ///
    /// let m: Mat3 = ((0.0, 1.0, 2.0),
    ///                (3.0, 4.0, 5.0),
    ///                (6.0, 7.0, 8.0)).into();
    ///
    /// assert_eq!(m.transposed(), ((0.0, 3.0, 6.0),
    ///                             (1.0, 4.0, 7.0),
    ///                             (2.0, 5.0, 8.0)).into());
    /// ```
    pub fn transposed(&self) -> Mat3 {
        (
            (self[0][0], self[1][0], self[2][0]),
            (self[0][1], self[1][1], self[2][1]),
            (self[0][2], self[1][2], self[2][2]),
        )
            .into()
    }

    /// Performs a transpose operation on the calling `Mat3` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat3;
    ///
    /// let mut m: Mat3 = ((0.0, 1.0, 2.0),
    ///                    (3.0, 4.0, 5.0),
    ///                    (6.0, 7.0, 8.0)).into();
    ///
    /// m.transpose();
    ///
    /// assert_eq!(m, ((0.0, 3.0, 6.0),
    ///                (1.0, 4.0, 7.0),
    ///                (2.0, 5.0, 8.0)).into());
    /// ```
    pub fn transpose(&mut self) {
        *self = self.transposed();
    }

    /// Constructs a 3x3 rotation matrix from a radians value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat3, Vec3};
    ///
    /// let m = Mat3::rotation(1.0);
    ///
    /// assert_eq!(m[0], Vec3::new(0.5403023,  -0.84147096, 0.0));
    /// assert_eq!(m[1], Vec3::new(0.84147096,  0.5403023,  0.0));
    /// assert_eq!(m[2], Vec3::new(0.0,         0.0,        1.0));
    /// ```
    pub fn rotation(radians: f32) -> Mat3 {
        let sin = radians.sin();
        let cos = radians.cos();

        ((cos, -sin, 0.0), (sin, cos, 0.0), (0.0, 0.0, 1.0)).into()
    }

    /// Calculates and returns a `Mat4` object representing the calling `Mat3` object rotated
    /// by a radians value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat3;
    ///
    /// let m = Mat3::identity().rotated(1.0);
    ///
    /// assert_eq!(m, ((0.5403023,  -0.84147096, 0.0),
    ///                (0.84147096,  0.5403023,  0.0),
    ///                (0.0,         0.0,        1.0)).into());
    /// ```
    pub fn rotated(&self, radians: f32) -> Mat3 {
        *self * Mat3::rotation(radians)
    }

    /// Rotates the calling `Mat3` object by a radians value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat3;
    ///
    /// let mut m = Mat3::identity();
    ///
    /// m.rotate(1.0);
    ///
    /// assert_eq!(m, ((0.5403023,  -0.84147096, 0.0),
    ///                (0.84147096,  0.5403023,  0.0),
    ///                (0.0,         0.0,        1.0)).into());
    /// ```
    pub fn rotate(&mut self, radians: f32) {
        *self = self.rotated(radians);
    }

    /// Calculates and returns a `Mat3` object representing the calling `Mat3` object scaled
    /// by a `Vec2<f32>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat3, Vec2};
    ///
    /// let m = Mat3::identity();
    ///
    /// assert_eq!(m.scaled(Vec2::new(1.0, 2.0)), ((1.0, 0.0, 0.0),
    ///                                            (0.0, 2.0, 0.0),
    ///                                            (0.0, 0.0, 1.0)).into());
    /// ```
    pub fn scaled(&self, factor: Vec2<f32>) -> Mat3 {
        let mut matrix = *self;

        matrix[0] *= factor.x;
        matrix[1] *= factor.y;

        matrix
    }

    /// Performs the scale operation on the calling `Mat3` object, scaling it by a `Vec2<f32>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat3, Vec2};
    ///
    /// let mut m = Mat3::identity();
    ///
    /// m.scale(Vec2::new(1.0, 2.0));
    ///
    /// assert_eq!(m, ((1.0, 0.0, 0.0),
    ///                (0.0, 2.0, 0.0),
    ///                (0.0, 0.0, 1.0)).into());
    /// ```
    pub fn scale(&mut self, factor: Vec2<f32>) {
        *self = self.scaled(factor);
    }

    /// Calculates and returns a `Mat3` object representing the calling `Mat3` object translated
    /// by a `Vec2<f32>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat3, Vec2};
    ///
    /// let m = Mat3::identity();
    ///
    /// assert_eq!(m.translated(Vec2::new(1.0, 2.0)), ((1.0, 0.0, 0.0),
    ///                                                (0.0, 1.0, 0.0),
    ///                                                (1.0, 2.0, 1.0)).into());
    /// ```
    pub fn translated(&self, translation: Vec2<f32>) -> Mat3 {
        let mut result = *self;

        result[2][0] += self[0][0] * translation.x + self[1][0] * translation.y;

        result[2][1] += self[0][1] * translation.x + self[1][1] * translation.y;

        result[2][2] += self[0][2] * translation.x + self[1][2] * translation.y;

        result
    }

    /// Performs the translate operation on the calling `Mat3` object, translating it by a
    /// `Vec2<f32>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat3, Vec2};
    ///
    /// let mut m = Mat3::identity();
    ///
    /// m.translate(Vec2::new(1.0, 2.0));
    ///
    /// assert_eq!(m, ((1.0, 0.0, 0.0),
    ///                (0.0, 1.0, 0.0),
    ///                (1.0, 2.0, 1.0)).into());
    /// ```
    pub fn translate(&mut self, translation: Vec2<f32>) {
        *self = self.translated(translation);
    }
}

impl Default for Mat3 {
    fn default() -> Mat3 {
        ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)).into()
    }
}

impl From<f32> for Mat3 {
    fn from(value: f32) -> Mat3 {
        Mat3 {
            rows: [
                (value, 0.0, 0.0).into(),
                (0.0, value, 0.0).into(),
                (0.0, 0.0, value).into(),
            ],
        }
    }
}

impl From<(Row, Row, Row)> for Mat3 {
    fn from(tuple: (Row, Row, Row)) -> Mat3 {
        Mat3 {
            rows: [tuple.0.into(), tuple.1.into(), tuple.2.into()],
        }
    }
}

impl From<InlineMat3> for Mat3 {
    fn from(tuple: InlineMat3) -> Mat3 {
        Mat3 {
            rows: [
                (tuple.0, tuple.1, tuple.2).into(),
                (tuple.3, tuple.4, tuple.5).into(),
                (tuple.6, tuple.7, tuple.8).into(),
            ],
        }
    }
}

impl From<[[f32; 3]; 3]> for Mat3 {
    fn from(slice: [[f32; 3]; 3]) -> Mat3 {
        Mat3 {
            rows: [slice[0].into(), slice[1].into(), slice[2].into()],
        }
    }
}

impl From<[f32; 9]> for Mat3 {
    fn from(slice: [f32; 9]) -> Mat3 {
        Mat3 {
            rows: [
                (slice[0], slice[1], slice[2]).into(),
                (slice[3], slice[4], slice[5]).into(),
                (slice[6], slice[7], slice[8]).into(),
            ],
        }
    }
}

impl From<[Vec3<f32>; 3]> for Mat3 {
    fn from(slice: [Vec3<f32>; 3]) -> Mat3 {
        Mat3 {
            rows: [slice[0], slice[1], slice[2]],
        }
    }
}

impl From<(Vec3<f32>, Vec3<f32>, Vec3<f32>)> for Mat3 {
    fn from(tuple: (Vec3<f32>, Vec3<f32>, Vec3<f32>)) -> Mat3 {
        Mat3 {
            rows: [tuple.0, tuple.1, tuple.2],
        }
    }
}

impl std::ops::Index<usize> for Mat3 {
    type Output = Vec3<f32>;

    fn index(&self, index: usize) -> &Vec3<f32> {
        match index {
            0 => &self.rows[0],
            1 => &self.rows[1],
            2 => &self.rows[2],
            _ => panic!("Mat3 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut Vec3<f32> {
        match index {
            0 => &mut self.rows[0],
            1 => &mut self.rows[1],
            2 => &mut self.rows[2],
            _ => panic!("Mat3 index out of range!"),
        }
    }
}

impl std::ops::Index<(usize, usize)> for Mat3 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        &self.rows[index.0][index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Mat3 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        &mut self.rows[index.0][index.1]
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

impl std::ops::Mul<Vec3<f32>> for Mat3 {
    type Output = Vec3<f32>;

    fn mul(self, vec: Vec3<f32>) -> Vec3<f32> {
        (self[0].dot(vec), self[1].dot(vec), self[2].dot(vec)).into()
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, right: Mat3) -> Mat3 {
        let mut result: Mat3 = 0.0.into();

        result[0][0] =
            self[0][0] * right[0][0] + self[1][0] * right[0][1] + self[2][0] * right[0][2];
        result[0][1] =
            self[0][1] * right[0][0] + self[1][1] * right[0][1] + self[2][1] * right[0][2];
        result[0][2] =
            self[0][2] * right[0][0] + self[1][2] * right[0][1] + self[2][2] * right[0][2];

        result[1][0] =
            self[0][0] * right[1][0] + self[1][0] * right[1][1] + self[2][0] * right[1][2];
        result[1][1] =
            self[0][1] * right[1][0] + self[1][1] * right[1][1] + self[2][1] * right[1][2];
        result[1][2] =
            self[0][2] * right[1][0] + self[1][2] * right[1][1] + self[2][2] * right[1][2];

        result[2][0] =
            self[0][0] * right[2][0] + self[1][0] * right[2][1] + self[2][0] * right[2][2];
        result[2][1] =
            self[0][1] * right[2][0] + self[1][1] * right[2][1] + self[2][1] * right[2][2];
        result[2][2] =
            self[0][2] * right[2][0] + self[1][2] * right[2][1] + self[2][2] * right[2][2];

        result
    }
}

impl std::ops::MulAssign<Mat3> for Mat3 {
    fn mul_assign(&mut self, right: Mat3) {
        *self = *self * right;
    }
}
