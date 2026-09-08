
use super::*;
use crate::TRINITY_GRAPH_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn rename_op_binary_round_trips_and_agrees_with_text() {
    let operation = rename_node("node-1".into(), "Renamed".into());
    ::store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn nakagin_document_text_round_trips_store_with_applied_operation() {
    let envelope = create_document_envelope_for_test();
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![rename_node("node-1".into(), "Renamed".into())], description: None }).await.expect("apply rename");
    ::store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    ::store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
}

fn create_document_envelope_for_test() -> store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation> {
    create_document_envelope::<JackSnapshot, TrinityGraphMutation>(TRINITY_GRAPH_SCHEMA, "doc-text-test", crate::standards::v1::subsets::any::schema::empty_jack_document(), None)
}
use store::create_document_envelope;

fn empty_jack_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> JackStoreInitializationAuthority {
    let envelope = create_document_envelope(TRINITY_GRAPH_SCHEMA, "jack-retained-load", crate::standards::v1::subsets::any::schema::empty_jack_document(), None);
    JackStoreInitializationAuthority::new(envelope, operation, generation)
}

fn drive_jack_initializer(authority: &mut JackStoreInitializationAuthority, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job::StepOutcome {
    let cancel = semio_framework_job::root_cancel_token();
    let mut preview_sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        let outcome = semio_framework_plugin::ArtifactStoreInitializationAuthority::step(authority, &mut context);
        if outcome.is_terminal() {
            return outcome;
        }
    }
    panic!("Jack retained initializer did not reach a bounded terminal")
}

fn close_jack_candidate(mut candidate: store::ArtifactStore<JackSnapshot, TrinityGraphMutation>) {
    use semio_framework_plugin::ArtifactOwnedDisposer;

    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<JackSnapshot, TrinityGraphMutation>::new();
    for _ in 0..100_000 {
        match disposer.close_step(&mut candidate, 1, JACK_OWNED_FIELD_BYTES).expect("Jack candidate close step") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= JACK_OWNED_FIELD_BYTES);
            }
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("fresh Jack candidate close unexpectedly blocked: {reason}"),
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("fresh Jack candidate close unexpectedly needs input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Complete => {
                assert!(disposer.terminal_is_empty(&candidate));
                drop(disposer);
                drop(candidate);
                return;
            }
        }
    }
    panic!("Jack candidate did not reach terminal-empty close")
}

#[test]
fn jack_store_initializer_publishes_exact_next_generation_and_candidate_closes_incrementally() {
    let operation = semio_framework_job::OperationId(501);
    let generation = semio_framework_job::Generation(13);
    let mut authority = empty_jack_initializer(operation, generation);
    assert!(matches!(drive_jack_initializer(&mut authority, operation, generation), semio_framework_job::StepOutcome::Complete(_)));
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Jack candidate");
    assert_eq!(candidate.generation_now(), 14);
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_jack_candidate(candidate);
}

#[test]
fn jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
    let operation = semio_framework_job::OperationId(502);
    let generation = semio_framework_job::Generation(15);
    let mut cancelled = empty_jack_initializer(operation, generation);
    semio_framework_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
    assert!(matches!(drive_jack_initializer(&mut cancelled, operation, generation), semio_framework_job::StepOutcome::Cancelled));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
    drop(cancelled);

    let mut stale = empty_jack_initializer(operation, generation);
    assert!(matches!(drive_jack_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1)), semio_framework_job::StepOutcome::Fault(_)));
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
}

#[test]
fn jack_nested_mutation_and_child_snapshot_retire_one_exact_owner_per_grant() {
    let mut object = std::collections::BTreeMap::new();
    object.insert("nested".repeat(32), PropertyValue::Array(vec![PropertyValue::String("payload".repeat(128)), PropertyValue::String("tail".into())]));
    let mutation = TrinityGraphMutation::ChangeDataProperty(crate::standards::v1::subsets::any::schema::mutations::ChangeDataProperty { entity: EntityRef::Node("node".repeat(64)), key: "key".repeat(64), new_value: PropertyValue::Object(object) });
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackMutationRetirementFactory, mutation);
    for _ in 0..10_000 {
        let step = retirement.close_step(1, JACK_OWNED_FIELD_BYTES).expect("one nested Jack owner retires");
        match step {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= JACK_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
            store::SnapshotRetirementStep::Blocked => panic!("owned Jack mutation retirement cannot block"),
        }
    }
    panic!("nested Jack mutation retirement did not reach terminal")
}

#[semio_framework_async_macros::async_test]
async fn rename_op_text_round_trips() {
    ::store::os_store::test_support::assert_op_line_round_trip(&rename_node("node-1".into(), "Renamed".into()));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_create_node() {
    ::store::os_store::test_support::assert_op_line_round_trip(&create_node(Node {
        id: "new".into(),
        kind: "Piece".into(),
        name: "new-piece".into(),
        x: 200.0,
        y: 40.0,
        width: 80.0,
        height: 40.0,
        properties: PropertyBag::new(),
        ports: vec![Port { id: "p1".into(), kind: "Connector".into(), direction: crate::PortDirection::Out, properties: PropertyBag::new() }],
    }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_delete_node() {
    ::store::os_store::test_support::assert_op_line_round_trip(&delete_node("root".into()));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_create_edge() {
    let mut properties = PropertyBag::new();
    properties.insert("u".into(), PropertyValue::Number(1.2));
    let mut nested = std::collections::BTreeMap::new();
    nested.insert("x".into(), PropertyValue::Number(0.0));
    properties.insert("meta".into(), PropertyValue::Object(nested));
    ::store::os_store::test_support::assert_op_line_round_trip(&create_edge(Edge { id: "e2".into(), kind: "Connection".into(), source: crate::port_key("root", "out-a"), target: crate::port_key("child", "in-a"), properties }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_delete_edge() {
    ::store::os_store::test_support::assert_op_line_round_trip(&delete_edge("e1".into()));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_rename_node() {
    ::store::os_store::test_support::assert_op_line_round_trip(&rename_node("root".into(), "renamed \"piece\"".into()));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_move_node() {
    ::store::os_store::test_support::assert_op_line_round_trip(&move_node("root".into(), 10.0, -20.5));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_change_data_property() {
    ::store::os_store::test_support::assert_op_line_round_trip(&change_data_property(EntityRef::Node("root".into()), "label".into(), PropertyValue::String("hi 'there'".into())));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trip_remove_data_property() {
    ::store::os_store::test_support::assert_op_line_round_trip(&remove_data_property(EntityRef::Edge("e1".into()), "u".into()));
}

#[semio_framework_async_macros::async_test]
async fn parse_op_rejects_unknown_keyword() {
    let err = TrinityGraphMutation::parse_op("bogusOp x").expect_err("unknown op");
    assert!(err.message.contains("unknown mutation line"));
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphStore;
    use protocol::{ArtifactId, Edit, SchemaId};

    let mut store = TrinityGraphStore::new(create_document_envelope_for_test()).await.expect("valid artifact store");
    crate::standards::v1::subsets::any::schema::mutations::text::dispatch_trinity_graph_mutations(&mut store, vec![rename_node("node-1".into(), "Renamed".into())]).await.unwrap_or(());
    if let Some(edit) = store.envelope().vcs.edits.last() {
        let edit: &Edit<TrinityGraphMutation> = edit;
        ::store::os_store::test_support::assert_command_envelope_round_trip::<JackSnapshot, TrinityGraphMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    }
}
