// pub struct Mesh {
//     pub vertices: Vec<Vertex>,
//     pub indices: Vec<u32>,
//     pub VAO: u32,
//     pub color: Vector<f32, 3>,

//     VBO: u32,
//     EBO: u32,
// }
extern crate gl;

use std::ptr;
use gl::types::GLint;
use matrix::{Vector, Matrix, Matrix4};


pub fn add_forty_two(x: u32) -> u32 {x + 42}
pub fn sub_forty_two(x: u32) -> u32 {x - 42}
use crate::animation;
use crate::animation::{Keyframe};
use crate::mesh::Mesh;
use crate::create_cuboid::create_cuboid;

type TVector3<T> = Vector<T, 3>;
type TMatrix4<T> = Matrix<T, 4, 4>;

#[derive(Clone, Debug)]
pub struct Node {
    pub mesh: Mesh,
    pub name : String,
    pub children: Vec<Node>,
    pub keyframes: Vec<Keyframe>,
    pub rot_center: TVector3<f32>,
    // pub isometry : i32,
}

impl Node {
    pub fn new(name: &str, mesh: Mesh, children: Vec<Node>, keyframes: Vec<Keyframe>, rot_center: TVector3<f32>) -> Node {
        Node {
            name: name.to_string(),
            mesh,
            children,
            keyframes,
            rot_center,
        }
    }

    pub fn set_name(&mut self, name: &str, new_name: &str) {
        fn recursion(node: &mut Node, name: &str, new_name: &str) {
            if node.name == name {
                node.name = new_name.to_string();
            } else {
                for it in node.children.iter_mut() {
                    recursion(it, name, new_name);
                }
            }
        }
        recursion(self, name, new_name);
    }

    fn draw_mesh(&mut self, color_location: GLint) {
        unsafe {

        }
    }

    pub fn render_animation(&mut self, time: u32, model_location: GLint, color_location: GLint) {
        fn recursion(node: &mut Node, time: u32, model_location: GLint, color_location: GLint) {
            let iso_matrix = animation::animate(node.keyframes.clone(), time);
            // let iso = iso_matrix.transpose();
            let to_rot_center = animation::get_translation(node.rot_center * -1.);
            let iso = iso_matrix * to_rot_center;
            let model: Vec<f32> = iso.arr.iter().flat_map(|row| row.iter().cloned()).collect();
            // create matrix and render
            unsafe {
                gl::UniformMatrix4fv(model_location, 1, gl::TRUE, model.as_ptr());
                gl::Uniform3fv(color_location, 1, node.mesh.color.arr.as_ptr());
                gl::BindVertexArray(node.mesh.vao);
                gl::DrawElements(gl::TRIANGLES, node.mesh.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            }

            for it in node.children.iter_mut() {
                recursion(it, time, model_location, color_location);
            }
        }
        recursion(self, time, model_location, color_location);
    }

    pub fn info(& mut self) {
        fn recursion(node: &mut Node, data: String) {
            for child in node.children.iter_mut() {
                let mut total = data.clone();
                total.push_str(":");
                total.push_str(&child.name);
                
                println!("my name is: {}, route is: {:?}", child.name, total);
                recursion(child, total.clone());
            }
        }
        println!("my name is: {}, route is : {}", self.name, self.name);
        recursion(self, self.name.clone());
    }
}

pub fn show() {
    let mesh = create_cuboid(1., 1., 1., [1.0, 0.5, 0.2].into());
    let mut rhand = Node::new("rhand", mesh, Vec::new(), animation::walk_rhand(), TVector3::from([0., 1., 0.]));

    // for i in 0..1000 {
    //     rhand.exec_function(i);
    // }
    // let mut rhand = Node::new("rhand", Vec::new(), animation::walk_rhand());
    // let mut rarm = Node::new("rarm", Vec::from([rhand]), animation::no_animation());
    // let mut lhand = Node::new("lhand", Vec::new(), animation::no_animation());
    // let mut larm = Node::new("larm", Vec::from([lhand]), animation::no_animation());
    // let mut body = Node::new("body", Vec::from([rarm, larm]), animation::no_animation());
    // println!("before");
    // body.info();
    // println!("after");
    // body.set_name("rarm", "new_rarm");
    // body.info();
    // body.exec_function(42);
}
