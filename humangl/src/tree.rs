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
use matrix::{Matrix, Matrix4, Matrix4f};

use crate::animation;
use crate::animation::Keyframe;
use crate::mesh::Mesh;

type TMatrix4<T> = Matrix<T, 4, 4>;

#[derive(Clone, Debug)]
pub struct Node {
    pub mesh: Mesh,
    pub name : String,
    pub children: Vec<Node>,
    pub keyframes: Vec<Keyframe>,
    pub isometry : TMatrix4<f32>,
}

impl Node {
    pub fn new(name: &str, mesh: Mesh, children: Vec<Node>, keyframes: Vec<Keyframe>, isometry: TMatrix4<f32>) -> Node {
        Node {
            name: name.to_string(),
            mesh,
            children,
            keyframes,
            isometry,
        }
    }

    pub fn render_animation(&mut self, time: u32, model_location: GLint, color_location: GLint) {
        fn recursion(node: &mut Node, time: u32, model_location: GLint, color_location: GLint, data: Matrix4<f32>) {
            // create iso matrix
            let animate_mat = animation::animate(&node.keyframes, time);
            let iso_mat = data * node.isometry * animate_mat;
            let model: Vec<f32> = iso_mat.arr.iter().flat_map(|row| row.iter().cloned()).collect();
            // create matrix and render
            unsafe {
                gl::UniformMatrix4fv(model_location, 1, gl::TRUE, model.as_ptr());
                gl::Uniform3fv(color_location, 1, node.mesh.color.arr.as_ptr());
                gl::BindVertexArray(node.mesh.vao);
                gl::DrawElements(gl::TRIANGLES, node.mesh.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            }

            for it in node.children.iter_mut() {
                recursion(it, time, model_location, color_location, iso_mat);
            }
        }
        let data = Matrix4f::identity();
        recursion(self, time, model_location, color_location, data);
    }
}
