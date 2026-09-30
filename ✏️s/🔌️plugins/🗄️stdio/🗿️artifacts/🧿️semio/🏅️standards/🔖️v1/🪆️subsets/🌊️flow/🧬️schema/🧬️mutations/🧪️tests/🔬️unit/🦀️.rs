use super::*;
use protocol::command::DiffAlgebra;

//#region 🔖️MutationDiffLaw
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    for mutation in demo_mutation_cases() {
        let base = fixture();
        let diff_direct = Mutation::diff(&mutation, &base);
        let applied_via_diff = protocol::MutationDiff::apply(diff_direct.diff(), &base).expect("apply must succeed for a well-formed fixture");

        let mut via_apply = base.clone();
        let diff_from_apply = apply_semio_flow_mutation(&mut via_apply, &mutation);

        assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
        assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
    }
}
//#endregion 🔖️MutationDiffLaw

//#region 🔖️InverseLaw
#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    for mutation in demo_mutation_cases() {
        let base = fixture();

        let mut round_tripped = base.clone();
        apply_semio_flow_mutation(&mut round_tripped, &mutation);
        for inverse_mutation in <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(&mutation, &base) {
            apply_semio_flow_mutation(&mut round_tripped, &inverse_mutation);
        }
        assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

        let diff = Mutation::diff(&mutation, &base);
        let next = protocol::MutationDiff::apply(diff.diff(), &base).expect("apply must succeed for a well-formed fixture");
        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = protocol::MutationDiff::apply(&inverse_diff, &next).expect("apply must succeed for a well-formed fixture");
        assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
    }
}
//#endregion 🔖️InverseLaw

//#region 🔖️OpTextBinaryRoundtripLaw
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = SemioFlowMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = SemioFlowMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️OpTextBinaryRoundtripLaw

//#region 🔖️KindsCatalog
/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling the binary op
/// frame's `tag` ordinal and the text grammar's keyword both use, and every one of those
/// spellings must also appear in the committed oracle manifest's catalog. The framework never
/// parses Rust, so this is what makes the declaration honest.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    assert_eq!(KINDS.len(), 13, "KINDS must name exactly one entry per declared SemioFlowMutation variant");
    let mut seen = vec![false; KINDS.len()];
    for mutation in demo_mutation_cases() {
        let keyword = print_flow_mutation(&mutation).split(' ').next().expect("printed op is never empty").to_string();
        let ordinal = wire_tag(&mutation) as usize;
        assert_eq!(KINDS[ordinal], keyword, "KINDS must match the declaration order and spelling for {mutation:?}");
        seen[ordinal] = true;
    }
    assert!(seen.iter().all(|hit| *hit), "demo_mutation_cases must reach every KINDS entry, missing {:?}", KINDS.iter().zip(seen.iter()).filter(|(_, hit)| !**hit).map(|(kind, _)| *kind).collect::<Vec<_>>());
    let manifest = include_str!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
//#endregion 🔖️KindsCatalog

//#region 🔖️VariantBehavior
#[semio_framework_async_macros::async_test]
async fn insert_then_remove_node_apply_and_inverse() {
    let base = fixture();
    let insert = SemioFlowMutation::InsertNode(insert_node::InsertNode { node: node("n3", "transform", "T", 5.0, 5.0) });
    let mut after = base.clone();
    apply_semio_flow_mutation(&mut after, &insert);
    assert_eq!(after.nodes.len(), 3);
    for inv in Mutation::inverse(&insert, &base) {
        apply_semio_flow_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}

#[semio_framework_async_macros::async_test]
async fn node_param_mutations_apply_and_inverse() {
    let base = fixture();
    let set = SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "k".into(), value: "new".into() });
    let mut after = base.clone();
    apply_semio_flow_mutation(&mut after, &set);
    assert_eq!(param_value_at(&after, "n1", "k"), Some("new"));
    for inv in Mutation::inverse(&set, &base) {
        apply_semio_flow_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let add = SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "fresh".into(), value: "added".into() });
    let mut after2 = base.clone();
    apply_semio_flow_mutation(&mut after2, &add);
    assert_eq!(param_value_at(&after2, "n1", "fresh"), Some("added"));
    for inv in Mutation::inverse(&add, &base) {
        apply_semio_flow_mutation(&mut after2, &inv);
    }
    assert_eq!(after2, base);
}

