mod connection;
mod node;
mod network;

use node::{Node, NodeType};
use connection::Connection;

fn main() {
    let mut node_list : Vec<Node> = vec![
        Node::new("i1", NodeType::Input_1, 0.0),
        Node::new("i2", NodeType::Input_2, 0.0),
        Node::new("b1", NodeType::InputBias, 1.0),
        Node::new("h1", NodeType::Hidden, 0.0),
        Node::new("h2", NodeType::Hidden, 0.0),
        Node::new("b2", NodeType::HiddenBias, 1.0),
        Node::new("o1", NodeType::Output, 0.0),
    ];

    let mut conn_list : Vec<Connection> = Vec::new();
    conn_list.push(Connection::new("i1".to_string(), "h1".to_string(), -0.07));
    conn_list.push(Connection::new("i1".to_string(), "h2".to_string(),  0.94));
    conn_list.push(Connection::new("i2".to_string(), "h1".to_string(),  0.22));
    conn_list.push(Connection::new("i2".to_string(), "h2".to_string(),  0.46));
    conn_list.push(Connection::new("b1".to_string(), "h1".to_string(), -0.46));
    conn_list.push(Connection::new("b1".to_string(), "h2".to_string(),  0.10));
    conn_list.push(Connection::new("h1".to_string(), "o1".to_string(), -0.22));
    conn_list.push(Connection::new("h2".to_string(), "o1".to_string(),  0.58));
    conn_list.push(Connection::new("b2".to_string(), "o1".to_string(),  0.78));

    network::reset_node_list(&mut node_list);
    network::walk_for_output(&mut node_list, &conn_list, 1.0, 0.0);

    for node in &node_list {
        println!("{}", node);
    }
}
