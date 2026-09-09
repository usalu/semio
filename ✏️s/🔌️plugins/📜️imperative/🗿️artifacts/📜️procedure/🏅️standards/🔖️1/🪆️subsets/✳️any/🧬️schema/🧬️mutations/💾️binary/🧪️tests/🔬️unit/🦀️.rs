use super::*;
use crate::mutations::{create_step, delete_step, edit_step_params, reorder_steps};
use crate::{PathRef, ProcedureSnapshot};

#[test]
fn direct_wire_records_preserve_keyword_fields_and_tag_order() {
    let expected = [("create-step", &["owner", "slot", "item"][..]), ("delete-step", &["owner", "slot", "id"][..]), ("reorder-steps", &["owner", "slot", "id", "to"][..]), ("edit-step-params", &["owner", "slot", "id", "params"][..])];
    let variants = <ProcedureMutationDsl as dsl::DslVariants>::variants();
    assert_eq!(variants.len(), expected.len());
    for (index, ((keyword, spec), (expected_keyword, expected_fields))) in variants.iter().zip(expected.iter()).enumerate() {
        assert_eq!(keyword.as_str(), *expected_keyword);
        assert_eq!(BINARY_TAG_REGISTRY[index], (*expected_keyword, index as u8));
        let fields: Vec<_> = spec().fields.into_iter().map(|field| field.key).collect();
        assert_eq!(fields, expected_fields.iter().map(|field| (*field).to_owned()).collect::<Vec<_>>());
    }
}

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = delete_step(PathRef::default(), "step-1".into());
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trip_with_applied_operation() {
    use crate::{Dictionary, Step};
    use std::collections::BTreeMap;

    let document = crate::schema::default_snapshot();
    let envelope = store::create_document_envelope::<ProcedureSnapshot, ProcedureMutation>("procedure.document/v1", "test", document, None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    let step = Step { id: "step-x".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
    let operation = create_step(PathRef::default(), step);
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![operation], description: None }).await.expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
}

#[semio_framework_async_macros::async_test]
async fn op_text_rejects_unknown_operation_keyword() {
    let line = r#"frobnicate owner=- slot=- id="step-1""#;
    assert!(<ProcedureMutation as protocol::OpText>::parse_op(line).is_err());
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_create_step_with_owner_and_slot() {
    use crate::{Dictionary, Step};
    use std::collections::BTreeMap;
    let step = Step { id: "step-nested".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
    let operation = create_step(PathRef { owner: Some("step-if".into()), slot: Some("then".into()) }, step);
    let printed = <ProcedureMutation as protocol::OpText>::print_op(&operation);
    assert!(printed.contains("owner=step-if"), "printed: {printed}");
    assert!(printed.contains("slot=then"), "printed: {printed}");
    let parsed = <ProcedureMutation as protocol::OpText>::parse_op(&printed).expect("round trips");
    assert_eq!(parsed, operation);
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_reorder_steps() {
    let operation = reorder_steps(PathRef::default(), "step-2".into(), 0);
    let printed = <ProcedureMutation as protocol::OpText>::print_op(&operation);
    let parsed = <ProcedureMutation as protocol::OpText>::parse_op(&printed).expect("round trips");
    assert_eq!(parsed, operation);
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_edit_step_params() {
    use crate::Dictionary;
    use neural_engine::{Atom, Value};
    let operation = edit_step_params(PathRef::default(), "step-2".into(), Dictionary::new().insert("message", Value::Atom(Atom::String("hi".into()))));
    let printed = <ProcedureMutation as protocol::OpText>::print_op(&operation);
    let parsed = <ProcedureMutation as protocol::OpText>::parse_op(&printed).expect("round trips");
    assert_eq!(parsed, operation);
}
