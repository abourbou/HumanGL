
use super::{scalar::Scalar, matrix::Matrix};
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

//impl overload operator
impl<T : Scalar, const M: usize, const N: usize> Add<Matrix<T,M,N>> for Matrix<T, M, N> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		let mut result = Self::default();
		for i in 0..M {
			for j in 0..N {
				result.arr[i][j] = self.arr[i][j] + rhs.arr[i][j];
			}
		}
		result
	}
}

impl<T : Scalar, const M: usize, const N: usize> Sub<Matrix<T,M,N>> for Matrix<T, M, N> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		let mut result = Self::default();
		for i in 0..M {
			for j in 0..N {
				result.arr[i][j] = self.arr[i][j] - rhs.arr[i][j];
			}
		}
		result
	}
}

impl<T : Scalar, const M: usize, const N: usize> Mul<T> for Matrix<T, M, N> {
	type Output = Self;
	fn mul(self, k: T) -> Self {
		let mut result = Self::default();
		for i in 0..M {
			for j in 0..N {
				result.arr[i][j] = k * self.arr[i][j];
			}
		}
		result
	}
}

// implementation for f32
impl<const M: usize, const N: usize> Mul<Matrix<f32, M, N>> for f32 {
	type Output = Matrix<f32,M,N>;
	fn mul(self, mat : Matrix<f32,M,N>) -> Matrix<f32,M,N> {
		let mut result = Matrix::<f32,M,N>::default();
		for i in 0..M {
			for j in 0..N {
				result.arr[i][j] = self * mat.arr[i][j];
			}
		}
		result
	}
}

// Overload +=, -= and *=
impl<T : Scalar, const M: usize, const N: usize> AddAssign<Matrix<T,M,N>> for Matrix<T, M, N> {
	fn add_assign(&mut self, rhs: Self){
		for i in 0..M {
			for j in 0..N {
				self.arr[i][j] += rhs.arr[i][j];
			}
		};
	}
}
impl<T : Scalar, const M: usize, const N: usize> SubAssign<Matrix<T,M,N>> for Matrix<T, M, N> {
	fn sub_assign(&mut self, rhs: Self){
		for i in 0..M {
			for j in 0..N {
				self.arr[i][j] -= rhs.arr[i][j];
			}
		};
	}
}
impl<T : Scalar, const M: usize, const N: usize> MulAssign<T> for Matrix<T, M, N> {
	fn mul_assign(&mut self, k: T){
		for i in 0..M {
			for j in 0..N {
				self.arr[i][j] *= k;
			}
		};
	}
}

#[cfg(test)]
	#[test]
	fn add_test() {
		let mut mat = Matrix::<f32,3,2>::new(1.0);
		let mat2 = Matrix::<f32,3,2>::new(2.0);
		assert_eq!(mat + mat2, Matrix::<f32,3,2>::new(3.0));
		mat += mat2;
		assert_eq!(mat, Matrix::<f32,3,2>::new(3.0));
	}
	#[test]
	fn sub_test() {
		let mut mat = Matrix::<f32,3,2>::new(1.0);
		let mat2 = Matrix::<f32,3,2>::new(2.0);
		assert_eq!(mat - mat2, Matrix::<f32,3,2>::new(-1.0));
		mat -= mat2;
		assert_eq!(mat, Matrix::<f32,3,2>::new(-1.0));
	}
	#[test]
	fn mul_test() {
		let mut mat = Matrix::<f32,3,2>::new(1.0);
		assert_eq!(mat * 2.0, Matrix::<f32,3,2>::new(2.0));
		assert_eq!(-1.0 * mat, Matrix::<f32,3,2>::new(-1.0));
		mat *= 3.1415;
		assert_eq!(mat, Matrix::<f32,3,2>::new(3.1415));
	}
