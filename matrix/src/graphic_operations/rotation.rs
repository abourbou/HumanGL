
use crate::*;

// Angle in degree
pub fn rx(angle : f32) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	let cos_angle = angle.to_radians().cos();
	let sin_angle = angle.to_radians().sin();

	result.arr[1][1] = cos_angle;
	result.arr[2][2] = cos_angle;
	result.arr[1][2] = -1. * sin_angle;
	result.arr[2][1] = sin_angle;

	result
}

// Angle in degree
pub fn ry(angle : f32) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	let cos_angle = angle.to_radians().cos();
	let sin_angle = angle.to_radians().sin();

	result.arr[0][0] = cos_angle;
	result.arr[2][2] = cos_angle;
	result.arr[2][0] = -1. * sin_angle;
	result.arr[0][2] = sin_angle;

	result
}

// Angle in degree
pub fn rz(angle : f32) -> Matrix4f
{
	let mut result = Matrix4f::identity();

	let cos_angle = angle.to_radians().cos();
	let sin_angle = angle.to_radians().sin();

	result.arr[0][0] = cos_angle;
	result.arr[1][1] = cos_angle;
	result.arr[0][1] = -1. * sin_angle;
	result.arr[1][0] = sin_angle;

    result
}

// Create rotation matrix from a quaternion
// ! The quaternion need to be normalized
pub fn quat_to_rotation(quaternion : Vector4f) -> Matrix4f {
	let mut rotation_matrix = Matrix4f::identity();
    let w = quaternion[0];
    let x = quaternion[1];
    let y = quaternion[2];
    let z = quaternion[3];

    rotation_matrix[0][0] = 1.0 - 2.0 * y * y - 2.0 * z * z;
    rotation_matrix[0][1] = 2.0 * x * y - 2.0 * w * z;
    rotation_matrix[0][2] = 2.0 * x * z + 2.0 * w * y;

    rotation_matrix[1][0] = 2.0 * x * y + 2.0 * w * z;
    rotation_matrix[1][1] = 1.0 - 2.0 * x * x - 2.0 * z * z;
    rotation_matrix[1][2] = 2.0 * y * z - 2.0 * w * x;

    rotation_matrix[2][0] = 2.0 * x * z - 2.0 * w * y;
    rotation_matrix[2][1] = 2.0 * y * z + 2.0 * w * x;
    rotation_matrix[2][2] = 1.0 - 2.0 * x * x - 2.0 * y * y;

	rotation_matrix
}

pub fn axis_angle_to_quaternion(axis: Vector3f, angle: f32) -> Vector4f {
    let half_angle = angle / 2.0;
    let sin_half_angle = half_angle.sin();

    let w = half_angle.cos();
    let x = axis[0] * sin_half_angle;
    let y = axis[1] * sin_half_angle;
    let z = axis[2] * sin_half_angle;

    [w, x, y, z].into()
}

//   Convert euler angles : roll, pitch, yall to quaternion
// ! Angles in degrees
pub fn euler_angle_to_quaternion(rpy_vec : Vector3f) -> Vector4f {
    let roll  = rpy_vec[0].to_radians() / 2.;
    let pitch = rpy_vec[1].to_radians() / 2.;
    let yaw   = rpy_vec[2].to_radians() / 2.;

    let cy = yaw.cos();
    let sy = yaw.sin();
    let cp = pitch.cos();
    let sp = pitch.sin();
    let cr = roll.cos();
    let sr = roll.sin();

    let w = cr * cp * cy + sr * sp * sy;
    let x = sr * cp * cy - cr * sp * sy;
    let y = cr * sp * cy + sr * cp * sy;
    let z = cr * cp * sy - sr * sp * cy;

    [w, x, y, z].into()
}