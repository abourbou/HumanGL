// pub struct Mesh {
//     pub vertices: Vec<Vertex>,
//     pub indices: Vec<u32>,
//     pub VAO: u32,
//     pub color: Vector<f32, 3>,

//     VBO: u32,
//     EBO: u32,
// }
extern crate gl;
use matrix::{Vector, Matrix};


pub fn add_forty_two(x: u32) -> u32 {x + 42}
pub fn sub_forty_two(x: u32) -> u32 {x - 42}
use crate::animation;
use crate::animation::{Keyframe};

type TVector3<T> = Vector<T, 3>;
type TMatrix4<T> = Matrix<T, 4, 4>;

#[derive(Clone, Debug)]
pub struct Node {
    // pub mesh: Mesh,
    pub name : String,
    pub children: Vec<Node>,
    pub keyframes: Vec<Keyframe>,
    // pub isometry : i32,
}

impl Node {
    pub fn new(name: &str, children: Vec<Node>, keyframes: Vec<Keyframe>) -> Node {
        Node {
            name: name.to_string(),
            children,
            keyframes,
        }
    }

    pub fn set_name(&mut self, name: &str, new_name: &str) {
        fn recursion(node: &mut Node, name: &str, new_name: &str) {
            if node.name == name {
                node.name = new_name.to_string();
            } else {
                for it in node.children.iter_mut() {
                    recursion(it, name, new_name);
                }
            }
        }
        recursion(self, name, new_name);
    }

    pub fn exec_function(&mut self, time: u32) {
        fn recursion(node: &mut Node, time: u32) {
            let iso_matrix = animation::animate(node.keyframes.clone(), time);
            for it in node.children.iter_mut() {
                recursion(it, time);
            }
        }
        recursion(self, time);
    }

    pub fn info(& mut self) {
        fn recursion(node: &mut Node, data: String) {
            for child in node.children.iter_mut() {
                let mut total = data.clone();
                total.push_str(":");
                total.push_str(&child.name);
                
                println!("my name is: {}, route is: {:?}", child.name, total);
                recursion(child, total.clone());
            }
        }
        println!("my name is: {}, route is : {}", self.name, self.name);
        recursion(self, self.name.clone());
    }
}

pub fn show() {
    let mut rhand = Node::new("rhand", Vec::new(), animation::walk_rhand());

    for i in 0..1000 {
        rhand.exec_function(i);
    }
    // let mut rhand = Node::new("rhand", Vec::new(), animation::walk_rhand());
    // let mut rarm = Node::new("rarm", Vec::from([rhand]), animation::no_animation());
    // let mut lhand = Node::new("lhand", Vec::new(), animation::no_animation());
    // let mut larm = Node::new("larm", Vec::from([lhand]), animation::no_animation());
    // let mut body = Node::new("body", Vec::from([rarm, larm]), animation::no_animation());
    // println!("before");
    // body.info();
    // println!("after");
    // body.set_name("rarm", "new_rarm");
    // body.info();
    // body.exec_function(42);
}
