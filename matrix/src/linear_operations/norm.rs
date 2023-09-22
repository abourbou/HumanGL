
use crate::base::{scalar::Scalar, vector::Vector};

impl <T : Scalar, const M : usize> Vector<T,M> {
	pub fn norm_1(&self) -> f32 {
		let mut norm = T::zero().norm();
		for i in 0..M {
			norm += self.arr[i].norm();
		}
		norm
	}

	pub fn norm(&self) -> f32 {
		let mut norm = T::zero().norm();
		for i in 0..M {
			norm += self.arr[i].norm() * self.arr[i].norm();
		}
		norm.sqrt()
	}

	pub fn norm_inf(&self) -> f32 {
		let mut norm = T::zero().norm();
		for i in 0..M {
			let buffer = self.arr[i].norm();
			if buffer > norm {
				norm = buffer;
			}
		}
		norm
	}
}

impl <T : Scalar, const M : usize> Vector<T,M> {
	pub fn normalize(&mut self) {
		*self *= (1. / self.norm()).into();
	}
	pub fn normalized(&self) -> Self {
		*self * (1. / self.norm()).into()
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Vector::from([0., 0., 0.]);
		assert_eq!(u.norm_1(), 0.);
		assert_eq!(u.norm(), 0.);
		assert_eq!(u.norm_inf(), 0.);
	}
	#[test]
	fn test01() {
		let u = Vector::from([1., 2., 3.]);
		assert_eq!(u.norm_1(), 6.);
		assert_eq!(u.norm(), f32::sqrt(14.));
		assert_eq!(u.norm_inf(), 3.);
	}
	#[test]
	fn test02() {
		let u = Vector::from([-1., -2.]);
		assert_eq!(u.norm_1(), 3.0);
		assert_eq!(u.norm(), f32::sqrt(5.0));
		assert_eq!(u.norm_inf(), 2.);
	}
