
use super::*;

// #subregion CrudDirection
#[test]
fn add_edge_respects_direction() {
    let mut g = DirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    g.add_edge(a, b);
    assert!(g.has_edge(a, b));
    assert!(!g.has_edge(b, a));
    assert_eq!(g.number_of_edges(), 1);
    assert_eq!(g.out_degree(a), 1);
    assert_eq!(g.in_degree(a), 0);
    assert_eq!(g.out_degree(b), 0);
    assert_eq!(g.in_degree(b), 1);
}

#[test]
fn add_edge_upserts_and_auto_creates_nodes() {
    let mut g = DirectedGraph::new();
    let id1 = g.add_edge(10, 20);
    assert!(g.has_node(10));
    assert!(g.has_node(20));
    let mut attrs = PropertyBag::new();
    attrs.insert("weight".to_string(), PropertyValue::Number(2.0));
    let id2 = g.add_edge_with(10, 20, attrs);
    assert_eq!(id1, id2);
    assert_eq!(g.number_of_edges(), 1);
}

#[test]
fn remove_edge_is_directional() {
    let mut g = DirectedGraph::new();
    g.add_edge(1, 2);
    g.add_edge(2, 1);
    assert!(g.remove_edge(1, 2));
    assert!(!g.has_edge(1, 2));
    assert!(g.has_edge(2, 1));
    assert!(!g.remove_edge(1, 2));
}
// #endsubregion

// #subregion SelfLoopDegree
#[test]
fn selfloop_counts_twice_towards_degree() {
    let mut g = DirectedGraph::new();
    let a = g.add_node();
    g.add_edge(a, a);
    assert_eq!(g.number_of_selfloops(), 1);
    assert_eq!(g.nodes_with_selfloops().collect::<Vec<_>>(), vec![a]);
    assert_eq!(g.out_degree(a), 1);
    assert_eq!(g.in_degree(a), 1);
    assert_eq!(g.degree(a), 2);
}
// #endsubregion

// #subregion Density
#[test]
fn density_on_small_complete_digraph() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    for &u in &[a, b, c] {
        for &v in &[a, b, c] {
            if u != v {
                g.add_edge(u, v);
            }
        }
    }
    assert_eq!(g.number_of_edges(), 6);
    assert!((g.density() - 1.0).abs() < 1e-12);
}

#[test]
fn density_is_zero_below_two_nodes() {
    let mut g = DirectedGraph::new();
    assert_eq!(g.density(), 0.0);
    g.add_node();
    assert_eq!(g.density(), 0.0);
}
// #endsubregion

// #subregion IndependentCopies
#[test]
fn subgraph_is_an_independent_copy() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(b, c);
    let mut sub = g.subgraph([a, b]);
    assert_eq!(sub.number_of_nodes(), 2);
    assert!(sub.has_edge(a, b));
    assert!(!sub.has_node(c));
    sub.add_edge(a, a);
    assert!(!g.has_edge(a, a));
}

#[test]
fn edge_subgraph_is_an_independent_copy() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    let e_ab = g.add_edge(a, b);
    g.add_edge(b, c);
    let mut esub = g.edge_subgraph([e_ab]);
    assert_eq!(esub.number_of_nodes(), 2);
    assert_eq!(esub.number_of_edges(), 1);
    assert!(esub.has_edge(a, b));
    esub.add_edge(b, a);
    assert!(!g.has_edge(b, a));
}
// #endsubregion

// #subregion Reverse
#[test]
fn reverse_swaps_every_edge_and_round_trips() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(b, c);
    let r = g.reverse();
    assert!(r.has_edge(b, a));
    assert!(r.has_edge(c, b));
    assert!(!r.has_edge(a, b));
    let rr = r.reverse();
    let mut original: Vec<(NodeId, NodeId)> = g.edges().map(|e| (e.u, e.v)).collect();
    let mut round_tripped: Vec<(NodeId, NodeId)> = rr.edges().map(|e| (e.u, e.v)).collect();
    original.sort();
    round_tripped.sort();
    assert_eq!(original, round_tripped);
}
// #endsubregion

// #subregion ToUndirected
#[test]
fn to_undirected_non_reciprocal_dedupes() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(b, a);
    g.add_edge(b, c);
    let u = g.to_undirected(false);
    assert_eq!(u.edge_count(), 2);
    assert_eq!(u.edges_between(a, b).count(), 1);
    assert_eq!(u.edges_between(b, c).count(), 1);
}

