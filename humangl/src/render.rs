extern crate glfw;
use self::glfw::{Context, Key, Action, Glfw, Window, WindowEvent};
extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::{ptr, mem};
use std::os::raw::c_void;

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

	(glfw, window, events)
}

pub fn create_rectangle() -> (GLuint, GLuint) {
    let (shader_program, vao) = unsafe {
        // build and compile our shader program
        let shader_program = compute_shader("humangl/shaders/vertex_shader.vs", "humangl/shaders/fragment_shader.fs");

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 24] = [
            //first rectangle
             0.5,  0.5, 0.0,  // top right
             0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
            -0.5,  0.5, 0.0,  // top left

            //second rectangle
            0.6,  0.6, 0.0,  // top right
            0.6,  0.5, 0.0,  // bottom right
            0.5,  0.5, 0.0,  // bottom left
            0.5,  0.6, 0.0,  // top left
        ];
        let indices = [ // note that we start from 0!
            0, 1, 3,  // first Triangle
            1, 2, 3,  // second Triangle

            4, 5, 7,  // first Triangle
            5, 6, 7,  // second Triangle
        ];
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // remember: do NOT unbind the ebo while a vao is active as the bound element buffer object IS stored in the vao; keep the ebo bound.
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao)
    };
    (shader_program, vao)
}

pub fn create_triangle() -> (GLuint, GLuint) {
    let (shader_program, vao) = unsafe {
        // build and compile our shader program
        let shader_program = compute_shader("humangl/shaders/vertex_shader.vs", "humangl/shaders/fragment_shader.fs");

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 24] = [
            //first rectangle
             0.5,  0.5, 0.0,  // top right
             0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
            -0.5,  0.5, 0.0,  // top left

            //seconde rectangle
            1.5,  1.5, 0.0,  // top right
            1.5, -1.5, 0.0,  // bottom right
           -1.5, -1.5, 0.0,  // bottom left
           -1.5,  1.5, 0.0,  // top left
        ];
        let indices = [ // note that we start from 0!
            0, 1, 3,  // first Triangle
            1, 2, 3,  // second Triangle

            4, 5, 7,  // first Triangle
            5, 6, 7,  // second Triangle
        ];
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // remember: do NOT unbind the ebo while a vao is active as the bound element buffer object IS stored in the vao; keep the ebo bound.
        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

        // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao)
    };
    (shader_program, vao)
}

pub fn window() {

	let (mut glfw, mut window, events) = initialize_glfw();
    let (shader_program, vao) = create_rectangle();
    // let (shader_program, vao) = create_triangle();
    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            // draw our first triangle
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 12, gl::UNSIGNED_INT, ptr::null());
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
