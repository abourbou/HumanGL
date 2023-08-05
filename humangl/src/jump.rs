use crate::animation;
use crate::animation::Keyframe;
use crate::tree::Node;
use crate::mesh::Mesh;
use crate::create_cuboid::create_cuboid;
use matrix::Vector;

//this class creates the mesh and vector of keyframes for each body parts
//the animation is set to jump
//call get_body() to get the main node

fn head() -> Vec<Keyframe> {
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

fn body() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes.push(Keyframe{
        time: 250,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0.35, 0.]),
    });
    keyframes.push(Keyframe{
        time: 400,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0.5, 0.]),
    });
    keyframes.push(Keyframe{
        time: 550,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0.35, 0.]),
    });
    keyframes.push(Keyframe{
        time: 800,
        rot: Vector::from([0., 0., 0.]),
        trans: Vector::from([0., 0., 0.]),
    });
    keyframes
}

fn rhand() -> Vec<Keyframe> {
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

fn lhand() -> Vec<Keyframe> {
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

fn rarm() -> Vec<Keyframe> {
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

fn larm() -> Vec<Keyframe> {
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

fn rleg() -> Vec<Keyframe> {
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

fn lleg() -> Vec<Keyframe> {
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

fn rfoot() -> Vec<Keyframe> {
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

fn lfoot() -> Vec<Keyframe> {
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

pub fn get_body() -> Node {
    let mesh_head : Mesh = create_cuboid(0.36, 0.3, 0.3, [1.0, 0.8, 0.6].into());
    let mesh_rhand : Mesh = create_cuboid(0.18, 0.3, 0.15, [1.0, 0.8, 0.6].into());
    let mesh_rarm : Mesh = create_cuboid(0.18, 0.3, 0.15, [0., 1., 1.].into());
    let mesh_lhand : Mesh = create_cuboid(0.18, 0.3, 0.15, [1.0, 0.8, 0.6].into());
    let mesh_larm : Mesh = create_cuboid(0.18, 0.3, 0.15, [0., 1., 1.].into());
    let mesh_rleg : Mesh = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_rfoot : Mesh = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_lleg : Mesh = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_lfoot : Mesh = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_body : Mesh = create_cuboid(0.36, 0.5, 0.15, [0., 1., 1.].into());

    let head = Node::new("head", mesh_head, Vec::new(), head(), animation::get_translation(&Vector::from([0.0, 0.15, 0.])));
    let rhand = Node::new("rhand", mesh_rhand, Vec::new(), rhand(),  animation::get_translation(&Vector::from([0.0, -0.15, 0.])));
    let rarm = Node::new("rarm", mesh_rarm, Vec::from([rhand]), rarm(),  animation::get_translation(&Vector::from([0.125, 0.0, 0.])));
    let lhand = Node::new("lhand", mesh_lhand, Vec::new(), lhand(),  animation::get_translation(&Vector::from([0.0, -0.15, 0.])));
    let larm = Node::new("larm", mesh_larm, Vec::from([lhand]), larm(),  animation::get_translation(&Vector::from([-0.125, 0.0, 0.])));
    let rfoot = Node::new("rfoot", mesh_rfoot, Vec::from([]), rfoot(),  animation::get_translation(&Vector::from([0., -0.15, 0.])));
    let rleg = Node::new("rleg", mesh_rleg, Vec::from([rfoot]), rleg(),  animation::get_translation(&Vector::from([0.045, -0.25, 0.])));
    let lfoot = Node::new("lfoot", mesh_lfoot, Vec::from([]), lfoot(),  animation::get_translation(&Vector::from([0., -0.15, 0.])));
    let lleg = Node::new("lleg", mesh_lleg, Vec::from([lfoot]), lleg(),  animation::get_translation(&Vector::from([-0.045, -0.25, 0.])));
    let body = Node::new("body", mesh_body, Vec::from([head, rarm, larm, rleg, lleg]), body(),  animation::get_translation(&Vector::from([0., 0., 0.])));
    body
}
