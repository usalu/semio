
use super::*;

/// 🧬️ Additive `#[derive(ToValue, FromValue)]` round-trip (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01): `FromValue(ToValue(x)) == x`, covering plain derives, the hand-written
/// zero-field marker types (`Directed`/`Normal`/…), `GraphError::NotImplementedForKind`'s
/// `String` fields, the `geometry::Point` bridge (`Node`/`Handle`), and a POPULATED `Storage`
/// (so its four `u64`-keyed `BTreeMap` bridges — `nodes`/`edges`/`successors`/`handle_owner` —
/// are all actually exercised, not just compiled).
#[test]
fn value_round_trip_matches_serde_shape() {
    fn check<T: dsl_core::ToValue + dsl_core::FromValue + std::fmt::Debug + PartialEq>(value: T) {
        let round_tripped = <T as dsl_core::FromValue>::from_value(dsl_core::ToValue::to_value(&value)).expect("round-trip decode");
        assert_eq!(round_tripped, value);
    }

    // Zero-field marker types have no `PartialEq` (pre-existing), so their round-trip is just
    // "decodes without error and re-encodes identically" rather than `check`'s equality form.
    for encoded in [dsl_core::ToValue::to_value(&Directed), dsl_core::ToValue::to_value(&Undirected), dsl_core::ToValue::to_value(&Normal), dsl_core::ToValue::to_value(&Ported), dsl_core::ToValue::to_value(&UnitWeight)] {
        assert_eq!(encoded, dsl_core::DslValue::Null);
    }
    <Directed as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <Undirected as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <Normal as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <Ported as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");
    <UnitWeight as dsl_core::FromValue>::from_value(dsl_core::DslValue::Null).expect("decode");

    check(EdgeRef { id: 3, u: 1, v: 2 });
    check(GraphError::NodeNotFound(7));
    check(GraphError::NotImplementedForKind { algorithm: "planarity".to_string(), kind: "multigraph".to_string() });
    check(Node { id: 1, center: Point::new(1.5, -2.5), radius: 3.0, width: 4.0, height: 5.0, shape: NodeShape::Rectangle, draggable: true, kind: Some("box".to_string()), label: None, properties: PropertyBag::new() });
    check(Handle { angle: 0.5, id: 9, node_id: 1, radius: 2.0, role: HandleRole::Source, kind: None, properties: PropertyBag::new() });

    let mut storage: Storage<Ported, Directed> = Storage::default();
    let n0 = storage.add_node();
    let n1 = storage.add_node();
    storage.add_handle(n0);
    let h1 = storage.add_handle(n1).expect("handle");
    let h0 = storage.add_handle(n0).expect("handle");
    storage.add_edge(h0, h1);
    let encoded = dsl_core::ToValue::to_value(&storage);
    let decoded = <Storage<Ported, Directed> as dsl_core::FromValue>::from_value(encoded.clone()).expect("round-trip decode");
    assert_eq!(dsl_core::ToValue::to_value(&decoded), encoded);

    let csr = Csr::from_view(&storage);
    let csr_encoded = dsl_core::ToValue::to_value(&csr);
    let csr_decoded = <Csr as dsl_core::FromValue>::from_value(csr_encoded.clone()).expect("round-trip decode");
    assert_eq!(dsl_core::ToValue::to_value(&csr_decoded), csr_encoded);
}

// 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
// an `async fn` directly (std has no executor for it), so every async test body in this
// module runs through this instead. Sound because this crate performs no real I/O: every
// future here resolves on its first poll, so a single poll (never a spin-park loop) is
// enough — panics loudly if that invariant is ever violated rather than hanging.
fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop(_: *const ()) {}
    fn clone_raw(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
    }
}

type NU = Storage<Normal, Undirected>;
type ND = Storage<Normal, Directed>;
type PU = Storage<Ported, Undirected>;
type PD = Storage<Ported, Directed>;

// #subregion Storage
#[test]
fn add_node_allocates_monotone_ids() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(g.node_count(), 2);
    });
}

