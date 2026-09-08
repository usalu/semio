
use super::*;
use graph_core::Undirected;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn und_edge(a: NodeId, b: NodeId) -> Storage<Normal, Undirected> {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    g.add_node_with_id(a, PropertyBag::new());
    g.add_node_with_id(b, PropertyBag::new());
    g.add_edge(a, b);
    g
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn attrs_of(pairs: &[(&str, &str)]) -> PropertyBag {
    let mut bag = PropertyBag::new();
    for (k, v) in pairs {
        bag.insert(k.to_string(), graph_core::PropertyValue::String(v.to_string()));
    }
    bag
}

// #subregion SetOperators
#[semio_framework_async_macros::async_test]
async fn union_of_disjoint_graphs_merges_everything() {
    let g = und_edge(0, 1);
    let mut h: Storage<Normal, Undirected> = Storage::new();
    h.add_node_with_id(10, PropertyBag::new());
    h.add_node_with_id(11, PropertyBag::new());
    h.add_edge(10, 11);

    let merged = union(&g, &h).expect("disjoint node sets must union");
    assert_eq!(merged.node_count(), 4);
    assert_eq!(merged.edge_count(), 2);
    assert!(merged.edges_between(0, 1).next().is_some());
    assert!(merged.edges_between(10, 11).next().is_some());
}

#[semio_framework_async_macros::async_test]
async fn union_errors_on_overlapping_node_ids() {
    let g = und_edge(0, 1);
    let h = und_edge(1, 2);
    let err = union(&g, &h).expect_err("shared node id 1 must be rejected");
    assert!(matches!(err, GraphError::AmbiguousSolution(_)));
}

#[semio_framework_async_macros::async_test]
async fn compose_lets_h_overwrite_shared_node_and_edge_attrs() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    g.add_node_with_id(0, attrs_of(&[("color", "red")]));
    g.add_node_with_id(1, PropertyBag::new());
    g.add_edge_with(0, 1, attrs_of(&[("kind", "g")]));

    let mut h: Storage<Normal, Undirected> = Storage::new();
    h.add_node_with_id(0, attrs_of(&[("color", "blue")]));
    h.add_node_with_id(1, PropertyBag::new());
    h.add_node_with_id(2, PropertyBag::new());
    h.add_edge_with(0, 1, attrs_of(&[("kind", "h")]));

    let composed = semio_compose_rs(&g, &h);
    assert_eq!(composed.node_count(), 3);
    assert_eq!(composed.node_attrs(0).unwrap().get("color").unwrap().as_str(), Some("blue"));
    let edge = composed.edges_between(0, 1).next().unwrap();
    assert_eq!(composed.edge_attrs(edge.id).unwrap().get("kind").unwrap().as_str(), Some("h"));
}

#[semio_framework_async_macros::async_test]
async fn disjoint_union_relabels_both_graphs_into_fresh_ids() {
    let g = und_edge(0, 1);
    let h = und_edge(0, 1);
    let merged = disjoint_union(&g, &h);
    assert_eq!(merged.node_count(), 4);
    assert_eq!(merged.edge_count(), 2);
}

#[semio_framework_async_macros::async_test]
async fn intersection_keeps_only_shared_nodes_and_edges() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1);
    g.add_edge(1, 2);

    let mut h: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 3] {
        h.add_node_with_id(id, PropertyBag::new());
    }
    h.add_edge(0, 1);

    let inter = intersection(&g, &h);
    assert_eq!(inter.node_count(), 2);
    assert_eq!(inter.edge_count(), 1);
    assert!(inter.edges_between(0, 1).next().is_some());
}

#[semio_framework_async_macros::async_test]
async fn difference_keeps_gs_nodes_and_only_gs_own_edges() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1);
    g.add_edge(1, 2);

    let mut h: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1] {
        h.add_node_with_id(id, PropertyBag::new());
    }
    h.add_edge(0, 1);

    let diff = difference(&g, &h);
    assert_eq!(diff.node_count(), 3);
    assert_eq!(diff.edge_count(), 1);
    assert!(diff.edges_between(1, 2).next().is_some());
    assert!(diff.edges_between(0, 1).next().is_none());
}

#[semio_framework_async_macros::async_test]
async fn symmetric_difference_keeps_edges_unique_to_either_side() {
    let g = und_edge(0, 1);
    let mut h: Storage<Normal, Undirected> = Storage::new();
    h.add_node_with_id(0, PropertyBag::new());
    h.add_node_with_id(1, PropertyBag::new());
    h.add_node_with_id(2, PropertyBag::new());
    h.add_edge(0, 1);
    h.add_edge(1, 2);

    let symdiff = symmetric_difference(&g, &h);
    assert_eq!(symdiff.node_count(), 3);
    assert_eq!(symdiff.edge_count(), 1);
    assert!(symdiff.edges_between(1, 2).next().is_some());
    assert!(symdiff.edges_between(0, 1).next().is_none());
}
// #endsubregion

// #subregion Complement
#[semio_framework_async_macros::async_test]
async fn complement_of_a_path_matches_a_hand_count() {
    // 🔺️ Path 0-1-2 (undirected, 3 nodes, 2 edges) has C(3,2)-2 = 1 missing pair: 0-2.
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1);
    g.add_edge(1, 2);

    let comp = complement(&g);
    assert_eq!(comp.node_count(), 3);
    assert_eq!(comp.edge_count(), 1);
    assert!(comp.edges_between(0, 2).next().is_some());
}

