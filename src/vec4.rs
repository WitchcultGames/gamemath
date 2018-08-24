use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg, Index, IndexMut};
use std::fmt::Debug;
use vec2::Vec2;
use vec3::Vec3;
//use quat::Quat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T>
    where T: Copy + Debug + PartialEq + Default + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 {
            x,
            y,
            z,
            w,
        }
    }

    pub fn dot(&self, right: Vec4<T>) -> T {
        self.x * right.x + self.y * right.y + self.z * right.z + self.w * right.w
    }

    pub fn fill(&mut self, value: T) {
        self.x = value;
        self.y = value;
        self.z = value;
        self.w = value;
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl Vec4<f32> {
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

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