#[test]
fn add_node_with_id_upserts_attrs_and_bumps_allocator() {
    block_on_test(async {
        let mut g = NU::new();
        let mut attrs = PropertyBag::new();
        attrs.insert("color".into(), PropertyValue::String("red".into()));
        g.add_node_with_id(5, attrs);
        assert!(g.contains_node(5));
        let next = g.add_node();
        assert_eq!(next, 6, "auto id must skip past the caller-supplied id");

        let mut more = PropertyBag::new();
        more.insert("size".into(), PropertyValue::Number(3.0));
        g.add_node_with_id(5, more);
        let record = g.node_attrs(5).expect("node 5 exists");
        assert_eq!(record.get("color").and_then(PropertyValue::as_str), Some("red"));
        assert_eq!(record.get("size").and_then(PropertyValue::as_f64), Some(3.0));
    });
}

#[test]
fn remove_node_cascades_edges_and_handles() {
    block_on_test(async {
        let mut g = PU::new();
        let a = g.add_node();
        let b = g.add_node();
        let ha = g.add_handle(a).expect("ported storage grants handles");
        let hb = g.add_handle(b).expect("ported storage grants handles");
        let e = g.add_edge(ha, hb);
        assert!(g.remove_node(a));
        assert!(!g.contains_node(a));
        assert!(g.edge_endpoints(e).is_none(), "incident edge must be cascaded away");
        assert!(g.handle_owner(ha).is_none(), "handle on the removed node must be cascaded away");
        assert_eq!(g.handles(b), &[hb]);
    });
}

#[test]
fn normal_add_edge_upserts_instead_of_duplicating() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut first = PropertyBag::new();
        first.insert("weight".into(), PropertyValue::Number(1.0));
        let e1 = g.add_edge_with(a, b, first);
        let mut second = PropertyBag::new();
        second.insert("label".into(), PropertyValue::String("x".into()));
        let e2 = g.add_edge_with(a, b, second);
        assert_eq!(e1, e2, "Normal storages upsert an existing pair instead of creating a parallel edge");
        assert_eq!(g.edge_count(), 1);
        let attrs = g.edge_attrs(e1).expect("edge exists");
        assert_eq!(attrs.get("weight").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(attrs.get("label").and_then(PropertyValue::as_str), Some("x"));
    });
}

#[test]
fn ported_add_edge_always_creates_parallel_edges() {
    block_on_test(async {
        let mut g = PD::new();
        let a = g.add_node();
        let b = g.add_node();
        let ha = g.add_handle(a).expect("ported");
        let hb = g.add_handle(b).expect("ported");
        let e1 = g.add_edge(ha, hb);
        let e2 = g.add_edge(ha, hb);
        assert_ne!(e1, e2, "Ported storages always create a fresh parallel edge");
        assert_eq!(g.edge_count(), 2);
        assert_eq!(g.out_degree(a), 2);
    });
}

#[test]
fn normal_storage_denies_handles() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        assert!(g.add_handle(a).is_none());
        assert!(g.handles(a).is_empty());
    });
}

#[test]
fn remove_edge_unlinks_adjacency_both_ways_when_undirected() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        assert!(g.remove_edge(e));
        assert_eq!(g.out_degree(a), 0);
        assert_eq!(g.out_degree(b), 0);
        assert!(g.edges_between(a, b).next().is_none());
    });
}

#[test]
fn clear_edges_keeps_nodes_clear_removes_everything() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.clear_edges();
        assert_eq!(g.node_count(), 2);
        assert_eq!(g.edge_count(), 0);
        g.clear();
        assert_eq!(g.node_count(), 0);
    });
}

#[test]
fn remove_edge_and_remove_node_return_false_for_unknown_ids() {
    block_on_test(async {
        let mut g = NU::new();
        assert!(!g.remove_edge(999), "removing a never-created edge id must fail cleanly");
        assert!(!g.remove_node(999), "removing a never-created node id must fail cleanly");
    });
}

#[test]
fn node_attrs_mut_and_edge_attrs_mut_edit_in_place_and_are_none_for_unknown_ids() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        g.node_attrs_mut(a).expect("node exists").insert("k".into(), PropertyValue::Number(1.0));
        g.edge_attrs_mut(e).expect("edge exists").insert("w".into(), PropertyValue::Number(2.0));
        assert_eq!(g.node_attrs(a).unwrap().get("k").and_then(PropertyValue::as_f64), Some(1.0));
        assert_eq!(g.edge_attrs(e).unwrap().get("w").and_then(PropertyValue::as_f64), Some(2.0));
        assert!(g.node_attrs_mut(999).is_none());
        assert!(g.edge_attrs_mut(999).is_none());
    });
}

