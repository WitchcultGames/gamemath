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
