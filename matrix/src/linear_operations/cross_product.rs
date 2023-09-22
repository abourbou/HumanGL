
use crate::base::{scalar::Scalar, vector::Vector};

pub fn cross_product<T : Scalar>(u: &Vector<T,3>, v: &Vector<T,3>) -> Vector<T,3> {
	Vector::from([	u.arr[1] * v.arr[2] - u.arr[2] * v.arr[1],
					u.arr[2] * v.arr[0] - u.arr[0] * v.arr[2],
					u.arr[0] * v.arr[1] - u.arr[1] * v.arr[0]])
}

#[cfg(test)]
	#[test]
	fn test00() {
		let u = Vector::from([0., 0., 1.]);
		let v = Vector::from([1., 0., 0.]);
		assert_eq!(cross_product::<f32>(&u, &v), Vector::from([0., 1., 0.]));
	}
	#[test]
	fn test01() {
		let u = Vector::from([1., 2., 3.]);
		let v = Vector::from([4., 5., 6.]);
		assert_eq!(cross_product::<f32>(&u, &v), Vector::from([-3., 6., -3.]));
	}
	#[test]
	fn test02() {
		let u = Vector::from([4., 2., -3.]);
		let v = Vector::from([-2., -5., 16.]);
		assert_eq!(cross_product::<f32>(&u, &v), Vector::from([17., -58., -16.]));
	}