#[test]
fn add_handle_denies_missing_node_and_handle_owner_is_none_for_unknown_handle() {
    block_on_test(async {
        let mut g = PU::new();
        assert!(g.add_handle(999).is_none(), "cannot anchor a handle on a node that doesn't exist");
        assert!(g.handle_owner(999).is_none());
    });
}

#[test]
fn core_edge_normalize_undirected_orders_the_pair() {
    block_on_test(async {
        assert_eq!(CoreEdge::<u64>::normalize_undirected(5, 2), (2, 5));
        assert_eq!(CoreEdge::<u64>::normalize_undirected(2, 5), (2, 5));
    });
}
// #endsubregion

// #subregion GraphView
#[test]
fn undirected_self_loop_counts_twice_towards_degree() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.degree(a), 2);
        assert_eq!(g.edge_count(), 1, "edges() still lists the self-loop once");
        assert_eq!(g.edges_between(a, a).count(), 2);
    });
}

#[test]
fn directed_degree_is_in_plus_out() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(c, a);
        assert_eq!(g.out_degree(a), 1);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 2);
        assert_eq!(GraphView::neighbors(&g, a).collect::<Vec<_>>(), vec![b], "neighbors == out_neighbors for directed storages");
    });
}

#[test]
fn undirected_in_neighbors_equals_out_neighbors() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let out: Vec<_> = g.out_neighbors(a).collect();
        let inn: Vec<_> = g.in_neighbors(a).collect();
        assert_eq!(out, inn);
    });
}

#[test]
fn is_directed_and_is_multigraph_reflect_type_axes() {
    block_on_test(async {
        assert!(!NU::new().is_directed());
        assert!(ND::new().is_directed());
        assert!(!NU::new().is_multigraph());
        assert!(PU::new().is_multigraph());
    });
}

#[test]
fn directed_self_loop_counts_once_each_towards_out_and_in_degree() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        g.add_edge(a, a);
        assert_eq!(g.out_degree(a), 1);
        assert_eq!(g.in_degree(a), 1);
        assert_eq!(g.degree(a), 2);
    });
}
// #endsubregion

// #subregion EdgeWeights
#[test]
fn unit_weight_is_always_one() {
    block_on_test(async {
        let w = UnitWeight;
        assert_eq!(w.weight(EdgeRef { id: 0, u: 0, v: 1 }), 1.0);
    });
}

#[test]
fn storage_default_weight_reads_weight_attr_with_fallback() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs = PropertyBag::new();
        attrs.insert("weight".into(), PropertyValue::Number(4.5));
        let e = g.add_edge_with(a, b, attrs);
        let edge_ref = EdgeRef { id: e, u: a, v: b };
        assert_eq!(g.weight(edge_ref), 4.5);

        let e2 = g.add_edge(b, a);
        assert_eq!(e2, e, "Normal upsert must keep returning the same edge id");

        let mut g2 = NU::new();
        let x = g2.add_node();
        let y = g2.add_node();
        let unweighted_edge = g2.add_edge(x, y);
        assert_eq!(g2.weight(EdgeRef { id: unweighted_edge, u: x, v: y }), 1.0);
    });
}

#[test]
fn attr_weight_falls_back_to_default_when_missing_or_non_numeric() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let mut attrs = PropertyBag::new();
        attrs.insert("cost".into(), PropertyValue::String("not-a-number".into()));
        let e = g.add_edge_with(a, b, attrs);
        let aw = AttrWeight { graph: &g, name: "cost", default: 2.0 };
        assert_eq!(aw.weight(EdgeRef { id: e, u: a, v: b }), 2.0);
    });
}

#[test]
fn closure_implements_edge_weights() {
    block_on_test(async {
        let double = |edge: EdgeRef| (edge.id as f64) * 2.0;
        assert_eq!(double.weight(EdgeRef { id: 3, u: 0, v: 1 }), 6.0);
    });
}
// #endsubregion

