use std::ops::IndexMut;

// LU decomposition used for solving A * x = b for x. 
// Generic implementation for all sizes of matrices and vectors.

pub trait LU<Vector: IndexMut<usize, Output = f32> + Default + Clone + Permute>:
  IndexMut<usize, Output = Vector> + Default + Zero + Dimensional
{
  /// Returns vector x such that self * x = b. 
  /// Numerically stable and fairly efficient (compared to computing matrix inverse).
  /// Matrix self must be of full rank.
  /// 
  /// ```
  /// use gamemath::{Mat3, Vec3, LU};
  ///
  /// let m: Mat3 = ((1.0, 3.0, 5.0),
  ///                (2.0, 4.0, 7.0),
  ///                (1.0, 1.0, 0.0)).into();
  ///
  /// let x = m.lu_solve(Vec3::from((5.0, 6.0, 10.0)));
  ///
  /// assert_eq!(x, Vec3::from((1.25, 8.75, -4.5)));
  /// ```
  #[allow(non_snake_case)]
  fn lu_solve(&self, b: Vector) -> Vector {
    let (L, U, p) = self.decompose_lu();

    let y = LU::solve_with_L(&L, b.permuted(&p));

    LU::solve_with_U(&U, y)
  }

  /// Returns vector y such that self * b = y.
  /// Self must be lower triangular matrix and have all diagonal elements equal 1.
  #[allow(non_snake_case)]
  fn solve_with_L(L: &Self, b: Vector) -> Vector {
    let mut y: Vector = Vector::default();

    for i in 0..Self::dimension() {
      let mut sum = 0.0;
      for j in 0..i {
        sum += y[j] * L[i][j];
      }
      y[i] = b[i] - sum;
    }
    y
  }

  /// Returns vector x such that self * y = x.
  /// Self must be upper triangular matrix.
  #[allow(non_snake_case)]
  fn solve_with_U(U: &Self, y: Vector) -> Vector {
    let mut x: Vector = Vector::default();

    for i in (0..Self::dimension()).rev() {
      let mut sum = 0.0;
      for j in i + 1..Self::dimension() {
        sum += x[j] * U[i][j];
      }
      x[i] = (y[i] - sum) / U[i][i];
    }
    x
  }

  /// Returns row permutation such that for every i, is
  /// the element (i, i) greater or equal to every element (i, j) for j > i.
  ///
  /// (Element (i, i) is greater than all elements below it.)
  ///
  /// ```
  /// use gamemath::{Mat3, LU};
  ///
  /// let m: Mat3 = ((1.0, 3.0, 5.0),
  ///                (2.0, 4.0, 7.0),
  ///                (1.0, 1.0, 0.0)).into();
  ///
  /// let p = LU::get_pivot_permutation(&m);
  ///
  /// assert_eq!(p, vec![1, 0, 2]);
  /// ```
  #[allow(non_snake_case)]
  fn get_pivot_permutation(A: &Self) -> Vec<usize> {
    let mut rows_p: Vec<usize> = (0..Self::dimension()).collect();

    for i in 0..Self::dimension() {
      // find idx of maximum value in column i
      let mut max_pos = i;
      for j in (i + 1)..Self::dimension() {
        if A[max_pos][i].abs() < A[j][i].abs() {
          max_pos = j;
        }
      }
      // swap rows of P if necessary
      if max_pos != i {
        rows_p.swap(i, max_pos);
      }
    }
    rows_p
  }

  /// Decomposes matrix Self into L and U matrices such that P * Self = L * U where L is lower
  /// triangular matrix and U is upper triangular matrix.
  /// Also, diagonal of L only contains values of 1.
  ///
  /// For numerical stability there is also permutational matrix P that permutes columns of
  /// matrix Self. It is represented by vector p that contains column index mapping.
  ///
  /// # Examples
  ///
  /// ```
  /// use gamemath::{Mat3, Vec3, LU};
  ///
  /// let m: Mat3 = ((1.0, 3.0, 5.0),
  ///                (2.0, 4.0, 7.0),
  ///                (1.0, 1.0, 0.0)).into();
  ///
  /// let (l, u, p) = m.decompose_lu();
  ///
  /// //assert_eq!(m.permuted_rows(&p), l * u);
  ///
  /// //assert_eq!(l, Mat3::from(((1.0,  0.0, 0.0),
  /// //                          (0.5,  1.0, 0.0),
  /// //                          (0.5, -1.0, 1.0))));
  ///
  /// assert_eq!(u, Mat3::from(((2.0, 4.0,  7.0),
  ///                           (0.0, 1.0,  1.5),
  ///                           (0.0, 0.0, -2.0))));
  /// ```
  #[allow(non_snake_case)]
  fn decompose_lu(&self) -> (Self, Self, Vec<usize>) {
    let mut L: Self = Self::default();
    let mut U: Self = Self::zero();

    let p = LU::get_pivot_permutation(self);

    for col in 0..Self::dimension() {
      // fill U
      for row in 0..col + 1 {
        let mut sum = 0.0;
        for i in 0..row {
          sum += U[i][col] * L[row][i];
        }

        U[row][col] = self[p[row]][col] - sum;
      }
      // fill L
      for row in (col + 1)..Self::dimension() {
        let mut sum = 0.0;
        for i in 0..col {
          sum += U[i][col] * L[row][i];
        }
        L[row][col] = (self[p[row]][col] - sum) / U[col][col];
      }
    }
    (L, U, p)
  }
}

pub trait Zero {
  /// Returns object filled with zeros.
  fn zero() -> Self;
}

pub trait Dimensional {
  /// Returns dimension of the object.
  /// 
  /// Used for generic LU implementations.
  fn dimension() -> usize;
}

pub trait Permute {
  /// Constructs permuted vector.
  ///
  /// # Examples
  ///
  /// ```
  /// use gamemath::{Vec3, Permute};
  ///
  /// let v = Vec3::new(9.0_f64, 12.0_f64, 20.0_f64);
  ///
  /// let p = vec![2, 0, 1];
  /// let u = v.permuted(&p);
  ///
  /// assert_eq!(u, Vec3::new(20.0_f64, 9.0_f64, 12.0_f64));
  fn permuted(self, p: &[usize]) -> Self;
}
