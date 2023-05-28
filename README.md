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

- **TopologyGraph**: The design of the TopologyGraph is still under discussion and will be updated in future iterations.

## Future Work
TransitGrid will include several major components:

- **Graph Operations**: Functions to add and remove nodes and edges from the graphs.

- **Graph Algorithms**: Algorithms for finding the shortest path between two nodes, taking into account the current state of the Topology Graph.

We're excited about the possibilities that TransitGrid can offer, and we're looking forward to seeing what the community can build with it.

## License
TransitGrid is licensed under the [MIT License](LICENSE).
