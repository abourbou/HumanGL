
use crate::*;
use base::scalar::Scalar;
use linear_operations::{cross_product::cross_product};

pub fn view_matrix<T: Scalar>(eye : Vector3<T>, center : Vector3<T>, head_up : Vector3<T>) -> Matrix4<T> {
    let forward = (eye - center).normalized();
    let right_axis = cross_product(&head_up, &forward);
    let up_axis = cross_product(&forward, &right_axis);
    let mut result = Matrix4::<T>::identity();

	result.arr[0][0] = right_axis.arr[0];
	result.arr[0][1] = right_axis.arr[1];
	result.arr[0][2] = right_axis.arr[2];
	result.arr[1][0] = up_axis.arr[0];
	result.arr[1][1] = up_axis.arr[1];
	result.arr[1][2] = up_axis.arr[2];
	result.arr[2][0] = forward.arr[0];
	result.arr[2][1] = forward.arr[1];
	result.arr[2][2] = forward.arr[2];
	result.arr[0][3] = T::zero() - T::one() * right_axis.dot_product(&eye);
	result.arr[1][3] = T::zero() - T::one() * up_axis.dot_product(&eye);
	result.arr[2][3] = T::zero() - T::one() * forward.dot_product(&eye);

    result
}
