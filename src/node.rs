use std::fmt;

#[derive(Clone)]
pub enum NodeType {
    Input_1,
    Input_2,
    InputBias,
    Hidden,
    HiddenBias,
    Output,
}

pub struct Node {
    name : String,
    node_type : NodeType,
    input_list : Vec<f64>,
    initial_value : f64,
    delta : f64
}

impl Node {
    pub fn new(name : &str, node_type : NodeType, initial_value : f64) -> Node {
        Node {
            name : name.to_string(),
            node_type: node_type,
            input_list : Vec::new(),
            initial_value : initial_value,
            delta : 0.0,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn sum(&self) -> f64 {
        self.input_list.iter().fold(self.initial_value, |acc, &x| acc + x)
    }
    pub fn node_type(&self) -> &NodeType {
        &self.node_type
    }
    pub fn delta(&self) -> f64 {
        self.delta
    }
    pub fn reset(&mut self) {
        self.input_list.clear();
        self.delta = 0.0;
    }
    pub fn add_input(&mut self, value : f64) {
        self.input_list.push(value);
    }
    pub fn set_delta(&mut self, value : f64) {
        self.delta = value;
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node {
            name : self.name().to_string(),
            node_type: self.node_type().clone(),
            input_list : self.input_list.clone(),
            initial_value : self.initial_value,
            delta : self.delta,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node [{}: sum[{}], delta[{}]]", self.name(), self.sum(), self.delta())
    }
}
