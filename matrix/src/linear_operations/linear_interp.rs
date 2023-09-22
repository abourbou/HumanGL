
use std::ops::{Add, Mul};

pub fn lerp<T:>(u: T, v: T, t: f32) -> T
where T :  Mul<f32, Output=T> + Add<T, Output=T>
{
	let opposite = 1. - t;
	u * opposite + v * t
}

#[cfg(test)]
	use crate::base::{vector::Vector, matrix::Matrix};
	#[test]
	fn f32_test() {
		assert_eq!(lerp(1., 2., 0.), 1.);
		assert_eq!(lerp(1., 2., 1.), 2.);
		assert_eq!(lerp(1., 2., 0.5), 1.5);
		assert_eq!(lerp(21., 42., 0.3), 27.3);
	}
	#[test]
	fn vector_test() {
		let vec = Vector::from([1., 2., 3.]);
		let vec2 = Vector::from([3., 6., 9.]);
		assert_eq!(lerp(vec, vec2, 0.0), vec);
		assert_eq!(lerp(vec, vec2, 0.5), Vector::from([2., 4., 6.]));
		assert_eq!(lerp(vec, vec2, 1.0), vec2);
	}
	#[test]
	fn matrix_test() {
		let mat = Matrix::from([[1., 2.], [0., 1.]]);
		let mat2 = Matrix::default();
		assert_eq!(lerp(mat, mat2, 0.0), mat);
		assert_eq!(lerp(mat, mat2, 0.64), Matrix::from([[0.36, 0.72], [0., 0.36]]));
		assert_eq!(lerp(mat, mat2, 1.0), mat2);
	}