use crate::prelude::{TopoEdge, TopoNode};
use geo::CoordNum;
use petgraph::Directed;

use petgraph::stable_graph::{EdgeIndex, NodeIndex, NodeIndices, StableDiGraph};
use petgraph::visit::{
    Data, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges, IntoNeighbors, IntoNodeIdentifiers,
    NodeIndexable, Visitable,
};

use super::TransitNetwork;

/// The `UndirectedGraph` struct represents an undirected graph for a transit network.
///
/// # Types
///
/// * `'a` - Lifetime associated with the transit network reference.
/// * `R` - Copyable type associated with the TransitNode.
/// * `T` - CoordNum type associated with the TransitEdge.
///
/// # Fields
///
/// * `network` - A reference to the transit network.
#[derive(Copy, Clone)]
pub struct DirectedGraph<'a, R: std::marker::Copy, T: CoordNum> {
    network: &'a TransitNetwork<R, T>,
}

/// This implementation block contains functions associated with the `UndirectedGraph` struct.
impl<'a, R: std::marker::Copy, T: CoordNum> DirectedGraph<'a, R, T> {
    /// Constructs a new `UndirectedGraph`.
    ///
    /// # Arguments
    ///
    /// * `network` - A reference to the transit network.
    ///
    /// # Returns
    ///
    /// A new `UndirectedGraph`.
    pub fn new(network: &'a TransitNetwork<R, T>) -> Self {
        DirectedGraph { network }
    }
}

// Then you can implement the required traits on your TopologicalGraph:

impl<'a, R: std::marker::Copy, T: CoordNum> GraphBase for DirectedGraph<'a, R, T> {
    type NodeId = NodeIndex;
    type EdgeId = EdgeIndex;
}

impl<'a, R: std::marker::Copy, T: CoordNum> GraphRef for DirectedGraph<'a, R, T> {}

impl<'a, R: std::marker::Copy, T: CoordNum> Data for DirectedGraph<'a, R, T> {
    type NodeWeight = TopoNode;
    type EdgeWeight = TopoEdge;
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoNodeIdentifiers for DirectedGraph<'a, R, T> {
    type NodeIdentifiers = std::iter::Map<
        NodeIndices<'a, TopoNode>,
        fn(petgraph::stable_graph::NodeIndex<u32>) -> Self::NodeId,
    >;

    fn node_identifiers(self) -> Self::NodeIdentifiers {
        self.network.topology_graph.graph.node_indices().map(|i| i)
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoEdgeReferences for DirectedGraph<'a, R, T> {
    type EdgeRef = petgraph::stable_graph::EdgeReference<'a, TopoEdge, u32>;
    type EdgeReferences = petgraph::stable_graph::EdgeReferences<'a, TopoEdge, u32>;

    fn edge_references(self) -> Self::EdgeReferences {
        (self.network.topology_graph.graph).edge_references()
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> Visitable for DirectedGraph<'a, R, T> {
    type Map = <StableDiGraph<TopoNode, TopoEdge> as Visitable>::Map;

    fn visit_map(&self) -> Self::Map {
        self.network.topology_graph.graph.visit_map()
    }

    fn reset_map(&self, map: &mut Self::Map) {
        self.network.topology_graph.graph.reset_map(map)
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> NodeIndexable for DirectedGraph<'a, R, T> {
    fn node_bound(&self) -> usize {
        self.network.topology_graph.graph.node_bound()
    }

    fn to_index(&self, a: NodeIndex<u32>) -> usize {
        self.network.topology_graph.graph.to_index(a)
    }

    fn from_index(&self, i: usize) -> NodeIndex<u32> {
        self.network.topology_graph.graph.from_index(i)
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoEdges for DirectedGraph<'a, R, T> {
    type Edges = petgraph::stable_graph::Edges<'a, TopoEdge, Directed>;

    fn edges(self, a: Self::NodeId) -> Self::Edges {
        self.network.topology_graph.graph.edges(a)
    }
}

impl<'a, R: std::marker::Copy, T: CoordNum> IntoNeighbors for DirectedGraph<'a, R, T> {
    type Neighbors = petgraph::stable_graph::Neighbors<'a, TopoEdge>;

    fn neighbors(self, n: NodeIndex<u32>) -> Self::Neighbors {
        self.network.topology_graph.graph.neighbors(n)
    }
}

#[cfg(test)]
mod tests {
    use geo::{coord, point, LineString};
    use petgraph::dot::Dot;

    use crate::{
        core::{Accessability, TransitEdge, TransitNode},
        prelude::TransitNetworkModifier,
    };

    use super::*;

    #[test]
    #[ignore = "fails because not implemented"]
    fn test_dijkstra() {
        let mut network = TransitNetwork::new();

        let node1 = TransitNode {
            id: 1,
            location: point!(x: 0.0, y: 0.0),
        };

        let node2 = TransitNode {
            id: 2,
            location: point!(x: 1.0, y: 1.0),
        };

        let node3 = TransitNode {
            id: 3,
            location: point!(x: 2.0, y: 2.0),
        };

        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);

        let edge1 = TransitEdge {
            id: 1,
            from: 0,
            to: 1,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
        };

        let edge2 = TransitEdge {
            id: 2,
            from: 1,
            to: 2,
            path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 2.0, y: 2.0}]),
        };

        network.add_edge(edge1);
        let accessability = Accessability::ReachableNodes(vec![1]);
        network.add_edge_with_accessibility(edge2, accessability);

        let undirected_graph = DirectedGraph::new(&network);

        let target_node = NodeIndex::new(2);
        let path = petgraph::algo::dijkstra(
            &undirected_graph,
            NodeIndex::new(1),
            Some(target_node),
            |_e| 1,
        );
        println!("{:?}", Dot::new(&network.topology_graph.graph));
        println!("{:?}", path);
        assert_eq!(path[&target_node], 2);
    }
}
