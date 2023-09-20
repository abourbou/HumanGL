use crate::animation::Keyframe;
use crate::node::Node;
use crate::mesh::Mesh;
use crate::create_cuboid::create_cuboid;
use matrix::{Vector, Vector3f, graphic_operations::*};

//this class creates the mesh and vector of keyframes for each body parts
//the animation is set to jump
//call get_body() to get the main node

fn head() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: [30., 80., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 1500,
        rot: [30., 80., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [30., 80., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn body() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 250,
        rot: [0., -60., 0.].into(),
        trans: [0., 1., 1.].into(),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: [0., -120., 0.].into(),
        trans: [0., 0., 1.].into(),
    });
    keyframes.push(Keyframe{
        time: 1500,
        rot: [0., -120., 0.].into(),
        trans: [0., 0., 1.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [0., -120., 0.].into(),
        trans: [0., 0., 1.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [-20., 0., 0.].into(),
        trans: [-0.05, 0., -0.45].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [-20., 0., 0.].into(),
        trans: [-0.05, 0., -0.45].into(),
    });
    keyframes
}

fn rhand() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: [120., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [120., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn lhand() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();

    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [135., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [135., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn rarm() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 50,
        rot: [90., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 100,
        rot: [180., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 150,
        rot: [270., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 200,
        rot: [360., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 250,
        rot: [90., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 300,
        rot: [180., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 350,
        rot: [270., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 400,
        rot: [360., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 450,
        rot: [90., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: [90., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 550,
        rot: [180., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 600,
        rot: [270., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 650,
        rot: [360., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 700,
        rot: [90., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 750,
        rot: [180., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 800,
        rot: [270., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 850,
        rot: [360., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 900,
        rot: [0., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 1200,
        rot: [0., -40., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 1300,
        rot: [0., -20., 30.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 1900,
        rot: [-45., 0., 60.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [-45., 0., 60.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [90., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [90., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn larm() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: [45., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [45., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [-45., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [-45., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn rleg() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [-45., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [-45., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn lleg() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: [90., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [90., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3100,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn rfoot() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

fn lfoot() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time: 0,
        rot: [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 500,
        rot: [-10., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 1500,
        rot: [-10., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 3000,
        rot: [-10., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 4000,
        rot: [-10., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time: 5000,
        rot: [-10., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn get_body() -> Node {
    let mesh_head : Mesh  = create_cuboid(0.36, 0.3, 0.3, [1.0, 0.8, 0.6].into());
    let mesh_rarm : Mesh  = create_cuboid(0.18, 0.3, 0.15, [0., 1., 1.].into());
    let mesh_rhand : Mesh = create_cuboid(0.18, 0.3, 0.15, [1.0, 0.8, 0.6].into());
    let mesh_lhand : Mesh = create_cuboid(0.18, 0.3, 0.15, [1.0, 0.8, 0.6].into());
    let mesh_larm : Mesh  = create_cuboid(0.18, 0.3, 0.15, [0., 1., 1.].into());
    let mesh_rleg : Mesh  = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_rfoot : Mesh = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_lleg : Mesh  = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_lfoot : Mesh = create_cuboid(0.18, 0.3, 0.15, [0.43, 0.2, 1.].into());
    let mesh_body : Mesh  = create_cuboid(0.36, 0.5, 0.15, [0., 1., 1.].into());

    let head = Node::new("head", mesh_head, Vec::new(), head(),
                         translation_v(&[0.0, 0.4, 0.].into()),
                         Vector3f::from([0., -0.15, 0.]));
    let rhand = Node::new("rhand", mesh_rhand, Vec::new(), rhand(),
                          translation_v(&[0.0, -0.3, 0.].into()),
                          [0., 0.15, 0.].into());
    let rarm = Node::new("rarm", mesh_rarm, Vec::from([rhand]), rarm(),
                         translation_v(&[0.275, 0.1, 0.].into()),
                         [0., 0.15, 0.].into());
    let lhand = Node::new("lhand", mesh_lhand, Vec::new(), lhand(),
				translation_v(&Vector::from([0.0, -0.3, 0.])),
				[0., 0.15, 0.].into());
    let larm = Node::new("larm", mesh_larm, Vec::from([lhand]), larm(),
				translation_v(&[-0.275, 0.1, 0.].into()),
				[0., 0.15, 0.].into());
    let rfoot = Node::new("rfoot", mesh_rfoot, Vec::from([]), rfoot(),
				translation_v(&Vector::from([0., -0.3, 0.])),
				[0., 0.15, 0.].into());
    let rleg = Node::new("rleg", mesh_rleg, Vec::from([rfoot]), rleg(),
				translation_v(&Vector::from([0.08, -0.4, 0.])),
				[0., 0.125, 0.].into());
    let lfoot = Node::new("lfoot", mesh_lfoot, Vec::from([]), lfoot(),
				translation_v(&Vector::from([0., -0.3, 0.])),
				[0., 0.15, 0.].into());
    let lleg = Node::new("lleg", mesh_lleg, Vec::from([lfoot]), lleg(),
				translation_v(&Vector::from([-0.08, -0.4, 0.])),
				[0., 0.125, 0.].into());
    let body = Node::new("body", mesh_body, Vec::from([head, rarm, larm, rleg, lleg]), body(),
                         translation_v(&[0., 0., 0.].into()),
                         Vector3f::from([0., 0., 0.]));

	body
}
