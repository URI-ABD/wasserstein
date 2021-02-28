#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Vertex {
    pub index: usize,
    pub coordinates: (usize, usize),
    pub supply: i64,
}

impl Vertex {
    pub fn new(index: usize) -> Self {
        Vertex {
            index,
            coordinates: (0, 0),
            supply: 0,
        }
    }
}

// For now, ground cost across an edge is manhattan distance
pub struct Edge {
    pub left: Vertex,
    pub right: Vertex,
    pub cost: usize,
    pub flow: usize,
}

impl Edge {
    pub fn new(left: Vertex, right: Vertex) -> Self {
        let diff = |l, r| if l < r { r - l } else { l - r };
        let cost = diff(left.coordinates.0, right.coordinates.0)
            + diff(left.coordinates.1, right.coordinates.1);
        Edge {
            left,
            right,
            cost,
            flow: 0,
        }
    }
}

pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub max_capacity: usize,
}

impl Graph {
    pub fn new(num_vertices: usize, max_capacity: usize) -> Result<Self, String> {
        if max_capacity == 0 {
            Err(format!(
                "Need a positive max-capacity on the graph. Got {}",
                max_capacity
            ))
        } else {
            Ok(Graph {
                vertices: (0..num_vertices).map(Vertex::new).collect(),
                edges: Vec::new(),
                max_capacity,
            })
        }
    }

    pub fn add_edge(
        &mut self,
        left: Vertex,
        right: Vertex,
        cost: usize,
        flow: usize,
    ) -> Result<(), String> {
        if left.index >= self.vertices.len() {
            Err(format!(
                "left index {} is out of range {}",
                left.index,
                self.vertices.len()
            ))
        } else if right.index >= self.vertices.len() {
            Err(format!(
                "right index {} is out of range {}",
                right.index,
                self.vertices.len()
            ))
        } else {
            self.edges.push(Edge {
                left,
                right,
                cost,
                flow,
            });
            Ok(())
        }
    }

    pub fn designate_supply(&mut self, vertex: Vertex, supply: i64) {
        self.vertices[vertex.index].supply = supply;
    }

    pub fn designate_demand(&mut self, vertex: Vertex, demand: i64) {
        self.vertices[vertex.index].supply = -demand;
    }

    pub fn mcmf(&mut self) -> Result<usize, String> {
        let num_vertices = self.vertices.len() as i64;
        let num_edges = self.edges.len() as i64;
        let max_capacity = self.max_capacity as i64;
        let vertex_supplies: Vec<i64> = self
            .vertices
            .iter()
            .map(|v| clamp_to_i32(v.supply))
            .collect();
        let edges_left: Vec<i64> = self.edges.iter().map(|e| e.left.index as i64).collect();
        let edges_right: Vec<i64> = self.edges.iter().map(|e| e.right.index as i64).collect();
        let edge_costs: Vec<i64> = self
            .edges
            .iter()
            .map(|e| clamp_to_i32(e.cost as i64))
            .collect();
        let mut edge_flows: Vec<i64> = vec![0; self.edges.len()];
        let total_cost: i64;
        unsafe {
            total_cost = min_cost_max_flow_i64(
                num_vertices,
                num_edges,
                max_capacity,
                vertex_supplies.as_ptr(),
                edges_left.as_ptr(),
                edges_right.as_ptr(),
                edge_costs.as_ptr(),
                edge_flows.as_mut_ptr(),
            );
        }
        for (edge, &flow) in self.edges.iter_mut().zip(edge_flows.iter()) {
            if flow < 0 {
                return Err(format!(
                    "found negative flow {} on edge {} -> {}",
                    flow, edge.left.index, edge.right.index,
                ));
            } else {
                edge.flow = flow as usize
            }
        }
        Ok(total_cost as usize)
    }
}

// Cost, Supply and Demand values must fit in i32
fn clamp_to_i32(value: i64) -> i64 {
    let max = std::i32::MAX as i64;
    let value = std::cmp::min(value, max);
    std::cmp::max(value, -max)
}

#[link(name = "wasserstein")]
extern "C" {
    fn min_cost_max_flow_i64(
        num_vertices: i64,
        num_edges: i64,
        max_capacity: i64,
        vertex_supplies: *const i64,
        edges_left: *const i64,
        edges_right: *const i64,
        edge_costs: *const i64,
        edge_flows: *mut i64,
    ) -> i64;
}
