use matrix::{Vector, Matrix};
use matrix::linear_operations::linear_interp::lerp;
use matrix::graphic_operations::translation::translation;
use matrix::graphic_operations::rotation::{rx, ry, rz};

type TVector3<T> = Vector<T, 3>;
type TMatrix4<T> = Matrix<T, 4, 4>;

#[derive(Clone, Debug)]
pub struct Keyframe {
    pub time: u32, //ms
    pub rot: TVector3<f32>,
    pub trans: TVector3<f32>,
}

pub fn get_rotation(angles: &TVector3<f32>) -> TMatrix4<f32> {
    let a = angles.arr;
    let mat_x = rx(a[0]);
    let mat_y = ry(a[1]);
    let mat_z = rz(a[2]);
    mat_x * mat_y * mat_z
}

pub fn get_translation(trans: &TVector3<f32>) -> TMatrix4<f32> {
    let t = trans.arr;
    translation(t[0], t[1], t[2])
}

pub fn animate(keyframes: &Vec<Keyframe>, time: u32) -> TMatrix4<f32> {
    if keyframes.len() == 1 {
        let rot_mat = get_rotation(&(keyframes[0].rot));
        let tran_mat = get_translation(&(keyframes[0].trans));
        return tran_mat * rot_mat
    }
    let start_time: u32 = keyframes[0].time;
    let end_time = keyframes.last().unwrap().time;
    let now = time % (end_time - start_time);
    let mut low = 0;
    let mut high = 0;
    for i in 1..keyframes.len() {
        if now < keyframes[i].time {
            high = i;
            low = i - 1;
            break;
        }
    }
    let t = (now - keyframes[low].time) as f32 / (keyframes[high].time - keyframes[low].time) as f32;
    //slerp
    let rot_vec = lerp(keyframes[low].rot, keyframes[high].rot, t);
    let trans_vec = lerp(keyframes[low].trans, keyframes[high].trans, t);
    let rot_mat = get_rotation(&rot_vec);
    let trans_mat = get_translation(&trans_vec);
    rot_mat * trans_mat
}
