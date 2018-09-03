//! A simple math library containing the most common data structures used for 3D rendering and
//! general game development.

mod vec2;
mod vec3;
mod vec4;
mod mat4;
mod quat;
mod curve;

pub use self::vec2::Vec2;
pub use self::vec3::Vec3;
pub use self::vec4::Vec4;
pub use self::mat4::Mat4;
pub use self::quat::Quat;
pub use self::curve::Curve;

pub trait Lerp {
    fn lerp(self, other: Self, factor: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, other: f32, factor: f32) -> f32 {
        let factor_clamped = 0.0_f32.max(1.0_f32.min(factor));
        let (start, end) = match self < other {
            true => (self, other),
            false => (other, self),
        };

        (1.0 - factor_clamped) * start + factor_clamped * end
    }
}
