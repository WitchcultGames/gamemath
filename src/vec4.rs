use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut};
use std::fmt::Debug;
use vec2::Vec2;
use vec3::Vec3;
//use quat::Quat;

/// A four-component Euclidean vector usefull for linear algebra computation in game development
/// and 3D rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4<T> {
    /// The X/first component of the vector.
    pub x: T,
    /// The Y/second component of the vector.
    pub y: T,
    /// The Z/third component of the vector.
    pub z: T,
    /// The W/fourth component of the vector.
    pub w: T,
}

impl<T> Vec4<T>
    where T: Copy + Debug + PartialEq + Default + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    /// Constructs a new `Vec4<T>` from three initial values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let v = Vec4::new(1.0, 5.0, 23.0, -7.0);
    ///
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 5.0);
    /// assert_eq!(v.z, 23.0);
    /// assert_eq!(v.w, -7.0);
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 {
            x,
            y,
            z,
            w,
        }
    }

    /// Calculates the dot/scalar product of two `Vec4<T>`s.
    ///
    /// The calling object is considered the left value and the argument object is considered the
    /// right value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vec4::new(5.0, 6.0, 7.0, 8.0);
    ///
    /// assert_eq!(v1.dot(v2), 70.0);
    /// assert_eq!(v2.dot(v1), 70.0);
    /// ```
    pub fn dot(&self, right: Vec4<T>) -> T {
        self.x * right.x + self.y * right.y + self.z * right.z + self.w * right.w
    }

    /// Fills all components of the calling `Vec4<T>` with the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let mut v = Vec4::new(0.0, 0.0, 0.0, 0.0);
    ///
    /// v.fill(6.0);
    ///
    /// assert_eq!(v, Vec4::new(6.0, 6.0, 6.0, 6.0));
    pub fn fill(&mut self, value: T) {
        self.x = value;
        self.y = value;
        self.z = value;
        self.w = value;
    }

    /// Calculates the squared length/magnitude/norm of a `Vec4<T>`.
    /// This saves an expensive square root calculation compared to calculating the actual length,
    /// and comparing two squared lengths can therefore often be cheaper than, and yield the same
    /// result as, computing two real lengths.
    ///
    /// Also usefull for data types that does not implement a square root function, i.e.
    /// non-floating-point data types.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
    ///
    /// assert_eq!(v.length_squared(), 30.0);
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl Vec4<f32> {
    /// Calculates the real length/magnitude/norm of a `Vec4<f32>`.
    /// This results in an expensive square root calculation, and you might want to consider using
    /// a squared length instead when possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let v = Vec4::new(1.0_f32, 4.0_f32, 4.0_f32, 16.0_f32);
    ///
    /// assert_eq!(v.length(), 17.0_f32);
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Calculates and returns the unit vector representation of a `Vec4<f32>`.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let v = Vec4::new(1.0_f32, 2.0_f32, 2.0_f32, 4.0_f32);
    ///
    /// assert_eq!(v.normalized(), Vec4::new(0.2_f32, 0.4_f32, 0.4_f32, 0.8_f32));
    pub fn normalized(&self) -> Vec4<f32> {
        let mut length = self.length();

        if length == 0.0 {
            length = 1.0;
        }

        Vec4 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w / length,
        }
    }

    /// Normalizes a `Vec4<f32>` into its unit vector representation.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let mut v = Vec4::new(1.0_f32, 2.0_f32, 2.0_f32, 4.0_f32);
    ///
    /// v.normalize();
    ///
    /// assert_eq!(v, Vec4::new(0.2_f32, 0.4_f32, 0.4_f32, 0.8_f32));
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl Vec4<f64> {
    /// Calculates the real length/magnitude/norm of a `Vec4<f64>`.
    /// This results in an expensive square root calculation, and you might want to consider using
    /// a squared length instead when possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let v = Vec4::new(1.0_f64, 4.0_f64, 4.0_f64, 16.0_f64);
    ///
    /// assert_eq!(v.length(), 17.0_f64);
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculates and returns the unit vector representation of a `Vec4<f64>`.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let v = Vec4::new(1.0_f64, 2.0_f64, 2.0_f64, 4.0_f64);
    ///
    /// assert_eq!(v.normalized(), Vec4::new(0.2_f64, 0.4_f64, 0.4_f64, 0.8_f64));
    pub fn normalized(&self) -> Vec4<f64> {
        let mut length = self.length();

        if length == 0.0 {
            length = 1.0;
        }

        Vec4 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w / length,
        }
    }

    /// Normalizes a `Vec4<f64>` into its unit vector representation.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec4;
    ///
    /// let mut v = Vec4::new(1.0_f64, 2.0_f64, 2.0_f64, 4.0_f64);
    ///
    /// v.normalize();
    ///
    /// assert_eq!(v, Vec4::new(0.2_f64, 0.4_f64, 0.4_f64, 0.8_f64));
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl<T: Default> Default for Vec4<T> {
    fn default() -> Vec4<T> {
        Vec4 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
        }
    }
}

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from(tuple: (T, T, T, T)) -> Vec4<T>{
        Vec4 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}

impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(slice: [T; 4]) -> Vec4<T> {
        Vec4 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl<T: Default> From<Vec2<T>> for Vec4<T> {
    fn from(vec: Vec2<T>) -> Vec4<T> {
        Vec4 {
            x: vec.x,
            y: vec.y,
            z: T::default(),
            w: T::default(),
        }
    }
}

impl<T: Default> From<Vec3<T>> for Vec4<T> {
    fn from(vec: Vec3<T>) -> Vec4<T> {
        Vec4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: T::default(),
        }
    }
}

//impl From<Quat> for Vec4 {
//    fn from(quat: Quat) -> Vec4 {
//        Vec4 {
//            x: quat.x,
//            y: quat.y,
//            z: quat.z,
//            w: quat.w,
//        }
//    }
//}

impl<T: Copy> From<T> for Vec4<T> {
    fn from(value: T) -> Vec4<T> {
        Vec4 {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Vec4 index out of range!"),
        }
    }
}

impl<T> IndexMut<usize> for Vec4<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Vec4 index out of range!"),
        }
    }
}

impl<T: Add<Output = T>> Add for Vec4<T> {
    type Output = Vec4<T>;

    fn add(self, right: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x + right.x,
            y: self.y + right.y,
            z: self.z + right.z,
            w: self.w + right.w,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec4<T> {
    fn add_assign(&mut self, right: Vec4<T>) {
        self.x += right.x;
        self.y += right.y;
        self.z += right.z;
        self.w += right.w;
    }
}

impl<T: Sub<Output = T>> Sub for Vec4<T> {
    type Output = Vec4<T>;

    fn sub(self, right: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x - right.x,
            y: self.y - right.y,
            z: self.z - right.z,
            w: self.w - right.w,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec4<T> {
    fn sub_assign(&mut self, right: Vec4<T>) {
        self.x -= right.x;
        self.y -= right.y;
        self.z -= right.z;
        self.w -= right.w;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Vec4<T>;

    fn mul(self, right: T) -> Vec4<T> {
        Vec4 {
            x: self.x * right,
            y: self.y * right,
            z: self.z * right,
            w: self.w * right,
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, right: T) {
        self.x *= right;
        self.y *= right;
        self.z *= right;
        self.w *= right;
    }
}

impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Vec4<T>;

    fn neg(self) -> Vec4<T> {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
