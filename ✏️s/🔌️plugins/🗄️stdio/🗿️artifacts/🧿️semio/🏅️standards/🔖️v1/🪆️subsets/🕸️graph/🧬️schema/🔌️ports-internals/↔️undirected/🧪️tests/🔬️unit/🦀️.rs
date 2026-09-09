use super::*;

#[test]
fn parallel_edges_get_distinct_ids() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e1 = g.add_edge(a, b);
    let e2 = g.add_edge(a, b);
    assert_ne!(e1, e2);
    assert_eq!(g.edges_between(a, b).count(), 2);
}

#[test]
fn neighbors_dedupe_across_parallel_edges() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, b);
    g.add_edge(a, b);
    let neighbors: Vec<NodeId> = g.neighbors(a).collect();
    assert_eq!(neighbors, vec![b]);
}

#[test]
fn degree_counts_every_parallel_edge() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, b);
    assert_eq!(g.degree(a), 2);
    assert_eq!(g.degree(b), 2);
}

#[test]
fn self_loop_counts_twice_towards_degree() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    g.add_edge(a, a);
    assert_eq!(g.degree(a), 2);
}

#[test]
fn remove_one_edge_drops_exactly_one_parallel_edge() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e1 = g.add_edge(a, b);
    let e2 = g.add_edge(a, b);
    assert!(g.remove_one_edge(a, b));
    let remaining: Vec<EdgeId> = g.edges_between(a, b).collect();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0], e1.max(e2));
}

#[test]
fn to_simple_sums_parallel_edge_weights() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    for weight in [1.0, 2.0, 3.0] {
        let mut attrs = PropertyBag::default();
        attrs.insert("weight".to_string(), PropertyValue::Number(weight));
        g.add_edge_with(a, b, attrs);
    }
    let simple = g.to_simple();
    assert_eq!(simple.edge_count(), 1);
    let edge = simple.edges().next().expect("one collapsed edge");
    let weight = simple.edge_attrs(edge.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64);
    assert_eq!(weight, Some(6.0));
}

#[test]
fn edges_between_returns_all_parallel_ids() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e1 = g.add_edge(a, b);
    let e2 = g.add_edge(a, b);
    let e3 = g.add_edge(a, b);
    let mut ids: Vec<EdgeId> = g.edges_between(a, b).collect();
    ids.sort_unstable();
    let mut expected = vec![e1, e2, e3];
    expected.sort_unstable();
    assert_eq!(ids, expected);
}

#[test]
fn add_edge_auto_creates_unseen_nodes() {
    let mut g = PortUndirectedGraph::new();
    assert!(!g.has_node(42));
    assert!(!g.has_node(7));
    g.add_edge(42, 7);
    assert!(g.has_node(42));
    assert!(g.has_node(7));
    assert!(g.has_edge(42, 7));
}

#[test]
fn subgraph_and_edge_subgraph_are_independent_copies() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    let e_ab = g.add_edge(a, b);
    g.add_edge(b, c);

    let sub = g.subgraph([a, b]);
    assert_eq!(sub.number_of_nodes(), 2);
    assert_eq!(sub.number_of_edges(None, None), 1);

    let esub = g.edge_subgraph([e_ab]);
    assert_eq!(esub.number_of_nodes(), 2);
    assert_eq!(esub.number_of_edges(None, None), 1);

    g.add_edge(a, c);
    assert_eq!(sub.number_of_edges(None, None), 1);
    assert_eq!(esub.number_of_edges(None, None), 1);
}

#[test]
fn add_node_with_stores_attrs() {
    let mut g = PortUndirectedGraph::new();
    let mut attrs = PropertyBag::default();
    attrs.insert("color".to_string(), PropertyValue::String("red".to_string()));
    let n = g.add_node_with(attrs);
    assert_eq!(g.get_node_attributes(n).and_then(|a| a.get("color")).and_then(PropertyValue::as_str), Some("red"));
}

#[test]
fn add_node_with_id_reuses_given_id() {
    let mut g = PortUndirectedGraph::new();
    let n = g.add_node_with_id(99, PropertyBag::default());
    assert_eq!(n, 99);
    assert!(g.has_node(99));
}

#[test]
fn add_nodes_from_creates_every_node() {
    let mut g = PortUndirectedGraph::new();
    g.add_nodes_from([1, 2, 3]);
    assert_eq!(g.number_of_nodes(), 3);
    assert!(g.has_node(1) && g.has_node(2) && g.has_node(3));
}

#[test]
fn remove_node_drops_incident_edges_and_reallocates_fresh_handle() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    assert!(g.remove_node(a));
    assert!(!g.has_node(a));
    assert_eq!(g.number_of_edges(None, None), 0);
    assert!(!g.remove_node(a));
    let a2 = g.add_node_with_id(a, PropertyBag::default());
    g.add_edge(a2, b);
    assert!(g.has_edge(a2, b));
}

