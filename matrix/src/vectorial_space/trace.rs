
use crate::base::{scalar::Scalar, matrix::Matrix};

impl <T : Scalar, const M : usize> Matrix<T, M, M> {
	pub fn trace(&self) -> T {
		let mut val = T::zero();
		for i in 0..M {
			val += self.arr[i][i];
		}
		val
	}
}

#[cfg(test)]
	#[test]
	fn test00() {
		let mat = Matrix::from([
			[1., 0.],
			[0., 1.]
		]);
		assert_eq!(mat.trace(), 2.);
	}
	#[test]
	fn test01() {
		let mat = Matrix::<f32,3,3>::default();
		assert_eq!(mat.trace(), 0.);
	}
	#[test]
	fn test02() {
		let mat = Matrix::<f32,5,5>::new(2.);
		assert_eq!(mat.trace(), 10.);
	}
	#[test]
	fn test03() {
		let mat = Matrix::from([
			[2., -5., 0.],
			[4., 3., 7.],
			[-2., 3., 4.],
			]);
		assert_eq!(mat.trace(), 9.);
	}
	#[test]
	fn test04() {
		let mat = Matrix::from([
			[-2., -8., 4.],
			[1., -23., 4.],
			[0., 6., 4.],
		]);
		assert_eq!(mat.trace(), -21.);
	}