#[test]
fn to_undirected_reciprocal_keeps_only_mutual_edges() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(b, a);
    g.add_edge(b, c);
    let u = g.to_undirected(true);
    assert_eq!(u.edge_count(), 1);
    assert_eq!(u.edges_between(a, b).count(), 1);
    assert_eq!(u.edges_between(b, c).count(), 0);
}
// #endsubregion

// #subregion Builders
#[test]
fn path_cycle_star_respect_direction() {
    let mut g = DirectedGraph::new();
    g.add_path(&[1, 2, 3]);
    assert!(g.has_edge(1, 2));
    assert!(g.has_edge(2, 3));
    assert!(!g.has_edge(2, 1));

    let mut g2 = DirectedGraph::new();
    g2.add_cycle(&[1, 2, 3]);
    assert!(g2.has_edge(3, 1));
    assert!(!g2.has_edge(1, 3));

    let mut g3 = DirectedGraph::new();
    g3.add_star(&[0, 1, 2, 3]);
    assert!(g3.has_edge(0, 1));
    assert!(g3.has_edge(0, 2));
    assert!(g3.has_edge(0, 3));
    assert!(!g3.has_edge(1, 0));
}

#[test]
fn single_node_cycle_is_a_selfloop() {
    let mut g = DirectedGraph::new();
    let ids = g.add_cycle(&[7]);
    assert_eq!(ids.len(), 1);
    assert!(g.has_edge(7, 7));
}
// #endsubregion

// #subregion AllNeighbors
#[test]
fn all_neighbors_is_union_of_predecessors_and_successors() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(c, a);
    let mut all: Vec<NodeId> = g.all_neighbors(a).collect();
    all.sort();
    assert_eq!(all, vec![b, c]);
}
// #endsubregion

