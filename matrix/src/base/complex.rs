

use num_complex::Complex;
use super::scalar::Scalar;

impl Scalar for Complex<f32> {
	fn norm(self) ->f32 {
		self.norm()
	}
}

#[cfg(test)]
	use super::vector::Vector;
	#[test]
	fn test00() {
		let vec = Vector::<Complex<f32>,3>::default();
		assert_eq!(vec.norm(), 0.);
	}
	#[test]
	fn test01() {
		let vec = Vector::<Complex<f32>,3>::new(Complex::<f32>::new(1., -2.));
		assert_eq!(vec.norm(), 15_f32.sqrt());
	}
