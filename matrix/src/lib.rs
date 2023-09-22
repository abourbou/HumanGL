
#[macro_use]
extern crate assert_float_eq;

pub mod base;
pub use base::{matrix::Matrix, vector::Vector};
pub mod graphic_operations;
pub mod linear_operations;
pub mod vectorial_space;

// Alias
pub type Matrix2<T> = Matrix<T, 2, 2>;
pub type Matrix3<T> = Matrix<T, 3, 3>;
pub type Matrix4<T> = Matrix<T, 4, 4>;
pub type Matrix2f = Matrix<f32, 2, 2>; 
pub type Matrix3f = Matrix<f32, 3, 3>; 
pub type Matrix4f = Matrix<f32, 4, 4>; 

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;
pub type Vector2f = Vector<f32, 2>;
pub type Vector3f = Vector<f32, 3>;
pub type Vector4f = Vector<f32, 4>;

// !Later
// TODO Add matrix operation from scop
// TODO Add accessor for the lib to the array and put it not pub ?
// TODO Try Deref/DerefMut for Vector instead of reimplementing everything
// TODO Use MaybeUninit<T> to init array
// TODO Use iterator MUCH more