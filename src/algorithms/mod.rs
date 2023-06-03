//! Module providing traits and implementations for shortest path computations
//! on a transit network. The two main traits defined are `ShortestPath` and `ShortestPathWithAccessability`.
//!
//! `ShortestPath` trait is used for computing shortest path between two nodes in a network.
//!
//! `ShortestPathWithAccessability` extends `ShortestPath` and takes into account the accessibility
//! of nodes while computing the shortest path. This is useful in scenarios where certain nodes
//! in the network may not be accessible and need to be avoided.
//!
//! The module also provides implementations of these traits for `TransitNetwork` struct.
//! It uses A* algorithm from `petgraph` crate for shortest path computation.

use std::{cmp::Ordering, f64::INFINITY};

use geo::CoordNum;
use petgraph::{algo::astar, visit::EdgeRef};

use crate::{
    core::{Accessability, NodeId, TransitEdge},
    graphs::TransitNetwork,
};

pub mod edge_length;

/// `ShortestPath` trait provides functionality to compute shortest path in a network.
///
/// It is generic over two types `R` and `T`. `R` represents some properties of the network (like weights or capacities),
/// and `T` represents the numerical type for calculations (like distances or costs).
///
/// Implementations of `ShortestPath` provide a method `find_shortest_path()` that takes the starting and destination nodes,
/// and returns the shortest path from the start to the destination node as a vector of node IDs.

pub trait ShortestPath<R, T> {
    /// Finds the shortest path from the start node to the destination node.
    ///
    /// # Arguments
    ///
    /// * `from` - The ID of the starting node.
    /// * `to` - The ID of the destination node.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<NodeId>>` - If a path exists, returns the path as a vector of Node IDs, where the first element
    ///   is the start node and the last is the destination node. If no path exists, returns None.
    ///
    fn find_shortest_path(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>>;
}

/// This trait provides methods for finding the shortest path between two nodes in a graph
/// with consideration for the accessibility of nodes.
///
/// # Type Parameters
///
/// * `R`: The type that represents the route or connection between nodes.
/// * `T`: The type that represents the coordinate number used in the nodes. This should implement the `CoordNum` trait.
pub trait ShortestPathWithAccessability<R, T: CoordNum> {
    /// Calculates the cost of traversing from one node to another, considering the accessibility of the destination node.
    ///
    /// # Arguments
    ///
    /// * `from` - The ID of the node from where the traversal starts.
    /// * `to` - The ID of the node to which the traversal ends.
    /// * `accessability` - A reference to the accessibility information for nodes in the network.
    /// * `edge_cost` - A mutable reference to a function that calculates the cost of traversing an edge.
    ///
    /// # Returns
    ///
    /// * `f64` - The cost of the traversal from `from` node to `to` node.
    ///
    fn calc_edge_cost<F>(
        &self,
        from: NodeId,
        to: NodeId,
        accessability: &Accessability,
        edge_cost: &mut F,
    ) -> f64
    where
        F: FnMut(TransitEdge<T>) -> f64;

    /// Finds the shortest path between two nodes considering the accessibility of nodes.
    ///
    /// # Arguments
    ///
    /// * `from` - The ID of the starting node.
    /// * `to` - The ID of the destination node.
    /// * `accessability` - The accessibility information for nodes in the network.
    /// * `edge_cost` - A function to calculate the cost of traversing an edge.
    ///
    /// # Returns
    ///
    /// * `Option<(f64, Vec<NodeId>)>` - A tuple containing the length of the shortest path and the nodes in the path, or None if no path exists.
    ///
    fn find_shortest_path_with_accessability<F>(
        &self,
        from: NodeId,
        to: NodeId,
        accessability: Accessability,
        edge_cost: F,
    ) -> Option<(f64, Vec<NodeId>)>
    where
        F: FnMut(TransitEdge<T>) -> f64;
}

impl<R: Copy, T: CoordNum> ShortestPath<R, T> for TransitNetwork<R, T> {
    fn find_shortest_path(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>> {
        self.find_shortest_path_with_accessability(
            from,
            to,
            Accessability::UnreachableNodes(vec![]),
            |_edge| 1.0,
        )
        .map(|(_, path)| path)
    }
}

impl<R: Copy, T: CoordNum> ShortestPathWithAccessability<R, T> for TransitNetwork<R, T> {
    // Function to calculate edge cost
    fn calc_edge_cost<F>(
        &self,
        from: NodeId,
        to: NodeId,
        accessability: &Accessability,
        edge_cost: &mut F,
    ) -> f64
    where
        F: FnMut(TransitEdge<T>) -> f64,
    {
        if let Accessability::UnreachableNodes(reachable_nodes) = accessability {
            if reachable_nodes.contains(&to) {
                return INFINITY;
            }
        }
        let from = self.physical_graph.id_to_index(from);
        let to = self.physical_graph.id_to_index(to);
        if let (Some(from), Some(to)) = (from, to) {
            let edge = self.physical_graph.graph.find_edge(*from, *to).unwrap();
            edge_cost(self.physical_graph.graph[edge].clone())
        } else {
            INFINITY
        }
    }