#[semio_framework_async_macros::async_test]
async fn edge_mutations_apply_and_inverse() {
    let base = fixture();
    let set = SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: "e1".into(), from: PortRef { node: "n2".into(), port: "out".into() }, to: PortRef { node: "n1".into(), port: "in".into() } });
    let mut after = base.clone();
    apply_semio_flow_mutation(&mut after, &set);
    assert_eq!(edge_at(&after, "e1").unwrap().from.node, "n2");
    for inv in Mutation::inverse(&set, &base) {
        apply_semio_flow_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);
}
//#endregion 🔖️VariantBehavior

//#region ✋️DragNodes
/// ✋️ `drag-nodes` is relative and parametric: every addressed node moves by the offset read off its BASE position,
/// the undo is one exact `set-node-position` per moved node, and the outcome vocabulary is the frozen one.
#[semio_framework_async_macros::async_test]
async fn drag_nodes_moves_relative_to_its_base_and_undoes_exactly() {
    let base = fixture();
    let drag = SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into(), "n2".into()], dx: 12.5, dy: -4.0 });
    let mut after = base.clone();
    let outcome = apply_semio_flow_mutation(&mut after, &drag);
    assert!(outcome.messages().is_empty(), "{:?}", outcome.messages());
    assert_eq!((after.nodes[0].position.x, after.nodes[0].position.y), (12.5, -4.0));
    assert_eq!((after.nodes[1].position.x, after.nodes[1].position.y), (22.5, 6.0));
    let mut restored = after.clone();
    for step in inverse_semio_flow_mutation(&drag, &base) {
        apply_semio_flow_mutation(&mut restored, &step);
    }
    assert_eq!(restored, base, "drag-nodes' undo restores every base position exactly");
    let partial = Mutation::diff(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into(), "ghost".into()], dx: 1.0, dy: 0.0 }), &base);
    assert!(partial.messages().iter().any(|message| message.code.0 == "mutation.partial" && message.target == vec!["ghost".to_string()]), "{:?}", partial.messages());
    let missing = Mutation::diff(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["ghost".into()], dx: 1.0, dy: 0.0 }), &base);
    assert!(missing.messages().iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == protocol::Severity::Error), "{:?}", missing.messages());
    let still = Mutation::diff(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into()], dx: 0.0, dy: 0.0 }), &base);
    assert!(still.messages().iter().any(|message| message.code.0 == "mutation.no-op"), "{:?}", still.messages());
    for malformed in [drag_nodes::DragNodes { targets: Vec::new(), dx: 1.0, dy: 0.0 }, drag_nodes::DragNodes { targets: vec!["n1".into(), "n1".into()], dx: 1.0, dy: 0.0 }, drag_nodes::DragNodes { targets: vec!["n1".into()], dx: f64::NAN, dy: 0.0 }] {
        let outcome = Mutation::diff(&SemioFlowMutation::DragNodes(malformed), &base);
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.level == protocol::Severity::Fatal), "{:?}", outcome.messages());
    }
}

/// 🏷️ The history row reads the drag's own inputs, in English and German.
#[test]
fn drag_nodes_labels_its_row_from_its_inputs() {
    let label = <SemioFlowMutation as protocol::SemanticMutation<SemioFlowSnapshot>>::label(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into(), "n2".into()], dx: 80.0, dy: 40.5 }));
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 2 nodes by (80, 40.5)");
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::De), "2 Knoten um (80; 40,5) ziehen");
}
//#endregion ✋️DragNodes
