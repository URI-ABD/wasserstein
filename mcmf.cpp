#include "lemon/network_simplex.h"
#include "lemon/smart_graph.h"

#include <stdint.h>

#include <vector>
#include <cassert>

using namespace lemon;

template<typename T>
T min_cost_max_flow(
        T num_vertices,
        T num_edges,
        T max_capacity,
        const T* vertex_supplies,
        const T* edges_left,
        const T* edges_right,
        const T* edge_costs,
        T* edge_flows
        ) {
    // Check that the vertices are all numbered properly
    for (T i = 0; i < num_edges; i++) {
        assert(0 <= edges_left[i] && edges_left[i] < num_vertices);
        assert(0 <= edges_right[i] && edges_right[i] < num_vertices);
    }
    // Create a DirectedGraph
    DIGRAPH_TYPEDEFS(SmartDigraph);
    SmartDigraph graph;
    // Populate vertices
    std::vector<Node> vertices;
    for (T i = 0; i < num_vertices; i++) {
        vertices.push_back(graph.addNode());
    }
    // populate edges
    std::vector<Arc> edges;
    for (T i = 0; i < num_edges; i++) {
        edges.push_back(graph.addArc(vertices[edges_left[i]], vertices[edges_right[i]]));
    }
    // specify supply/demand vertices
    SmartDigraph::NodeMap<T> supplies(graph);
    for (T i = 0; i < num_vertices; i++) {
        supplies[vertices[i]] = vertex_supplies[i];
    }
    // set capacities to max_capacity
    SmartDigraph::ArcMap<T> capacities(graph);
    for (T i = 0; i < num_edges; i++) {
        capacities[edges[i]] = max_capacity;
    }
    // specify cost of flow through each edge
    SmartDigraph::ArcMap<T> costs(graph);
    for (T i = 0; i < num_edges; i++) {
        costs[edges[i]] = edge_costs[i];
    }
    // Set up network simplex object and provide the supply, capacity and cost maps.
    NetworkSimplex<SmartDigraph> network_simplex(graph);
    network_simplex.supplyMap(supplies).upperMap(capacities).costMap(costs);
    // Run the Network Simplex algorithm
    network_simplex.run();
    // retrieve calculated flow over each edge and populate in array
    for (T i = 0; i < num_edges; i++) {
        edge_flows[i] = network_simplex.flow(edges[i]);
    }
    // retrieve and return the total cost of the flow
    return network_simplex.totalCost<T>();
}

extern "C" int64_t min_cost_max_flow_i64(
    int64_t num_vertices,
    int64_t num_edges,
    int64_t max_capacity,
    const int64_t* vertex_supplies,
    const int64_t* edges_left,
    const int64_t* edges_right,
    const int64_t* edge_costs,
    int64_t* edge_flows
) {
    return min_cost_max_flow(
        num_vertices,
        num_edges,
        max_capacity,
        vertex_supplies,
        edges_left,
        edges_right,
        edge_costs,
        edge_flows
    );
}
