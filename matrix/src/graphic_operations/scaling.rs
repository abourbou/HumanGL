
use crate::*;

// Scale an object with x, y, z proportions
pub fn scaling(x_scaling : f32, y_scaling : f32, z_scaling : f32) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	result.arr[0][0] = x_scaling;
	result.arr[1][1] = y_scaling;
	result.arr[2][2] = z_scaling;

	result
}

// Scale an object with x, y, z proportions in a vector
pub fn scaling_v(scal : Vector3f) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	result.arr[0][0] = scal[0];
	result.arr[1][1] = scal[1];
	result.arr[2][2] = scal[2];

	result
}