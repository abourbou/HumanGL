

use super::scalar::Scalar;
use super::scalar;
use super::matrix;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone)]
pub struct Vector<T : Scalar, const M: usize>
{
	pub arr: [T; M],
	length: usize,
}

impl<T : Scalar, const M: usize> From<[T; M]> for Vector<T, M> {
	fn from(arr: [T; M]) -> Self {
		if M == 0 {
			panic!("invalid Vector dimension");
		}
		Self {arr, length: M}
	}
}

impl<T : Scalar, const M: usize> PartialEq for Vector<T, M> {
	fn eq(&self, other: &Self) -> bool {
		for i in 0..M {
			if (self.arr[i] - other.arr[i]).norm() > scalar::MATRIX_EPSILON {
				return false;
			}
		}
		true
	}
}

impl<T : Scalar, const M: usize> Default for Vector<T, M> {
	fn default() -> Self {
		Vector::from([T::zero(); M])
	}
}

impl<T : Scalar, const M: usize> Vector<T, M> {
	pub fn new(number: T) -> Self {
		Vector::from([number; M])
	}

	pub fn size(&self) -> usize {
		self.length
	}

	pub fn print(&self) {
		for i in 0..M {
			println!("[ {} ]", self.arr[i]);
		}
		println!();
	}
	pub fn matrix(&self) -> matrix::Matrix<T, 1, M> {
		matrix::Matrix::from([self.arr])
	}
}

// Accessors
impl<T : Scalar, const M: usize> Index<usize> for Vector<T, M> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		&self.arr[index]
	}
}

impl<T : Scalar, const M: usize> IndexMut<usize> for Vector<T, M> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.arr[index]
	}
}

impl<T : Scalar, const M: usize> fmt::Display for Vector<T, M> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut mat_string = String::new();
		for i in 0..M {
			mat_string.push_str("[ ");
			mat_string.push_str(&self.arr[i].to_string());
			mat_string.push_str(" ]");
			mat_string.push('\n');
		}
		write!(f, "{}", mat_string)
	}
}


#[cfg(test)]
mod tests {
	use super::Vector;

	#[test]
	fn array_init_test00() {
		let arr = [0.,0.];
		let vec = Vector::from(arr);
		assert_eq!(vec.arr, arr);
		assert_eq!(vec.size(), 2);
	}
	#[test]
	fn array_init_test01() {
		let arr = [0.,1.,2.];
		let vec = Vector::from(arr);
		assert_eq!(vec.arr, arr);
		assert_eq!(vec.length, 3);
	}
	#[test]
	#[should_panic]
	fn array_init_test03() {
		let arr : [f32; 0] = [];
		let _mat = Vector::from(arr);
	}

	#[test]
	fn default_test00() {
		let vec = Vector::<f32,3>::default();
		assert_eq!(vec.arr, [0.0;3]);
		assert_eq!(vec.size(), 3);
	}
	#[test]
	fn new_test00() {
		let vec = Vector::<f32,2>::new(1.0);
		assert_eq!(vec.arr, [1.0; 2]);
		assert_eq!(vec.size(), 2);
	}
}