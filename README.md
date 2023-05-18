# TransitGrid
[![Status](https://img.shields.io/badge/status-ideation-blue.svg)](https://github.com/yourusername/TransitGrid)

TransitGrid is a Rust library for simulating transportation networks. It's designed to be flexible and general-purpose, able to simulate trains on tracks, planes in the air, or ships at sea.

## Project Status
TransitGrid is currently in the ideation phase. We're still designing the architecture of the library and determining its exact features. Please feel free to submit suggestions or feedback by opening an issue.

## Design Goals
TransitGrid is built around two core data structures: the Transport Graph and the Topology Graph.

- **Transport Graph**: This is a directed graph with nodes representing locations (given in GPS coordinates) and edges representing the paths between these locations (containing information like distance).
- **Topology Graph**: This is another directed graph that represents the topology of the network. It can be used to simulate features like switches in a railway network, where multiple edges meet at a node and not all paths can be traveled simultaneously.

## Future Work
TransitGrid will include several major components:

- **Graph Operations**: Functions to add and remove nodes and edges from the graphs, calculate routes between nodes, and simulate movement along a route.
- **Graph Algorithms**: Algorithms for finding the shortest path between two nodes, taking into account the current state of the Topology Graph.

## License
TransitGrid is licensed under the [MIT License](LICENSE).
