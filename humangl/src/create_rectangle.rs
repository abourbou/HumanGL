
use matrix::Vector;

pub fn create_rectangle(length : i32, width : i32, color : Vector::<f32, 3>) {

}


// pub fn create_rectangle(length: i32, width: i32, color: Vector::<f32, 3>) -> (GLuint, GLuint) {
//     let (shader_program, vao) = unsafe {
//         // build and compile our shader program
//         let shader_program = compute_shader("humangl/shaders/vertex_shader.vs", "humangl/shaders/fragment_shader.fs");

//         // set up vertex data (and buffer(s)) and configure vertex attributes
//         // ------------------------------------------------------------------
//         // HINT: type annotation is crucial since default for float literals is f64
//         let vertices: [f32; 12] = [
//              0.5,  0.5, 0.0,  // top right
//              0.5, -0.5, 0.0,  // bottom right
//             -0.5, -0.5, 0.0,  // bottom left
//             -0.5,  0.5, 0.0   // top left
//         ];
//         let indices = [ // note that we start from 0!
//             0, 1, 3,  // first Triangle
//             1, 2, 3   // second Triangle
//         ];
//         let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
//         gl::GenVertexArrays(1, &mut vao);
//         gl::GenBuffers(1, &mut vbo);
//         gl::GenBuffers(1, &mut ebo);
//         // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
//         gl::BindVertexArray(vao);

//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         gl::BufferData(gl::ARRAY_BUFFER,
//                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
//                        &vertices[0] as *const f32 as *const c_void,
//                        gl::STATIC_DRAW);

//         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
//         gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
//                        (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
//                        &indices[0] as *const i32 as *const c_void,
//                        gl::STATIC_DRAW);

//         gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
//         gl::EnableVertexAttribArray(0);

//         // note that this is allowed, the call to gl::VertexAttribPointer registered vbo as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
//         gl::BindBuffer(gl::ARRAY_BUFFER, 0);

//         // remember: do NOT unbind the ebo while a vao is active as the bound element buffer object IS stored in the vao; keep the ebo bound.
//         // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

//         // You can unbind the vao afterwards so other vao calls won't accidentally modify this vao, but this rarely happens. Modifying other
//         // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
//         gl::BindVertexArray(0);

//         // uncomment this call to draw in wireframe polygons.
//         // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

//         (shader_program, vao)
//     };
//     (shader_program, vao)
// }