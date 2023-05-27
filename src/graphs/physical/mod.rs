use std::collections::HashMap;

use crate::core::{EdgeId, NodeId, TransitEdge, TransitNode};
use geo::CoordNum;
use petgraph::graph::{NodeIndex, UnGraph};

/// `PhysicalGraph` represents the physical layout of the transit network.
///
/// It is an undirected graph where each node represents a transit node
/// (a point in the transit network where a vehicle can stop) and each edge
/// represents a transit edge (a path between two transit nodes).
///
/// The graph is implemented using `petgraph`'s `Csr` structure.
pub struct PhysicalGraph<R, T: CoordNum> {
    pub graph: UnGraph<TransitNode<R>, TransitEdge<T>, u32>,
    id_to_index: HashMap<NodeId, NodeIndex>,
    index_to_id: HashMap<NodeIndex, NodeId>,
}

impl<R: Copy, T: CoordNum> PhysicalGraph<R, T> {
    /// Creates a new, empty `PhysicalGraph`.
    pub fn new() -> Self {
        PhysicalGraph {
            graph: UnGraph::<TransitNode<R>, TransitEdge<T>, u32>::new_undirected(),
            id_to_index: HashMap::new(),
            index_to_id: HashMap::new(),
        }
    }

    pub fn index_to_id(&self, index: NodeIndex) -> NodeId {
        self.index_to_id[&index]
    }

    pub fn id_to_index(&self, id: NodeId) -> NodeIndex {
        self.id_to_index[&id]
    }

    /// Adds a `TransitNode` to the `PhysicalGraph`.
    ///
    /// # Example
    /// ```
    /// use transit_grid::prelude::PhysicalGraph;
    /// use transit_grid::core::TransitNode;
    /// use geo::{coord, Coord};
    ///
    /// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
    /// let node = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
    /// graph.add_transit_node(node);
    /// ```
    pub fn add_transit_node(&mut self, node: TransitNode<R>) -> NodeId {
        let index = self.graph.add_node(node);
        self.id_to_index.insert(node.id, index);
        self.index_to_id.insert(index, node.id);
        index.index().try_into().unwrap()
    }

    /// Adds a `TransitEdge` to the `PhysicalGraph`.
    ///
    /// # Example
    /// ```
    /// use transit_grid::prelude::PhysicalGraph;
    /// use transit_grid::core::{TransitNode, TransitEdge};
    /// use geo::{coord, Coord, LineString};
    /// use petgraph::csr::IndexType;
    ///
    /// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
    /// let node1 = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
    /// let node2 = TransitNode { id: 2, location: coord! { x:1.0, y:1.0 } };
    ///
    /// let node1_id = graph.add_transit_node(node1);
    /// let node2_id = graph.add_transit_node(node2);
    ///
    /// let edge = TransitEdge {
    ///     id: 1,
    ///     from: node1_id.index().try_into().unwrap(),
    ///     to: node2_id.index().try_into().unwrap(),
    ///     path: LineString(vec![coord! { x:0.0, y:0.0 }, coord! { x:1.0, y:1.0 }]),
    /// };
    ///
    /// graph.add_transit_edge(edge);
    /// ```
    pub fn add_transit_edge(&mut self, edge: TransitEdge<T>) -> EdgeId {
        let from = self.id_to_index(edge.from);
        let to = self.id_to_index(edge.to);
        self.graph
            .add_edge(from, to, edge)
            .index()
            .try_into()
            .unwrap()
    }
}

impl<R: Copy, T: CoordNum> Default for PhysicalGraph<R, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{coord, LineString};

    #[test]
    fn test_graph() {
        let mut graph = PhysicalGraph::new();

        let node1 = TransitNode {
            id: 1,
            location: coord! { x:0.0, y:0.0 },
        };

        let node2 = TransitNode {
            id: 2,
            location: coord! { x:1.0, y:1.0 },
        };

        let _node1_id = graph.add_transit_node(node1);
        let _node2_id = graph.add_transit_node(node2);

        let edge = TransitEdge {
            id: 1,
            from: 1,
            to: 2,
            path: LineString(vec![coord! { x:0.0, y:0.0 }, coord! { x:1.0, y:1.0 }]),
        };

        let _ = graph.add_transit_edge(edge);

        assert_eq!(graph.graph.node_count(), 2);
        assert_eq!(graph.graph.edge_count(), 1);
    }
}