// #subregion Csr
#[test]
fn csr_from_view_preserves_directed_adjacency() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, c);
        let csr = Csr::from_view(&g);
        assert_eq!(csr.node_count(), 3);
        let ia = csr.index_of(a).expect("a indexed");
        let ib = csr.index_of(b).expect("b indexed");
        let ic = csr.index_of(c).expect("c indexed");
        let mut out: Vec<usize> = csr.out_neighbors(ia).to_vec();
        out.sort_unstable();
        let mut expected = vec![ib, ic];
        expected.sort_unstable();
        assert_eq!(out, expected);
        assert_eq!(csr.node_of(ia), Some(a));
        assert!(csr.in_neighbors(ib).contains(&ia));
    });
}

#[test]
fn csr_from_view_mirrors_undirected_edges_both_ways() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let csr = Csr::from_view(&g);
        let ia = csr.index_of(a).unwrap();
        let ib = csr.index_of(b).unwrap();
        assert!(csr.out_neighbors(ia).contains(&ib));
        assert!(csr.out_neighbors(ib).contains(&ia));
    });
}

#[test]
fn csr_out_edges_and_unknown_ids_return_none() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let csr = Csr::from_view(&g);
        let ia = csr.index_of(a).unwrap();
        assert_eq!(csr.out_edges(ia), &[e]);
        assert_eq!(csr.node_of(999), None);
        assert_eq!(csr.index_of(999), None);
    });
}
// #endsubregion

// #subregion Views
#[test]
fn subgraph_view_drops_edges_leaving_the_subset() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let sub = SubgraphView::new(&g, [a, b]);
        assert_eq!(sub.node_count(), 2);
        assert_eq!(sub.edge_count(), 1);
        assert!(!sub.contains_node(c));
    });
}

#[test]
fn edge_subgraph_view_nodes_are_exactly_edge_endpoints() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_node(); // isolated node d, never referenced by an edge
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);
        let view = EdgeSubgraphView::new(&g, [e_ab]);
        let mut nodes: Vec<_> = view.nodes().collect();
        nodes.sort_unstable();
        assert_eq!(nodes, vec![a, b]);
        assert_eq!(view.edge_count(), 1);
    });
}

#[test]
fn subgraph_view_degree_counts_only_edges_within_subset() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(a, c);
        let sub = SubgraphView::new(&g, [a, b]);
        assert_eq!(sub.out_degree(a), 1, "the edge to c falls outside the node subset");
        assert_eq!(sub.in_degree(b), 1);
        assert_eq!(sub.degree(a), sub.out_degree(a) + sub.in_degree(a), "directed subgraph degree is out+in");
        assert!(sub.is_directed());
        assert!(!sub.is_multigraph());
    });
}

#[test]
fn subgraph_view_attr_view_hides_attrs_outside_the_node_subset() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let sub = SubgraphView::new(&g, [a]);
        assert!(sub.node_attrs(a).is_some());
        assert!(sub.node_attrs(b).is_none(), "b is outside the node subset");
        assert!(sub.edge_attrs(e).is_some(), "edge attrs are not filtered by SubgraphView");
        assert!(std::ptr::eq(sub.graph_attrs(), g.graph_attrs()));
    });
}

#[test]
fn edge_subgraph_view_degree_and_directed_flag() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        let e_ab = g.add_edge(a, b);
        g.add_edge(b, c);
        let view = EdgeSubgraphView::new(&g, [e_ab]);
        assert!(view.is_directed());
        assert_eq!(view.out_degree(a), 1);
        assert_eq!(view.in_degree(b), 1);
        assert_eq!(view.degree(a), 1);
        assert!(view.edge_attrs(e_ab).is_some());
    });
}

#[test]
fn edge_subgraph_view_undirected_in_neighbors_matches_out_neighbors() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = EdgeSubgraphView::new(&g, [e]);
        assert!(!view.is_directed());
        assert_eq!(view.in_neighbors(a).collect::<Vec<_>>(), view.out_neighbors(a).collect::<Vec<_>>());
        assert_eq!(view.degree(a), view.out_degree(a));
    });
}

