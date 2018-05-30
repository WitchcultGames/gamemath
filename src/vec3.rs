use std;
use vec2::Vec2;
use vec4::Vec4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn dot(&self, right: Vec3) -> f32 {
        self.x * right.x + self.y * right.y + self.z * right.z
    }

    pub fn cross(&self, right: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * right.z - self.z * right.y,
            y: self.z * right.x - self.x * right.z,
            z: self.x * right.y - self.y * right.x,
        }
    }

    pub fn fill(&mut self, value: f32) {
        self.x = value;
        self.y = value;
        self.z = value;
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
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

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(tuple: (f32, f32, f32)) -> Vec3 {
        Vec3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(slice: [f32; 3]) -> Vec3 {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl From<Vec2> for Vec3 {
    fn from(vec: Vec2) -> Vec3 {
        Vec3 {
            x: vec.x,
            y: vec.y,
            z: 0.0,
        }
    }
}

impl From<Vec4> for Vec3 {
    fn from(vec: Vec4) -> Vec3 {
        Vec3 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl From<f32> for Vec3 {
    fn from(value: f32) -> Vec3 {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of range!"),
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, right: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + right.x,
            y: self.y + right.y,
            z: self.z + right.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, right: Vec3) {
        self.x += right.x;
        self.y += right.y;
        self.z += right.z;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, right: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - right.x,
            y: self.y - right.y,
            z: self.z - right.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, right: Vec3) {
        self.x -= right.x;
        self.y -= right.y;
        self.z -= right.z;
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, right: f32) -> Vec3 {
        Vec3 {
            x: self.x * right,
            y: self.y * right,
            z: self.z * right,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, right: f32) {
        self.x *= right;
        self.y *= right;
        self.z *= right;
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
