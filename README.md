# TransitGrid
[![Status](https://img.shields.io/badge/status-ideation-blue.svg)](https://github.com/rusty-rails/transit-grid)
[![Github Repo](https://img.shields.io/badge/github-repo-green.svg)](https://github.com/rusty-rails/transit-grid/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/rusty-rails/transit-grid/branch/main/graph/badge.svg?token=TFJ8UT9W1J)](https://codecov.io/gh/rusty-rails/transit-grid)
[![Doc](https://img.shields.io/badge/Docs-online-green.svg)](https://rusty-rails.github.io/transit-grid/transit_grid/)

TransitGrid is a Rust library for simulating and analyzing transportation networks. It's designed to be flexible and general-purpose, capable of simulating trains on tracks, planes in the air, or ships at sea, and more.

## Project Status
TransitGrid is currently in the ideation phase. We're still designing the architecture of the library and determining its exact features. Please feel free to submit suggestions or feedback by opening an issue on our GitHub repository.

## Design Goals
TransitGrid is built around two core data structures: the PhysicalGraph and the TopologyGraph.

- **PhysicalGraph**: This is an undirected graph where each node represents a transit node (a point in the transit network where a vehicle can stop) and each edge represents a transit edge (a path between two transit nodes). The PhysicalGraph uses the UnGraph structure from the petgraph crate to internally represent this data. The PhysicalGraph maintains mappings between NodeId's and NodeIndexes (from petgraph), allowing for efficient conversion between the two.


- **TopologyGraph**: Represents the topological graph of a transit network as a skew-symmetric graph. In this model, let `G = (V, E)` be the directed graph with a function `σ` mapping vertices of `G` to other vertices, satisfying the following properties:
  1. For every vertex `v`, `σ(v)` ≠ `v`.
  2. For every vertex `v`, `σ(σ(v))` = `v`.
  3. For every edge `(u, v)`, `(σ(v), σ(u))` must also be an edge.
  
  In the context of the `TopologyGraph`, for each node `v` in `V`, there are two nodes in `V_t`, denoted as `v_entry` and `v_exit`. For each edge `(u, v)` in `E`, there are two directed edges in `E_t`: one from `u_exit` to `v_entry` and one from `v_exit` to `u_entry`. The mathematical representation of this is:
  * `V_t = {v_entry, v_exit | v ∈ V}`
  * `E_t = {(u_exit, v_entry), (v_exit, u_entry) | (u, v) ∈ E}`
  
  This skew-symmetric model is based on the definition by Goldberg & Karzanov (1996). It is particularly useful for scenarios such as rail switches where the directionality of edges matters. The TopologyGraph uses the StableDiGraph structure from the petgraph crate for internal representation and maintains mappings between custom NodeId's/EdgeId's and petgraph's NodeIndexes/EdgeIndexes.

## Future Work
TransitGrid will include several major components:

- **Graph Operations**: Functions to add and remove nodes and edges from the graphs.

- **Graph Algorithms**: Algorithms for finding the shortest path between two nodes, taking into account the current state of the Topology Graph.

We're excited about the possibilities that TransitGrid can offer, and we're looking forward to seeing what the community can build with it.

## License
TransitGrid is licensed under the [MIT License](LICENSE).