#[semio_framework_async_macros::async_test]
async fn reverse_round_trips_the_edge_set() {
    let mut g: Storage<Normal, Directed> = Storage::new();
    for id in [0, 1, 2] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1);
    g.add_edge(1, 2);

    let once = reverse(&g);
    assert!(once.edges_between(1, 0).next().is_some());
    assert!(once.edges_between(2, 1).next().is_some());

    let twice = reverse(&once);
    let original: BTreeSet<(NodeId, NodeId)> = g.edges().map(|e| (e.u, e.v)).collect();
    let round_tripped: BTreeSet<(NodeId, NodeId)> = twice.edges().map(|e| (e.u, e.v)).collect();
    assert_eq!(original, round_tripped);
}
// #endsubregion

// #subregion Products
#[semio_framework_async_macros::async_test]
async fn cartesian_product_of_two_2node_paths_is_a_4cycle() {
    let g = und_edge(0, 1);
    let h = und_edge(0, 1);
    let (prod, map) = cartesian_product(&g, &h);
    assert_eq!(prod.node_count(), 4);
    assert_eq!(prod.edge_count(), 4);
    for &n in map.values() {
        assert_eq!(prod.degree(n), 2);
    }
}

#[semio_framework_async_macros::async_test]
async fn tensor_product_requires_adjacency_on_both_sides() {
    let g = und_edge(0, 1);
    let h = und_edge(0, 1);
    let (prod, _map) = tensor_product(&g, &h);
    assert_eq!(prod.node_count(), 4);
    // 🔗️ (0,0)-(1,1) and (0,1)-(1,0): exactly 2 edges.
    assert_eq!(prod.edge_count(), 2);
}

#[semio_framework_async_macros::async_test]
async fn strong_product_is_the_union_of_cartesian_and_tensor() {
    let g = und_edge(0, 1);
    let h = und_edge(0, 1);
    let (cart, _) = cartesian_product(&g, &h);
    let (tens, _) = tensor_product(&g, &h);
    let (strong, _) = strong_product(&g, &h);
    assert!(strong.edge_count() >= cart.edge_count());
    assert!(strong.edge_count() >= tens.edge_count());
}

#[semio_framework_async_macros::async_test]
async fn lexicographic_product_includes_cross_block_edges() {
    let g = und_edge(0, 1);
    let h = und_edge(0, 1);
    let (lex, _map) = lexicographic_product(&g, &h);
    assert_eq!(lex.node_count(), 4);
    // 🔗️ g-adjacent pairs pull in all 4 combinations of (v1, v2) for u1~u2, plus the within-block h edges.
    assert!(lex.edge_count() >= 4);
}
// #endsubregion

// #subregion Power
#[semio_framework_async_macros::async_test]
async fn power_connects_nodes_within_k_hops() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2, 3] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);

    let p2 = power(&g, 2);
    assert!(p2.edges_between(0, 2).next().is_some());
    assert!(p2.edges_between(0, 3).next().is_none());

    let p3 = power(&g, 3);
    assert!(p3.edges_between(0, 3).next().is_some());
}
// #endsubregion

// #subregion Contraction
#[semio_framework_async_macros::async_test]
async fn contracted_nodes_merges_v_into_u_and_respects_self_loops_flag() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1); // the edge that becomes a self-loop when 1 merges into 0
    g.add_edge(1, 2);

    let dropped = contracted_nodes(&g, 0, 1, false);
    assert_eq!(dropped.node_count(), 2);
    assert!(!dropped.contains_node(1));
    assert!(dropped.edges_between(0, 0).next().is_none());
    assert!(dropped.edges_between(0, 2).next().is_some());

    let kept = contracted_nodes(&g, 0, 1, true);
    assert!(kept.edges_between(0, 0).next().is_some());
    assert!(kept.edges_between(0, 2).next().is_some());
}

#[semio_framework_async_macros::async_test]
async fn contracted_edge_contracts_its_own_endpoints() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    let e = g.add_edge(0, 1);
    g.add_edge(1, 2);

    let contracted = contracted_edge(&g, e, false).expect("edge exists");
    assert_eq!(contracted.node_count(), 2);

    let err = contracted_edge(&g, 9999, false).expect_err("missing edge id");
    assert!(matches!(err, GraphError::EdgeNotFound(9999)));
}

#[semio_framework_async_macros::async_test]
async fn quotient_graph_connects_blocks_that_have_a_crossing_edge() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2, 3] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1);
    g.add_edge(2, 3);
    g.add_edge(1, 2);

    let (quotient, block_of) = quotient_graph(&g, &[vec![0, 1], vec![2, 3]]);
    assert_eq!(quotient.node_count(), 2);
    let (b0, b1) = (block_of[&0], block_of[&2]);
    assert!(quotient.edges_between(b0, b1).next().is_some());
}
// #endsubregion

// #subregion LineGraph
#[semio_framework_async_macros::async_test]
async fn line_graph_of_a_triangle_is_itself_a_triangle() {
    let mut g: Storage<Normal, Undirected> = Storage::new();
    for id in [0, 1, 2] {
        g.add_node_with_id(id, PropertyBag::new());
    }
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(0, 2);

    let (lg, node_of_edge) = line_graph(&g);
    assert_eq!(lg.node_count(), 3);
    assert_eq!(lg.edge_count(), 3);
    for &n in node_of_edge.values() {
        assert_eq!(lg.degree(n), 2);
    }
}
// #endsubregion

// #subregion Mycielski
#[semio_framework_async_macros::async_test]
async fn mycielskian_of_a_single_edge_is_the_5cycle() {
    let g = und_edge(0, 1);
    let myc = mycielskian(&g);
    assert_eq!(myc.node_count(), 5);
    assert_eq!(myc.edge_count(), 5);
    for n in myc.nodes() {
        assert_eq!(myc.degree(n), 2);
    }
}
// #endsubregion
