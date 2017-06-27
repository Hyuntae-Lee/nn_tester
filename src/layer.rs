use node::{Node, NodeType};

pub enum LayerType {
    Input,
    Hidden,
    Output,
}

pub struct Layer {
    name : String,
    l_type : LayerType,
    node_id_list : Vec<String>,
}

impl Layer {
    pub fn new(name : &str, layer_type : LayerType) -> Layer {
        Layer {
            name : name.to_string(),
            l_type : layer_type,
            node_id_list : Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn add_node(&mut self, node : &Node) {
        self.node_id_list.push(node.name().to_string());
    }
}
