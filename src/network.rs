use std::f64::consts::E;
use node::{Node, NodeType};
use connection::Connection;

pub fn walk_and_get_new_connection_list(
    input_list : &Vec<(f64, f64)>,
    learning_rate : f64,
    momentum : f64,
    conn_list : &mut Vec<Connection>,
    node_list : &mut Vec<Node>) -> Vec<Connection> {

    let mut new_conn_list = conn_list.clone();
    for conn in &mut new_conn_list {
        let weight_delta = calc_weight_delta(&input_list, conn, learning_rate, momentum,
            conn_list, node_list);
        conn.set_prev_wd(weight_delta);
        conn.add_weight(weight_delta);
    }

    new_conn_list
}

pub fn calc_weight_delta(
    input_list : &Vec<(f64, f64)>,
    conn : &mut Connection,
    learning_rate : f64,
    momentum : f64,
    conn_list : &mut Vec<Connection>,
    node_list : &mut Vec<Node>) -> f64 {

    let mut sum_of_i_gradient = 0.0;
    for &(input_1, input_2) in input_list {
        reset_node_list(node_list);
        let output = walk_for_output(node_list, &conn_list, input_1, input_2);
        let error = calc_error(input_1, input_2, output);
        walk_for_delta(node_list, &conn_list, error);

        sum_of_i_gradient += calc_individual_gradient(conn, node_list);
    }

    learning_rate * sum_of_i_gradient + momentum * conn.prev_wd()
}

pub fn calc_error(input_1 : f64, input_2 : f64, output : f64) -> f64 {
    output - answer(input_1, input_2)
}

pub fn answer(x : f64, y : f64) -> f64 {
    // 중심 50, 50 반지름 40인 원의 내외부 여부
    let x_c = x - 40.0;
    let y_c = y - 40.0;

    if x_c * x_c + y_c * y_c > 40.0 * 40.0 {
        0.0
    }
    else {
        1.0
    }
}

pub fn calc_individual_gradient(conn : &Connection, node_list : &Vec<Node>) -> f64 {
    let left = node_list.iter().find(|n| n.name() == conn.l_name()).unwrap();
    let right = node_list.iter().find(|n| n.name() == conn.r_name()).unwrap();
    let left_output = calc_output(left);

    left_output * right.delta()
}

pub fn walk_for_delta(node_list : &mut Vec<Node>, conn_list : &Vec<Connection>, error : f64) {
    let copid_node_list = node_list.clone();
    for node in copid_node_list.iter().rev() {
        let delta = calc_delta(node, node_list, conn_list, error);

        let idx = node_list.iter().position(|x| x.name() == node.name()).unwrap();
        node_list[idx].set_delta(delta);
    }
}

pub fn walk_for_output(node_list : &mut Vec<Node>, conn_list : &Vec<Connection>, input1 : f64,
    input2 : f64) -> f64 {

    node_list[0].add_input(input1);
    node_list[1].add_input(input2);

    for conn in conn_list {
        let l_idx = node_list.iter().position(|x| x.name() == conn.l_name()).unwrap();
        let r_idx = node_list.iter().position(|x| x.name() == conn.r_name()).unwrap();

        let input_from_left_to_right = calc_output(&node_list[l_idx]) * conn.weight();
        node_list[r_idx].add_input(input_from_left_to_right);
    }

    calc_output(node_list.last().unwrap())
}

pub fn reset_node_list(node_list : &mut Vec<Node>) {
    for node in node_list {
        node.reset();
    }
}

pub fn reset_conn_list(conn_list : &mut Vec<Connection>) {
    for conn in conn_list {
        conn.reset_weight_delta_history();
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
    match *node.node_type() {
        NodeType::Output  => calc_delta_for_output(node, error),
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
