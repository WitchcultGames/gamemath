//! A simple math library for game development written in Rust.
//! You can get more information about this crate in [README.md](https://github.com/WitchcultGames/gamemath/blob/master/README.md)

pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat4;
pub mod quat;
pub mod curve;

mod private;
use private::num::PrivateIntoNum;

/// A function to get value between ```start``` and ```end```  
/// Example:  
/// ```Rust
/// assert_eq!(lerp(1, 3, 0.5), 2.0);
/// assert_eq!(lerp(0, 1.5, 0.5), 0.75);
/// ```
pub fn lerp<T: PrivateIntoNum, V: PrivateIntoNum>(start: T, end: V, factor: f32) -> f32 {
    let factor_clamped = 0.0_f32.max(1.0_f32.min(factor));
    let _start: f32 = start.into().into();
    let _end: f32 = end.into().into();
    (1.0 - factor_clamped) * _start + factor_clamped * _end
}

#[test]
fn test_lerp() {
    let temp = lerp(1, 3, 0.5);
    assert_eq!(temp, 2.0);
    let temp = lerp(2.0, 4.0, 0.5);
    assert_eq!(temp, 3.0);
    let temp = lerp(0, 1.5, 0.5);
    assert_eq!(temp, 0.75);
}
