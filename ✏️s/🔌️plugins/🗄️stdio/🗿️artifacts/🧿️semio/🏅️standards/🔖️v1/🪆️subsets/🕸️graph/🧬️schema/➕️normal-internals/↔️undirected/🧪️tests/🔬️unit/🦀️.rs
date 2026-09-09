use super::*;

#[test]
fn crud_round_trip() {
    let mut g = UndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    assert_eq!(g.number_of_nodes(), 2);
    assert!(g.has_node(a) && g.has_node(b));
    g.add_edge(a, b);
    assert_eq!(g.number_of_edges(), 1);
    assert!(g.has_edge(a, b) && g.has_edge(b, a));
    assert!(g.remove_edge(a, b));
    assert_eq!(g.number_of_edges(), 0);
    assert!(g.remove_node(a));
    assert_eq!(g.number_of_nodes(), 1);
    assert!(!g.has_node(a));
}

#[test]
fn add_edge_upsert_semantics() {
    let mut g = UndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let mut attrs1 = PropertyBag::new();
    attrs1.insert("weight".to_string(), PropertyValue::Number(1.0));
    let e1 = g.add_edge_with(a, b, attrs1);
    let mut attrs2 = PropertyBag::new();
    attrs2.insert("color".to_string(), PropertyValue::String("red".to_string()));
    let e2 = g.add_edge_with(a, b, attrs2);
    assert_eq!(e1, e2, "upserting an existing pair must not allocate a new edge id");
    assert_eq!(g.number_of_edges(), 1);
    let data = g.get_edge_data(a, b).expect("edge data");
    assert_eq!(data.get("weight").and_then(PropertyValue::as_f64), Some(1.0));
    assert_eq!(data.get("color").and_then(PropertyValue::as_str), Some("red"));
}

#[test]
fn selfloop_counts_double_degree() {
    let mut g = UndirectedGraph::new();
    let a = g.add_node();
    g.add_edge(a, a);
    assert_eq!(g.degree(a), 2, "NetworkX counts a self-loop twice towards degree");
    assert_eq!(g.number_of_selfloops(), 1);
    assert_eq!(g.nodes_with_selfloops().collect::<Vec<_>>(), vec![a]);
    assert_eq!(g.selfloop_edges().count(), 1);
}

#[test]
fn density_edge_cases() {
    let empty = UndirectedGraph::new();
    assert_eq!(empty.density(), 0.0, "empty graph is a documented n < 2 case, not a panic");

    let mut singleton = UndirectedGraph::new();
    singleton.add_node();
    assert_eq!(singleton.density(), 0.0);

    let mut k4 = UndirectedGraph::new();
    let nodes: Vec<NodeId> = (0..4).map(|_| k4.add_node()).collect();
    for i in 0..nodes.len() {
        for &v in &nodes[(i + 1)..] {
            k4.add_edge(nodes[i], v);
        }
    }
    assert!((k4.density() - 1.0).abs() < 1e-9, "K4 is a complete graph, density == 1.0");
}

#[test]
fn is_empty_vs_number_of_nodes() {
    let mut g = UndirectedGraph::new();
    assert!(g.is_empty());
    assert_eq!(g.number_of_nodes(), 0);
    let a = g.add_node();
    let b = g.add_node();
    assert!(g.is_empty(), "nodes without edges is still empty per NetworkX convention");
    assert_eq!(g.number_of_nodes(), 2);
    g.add_edge(a, b);
    assert!(!g.is_empty());
}

#[test]
fn subgraph_and_edge_subgraph_are_independent_copies() {
    let mut g = UndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    let e_ab = g.add_edge(a, b);
    g.add_edge(b, c);

    let mut sub = g.subgraph([a, b]);
    assert_eq!(sub.number_of_nodes(), 2);
    assert_eq!(sub.number_of_edges(), 1);
    sub.remove_edge(a, b);
    assert!(g.has_edge(a, b), "mutating the subgraph copy must not affect the original");

    let mut edge_sub = g.edge_subgraph([e_ab]);
    assert_eq!(edge_sub.number_of_nodes(), 2);
    assert_eq!(edge_sub.number_of_edges(), 1);
    edge_sub.add_node();
    assert_eq!(g.number_of_nodes(), 3, "mutating the edge_subgraph copy must not affect the original");
}

