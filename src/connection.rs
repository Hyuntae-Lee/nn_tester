use std::fmt;

pub struct Connection {
    left_name : String,
    right_name : String,
    weight : f64,
    prev_weight_delta : f64,
}

impl Connection {
    pub fn new(left_name : String, right_name : String, weight : f64) -> Connection {
        Connection {
            left_name : left_name,
            right_name : right_name,
            weight : weight,
            prev_weight_delta : 0.0,
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
    pub fn prev_wd(&self) -> f64 {
        self.prev_weight_delta
    }
    pub fn set_prev_wd(&mut self, value : f64) {
        self.prev_weight_delta = value
    }
    pub fn reset_weight_delta_history(&mut self) {
        self.prev_weight_delta = 0.0
    }
}

impl Clone for Connection {
    fn clone(&self) -> Connection {
        Connection {
            left_name : self.l_name().to_string(),
            right_name : self.r_name().to_string(),
            weight : self.weight(),
            prev_weight_delta : self.prev_wd(),
        }
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Connection [{}-{}], w[{}]]", self.l_name(), self.r_name(), self.weight())
    }
}
