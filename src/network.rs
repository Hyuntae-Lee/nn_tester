use std::f64::consts::E;
use node::Node;
use connection::Connection;

pub fn walk_for_output(node_list : &mut Vec<Node>, conn_list : &Vec<Connection>, input1 : f64,
    input2 : f64) {
    // 1. output
    for node in node_list {
        node.add_input();
    }
}

pub fn calc_output(node : &Node) -> f64 {
    match node.node_type() {
        Input   => {
            node.sum()
        },
        Hidden  => {
            1.0 / (1.0 + E.powf(-node.sum()))
        },
        Output  => {
            0.0
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
        if conn.left().name() == node.name() {
            concerned_conn_list.push(conn);
        }
    }

    // sum of weight * delta of right
    let mut r_delta_sum = 0.0;
    for conn in concerned_conn_list {
        let right = node_list.iter().find(|n| n.name() == conn.right().name()).unwrap();
        let delta = calc_delta(right, node_list, conn_list, error);
        r_delta_sum += delta * conn.weight();
    }

    // - delta
    let s = calc_output(node);
    let f = s * (1.0 - s);

    f * r_delta_sum
}