#[test]
fn reversed_view_swaps_direction_on_directed_graph() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.out_neighbors(b).collect::<Vec<_>>(), vec![a]);
        assert_eq!(rev.in_neighbors(a).collect::<Vec<_>>(), vec![b]);
    });
}

#[test]
fn reversed_view_is_a_no_op_on_undirected_graph() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.out_neighbors(a).collect::<Vec<_>>(), g.out_neighbors(a).collect::<Vec<_>>());
    });
}

#[test]
fn reversed_view_edges_and_edges_between_swap_endpoints() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let rev = ReversedView::new(&g);
        assert_eq!(rev.edges().collect::<Vec<_>>(), vec![EdgeRef { id: e, u: b, v: a }]);
        assert_eq!(rev.edges_between(b, a).next(), Some(EdgeRef { id: e, u: b, v: a }));
        assert_eq!(rev.degree(a), g.degree(a));
        assert_eq!(rev.is_multigraph(), g.is_multigraph());
    });
}

#[test]
fn filtered_view_keep_predicate_hides_by_inversion() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let c = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, c);
        let hidden: BTreeSet<NodeId> = [b].into_iter().collect();
        let view = FilteredView::new(&g, |n| !hidden.contains(&n), |_e| true);
        assert!(view.contains_node(a));
        assert!(!view.contains_node(b));
        assert_eq!(view.edge_count(), 0, "both edges touch the hidden node b");
    });
}

#[test]
fn filtered_view_keep_edge_predicate_hides_specific_edges_without_hiding_nodes() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e_bad = g.add_edge(a, b);
        let view = FilteredView::new(&g, |_n| true, move |e| e.id != e_bad);
        assert!(view.contains_node(a));
        assert!(view.contains_node(b));
        assert_eq!(view.edge_count(), 0);
        assert_eq!(view.out_degree(a), 0);
        assert_eq!(view.degree(a), 0);
    });
}

#[test]
fn filtered_view_attr_view_delegates_edge_and_graph_attrs() {
    block_on_test(async {
        let mut g = NU::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = FilteredView::new(&g, |_n| true, |_e| true);
        assert!(view.edge_attrs(e).is_some());
        assert!(std::ptr::eq(view.graph_attrs(), g.graph_attrs()));
    });
}

#[test]
fn undirected_view_merges_successors_and_predecessors() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        let view = UndirectedView::new(&g);
        assert!(!view.is_directed());
        assert_eq!(view.neighbors(a).collect::<Vec<_>>(), vec![b]);
        assert_eq!(view.neighbors(b).collect::<Vec<_>>(), vec![a]);
    });
}

#[test]
fn undirected_view_degree_and_edges_between_merge_both_directions() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        g.add_edge(a, b);
        g.add_edge(b, a);
        let view = UndirectedView::new(&g);
        assert_eq!(view.degree(a), 2, "both directed edges count towards undirected degree");
        assert_eq!(view.edges_between(a, b).count(), 2);
        assert_eq!(view.is_multigraph(), g.is_multigraph());
    });
}

#[test]
fn undirected_view_edges_normalizes_endpoint_order() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(b, a);
        let view = UndirectedView::new(&g);
        assert_eq!(view.edges().collect::<Vec<_>>(), vec![EdgeRef { id: e, u: a, v: b }], "edges() orders endpoints u <= v regardless of storage direction");
    });
}

#[test]
fn undirected_view_attr_view_delegates_to_parent() {
    block_on_test(async {
        let mut g = ND::new();
        let a = g.add_node();
        let b = g.add_node();
        let e = g.add_edge(a, b);
        let view = UndirectedView::new(&g);
        assert!(view.node_attrs(a).is_some());
        assert!(view.edge_attrs(e).is_some());
        assert!(std::ptr::eq(view.graph_attrs(), g.graph_attrs()));
    });
}
// #endsubregion

// #subregion Interner
#[test]
fn interner_intern_is_idempotent() {
    block_on_test(async {
        let mut interner: Interner<String> = Interner::new();
        let a1 = interner.intern("alpha".to_string());
        let a2 = interner.intern("alpha".to_string());
        let b = interner.intern("beta".to_string());
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        assert_eq!(interner.label_of(a1), Some(&"alpha".to_string()));
        assert_eq!(interner.id_of(&"beta".to_string()), Some(b));
        assert_eq!(interner.len(), 2);
    });
}

