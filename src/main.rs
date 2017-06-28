extern crate rand;

mod connection;
mod node;
mod lib1;

use rand::Rng;
use node::{Node, NodeType};
use connection::Connection;

fn main() {
    let mut node_list : Vec<Node> = sample_node_list();
    let mut conn_list : Vec<Connection> = sampel_conn_list();
    let trainning_data_list : Vec<(f64, f64)> = sample_training_data();
    let test_data_list : Vec<(f64, f64)> = sample_input_data();

    for _ in 0..200 {
        conn_list = lib1::walk_and_get_new_connection_list(&trainning_data_list, 0.7, 0.3,
            &mut conn_list, &mut node_list);

        println!("error [{:.5}]", calc_error_rate(&mut node_list, &conn_list, &test_data_list));
    }
}

fn calc_error_rate(node_list : &mut Vec<Node>, conn_list : &Vec<Connection>,
    input_list : &Vec<(f64, f64)>) -> f64 {

    let mut sum_of_error = 0.0;
    for &(input_1, input_2) in input_list {
        lib1::reset_node_list(node_list);
        let output = lib1::walk_for_output(node_list, &conn_list, input_1, input_2);
        sum_of_error += lib1::calc_error(input_1, input_2, output);
    }

    sum_of_error / input_list.len() as f64
}

fn sample_training_data() -> Vec<(f64, f64)> {
    vec![(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (0.0, 0.0)]
}

fn sample_input_data() -> Vec<(f64, f64)> {
    let mut data : Vec<(f64, f64)> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let x_f = (rng.gen::<u32>() % 2) as f64;
        let y_f = (rng.gen::<u32>() % 2) as f64;

        data.push((x_f, y_f));
    }

    return data;
}

fn sample_node_list() -> Vec<Node> {
    vec![
        Node::new("i1", NodeType::Input_1, 0.0),
        Node::new("i2", NodeType::Input_2, 0.0),
        Node::new("b1", NodeType::Bias, 1.0),
        Node::new("h1", NodeType::Hidden, 0.0),
        Node::new("h2", NodeType::Hidden, 0.0),
        Node::new("b2", NodeType::Bias, 1.0),
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
