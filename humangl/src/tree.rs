// pub struct Mesh {
//     pub vertices: Vec<Vertex>,
//     pub indices: Vec<u32>,
//     pub VAO: u32,
//     pub color: Vector<f32, 3>,

//     VBO: u32,
//     EBO: u32,
// }

use gl::PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED;

pub fn add_forty_two(x: u32) -> u32 {x + 42}
pub fn sub_forty_two(x: u32) -> u32 {x - 42}

#[derive(Clone, Debug)]
pub struct Node {
    // pub mesh: Mesh,
    pub name : String,
    pub children: Vec<Node>,
    // pub isometry : i32,
    pub animation_function: fn(u32) -> u32,
}

impl Node {
    pub fn new(name: &str, children: Vec<Node>, func: fn(u32) -> u32) -> Node {
        Node {
            name: name.to_string(),
            children,
            animation_function: func,
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

    pub fn exec_function(&mut self, x: u32) {
        fn recursion(node: &mut Node, x: u32) {
            let f = node.animation_function;
            let res = f(x);
            println!("my name is: {}, exectute function: {}", node.name, res);
            for it in node.children.iter_mut() {
                recursion(it, x);
            }
        }
        recursion(self, x);
    }

    pub fn info(&mut self) {
        fn recursion(node: &mut Node, data: String) {
            for it in node.children.iter_mut() {
                let mut total = data.clone();
                total.push_str(":");
                total.push_str(&it.name);
                println!("my name is: {}, route is: {:?}", it.name, total);
                recursion(it, total.clone());
            }
        }
        println!("my name is: {}, route is : {}", self.name, self.name);
        recursion(self, self.name.clone());
    }
}

pub fn show() {
    let mut rhand = Node::new("rhand", Vec::new(), add_forty_two);
    let mut rarm = Node::new("rarm", Vec::from([rhand]), sub_forty_two);
    let mut lhand = Node::new("lhand", Vec::new(), add_forty_two);
    let mut larm = Node::new("larm", Vec::from([lhand]), sub_forty_two);
    let mut body = Node::new("body", Vec::from([rarm, larm]), add_forty_two);
    // println!("before");
    // body.info();
    // println!("after");
    // body.set_name("rarm", "new_rarm");
    // body.info();
    body.exec_function(42);
}
