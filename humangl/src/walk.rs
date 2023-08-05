use crate::animation::Keyframe;
use matrix::Vector;

pub fn head() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
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
        time: 500,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
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
        time: 500,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn lhand() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn rarm() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([-45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([-45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn larm() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([-45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn rleg() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([-45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn lleg() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([-45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([-45., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn rfoot() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

pub fn lfoot() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 1000,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}