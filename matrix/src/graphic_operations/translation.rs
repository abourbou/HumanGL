
use crate::*;

pub fn translation(x : f32, y : f32, z : f32) -> Matrix4f {
    let mut translat = Matrix4f::identity();
    translat.arr[0][3] = x;
    translat.arr[1][3] = y;
    translat.arr[2][3] = z;

    translat
}

pub fn translation_v(pos : &Vector3f) -> Matrix4f {
    let mut translat = Matrix4f::identity();
    translat.arr[0][3] = pos[0];
    translat.arr[1][3] = pos[1];
    translat.arr[2][3] = pos[2];

    translat
}