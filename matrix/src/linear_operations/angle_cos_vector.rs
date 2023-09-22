
use crate::base::{scalar::Scalar, vector::Vector};
use std::ops::Div;

/// Calcule the cosinus of the angles between two vectors
pub fn angle_cos<T: Scalar, const M : usize>(u: &Vector<T,M>, v: &Vector<T,M>) -> T
	where T : Div<f32, Output = T> {
	u.dot_product(v) / (u.norm() * v.norm())
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Vector::from([1., 0.]);
		let v = Vector::from([0.5, 0.5]);
		assert_eq!(angle_cos::<f32, 2>(&u, &v), 2.0_f32.sqrt() / 2.0);
	}
	#[test]
	fn test01() {
		let u = Vector::from([1., 0.]);
		let v = Vector::from([0., 1.]);
		assert_eq!(angle_cos::<f32, 2>(&u, &v), 0.);
	}
	#[test]
	fn test02() {
		let u = Vector::from([4., 2.]);
		let v = Vector::from([2., 1.]);
		assert_eq!(angle_cos::<f32, 2>(&u, &v), 1.);
	}
	#[test]
	fn test03() {
		let u = Vector::from([1., 2., 3.]);
		let v = Vector::from([4., 5., 6.]);
			assert_eq!(angle_cos::<f32, 3>(&u, &v), 0.9746318);
	}
