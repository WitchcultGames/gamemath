use std::ops::{Index, IndexMut};

/// A heap allocated structure for representing a value curve.
pub struct Curve<T>(Vec<T>);

impl<T> Curve<T> where
    T: Default + Clone + Copy + From<f32> + Into<f32>,
{
    /// Constructs a `Curve` from a slice of values.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Curve;
    ///
    /// let c: Curve<f32> = Curve::new(&[0.0, 10.0, 5.0, 0.0]);
    ///
    /// assert_eq!(c[0], 0.0);
    /// assert_eq!(c[1], 10.0);
    /// assert_eq!(c[2], 5.0);
    /// assert_eq!(c[3], 0.0);
    /// ```
    pub fn new(values: &[T]) -> Curve<T> {
        Curve {
            0: values.into(),
        }
    }

    /// Linearly interpolates between the values of the curve by a factor.
    ///
    /// # Examples
    ///
    /// ```
    /// use gamemath::Curve;
    ///
    /// let c: Curve<f32> = Curve::new(&[0.0, 10.0, 5.0, 0.0]);
    ///
    /// assert_eq!(c.lerp(0.75), 3.75);
    /// ```
    pub fn lerp(&self, factor: f32) -> T {
        let len = self.0.len();

        if len > 1 {
            if factor < 1.0 {
                let factor_scaled = factor * (len - 1) as f32;
                let start = self.0[factor_scaled as usize];
                let end = self.0[(factor_scaled + 1.0) as usize];
                let new_factor = factor_scaled - (factor_scaled as u32) as f32;
                let factor_clamped = 0.0_f32.max(1.0_f32.min(new_factor));

                ((1.0 - factor_clamped) * start.into() + factor_clamped * end.into()).into()

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

impl<T> Index<usize> for Curve<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Curve<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.0[index]
    }
}
