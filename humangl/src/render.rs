extern crate glfw;
use self::glfw::{Context, Key, Action, Glfw, Window, WindowEvent};
extern crate gl;

use std::sync::mpsc::Receiver;
use std::time::SystemTime;

use crate::create_cuboid::create_unit_cuboid;

use crate::test_animation;

use crate::walk;
use crate::compute_shader::compute_shader;
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
    let mut walk_body = walk::get_body();


    let color_string = std::ffi::CString::new("color").unwrap();
    let color_location = unsafe {
        gl::GetUniformLocation(shader_program, color_string.as_ptr())
    };

    let model_string = std::ffi::CString::new("model").unwrap();
    let model_location = unsafe {
        gl::GetUniformLocation(shader_program, model_string.as_ptr())
    };

    let view_string = std::ffi::CString::new("view").unwrap();
    let view_location = unsafe {
        gl::GetUniformLocation(shader_program, view_string.as_ptr())
    };

    let proj_string = std::ffi::CString::new("projection").unwrap();
    let proj_location = unsafe {
        gl::GetUniformLocation(shader_program, proj_string.as_ptr())
    };

    let view = matrix::graphic_operations::view_matrix(Vector::from([0., 0., -1.5]), Vector::from([0.,0.,0.]), Vector::from([0.,1.,0.]));
    let proj = matrix::graphic_operations::projection_matrix(90., 4./3., 0.01, 100.);

    let flat_view: Vec<f32> = view.arr.iter().flat_map(|row| row.iter().cloned()).collect();
    let flat_proj: Vec<f32> = proj.arr.iter().flat_map(|row| row.iter().cloned()).collect();

    unsafe {
        gl::UseProgram(shader_program);

        gl::UniformMatrix4fv(view_location, 1, gl::TRUE, flat_view.as_ptr());
        gl::UniformMatrix4fv(proj_location, 1, gl::TRUE, flat_proj.as_ptr());
    }

    //set timer
    let sys_time = SystemTime::now();

    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(shader_program);

            //wire mode
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            // Animation with Node
            let time = sys_time.elapsed().unwrap().as_millis() as u32;

            //* KAZUMA
                walk_body.render_animation(time, model_location, color_location);
            //* KAZUMA

            // ! ARTHUR
                // test_animation::test_animation(model_location, color_location, time);
            // ! ARTHUR

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
