use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut};
use std::fmt::Debug;
use vec3::Vec3;
use vec4::Vec4;

/// A two-component Euclidean vector usefull for linear algebra computation in game development
/// and 3D rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2<T> {
    /// The X/first component of the vector.
    pub x: T,
    /// The Y/second component of the vector.
    pub y: T,
}

impl<T> Vec2<T>
    where T: Copy + Debug + PartialEq + Default + Mul<Output = T> + Add<Output = T>,
{
    /// Constructs a new `Vec2<T>` from two initial values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let v = Vec2::new(1.0, 5.0);
    ///
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 5.0);
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x,
            y,
        }
    }

    /// Calculates the dot/scalar product of two `Vec2<T>`s.
    /// The calling object is considered the left value and the argument object is considered the
    /// right value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let v1 = Vec2::new(1.0, 2.0);
    /// let v2 = Vec2::new(3.0, 4.0);
    ///
    /// assert_eq!(v1.dot(v2), 11.0);
    /// assert_eq!(v2.dot(v1), 11.0);
    /// ```
    pub fn dot(&self, right: Vec2<T>) -> T {
        self.x * right.x + self.y * right.y
    }

    /// Fills all components of the calling `Vec2<T>` with the provided value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let mut v = Vec2::new(0.0, 0.0);
    ///
    /// v.fill(6.0);
    ///
    /// assert_eq!(v, Vec2::new(6.0, 6.0));
    pub fn fill(&mut self, value: T) {
        self.x = value;
        self.y = value;
    }

    /// Calculates the squared length/magnitude/norm of a `Vec2<T>`.
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
    /// use gamemath::Vec2;
    ///
    /// let v = Vec2::new(1.0, 2.0);
    ///
    /// assert_eq!(v.length_squared(), 5.0);
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl Vec2<f32> {
    /// Calculates the real length/magnitude/norm of a `Vec2<f32>`.
    /// This results in an expensive square root calculation, and you might want to consider using
    /// a squared length instead when possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let v = Vec2::new(3.0_f32, 4.0_f32);
    ///
    /// assert_eq!(v.length(), 5.0_f32);
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Calculates and returns the unit vector representation of a `Vec2<f32>`.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let v = Vec2::new(3.0_f32, 4.0_f32);
    ///
    /// assert_eq!(v.normalized(), Vec2::new(0.6_f32, 0.8_f32));
    pub fn normalized(&self) -> Vec2<f32> {
        let mut length = self.length();

        if length == 0.0 {
            length = 1.0;
        }

        Vec2 {
            x: self.x / length,
            y: self.y / length,
        }
    }

    /// Normalizes a `Vec2<f32>` into its unit vector representation.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let mut v = Vec2::new(3.0_f32, 4.0_f32);
    ///
    /// v.normalize();
    ///
    /// assert_eq!(v, Vec2::new(0.6_f32, 0.8_f32));
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl Vec2<f64> {
    /// Calculates the real length/magnitude/norm of a `Vec2<f64>`.
    /// This results in an expensive square root calculation, and you might want to consider using
    /// a squared length instead when possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let v = Vec2::new(3.0_f64, 4.0_f64);
    ///
    /// assert_eq!(v.length(), 5.0_f64);
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculates and returns the unit vector representation of a `Vec2<f64>`.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let v = Vec2::new(3.0_f64, 4.0_f64);
    ///
    /// assert_eq!(v.normalized(), Vec2::new(0.6_f64, 0.8_f64));
    pub fn normalized(&self) -> Vec2<f64> {
        let mut length = self.length();

        if length == 0.0 {
            length = 1.0;
        }

        Vec2 {
            x: self.x / length,
            y: self.y / length,
        }
    }

    /// Normalizes a `Vec2<f32>` into its unit vector representation.
    /// This results in an an expensive square root calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Vec2;
    ///
    /// let mut v = Vec2::new(3.0_f64, 4.0_f64);
    ///
    /// v.normalize();
    ///
    /// assert_eq!(v, Vec2::new(0.6_f64, 0.8_f64));
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl<T> Default for Vec2<T>
    where T: Default,
{
    fn default() -> Vec2<T> {
        Vec2 {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Vec2<T> {
        Vec2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(slice: [T; 2]) -> Vec2<T> {
        Vec2 {
            x: slice[0],
            y: slice[1],
        }
    }
}

impl<T> From<Vec3<T>> for Vec2<T> {
    fn from(vector: Vec3<T>) -> Vec2<T> {
        Vec2 {
            x: vector.x,
            y: vector.y,
        }
    }
}

impl<T> From<Vec4<T>> for Vec2<T> {
    fn from(vector: Vec4<T>) -> Vec2<T> {
        Vec2 {
            x: vector.x,
            y: vector.y,
        }
    }
}

impl<T: Copy> From<T> for Vec2<T> {
    fn from(value: T) -> Vec2<T> {
        Vec2 {
            x: value,
            y: value,
        }
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index out of range!"),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 index out of range!"),
        }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, right: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + right.x,
            y: self.y + right.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, right: Vec2<T>) {
        self.x += right.x;
        self.y += right.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, right: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - right.x,
            y: self.y - right.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, right: Vec2<T>) {
        self.x -= right.x;
        self.y -= right.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, right: T) -> Vec2<T> {
        Vec2 {
            x: self.x * right,
            y: self.y * right,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, right: T) {
        self.x *= right;
        self.y *= right;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Vec2<T> {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
