
use crate::base::{scalar::Scalar, vector::Vector, matrix::Matrix};
use std::ops::Mul;

impl<T : Scalar, const M: usize, const N: usize> Mul<Vector<T,N>> for Matrix<T, M, N> {
	type Output = Vector<T,M>;
	fn mul(self, vec: Vector<T,N>) -> Vector<T,M> {
		let mut result = Vector::<T,M>::default();
		for i in 0..M {
			for j in 0..N {
				result.arr[i] += self.arr[i][j] * vec.arr[j];
			}
		}
		result
	}
}

impl<T : Scalar, const M: usize, const N: usize, const P: usize> Mul<Matrix<T,N,P>> for Matrix<T, M, N> {
	type Output = Matrix<T,M,P>;
	fn mul(self, mat: Matrix<T,N,P>) -> Matrix<T,M,P> {
		let mut result = Matrix::<T,M,P>::default();
		for i in 0..M {
			for j in 0..P {
				for k in 0..N {
					result.arr[i][j] += self.arr[i][k] * mat.arr[k][j];
				}
			}
		}
		result
	}
}

#[cfg(test)]
		#[test]
		fn test_mult_vec00() {
			let u = Matrix::from([
				[1., 0.],
				[0., 1.],
				]);
			let v = Vector::from([4., 2.]);
			assert_eq!(u * v, Vector::from([4.,2.]));
		}
		#[test]
		fn test_mult_vec01() {
			let u = Matrix::from([
				[2., 0.],
				[0., 2.],
				]);
			let v = Vector::from([4., 2.]);
			assert_eq!(u * v, Vector::from([8.,4.]));
		}
		#[test]
		fn test_mult_vec02() {
			let u = Matrix::from([
				[2., -2.],
				[-2., 2.],
				]);
			let v = Vector::from([4., 2.]);
			assert_eq!(u * v, Vector::from([4.,-4.]));
		}
		#[test]
		fn test_mult_vec03() {
			let u = Matrix::from([
				[2., -2.],
				]);
			let v = Vector::from([4., 2.]);
			assert_eq!(u * v, Vector::from([4.]));
		}

		#[test]
		fn test_mult_mat00() {
			let u = Matrix::from([
				[1., 0.],
				[0., 1.],
			]);
			let v = Matrix::from([
				[1., 0.],
				[0., 1.],
			]);
			assert_eq!(u * v, Matrix::from([[1.,0.],[0.,1.]]));
		}
		#[test]
		fn test_mult_mat01() {
			let u = Matrix::from([
				[1., 0.],
				[0., 1.],
			]);
			let v = Matrix::from([
				[2., 1.],
				[4., 2.],
			]);
			assert_eq!(u * v, Matrix::from([[2.,1.],[4.,2.]]));
		}
		#[test]
		fn test_mult_mat02() {
			let u = Matrix::from([
				[3., -5.],
				[6., 8.],
			]);
			let v = Matrix::from([
				[2., 1.],
				[4., 2.],
			]);
			assert_eq!(u * v, Matrix::from([[-14.,-7.],[44.,22.]]));
		}
		#[test]
		fn test_mult_mat03() {
			let u = Matrix::from([
				[1., 1.],
			]);
			let v = Matrix::from([
				[1.],
				[1.],
			]);
			assert_eq!(u * v, Matrix::from([[2.]]));
			assert_eq!(v * u, Matrix::from([[1., 1.],[1.,1.]]));
		}
