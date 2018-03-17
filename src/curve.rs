use super::lerp;

pub struct Curve(Vec<f32>);

impl Curve {
    pub fn new(values: &[f32]) -> Curve {
        Curve {
            0: values.into(),
        }
    }

    pub fn lerp(&self, factor: f32) -> f32 {
        let len = self.0.len();

        if len > 1 {
            if factor < 1.0 {
                let factor_scaled = factor * (len - 1) as f32;
                let start = self.0[factor_scaled as usize];
                let end = self.0[(factor_scaled + 1.0) as usize];
                let factor_new = factor_scaled - (factor_scaled as u32) as f32;

                lerp(start, end, factor_new)
            } else {
                self.0[len - 1]
            }
        } else if len == 1 {
            self.0[0]
        } else {
            1.0
        }
    }
}
