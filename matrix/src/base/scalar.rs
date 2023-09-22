
use num_traits::NumAssign;
use std::fmt::Display;

pub const MATRIX_EPSILON : f32 = 2e-6_f32;

pub trait Scalar : Copy + NumAssign + Display + From<f32> {
	fn norm(self) -> f32;
}

impl Scalar for f32 {
	fn norm(self) -> f32 {
		self.abs()
	}
}