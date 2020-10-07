use std::fmt::Debug;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use vec2::Vec2;
use vec4::Vec4;

/// A three-component Euclidean vector useful for linear algebra computation in game development
/// and 3D rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    /// The X/first component of the vector.
    pub x: T,
    /// The Y/second component of the vector.
    pub y: T,
    /// The Z/third component of the vector.
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Copy
        + Debug
        + PartialEq
        + Default
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + PartialOrd,
{
    /// Constructs a new `Vec3<T>` from three initial values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v = Vec3::new(1.0, 5.0, 23.0);
    ///
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 5.0);
    /// assert_eq!(v.z, 23.0);
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    /// Calculates the dot/scalar product of two `Vec3<T>`s.
    ///
    /// The calling object is considered the left value and the argument object is considered the
    /// right value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v1 = Vec3::new(1.0, 2.0, 3.0);
    /// let v2 = Vec3::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(v1.dot(v2), 32.0);
    /// assert_eq!(v2.dot(v1), 32.0);
    /// ```
    pub fn dot(&self, right: Vec3<T>) -> T {
        self.x * right.x + self.y * right.y + self.z * right.z
    }

    /// Calculates the cross/vector product of two `Vec3<T>`s.
    ///
    /// The calling object is considered the left value and the argument object is considered the
    /// right value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v1 = Vec3::new(1.0, 2.0, 3.0);
    /// let v2 = Vec3::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(v1.cross(v2), Vec3::new(-3.0, 6.0, -3.0));
    /// assert_eq!(v2.cross(v1), Vec3::new(3.0, -6.0, 3.0));
    /// ```
    pub fn cross(&self, right: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * right.z - self.z * right.y,
            y: self.z * right.x - self.x * right.z,
            z: self.x * right.y - self.y * right.x,
        }
    }

    /// Fills all components of the calling `Vec3<T>` with the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let mut v = Vec3::new(0.0, 0.0, 0.0);
    ///
    /// v.fill(6.0);
    ///
    /// assert_eq!(v, Vec3::new(6.0, 6.0, 6.0));
    pub fn fill(&mut self, value: T) {
        self.x = value;
        self.y = value;
        self.z = value;
    }

    /// Calculates the squared length/magnitude/norm of a `Vec3<T>`.
    /// This saves an expensive square root calculation compared to calculating the actual length,
    /// and comparing two squared lengths can therefore often be cheaper than, and yield the same
    /// result as, computing two real lengths.
    ///
    /// Also useful for data types that does not implement a square root function, i.e.
    /// non-floating-point data types.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(v.length_squared(), 14.0);
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculates and returns the manhattan distance between the two points pointed to by two
    /// `Vec3<T>` objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v1 = Vec3::new(1.0, 2.0, 3.0);
    /// let v2 = Vec3::new(2.0, 4.0, 6.0);
    ///
    /// assert_eq!(v1.manhattan_distance(v2), 6.0);
    pub fn manhattan_distance(&self, right: Vec3<T>) -> T {
        let mut a = self.x - right.x;
        let mut b = self.y - right.y;
        let mut c = self.z - right.z;

        if a < T::default() {
            a = -a;
        }

        if b < T::default() {
            b = -b;
        }

        if c < T::default() {
            c = -c;
        }

        a + b + c
    }
}

impl Vec3<f32> {
    /// Calculates the real length/magnitude/norm of a `Vec3<f32>`.
    /// This results in an expensive square root calculation, and you might want to consider using
    /// a squared length instead when possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v = Vec3::new(1.0_f32, 4.0_f32, 8.0_f32);
    ///
    /// assert_eq!(v.length(), 9.0_f32);
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Calculates and returns the unit vector representation of a `Vec3<f32>`.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v = Vec3::new(9.0_f32, 12.0_f32, 20.0_f32);
    ///
    /// assert_eq!(v.normalized(), Vec3::new(0.36_f32, 0.48_f32, 0.8_f32));
    pub fn normalized(&self) -> Vec3<f32> {
        let mut length = self.length();

        if length == 0.0 {
            length = 1.0;
        }

        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    /// Normalizes a `Vec3<f32>` into its unit vector representation.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let mut v = Vec3::new(9.0_f32, 12.0_f32, 20.0_f32);
    ///
    /// v.normalize();
    ///
    /// assert_eq!(v, Vec3::new(0.36_f32, 0.48_f32, 0.8_f32));
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl Vec3<f64> {
    /// Calculates the real length/magnitude/norm of a `Vec3<f64>`.
    /// This results in an expensive square root calculation, and you might want to consider using
    /// a squared length instead when possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v = Vec3::new(1.0_f64, 4.0_f64, 8.0_f64);
    ///
    /// assert_eq!(v.length(), 9.0_f64);
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculates and returns the unit vector representation of a `Vec3<f64>`.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let v = Vec3::new(9.0_f64, 12.0_f64, 20.0_f64);
    ///
    /// assert_eq!(v.normalized(), Vec3::new(0.36_f64, 0.48_f64, 0.8_f64));
    pub fn normalized(&self) -> Vec3<f64> {
        let mut length = self.length();

        if length == 0.0 {
            length = 1.0;
        }

        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    /// Normalizes a `Vec3<f64>` into its unit vector representation.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec3;
    ///
    /// let mut v = Vec3::new(9.0_f64, 12.0_f64, 20.0_f64);
    ///
    /// v.normalize();
    ///
    /// assert_eq!(v, Vec3::new(0.36_f64, 0.48_f64, 0.8_f64));
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Vec3<T> {
        Vec3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(tuple: (T, T, T)) -> Vec3<T> {
        Vec3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(slice: [T; 3]) -> Vec3<T> {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl<T: Default> From<Vec2<T>> for Vec3<T> {
    fn from(vec: Vec2<T>) -> Vec3<T> {
        Vec3 {
            x: vec.x,
            y: vec.y,
            z: T::default(),
        }
    }
}

impl<T> From<Vec4<T>> for Vec3<T> {
    fn from(vec: Vec4<T>) -> Vec3<T> {
        Vec3 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl<T: Copy> From<T> for Vec3<T> {
    fn from(value: T) -> Vec3<T> {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of range!"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of range!"),
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, right: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + right.x,
            y: self.y + right.y,
            z: self.z + right.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, right: Vec3<T>) {
        self.x += right.x;
        self.y += right.y;
        self.z += right.z;
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, right: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - right.x,
            y: self.y - right.y,
            z: self.z - right.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, right: Vec3<T>) {
        self.x -= right.x;
        self.y -= right.y;
        self.z -= right.z;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, right: T) -> Vec3<T> {
        Vec3 {
            x: self.x * right,
            y: self.y * right,
            z: self.z * right,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, right: T) {
        self.x *= right;
        self.y *= right;
        self.z *= right;
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Vec3<T> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
