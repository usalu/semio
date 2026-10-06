use crate::standards::v1::subsets::animation::io::binary::mutations::wire_tag;
use crate::standards::v1::subsets::flow::io::text::mutations::node;
use crate::standards::v1::subsets::flow::io::text::mutations::print_flow_mutation;
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
        for inverse_mutation in <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
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
    assert_eq!(KINDS.len(), 14, "KINDS must name exactly one entry per declared SemioFlowMutation variant");
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
    for inv in Mutation::inverse(&insert, &base).expect("valid retained mutation inverse fixture") {
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
    for inv in Mutation::inverse(&set, &base).expect("valid retained mutation inverse fixture") {
        apply_semio_flow_mutation(&mut after, &inv);
    }
    assert_eq!(after, base);

    let add = SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: "n1".into(), key: "fresh".into(), value: "added".into() });
    let mut after2 = base.clone();
    apply_semio_flow_mutation(&mut after2, &add);
    assert_eq!(param_value_at(&after2, "n1", "fresh"), Some("added"));
    for inv in Mutation::inverse(&add, &base).expect("valid retained mutation inverse fixture") {
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
    for inv in Mutation::inverse(&set, &base).expect("valid retained mutation inverse fixture") {
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
    for step in inverse_semio_flow_mutation(&drag, &base).expect("valid retained mutation inverse fixture") {
        apply_semio_flow_mutation(&mut restored, &step);
    }
    assert_eq!(restored, base, "drag-nodes' undo restores every base position exactly");
    let partial = Mutation::diff(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into(), "ghost".into()], dx: 1.0, dy: 0.0 }), &base);
    assert!(partial.messages().iter().any(|message| message.code.0 == "mutation.partial" && message.target == vec!["ghost".to_string()]), "{:?}", partial.messages());
    let missing = Mutation::diff(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["ghost".into()], dx: 1.0, dy: 0.0 }), &base);
    assert!(missing.messages().iter().any(|message| message.code.0 == "mutation.target-missing" && message.level == semio_framework_diagnostic::Severity::Error), "{:?}", missing.messages());
    let still = Mutation::diff(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into()], dx: 0.0, dy: 0.0 }), &base);
    assert!(still.messages().iter().any(|message| message.code.0 == "mutation.no-op"), "{:?}", still.messages());
    for malformed in [drag_nodes::DragNodes { targets: Vec::new(), dx: 1.0, dy: 0.0 }, drag_nodes::DragNodes { targets: vec!["n1".into(), "n1".into()], dx: 1.0, dy: 0.0 }, drag_nodes::DragNodes { targets: vec!["n1".into()], dx: f64::NAN, dy: 0.0 }] {
        let outcome = Mutation::diff(&SemioFlowMutation::DragNodes(malformed), &base);
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.level == semio_framework_diagnostic::Severity::Fatal), "{:?}", outcome.messages());
    }
}

/// 🏷️ The history row reads the drag's own inputs, in English and German.
#[test]
fn drag_nodes_labels_its_row_from_its_inputs() {
    let label = <SemioFlowMutation as protocol::SemanticMutation<SemioFlowSnapshot>>::label(&SemioFlowMutation::DragNodes(drag_nodes::DragNodes { targets: vec!["n1".into(), "n2".into()], dx: 80.0, dy: 40.5 }));
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 2 nodes by (80, 40.5)");
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "2 Knoten um (80; 40,5) ziehen");
}
//#endregion ✋️DragNodes

