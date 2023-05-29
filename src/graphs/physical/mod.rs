use std::collections::HashMap;

use crate::core::{NodeId, TransitEdge, TransitNode};
use geo::{Coord, CoordNum, EuclideanDistance};
use petgraph::{
    graph::EdgeIndex,
    graph::{NodeIndex, UnGraph},
};

/// Represents the physical layout of the transit network.
///
/// `PhysicalGraph` is an undirected graph where each node represents a transit node (a point in the transit network where a vehicle can stop) and each edge represents a transit edge (a path between two transit nodes).
/// The `PhysicalGraph` uses the `UnGraph` structure from the `petgraph` crate to internally represent this data. The `PhysicalGraph` maintains mappings between `NodeId`s and `NodeIndex`es (from `petgraph`), allowing for efficient conversion between the two.
///
/// # Examples
///
/// Creating a new `PhysicalGraph` and adding a `TransitNode`:
/// ```
/// use transit_grid::core::TransitNode;
/// use transit_grid::prelude::PhysicalGraph;
/// use geo::{coord, Coord};
///
/// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
/// let node = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
/// graph.add_transit_node(node);
/// ```
///
/// Adding a `TransitEdge` to the `PhysicalGraph`:
/// ```
/// use transit_grid::core::{TransitNode, TransitEdge};
/// use transit_grid::prelude::PhysicalGraph;
/// use geo::{coord, Coord, LineString};
///
/// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
/// let node1 = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
/// let node2 = TransitNode { id: 2, location: coord! { x:1.0, y:1.0 } };
/// let node1_id = graph.add_transit_node(node1);
/// let node2_id = graph.add_transit_node(node2);
/// let edge = TransitEdge {
///     id: 1,
///     from: 1,
///     to: 2,
///     path: LineString(vec![coord! { x:0.0, y:0.0 }, coord! { x:1.0, y:1.0 }]),
/// };
/// graph.add_transit_edge(edge);
/// ```
pub struct PhysicalGraph<R, T: CoordNum> {
    /// Underlying undirected graph.
    pub graph: UnGraph<TransitNode<R>, TransitEdge<T>, u32>,

    /// Mapping of NodeId to petgraph's NodeIndex.
    id_to_index: HashMap<NodeId, NodeIndex>,

    /// Mapping of petgraph's NodeIndex to NodeId.
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

    /// Converts a `NodeIndex` to a `NodeId`.
    ///
    /// This method provides a way to map from the petgraph's `NodeIndex` to
    /// the `NodeId` used in the `TransitNode`.
    ///
    /// # Arguments
    ///
    /// * `index` - The `NodeIndex` to be converted.
    ///
    /// # Returns
    ///
    /// * `NodeId` - The corresponding `NodeId` of the provided `NodeIndex`.
    ///
    /// # Example
    ///
    /// ```
    /// use transit_grid::prelude::PhysicalGraph;
    /// use transit_grid::core::TransitNode;
    /// use geo::{coord, Coord};
    ///
    /// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
    /// let node = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
    /// let node_index = graph.add_transit_node(node);
    /// let node_id = graph.index_to_id(node_index);
    /// assert_eq!(node_id, 1);
    /// ```
    pub fn index_to_id(&self, index: NodeIndex) -> NodeId {
        self.index_to_id[&index]
    }

