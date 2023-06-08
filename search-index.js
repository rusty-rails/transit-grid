var searchIndex = JSON.parse('{\
"transit_grid":{"doc":"TransitGrid","t":"AAAAAIIKAKKIKEGGGINDDNLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLMMLLLLLLLMMLMLLLMKLMKLLLLLLLLLLLLLLDDDDIDLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLKLLLLMLLLLLLLLLLLLLLLLLLLLLLMLLLLMMLLMMLLLLLLLLLLLLLLLLLLMLMLLLLKLLMLLLLLLLMLLLLLLLLLLLLLLLIIKKKKK","n":["algorithms","core","graphs","operations","prelude","ShortestPath","ShortestPathWithAccessability","calc_edge_cost","edge_length","find_shortest_path","find_shortest_path_with_accessability","EdgeLength","length","Accessability","EdgeId","IdType","NodeId","PathCoordinates","ReachableNodes","TransitEdge","TransitNode","UnreachableNodes","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","cmp","default","deserialize","deserialize","eq","eq","eq","equivalent","equivalent","equivalent","euclidean_length","fmt","fmt","fmt","from","from","from","hash","haversine_length","id","id","into","into","into","is_within","is_within","is_within","length","length","location","partial_cmp","path","reachable_nodes","serialize","serialize","source","source_coordinate","source_coordinate","target","target_coordinate","target_coordinate","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","unreachable_nodes","PhysicalGraph","TopoEdge","TopoNode","TopologyGraph","TopologyGraphRepairer","TransitNetwork","add_edge","add_edge","add_edge_with_accessibility","add_edge_with_accessibility","add_node","add_node","add_transit_edge","add_transit_node","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","calc_edge_cost","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","cmp","cross_link_dual_edge","cross_link_dual_edge","default","default","default","edge_id","edge_is_in_neighbors_direction","eq","eq","eq","equivalent","equivalent","find_edge_indices","find_node_index_with_edges","find_shortest_path","find_shortest_path_with_accessability","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","get_edge_by_id","get_other_toponode","get_transit_edge","get_transit_edge_by_id","graph","graph","has_incoming","hash","id","id","id_to_index","id_to_index","index_to_id","index_to_id","into","into","into","into","into","is_within","is_within","is_within","is_within","is_within","new","new","new","no_edges_in_direction","node_id","partial_cmp","physical_graph","repair","repair_edge","repair_edge","repair_edge","reverse_dual_edge","reverse_dual_edge","reverse_edge","to","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","topology_graph","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","TransitNetworkModifier","TransitNetworkRepairer","add_edge","add_edge_with_accessibility","add_node","repair","repair_edge"],"q":[[0,"transit_grid"],[5,"transit_grid::algorithms"],[11,"transit_grid::algorithms::edge_length"],[13,"transit_grid::core"],[88,"transit_grid::graphs"],[215,"transit_grid::operations"]],"d":["TransitNet is a Rust library for representing, …","This module provides basic structures for representing a …","This module contains the definition and implementation of …","This module provides abstractions and implementations for …","The <code>prelude</code> module re-exports the most commonly used items …","<code>ShortestPath</code> trait provides functionality to compute …","This trait provides methods for finding the shortest path …","Calculates the cost of traversing from one node to …","Edge length functions for <code>TransitEdge</code>.","Finds the shortest path from the start node to the …","Finds the shortest path between two nodes considering the …","EdgeLength trait provides the length of an element. It is …","Returns the Euclidean length of the element.","Re-export of the <code>Accessability</code> enum from the <code>accessability</code> …","Type alias for an edge identifier.","Type alias for an identifier.","Type alias for a node identifier.","Trait providing a way to get the coordinates of the source …","A variant holding a vector of reachable node IDs.","Structure representing a connection between two <code>TransitNode</code>…","Structure representing a node in the transit network.","A variant holding a vector of unreachable node IDs.","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","A unique identifier for the edge.","A unique identifier for the node.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","The length of the edge.","The location of the node, represented by a generic type <code>T</code>.","","The path of the edge, represented as a <code>LineString</code>.","Returns a reference to the vector of reachable nodes if …","","","The identifier of the node where the edge starts.","Returns the source coordinate of the path.","","The identifier of the node where the edge ends.","Returns the target coordinate of the path.","","","","","","","","","","","","","","Returns a reference to the vector of unreachable nodes if …","Represents the physical layout of the transit network.","Represents an edge in the <code>TopologyGraph</code>.","Represents a node in the <code>TopologyGraph</code>.","Represents the topological graph of a transit network as a …","<code>TopologyGraphRepairer</code> provides functionality to manipulate …","Represents a transit network as a graph with transit nodes …","Adds a <code>TopoEdge</code> to the topological graph.","Adds a <code>TransitEdge</code> to the physical graph of the network.","Adds an edge with a certain accessibility into the graph.","","Adds a Node with a <code>NodeId</code> to the topological graph. This …","Adds a <code>TransitNode</code> to the physical graph of the network.","Adds a <code>TransitEdge</code> to the <code>PhysicalGraph</code>.","Adds a <code>TransitNode</code> to the <code>PhysicalGraph</code>.","","","","","","","","","","","","","","","","","","","","","","","Cross-link the dual edge defined by the two given node IDs.","","","","","The custom identifier of the edge.","Checks if an edge is in the same direction as its …","","","","","","Returns the indices of edges between two nodes in all …","Returns the <code>NodeIndex</code> of the <code>NodeId</code> that does not have any …","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","The custom identifier of the node where the edge …","Returns a reference to the <code>TransitNode</code> with the given ID.","Returns the <code>NodeIndex</code> of the other <code>TopoNode</code> for a given …","Returns a reference to the <code>TransitEdge</code> connecting the two …","Returns a reference to the <code>TransitEdge</code> with the specified …","Underlying undirected graph.","the inner graph","Checks if a node has an incoming edge in the topological …","","The index of the node in the petgraph.","The index of the edge in the petgraph.","Converts a <code>NodeId</code> to a <code>NodeIndex</code>.","Returns the <code>NodeIndex</code> corresponding to a given <code>NodeId</code>.","Converts a <code>NodeIndex</code> to a <code>NodeId</code>.","Returns the <code>NodeId</code> corresponding to a given <code>NodeIndex</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","Creates a new, empty <code>PhysicalGraph</code>.","Creates a new instance of <code>TopologyGraph</code>.","Constructs a new <code>TransitNetwork</code> with an empty <code>PhysicalGraph</code>…","Checks if there are no edges in the specified direction …","The custom identifier of the node.","","The physical graph representing the transit network.","","Repairs a physical edge in the <code>PhysicalGraph</code> based on its …","Repairs the direction of edges in a graph if they are …","","Reverse the dual edge defined by the two given node IDs.","","Reverse the direction of a given edge.","The custom identifier of the node where the edge ends.","","","","","","","","The topological graph representing the transit network.","","","","","","","","","","","","","","","","Trait providing methods for modifying a transit network.","A trait for repairing transit networks, particularly for …","Adds a <code>TransitEdge</code> to the network.","Adds a <code>TransitEdge</code> to the network with a given …","Adds a <code>TransitNode</code> to the network.","Repairs the entire network.","Repairs the edge between two nodes in the network."],"i":[0,0,0,0,0,0,0,43,0,44,43,0,45,0,0,0,0,0,2,0,0,2,9,2,10,9,2,10,9,2,10,9,2,10,10,9,9,10,9,2,10,9,2,10,9,9,2,10,9,2,10,10,9,9,10,9,2,10,9,2,10,9,9,10,10,9,2,9,10,9,46,9,9,46,9,9,2,10,9,2,10,9,2,10,9,2,10,2,0,0,0,0,0,0,31,34,31,34,31,34,35,35,35,31,38,39,34,35,31,38,39,34,34,35,31,38,39,34,35,31,38,39,34,38,47,31,35,31,34,39,31,38,39,34,38,39,31,31,34,34,35,31,38,38,39,39,34,35,31,38,39,34,39,34,31,35,35,35,31,31,38,38,39,35,31,35,31,35,31,38,39,34,35,31,38,39,34,35,31,34,31,38,38,34,34,35,31,34,47,31,31,39,35,31,38,39,34,38,39,34,35,31,38,39,34,35,31,38,39,34,35,31,38,39,34,0,0,48,48,48,49,49],"f":[0,0,0,0,0,0,0,[[1,1,2,3],4],0,[[1,1],[[6,[[5,[1]]]]]],[[1,1,2,3],6],0,[[]],0,0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[[9,[[0,[7,8]]]]],[[9,[[0,[7,8]]]]]],[2,2],[[[10,[7]]],[[10,[7]]]],[[]],[[]],[[]],[[[10,[11]],[10,[11]]],12],[[],[[9,[8]]]],[13,[[15,[[9,[[0,[14,8]]]]]]]],[13,[[15,[[10,[14]]]]]],[[[9,[[0,[16,8]]]],[9,[[0,[16,8]]]]],17],[[2,2],17],[[[10,[16]],[10,[16]]],17],[[],17],[[],17],[[],17],[[[9,[[0,[18,19]]]]],[[0,[18,19]]]],[[[9,[[0,[20,8]]]],21],22],[[2,21],22],[[[10,[20]],21],22],[[]],[[]],[[]],[[[10,[23]],24]],[[[9,[[0,[18,25]]]]],[[0,[18,25]]]],0,0,[[]],[[]],[[]],[[],17],[[],17],[[],17],[[[9,[[0,[18,25,19]]]]],[[0,[18,25,19]]]],0,0,[[[10,[26]],[10,[26]]],[[6,[12]]]],0,[2,[[6,[[5,[1]]]]]],[[[9,[[0,[27,8]]]],28],15],[[[10,[27]],28],15],0,[[],29],[[[9,[8]]],[[29,[8]]]],0,[[],29],[[[9,[8]]],[[29,[8]]]],[[]],[[]],[[]],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],30],[[],30],[[],30],[2,[[6,[[5,[1]]]]]],0,0,0,0,0,0,[[31,32,1,1]],[[[34,[33,8]],[9,[8]]]],[[31,32,1,1,2]],[[[34,[33,8]],[9,[8]],2]],[[31,1]],[[[34,[33,8]],[10,[33]]],1],[[[35,[33,8]],[9,[8]]],36],[[[35,[33,8]],[10,[33]]],37],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[34,[33,8]],1,1,2,3],4],[[[35,[7,[0,[7,8]]]]],[[35,[7,[0,[7,8]]]]]],[31,31],[38,38],[39,39],[[[34,[[0,[7,33]],[0,[7,8]]]]],[[34,[[0,[7,33]],[0,[7,8]]]]]],[[]],[[]],[[]],[[]],[[]],[[38,38],12],[[1,1]],[[31,1,1]],[[],[[35,[33,8]]]],[[],31],[[],[[34,[33,8]]]],0,[[31,36],17],[[38,38],17],[[39,39],17],[[[34,[33,8]],[34,[33,8]]],17],[[],17],[[],17],[[31,1,1],6],[[31,1,[5,[1]],40],[[6,[37]]]],[[[34,[33,8]],1,1],[[6,[[5,[1]]]]]],[[[34,[33,8]],1,1,2,3],6],[[[35,[20,[0,[20,8]]]],21],22],[[31,21],22],[[38,21],22],[[38,21],22],[[39,21],22],[[39,21],22],[[[34,[[0,[20,33]],[0,[20,8]]]],21],22],[[]],[[]],[[]],[[]],[[]],0,[[[34,[33,8]],32],[[6,[[9,[8]]]]]],[[31,37],[[6,[37]]]],[[[35,[33,8]],1,1],[[6,[[9,[8]]]]]],[[[35,[33,8]],32],[[6,[[9,[8]]]]]],0,0,[[31,37],17],[[38,24]],0,0,[[[35,[33,8]],1],[[6,[37]]]],[[31,1]],[[[35,[33,8]],37],[[6,[1]]]],[[31,37],1],[[]],[[]],[[]],[[]],[[]],[[],17],[[],17],[[],17],[[],17],[[],17],[[],[[35,[33,8]]]],[[],31],[[],[[34,[33,8]]]],[[31,37,[5,[1]],40],17],0,[[38,38],[[6,[12]]]],0,[[[34,[[0,[33,[41,[8,[29,[8]]]]]],8]]]],[[[35,[[41,[8,[29,[8]]]],33,8]],1,1]],[[31,1,1]],[[[34,[[0,[33,[41,[8,[29,[8]]]]]],8]],1,1]],[[1,1]],[[31,1,1]],[[31,36]],0,[[]],[[]],[[]],[[]],[[]],[[],42],[[],42],0,[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],15],[[],30],[[],30],[[],30],[[],30],[[],30],0,0,[9],[[9,2]],[10,1],[[]],[[1,1]]],"c":[],"p":[[6,"NodeId"],[4,"Accessability"],[8,"FnMut"],[15,"f64"],[3,"Vec"],[4,"Option"],[8,"Clone"],[8,"CoordNum"],[3,"TransitEdge"],[3,"TransitNode"],[8,"Ord"],[4,"Ordering"],[8,"Deserializer"],[8,"Deserialize"],[4,"Result"],[8,"PartialEq"],[15,"bool"],[8,"CoordFloat"],[8,"Sum"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[8,"Hash"],[8,"Hasher"],[8,"FromPrimitive"],[8,"PartialOrd"],[8,"Serialize"],[8,"Serializer"],[3,"Coord"],[3,"TypeId"],[3,"TopologyGraph"],[6,"EdgeId"],[8,"Copy"],[3,"TransitNetwork"],[3,"PhysicalGraph"],[3,"EdgeIndex"],[3,"NodeIndex"],[3,"TopoNode"],[3,"TopoEdge"],[4,"Direction"],[8,"EuclideanDistance"],[3,"String"],[8,"ShortestPathWithAccessability"],[8,"ShortestPath"],[8,"EdgeLength"],[8,"PathCoordinates"],[8,"TopologyGraphRepairer"],[8,"TransitNetworkModifier"],[8,"TransitNetworkRepairer"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
