use std;
use vec3::Vec3;
use vec4::Vec4;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 {
            x,
            y,
        }
    }

    pub fn dot(&self, right: Vec2) -> f32 {
        self.x * right.x + self.y * right.y
    }

    pub fn fill(&mut self, value: f32) {
        self.x = value;
        self.y = value;
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vec2 {
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

impl Default for Vec2 {
    fn default() -> Vec2 {
        Vec2 {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(tuple: (f32, f32)) -> Vec2 {
        Vec2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(slice: [f32; 2]) -> Vec2 {
        Vec2 {
            x: slice[0],
            y: slice[1],
        }
    }
}

impl From<Vec3> for Vec2 {
    fn from(vector: Vec3) -> Vec2 {
        Vec2 {
            x: vector.x,
            y: vector.y,
        }
    }
}

impl From<Vec4> for Vec2 {
    fn from(vector: Vec4) -> Vec2 {
        Vec2 {
            x: vector.x,
            y: vector.y,
        }
    }
}

impl From<f32> for Vec2 {
    fn from(value: f32) -> Vec2 {
        Vec2 {
            x: value,
            y: value,
        }
    }
}

impl std::ops::Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index out of range!"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 index out of range!"),
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, right: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + right.x,
            y: self.y + right.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, right: Vec2) {
        self.x += right.x;
        self.y += right.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, right: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - right.x,
            y: self.y - right.y,
        }
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, right: Vec2) {
        self.x -= right.x;
        self.y -= right.y;
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, right: f32) -> Vec2 {
        Vec2 {
            x: self.x * right,
            y: self.y * right,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, right: f32) {
        self.x *= right;
        self.y *= right;
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
