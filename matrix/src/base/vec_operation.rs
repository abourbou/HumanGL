
use crate::base::{scalar::Scalar, vector::Vector};
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

//impl overload operator
impl<T : Scalar, const M: usize> Add<Vector<T,M>> for Vector<T, M> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		let mut result = Self::default();
		for i in 0..M {
			result.arr[i] = self.arr[i] + rhs.arr[i];
		}
		result
	}
}

impl<T : Scalar, const M: usize> Sub<Vector<T,M>> for Vector<T, M> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		let mut result = Self::default();
		for i in 0..M {
			result.arr[i] = self.arr[i] - rhs.arr[i];
		}
		result
	}
}

impl<T : Scalar, const M: usize> Mul<T> for Vector<T, M> {
	type Output = Self;
	fn mul(self, k: T) -> Self {
		let mut result = Self::default();
		for i in 0..M {
			result.arr[i] = k * self.arr[i];
		}
		result
	}
}

// implementation for f32
impl<const M: usize> Mul<Vector<f32, M>> for f32 {
	type Output = Vector<f32,M>;
	fn mul(self, mat : Vector<f32,M>) -> Vector<f32,M> {
		let mut result = Vector::<f32,M>::default();
		for i in 0..M {
			result.arr[i] = self * mat.arr[i];
		}
		result
	}
}

// Overload +=, -= and *=
impl<T : Scalar, const M: usize> AddAssign<Vector<T,M>> for Vector<T, M> {
	fn add_assign(&mut self, rhs: Self){
		for i in 0..M {
			self.arr[i] += rhs.arr[i];
		};
	}
}
impl<T : Scalar, const M: usize> SubAssign<Vector<T,M>> for Vector<T, M> {
	fn sub_assign(&mut self, rhs: Self){
		for i in 0..M {
			self.arr[i] -= rhs.arr[i];
		};
	}
}
impl<T : Scalar, const M: usize> MulAssign<T> for Vector<T, M> {
	fn mul_assign(&mut self, k: T){
		for i in 0..M {
			self.arr[i] *= k;
		};
	}
}

#[cfg(test)]
	#[test]
	fn add_test() {
		let mut mat = Vector::<f32,3>::new(1.0);
		let mat2 = Vector::<f32,3>::new(2.0);
		assert_eq!(mat + mat2, Vector::<f32,3>::new(3.0));
		mat += mat2;
		assert_eq!(mat, Vector::<f32,3>::new(3.0));
	}
	#[test]
	fn sub_test() {
		let mut mat = Vector::<f32,2>::new(1.0);
		let mat2 = Vector::<f32,2>::new(2.0);
		assert_eq!(mat - mat2, Vector::<f32,2>::new(-1.0));
		mat -= mat2;
		assert_eq!(mat, Vector::<f32,2>::new(-1.0));
	}
	#[test]
	fn mul_test() {
		let mut mat = Vector::<f32,3>::new(1.0);
		assert_eq!(mat * 2.0, Vector::<f32,3>::new(2.0));
		assert_eq!(-1.0 * mat, Vector::<f32,3>::new(-1.0));
		mat *= 3.1415;
		assert_eq!(mat, Vector::<f32,3>::new(3.1415));
	}
