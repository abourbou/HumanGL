
use crate::base::{scalar::Scalar, matrix::Matrix};

impl <T: Scalar, const M : usize, const N : usize> Matrix<T,M,N> {
	pub fn rank(mut self) -> usize{
		let mut r : usize = 0;
		for j in 0..N {
			if r == M {
				break;
			}
			//find pivot
			let mut some_pivot : Option<usize> = None;
			for i in r..M {
				if self.arr[i][j] != T::zero() {
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
				self.swap_line(pivot, r);
			}
			//reduce other lines
			for i in r + 1..M {
				if self.arr[i][j] != T::zero() {
					self.sub_line(i, r, self.arr[i][j] / self.arr[r][j]);
				}
			}
			r += 1;
		}
		r
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
		assert_eq!(u.rank(), 3);
	}
	#[test]
	fn test01() {
		let u = Matrix::from([
			[ 1., 2., 0., 0.],
			[ 2., 4., 0., 0.],
			[-1., 2., 1., 1.],
		]);
		assert_eq!(u.rank(), 2);
	}
	#[test]
	fn test02() {
		let u = Matrix::from([
			[ 8., 5., -2.],
			[ 4., 7., 20.],
			[ 7., 6., 1.],
			[21., 18., 7.],
		]);
		assert_eq!(u.rank(), 3);
	}
	#[test]
	fn test03() {
		let u = Matrix::from([
			[ 1., 2., 0., 0.],
			[ 2., 4., 0., 0.],
			[-1., -2., 0., 0.],
		]);
		assert_eq!(u.rank(), 1);
	}
