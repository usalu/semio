
use super::*;

#[test]
fn parallel_directed_edges_are_never_upserted() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e1 = g.add_edge(a, b);
    let e2 = g.add_edge(a, b);
    assert_ne!(e1, e2);
    assert_eq!(g.number_of_edges(Some(a), Some(b)), 2);
    assert_eq!(g.edges_between(a, b).collect::<Vec<_>>(), vec![e1, e2]);
}

#[test]
fn reverse_pair_is_independent() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    assert!(g.has_edge(a, b));
    assert!(!g.has_edge(b, a));
    g.add_edge(b, a);
    assert!(g.has_edge(a, b));
    assert!(g.has_edge(b, a));
    assert_eq!(g.number_of_edges(Some(a), Some(b)), 1);
    assert_eq!(g.number_of_edges(Some(b), Some(a)), 1);
}

#[test]
fn successors_and_predecessors_dedupe_despite_parallels() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, b);
    g.add_edge(a, b);
    assert_eq!(g.successors(a).collect::<Vec<_>>(), vec![b]);
    assert_eq!(g.predecessors(b).collect::<Vec<_>>(), vec![a]);
}

#[test]
fn in_out_degree_count_every_parallel_edge() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, b);
    g.add_edge(b, a);
    assert_eq!(g.out_degree(a), 2);
    assert_eq!(g.in_degree(a), 1);
    assert_eq!(g.degree(a), 3);
}

#[test]
fn selfloop_counts_twice_towards_degree() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    g.add_edge(a, a);
    assert_eq!(g.number_of_selfloops(), 1);
    assert_eq!(g.nodes_with_selfloops().collect::<Vec<_>>(), vec![a]);
    assert_eq!(g.out_degree(a), 1);
    assert_eq!(g.in_degree(a), 1);
    assert_eq!(g.degree(a), 2);
}

#[test]
fn remove_one_edge_leaves_the_rest() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e1 = g.add_edge(a, b);
    let e2 = g.add_edge(a, b);
    assert!(g.remove_one_edge(a, b));
    assert_eq!(g.edges_between(a, b).collect::<Vec<_>>(), vec![e2]);
    assert_ne!(e1, e2);
    assert_eq!(g.number_of_edges(Some(a), Some(b)), 1);
}

#[test]
fn to_undirected_non_reciprocal_keeps_both_directions_as_separate_parallel_edges() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(b, a);
    let u = g.to_undirected(false);
    assert_eq!(u.edge_count(), 2);
    assert_eq!(GraphView::degree(&u, a), 2);
    assert_eq!(GraphView::degree(&u, b), 2);
}

#[test]
fn to_undirected_reciprocal_keeps_only_mutual_pairs() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b);
    g.add_edge(b, a);
    g.add_edge(a, c); // not reciprocated
    let u = g.to_undirected(true);
    assert_eq!(u.edge_count(), 2); // a<->b pair survives as two edges, a-c dropped
    assert_eq!(GraphView::degree(&u, a), 2);
    assert_eq!(GraphView::degree(&u, c), 0);
}

#[test]
fn to_simple_sums_weights() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let mut attrs1 = PropertyBag::new();
    attrs1.insert("weight".to_string(), PropertyValue::Number(2.0));
    g.add_edge_with(a, b, attrs1);
    let mut attrs2 = PropertyBag::new();
    attrs2.insert("weight".to_string(), PropertyValue::Number(3.5));
    g.add_edge_with(a, b, attrs2);
    g.add_edge(b, a); // unweighted -> defaults to 1.0, separate pair
    let s = g.to_simple();
    assert_eq!(s.edge_count(), 2);
    let ab = s.edges_between(a, b).next().expect("a->b collapsed edge exists");
    assert_eq!(s.edge_attrs(ab.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64), Some(5.5));
    let ba = s.edges_between(b, a).next().expect("b->a collapsed edge exists");
    assert_eq!(s.edge_attrs(ba.id).and_then(|attrs| attrs.get("weight")).and_then(PropertyValue::as_f64), Some(1.0));
}

#[test]
fn reverse_flips_every_edge() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.add_edge(a, b);
    let r = g.reverse();
    assert!(!r.has_edge(a, b));
    assert_eq!(r.number_of_edges(Some(b), Some(a)), 2);
}

#[test]
fn add_edge_auto_creates_missing_nodes() {
    let mut g = PortDirectedGraph::new();
    assert!(!g.has_node(0));
    assert!(!g.has_node(1));
    g.add_edge(0, 1);
    assert!(g.has_node(0));
    assert!(g.has_node(1));
    assert!(g.has_edge(0, 1));
}