    fn find_shortest_path_with_accessability<F>(
        &self,
        from: NodeId,
        to: NodeId,
        accessability: Accessability,
        mut edge_cost: F,
    ) -> Option<(f64, Vec<NodeId>)>
    where
        F: FnMut(TransitEdge<T>) -> f64,
    {
        let start = self.topology_graph.id_to_index(from);
        let goal = self.topology_graph.id_to_index(to);

        let path1 = astar(
            &self.topology_graph.graph,
            start.0,
            |finish| finish == goal.0 || finish == goal.1,
            |edge| {
                self.calc_edge_cost(
                    self.topology_graph.index_to_id(edge.source()),
                    self.topology_graph.index_to_id(edge.target()),
                    &accessability,
                    &mut edge_cost,
                )
            },
            |_| 0.,
        );
        let path2 = astar(
            &self.topology_graph.graph,
            start.1,
            |finish| finish == goal.0 || finish == goal.1,
            |edge| {
                self.calc_edge_cost(
                    self.topology_graph.index_to_id(edge.source()),
                    self.topology_graph.index_to_id(edge.target()),
                    &accessability,
                    &mut edge_cost,
                )
            },
            |_| 1.,
        );

        let best = path1
            .into_iter()
            .chain(path2)
            .min_by(|(cost1, _), (cost2, _)| cost1.partial_cmp(cost2).unwrap_or(Ordering::Equal));

        best.map(|(cost, path)| {
            let path = path
                .into_iter()
                .map(|index| self.topology_graph.index_to_id(index))
                .collect();

            (cost, path)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::TransitNode, operations::TransitNetworkModifier};

    use super::*;
    use geo::{coord, point, LineString};

    #[test]
    fn test_shortest_path() {
        // Create a new TransitNetwork
        let mut network = TransitNetwork::new();

        // Define some nodes
        let node0 = TransitNode {
            id: 0,
            location: point!(x: 0.0, y: 0.0),
        };

        let node1 = TransitNode {
            id: 1,
            location: point!(x: 1.0, y: 1.0),
        };

        let node2 = TransitNode {
            id: 2,
            location: point!(x: 2.0, y: 2.0),
        };

        let node3 = TransitNode {
            id: 3,
            location: point!(x: 3.0, y: 3.0),
        };

        let node4 = TransitNode {
            id: 4,
            location: point!(x: 4.0, y: 4.0),
        };

        // Add nodes to the network
        network.add_node(node0);
        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);
        network.add_node(node4);

        // Define edges
        let edge01 = TransitEdge {
            id: 01,
            source: 0,
            target: 1,
            length: 1.0,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };
        let edge12 = TransitEdge {
            id: 12,
            source: 1,
            target: 2,
            length: 1.0,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 2.0, y: 2.0}]),
        };
        let edge23 = TransitEdge {
            id: 23,
            source: 2,
            target: 3,
            length: 1.0,
            path: LineString(vec![coord! {x: 2.0, y: 2.0}, coord! {x: 3.0, y: 3.0}]),
        };
        let edge34 = TransitEdge {
            id: 34,
            source: 3,
            target: 4,
            length: 1.0,
            path: LineString(vec![coord! {x: 3.0, y: 3.0}, coord! {x: 4.0, y: 4.0}]),
        };

