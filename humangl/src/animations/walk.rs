use crate::animation::Keyframe;
use crate::node::Node;
use crate::mesh::Mesh;
use crate::create_cuboid::create_cuboid;
use matrix::{Vector, Vector3f, graphic_operations::*};

//this class creates the mesh and vector of keyframes for each body parts
//the animation is set to walk
//call get_body() to get the main node

pub fn head() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn body() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 2.].into(),
    });
    keyframes.push(Keyframe{
        time:  5000,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., -2.].into(),
    });
    keyframes.push(Keyframe{
        time:  5001,
        rot:   [0., 90., 0.].into(),
        trans: [2., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  10000,
        rot:   [0., 90., 0.].into(),
        trans: [-2., 0., 0.].into(),
    });
    keyframes
}

pub fn rhand() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn lhand() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn rarm() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   Vector::from([-45., 0., 0.]),
        trans: [0., 0., 0.].into(),
	});
    keyframes.push(Keyframe{
        time:  500,
        rot:   Vector::from([45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   Vector::from([-45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn larm() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   Vector::from([45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   Vector::from([-45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   Vector::from([45., 0., 0.]),
        trans: [0., 0., 0.].into(),
	});
    keyframes
}

pub fn rleg() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   Vector::from([45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   Vector::from([-45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   Vector::from([45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn lleg() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   Vector::from([-45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   Vector::from([45., 0., 0.]),
        trans: [0., 0., 0.].into(),
	});
    keyframes.push(Keyframe{
        time:  1000,
        rot:   Vector::from([-45., 0., 0.]),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn rfoot() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes
}

pub fn lfoot() -> Vec<Keyframe> {
    let mut keyframes = Vec::new();
    keyframes.push(Keyframe{
        time:  0,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  500,
        rot:   [0., 0., 0.].into(),
        trans: [0., 0., 0.].into(),
    });
    keyframes.push(Keyframe{
        time:  1000,
        rot:   [0., 0., 0.].into(),
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
                         Vector3f::from([0., 0., 0.]));
    let rhand = Node::new("rhand", mesh_rhand, Vec::new(), rhand(),
                          translation_v(&[0.0, -0.3, 0.].into()),
                          [0., 0.15, 0.].into());
    let rarm = Node::new("rarm", mesh_rarm, Vec::from([rhand]), rarm(),
                         translation_v(&[0.275, 0.1, 0.].into()),
                         [0., 0.125, 0.].into());
    let lhand = Node::new("lhand", mesh_lhand, Vec::new(), lhand(),
				translation_v(&Vector::from([0.0, -0.3, 0.])),
				[0., 0.15, 0.].into());
    let larm = Node::new("larm", mesh_larm, Vec::from([lhand]), larm(),
				translation_v(&[-0.275, 0.1, 0.].into()),
				[0., 0.125, 0.].into());
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
