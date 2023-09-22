
use crate::base::{scalar::Scalar, matrix::Matrix};

// Compute determinant by reducing the matrix to an triangle matrix
// determinant of a triangle matrix is the product of his diagonal
// Swapping 2 lines change the sign of the determinant
impl <T: Scalar, const M : usize> Matrix<T,M,M> {
	pub fn determinant(&self) -> T {
		if M == 1 {
			self.arr[0][0]
		}
		else if M == 2 {
			self.arr[0][0] * self.arr[1][1] - self.arr[0][1] * self.arr[1][0]
		}
		else {
			let mut r : usize = 0;
			let mut det = T::one();
			let mut buffer_matrix = *self;
			for j in 0..M {
				if r == M {
					break;
				}
				//find pivot
				let mut some_pivot : Option<usize> = None;
				for i in r..M {
					if buffer_matrix.arr[i][j] != T::zero() {
						some_pivot = Some(i);
						break;
					}
				}
				if some_pivot.is_none() {
					continue;
				}
				//operations on pivot
				let pivot = some_pivot.unwrap();
				if pivot != r {
					det = T::zero() - det;
					buffer_matrix.swap_line(pivot, r);
				}
				//reduce other lines
				for i in r + 1..M {
					if buffer_matrix.arr[i][j] != T::zero() {
						buffer_matrix.sub_line(i, r, buffer_matrix.arr[i][j] / buffer_matrix.arr[r][j]);
					}
				}
				r += 1;
			}
			for i in 0..M {
				det *= buffer_matrix.arr[i][i];
			}
			det
		}
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Matrix::from([
			[ 1., -1.],
			[-1., 1.],
		]);
		assert_float_relative_eq!(u.determinant(), 0.);
	}
	#[test]
	fn test01() {
		let u = Matrix::from([
			[2., 0., 0.],
			[0., 2., 0.],
			[0., 0., 2.],
			]);
		assert_float_relative_eq!(u.determinant(), 8., 1e-6);
	}
	#[test]
	fn test02() {
		let u = Matrix::from([
			[8., 5., -2.],
			[4., 7., 20.],
			[7., 6., 1.],
		]);
		assert_float_relative_eq!(u.determinant(), -174., 1e-6);
	}
	#[test]
	fn test03() {
		let u = Matrix::from([
			[ 8., 5., -2., 4.],
			[ 4., 2.5, 20., 4.],
			[ 8., 5., 1., 4.],
			[28., -4., 17., 1.],
		]);
		assert_float_relative_eq!(u.determinant(), 1032., 1e-6);
	}
	#[test]
	fn test04() {
		let u = Matrix::from([
			[15., 4., 8.],
			[-12., -7., 5.],
			[0., -5., 15.],
		]);
		assert_float_absolute_eq!(u.determinant(), 0., 5e-4);
	}
	#[test]
	fn test05() {
		let u = Matrix::from([
			[1.,  2.,  3.,  4.],
			[-2., 1., -4.,  3.],
			[3., -4., -1.,  2.],
			[4.,  3., -2., -1.],
		]);
		assert_float_relative_eq!(u.determinant(), 900., 5e-5);
	}
