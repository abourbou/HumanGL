
use std::{mem};
use std::os::raw::c_void;

use matrix::{Vector};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub color: Vector<f32, 3>,
    pub vao: u32,

    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>, color : Vector<f32, 3>) -> Mesh {
        let mut mesh = Mesh {
            vertices, indices, color,
            vao: 0, vbo: 0, ebo: 0,
        };

        unsafe {
            // Create vbo
            gl::CreateBuffers(1, &mut mesh.vbo);
            let size = (mesh.vertices.len() * mem::size_of::<f32>()) as isize;
            let data = &mesh.vertices[0] as *const f32 as *const c_void;
            gl::NamedBufferStorage(mesh.vbo, size, data, gl::DYNAMIC_STORAGE_BIT);
        
            // Create ebo
            gl::CreateBuffers(1, &mut mesh.ebo);
            let size = (mesh.indices.len() * mem::size_of::<u32>()) as isize;
            let data = &mesh.indices[0] as *const u32 as *const c_void;
            gl::NamedBufferStorage(mesh.ebo, size, data, gl::DYNAMIC_STORAGE_BIT);
        
            // Create vao
            gl::CreateVertexArrays(1, &mut mesh.vao);
        
            // Bind it to vbo and ebo
            gl::VertexArrayVertexBuffer(mesh.vao, 0, mesh.vbo, 0, (3 * mem::size_of::<f32>()) as i32);
            gl::VertexArrayElementBuffer(mesh.vao, mesh.ebo);
        
            // Create attribute for vertices
            gl::EnableVertexArrayAttrib(mesh.vao, 0);
            gl::VertexArrayAttribFormat(mesh.vao, 0, 3, gl::FLOAT, gl::FALSE, 0);
            gl::VertexArrayAttribBinding(mesh.vao, 0, 0);
        
            // Unbind it
            gl::BindVertexArray(0);
        }
        mesh
    }
}
