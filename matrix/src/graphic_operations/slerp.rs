
use crate::*;

// Slerp between 2 quaternions
pub fn slerp(q1 : Vector4f, q2 : Vector4f, t : f32) -> Vector4f {
	let dot = q1.dot_product(&q2);
	let abs_dot = dot.abs();
	let (mut s1, s2, theta, sin_theta);

	if 1. - abs_dot <= 1e-6 {
		s1 = 1. - t;
		s2 = t;
	}
	else {
		theta = abs_dot.acos();
		sin_theta = theta.sin();
		s1 = ((1. - t) * theta).sin() / sin_theta;
		s2 = (t * theta).sin() / sin_theta;
	}

	if dot < -1e-6 {
		s1 *= -1.;
	}

	s1 * q1 + s2 * q2
}
