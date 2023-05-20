use super::NodeId;

/// Enum representing the accessibility of nodes in the network.
///
/// There are two variants:
/// - `ReachableNodes`: This variant contains a vector of node IDs that can be reached from a given node.
/// - `UnreachableNodes`: This variant contains a vector of node IDs that cannot be reached from a given node.
pub enum NodeAccessability {
    ReachableNodes(Vec<NodeId>),
    UnreachableNodes(Vec<NodeId>),
}

impl NodeAccessability {
    /// Returns a reference to the vector of reachable nodes if the `NodeAccessability` instance is the `ReachableNodes` variant.
    ///
    /// # Returns
    ///
    /// `Option<&Vec<NodeId>>` - Some reference to the vector of reachable nodes, or None if the instance is `UnreachableNodes`.
    pub fn reachable_nodes(&self) -> Option<&Vec<NodeId>> {
        match self {
            NodeAccessability::ReachableNodes(nodes) => Some(nodes),
            _ => None,
        }
    }

    /// Returns a reference to the vector of unreachable nodes if the `NodeAccessability` instance is the `UnreachableNodes` variant.
    ///
    /// # Returns
    ///
    /// `Option<&Vec<NodeId>>` - Some reference to the vector of unreachable nodes, or None if the instance is `ReachableNodes`.
    pub fn unreachable_nodes(&self) -> Option<&Vec<NodeId>> {
        match self {
            NodeAccessability::UnreachableNodes(nodes) => Some(nodes),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_accessibility() {
        let reachable_nodes = NodeAccessability::ReachableNodes(vec![1, 2, 3]);
        let unreachable_nodes = NodeAccessability::UnreachableNodes(vec![4, 5, 6]);

        assert_eq!(reachable_nodes.reachable_nodes(), Some(&vec![1, 2, 3]));
        assert_eq!(reachable_nodes.unreachable_nodes(), None);

        assert_eq!(unreachable_nodes.reachable_nodes(), None);
        assert_eq!(unreachable_nodes.unreachable_nodes(), Some(&vec![4, 5, 6]));
    }
}
