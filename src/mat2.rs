use std;
use vec2::Vec2;

type Row = (f32, f32);
type InlineMat2 = (f32, f32, f32, f32);

/// A 2x2-component Euclidean matrix useful for linear algebra computation in game development
/// and 2D rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat2 {
    /// The two rows of the matrix, represented by an array of two `Vec2<f32>` objects.
    pub rows: [Vec2<f32>; 2],
}

impl Mat2 {
    /// Constructs a 2x2 identity matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat2, Vec2};
    ///
    /// let m = Mat2::identity();
    ///
    /// assert_eq!(m[0], Vec2::new(1.0, 0.0));
    /// assert_eq!(m[1], Vec2::new(0.0, 1.0));
    /// ```
    pub fn identity() -> Mat2 {
        Self::default()
    }

    /// Extracts and returns a transposed representation of the calling `Mat2` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat2;
    ///
    /// let m: Mat2 = ((0.0, 1.0),
    ///                (2.0, 3.0)).into();
    ///
    /// assert_eq!(m.transposed(), ((0.0, 2.0),
    ///                             (1.0, 3.0)).into());
    /// ```
    pub fn transposed(&self) -> Mat2 {
        (
            (self[0][0], self[1][0]),
            (self[0][1], self[1][1]),
        ).into()
    }

    /// Performs a transpose operation on the calling `Mat2` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat2;
    ///
    /// let mut m: Mat2 = ((0.0, 1.0),
    ///                    (3.0, 4.0)).into();
    ///
    /// m.transpose();
    ///
    /// assert_eq!(m, ((0.0, 3.0),
    ///                (1.0, 4.0)).into());
    /// ```
    pub fn transpose(&mut self) {
        *self = self.transposed();
    }

    /// Constructs a 2x2 rotation matrix from a radians value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat2, Vec2};
    ///
    /// let m = Mat2::rotation(1.0);
    ///
    /// assert_eq!(m[0], Vec2::new(0.5403023,  -0.84147096,));
    /// assert_eq!(m[1], Vec2::new(0.84147096,  0.54030230,));
    /// ```
    pub fn rotation(radians: f32) -> Mat2 {
        let sin = radians.sin();
        let cos = radians.cos();

        ((cos, -sin), (sin, cos)).into()
    }

    /// Calculates and returns a `Mat2` object representing the calling `Mat2` object rotated
    /// by a radians value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat2;
    ///
    /// let m = Mat2::identity().rotated(1.0);
    ///
    /// assert_eq!(m, ((0.5403023,  -0.84147096,),
    ///                (0.84147096,  0.54030230,)).into());
    /// ```
    pub fn rotated(&self, radians: f32) -> Mat2 {
        *self * Mat2::rotation(radians)
    }

    /// Rotates the calling `Mat2` object by a radians value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Mat2;
    ///
    /// let mut m = Mat2::identity();
    ///
    /// m.rotate(1.0);
    ///
    /// assert_eq!(m, ((0.5403023,  -0.84147096,),
    ///                (0.84147096,  0.54030230,)).into());
    /// ```
    pub fn rotate(&mut self, radians: f32) {
        *self = self.rotated(radians);
    }

    /// Calculates and returns a `Mat2` object representing the calling `Mat2` object scaled
    /// by a `Vec2<f32>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat2, Vec2};
    ///
    /// let m = Mat2::identity();
    ///
    /// assert_eq!(m.scaled(Vec2::new(3.0, 6.0)), ((3.0, 0.0),
    ///                                            (0.0, 6.0)).into());
    /// ```
    pub fn scaled(&self, factor: Vec2<f32>) -> Mat2 {
        let mut matrix = *self;

        matrix[0] *= factor.x;
        matrix[1] *= factor.y;

        matrix
    }