// #subregion Attributes
#[test]
fn node_and_edge_attributes_round_trip() {
    let mut g = DirectedGraph::new();
    let (a, b) = (g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.set_node_attributes("color", [(a, PropertyValue::String("red".to_string()))]);
    g.set_edge_attributes("weight", [((a, b), PropertyValue::Number(3.0))]);
    assert_eq!(g.get_node_attributes("color").get(&a), Some(&PropertyValue::String("red".to_string())));
    assert_eq!(g.get_edge_attributes("weight").get(&(a, b)), Some(&PropertyValue::Number(3.0)));
    g.set_name("demo");
    assert_eq!(g.name(), "demo");
}

#[test]
fn name_defaults_to_empty_string() {
    let g = DirectedGraph::new();
    assert_eq!(g.name(), "");
}

#[test]
fn get_node_and_edge_attributes_skip_entries_without_the_key() {
    let mut g = DirectedGraph::new();
    let (a, b) = (g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.set_node_attributes("color", [(a, PropertyValue::String("red".to_string()))]);
    assert!(!g.get_node_attributes("color").contains_key(&b));
    assert!(g.get_edge_attributes("weight").is_empty());
}

#[test]
fn set_node_and_edge_attributes_skip_missing_ids() {
    let mut g = DirectedGraph::new();
    let a = g.add_node();
    g.set_node_attributes("color", [(999, PropertyValue::String("red".to_string()))]);
    assert!(g.get_node_attributes("color").is_empty());
    g.set_edge_attributes("weight", [((a, 999), PropertyValue::Number(1.0))]);
    assert!(g.get_edge_attributes("weight").is_empty());
}
// #endsubregion

// #subregion NodeCrud
#[test]
fn add_node_with_attrs_and_merge_by_id() {
    let mut g = DirectedGraph::new();
    let mut attrs = PropertyBag::new();
    attrs.insert("kind".to_string(), PropertyValue::String("router".to_string()));
    let a = g.add_node_with(attrs);
    assert!(g.has_node(a));

    let mut more = PropertyBag::new();
    more.insert("region".to_string(), PropertyValue::String("eu".to_string()));
    let id = g.add_node_with_id(a, more);
    assert_eq!(id, a);
    assert_eq!(g.get_node_attributes("kind").get(&a), Some(&PropertyValue::String("router".to_string())));
    assert_eq!(g.get_node_attributes("region").get(&a), Some(&PropertyValue::String("eu".to_string())));
}

#[test]
fn add_nodes_from_and_remove_nodes_from() {
    let mut g = DirectedGraph::new();
    g.add_nodes_from([1, 2, 3]);
    assert_eq!(g.number_of_nodes(), 3);
    assert_eq!(g.order(), 3);
    let mut ids: Vec<NodeId> = g.nodes().collect();
    ids.sort();
    assert_eq!(ids, vec![1, 2, 3]);

    g.add_edge(1, 2);
    g.remove_nodes_from([1, 3]);
    assert_eq!(g.number_of_nodes(), 1);
    assert!(g.has_node(2));
    assert!(!g.has_edge(1, 2));
}

#[test]
fn remove_node_cascades_incident_edges() {
    let mut g = DirectedGraph::new();
    let (a, b) = (g.add_node(), g.add_node());
    g.add_edge(a, b);
    assert!(g.remove_node(a));
    assert!(!g.has_edge(a, b));
    assert_eq!(g.number_of_edges(), 0);
    assert!(!g.remove_node(a));
}
// #endsubregion

// #subregion EdgeBulk
#[test]
fn add_edges_from_and_weighted_edges_from() {
    let mut g = DirectedGraph::new();
    let ids = g.add_edges_from([(1, 2), (2, 3)]);
    assert_eq!(ids.len(), 2);
    assert_eq!(g.number_of_edges(), 2);

    let mut g2 = DirectedGraph::new();
    g2.add_weighted_edges_from([(1, 2, 2.5), (2, 3, 1.5)]);
    assert_eq!(g2.size(false), 2.0);
    assert_eq!(g2.size(true), 4.0);
    assert_eq!(g2.get_edge_data(1, 2).and_then(|a| a.get("weight")).cloned(), Some(PropertyValue::Number(2.5)));
}

#[test]
fn get_edge_data_is_none_for_missing_pair() {
    let mut g = DirectedGraph::new();
    let (a, b) = (g.add_node(), g.add_node());
    assert!(g.get_edge_data(a, b).is_none());
}
// #endsubregion

// #subregion BuildersEdgeCases
#[test]
fn single_node_path_adds_isolated_node_without_edges() {
    let mut g = DirectedGraph::new();
    let ids = g.add_path(&[5]);
    assert!(ids.is_empty());
    assert!(g.has_node(5));
    assert_eq!(g.number_of_edges(), 0);
}

#[test]
fn star_with_only_center_or_no_nodes_adds_no_edges() {
    let mut g = DirectedGraph::new();
    let ids = g.add_star(&[9]);
    assert!(ids.is_empty());
    assert!(g.has_node(9));

    let mut empty = DirectedGraph::new();
    assert!(empty.add_star(&[]).is_empty());
    assert_eq!(empty.number_of_nodes(), 0);
}
// #endsubregion

// #subregion InOutEdgesEmpty
#[test]
fn in_edges_and_out_edges_are_direction_filtered() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(c, b);
    let out_of_a: Vec<NodeId> = g.out_edges(a).map(|e| e.v).collect();
    assert_eq!(out_of_a, vec![b]);
    let mut into_b: Vec<NodeId> = g.in_edges(b).map(|e| e.u).collect();
    into_b.sort();
    assert_eq!(into_b, vec![a, c]);
}

#[test]
fn is_empty_reflects_edge_count_not_node_count() {
    let mut g = DirectedGraph::new();
    assert!(g.is_empty());
    g.add_node();
    assert!(g.is_empty());
    let (a, b) = (g.add_node(), g.add_node());
    g.add_edge(a, b);
    assert!(!g.is_empty());
}
// #endsubregion

// #subregion CopyClear
#[test]
fn copy_is_independent_of_original() {
    let mut g = DirectedGraph::new();
    let (a, b) = (g.add_node(), g.add_node());
    g.add_edge(a, b);
    let mut c = g.copy();
    c.add_edge(b, a);
    assert!(!g.has_edge(b, a));
    assert!(c.has_edge(b, a));
}

#[test]
fn clear_removes_everything_clear_edges_keeps_nodes() {
    let mut g = DirectedGraph::new();
    let (a, b) = (g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.set_name("demo");
    g.clear_edges();
    assert_eq!(g.number_of_edges(), 0);
    assert_eq!(g.number_of_nodes(), 2);
    assert_eq!(g.name(), "demo");

    g.clear();
    assert_eq!(g.number_of_nodes(), 0);
    assert_eq!(g.name(), "");
}
// #endsubregion

// #subregion PathHelpersCoverage
#[test]
fn is_path_rejects_missing_node_and_wrong_direction() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(b, c);
    assert!(g.is_path(&[a, b, c]));
    assert!(!g.is_path(&[c, b, a]));
    assert!(!g.is_path(&[a, b, 999]));
}

#[test]
fn path_weight_sums_weights_and_is_none_off_path() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    let mut w1 = PropertyBag::new();
    w1.insert("weight".to_string(), PropertyValue::Number(2.0));
    g.add_edge_with(a, b, w1);
    let mut w2 = PropertyBag::new();
    w2.insert("weight".to_string(), PropertyValue::Number(3.0));
    g.add_edge_with(b, c, w2);
    assert_eq!(g.path_weight(&[a, b, c]), Some(5.0));
    assert_eq!(g.path_weight(&[c, b, a]), None);
}

