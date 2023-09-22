

use super::scalar::Scalar;
use super::scalar;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T : Scalar, const M: usize, const N: usize>
{
	pub arr: [[T; N]; M],
	number_rows: usize,
	number_cols: usize,
}

impl<T : Scalar, const M: usize, const N: usize> From<[[T; N]; M]> for Matrix<T, M, N> {
	fn from(arr: [[T; N]; M]) -> Self {
		if M == 0 || N == 0 {
			panic!("invalid matrix dimension");
		}
		Self {arr, number_rows: M, number_cols: N}
	}
}

impl<T : Scalar, const M: usize, const N: usize> PartialEq for Matrix<T, M, N> {
	fn eq(&self, other: &Self) -> bool {
		for i in 0..M {
			for j in 0..N {
				if (self.arr[i][j] - other.arr[i][j]).norm() > scalar::MATRIX_EPSILON {
					return false;
				}
			}
		}
		true
	}
}

impl<T : Scalar, const M: usize, const N: usize> Default for Matrix<T, M, N> {
	fn default() -> Self {
		Matrix::from([[T::zero(); N]; M])
	}
}

impl<T : Scalar, const M: usize, const N: usize> Matrix<T, M, N> {
	pub fn new(number: T) -> Self {
		Matrix::from([[number; N]; M])
	}

	pub fn shape(&self) -> [usize; 2] {
		[self.number_rows, self.number_cols]
	}

	pub fn row(&self) -> usize {
		self.number_rows
	}

	pub fn col(&self) -> usize {
		self.number_cols
	}

	pub fn is_square(&self) -> bool {
		self.number_rows == self.number_cols
	}

	pub fn print(&self) {
		for i in 0..M {
			for j in 0..N {
				if j == 0 {
					print!("[");
				}
				print!(" {}", self.arr[i][j]);
				if j != self.number_cols - 1 {
					print!(",");
				}
				else {
					print!(" ]");
					if i != M - 1 {
						println!();
					}
				}
			}
		}
	}
}

// Accessors
impl<T : Scalar, const M: usize, const N: usize> Index<usize> for Matrix<T, M, N> {
	type Output = [T; N];
	fn index(&self, index: usize) -> &Self::Output {
		&self.arr[index]
	}
}

impl<T : Scalar, const M: usize, const N: usize> IndexMut<usize> for Matrix<T, M, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.arr[index]
	}
}



impl<T : Scalar, const M: usize> Matrix<T, M, M> {
	pub fn identity() -> Self {
		let mut id = Self::default();
		for i in 0..M {
			id.arr[i][i] = T::one();
		}
		id
	}
}

impl<T : Scalar, const M: usize, const N: usize> fmt::Display for Matrix<T, M, N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let mut mat_string = String::new();
	for i in 0..M {
		for j in 0..N {
			if j == 0 {
				mat_string.push_str("[ ");
			}
			//mat_string.push_str(" ");
			mat_string.push_str(&self.arr[i][j].to_string());
			if j != N - 1 {
				mat_string.push_str(", ");
			}
			else {
				mat_string.push_str(" ]");
			}
		}
		mat_string.push('\n');
	}
	write!(f, "{}", mat_string)
}
}


#[cfg(test)]
mod tests {
	use super::Matrix;

	#[test]
	fn array_init_test00() {
		let arr = [[0.,0.], [0.,0.]];
		let mat = Matrix::from(arr);
		assert_eq!(mat.arr, arr);
		assert_eq!(mat.number_rows, 2);
		assert_eq!(mat.number_cols, 2);
		assert_eq!(mat.is_square(), true);
	}
	#[test]
	fn array_init_test01() {
		let arr = [[0.,1.]];
		let mat = Matrix::from(arr);
		assert_eq!(mat.arr, arr);
		assert_eq!(mat.number_rows, 1);
		assert_eq!(mat.number_cols, 2);
		assert_eq!(mat.is_square(), false);
	}
	#[test]
	fn array_init_test02() {
		let arr = [[0.], [1.]];
		let mat = Matrix::from(arr);
		assert_eq!(mat.arr, arr);
		assert_eq!(mat.number_rows, 2);
		assert_eq!(mat.number_cols, 1);
	}
	#[test]
	#[should_panic]
	fn array_init_test03() {
		let arr : [[f32; 0];1] = [[]];
		let _mat = Matrix::from(arr);
	}
	#[test]
	#[should_panic]
	fn array_init_test04() {
		let arr : [[f32; 1];0] = [];
		let _mat = Matrix::from(arr);
	}

	#[test]
	fn default_test00() {
		let mat = Matrix::<f32,2,3>::default();
		assert_eq!(mat.arr, [[0.0;3];2]);
		assert_eq!(mat.row(), 2);
		assert_eq!(mat.col(), 3);
	}
	#[test]
	fn new_test00() {
		let mat = Matrix::<f32,3,2>::new(1.0);
		assert_eq!(mat.arr, [[1.0; 2];3]);
		assert_eq!(mat.shape(), [3, 2]);
	}
}