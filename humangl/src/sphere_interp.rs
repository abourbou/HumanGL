use std::ops::{Add, Mul};
use matrix::{Vector};
use matrix::base::scalar::Scalar;
use matrix::linear_operations::linear_interp::lerp;

pub fn slerp(u: Vector<f32, 3>, v: Vector<f32, 3>, t: f32) -> Vector<f32, 3>
{
    let unorm = u.normalized();
    let vnorm = v.normalized();

    let theta = f32::cos(unorm.dot_product(&vnorm));
    let sin_theta = f32::sin(theta);
    let sin_theta_from = f32::sin((1. - t) * theta);
    let sin_theta_to = f32::sin(t * theta);

    let magnitude_lerp = lerp(unorm, vnorm, t);
    let slearp_vector = (sin_theta_from * unorm + sin_theta_to * vnorm) * (1. / sin_theta);

    magnitude_lerp * slearp_vector


}