#[test]
fn paged_flow_original_source_keeps_all_fourteen_frames_and_exact_owner_policy(){
    use protocol::io::binary::operation_bytes::{OwnedOperationBytes,OperationByteMeasurement,OperationBytePreparation,OperationByteCloseStep};
    use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📦️operation-source.json")).unwrap();
    let mut operations=Vec::new();
    for case in fixture["cases"].as_array().unwrap(){let operation=SemioFlowMutation::parse_op(case["text"].as_str().unwrap()).unwrap();let mut literal=vec![1,case["tag"].as_u64().unwrap()as u8];literal.extend_from_slice(case["body"].as_str().unwrap().as_bytes());assert_eq!(operation.encode_op().unwrap(),literal);operations.push(operation);}
    let patch=semio_s_artifact_stdio_contract::editing::SnapshotPatch::parse_op(fixture["patchJson"].as_str().unwrap()).unwrap();let patch=SemioFlowMutation::PatchSnapshot(patch_snapshot::PatchSnapshot{patch});let mut literal=vec![1,13];literal.extend_from_slice(fixture["patchJson"].as_str().unwrap().as_bytes());assert_eq!(patch.encode_op().unwrap(),literal);operations.push(patch);assert_eq!(operations.len(),KINDS.len());
    operations.push(SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition{id:"n".into(),position:SemioPoint2{x:f64::MAX,y:f64::from_bits(1)}}));
    operations.push(SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel{id:"n".into(),label:fixture["word"].as_str().unwrap().repeat(fixture["payloadBytes"].as_u64().unwrap()as usize)}));
    let allocation=fixture["allocationBytes"].as_u64().unwrap()as usize;let items=fixture["maximumCloseItems"].as_u64().unwrap()as usize;let bytes=fixture["maximumCloseBytes"].as_u64().unwrap()as usize;
    let close=|owner:&mut OwnedOperationBytes,steps:usize|{let mut released=0;for _ in 0..steps{match owner.close_one(items,bytes).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=bytes);released+=released_bytes;}}}assert!(owner.terminal_is_empty());assert_eq!(owner.allocated_bytes(),0);released};
    let kind=|error:protocol::ProtocolError|match error{protocol::ProtocolError::Pack(protocol::PackError::Refusal(refusal))=>refusal.kind(),error=>panic!("expected original typed refusal, got {error:?}")};
    for operation in &operations{
        let expected=operation.encode_op().unwrap();let steps=expected.len()+fixture["closeStepScaffold"].as_u64().unwrap()as usize;let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_file_len=expected.len()as u64;
        let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(0,&mut allow);operation.encode_op_into(&options,&mut measure,&mut encoding).unwrap();assert_eq!(measure.exact_length().unwrap(),expected.len());assert_eq!(encoding.owned_bytes(),0);
        let mut preparation=OperationBytePreparation::try_new(expected.len(),allocation).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);
        for _ in 0..steps{preparation.fund_one(items,bytes,&mut encoding).unwrap();if preparation.is_funded(){break;}}assert!(preparation.is_funded());let paid=encoding.owned_bytes();let backing=preparation.allocated_bytes();operation.encode_op_into(&options,&mut preparation,&mut encoding).unwrap();assert_eq!(encoding.owned_bytes(),paid);assert_eq!(preparation.allocated_bytes(),backing);
        let mut owner=preparation.take_ready().unwrap();assert!(owner.iter().eq(expected.iter().copied()));assert_eq!(serde_json::to_value(&owner).unwrap(),serde_json::to_value(&expected).unwrap());assert_eq!(owner.close_one(0,bytes).unwrap(),OperationByteCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!(owner.len(),expected.len());assert!(close(&mut owner,steps)>=expected.len());
        let mut short=options.clone();short.limits.max_file_len-=1;let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(allocation,&mut allow);assert_eq!(kind(operation.encode_op_into(&short,&mut prefix,&mut encoding).unwrap_err()),ValueRefusalKind::OwnershipLimit);assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix,steps);
    }
    let operation=operations.last().unwrap();let expected=operation.encode_op().unwrap();let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_file_len=expected.len()as u64;let mut prefix=OwnedOperationBytes::try_new(expected.len(),allocation).unwrap();let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|progress.completed<fixture["cancelAt"].as_u64().unwrap()as usize;let mut encoding=NativeEncodeControl::new(allocation,&mut cancel);
    assert_eq!(kind(operation.encode_op_into(&options,&mut prefix,&mut encoding).unwrap_err()),ValueRefusalKind::Canceled);assert!(prefix.len()<expected.len());assert!(prefix.iter().eq(expected[..prefix.len()].iter().copied()));close(&mut prefix,expected.len()+fixture["closeStepScaffold"].as_u64().unwrap()as usize);
    println!("[DEBUG] Original fourteen Flow headers and text/Patch JSON bodies preserve neutral literal octets; original8194 label and extreme native f64 Display use fixed source cells, caller policy/refused prefixes and terminal4096 page grants");
}
