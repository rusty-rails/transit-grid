#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
/// TransitNet is a Rust library for representing, manipulating, and performing computations on transit networks.
/// It provides a set of core data structures and algorithms that are commonly used in transit network analysis.
///
/// The library is organized into several modules:
///
/// * `algorithms` - This module provides a collection of algorithms for performing various computations on transit networks,
///                  such as finding the shortest path, calculating the centrality of a node, etc.
///
/// * `core` - This module defines the core data structures used throughout the library,
///            including various types of nodes, edges, and graphs.
///
/// * `graphs` - This module defines several types of graphs that can represent a transit network at different levels of detail,
///              including the physical graph, the topological graph, and the transit network.
///
/// * `operations` - This module provides operations for manipulating transit networks,
///                  such as adding or removing nodes or edges, merging networks, etc.
///
/// The `prelude` module re-exports the most commonly used items from the `core`, `graphs`, and `operations` modules,
/// providing a convenient way to import many items at once.
///
/// The top level of the library also includes some general attributes and macros,
/// as well as an inclusion of the README file as module documentation.
pub mod algorithms;
pub mod core;
pub mod graphs;
pub mod operations;

/// The `prelude` module re-exports the most commonly used items from the `core`, `graphs`, and `operations` modules,
pub mod prelude {
    pub use crate::core::*;
    pub use crate::graphs::*;
    pub use crate::operations::*;
}
