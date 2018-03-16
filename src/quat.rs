use vec3::Vec3;
use mat4::Mat4;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub fn identity() -> Quat {
        Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn rotation(axis: Vec3, angle: f32) -> Quat {
        let r = angle / 2.0;
        let s = r.sin();

        Quat {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: r.cos(),
        }
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        *self = *self * Quat::rotation(axis.normalized(), angle);
    }

    pub fn rotated(&mut self, axis: Vec3, angle: f32) -> Quat {
        *self * Quat::rotation(axis.normalized(), angle)
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Quat {
        let f = 1.0 / self.length();

        Quat {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
            w: self.w * f,
        }
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

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

//impl Into<Mat4> for Quat {
//    fn into(self) -> Mat4 {
//        let mut result = Mat4::identity();
//        let x2 = self.x * 2.0;
//        let y2 = self.y * 2.0;
//        let z2 = self.z * 2.0;
//        let xx = self.x * x2;
//        let yx = self.y * x2;
//        let zx = self.z * x2;
//        let wx = self.w * x2;
//        let yy = self.y * y2;
//        let zy = self.z * y2;
//        let wy = self.w * y2;
//        let zz = self.z * z2;
//        let wz = self.w * z2;
//
//        result[0][0] = 1.0 - yy - zz;
//        result[0][1] = yx + wz;
//        result[0][2] = zx - wy;
//        result[1][0] = yx - wz;
//        result[1][1] = 1.0 - xx - zz;
//        result[1][2] = zy + wx;
//        result[2][0] = zx + wy;
//        result[2][1] = zy - wx;
//        result[2][2] = 1.0 - xx - yy;
//
//        result
//    }
//}
