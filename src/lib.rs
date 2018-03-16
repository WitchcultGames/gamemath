pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod quat;
pub mod curve;

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    let t_clamped = 0.0_f32.max(1.0_f32.min(t));
    let diff = end - start;

    (1.0 - t_clamped) * start + t_clamped * (start + diff)
}
