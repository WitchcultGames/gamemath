pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat4;
pub mod quat;
pub mod curve;

pub fn lerp(start: f32, end: f32, factor: f32) -> f32 {
    let factor_clamped = 0.0_f32.max(1.0_f32.min(factor));

    (1.0 - factor_clamped) * start + factor_clamped * end
}
