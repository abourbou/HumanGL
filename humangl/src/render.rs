extern crate glfw;
use self::glfw::{Context, Key, Action, Glfw, Window, WindowEvent};
extern crate gl;

use std::sync::mpsc::Receiver;
use std::{ptr};


use crate::compute_shader::compute_shader;
use crate::mesh::Mesh;
use crate::create_cuboid::create_cuboid;
use matrix::Vector;

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn initialize_glfw() -> (Glfw, Window, Receiver<(f64, WindowEvent)>){
	// glfw: initialize and configure
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
	glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
	glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
	#[cfg(target_os = "macos")]
	glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

	// glfw window creation
	let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "HumanGL", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window");

	window.make_current();
	window.set_key_polling(true);
	window.set_framebuffer_size_polling(true);

	// gl: load all OpenGL function pointers
	gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        // Enable depth test
        gl::Enable(gl::DEPTH_TEST);
        // Accept fragment if it closer to the camera than the former one
        gl::DepthFunc(gl::LESS);
    }
	(glfw, window, events)
}

pub fn window() {

	let (mut glfw, mut window, events) = initialize_glfw();
    let shader_program = compute_shader("humangl/shaders/vertex_shader.vs", "humangl/shaders/fragment_shader.fs");
    let mesh : Mesh = create_cuboid(1., 1.01, 1., [1.0, 0.5, 0.2].into());
    let mesh2 : Mesh = create_cuboid(1.5, 1., 0.6, [0.2, 0.5, 0.8].into());

    let color_string = std::ffi::CString::new("color").unwrap();
    let color_location = unsafe {
        gl::GetUniformLocation(shader_program, color_string.as_ptr())
    };

    let mvp_string = std::ffi::CString::new("MVP").unwrap();
    let mvp_location = unsafe {
        gl::GetUniformLocation(shader_program, mvp_string.as_ptr())
    };

    let model      = matrix::Matrix4f::identity();
    let view       = matrix::graphic_operations::view_matrix(Vector::from([0., 0., -1.5]), Vector::from([0.,0.,0.]), Vector::from([0.,1.,0.]));
    let projection = matrix::graphic_operations::projection_matrix(90., 4./3., 0.01, 100.);

    // Create MVP, transpose because openGL is row major
    let mvp = (projection * view * model).transpose();
    let flat_mvp : Vec<f32> = mvp.arr.iter().flat_map(|row| row.iter().cloned()).collect();

    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UseProgram(shader_program);
            
            gl::UniformMatrix4fv(mvp_location, 1, gl::FALSE, flat_mvp.as_ptr());

            // Draw our first rectangle
            gl::Uniform3fv(color_location, 1, mesh.color.arr.as_ptr());
            gl::BindVertexArray(mesh.vao);
            gl::DrawElements(gl::TRIANGLES, mesh.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());

            // Draw our second rectangle
            gl::Uniform3fv(color_location, 1, mesh2.color.arr.as_ptr());
            gl::BindVertexArray(mesh2.vao);
            gl::DrawElements(gl::TRIANGLES, mesh.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());

            gl::BindVertexArray(0);
        }


        // glfw: swap buffers and poll IO events
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
