use vec3::Vec3;
use vec4::Vec4;
use mat4::Mat4;
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// A quaternion data type used for representing spatial rotation in a 3D enviornment.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat {
    /// The X/first component of the quaternion.
    pub x: f32,
    /// The Y/first component of the quaternion.
    pub y: f32,
    /// The Z/first component of the quaternion.
    pub z: f32,
    /// The W/first component of the quaternion.
    pub w: f32,
}

impl Quat {
    /// Constructs an identity quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Quat;
    ///
    /// let q = Quat::identity();
    ///
    /// assert_eq!(q, (0.0, 0.0, 0.0, 1.0).into());
    /// ```
    pub fn identity() -> Quat {
        Self::default()
    }

    /// Constructs a rotation quaternion from an angle and an axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Vec3, Quat};
    ///
    /// let q = Quat::rotation(1.0, Vec3::new(1.0, 2.0, 3.0));
    ///
    /// assert_eq!(q, (0.12813187, 0.25626373, 0.38439557, 0.87758255).into());
    /// ```
    pub fn rotation(radians: f32, axis: Vec3<f32>) -> Quat {
        let a = axis.normalized();
        let r = radians / 2.0;
        let s = r.sin();

        Quat {
            x: a.x * s,
            y: a.y * s,
            z: a.z * s,
            w: r.cos(),
        }
    }

    /// Calculate and returns a quaternion representing the calling object rotated by an angle
    /// around an axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Vec3, Quat};
    ///
    /// let q = Quat::identity();
    ///
    /// assert_eq!(q.rotated(1.0, Vec3::new(1.0, 2.0, 3.0)), (0.12813187, 0.25626373, 0.38439557, 0.87758255).into());
    /// ```
    pub fn rotated(&self, radians: f32, axis: Vec3<f32>) -> Quat {
        *self * Quat::rotation(radians, axis)
    }

    /// Applies a rotation around and axis by an angle on the calling `Quat` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Vec3, Quat};
    ///
    /// let mut q = Quat::identity();
    ///
    /// q.rotate(1.0, Vec3::new(1.0, 2.0, 3.0));
    ///
    /// assert_eq!(q, (0.12813187, 0.25626373, 0.38439557, 0.87758255).into());
    /// ```
    pub fn rotate(&mut self, radians: f32, axis: Vec3<f32>) {
        *self = *self * Quat::rotation(radians, axis);
    }

    /// Calculates the squared length/magnitude/norm of a `Quat`.
    /// This saves an expensive square root calculation compared to calculating the actual length,
    /// and comparing two squared lengths can therefore often be cheaper than, and yield the same
    /// result as, computing two real lengths.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Quat;
    ///
    /// let q: Quat = (1.0, 2.0, 3.0, 4.0).into();
    ///
    /// assert_eq!(q.length_squared(), 30.0);
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Calculates the real length/magnitude/norm of a `Quat`.
    /// This results in an expensive square root calculation, and you might want to consider using
    /// a squared length instead when possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Quat;
    ///
    /// let q: Quat = (1.0, 4.0, 4.0, 16.0).into();
    ///
    /// assert_eq!(q.length(), 17.0);
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Calculates and returns the unit quaternion representation of a `Quat`.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Quat;
    ///
    /// let q: Quat = (1.0, 2.0, 2.0, 4.0).into();
    ///
    /// assert_eq!(q.normalized(), (0.2, 0.4, 0.4, 0.8).into());
    pub fn normalized(&self) -> Quat {
        let f = 1.0 / self.length();

        Quat {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
            w: self.w * f,
        }
    }

    /// Normalizes a `Quat` into its unit quaternion representation.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Quat;
    ///
    /// let mut q: Quat = (1.0, 2.0, 2.0, 4.0).into();
    ///
    /// q.normalize();
    ///
    /// assert_eq!(q, (0.2, 0.4, 0.4, 0.8).into());
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    /// Calculates and returns a `Mat4` object representing the rotation of the calling `Quat`
    /// object.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::{Vec3, Mat4, Quat};
    ///
    /// let q = Quat::rotation(1.0, Vec3::new(1.0, 2.0, 3.0));
    ///
    /// assert_eq!(q.extract_matrix(), (( 0.5731379,  0.74034876, -0.35127854, 0.0),
    ///                                 (-0.6090066,  0.67164457,  0.42190588, 0.0),
    ///                                 ( 0.5482918, -0.027879298, 0.8358222,  0.0),
    ///                                 ( 0.0,        0.0,         0.0,        1.0)).into());
    /// ```
    pub fn extract_matrix(&self) -> Mat4 {
        let mut result = Mat4::identity();;
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let yx = y * x2;
        let yy = y * y2;
        let zx = z * x2;
        let zy = z * y2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        result[0][0] = 1.0 - yy - zz;
        result[0][1] = yx + wz;
        result[0][2] = zx - wy;
        result[1][0] = yx - wz;
        result[1][1] = 1.0 - xx - zz;
        result[1][2] = zy + wx;
        result[2][0] = zx + wy;
        result[2][1] = zy - wx;
        result[2][2] = 1.0 - xx - yy;

        result
    }
}

impl Default for Quat {
    fn default() -> Quat {
        Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl From<f32> for Quat {
    fn from(value: f32) -> Quat {
        Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: value,
        }
    }
}

impl From<Vec4<f32>> for Quat {
    fn from(vec: Vec4<f32>) -> Quat {
        Quat {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: vec.w,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Quat {
    fn from(tuple: (f32, f32, f32, f32)) -> Quat {
        Quat {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}

impl From<[f32; 4]> for Quat {
    fn from(slice: [f32; 4]) -> Quat {
        Quat {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, right: Quat) -> Quat {
        Quat {
            x: (right.w * self.x) + (right.x * self.w) + (right.y * self.z) - (right.z * self.y),
            y: (right.w * self.y) + (right.y * self.w) + (right.z * self.x) - (right.x * self.z),
            z: (right.w * self.z) + (right.z * self.w) + (right.x * self.y) - (right.y * self.x),
            w: (right.w * self.w) - (right.x * self.x) - (right.y * self.y) - (right.z * self.z),
        }
    }
}

impl MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, right: Quat) {
        *self = *self * right;
    }
}

impl Add<Quat> for Quat {
    type Output = Quat;

    fn add(self, right: Quat) -> Quat {
        Quat {
            x: self.x + right.x,
            y: self.y + right.y,
            z: self.z + right.z,
            w: self.w + right.w,
        }
    }
}

impl AddAssign<Quat> for Quat {
    fn add_assign(&mut self, right: Quat) {
        *self = *self + right;
    }
}
