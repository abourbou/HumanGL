
use crate::base::{scalar::Scalar, vector::Vector};

impl <T : Scalar, const M : usize> Vector<T,M> {
	pub fn dot_product(&self, v: &Self) -> T {
		let mut dot_product = T::zero();
		for i in 0..M {
			dot_product += self.arr[i] * v.arr[i];
		}
		dot_product
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Vector::from([0., 0.]);
		let v = Vector::from([0., 0.]);
		assert_eq!(u.dot_product(&v), 0.);
	}
	#[test]
	fn test01() {
		let u = Vector::from([1., 0.]);
		let v = Vector::from([1., 0.]);
		assert_eq!(u.dot_product(&v), 1.);
	}
	#[test]
	fn test02() {
		let u = Vector::from([4., 2.]);
		let v = Vector::from([2., 1.]);
		assert_eq!(u.dot_product(&v), 10.);
	}
	#[test]
	fn test03() {
		let u = Vector::from([-3., 6., 4.25]);
		let v = Vector::from([2., 1.65, 1.]);
		assert_eq!(u.dot_product(&v), 8.15);
	}
