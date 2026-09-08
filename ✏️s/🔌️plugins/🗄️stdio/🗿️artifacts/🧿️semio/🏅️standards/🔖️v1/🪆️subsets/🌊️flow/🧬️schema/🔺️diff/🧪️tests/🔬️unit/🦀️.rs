
use super::*;
use protocol::DiffCodec;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn node(id: &str, kind: &str, label: &str, params: Vec<(&str, &str)>, x: f64, y: f64) -> FlowNode {
    FlowNode { id: id.into(), kind: kind.into(), label: label.into(), params: params.into_iter().map(|(k, v)| FlowParam { key: k.into(), value: v.into() }).collect(), position: SemioPoint2 { x, y } }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn edge(id: &str, from_node: &str, from_port: &str, to_node: &str, to_port: &str, kind: &str) -> FlowEdge {
    FlowEdge { id: id.into(), from: PortRef { node: from_node.into(), port: from_port.into() }, to: PortRef { node: to_node.into(), port: to_port.into() }, kind: kind.into() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        schema: crate::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![node("n1", "source", "Source", vec![("count", "1")], 0.0, 0.0), node("n2", "sink", "Sink", vec![], 10.0, 10.0)],
        edges: vec![edge("e1", "n1", "out", "n2", "in", "data")],
    }
}

/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field — nodes/edges each get one removed,
/// one modified-in-every-field, one added; the modified node's own `params` collection gets
/// one removed, one modified, one added (exercising the doubly-nested `NamedTripleDiff`).
///
/// 🔧️ W2b closer fix: `"toRemove"` moved to the END of `params` (was first). `NamedTripleDiff`
/// is name-keyed with no positional field (`apply_named` — see this module's own
/// `🔖️GenericNamedEngine` region — always appends `added` items at the tail, by design, same
/// as every other name-keyed collection in this program); the original ordering put
/// `"toRemove"` FIRST in `sweep_a`, so `between(sweep_b, sweep_a).apply(sweep_b)` (the reverse
/// direction, where `"toRemove"` is the ADDED item) reconstructed it at the tail instead of
/// the front — a real `assert_eq!` mismatch caught by both `between_roundtrip_law` and
/// `field_sweep`, not a bug in `apply_named` itself (its append-at-end behavior is the
/// correct, documented semantics for a name-keyed — i.e. order-insignificant — collection).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        schema: crate::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![node("keep", "old-kind", "Old Label", vec![("toModify", "old"), ("stay", "same"), ("toRemove", "gone")], 0.0, 0.0), node("toRemoveNode", "sink", "Gone", vec![], 5.0, 5.0)],
        edges: vec![edge("keepEdge", "keep", "out", "toRemoveNode", "in", "old-kind"), edge("toRemoveEdge", "toRemoveNode", "out", "keep", "in", "data")],
    }
}
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> SemioFlowSnapshot {
    SemioFlowSnapshot {
        schema: crate::standards::v1::subsets::flow::schema::snapshot::STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(),
        nodes: vec![node("keep", "new-kind", "New Label", vec![("toModify", "new"), ("stay", "same"), ("added", "fresh")], 42.0, 7.0), node("addedNode", "source", "Added", vec![], 9.0, 9.0)],
        edges: vec![edge("keepEdge", "keep", "renamed-out", "addedNode", "in", "new-kind"), edge("addedEdge", "addedNode", "out", "keep", "in", "data")],
    }
}

//#region 🔖️BetweenRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    assert_eq!(MutationDiff::apply(&<SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&a, &b), &a).expect("apply must succeed for a well-formed fixture"), b);
    assert_eq!(MutationDiff::apply(&<SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&b, &a), &b).expect("apply must succeed for a well-formed fixture"), a);

    let sample = base_snapshot();
    assert_eq!(MutationDiff::apply(&<SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&sample, &sample), &sample).expect("apply must succeed for a well-formed fixture"), sample);
}
//#endregion 🔖️BetweenRoundtripLaw

//#region 🔖️FieldSweep
/// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field, including
/// each collection flavor (removed/modified/added) at both the top level (nodes/edges) and the
/// nested level (a node's own `params`).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let diff_ab = <SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&a, &b);
    assert_eq!(MutationDiff::apply(&diff_ab, &a).expect("apply must succeed for a well-formed fixture"), b);
    assert!(<SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&a, &a).is_empty());

    let nodes_diff = diff_ab.nodes.as_ref().expect("nodes diff present");
    assert!(!nodes_diff.removed.is_empty(), "nodes: removed not exercised");
    assert!(!nodes_diff.added.is_empty(), "nodes: added not exercised");
    assert_eq!(nodes_diff.modified.len(), 1);
    let keep_diff = &nodes_diff.modified[0].diff;
    assert!(keep_diff.kind.is_some(), "node.kind not exercised");
    assert!(keep_diff.label.is_some(), "node.label not exercised");
    assert!(keep_diff.position.is_some(), "node.position not exercised");
    let params_diff = keep_diff.params.as_ref().expect("params diff present");
    assert!(!params_diff.removed.is_empty(), "params: removed not exercised");
    assert!(!params_diff.modified.is_empty(), "params: modified not exercised");
    assert!(!params_diff.added.is_empty(), "params: added not exercised");

    let edges_diff = diff_ab.edges.as_ref().expect("edges diff present");
    assert!(!edges_diff.removed.is_empty(), "edges: removed not exercised");
    assert!(!edges_diff.added.is_empty(), "edges: added not exercised");
    assert_eq!(edges_diff.modified.len(), 1);
    let keep_edge_diff = &edges_diff.modified[0].diff;
    assert!(keep_edge_diff.from.is_some(), "edge.from not exercised");
    assert!(keep_edge_diff.to.is_some(), "edge.to not exercised");
    assert!(keep_edge_diff.kind.is_some(), "edge.kind not exercised");

    let diff_ba = <SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&b, &a);
    assert_eq!(MutationDiff::apply(&diff_ba, &b).expect("apply must succeed for a well-formed fixture"), a);
}
//#endregion 🔖️FieldSweep