#[test]
fn interner_from_labels_is_sorted_and_deduplicated() {
    block_on_test(async {
        let interner: Interner<String> = Interner::from_labels(["c".to_string(), "a".to_string(), "a".to_string(), "b".to_string()]);
        assert_eq!(interner.len(), 3);
        assert_eq!(interner.label_of(0), Some(&"a".to_string()));
        assert_eq!(interner.label_of(1), Some(&"b".to_string()));
        assert_eq!(interner.label_of(2), Some(&"c".to_string()));
    });
}

#[test]
fn interner_is_empty_and_unknown_lookups_return_none() {
    block_on_test(async {
        let mut interner: Interner<String> = Interner::new();
        assert!(interner.is_empty());
        assert_eq!(interner.label_of(0), None);
        assert_eq!(interner.id_of(&"ghost".to_string()), None);
        interner.intern("alpha".to_string());
        assert!(!interner.is_empty());
    });
}
// #endsubregion

// #subregion GraphError
#[test]
fn graph_error_display_reads_clearly() {
    assert_eq!(GraphError::NodeNotFound(7).to_string(), "node 7 not found");
    assert_eq!(GraphError::NoPath { source: 1, target: 2 }.to_string(), "no path from node 1 to node 2");
    assert_eq!(GraphError::NotImplementedForKind { algorithm: "planarity".to_string(), kind: "multigraph".to_string() }.to_string(), "planarity is not implemented for multigraph");
}

#[test]
fn graph_error_is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(GraphError::HasACycle);
    assert_eq!(err.to_string(), "graph has a cycle");
}

#[test]
fn graph_error_display_covers_remaining_variants() {
    assert_eq!(GraphError::EdgeNotFound(3).to_string(), "edge 3 not found");
    assert_eq!(GraphError::NoCycle.to_string(), "graph has no cycle");
    assert_eq!(GraphError::Unfeasible("x".into()).to_string(), "unfeasible: x");
    assert_eq!(GraphError::Unbounded("y".into()).to_string(), "unbounded: y");
    assert_eq!(GraphError::NotATree.to_string(), "graph is not a tree");
    assert_eq!(GraphError::NotAForest.to_string(), "graph is not a forest");
    assert_eq!(GraphError::NotBipartite.to_string(), "graph is not bipartite");
    assert_eq!(GraphError::NotPlanar.to_string(), "graph is not planar");
    assert_eq!(GraphError::NotEulerian.to_string(), "graph is not eulerian");
    assert_eq!(GraphError::NotConnected.to_string(), "graph is not connected");
    assert_eq!(GraphError::NotStronglyConnected.to_string(), "graph is not strongly connected");
    assert_eq!(GraphError::AmbiguousSolution("z".into()).to_string(), "ambiguous solution: z");
    assert_eq!(GraphError::ExceededMaxIterations { iterations: 5 }.to_string(), "exceeded max iterations (5)");
    assert_eq!(GraphError::PowerIterationFailedConvergence { iterations: 8 }.to_string(), "power iteration failed to converge after 8 iterations");
    assert_eq!(GraphError::NegativeCycle.to_string(), "graph has a negative cycle");
    assert_eq!(GraphError::NotGraphical("odd sum".into()).to_string(), "not a graphical degree sequence: odd sum");
    assert_eq!(GraphError::Io("disk full".into()).to_string(), "io error: disk full");
    assert_eq!(GraphError::Parse { line: 4, message: "bad token".into() }.to_string(), "parse error at line 4: bad token");
}
// #endsubregion

// #subregion Utils
#[test]
fn pairwise_yields_consecutive_pairs() {
    let items = [1, 2, 3, 4];
    assert_eq!(pairwise(&items).collect::<Vec<_>>(), vec![(1, 2), (2, 3), (3, 4)]);
}