#[test]
fn add_path_cycle_star_build_directed_shapes() {
    let mut g = PortDirectedGraph::new();
    g.add_path([0, 1, 2]);
    assert!(g.has_edge(0, 1));
    assert!(g.has_edge(1, 2));
    assert!(!g.has_edge(2, 0));

    let mut cyc = PortDirectedGraph::new();
    cyc.add_cycle([0, 1, 2]);
    assert!(cyc.has_edge(0, 1));
    assert!(cyc.has_edge(1, 2));
    assert!(cyc.has_edge(2, 0));

    let mut star = PortDirectedGraph::new();
    star.add_star([0, 1, 2, 3]);
    assert!(star.has_edge(0, 1));
    assert!(star.has_edge(0, 2));
    assert!(star.has_edge(0, 3));
    assert!(!star.has_edge(1, 2));
}

#[test]
fn subgraph_and_edge_subgraph_restrict_correctly() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b);
    let ac = g.add_edge(a, c);

    let sub = g.subgraph([a, b]);
    assert_eq!(sub.number_of_nodes(), 2);
    assert!(sub.has_edge(a, b));
    assert!(!sub.has_node(c));

    let esub = g.edge_subgraph([ac]);
    assert_eq!(esub.number_of_nodes(), 2);
    assert!(esub.has_edge(a, c));
    assert!(!esub.has_node(b));
}

#[test]
fn density_can_exceed_one_for_multigraph() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    for _ in 0..5 {
        g.add_edge(a, b);
    }
    assert!(g.density() > 1.0);
}

#[test]
fn clear_and_clear_edges_behave() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    g.clear_edges();
    assert_eq!(g.number_of_edges(None, None), 0);
    assert_eq!(g.number_of_nodes(), 2);
    g.clear();
    assert_eq!(g.number_of_nodes(), 0);
}

#[test]
fn add_node_with_and_with_id_apply_attrs() {
    let mut g = PortDirectedGraph::new();
    let mut attrs = PropertyBag::new();
    attrs.insert("color".to_string(), PropertyValue::String("red".to_string()));
    let a = g.add_node_with(attrs.clone());
    assert_eq!(g.get_node_attributes(a), Some(&attrs));
    let b = g.add_node_with_id(42, attrs.clone());
    assert_eq!(b, 42);
    assert!(g.has_node(42));
    assert_eq!(g.get_node_attributes(42), Some(&attrs));
}

#[test]
fn add_nodes_from_and_remove_nodes_from() {
    let mut g = PortDirectedGraph::new();
    g.add_nodes_from([1, 2, 3]);
    assert_eq!(g.number_of_nodes(), 3);
    assert_eq!(g.order(), 3);
    assert_eq!(g.nodes().collect::<Vec<_>>(), vec![1, 2, 3]);
    g.remove_nodes_from([1, 3]);
    assert!(!g.has_node(1));
    assert!(g.has_node(2));
    assert!(!g.has_node(3));
    assert_eq!(g.number_of_nodes(), 1);
}

#[test]
fn remove_node_cascades_incident_edges_and_forgets_handle() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    assert!(g.remove_node(a));
    assert!(!g.has_node(a));
    assert_eq!(g.number_of_edges(None, None), 0);
    assert!(!g.remove_node(a));
    let a2 = g.add_node_with_id(a, PropertyBag::new());
    assert_eq!(a2, a);
    assert_eq!(g.out_degree(a), 0);
}

#[test]
fn remove_edge_reports_existence() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e = g.add_edge(a, b);
    assert!(g.remove_edge(e));
    assert!(!g.has_edge(a, b));
    assert!(!g.remove_edge(e));
    assert!(!g.remove_one_edge(a, b));
}

#[test]
fn add_edges_from_and_weighted_edges_from() {
    let mut g = PortDirectedGraph::new();
    g.add_edges_from([(0, 1), (1, 2)]);
    assert!(g.has_edge(0, 1));
    assert!(g.has_edge(1, 2));

    let mut w = PortDirectedGraph::new();
    w.add_weighted_edges_from([(0, 1, 2.5), (1, 2, 4.0)]);
    let e = w.edges_between(0, 1).next().expect("edge exists");
    assert_eq!(w.get_edge_data(e).and_then(|a| a.get("weight")).and_then(PropertyValue::as_f64), Some(2.5));
}

#[test]
fn number_of_edges_single_endpoint_counts_in_and_out() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b);
    g.add_edge(c, a);
    assert_eq!(g.number_of_edges(Some(a), None), 2);
    assert_eq!(g.number_of_edges(None, Some(a)), 2);
}

#[test]
fn in_edges_and_out_edges_yield_correct_refs() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    let e1 = g.add_edge(a, b);
    let e2 = g.add_edge(c, a);
    assert_eq!(g.out_edges(a).map(|e| e.id).collect::<Vec<_>>(), vec![e1]);
    assert_eq!(g.in_edges(a).map(|e| e.id).collect::<Vec<_>>(), vec![e2]);
}

#[test]
fn all_neighbors_unions_predecessors_and_successors() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b);
    g.add_edge(c, a);
    assert_eq!(g.all_neighbors(a).collect::<Vec<_>>(), vec![b, c]);
}

