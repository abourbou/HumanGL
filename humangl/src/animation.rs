use matrix::{Vector, Matrix};
use matrix::linear_operations::linear_interp::lerp;

type TVector3<T> = Vector<T, 3>;
type TMatrix4<T> = Matrix<T, 4, 4>;

pub struct Keyframe {
    pub time: u32, //ms
    pub rot: TVector3<f32>,
    pub trans: TVector3<f32>,
}

fn create_iso(keyframes: Vec<Keyframe>, time: u32) -> TMatrix4<f32> {
    let start_time = keyframes[0].time;
    let end_time = keyframes.last().unwrap().time;
    let now = time % (end_time - start_time);
    let mut low = 0;
    let mut high = 0;
    for i in 1..keyframes.len() {
        high = i;
        low = i - 1;
        if keyframes[i].time >= keyframes[high].time {
            break;
        }
    }
    let t = (now - keyframes[low].time) as f32 / (keyframes[high].time - keyframes[low].time) as f32;
    let rot_vec = lerp(keyframes[low].rot, keyframes[high].rot, t);
    let trans_vec = lerp(keyframes[low].trans, keyframes[high].trans, t);
    let rot_mat = get_rotation(rot_vec);
    let trans_mat = get_translation(trans_vec);
    rot_mat * trans_mat
}

pub fn animate_rhand(time: u32) -> TMatrix4<f32> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([0., 90., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    create_iso(keyframes, time)
}