    /// Converts a `NodeId` to a `NodeIndex`.
    ///
    /// This method provides a way to map from a `NodeId` used in the `TransitNode` to
    /// the petgraph's `NodeIndex`.
    ///
    /// # Arguments
    ///
    /// * `id` - The `NodeId` to be converted.
    ///
    /// # Returns
    ///
    /// * `NodeIndex` - The corresponding `NodeIndex` of the provided `NodeId`.
    ///
    /// # Example
    ///
    /// ```
    /// use transit_grid::prelude::PhysicalGraph;
    /// use transit_grid::core::TransitNode;
    /// use geo::{coord, Coord};
    ///
    /// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
    /// let node = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
    /// let node_index = graph.add_transit_node(node);
    /// let queried_index = graph.id_to_index(1);
    /// assert_eq!(node_index, queried_index);
    /// ```
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
    pub fn add_transit_node(&mut self, node: TransitNode<R>) -> NodeIndex {
        let index = self.graph.add_node(node);
        self.id_to_index.insert(node.id, index);
        self.index_to_id.insert(index, node.id);
        index
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
    ///     from: 1,
    ///     to: 2,
    ///     path: LineString(vec![coord! { x:0.0, y:0.0 }, coord! { x:1.0, y:1.0 }]),
    /// };
    ///
    /// graph.add_transit_edge(edge);
    /// ```
    pub fn add_transit_edge(&mut self, edge: TransitEdge<T>) -> EdgeIndex {
        let from = self.id_to_index(edge.from);
        let to = self.id_to_index(edge.to);
        self.graph.add_edge(from, to, edge)
    }

    /// Returns a reference to the `TransitEdge` connecting the two nodes specified by `node1` and `node2`.
    ///
    /// # Arguments
    ///
    /// * `node1` - The `NodeId` of the first node.
    /// * `node2` - The `NodeId` of the second node.
    ///
    /// # Returns
    ///
    /// A reference to the `TransitEdge` connecting `node1` and `node2`. This function will panic if there is no edge between the nodes.
    ///
    /// # Panics
    ///
    /// This function will panic in the following cases:
    ///
    /// * If `node1` or `node2` are not valid node IDs in the graph.
    /// * If there is no edge between `node1` and `node2`.
    pub fn get_transit_edge(&self, node1: NodeId, node2: NodeId) -> &TransitEdge<T> {
        let node1_index = self.id_to_index(node1);
        let node2_index = self.id_to_index(node2);
        let edge_index = self.graph.find_edge(node1_index, node2_index).unwrap();
        self.graph.edge_weight(edge_index).unwrap()
    }

    /// Repairs a physical edge in the `PhysicalGraph` based on its nodes' locations.
    ///
    /// # Arguments
    ///
    /// * `edge` - The `TransitEdge` to be repaired.
    ///
    /// # Example
    ///
    /// ```
    /// use transit_grid::prelude::PhysicalGraph;
    /// use transit_grid::core::{TransitNode, TransitEdge};
    /// use geo::{coord, Coord, LineString};
    ///
    /// let mut graph: PhysicalGraph<Coord, f64> = PhysicalGraph::new();
    /// let node1 = TransitNode { id: 1, location: coord! { x:0.0, y:0.0 } };
    /// let node2 = TransitNode { id: 2, location: coord! { x:1.0, y:1.0 } };
    ///
    /// let node1_id = graph.add_transit_node(node1);
    /// let node2_id = graph.add_transit_node(node2);
    ///
    /// let mut edge = TransitEdge {
    ///     id: 1,
    ///     from: 1,
    ///     to: 2,
    ///     path: LineString(vec![coord! { x:1.0, y:1.0 }, coord! { x:0.0, y:0.0 }]),  // Note that the direction is initially reversed
    /// };
    ///
    /// graph.add_transit_edge(edge.clone());
    /// graph.repair_edge(1, 2);
    /// let edge = graph.get_transit_edge(1, 2);
    /// assert_eq!(
    ///    edge.path,
    ///    LineString(vec![Coord { x: 0.0, y: 0.0 }, Coord { x: 1.0, y: 1.0 }])
    /// );
    ///
    /// // After repair, the edge path should be from 0,0 to 1,1
    /// //assert_eq!(edge.path.0.first().unwrap(), &coord! { x:0.0, y:0.0 });
    /// //assert_eq!(edge.path.0.last().unwrap(), &coord! { x:1.0, y:1.0 });
    /// ```
    pub fn repair_edge(&mut self, node1: NodeId, node2: NodeId)
    where
        R: EuclideanDistance<T, Coord<T>>,
    {
        let node1_index = self.id_to_index(node1);
        let node2_index = self.id_to_index(node2);
        let from_node_location = {
            let from_node: &TransitNode<R> = self.graph.node_weight(node1_index).unwrap();
            from_node.location.clone() // Clone the location to use it later
        };

        let edge_index = self.graph.find_edge(node1_index, node2_index).unwrap();
        let edge = self.graph.edge_weight_mut(edge_index).unwrap();

        let first_point = edge.path.0.first().unwrap();
        let last_point = edge.path.0.last().unwrap();

        let dist_to_first = from_node_location.euclidean_distance(first_point);
        let dist_to_last = from_node_location.euclidean_distance(last_point);

        if dist_to_first > dist_to_last {
            edge.path.0.reverse();
        }
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
    use geo::{coord, Coord, LineString};

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

    #[test]
    fn test_index_to_id() {
        let mut graph = PhysicalGraph::<Coord, f64>::new();

        let node1 = TransitNode {
            id: 1,
            location: coord! { x:0.0, y:0.0 },
        };

        let node2 = TransitNode {
            id: 2,
            location: coord! { x:1.0, y:1.0 },
        };

        let node1_index = graph.add_transit_node(node1);
        let node2_index = graph.add_transit_node(node2);

        let node1_id = graph.index_to_id(node1_index);
        let node2_id = graph.index_to_id(node2_index);

        assert_eq!(node1_id, 1);
        assert_eq!(node2_id, 2);
    }

    #[test]
    fn test_id_to_index() {
        let mut graph = PhysicalGraph::<Coord, f64>::new();

        let node1 = TransitNode {
            id: 1,
            location: coord! { x:0.0, y:0.0 },
        };

        let node2 = TransitNode {
            id: 2,
            location: coord! { x:1.0, y:1.0 },
        };

        let node1_index = graph.add_transit_node(node1);
        let node2_index = graph.add_transit_node(node2);

        let queried_node1_index = graph.id_to_index(1);
        let queried_node2_index = graph.id_to_index(2);

        assert_eq!(node1_index, queried_node1_index);
        assert_eq!(node2_index, queried_node2_index);
    }

    #[test]
    fn test_default() {
        let graph: PhysicalGraph<u32, f64> = PhysicalGraph::default();

        assert_eq!(graph.graph.node_count(), 0);
        assert_eq!(graph.graph.edge_count(), 0);
    }

    #[test]
    fn test_repair_physical() {
        let mut graph = PhysicalGraph::<Coord, f64>::new();
        let node1 = TransitNode {
            id: 1,
            location: Coord { x: 0.0, y: 0.0 },
        };
        let node2 = TransitNode {
            id: 2,
            location: Coord { x: 1.0, y: 1.0 },
        };

        graph.add_transit_node(node1);
        graph.add_transit_node(node2);

        let edge = TransitEdge {
            id: 1,
            from: 1,
            to: 2,
            path: LineString(vec![Coord { x: 1.0, y: 1.0 }, Coord { x: 0.0, y: 0.0 }]),
        };

        graph.add_transit_edge(edge.clone());

        graph.repair_edge(1, 2);

        let edge = graph.get_transit_edge(1, 2);

        assert_eq!(
            edge.path,
            LineString(vec![Coord { x: 0.0, y: 0.0 }, Coord { x: 1.0, y: 1.0 }])
        );
    }
}
