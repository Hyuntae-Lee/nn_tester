extern crate rand;

mod connection;
mod node;
mod network;

use node::{Node, NodeType};
use connection::Connection;
use rand::Rng;

fn main() {
    let mut node_list : Vec<Node> = sample_node_list();
    let mut conn_list : Vec<Connection> = sampel_conn_list();
    let trainning_data_list : Vec<(f64, f64)> = sample_training_data();
    let test_data_list : Vec<(f64, f64)> = sample_input_data();

    for _ in 0..10 {
        conn_list = network::walk_and_get_new_connection_list(&trainning_data_list, 0.7, 0.3,
            &mut conn_list, &mut node_list);

        println!("error [{}]", calc_error_rate(&mut node_list, &conn_list, &test_data_list));
    }
}

fn calc_error_rate(node_list : &mut Vec<Node>, conn_list : &Vec<Connection>,
    input_list : &Vec<(f64, f64)>) -> f64 {

    let mut sum_of_error = 0.0;
    for &(input_1, input_2) in input_list {
        network::reset_node_list(node_list);
        let output = network::walk_for_output(node_list, &conn_list, input_1, input_2);
        sum_of_error += network::calc_error(input_1, input_2, output);
    }

    sum_of_error / input_list.len() as f64
}

fn sample_training_data() -> Vec<(f64, f64)> {
    let mut data : Vec<(f64, f64)> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x_f = (rng.gen::<u32>() % 100) as f64 / 100.0;
        let y_f = (rng.gen::<u32>() % 100) as f64 / 100.0;

        data.push((x_f, y_f));
    }

    return data;
}

fn sample_input_data() -> Vec<(f64, f64)> {
    let mut data : Vec<(f64, f64)> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x_f = (rng.gen::<u32>() % 100) as f64 / 100.0;
        let y_f = (rng.gen::<u32>() % 100) as f64 / 100.0;

        data.push((x_f, y_f));
    }

    return data;
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
