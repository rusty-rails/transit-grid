//! This module contains the definition and implementation of various types of graphs
//! used to represent and manipulate transit networks.
//!
//! The `graphs` module provides three submodules:
//!
//! * `physical` - This module contains the `PhysicalGraph` structure and its associated functionality.
//!                The `PhysicalGraph` represents the physical layout of the transit network, including routes and nodes.
//!
//! * `topology` - This module contains the `TopologyGraph` and its associated structures (`TopoNode`, `TopoEdge`),
//!                as well as their related functionality.
//!                The `TopologyGraph` represents the topological layout of the transit network,
//!                abstracting away the details of the physical layout.
//!
//! * `transit_network` - This module contains the `TransitNetwork` structure,
//!                       which provides a higher-level interface to the physical and topological graphs.
//!                       It combines the functionalities of the physical and topological graphs
//!                       and offers a unified and simplified interface for interacting with the transit network.
//!
//! By using the `graphs` module, one can easily create, modify, and interact with various representations of transit networks.
mod physical;
mod topology;
mod transit_network;

pub use physical::PhysicalGraph;
pub use topology::*;
pub use transit_network::TransitNetwork;
