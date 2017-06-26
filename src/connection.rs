use std::fmt;
use node::Node;

pub struct Connection {
    left_name : String,
    right_name : String,
    weight : f64,
}

impl Connection {
    pub fn new(left_name : String, right_name : String, weight : f64) -> Connection {
        Connection {
            left_name : left_name,
            right_name : right_name,
            weight : weight,
        }
    }
    pub fn l_name(&self) -> &str {
        &self.left_name
    }
    pub fn r_name(&self) -> &str {
        &self.right_name
    }
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection [{}-{}], w[{}]]", self.l_name(), self.r_name(), self.weight())
    }
}