#[test]
fn arbitrary_element_is_deterministic() {
    block_on_test(async {
        assert_eq!(arbitrary_element(&[9, 1, 2]), Some(9));
        assert_eq!(arbitrary_element::<i32>(&[]), None);
    });
}

#[test]
fn tolerance_constants_are_ordered() {
    const { assert!(TOL_STRICT < TOL_LOOSE) };
}

#[test]
fn mapped_heap_pops_in_ascending_priority_order() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("c", 30);
        heap.push_or_decrease("a", 10);
        heap.push_or_decrease("b", 20);
        assert_eq!(heap.pop_min(), Some((10, "a")));
        assert_eq!(heap.pop_min(), Some((20, "b")));
        assert_eq!(heap.pop_min(), Some((30, "c")));
        assert_eq!(heap.pop_min(), None);
    });
}

#[test]
fn mapped_heap_decrease_key_reorders() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("a", 10);
        heap.push_or_decrease("b", 20);
        assert!(heap.decrease_key(&"b", 5));
        assert!(!heap.decrease_key(&"b", 100), "raising priority via decrease_key is a no-operation");
        assert_eq!(heap.pop_min(), Some((5, "b")));
        assert!(heap.contains(&"a"));
        assert!(!heap.contains(&"b"));
    });
}

#[test]
fn mapped_heap_len_and_is_empty_track_size() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.len(), 0);
        heap.push_or_decrease("a", 5);
        assert!(!heap.is_empty());
        assert_eq!(heap.len(), 1);
    });
}

#[test]
fn mapped_heap_push_or_decrease_ignores_higher_or_equal_priority() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        heap.push_or_decrease("a", 5);
        heap.push_or_decrease("a", 10);
        assert_eq!(heap.len(), 1, "a higher priority for an already-present item must be a no-operation");
        heap.push_or_decrease("a", 5);
        assert_eq!(heap.pop_min(), Some((5, "a")), "priority must stay at the lowest value ever pushed");
    });
}

#[test]
fn decrease_key_returns_false_for_absent_item() {
    block_on_test(async {
        let mut heap: MappedHeap<i64, &str> = MappedHeap::new();
        assert!(!heap.decrease_key(&"missing", 1));
    });
}
// #endsubregion

// #subregion Randomized consistency (expensive-ish; kept here since it's the one genuinely property-style check in this file)
mod quick {
    use super::*;