#[test]
fn remove_nodes_from_removes_every_listed_node() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.remove_nodes_from([a, b]);
    assert!(!g.has_node(a));
    assert!(!g.has_node(b));
    assert!(g.has_node(c));
}

#[test]
fn order_matches_number_of_nodes_and_nodes_iterator() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    assert_eq!(g.order(), g.number_of_nodes());
    let mut ids: Vec<NodeId> = g.nodes().collect();
    ids.sort_unstable();
    assert_eq!(ids, vec![a, b]);
}

#[test]
fn add_edges_from_and_add_weighted_edges_from() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    let ids = g.add_edges_from([(a, b), (b, c)]);
    assert_eq!(ids.len(), 2);
    assert!(g.has_edge(a, b) && g.has_edge(b, c));

    let weighted_ids = g.add_weighted_edges_from([(a, c, 4.5)]);
    assert_eq!(weighted_ids.len(), 1);
    let attrs = g.get_edge_data(weighted_ids[0]).expect("edge attrs");
    assert_eq!(attrs.get("weight").and_then(PropertyValue::as_f64), Some(4.5));
}

#[test]
fn remove_edge_by_id_and_missing_edge_data() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e = g.add_edge(a, b);
    assert!(g.remove_edge(e));
    assert!(!g.remove_edge(e));
    assert!(g.get_edge_data(e).is_none());
}

#[test]
fn remove_one_edge_returns_false_when_pair_has_no_edge() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    assert!(!g.remove_one_edge(a, b));
}

#[test]
fn number_of_edges_falls_back_to_total_unless_both_endpoints_given() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b);
    g.add_edge(b, c);
    assert_eq!(g.number_of_edges(Some(a), Some(b)), 1);
    assert_eq!(g.number_of_edges(None, None), 2);
    assert_eq!(g.number_of_edges(Some(a), None), 2);
    assert_eq!(g.number_of_edges(None, Some(b)), 2);
}

#[test]
fn add_path_chains_consecutive_nodes() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    let ids = g.add_path([a, b, c]);
    assert_eq!(ids.len(), 2);
    assert!(g.has_edge(a, b) && g.has_edge(b, c));
    assert!(!g.has_edge(a, c));
}

#[test]
fn add_cycle_empty_single_and_many() {
    let mut empty_graph = PortUndirectedGraph::new();
    assert!(empty_graph.add_cycle(Vec::<NodeId>::new()).is_empty());

    let mut single_graph = PortUndirectedGraph::new();
    let a = single_graph.add_node();
    let ids = single_graph.add_cycle([a]);
    assert_eq!(ids.len(), 1);
    assert_eq!(single_graph.number_of_selfloops(), 1);

    let mut ring_graph = PortUndirectedGraph::new();
    let a = ring_graph.add_node();
    let b = ring_graph.add_node();
    let c = ring_graph.add_node();
    let ids = ring_graph.add_cycle([a, b, c]);
    assert_eq!(ids.len(), 3);
    assert!(ring_graph.has_edge(a, b) && ring_graph.has_edge(b, c) && ring_graph.has_edge(c, a));
}

#[test]
fn add_star_empty_and_hub_with_leaves() {
    let mut g = PortUndirectedGraph::new();
    assert!(g.add_star(Vec::<NodeId>::new()).is_empty());

    let semio_hub = g.add_node();
    let leaf1 = g.add_node();
    let leaf2 = g.add_node();
    let ids = g.add_star([semio_hub, leaf1, leaf2]);
    assert_eq!(ids.len(), 2);
    assert!(g.has_edge(semio_hub, leaf1) && g.has_edge(semio_hub, leaf2));
    assert!(!g.has_edge(leaf1, leaf2));
}

#[test]
fn weighted_degree_sums_named_attribute_with_default_and_counts_selfloop_twice() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let mut weighted = PropertyBag::default();
    weighted.insert("cost".to_string(), PropertyValue::Number(2.0));
    g.add_edge_with(a, b, weighted);
    g.add_edge(a, b);
    assert_eq!(g.weighted_degree(a, "cost"), 3.0);

    let mut loop_graph = PortUndirectedGraph::new();
    let n = loop_graph.add_node();
    let mut loop_weight = PropertyBag::default();
    loop_weight.insert("cost".to_string(), PropertyValue::Number(5.0));
    loop_graph.add_edge_with(n, n, loop_weight);
    assert_eq!(loop_graph.weighted_degree(n, "cost"), 10.0);
}

#[test]
fn density_zero_for_zero_or_one_node_else_multigraph_formula() {
    let empty_graph = PortUndirectedGraph::new();
    assert_eq!(empty_graph.density(), 0.0);

    let mut single_graph = PortUndirectedGraph::new();
    single_graph.add_node();
    assert_eq!(single_graph.density(), 0.0);

    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, b);
    assert_eq!(g.density(), 2.0);
}