    /// Performs the scale operation on the calling `Mat2` object, scaling it by a `Vec2<f32>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Mat2, Vec2};
    ///
    /// let mut m = Mat2::identity();
    ///
    /// m.scale(Vec2::new(5.0, 2.0));
    ///
    /// assert_eq!(m, ((5.0, 0.0),
    ///                (0.0, 2.0)).into());
    /// ```
    pub fn scale(&mut self, factor: Vec2<f32>) {
        *self = self.scaled(factor);
    }
}

impl Default for Mat2 {
    fn default() -> Mat2 {
        ((1.0, 0.0), (0.0, 1.0)).into()
    }
}

impl From<f32> for Mat2 {
    fn from(value: f32) -> Mat2 {
        Mat2 {
            rows: [
                (value, 0.0).into(),
                (0.0, value).into(),
            ],
        }
    }
}

impl From<(Row, Row)> for Mat2 {
    fn from(tuple: (Row, Row)) -> Mat2 {
        Mat2 {
            rows: [tuple.0.into(), tuple.1.into()],
        }
    }
}

impl From<InlineMat2> for Mat2 {
    fn from(tuple: InlineMat2) -> Mat2 {
        Mat2 {
            rows: [
                (tuple.0, tuple.1).into(),
                (tuple.2, tuple.3).into(),
            ],
        }
    }
}

impl From<[[f32; 2]; 2]> for Mat2 {
    fn from(slice: [[f32; 2]; 2]) -> Mat2 {
        Mat2 {
            rows: [slice[0].into(), slice[1].into()],
        }
    }
}

impl From<[f32; 4]> for Mat2 {
    fn from(slice: [f32; 4]) -> Mat2 {
        Mat2 {
            rows: [
                (slice[0], slice[1]).into(),
                (slice[2], slice[3]).into(),
            ],
        }
    }
}

impl From<[Vec2<f32>; 2]> for Mat2 {
    fn from(slice: [Vec2<f32>; 2]) -> Mat2 {
        Mat2 {
            rows: [slice[0], slice[1]],
        }
    }
}

impl From<(Vec2<f32>, Vec2<f32>, Vec2<f32>)> for Mat2 {
    fn from(tuple: (Vec2<f32>, Vec2<f32>, Vec2<f32>)) -> Mat2 {
        Mat2 {
            rows: [tuple.0, tuple.1],
        }
    }
}

impl std::ops::Index<usize> for Mat2 {
    type Output = Vec2<f32>;

    fn index(&self, index: usize) -> &Vec2<f32> {
        match index {
            0 => &self.rows[0],
            1 => &self.rows[1],
            _ => panic!("Mat2 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Mat2 {
    fn index_mut(&mut self, index: usize) -> &mut Vec2<f32> {
        match index {
            0 => &mut self.rows[0],
            1 => &mut self.rows[1],
            _ => panic!("Mat2 index out of range!"),
        }
    }
}

impl std::ops::Index<(usize, usize)> for Mat2 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        &self.rows[index.0][index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Mat2 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f32 {
        &mut self.rows[index.0][index.1]
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

impl std::ops::Mul<Vec2<f32>> for Mat2 {
    type Output = Vec2<f32>;

    fn mul(self, vec: Vec2<f32>) -> Vec2<f32> {
        (self[0].dot(vec), self[1].dot(vec)).into()
    }
}

impl std::ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;

    fn mul(self, right: Mat2) -> Mat2 {
        let mut result: Mat2 = 0.0.into();

        result[0][0] =
            self[0][0] * right[0][0] + self[1][0] * right[0][1];
        result[0][1] =
            self[0][1] * right[0][0] + self[1][1] * right[0][1];
        
        result[1][0] =
            self[0][0] * right[1][0] + self[1][0] * right[1][1];
        result[1][1] =
            self[0][1] * right[1][0] + self[1][1] * right[1][1];

        result
    }
}

impl std::ops::MulAssign<Mat2> for Mat2 {
    fn mul_assign(&mut self, right: Mat2) {
        *self = *self * right;
    }
}
