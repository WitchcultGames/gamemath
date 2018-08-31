pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat4;
pub mod quat;
pub mod curve;

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
