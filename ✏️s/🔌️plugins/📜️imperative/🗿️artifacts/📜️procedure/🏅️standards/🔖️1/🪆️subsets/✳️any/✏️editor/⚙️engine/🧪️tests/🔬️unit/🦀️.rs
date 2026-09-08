
use super::*;

#[semio_framework_async_macros::async_test]
async fn imperative_io_declares_result_out_reusing_the_computation_procedure_kind() {
    let io = imperative_io();
    assert_eq!(io.document_schema, "procedure.document/v1");
    assert_eq!(io.artifact.id, "computation.procedure");
    assert_eq!(io.ports.len(), 1);
    let port = &io.ports[0];
    assert_eq!(port.id, "result:out");
    assert_eq!(port.kind_id.as_deref(), Some("computation.procedure"));
    assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
    assert!(!port.required);
}

#[semio_framework_async_macros::async_test]
async fn host_runs_default_snapshot() {
    let host = ImperativeHost::default();
    let result = host.run();
    assert_eq!(result.effects.len(), 2);
    assert!(result.effects.iter().all(|entry| entry.error.is_none()));
}

#[semio_framework_async_macros::async_test]
async fn host_adds_nested_step_in_control_body() {
    let mut host = ImperativeHost::default();
    let owner = host.add_step("control.if", None);
    let path_ref = PathRef { owner: Some(owner.clone()), slot: Some("then".into()) };
    let nested = host.add_step_at(&path_ref, "log.print", None).expect("add nested");
    assert_eq!(nested, "step-102");
    let owner_step = host.path.steps.iter().find(|step| step.id == owner).expect("owner");
    assert_eq!(owner_step.bodies.get("then").map(|path| path.steps.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn imperative_core_error_messages() {
    assert_eq!(ImperativeCoreError::MissingOwner.to_string(), "missing owner");
    assert_eq!(ImperativeCoreError::MissingSlot.to_string(), "missing slot");
    assert_eq!(ImperativeCoreError::UnsupportedSchema("bad.schema".into()).to_string(), "unsupported schema: bad.schema");
    assert_eq!(ImperativeCoreError::UnknownOwnerStep("step-9".into()).to_string(), "unknown owner step: step-9");
    assert_eq!(ImperativeCoreError::UnknownStep("step-9".into()).to_string(), "unknown step: step-9");
}

#[semio_framework_async_macros::async_test]
async fn host_load_json_rejects_unsupported_schema() {
    let json = r#"{"schema":"not.procedure","flow":{"childId":"f","target":{"artifactId":"f","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"flow"}}},"text":{"childId":"t","target":{"artifactId":"t","dialect":{"artifactKind":"s.stdio.semio","standard":"v1","subset":"text"}}}}"#;
    assert!(matches!(ImperativeHost::load_json(json), Err(ImperativeCoreError::UnsupportedSchema(schema)) if schema == "not.procedure"));
}

#[semio_framework_async_macros::async_test]
async fn host_load_json_rejects_invalid_json() {
    assert!(matches!(ImperativeHost::load_json("not json"), Err(ImperativeCoreError::Json(_))));
}

#[semio_framework_async_macros::async_test]
async fn host_load_json_and_to_json_round_trip() {
    let json = ImperativeHost::default().to_json().expect("serializes");
    let host = ImperativeHost::load_json(&json).expect("parses back");
    assert_eq!(host.to_json().expect("serializes again"), json);
}

#[semio_framework_async_macros::async_test]
async fn host_catalogue_json_is_nonempty() {
    assert!(!ImperativeHost::default().catalogue_json().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn host_add_step_at_reports_missing_owner_and_slot() {
    let mut host = ImperativeHost::default();
    let missing_owner = PathRef { owner: None, slot: Some("then".into()) };
    assert!(matches!(host.add_step_at(&missing_owner, "log.print", None), Err(ImperativeCoreError::MissingOwner)));
    let missing_slot = PathRef { owner: Some("step-1".into()), slot: None };
    assert!(matches!(host.add_step_at(&missing_slot, "log.print", None), Err(ImperativeCoreError::MissingSlot)));
}

#[semio_framework_async_macros::async_test]
async fn host_add_step_at_reports_unknown_owner_step() {
    let mut host = ImperativeHost::default();
    let path_ref = PathRef { owner: Some("does-not-exist".into()), slot: Some("then".into()) };
    assert!(matches!(host.add_step_at(&path_ref, "log.print", None), Err(ImperativeCoreError::UnknownOwnerStep(owner)) if owner == "does-not-exist"));
}

#[semio_framework_async_macros::async_test]
async fn host_add_step_clamps_out_of_range_index() {
    let mut host = ImperativeHost::default();
    let before = host.path.steps.len();
    let id = host.add_step("log.print", Some(9999));
    assert_eq!(host.path.steps.last().map(|step| &step.id), Some(&id));
    assert_eq!(host.path.steps.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn host_remove_step_false_for_unresolvable_path_ref_and_unknown_id() {
    let mut host = ImperativeHost::default();
    let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
    assert!(!host.remove_step_at(&bad_path_ref, "step-1"));
    assert!(!host.remove_step("does-not-exist"));
}

#[semio_framework_async_macros::async_test]
async fn host_remove_step_true_when_removed() {
    let mut host = ImperativeHost::default();
    assert!(host.remove_step("step-1"));
    assert!(host.path.steps.iter().all(|step| step.id != "step-1"));
}

#[semio_framework_async_macros::async_test]
async fn host_move_step_false_for_unresolvable_path_ref_and_unknown_id() {
    let mut host = ImperativeHost::default();
    let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
    assert!(!host.move_step_at(&bad_path_ref, "step-1", 0));
    assert!(!host.move_step("does-not-exist", 0));
}

#[semio_framework_async_macros::async_test]
async fn host_move_step_true_and_reorders() {
    let mut host = ImperativeHost::default();
    assert!(host.move_step("step-2", 0));
    assert_eq!(host.path.steps[0].id, "step-2");
}

#[semio_framework_async_macros::async_test]
async fn host_set_step_params_at_rejects_invalid_json_and_unknown_step() {
    let mut host = ImperativeHost::default();
    assert!(matches!(host.set_step_params_json("step-1", "not json"), Err(ImperativeCoreError::Json(_))));
    assert!(matches!(host.set_step_params_json("does-not-exist", "{}"), Err(ImperativeCoreError::UnknownStep(id)) if id == "does-not-exist"));
    let bad_path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
    assert!(matches!(host.set_step_params_at(&bad_path_ref, "step-1", "{}"), Err(ImperativeCoreError::UnknownOwnerStep(_))));
}

#[semio_framework_async_macros::async_test]
async fn host_set_step_params_updates_existing_step() {
    use neural_engine::{Atom, Value};
    let mut host = ImperativeHost::default();
    host.set_step_params_json("step-2", r#"{"message":"updated"}"#).expect("sets params");
    let step = host.path.steps.iter().find(|step| step.id == "step-2").expect("step-2 exists");
    assert_eq!(step.params.get("message"), Some(&Value::Atom(Atom::String("updated".into()))));
}

#[semio_framework_async_macros::async_test]
async fn host_compile_text_contains_step_kinds() {
    let host = ImperativeHost::default();
    let compiled = host.compile_text();
    assert!(compiled.contains("state.set"));
    assert!(compiled.contains("log.print"));
}
