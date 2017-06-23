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
    conn_list.push(Connection::new(&node_list[0], &node_list[3], -0.07));
    conn_list.push(Connection::new(&node_list[0], &node_list[4],  0.94));
    conn_list.push(Connection::new(&node_list[1], &node_list[3],  0.22));
    conn_list.push(Connection::new(&node_list[1], &node_list[4],  0.46));
    conn_list.push(Connection::new(&node_list[2], &node_list[3], -0.46));
    conn_list.push(Connection::new(&node_list[2], &node_list[4],  0.10));
    conn_list.push(Connection::new(&node_list[3], &node_list[6], -0.22));
    conn_list.push(Connection::new(&node_list[4], &node_list[6],  0.58));
    conn_list.push(Connection::new(&node_list[5], &node_list[6],  0.78));

    for node in &node_list {
        println!("{}", node);
    }
}
