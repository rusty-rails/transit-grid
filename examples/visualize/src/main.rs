use std::collections::HashMap;

use eframe::{run_native, App, CreationContext};
use egui::{Context, ScrollArea, Vec2};
use egui_graphs::{Edge, GraphView, Node, SettingsInteraction};
use geo::{coord, LineString};
use petgraph::{
    stable_graph::{NodeIndex, StableDiGraph, StableGraph},
    visit::{EdgeRef, IntoEdgeReferences},
};
use rand::Rng;
use transit_grid::prelude::{
    TopoEdge, TopoNode, TransitEdge, TransitNetwork, TransitNetworkModifier,
    TransitNetworkRepairer, TransitNode,
};

const SIDE_SIZE: f32 = 250.;

pub struct GraphVisualizeApp {
    g: StableGraph<Node<TopoNode>, Edge<()>>,
    selected_nodes: Vec<Node<TopoNode>>,
}

impl GraphVisualizeApp {
    fn new(_: &CreationContext<'_>) -> Self {
        let g = generate_graph();
        Self {
            g,
            selected_nodes: vec![],
        }
    }

    fn sync_graph_with_simulation(&mut self) {
        self.selected_nodes = vec![];

        let g_indices = self.g.node_indices().collect::<Vec<_>>();
        g_indices.iter().for_each(|g_n_idx| {
            let g_n = self.g.node_weight_mut(*g_n_idx).unwrap();

            if g_n.selected {
                self.selected_nodes.push(*g_n);
            }
        });
    }
}

impl App for GraphVisualizeApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel")
            .min_width(250.)
            .show(ctx, |ui| {
                ScrollArea::vertical().max_height(200.).show(ui, |ui| {
                    self.selected_nodes.iter().for_each(|node| {
                        ui.label(format!("{:?}", node.data));
                    });
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                &mut GraphView::new(&mut self.g).with_interactions(&SettingsInteraction {
                    node_drag: true,
                    node_click: true,
                    node_select: true,
                    node_multiselect: true,
                    ..Default::default()
                }),
            );
        });
        self.sync_graph_with_simulation();
    }
}

pub fn convert_graph(
    old_graph: &StableDiGraph<TopoNode, TopoEdge, u32>,
) -> StableGraph<Node<TopoNode>, Edge<()>> {
    let mut new_graph = StableGraph::<Node<TopoNode>, Edge<()>>::default();

    let mut rng = rand::thread_rng();

    let node_mapping: HashMap<NodeIndex, NodeIndex> = old_graph
        .node_indices()
        .map(|old_node_index| {
            let old_node = &old_graph[old_node_index];
            let new_node = Node {
                data: Some(*old_node),
                location: Vec2::new(rng.gen_range(0.0..SIDE_SIZE), rng.gen_range(0.0..SIDE_SIZE)),
                ..Default::default()
            };
            let new_node_index = new_graph.add_node(new_node);
            (old_node_index, new_node_index)
        })
        .collect();

    for edge in old_graph.edge_references() {
        // Convert the old edge into a new edge here.
        let new_edge = Edge::default();
        let source_node = *node_mapping.get(&edge.source()).unwrap();
        let target_node = *node_mapping.get(&edge.target()).unwrap();
        new_graph.add_edge(source_node, target_node, new_edge);
    }

    new_graph
}

fn generate_graph() -> StableGraph<Node<TopoNode>, Edge<()>> {
    let mut network = TransitNetwork::new();

    // Define some nodes
    let node1 = TransitNode {
        id: 1,
        location: coord!(x: 0.0, y: 0.0),
    };

    let node2 = TransitNode {
        id: 2,
        location: coord!(x: 1.0, y: 1.0),
    };

    let node3 = TransitNode {
        id: 3,
        location: coord!(x: 2.0, y: 2.0),
    };

    // Add nodes to the network
    network.add_node(node1);
    network.add_node(node2);
    network.add_node(node3);

    // Define edges
    let edge1 = TransitEdge {
        id: 1,
        source: 1,
        target: 2,
        length: 1.0,
        path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 1.0, y: 1.0}]),
    };

    let edge2 = TransitEdge {
        id: 2,
        source: 2,
        target: 3,
        length: 1.0,
        path: LineString(vec![coord! {x: 0.0, y: 0.0}, coord! {x: 2.0, y: 2.0}]),
    };

    // Add edges to the network
    network.add_edge(edge1);
    network.add_edge(edge2);
    network.repair();

    convert_graph(&network.topology_graph.graph)
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    run_native(
        "graph visualization example",
        native_options,
        Box::new(|cc| Box::new(GraphVisualizeApp::new(cc))),
    )
    .unwrap();
}
