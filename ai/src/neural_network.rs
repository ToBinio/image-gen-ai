use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::input::Input;
use crate::output::Output;

#[derive(Deserialize, Serialize)]
pub struct NeuralNetwork {
    nodes: Vec<Vec<Node>>,
}

impl NeuralNetwork {
    pub fn new(input: &Input, sizes: Vec<usize>, output: &Output) -> NeuralNetwork {
        let mut rng = thread_rng();

        let mut nodes = vec![];

        let mut sizes = sizes.clone();
        sizes.insert(0, input.get_size());
        sizes.push(output.get_size());

        let mut prev_size = 0;

        for node_size in sizes {
            let mut new_nodes = vec![];

            for _ in 0..node_size {
                let mut connects = vec![];

                for _ in 0..prev_size {
                    connects.push(rng.gen_range(-1.0..1.0));
                }

                let new_node = Node {
                    value: 0.0,
                    connections: connects,
                };

                new_nodes.push(new_node);
            }

            prev_size = node_size;
            nodes.push(new_nodes)
        }

        NeuralNetwork { nodes }
    }

    pub fn set_input(&mut self, input: &Input) {
        let first = self.nodes.first_mut().unwrap();

        for (index, value) in input.get().iter().enumerate() {
            first[index].value = *value;
        }
    }

    pub fn get_output(&self, output: &mut Output) {
        let values = self
            .nodes
            .last()
            .unwrap()
            .iter()
            .map(|node| node.value)
            .collect();

        output.set(values);
    }

    pub fn compute(&mut self) {
        let max = self.nodes.len();

        for index in 1..max {
            let prev_values: Vec<f32> = self
                .nodes
                .get(index - 1)
                .unwrap()
                .iter()
                .map(|node| node.value)
                .collect();
            let current_nodes = self.nodes.get_mut(index).unwrap();

            for node in current_nodes {
                let mut sum = 0.0;

                for (index, strength) in node.connections.iter().enumerate() {
                    sum += *prev_values.get(index).unwrap() * strength
                }

                //todo better activation function
                sum = sum.max(-1.0).min(1.0);

                node.value = sum;
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Node {
    value: f32,
    connections: Vec<f32>,
}
