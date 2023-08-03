use crate::animation::Keyframe;
use matrix::{Vector, Matrix};

pub fn head() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 10000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn body() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 10000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn rhand() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 10000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn rarm() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: Vector::from([0., 0., 360.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 10000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}