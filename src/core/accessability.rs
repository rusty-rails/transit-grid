use super::NodeId;

/// Enum `Accessability` representing the accessibility of nodes in a network.
///
/// This enum has two variants:
/// * `ReachableNodes`: This variant holds a vector of `NodeId`s which are reachable from a specific node.
///     This information could be used to limit the search space during network traversal operations.
///
/// * `UnreachableNodes`: This variant holds a vector of `NodeId`s which cannot be reached from a specific node.
///     This information could be used to prevent the search from exploring infeasible paths during network traversal operations.
///
/// # Variants
///
/// * `ReachableNodes(Vec<NodeId>)`: A variant holding a vector of reachable node IDs.
///
/// * `UnreachableNodes(Vec<NodeId>)`: A variant holding a vector of unreachable node IDs.
///
/// # Example
///
/// ```
/// use transit_grid::core::{Accessability, NodeId};
///
/// // Define some node IDs
/// let nodes = vec![1, 2, 3, 4, 5];
///
/// // Define Accessability
/// let access = Accessability::ReachableNodes(nodes);
///
/// match access {
///     Accessability::ReachableNodes(ids) => {
///         // Process reachable nodes
///     }
///     Accessability::UnreachableNodes(ids) => {
///         // Process unreachable nodes
///     }
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Accessability {
    /// A variant holding a vector of reachable node IDs.
    ReachableNodes(Vec<NodeId>),
    /// A variant holding a vector of unreachable node IDs.
    UnreachableNodes(Vec<NodeId>),
}

impl Accessability {
    /// Returns a reference to the vector of reachable nodes if the `NodeAccessability` instance is the `ReachableNodes` variant.
    ///
    /// # Returns
    ///
    /// `Option<&Vec<NodeId>>` - Some reference to the vector of reachable nodes, or None if the instance is `UnreachableNodes`.
    pub fn reachable_nodes(&self) -> Option<&Vec<NodeId>> {
        match self {
            Accessability::ReachableNodes(nodes) => Some(nodes),
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
            Accessability::UnreachableNodes(nodes) => Some(nodes),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_accessibility() {
        let reachable_nodes = Accessability::ReachableNodes(vec![1, 2, 3]);
        let unreachable_nodes = Accessability::UnreachableNodes(vec![4, 5, 6]);

        assert_eq!(reachable_nodes.reachable_nodes(), Some(&vec![1, 2, 3]));
        assert_eq!(reachable_nodes.unreachable_nodes(), None);

        assert_eq!(unreachable_nodes.reachable_nodes(), None);
        assert_eq!(unreachable_nodes.unreachable_nodes(), Some(&vec![4, 5, 6]));
    }
}