        // Add edges to the network
        network.add_edge(edge01);
        network.add_edge_with_accessibility(edge12, Accessability::ReachableNodes(vec![0]));
        network.add_edge_with_accessibility(edge23, Accessability::ReachableNodes(vec![1]));
        network.add_edge_with_accessibility(edge34, Accessability::ReachableNodes(vec![2]));

        let result = network.find_shortest_path(0, 4);
        assert_eq!(result, Some(vec![0, 1, 2, 3, 4])); // Expected shortest path is 0 -> 1 -> 2 -> 3 -> 4
    }

    #[test]
    fn test_shortest_path_with_accessability() {
        // Create a new TransitNetwork

        let mut network = TransitNetwork::new();

        // Define some nodes
        let node0 = TransitNode {
            id: 0,
            location: point!(x: 0.0, y: 0.0),
        };

        let node1 = TransitNode {
            id: 1,
            location: point!(x: 1.0, y: 1.0),
        };

        let node2 = TransitNode {
            id: 2,
            location: point!(x: 2.0, y: 2.0),
        };

        let node3 = TransitNode {
            id: 3,
            location: point!(x: 3.0, y: 3.0),
        };

        let node4 = TransitNode {
            id: 4,
            location: point!(x: 4.0, y: 4.0),
        };

        let node5 = TransitNode {
            id: 5,
            location: point!(x: 5.0, y: 5.0),
        };

        // Add nodes to the network
        network.add_node(node0);
        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);
        network.add_node(node4);
        network.add_node(node5);

        // Define edges
        let edge01 = TransitEdge {
            id: 01,
            source: 0,
            target: 1,
            length: 1.0,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };
        let edge02 = TransitEdge {
            id: 02,
            source: 0,
            target: 2,
            length: 1.0,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 2.0, y: 2.0}]),
        };
        let edge13 = TransitEdge {
            id: 13,
            source: 1,
            target: 3,
            length: 1.0,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 3.0, y: 3.0}]),
        };
        let edge14 = TransitEdge {
            id: 14,
            source: 1,
            target: 4,
            length: 1.0,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 4.0, y: 4.0}]),
        };
        let edge25 = TransitEdge {
            id: 25,
            source: 2,
            target: 5,
            length: 1.0,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 4.0, y: 4.0}]),
        };
        let edge45 = TransitEdge {
            id: 45,
            source: 5,
            target: 4,
            length: 1.0,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 4.0, y: 4.0}]),
        };

        // Add edges to the network
        network.add_edge(edge01);
        network.add_edge_with_accessibility(edge14, Accessability::ReachableNodes(vec![0]));
        network.add_edge_with_accessibility(edge02, Accessability::ReachableNodes(vec![0]));
        network.add_edge_with_accessibility(edge13, Accessability::ReachableNodes(vec![0]));
        network.add_edge_with_accessibility(edge25, Accessability::ReachableNodes(vec![1]));
        network.add_edge_with_accessibility(edge45, Accessability::ReachableNodes(vec![2, 1]));

        // Define edge cost function
        let edge_cost = |_edge: TransitEdge<f32>| 1.0;

        // Test case 1: No node is unreachable
        let result = network.find_shortest_path_with_accessability(
            0,                                       // from node 0
            4,                                       // to node 4
            Accessability::UnreachableNodes(vec![]), // no unreachable node
            edge_cost,
        );
        assert_eq!(result, Some((2.0, vec![0, 1, 4]))); // Expected shortest path is 0 -> 1 -> 4

        // Test case 2: Node 4 is unreachable
        let result = network.find_shortest_path_with_accessability(
            3, // from node 2
            4, // to node 4
            Accessability::UnreachableNodes(vec![]),
            edge_cost,
        );
        assert_eq!(result, None); // Expected result is None as there is no path to node 4

        let result = network.find_shortest_path_with_accessability(
            0,                                        // from node 0
            4,                                        // to node 4
            Accessability::UnreachableNodes(vec![1]), // no unreachable node
            edge_cost,
        );
        assert_eq!(result, Some((3.0, vec![0, 2, 5, 4]))); // Expected shortest path is 0 -> 1 -> 4
    }
}
