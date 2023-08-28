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
use matrix::graphic_operations::scaling_v;

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
    pub start_pose : TMatrix4<f32>,
}

impl Node {
    pub fn new(name: &str, mesh: Mesh, children: Vec<Node>, keyframes: Vec<Keyframe>, start_pose: TMatrix4<f32>) -> Node {
        Node {
            name: name.to_string(),
            mesh,
            children,
            keyframes,
            start_pose,
        }
    }

    pub fn render_animation(&mut self, time: u32, model_location: GLint, color_location: GLint) {
        fn recurs_render(node: &mut Node, time: u32, model_location: GLint, color_location: GLint, previous_isometry: Matrix4<f32>) {
            // create iso matrix
            let animate_mat = animation::animate(&node.keyframes, time) * scaling_v(node.mesh.scaling);
            let iso_mat = previous_isometry * node.start_pose * animate_mat;
            let model: Vec<f32> = iso_mat.arr.iter().flat_map(|row| row.iter().cloned()).collect();
            // create matrix and render
            unsafe {
                gl::UniformMatrix4fv(model_location, 1, gl::TRUE, model.as_ptr());
                gl::Uniform3fv(color_location, 1, node.mesh.color.arr.as_ptr());
                gl::BindVertexArray(node.mesh.vao);
                gl::DrawElements(gl::TRIANGLES, node.mesh.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            }

            for it in node.children.iter_mut() {
                recurs_render(it, time, model_location, color_location, iso_mat);
            }
        }

        recurs_render(self, time, model_location, color_location, Matrix4f::identity())
    }
}