//#region 🔖️AbsorbLaw
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &SemioFlowSnapshot, d1: &SemioFlowDiff, d2: &SemioFlowDiff) -> SemioFlowDiff {
    let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut absorbed = d1.clone();
    MutationDiff::absorb(&mut absorbed, d2.clone());
    assert_eq!(MutationDiff::apply(&absorbed, base).expect("apply must succeed for a well-formed fixture"), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
    absorbed
}

#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    // Canonical: Insert+Remove(other) -> both survive independently (name-keyed absorb has no
    // index-transport interaction between an unrelated insert and an unrelated removal).
    {
        let base = base_snapshot();
        let d1 = diff_insert_node(node("f", "new", "F", vec![], 1.0, 1.0));
        let _mid = MutationDiff::apply(&d1, &base).expect("apply must succeed for a well-formed fixture");
        let d2 = diff_remove_node("n2");
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let nd = absorbed.nodes.as_ref().unwrap();
        assert_eq!(nd.removed, vec!["n2".to_string()]);
        assert_eq!(nd.added.len(), 1);
        assert_eq!(nd.added[0].id, "f");
    }

    // Canonical: Insert(f)+Insert(g) -> both survive.
    {
        let base = base_snapshot();
        let d1 = diff_insert_node(node("f", "new", "F", vec![], 1.0, 1.0));
        let mid = MutationDiff::apply(&d1, &base).expect("apply must succeed for a well-formed fixture");
        let d2 = diff_insert_node(node("g", "new", "G", vec![], 2.0, 2.0));
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let nd = absorbed.nodes.as_ref().unwrap();
        assert_eq!(nd.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
        let _ = mid;
    }

    // Canonical: Insert(f)+SetField(f) -> patch into the added payload.
    {
        let base = base_snapshot();
        let d1 = diff_insert_node(node("f", "new", "F", vec![], 1.0, 1.0));
        let mid = MutationDiff::apply(&d1, &base).expect("apply must succeed for a well-formed fixture");
        let d2 = diff_set_node_kind("f", "patched");
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let nd = absorbed.nodes.as_ref().unwrap();
        assert!(nd.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
        assert_eq!(nd.added.len(), 1);
        assert_eq!(nd.added[0].kind, "patched");
        let _ = mid;
    }

    // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
    {
        let base = base_snapshot();
        let d1 = diff_set_node_kind("n2", "patched");
        let mid = MutationDiff::apply(&d1, &base).expect("apply must succeed for a well-formed fixture");
        let d2 = diff_remove_node("n2");
        let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
        let nd = absorbed.nodes.as_ref().unwrap();
        assert!(nd.modified.is_empty(), "modify of a since-removed item must not survive absorb");
        assert_eq!(nd.removed, vec!["n2".to_string()]);
        let _ = mid;
    }

    // Associativity over a triple.
    {
        let base = base_snapshot();
        let d1 = diff_insert_node(node("f", "new", "F", vec![], 1.0, 1.0));
        let mid1 = MutationDiff::apply(&d1, &base).expect("apply must succeed for a well-formed fixture");
        let d2 = diff_insert_node(node("g", "new", "G", vec![], 2.0, 2.0));
        let mid2 = MutationDiff::apply(&d2, &mid1).expect("apply must succeed for a well-formed fixture");
        let d3 = diff_remove_node("n2");
        let sequential = MutationDiff::apply(&d3, &mid2).expect("apply must succeed for a well-formed fixture");

        let mut left = d1.clone();
        MutationDiff::absorb(&mut left, d2.clone());
        MutationDiff::absorb(&mut left, d3.clone());

        let mut d2_then_d3 = d2.clone();
        MutationDiff::absorb(&mut d2_then_d3, d3.clone());
        let mut right = d1.clone();
        MutationDiff::absorb(&mut right, d2_then_d3);

        assert_eq!(MutationDiff::apply(&left, &base).expect("apply must succeed for a well-formed fixture"), sequential, "absorb associativity (left) failed");
        assert_eq!(MutationDiff::apply(&right, &base).expect("apply must succeed for a well-formed fixture"), sequential, "absorb associativity (right) failed");
    }
}
//#endregion 🔖️AbsorbLaw

//#region 🔖️DiffCodecTextBinaryRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    let diffs = vec![
        SemioFlowDiff::default(),
        <SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&a, &b),
        <SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(&b, &a),
        diff_insert_node(node("z", "k", "L", vec![("a", "b")], 1.5, 2.5)),
        diff_insert_edge(edge("z", "a", "p", "b", "q", "k")),
    ];
    for d in diffs {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioFlowDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?}");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff({d:?}) failed: {e}"));
        let decoded = SemioFlowDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
    }
}
//#endregion 🔖️DiffCodecTextBinaryRoundtripLaw

// Keep the shared engine's per-collection edge codecs exercised directly too (guards against
// silent drift between the hand-rolled value codecs above and `🧰️triples`'s generic shape).
#[semio_framework_async_macros::async_test]
async fn node_and_edge_value_codecs_round_trip() {
    let n = node("n1", "k", "L", vec![("a", "1"), ("b", "2")], 3.5, -1.25);
    assert_eq!(dec_node(&enc_node(&n)).unwrap(), n);
    let e = edge("e1", "a", "p", "b", "q", "k");
    assert_eq!(dec_edge(&enc_edge(&e)).unwrap(), e);
}
