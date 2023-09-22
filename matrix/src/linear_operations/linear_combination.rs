
use crate::base::{scalar::Scalar, vector::Vector};

pub fn linear_combination<T: Scalar, const M : usize>(u: &[Vector<T,M>], coeffs: &[T]) -> Vector<T,M> {
		if u.is_empty() {
			panic!("empty list of vector");
		}
		else if u.len() != coeffs.len() {
			panic!("list coeff need to be the same size that the list of vector");
		}

		let mut vec_lincb = Vector::<T,M>::default();
		for i in  0..M {
			let mut buffer_value = T::zero();
			for j in 0..u.len() {
				buffer_value += coeffs[j] * u[j].arr[i];
			}
			vec_lincb.arr[i] = buffer_value;
		}
		vec_lincb
	}


#[cfg(test)]
	#[test]
	fn linear_comb_test00() {
		let vec = Vector::from([1.0, 2.0, 3.0]);
		let vec2 = Vector::<f32,3>::new(2.0);
		let u = [vec, vec2];
		let coeffs = [2.0, 1.0];
		assert_eq!(linear_combination::<f32,3>(&u, &coeffs), Vector::from([4.0, 6.0, 8.0]));
	}
	#[test]
	fn linear_comb_test01() {
		let vec = Vector::from([1.0, 2.0]);
		let vec2 = Vector::<f32,2>::new(2.0);
		let u = [vec, vec2];
		let coeffs = [0.0, 3.0];
		assert_eq!(linear_combination::<f32,2>(&u, &coeffs), Vector::from([6.0, 6.0]));
	}
	#[test]
	#[should_panic]
	fn linear_comb_test02() {
		let vec = Vector::from([1.0, 2.0]);
		let vec2 = Vector::<f32,2>::new(2.0);
		let u = [vec, vec2];
		let coeffs = [1.0];
		assert_eq!(linear_combination::<f32,2>(&u, &coeffs), Vector::from([6.0, 6.0]));
	}
	#[test]
	#[should_panic]
	fn linear_comb_test03() {
		let u = [];
		let coeffs = [1.0];
		assert_eq!(linear_combination::<f32,2>(&u, &coeffs), Vector::from([6.0, 6.0]));
	}
