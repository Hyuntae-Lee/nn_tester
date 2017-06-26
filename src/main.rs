mod connection;
mod node;
mod network;

use node::{Node, NodeType};
use connection::Connection;

fn main() {
    let mut node_list : Vec<Node> = sample_node_list();
    let mut conn_list : Vec<Connection> = sampel_conn_list();
    let input_list : Vec<(f64, f64)> = sampel_input_list();

    for _ in 0..3 {
        conn_list = network::walk_and_get_new_connection_list(&input_list, 0.7, 0.3,
            &mut conn_list, &mut node_list);
    }
}

fn sampel_input_list() -> Vec<(f64, f64)> {
    vec![
        (1.0, 1.0),
        (1.0, 0.0),
        (0.0, 1.0),
        (0.0, 0.0),
    ]
}

fn sample_node_list() -> Vec<Node> {
    vec![
        Node::new("i1", NodeType::Input_1, 0.0),
        Node::new("i2", NodeType::Input_2, 0.0),
        Node::new("b1", NodeType::InputBias, 1.0),
        Node::new("h1", NodeType::Hidden, 0.0),
        Node::new("h2", NodeType::Hidden, 0.0),
        Node::new("b2", NodeType::HiddenBias, 1.0),
        Node::new("o1", NodeType::Output, 0.0),
    ]
}

fn sampel_conn_list() -> Vec<Connection> {
    vec![
        Connection::new("i1".to_string(), "h1".to_string(), -0.07),
        Connection::new("i1".to_string(), "h2".to_string(),  0.94),
        Connection::new("i2".to_string(), "h1".to_string(),  0.22),
        Connection::new("i2".to_string(), "h2".to_string(),  0.46),
        Connection::new("b1".to_string(), "h1".to_string(), -0.46),
        Connection::new("b1".to_string(), "h2".to_string(),  0.10),
        Connection::new("h1".to_string(), "o1".to_string(), -0.22),
        Connection::new("h2".to_string(), "o1".to_string(),  0.58),
        Connection::new("b2".to_string(), "o1".to_string(),  0.78),
    ]
}
