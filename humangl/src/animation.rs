use matrix::{Vector, Matrix};
use matrix::linear_operations::linear_interp::lerp;
use matrix::graphic_operations::*;
use super::node::Node;
use matrix::*;

#[derive(Clone, Debug)]
pub struct Keyframe {
    pub time: u32, //ms
    pub rot: Vector3f,
    pub trans: Vector3f,
}

// Transform coeffs x, y, z to a rotation matrix rx * ry * rz
pub fn rpy_to_rotation(angles: &Vector3f) -> Matrix4f {
    let a = &angles.arr;
    let mat_x = rotation::rx(a[0]);
    let mat_y = rotation::ry(a[1]);
    let mat_z = rotation::rz(a[2]);
    mat_x * mat_y * mat_z
}

// Transform coeffs x, y, z to a translation of (x, y, z)
pub fn vector_to_translation(trans: &Vector3f) -> Matrix4f {
    let t = &trans.arr;
    translation(t[0], t[1], t[2])
}

// Create isometry matrix corresponding to a rotation of quat at the position center_rot
pub fn center_then_rotate(center_rot : matrix::Vector3f, quat: matrix::Vector4f) -> matrix::Matrix4f {
    let rotation_matrix = quat_to_rotation(quat);
    
    translation_v(center_rot) * rotation_matrix * translation_v(-1. * center_rot)
}

pub fn animate(node: &Node, current_time: u32) -> Matrix4f {

    let keyframes = &node.keyframes;
    let time_ratio = 1;

    if keyframes.len() == 1 {
        let kf = &keyframes[0];
        let rot_mat = center_then_rotate(node.center_rot, euler_angle_to_quaternion(kf.rot));
        let trans_mat = vector_to_translation(&kf.trans);
        return trans_mat * rot_mat;
    }
    let start_time: u32 = keyframes[0].time;
    let end_time = keyframes.last().unwrap().time;
    let now = (current_time / time_ratio) % (end_time - start_time);
    let mut low = 0;
    let mut high = 0;
    for i in 1..keyframes.len() {
        if now < keyframes[i].time {
            high = i;
            low = i - 1;
            break;
        }
    }
    let kf_low   = &keyframes[low];
    let kf_high = &keyframes[high];
    let t = (now - kf_low.time) as f32 / (kf_high.time - kf_low.time) as f32;
    
    let trans_vec = lerp(kf_low.trans, kf_high.trans, t);

    //Compute rotation
    let quat_low  = euler_angle_to_quaternion(kf_low.rot);
    let quat_high = euler_angle_to_quaternion(kf_high.rot);
    let quat_slerp = slerp(quat_low, quat_high, t);
    let rot_mat = center_then_rotate(node.center_rot, quat_slerp);

    vector_to_translation(&trans_vec) * rot_mat
}
