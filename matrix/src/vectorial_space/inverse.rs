
use crate::base::{scalar::Scalar, matrix::Matrix};

impl <T: Scalar, const M : usize> Matrix<T,M,M> {
	pub fn inverse(&self) -> Result<Self, &'static str> {
		//check determinant
		if self.determinant() == T::zero() {
			return Err("Singular matrix, no inverse");
		}

		//creation identity matrix
		let mut result_matrix = Matrix::identity();

		//row reduce the matrix
		let mut r : usize = 0;
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
				result_matrix.swap_line(pivot, r);
				buffer_matrix.swap_line(pivot, r);
			}
			if buffer_matrix.arr[r][j] != T::one() {
				result_matrix.mult_line(r, T::one() / buffer_matrix.arr[r][j]);
				buffer_matrix.mult_line(r, T::one() / buffer_matrix.arr[r][j]);
			}

			//reduce other lines
			for i in 0..M {
				if i == pivot {
					continue;
				}
				if buffer_matrix.arr[i][j] != T::zero() {
					result_matrix.sub_line(i, r, buffer_matrix.arr[i][j] / buffer_matrix.arr[r][j]);
					buffer_matrix.sub_line(i, r, buffer_matrix.arr[i][j] / buffer_matrix.arr[r][j]);
				}
			}
			r += 1;
		}
		Ok(result_matrix)
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Matrix::from([[0.25]]);
		assert_eq!(u.inverse(), Ok(Matrix::from([[4.]])));
	}
	#[test]
	fn test01() {
		let u = Matrix::from([
			[1., 0., 0.],
			[0., 1., 0.],
			[0., 0., 1.],
		]);
		assert_eq!(u.inverse(), Ok(Matrix::from([
			[1., 0., 0.],
			[0., 1., 0.],
			[0., 0., 1.],
		])));
	}
	#[test]
	fn test02() {
		let u = Matrix::from([
			[2., 0., 0.],
			[0., 2., 0.],
			[0., 0., 2.],
		]);
		assert_eq!(u.inverse(), Ok(Matrix::from([
			[0.5, 0., 0.],
			[0., 0.5, 0.],
			[0., 0., 0.5],
		])));
	}
	#[test]
	fn test03() {
		let u = Matrix::from([
			[8., 5., -2.],
			[4., 7., 20.],
			[7., 6., 1.],
		]);
		assert_eq!(u.inverse(), Ok(Matrix::from([
			[113./174., 17./174., -19./29.],
			[-68./87., -11./87., 28./29.],
			[25./174., 13./174., -6./29.]
		])));
	}
	#[test]
	fn test_invalid00() {
		let u = Matrix::from([
			[ 1., -1.],
			[-1., 1.],
		]);
		assert!(u.inverse().is_err());
	}