
use crate::base::{scalar::Scalar, matrix::Matrix};

impl <T: Scalar, const M : usize, const N : usize> Matrix<T,M,N> {
	pub fn transpose(&self) -> Matrix<T,N,M> {
		let mut mat = Matrix::<T,N,M>::default();
		for i in 0..M {
			for j in 0..N {
				mat.arr[j][i] = self.arr[i][j];
			}
		}
		mat
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let mat = Matrix::from([
			[0.,1.,2.],
			[3.,4.,5.],
			[6.,7.,8.]
		]);
		assert_eq!(mat.transpose(),Matrix::from([
			[0., 3., 6.],
			[1., 4., 7.],
			[2., 5., 8.],
		]));
	}
	#[test]
	fn test01() {
		let mat = Matrix::from([
			[0.,1.,2.],
			[3.,4.,5.],
		]);
		assert_eq!(mat.transpose(),Matrix::from([
			[0., 3.],
			[1., 4.],
			[2., 5.],
		]));
	}
	#[test]
	fn test02() {
		let mat = Matrix::from([
			[0.,1.],
			[3.,4.],
			[6.,7.]
		]);
		assert_eq!(mat.transpose(),Matrix::from([
			[0., 3., 6.],
			[1., 4., 7.],
		]));
	}
