use std::f64::consts::E;
use node::{Node, NodeType};
use connection::Connection;

pub fn walk_for_delta(node_list : &mut Vec<Node>, conn_list : &Vec<Connection>, input1 : f64,
    input2 : f64) {

}

pub fn walk_for_output(node_list : &mut Vec<Node>, conn_list : &Vec<Connection>, input1 : f64,
    input2 : f64) {

    node_list[0].add_input(input1);
    node_list[1].add_input(input2);

    for conn in conn_list {
        let l_idx = node_list.iter().position(|x| x.name() == conn.l_name()).unwrap();
        let r_idx = node_list.iter().position(|x| x.name() == conn.r_name()).unwrap();

        let input_from_left_to_right = calc_output(&node_list[l_idx]) * conn.weight();
        node_list[r_idx].add_input(input_from_left_to_right);
    }
}

pub fn reset_node_list(node_list : &mut Vec<Node>) {
    for node in node_list {
        node.reset();
    }
}

pub fn calc_output(node : &Node) -> f64 {
    match *node.node_type() {
        NodeType::Hidden => {
            1.0 / (1.0 + E.powf(-node.sum()))
        },
        NodeType::Output => {
            1.0 / (1.0 + E.powf(-node.sum()))
        },
        _ => {
            node.sum()
        },
    }
}

pub fn calc_delta(node : &Node, node_list : &Vec<Node>, conn_list : &Vec<Connection>,
    error : f64) -> f64 {
    match node.node_type() {
        Output  => calc_delta_for_output(node, error),
        _   => calc_delta_for_interier(node, node_list, conn_list, error),
    }
}

fn calc_delta_for_output(node : &Node, error : f64) -> f64 {
    let s = calc_output(node);
    let f = s * (1.0 - s);

    -error * f
}

fn calc_delta_for_interier(node : &Node, node_list : &Vec<Node>,
    conn_list : &Vec<Connection>, error : f64) -> f64 {

    // collect connections
    let mut concerned_conn_list : Vec<&Connection> = Vec::new();
    for conn in conn_list {
        if conn.l_name() == node.name() {
            concerned_conn_list.push(conn);
        }
    }

    // sum of weight * delta of right
    let mut r_delta_sum = 0.0;
    for conn in concerned_conn_list {
        let right = node_list.iter().find(|n| n.name() == conn.r_name()).unwrap();
        let delta = calc_delta(right, node_list, conn_list, error);
        r_delta_sum += delta * conn.weight();
    }

    // - delta
    let s = calc_output(node);
    let f = s * (1.0 - s);

    f * r_delta_sum
}
