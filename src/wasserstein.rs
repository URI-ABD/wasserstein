use ndarray::Array2;

use crate::graph::{Edge, Graph, Vertex};

pub fn wasserstein_1d(left: Vec<u64>, right: Vec<u64>) -> Result<usize, String> {
    let total_supply: u64 = left.iter().sum();
    let total_demand: u64 = right.iter().sum();

    if left.len() != right.len() {
        Err("Wasserstein is not implemented between histograms with different shapes".to_string())
    } else if total_supply > std::i32::MAX as u64 {
        Err("total supply must fit in i32".to_string())
    } else if total_supply != total_demand {
        Err("Wasserstein is not implemented for cases where supply != demand.".to_string())
    } else {
        let left_vertices: Vec<Vertex> = left
            .iter()
            .enumerate()
            .map(|(i, &v)| Vertex {
                index: i,
                coordinates: (i, 0),
                supply: v as i64,
            })
            .collect();
        let mut right_vertices: Vec<Vertex> = right
            .iter()
            .enumerate()
            .map(|(i, &v)| Vertex {
                index: i + left.len(),
                coordinates: (i, 0),
                supply: -(v as i64),
            })
            .collect();

        let mut edges: Vec<Edge> = Vec::new();
        for &l in left_vertices.iter() {
            for &r in right_vertices.iter() {
                edges.push(Edge::new(l, r));
            }
        }

        let mut vertices = left_vertices;
        vertices.append(&mut right_vertices);
        let mut graph = Graph {
            vertices,
            edges,
            max_capacity: total_supply as usize,
        };
        let total_cost = graph.mcmf()?;
        Ok(total_cost)
    }
}

pub fn wasserstein_2d(left: Array2<u64>, right: Array2<u64>) -> Result<usize, String> {
    let total_supply: u64 = left.sum();
    let total_demand: u64 = right.sum();

    if left.shape() != right.shape() {
        Err("Wasserstein is not implemented between histograms with different shapes".to_string())
    } else if total_supply > std::i32::MAX as u64 {
        Err("total supply must fit in i32".to_string())
    } else if total_supply != total_demand {
        Err("Wasserstein is not implemented for cases where supply != demand.".to_string())
    } else {
        let (n, m) = (left.shape()[0], left.shape()[1]);
        let num_bins = n * m;

        let mut vertices = vec![];
        for l in 0..3 {
            for i in 0..n {
                for j in 0..m {
                    let index = l * num_bins + m * i + j;
                    let supply = if l == 0 {
                        left[[i, j]] as i64
                    } else if l == 2 {
                        -(right[[i, j]] as i64)
                    } else {
                        0
                    };
                    let vertex = Vertex {
                        index,
                        supply,
                        coordinates: (i, j),
                    };
                    vertices.push(vertex);
                }
            }
        }

        let mut edges = vec![];
        for i in 0..n {
            for j in 0..m {
                let left_index = m * i + j;
                let right_indices: Vec<usize> = (0..m).map(|k| num_bins + k + i * m).collect();
                for &right_index in right_indices.iter() {
                    edges.push(Edge::new(vertices[left_index], vertices[right_index]));
                }

                let left_index = num_bins + left_index;
                let right_indices: Vec<usize> = (0..n).map(|k| 2 * num_bins + k * m + j).collect();
                for &right_index in right_indices.iter() {
                    edges.push(Edge::new(vertices[left_index], vertices[right_index]));
                }
            }
        }

        let mut graph = Graph {
            vertices,
            edges,
            max_capacity: total_supply as usize,
        };
        let total_cost = graph.mcmf()?;
        Ok(total_cost)
    }
}
