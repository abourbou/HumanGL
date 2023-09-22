
use crate::base::{scalar::Scalar, matrix::Matrix};

impl <T: Scalar, const M : usize, const N : usize> Matrix<T,M,N> {
	//swap line : swap two lines of a matrix
	// L1 <-> L2
	pub fn swap_line(&mut self, line1 : usize, line2: usize) {
		if line1 >= M || line2 >= M {
			panic!("line's index of the matrix overflow");
		}
		if line1 == line2 {
			return;
		}
		let mut buffer : T;
		for j in 0..N {
			buffer = self.arr[line2][j];
			self.arr[line2][j] = self.arr[line1][j];
			self.arr[line1][j] = buffer;
		}
	}
	//add_line : add a line multiplied by a coeff to an other line
	//L_changed <- L_changed + coeff * L_added
	pub fn add_line(&mut self, line_changed : usize, line_added : usize, coeff: T) {
		if line_changed >= M || line_added >= M {
			panic!("line's index of the matrix overflow");
		}
		for j in 0..N {
			self.arr[line_changed][j] += coeff * self.arr[line_added][j];
		}
	}
	//sub_line : sub a line multiplied by a coeff to an other line
	//L_changed <- L_changed - coeff * L_substr
	pub fn sub_line(&mut self, line_changed : usize, line_substr : usize, coeff: T) {
		if line_changed >= M || line_substr >= M {
			panic!("line's index of the matrix overflow");
		}
		for j in 0..N {
			self.arr[line_changed][j] -= coeff * self.arr[line_substr][j];
		}
	}
	//mult_line: multiply a line of a matrix by a coeff
	// L_changed <- coeff * L_changed
	pub fn mult_line(&mut self, line_changed : usize, coeff: T) {
		if line_changed >= M {
			panic!("line's index of the matrix overflow");
		}
		for j in 0..N {
			self.arr[line_changed][j] *= coeff;
		}
	}

	pub fn row_echelon(&self) -> Self {
		let mut r : usize = 0;
		let mut buffer_matrix = *self;
		for j in 0..N {
			if r == M {
				break;
			}
			//find pivot
			let mut some_pivot : Option<usize> = None;
			for i in r..M {
				if (buffer_matrix).arr[i][j] != T::zero() {
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
				buffer_matrix.swap_line(pivot, r);
			}
			if buffer_matrix.arr[r][j] != T::one() {
				buffer_matrix.mult_line(r, T::one() / buffer_matrix.arr[r][j]);
			}

			//reduce other lines
			for i in 0..M {
				if i == r {
					continue;
				}
				if buffer_matrix.arr[i][j] != T::zero() {
					buffer_matrix.sub_line(i, r, buffer_matrix.arr[i][j] / buffer_matrix.arr[r][j]);
				}
			}
			r += 1;
		}
		buffer_matrix
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Matrix::from([
			[1., 0., 0.],
			[0., 1., 0.],
			[0., 0., 1.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from([
			[1., 0., 0.],
			[0., 1., 0.],
			[0., 0., 1.],
		]));
	}
	#[test]
	fn test01() {
		let u = Matrix::from([
			[1., 2.],
			[2., 4.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from([
			[1.0, 2.0],
			[0.0, 0.0]
		]));
	}
	#[test]
	fn test02() {
		let u = Matrix::from([
			[8., 5., -2., 4., 28.],
			[4., 2.5, 20., 4., -4.],
			[8., 5., 1., 4., 17.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from([
			[1.0, 0.625, 0.0, 0.0, -73./6.],
			[0.0, 0.0, 1.0, 0.0, -11./3.],
			[0.0, 0.0, 0.0, 1.0, 29.5]
		]));
	}
	#[test]
	fn test03() {
		let u = Matrix::from([
			[1., 1., 2., -1.],
			[2., -1., 2., -4.],
			[4., 1., 4., -2.],
		]);
		assert_eq!(u.row_echelon(), Matrix::from([
			[1.0, 0.0, 0.0, 1.0],
			[0.0, 1.0, 0.0, 2.0],
			[0.0, 0.0, 1.0, -2.0]
		]));
	}
