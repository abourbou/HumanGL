
use matrix::{Vector};
use super::mesh::Mesh;

pub fn create_cuboid(length : f32, width : f32, height : f32, color : Vector::<f32, 3>) -> Mesh {

    // set up vertex data (and buffer(s)) and configure vertex attributes
    let vertices = [
         1. * length,  1. * width,  1. * height, // front top right
         1. * length, -1. * width,  1. * height, // front bottom right
        -1. * length, -1. * width,  1. * height, // front bottom left
        -1. * length,  1. * width,  1. * height, // front top left
        -1. * length,  1. * width, -1. * height, // back top left
        -1. * length, -1. * width, -1. * height, // back bottom left
         1. * length, -1. * width, -1. * height, // back bottom right
         1. * length,  1. * width, -1. * height // back top right
    ];
    let vertices = (Vector::from(vertices) * 0.5).arr;
    
    let indices = [
        0, 1, 3,  //  1 Triangle (front) 
        1, 2, 3,  //  2 Triangle (front)
        4, 5, 6,  //  3 Triangle (back) 
        4, 6, 7,  //  4 Triangle (back)
        2, 3, 4,  //  5 Triangle (left)
        2, 4, 5,  //  6 Triangle (left)
        0, 1, 6,  //  7 Triangle (right)
        0, 6, 7,  //  8 Triangle (right)
        1, 2, 5,  //  9 Triangle (bottom)
        1, 5, 6,  // 10 Triangle (bottom)
        0, 3, 4,  // 11 Triangle (top)
        0, 4, 7,  // 12 Triangle (top)   
    ];

    Mesh::new(vertices.to_vec(), indices.to_vec(), color)
}