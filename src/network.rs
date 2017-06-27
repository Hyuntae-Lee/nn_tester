use node::{Node, NodeType};
use connection::Connection;
use layer::Layer;

pub struct Network {
    layer_id_list : Vec<String>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            layer_id_list : Vec::new()
        }
    }

    pub fn add_layer(&mut self, layer : &Layer) {
        self.layer_id_list.push(layer.name().to_string());
    }
}
