// pub struct Mesh {
//     pub vertices: Vec<Vertex>,
//     pub indices: Vec<u32>,
//     pub VAO: u32,
//     pub color: Vector<f32, 3>,

//     VBO: u32,
//     EBO: u32,
// }

#[derive(Clone, Debug)]
pub struct Node {
    // pub mesh: Mesh,
    pub name : String,
    pub children: Vec<Node>,
    // pub isometry : i32,
    // pub function_change(f32 time),
}

impl Node {
    pub fn new(name: String, children: Vec<Node>) -> Node {
        Node {
            name,
            children,
        }
    }

    pub fn info(&self) {
        fn recursion(node: &mut Node, data: String) {
            for it in node.children.iter() {
                let mut total = data.clone();
                total.push_str(":");
                total.push_str(&it.name);
                
                println!("my name is: {}, route is: {:?}", it.name, total);
                for child in node.children.mut_iter() {
                    recursion(&mut child, total.clone());
                }
            }
        }
        println!("my name is: {}, route is : {}", self.name, self.name);
        recursion(&mut self, self.name.clone());
    }
}

pub fn show() {
    let mut rhand = Node::new("rhand".to_string(), Vec::new());
    let mut rarm = Node::new("rarm".to_string(), Vec::from([rhand]));
    let mut lhand = Node::new("lhand".to_string(), Vec::new());
    let mut larm = Node::new("larm".to_string(), Vec::from([lhand]));
    let mut body = Node::new("body".to_string(), Vec::from([rarm, larm]));

    body.info();
}