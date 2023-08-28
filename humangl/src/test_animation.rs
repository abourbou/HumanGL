use crate::animation;
use crate::animation::Keyframe;
use crate::tree::Node;
use crate::mesh::Mesh;
use crate::create_cuboid::create_cuboid;
use matrix::{Vector, Matrix4f};
use gl::types::*;
use crate::create_cuboid::create_unit_cuboid;
use matrix::graphic_operations::*;
// pub fn get_body() -> Node {
//     let mesh_head : Mesh = create_cuboid(1., 1., 1., [1.0, 0.8, 0.6].into());
//     let mesh_rhand : Mesh = create_cuboid(1., 1., 1., [1.0, 0.8, 0.6].into());
//     let mesh_rarm : Mesh = create_cuboid(1., 1., 1., [0., 1., 1.].into());
//     let mesh_rarm : Mesh = create_cuboid(1., 1., 1., [0., 1., 1.].into());

//     let mesh_body : Mesh = create_cuboid(0.36, 0.5, 0.15, [0., 1., 1.].into());
// }

fn center_then_rotate(isometry: matrix::Matrix4f, rotation : matrix::Matrix4f) -> matrix::Matrix4f {
    let pos = matrix::Vector3f::from([isometry[0][3], isometry[1][3], isometry[2][3]]);

    translation_v(-1. * pos) * rotation * translation_v(pos)
}

unsafe fn draw_cube(model_location : GLint, color_location : GLint, cube : &Mesh, isometry : matrix::Matrix4f)
{
    let gl_model: Vec<f32> = isometry.arr.iter().flat_map(|row| row.iter().cloned()).collect();

    gl::UniformMatrix4fv(model_location, 1, gl::TRUE, gl_model.as_ptr());
    gl::Uniform3fv(color_location, 1, cube.color.arr.as_ptr());
    gl::BindVertexArray(cube.vao);
    gl::DrawElements(gl::TRIANGLES, cube.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
}

pub fn test_animation(model_location : GLint, color_location : GLint, time : u32) {
    // Cube 1 (body)
    let cube1 = create_unit_cuboid([1.0, 0.8, 0.6].into());
    let isometry1 = Matrix4f::identity()
					* rotation::ry(((time / 50) % 360) as f32)
                    // * rotation::ry(75.)
					// * scaling(0.5, 1., 0.1)
                    * Matrix4f::identity()
                    ;
	let scaling1 = scaling(0.5, 1., 0.1);

    // Cube 2 (arm)
    let cube2 = create_unit_cuboid([0.2, 0.4, 0.7].into());
    // let isometry2 = isometry1 * translation(1., 1., 0.);
	// let q_x = matrix::Vector4f::from([1., 0., 0., 0.]);
	// let q_y = matrix::Vector4f::from([0., 1., 0., 0.]);
	let q_x = axis_angle_to_quaternion([1., 0., 0.].into(), 0.);
	let q_y = axis_angle_to_quaternion([1., 0., 0.].into(), std::f32::consts::PI / 2.);
	println!("q_x: {}, q_y: {}", q_x, q_y);
	let interpol = (time % 3000) as f32 / 3000.;
	let quaternion = slerp(q_x, q_y, interpol);
	println!("Quaternion : {}\n interpo : {}", quaternion.matrix(), interpol);

	let mut isometry2 =
		isometry1
		* translation(0.3, 0.3, 0.)
		// * center_then_rotate(translation(0., -0.3, 0.), rotation::rx(((time / 10) % 360) as f32))
		* center_then_rotate(translation(0., -0.15, 0.), quat_to_rotation(quaternion))
		;
	// println!("Isometry2 : {}", isometry2);
		// * scaling(0.1, 0.7, 0.1);
	// * rotation::rx(((time / 100) % 360) as f32)
	// * rotation::rx(((time / 10) % 360) as f32)

	let scaling2 = scaling(0.1, 0.3, 0.1);


    // Cube 3 (head)
    let cube3 = create_unit_cuboid([0.2, 0.4, 0.7].into());
    let isometry3 = isometry1
	 * translation(0., 0.6, 0.)
	//  * scaling(0.3, 0.2, 0.1)
	 ;
	let scaling3 = scaling(0.3, 0.2, 0.1);

	// Cube 4 (forearm)
    let cube4 = create_unit_cuboid([0.2, 0.4, 0.2].into());
	let mut isometry4 =
		isometry2
		* translation(0., -0.35, 0.)
		* center_then_rotate(translation(0., -0.2, 0.), quat_to_rotation(quaternion))
		;
	let scaling4 = scaling(0.1, 0.4, 0.1);


    unsafe {
        // draw_cube(model_location, color_location, &cube1, isometry1);
        // draw_cube(model_location, color_location, &cube2, isometry2);
        // draw_cube(model_location, color_location, &cube3, isometry3);
        draw_cube(model_location, color_location, &cube1, isometry1 * scaling1);
        draw_cube(model_location, color_location, &cube2, isometry2 * scaling2);
        draw_cube(model_location, color_location, &cube3, isometry3 * scaling3);
        draw_cube(model_location, color_location, &cube4, isometry4 * scaling4);
    }
}