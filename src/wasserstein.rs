use crate::graph::{Graph, Vertex, Edge};

pub fn wasserstein_1d(left: Vec<u64>, right: Vec<u64>) -> Result<(usize, Graph), String> {
    let total_supply: u64 = left.iter().sum();
    let total_demand: u64 = right.iter().sum();

    if total_supply > std::i32::MAX as u64 {
        Err("total supply must fit in i32".to_string())
    } else if total_supply != total_demand {
        Err("Wasserstein is not implemented for cases where supply != demand.".to_string())
    } else {
        let left_vertices: Vec<Vertex> = left
        .iter()
        .enumerate()
        .map(|(i, &v)| Vertex {
            index: i,
            supply: v as i64,
        })
        .collect();
        let mut right_vertices: Vec<Vertex> = right
            .iter()
            .enumerate()
            .map(|(i, &v)| Vertex {
                index: i + left.len(),
                supply: -(v as i64),
            })
            .collect();

        let mut edges: Vec<Edge> = Vec::new();
        for (i, &l) in left_vertices.iter().enumerate() {
            for (j, &r) in right_vertices.iter().enumerate() {
                edges.push(Edge {
                    left: l,
                    right: r,
                    cost: if j > i { j - i } else { i - j },
                    flow: 0,
                });
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
        Ok((total_cost, graph))
    }
}