#[test]
fn common_neighbors_uses_successors_only() {
    let mut g = DirectedGraph::new();
    let (a, b, c, d) = (g.add_node(), g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, c);
    g.add_edge(b, c);
    g.add_edge(a, d);
    let mut common: Vec<NodeId> = g.common_neighbors(a, b).collect();
    common.sort();
    assert_eq!(common, vec![c]);
}

#[test]
fn non_neighbors_excludes_self_and_successors() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    let mut non: Vec<NodeId> = g.non_neighbors(a).collect();
    non.sort();
    assert_eq!(non, vec![c]);
}

#[test]
fn non_edges_lists_every_missing_ordered_pair() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    let mut pairs: Vec<(NodeId, NodeId)> = g.non_edges().collect();
    pairs.sort();
    let mut expected = vec![(a, c), (b, a), (b, c), (c, a), (c, b)];
    expected.sort();
    assert_eq!(pairs, expected);
}
// #endsubregion

// #subregion TraitImpls
#[test]
fn graph_view_trait_methods_delegate_to_storage() {
    let mut g = DirectedGraph::new();
    let (a, b, c) = (g.add_node(), g.add_node(), g.add_node());
    g.add_edge(a, b);
    g.add_edge(b, c);

    assert_eq!(GraphView::node_count(&g), 3);
    assert_eq!(GraphView::edge_count(&g), 2);
    assert!(GraphView::contains_node(&g, a));
    assert!(!GraphView::contains_node(&g, 999));
    assert_eq!(GraphView::edges(&g).count(), 2);
    assert_eq!(GraphView::degree(&g, b), 2);
    assert_eq!(GraphView::out_degree(&g, b), 1);
    assert_eq!(GraphView::in_degree(&g, b), 1);
    assert!(GraphView::is_directed(&g));
    assert!(!GraphView::is_multigraph(&g));
    assert_eq!(GraphView::edges_between(&g, a, b).count(), 1);
    let neighbors: Vec<NodeId> = GraphView::neighbors(&g, b).collect();
    assert_eq!(neighbors, vec![c]);
    let out: Vec<NodeId> = GraphView::out_neighbors(&g, a).collect();
    assert_eq!(out, vec![b]);
    let inn: Vec<NodeId> = GraphView::in_neighbors(&g, b).collect();
    assert_eq!(inn, vec![a]);
}

#[test]
fn attr_view_and_edge_weights_trait_methods() {
    let mut g = DirectedGraph::new();
    let a = g.add_node();
    let b = g.add_node();
    let mut w = PropertyBag::new();
    w.insert("weight".to_string(), PropertyValue::Number(4.0));
    g.add_edge_with(a, b, w);
    g.set_name("t");

    assert!(AttrView::node_attrs(&g, a).is_some());
    assert!(AttrView::node_attrs(&g, 999).is_none());
    let edge = g.edges().next().expect("edge exists");
    assert!(AttrView::edge_attrs(&g, edge.id).is_some());
    assert_eq!(AttrView::graph_attrs(&g).get("name"), Some(&PropertyValue::String("t".to_string())));
    assert_eq!(EdgeWeights::weight(&g, edge), 4.0);
}
// #endsubregion
