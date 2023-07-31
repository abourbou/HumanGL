
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

pub fn compute_shader(vertex_path: &str, fragment_path: &str) -> GLuint {
    let shader_id : GLuint;
    
    let mut v_shader_file = File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
    let mut f_shader_file = File::open(fragment_path).unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
    
    let mut vertex_code = String::new();
    let mut fragment_code = String::new();
    
    v_shader_file.read_to_string(&mut vertex_code).expect("Failed to read vertex shader");
    f_shader_file.read_to_string(&mut fragment_code).expect("Failed to read fragment shader");
    
    let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
    let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

    unsafe {
        // Create vertex shader
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
        gl::CompileShader(vertex);
        check_compile_errors(vertex, "VERTEX");

        // Create fragment Shader
        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
        gl::CompileShader(fragment);
        check_compile_errors(fragment, "FRAGMENT");

        // Create shader Program
        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex);
        gl::AttachShader(id, fragment);
        gl::LinkProgram(id);
        check_compile_errors(id, "PROGRAM");

        // Delete the shaders as they're linked into our program now and no longer necessary
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);
        shader_id = id;
    }

    shader_id
}

// Check shader compilation/linking errors
unsafe fn check_compile_errors(shader: u32, type_: &str) {
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(1024);
    // Subtract 1 to skip the trailing null character
    info_log.set_len(1024 - 1);
    if type_ != "PROGRAM" {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                      -- --------------------------------------------------- -- ",
                      type_,
                      str::from_utf8(&info_log).unwrap());
        }

    } else {
        gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                      -- --------------------------------------------------- -- ",
                      type_,
                      str::from_utf8(&info_log).unwrap());
        }
    }
}
