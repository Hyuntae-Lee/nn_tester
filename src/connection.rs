use std::fmt;
use node::Node;

pub struct Connection<'a> {
    left : &'a Node,
    right : &'a Node,
    weight : f64,
}

impl<'a> Connection<'a> {
    pub fn new(left : &'a Node, right : &'a Node, weight : f64) -> Connection<'a> {
        Connection {
            left : left,
            right : right,
            weight : weight,
        }
    }
    pub fn left(&self) -> &Node {
        self.left
    }
    pub fn right(&self) -> &Node {
        self.right
    }
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl<'a> fmt::Display for Connection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection [{}-{}], w[{}]]", self.left(), self.right(), self.weight())
    }
}
