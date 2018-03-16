use super::lerp;

pub struct Curve(Vec<f32>);

impl Curve {
    pub fn new(values: &[f32]) -> Curve {
        Curve {
            0: values.into(),
        }
    }

    pub fn lerp(&self, t: f32) -> f32 {
        let len = self.0.len();

        if len > 1 {
            if t < 1.0 {
                let scaled_t = t * (len - 1) as f32;
                let start = self.0[scaled_t as usize];
                let end = self.0[(scaled_t + 1.0) as usize];
                let new_t = scaled_t - (scaled_t as u32) as f32;

                lerp(start, end, new_t)
            } else if len > 0 {
                self.0[len - 1]
            } else {
                1.0
            }
        } else if len == 1 {
            self.0[0]
        } else {
            1.0
        }
    }
}