#[test]
fn to_directed_doubles_edge_count() {
    let mut g = UndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b);
    g.add_edge(b, c);
    let directed = g.to_directed();
    assert_eq!(GraphView::edge_count(&directed), 4);
    assert!(GraphView::is_directed(&directed));

    let mut looped = UndirectedGraph::new();
    let n = looped.add_node();
    looped.add_edge(n, n);
    let looped_directed = looped.to_directed();
    assert_eq!(GraphView::edge_count(&looped_directed), 1, "a self-loop has only one direction, so it must not double");
}

#[test]
fn path_cycle_star_builders() {
    let mut g = UndirectedGraph::new();
    let nodes: Vec<NodeId> = (0..4).map(|_| g.add_node()).collect();
    g.add_path(&nodes);
    assert!(g.is_path(&nodes));
    assert_eq!(g.number_of_edges(), 3);

    let mut cyc = UndirectedGraph::new();
    let cnodes: Vec<NodeId> = (0..4).map(|_| cyc.add_node()).collect();
    cyc.add_cycle(&cnodes);
    assert_eq!(cyc.number_of_edges(), 4);

    let mut looped_cycle = UndirectedGraph::new();
    let solo = looped_cycle.add_node();
    looped_cycle.add_cycle(&[solo]);
    assert_eq!(looped_cycle.number_of_selfloops(), 1);

    let mut star = UndirectedGraph::new();
    let center = star.add_node();
    let leaves: Vec<NodeId> = (0..3).map(|_| star.add_node()).collect();
    star.add_star(center, &leaves);
    assert_eq!(star.degree(center), 3);
}

#[test]
fn attributes_round_trip() {
    let mut g = UndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);

    let mut node_attrs = PropertyBag::new();
    node_attrs.insert("label".to_string(), PropertyValue::String("A".to_string()));
    g.set_node_attributes([(a, node_attrs)]);
    let got_nodes = g.get_node_attributes("label");
    assert_eq!(got_nodes.get(&a).and_then(PropertyValue::as_str), Some("A"));

    let mut edge_attrs = PropertyBag::new();
    edge_attrs.insert("weight".to_string(), PropertyValue::Number(2.5));
    g.set_edge_attributes([(a, b, edge_attrs)]);
    let got_edges = g.get_edge_attributes("weight");
    let key = if a <= b { (a, b) } else { (b, a) };
    assert_eq!(got_edges.get(&key).and_then(PropertyValue::as_f64), Some(2.5));

    assert_eq!(g.name(), None);
    g.set_name("test-graph".to_string());
    assert_eq!(g.name().as_deref(), Some("test-graph"));
}

#[test]
fn path_helpers() {
    let mut g = UndirectedGraph::new();
    let nodes: Vec<NodeId> = (0..4).map(|_| g.add_node()).collect();
    g.add_weighted_edges_from([(nodes[0], nodes[1], 2.0), (nodes[1], nodes[2], 3.0)]);
    assert_eq!(g.path_weight(&nodes[0..3], "weight"), Some(5.0));
    assert_eq!(g.path_weight(&[nodes[0], nodes[3]], "weight"), None);

    assert_eq!(g.common_neighbors(nodes[0], nodes[2]).collect::<Vec<_>>(), vec![nodes[1]]);
    assert!(g.non_neighbors(nodes[0]).collect::<Vec<_>>().contains(&nodes[3]));
    assert!(g.non_edges().any(|(u, v)| (u, v) == (nodes[0], nodes[2]) || (u, v) == (nodes[2], nodes[0])));
}

#[test]
fn weighted_degree_and_size() {
    let mut g = UndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_weighted_edges_from([(a, b, 4.0)]);
    assert_eq!(g.weighted_degree(a, "weight"), 4.0);
    assert_eq!(g.size(false), 1.0);
    assert_eq!(g.size(true), 4.0);
}
