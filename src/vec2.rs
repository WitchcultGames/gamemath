use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut};
use std::fmt::Debug;
use vec3::Vec3;
use vec4::Vec4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
    where T: Copy + Debug + PartialEq + Default + Mul<Output = T> + Add<Output = T>,
{
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 {
            x,
            y,
        }
    }

    pub fn dot(&self, right: Vec2<T>) -> T {
        self.x * right.x + self.y * right.y
    }

    pub fn fill(&mut self, value: T) {
        self.x = value;
        self.y = value;
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl Vec2<f32> {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

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

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl Vec2<f64> {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

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