#[test]
fn is_empty_reflects_edge_presence_not_node_presence() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    assert!(g.is_empty());
    g.add_edge(a, b);
    assert!(!g.is_empty());
}

#[test]
fn copy_is_an_independent_snapshot() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    let snapshot = g.copy();
    g.add_edge(a, b);
    assert_eq!(snapshot.number_of_edges(None, None), 1);
    assert_eq!(g.number_of_edges(None, None), 2);
}

#[test]
fn to_directed_mirrors_edges_both_ways_but_selfloop_once() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, a);
    let directed = g.to_directed();
    assert_eq!(directed.edges_between(a, b).count(), 1);
    assert_eq!(directed.edges_between(b, a).count(), 1);
    assert_eq!(directed.edges_between(a, a).count(), 1);
    assert_eq!(directed.edge_count(), 3);
}

#[test]
fn clear_removes_nodes_edges_and_handles() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.clear();
    assert_eq!(g.number_of_nodes(), 0);
    assert_eq!(g.number_of_edges(None, None), 0);
    assert!(!g.has_node(a));
}

#[test]
fn clear_edges_keeps_nodes_but_drops_edges() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.clear_edges();
    assert_eq!(g.number_of_nodes(), 2);
    assert_eq!(g.number_of_edges(None, None), 0);
    assert!(g.has_node(a) && g.has_node(b));
}

#[test]
fn set_node_attributes_extends_existing_and_ignores_missing_node() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let mut attrs = PropertyBag::default();
    attrs.insert("label".to_string(), PropertyValue::String("A".to_string()));
    g.set_node_attributes(a, attrs);
    assert_eq!(g.get_node_attributes(a).and_then(|b| b.get("label")).and_then(PropertyValue::as_str), Some("A"));

    let mut missing_attrs = PropertyBag::default();
    missing_attrs.insert("label".to_string(), PropertyValue::String("ghost".to_string()));
    g.set_node_attributes(9999, missing_attrs);
    assert!(g.get_node_attributes(9999).is_none());
}

#[test]
fn set_edge_attributes_extends_and_get_edge_attributes_missing() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e = g.add_edge(a, b);
    let mut attrs = PropertyBag::default();
    attrs.insert("weight".to_string(), PropertyValue::Number(1.5));
    g.set_edge_attributes(e, attrs);
    assert_eq!(g.get_edge_attributes(e).and_then(|a| a.get("weight")).and_then(PropertyValue::as_f64), Some(1.5));
    assert!(g.get_edge_attributes(e + 1000).is_none());
}

#[test]
fn name_defaults_to_none_then_reflects_set_name() {
    let mut g = PortUndirectedGraph::new();
    assert_eq!(g.name(), None);
    g.set_name("social");
    assert_eq!(g.name(), Some("social"));
}

#[test]
fn selfloop_edges_and_nodes_with_selfloops_report_only_loops() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, a);
    let loops: Vec<EdgeId> = g.selfloop_edges().collect();
    assert_eq!(loops.len(), 1);
    assert_eq!(g.number_of_selfloops(), 1);
    let loop_nodes: Vec<NodeId> = g.nodes_with_selfloops().collect();
    assert_eq!(loop_nodes, vec![a]);
}

#[test]
fn graphview_trait_delegates_match_native_methods() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);

    assert_eq!(GraphView::node_count(&g), g.number_of_nodes());
    assert_eq!(GraphView::edge_count(&g), g.number_of_edges(None, None));
    assert!(GraphView::contains_node(&g, a));
    assert!(!GraphView::contains_node(&g, 12345));
    assert_eq!(GraphView::degree(&g, a), g.degree(a));
    assert_eq!(GraphView::out_degree(&g, a), GraphView::degree(&g, a));
    assert_eq!(GraphView::in_degree(&g, a), GraphView::degree(&g, a));
    let out_neighbors: Vec<NodeId> = GraphView::out_neighbors(&g, a).collect();
    assert_eq!(out_neighbors, vec![b]);
    let in_neighbors: Vec<NodeId> = GraphView::in_neighbors(&g, a).collect();
    assert_eq!(in_neighbors, vec![b]);
    assert!(!GraphView::is_directed(&g));
    assert!(GraphView::is_multigraph(&g));
    assert_eq!(GraphView::edges_between(&g, a, b).count(), 1);
    assert_eq!(GraphView::edges(&g).count(), 1);
}

#[test]
fn attrview_and_edgeweights_delegate_correctly() {
    let mut g = PortUndirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let mut attrs = PropertyBag::default();
    attrs.insert("weight".to_string(), PropertyValue::Number(3.0));
    g.add_edge_with(a, b, attrs);
    assert!(AttrView::node_attrs(&g, a).is_some());
    assert!(AttrView::graph_attrs(&g).is_empty());
    let edge = g.storage.edges().next().expect("one edge");
    assert_eq!(EdgeWeights::weight(&g, edge), 3.0);
}