    /// 🎲️ Tiny deterministic xorshift so this crate doesn't need `crate::random` as a dependency just for one fuzz test.
    fn xorshift(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    #[test]
    fn csr_out_degree_matches_storage_out_degree_under_random_directed_graphs() {
        block_on_test(async {
            let mut seed = 0x5eed_u64;
            for _ in 0..20 {
                let mut g = ND::new();
                let n = 3 + (xorshift(&mut seed) % 8) as usize;
                // 🔀️ Rewritten from `.map(..)` — `add_node` is async and cannot be called inside
                // the sync closure that used to build `nodes` (R10 residue shape #1).
                let mut nodes: Vec<NodeId> = Vec::with_capacity(n);
                for _ in 0..n {
                    nodes.push(g.add_node());
                }
                let edge_attempts = n * 2;
                for _ in 0..edge_attempts {
                    let u = nodes[(xorshift(&mut seed) as usize) % n];
                    let v = nodes[(xorshift(&mut seed) as usize) % n];
                    g.add_edge(u, v);
                }
                let csr = Csr::from_view(&g);
                for &node in &nodes {
                    let i = csr.index_of(node).expect("every storage node is indexed");
                    assert_eq!(csr.out_neighbors(i).len(), g.out_degree(node), "csr out-degree must match storage out-degree for node {node}");
                }
            }
        });
    }
}
// #endsubregion

// #subregion MaxFlow
/// 🏗️ The classic CLRS Ford-Fulkerson network (Fig. 26.1): six nodes `s=0, v1=1, v2=2, v3=3, v4=4, t=5`, known max flow `23`.
fn clrs_flow_network() -> FlowNetwork {
    let mut net = FlowNetwork::new(6);
    net.add_edge(0, 1, 16.0);
    net.add_edge(0, 2, 13.0);
    net.add_edge(1, 3, 12.0);
    net.add_edge(2, 1, 4.0);
    net.add_edge(3, 2, 9.0);
    net.add_edge(2, 4, 14.0);
    net.add_edge(4, 3, 7.0);
    net.add_edge(3, 5, 20.0);
    net.add_edge(4, 5, 4.0);
    net
}

#[test]
fn max_flow_matches_clrs_textbook_network() {
    block_on_test(async {
        let mut net = clrs_flow_network();
        assert_eq!(net.max_flow(0, 5), 23.0);
    });
}

#[test]
fn min_cut_capacity_matches_max_flow_value_duality() {
    block_on_test(async {
        let mut net = clrs_flow_network();
        let flow = net.max_flow(0, 5);
        let reachable: BTreeSet<u32> = net.min_cut(0).into_iter().collect();
        assert!(!reachable.contains(&5), "sink must land on the far side of a valid cut");
        let clrs_edges = [(0u32, 1u32, 16.0), (0, 2, 13.0), (1, 3, 12.0), (2, 1, 4.0), (3, 2, 9.0), (2, 4, 14.0), (4, 3, 7.0), (3, 5, 20.0), (4, 5, 4.0)];
        let crossing: f64 = clrs_edges.iter().filter(|&&(u, v, _)| reachable.contains(&u) && !reachable.contains(&v)).map(|&(_, _, cap)| cap).sum();
        assert_eq!(crossing, flow, "total capacity crossing the min cut must equal the max flow value");
    });
}

#[test]
fn max_flow_saturates_branching_level_graph() {
    block_on_test(async {
        let mut net = FlowNetwork::new(5);
        net.add_edge(0, 1, 10.0);
        net.add_edge(0, 2, 10.0);
        net.add_edge(0, 3, 10.0);
        net.add_edge(1, 2, 2.0);
        net.add_edge(2, 3, 2.0);
        net.add_edge(1, 4, 4.0);
        net.add_edge(2, 4, 4.0);
        net.add_edge(3, 4, 4.0);
        assert_eq!(net.max_flow(0, 4), 12.0, "sink in-degree 3 at capacity 4 each caps the flow at 12 regardless of source out-degree 3");
    });
}

#[test]
fn max_flow_is_zero_when_source_and_sink_are_disconnected() {
    block_on_test(async {
        let mut net = FlowNetwork::new(2);
        assert_eq!(net.max_flow(0, 1), 0.0);
        assert_eq!(net.min_cut(0), vec![0], "with no path at all, only the source itself is reachable");
    });
}

#[test]
fn max_flow_and_min_cut_are_deterministic_across_fresh_instances() {
    block_on_test(async {
        let mut first = clrs_flow_network();
        let mut second = clrs_flow_network();
        let flow_a = first.max_flow(0, 5);
        let flow_b = second.max_flow(0, 5);
        assert_eq!(flow_a, flow_b, "identically constructed networks must yield byte-identical flow values");
        assert_eq!(first.min_cut(0), second.min_cut(0), "identically constructed networks must yield byte-identical min-cut node sets");
    });
}
// #endsubregion

// #subregion PropertyValue
#[test]
fn property_bag_value_round_trips_and_empty_bag_serializes_to_none() {
    block_on_test(async {
        let mut bag = PropertyBag::new();
        bag.insert("label".into(), PropertyValue::String("hi".into()));
        bag.insert("count".into(), PropertyValue::Number(3.0));
        let value = property_bag_to_value(&bag).expect("non-empty bag serializes to Some");
        let round_tripped = property_bag_from_value(&value);
        assert_eq!(round_tripped.get("label").and_then(PropertyValue::as_str), Some("hi"));
        assert_eq!(round_tripped.get("count").and_then(PropertyValue::as_f64), Some(3.0));
        assert!(property_bag_to_value(&PropertyBag::new()).is_none(), "an empty bag serializes to None");
    });
}

#[test]
fn property_bag_from_value_falls_back_to_default_on_unparsable_shape() {
    block_on_test(async {
        let value = dsl_core::DslValue::String("not-an-object-map".to_string());
        let bag = property_bag_from_value(&value);
        assert!(bag.is_empty(), "a value that can't deserialize into a PropertyBag falls back to empty");
    });
}
// #endsubregion