#[test]
fn weighted_degree_sums_in_and_out_weights() {
    struct UnitWeights;
    impl EdgeWeights for UnitWeights {
        fn weight(&self, edge: EdgeRef) -> f64 {
            edge.id as f64
        }
    }
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let out = g.add_edge(a, b);
    let inn = g.add_edge(b, a);
    let weights = UnitWeights;
    assert_eq!(g.weighted_degree(a, &weights), (out + inn) as f64);
}

#[test]
fn density_is_zero_below_two_nodes() {
    let mut g = PortDirectedGraph::new();
    assert_eq!(g.density(), 0.0);
    g.add_node();
    assert_eq!(g.density(), 0.0);
}

#[test]
fn is_empty_tracks_edge_presence_not_node_presence() {
    let mut g = PortDirectedGraph::new();
    assert!(g.is_empty());
    g.add_node();
    assert!(g.is_empty());
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    assert!(!g.is_empty());
}

#[test]
fn copy_produces_an_independent_clone() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    let mut c = g.copy();
    c.add_edge(b, a);
    assert!(!g.has_edge(b, a));
    assert!(c.has_edge(b, a));
}

#[test]
fn edge_subgraph_skips_edges_not_in_the_keep_set() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let c = g.add_node();
    g.add_edge(a, b);
    g.add_edge(b, c);
    let esub = g.edge_subgraph([]);
    assert_eq!(esub.number_of_nodes(), 0);
    assert_eq!(esub.number_of_edges(None, None), 0);
}

#[test]
fn subgraph_ignores_nonexistent_node_ids() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let sub = g.subgraph([a, 999]);
    assert_eq!(sub.number_of_nodes(), 1);
    assert!(!sub.has_node(999));
}

#[test]
fn set_and_get_node_and_edge_attributes() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e = g.add_edge(a, b);
    let mut node_attrs = PropertyBag::new();
    node_attrs.insert("label".to_string(), PropertyValue::String("A".to_string()));
    g.set_node_attributes(a, node_attrs.clone());
    assert_eq!(g.get_node_attributes(a), Some(&node_attrs));

    let mut edge_attrs = PropertyBag::new();
    edge_attrs.insert("weight".to_string(), PropertyValue::Number(1.5));
    g.set_edge_attributes(e, edge_attrs.clone());
    assert_eq!(g.get_edge_attributes(e), Some(&edge_attrs));

    g.set_node_attributes(999, node_attrs);
    assert_eq!(g.get_node_attributes(999), None);
}

#[test]
fn name_defaults_empty_and_is_settable() {
    let mut g = PortDirectedGraph::new();
    assert_eq!(g.name(), "");
    g.set_name("my-graph");
    assert_eq!(g.name(), "my-graph");
}

#[test]
fn default_matches_new() {
    let g = PortDirectedGraph::default();
    assert_eq!(g.number_of_nodes(), 0);
    assert_eq!(g.number_of_edges(None, None), 0);
}

#[test]
fn graph_view_and_attr_view_delegate_correctly() {
    let mut g = PortDirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let e = g.add_edge_with(a, b, {
        let mut attrs = PropertyBag::new();
        attrs.insert("weight".to_string(), PropertyValue::Number(3.0));
        attrs
    });
    assert_eq!(GraphView::node_count(&g), 2);
    assert_eq!(GraphView::edge_count(&g), 1);
    assert!(GraphView::contains_node(&g, a));
    assert_eq!(GraphView::nodes(&g).collect::<Vec<_>>(), vec![a, b]);
    assert_eq!(GraphView::edges(&g).map(|e| e.id).collect::<Vec<_>>(), vec![e]);
    assert_eq!(GraphView::neighbors(&g, a).collect::<Vec<_>>(), vec![b]);
    assert_eq!(GraphView::out_neighbors(&g, a).collect::<Vec<_>>(), vec![b]);
    assert_eq!(GraphView::in_neighbors(&g, b).collect::<Vec<_>>(), vec![a]);
    assert_eq!(GraphView::degree(&g, a), 1);
    assert_eq!(GraphView::out_degree(&g, a), 1);
    assert_eq!(GraphView::in_degree(&g, b), 1);
    assert!(GraphView::is_directed(&g));
    assert!(GraphView::is_multigraph(&g));
    assert_eq!(GraphView::edges_between(&g, a, b).map(|e| e.id).collect::<Vec<_>>(), vec![e]);

    assert!(AttrView::node_attrs(&g, a).is_some());
    let edge_ref = GraphView::edges(&g).next().expect("edge exists");
    assert_eq!(AttrView::edge_attrs(&g, e).and_then(|a| a.get("weight")).and_then(PropertyValue::as_f64), Some(3.0));
    assert!(AttrView::graph_attrs(&g).is_empty());
    assert_eq!(EdgeWeights::weight(&g, edge_ref), 3.0);
}
