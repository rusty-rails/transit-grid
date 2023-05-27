use geo::CoordNum;
use petgraph::{algo::astar, dot::Dot, visit::EdgeRef};

use crate::{
    core::{Accessability, NodeId, TransitEdge},
    graphs::TransitNetwork,
};

pub trait ShortestPath<R, T> {
    fn find_shortest_path(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>>;
}

pub trait ShortestPathWithAccessability<R, T: CoordNum> {
    /// Finds the shortest path between two nodes in the network.
    ///
    /// # Arguments
    ///
    /// * `from` - The ID of the node to start the search from.
    /// * `to` - The ID of the node to end the search at.
    /// * `accessability` - The accessability of nodes in the network.
    ///
    /// # Returns
    ///
    /// * `Option<(f64, Vec<NodeId>)>` - Some tuple containing the length of the shortest path and the path itself, or None if no path was found.
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
            |edge| 1.0,
        )
        .map(|(_, path)| path)
    }
}

impl<R: Copy, T: CoordNum> ShortestPathWithAccessability<R, T> for TransitNetwork<R, T> {
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

        println!("{:?} {:?}", start, goal);

        let path1 = astar(
            &self.topology_graph.graph,
            start.0,
            |finish| finish == goal.0 || finish == goal.1,
            |edge| {
                let from: NodeId = self.topology_graph.index_to_id(edge.source());
                let to: NodeId = self.topology_graph.index_to_id(edge.target());

                let from = self.physical_graph.id_to_index(from);
                let to = self.physical_graph.id_to_index(to);
                let edge = self.physical_graph.graph.find_edge(from, to).unwrap();

                edge_cost(self.physical_graph.graph[edge].clone())
            },
            |_| 0.,
        );
        let path2 = astar(
            &self.topology_graph.graph,
            start.1,
            |finish| finish == goal.0 || finish == goal.1,
            |edge| {
                let from: NodeId = self.topology_graph.index_to_id(edge.source());
                let to: NodeId = self.topology_graph.index_to_id(edge.target());

                let from = self.physical_graph.id_to_index(from);
                let to = self.physical_graph.id_to_index(to);
                let edge = self.physical_graph.graph.find_edge(from, to).unwrap();

                edge_cost(self.physical_graph.graph[edge].clone())
            },
            |_| 0.,
        );

        println!("{:?}", Dot::new(&self.topology_graph.graph));

        println!("{:?}", path1);
        println!("{:?}", path2);

        let best = match (path1, path2) {
            (Some((cost1, path1)), Some((cost2, path2))) => {
                if cost1 < cost2 {
                    Some((cost1, path1))
                } else {
                    Some((cost2, path2))
                }
            }
            (Some((cost, path)), None) => Some((cost, path)),
            (None, Some((cost, path))) => Some((cost, path)),
            (None, None) => None,
        };

        println!("{:?}", best);

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

        // Add nodes to the network
        network.add_node(node0);
        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);
        network.add_node(node4);

        // Define edges
        let edge01 = TransitEdge {
            id: 1,
            from: 0,
            to: 1,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        let edge14 = TransitEdge {
            id: 2,
            from: 1,
            to: 4,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 4.0, y: 4.0}]),
        };

        let edge12 = TransitEdge {
            id: 3,
            from: 1,
            to: 2,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 2.0, y: 2.0}]),
        };

        let edge13 = TransitEdge {
            id: 4,
            from: 1,
            to: 3,
            path: LineString(vec![coord! {x: 1.0, y: 1.0}, coord! {x: 3.0, y: 3.0}]),
        };

        // Add edges to the network
        network.add_edge(edge01);
        network.add_edge(edge14);
        network.add_edge(edge12);
        network.add_edge(edge13);

        // Add edge with accessibility
        let edge40 = TransitEdge {
            id: 5,
            from: 4,
            to: 0,
            path: LineString(vec![coord! {x: 4.0, y: 4.0}, coord! {x: 0.0, y: 0.0}]),
        };

        network.add_edge_with_accessibility(edge40, Accessability::ReachableNodes(vec![2, 3]));

        // Define edge cost function
        let edge_cost = |edge: TransitEdge<f32>| 1.0;

        // Test case 1: No node is unreachable
        let result = network.find_shortest_path_with_accessability(
            0,                                       // from node 0
            4,                                       // to node 4
            Accessability::UnreachableNodes(vec![]), // no unreachable node
            edge_cost,
        );
        assert_eq!(result, Some((3.0, vec![0, 1, 4]))); // Expected shortest path is 0 -> 1 -> 4

        // Test case 2: Node 4 is unreachable
        let result = network.find_shortest_path_with_accessability(
            0,                                        // from node 0
            4,                                        // to node 4
            Accessability::UnreachableNodes(vec![4]), // node 4 is unreachable
            edge_cost,
        );
        assert_eq!(result, None); // Expected result is None as there is no path to node 4

        // Test case 3: Nodes 2 and 3 are unreachable
        let result = network.find_shortest_path_with_accessability(
            0,                                           // from node 0
            4,                                           // to node 4
            Accessability::UnreachableNodes(vec![2, 3]), // nodes 2 and 3 are unreachable
            edge_cost,
        );
        assert_eq!(result, Some((5.0, vec![0, 1, 4]))); // Expected shortest path is 0 -> 1 -> 4 as nodes 2 and 3 are unreachable
    }
}
