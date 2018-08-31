use super::Lerp;

pub struct Curve<T>(Vec<T>);

impl<T: Lerp + Default + Clone + Copy> Curve<T> {
    pub fn new(values: &[T]) -> Curve<T> {
        Curve {
            0: values.into(),
        }
    }

    pub fn lerp(&self, factor: f32) -> T {
        let len = self.0.len();

        if len > 1 {
            if factor < 1.0 {
                let factor_scaled = factor * (len - 1) as f32;
                let start = self.0[factor_scaled as usize];
                let end = self.0[(factor_scaled + 1.0) as usize];
                let new_factor = factor_scaled - (factor_scaled as u32) as f32;

                T::lerp(start, end, new_factor)
            } else {
                self.0[len - 1]
            }
        } else if len == 1 {
            self.0[0]
        } else {
            T::default()
        }
    }
}
