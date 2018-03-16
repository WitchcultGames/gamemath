use std;
use vec2::Vec2;
use vec3::Vec3;
use quat::Quat;

#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x,
            y,
            z,
            w,
        }
    }

    pub fn dot(&self, right: Vec4) -> f32 {
        self.x * right.x + self.y * right.y + self.z * right.z + self.w * right.w
    }

    pub fn fill(&mut self, value: f32) {
        self.x = value;
        self.y = value;
        self.z = value;
        self.w = value;
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec4 {
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

impl Default for Vec4 {
    fn default() -> Vec4 {
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from(tuple: (f32, f32, f32, f32)) -> Vec4 {
        Vec4 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(slice: [f32; 4]) -> Vec4 {
        Vec4 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl From<Vec2> for Vec4 {
    fn from(vec: Vec2) -> Vec4 {
        Vec4 {
            x: vec.x,
            y: vec.y,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl From<Vec3> for Vec4 {
    fn from(vec: Vec3) -> Vec4 {
        Vec4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: 0.0,
        }
    }
}

impl From<Quat> for Vec4 {
    fn from(quat: Quat) -> Vec4 {
        Vec4 {
            x: quat.x,
            y: quat.y,
            z: quat.z,
            w: quat.w,
        }
    }
}

impl From<f32> for Vec4 {
    fn from(value: f32) -> Vec4 {
        Vec4 {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }
}

impl std::ops::Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Vec4 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Vec4 index out of range!"),
        }
    }
}

impl std::ops::Add for Vec4 {
    type Output = Vec4;

    fn add(self, right: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + right.x,
            y: self.y + right.y,
            z: self.z + right.z,
            w: self.w + right.w,
        }
    }
}

impl std::ops::AddAssign for Vec4 {
    fn add_assign(&mut self, right: Vec4) {
        self.x += right.x;
        self.y += right.y;
        self.z += right.z;
        self.w += right.w;
    }
}

impl std::ops::Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, right: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - right.x,
            y: self.y - right.y,
            z: self.z - right.z,
            w: self.w - right.w,
        }
    }
}

impl std::ops::SubAssign for Vec4 {
    fn sub_assign(&mut self, right: Vec4) {
        self.x -= right.x;
        self.y -= right.y;
        self.z -= right.z;
        self.w -= right.w;
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, right: f32) -> Vec4 {
        Vec4 {
            x: self.x * right,
            y: self.y * right,
            z: self.z * right,
            w: self.w * right,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, right: f32) {
        self.x *= right;
        self.y *= right;
        self.z *= right;
        self.w *= right;
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Vec4 {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
