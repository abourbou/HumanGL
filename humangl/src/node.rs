extern crate gl;

use std::ptr;
use gl::types::GLint;
use matrix::Matrix4f;
use matrix::graphic_operations::scaling_v;
use matrix::*;

use crate::animation;
use crate::animation::Keyframe;
use crate::mesh::Mesh;

#[derive(Clone, Debug)]
pub struct Node {
    pub mesh: Mesh,
    pub name : String,
    pub children: Vec<Node>,
    pub keyframes: Vec<Keyframe>,
    pub start_pose : Matrix4f,
    pub center_rot : Vector3f,
}

impl Node {
    pub fn new(name: &str, mesh: Mesh, children: Vec<Node>, keyframes: Vec<Keyframe>, start_pose: Matrix4f, center_rot: Vector3f) -> Node {
        Node {
            name: name.to_string(),
            mesh,
            children,
            keyframes,
            start_pose,
            center_rot,
        }
    }

    fn recurs_render(&self, time: u32, model_location: GLint, color_location: GLint, previous_isometry: Matrix4f) {
        // create iso matrix
        let animate_mat = animation::animate(self, time);
        let iso_mat = previous_isometry * self.start_pose * animate_mat;
  
        let final_transfo = iso_mat * scaling_v(self.mesh.scaling);
        let model: Vec<f32> = final_transfo.arr.iter().flat_map(|row| row.iter().cloned()).collect();
        // create matrix and render
        unsafe {
            gl::UniformMatrix4fv(model_location, 1, gl::TRUE, model.as_ptr());
            gl::Uniform3fv(color_location, 1, self.mesh.color.arr.as_ptr());
            gl::BindVertexArray(self.mesh.vao);
            gl::DrawElements(gl::TRIANGLES, self.mesh.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        }

        for it in self.children.iter() {
            it.recurs_render(time, model_location, color_location, iso_mat);
        }
    }

    pub fn render_animation(&self, time: u32, model_location: GLint, color_location: GLint) {
        self.recurs_render(time, model_location, color_location, Matrix4f::identity())
    }
}
