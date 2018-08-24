use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut};
use std::fmt::Debug;
use vec2::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
    where T: Copy + Debug + PartialEq + Default + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn dot(&self, right: Vec3<T>) -> T {
        self.x * right.x + self.y * right.y + self.z * right.z
    }

    pub fn cross(&self, right: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * right.z - self.z * right.y,
            y: self.z * right.x - self.x * right.z,
            z: self.x * right.y - self.y * right.x,
        }
    }

    pub fn fill(&mut self, value: T) {
        self.x = value;
        self.y = value;
        self.z = value;
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Vec3<f32> {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

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

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl Vec3<f64> {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

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

//impl From<Vec4> for Vec3 {
//    fn from(vec: Vec4) -> Vec3 {
//        Vec3 {
//            x: vec.x,
//            y: vec.y,
//            z: vec.z,
//        }
//    }
//}

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
