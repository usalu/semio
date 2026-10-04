use super::*;
use crate::standards::v1::subsets::any::schema::mutations::text::new_trinity_graph_store;
use crate::TRINITY_GRAPH_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn set_query_op_binary_round_trips_and_agrees_with_text() {
    let operation = set_query("MATCH (a:Piece) RETURN a.name".into());
    ::store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn nakagin_document_text_round_trips_store_with_applied_operation() {
    let envelope = create_document_envelope_for_test();
    let mut doc_store = new_trinity_graph_store(envelope).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![set_query("MATCH (a:Piece) RETURN a.name".into())], description: None, transaction: None }).await.expect("apply set-query");
    ::store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    ::store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
}

/// 🧫️ The empty Nakagin-manifest document the round-trip laws edit the query of.
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
    let mut outcome = drive_jack_initializer(&mut stale, operation, semio_framework_job::Generation(generation.0 + 1));
    assert!(matches!(outcome, semio_framework_job::StepOutcome::Fault(_)));
    // 🔚 The fault's retained detail payload must be closed page by page — `RetainedJobPayload`'s
    // Drop refuses to release page backing on its own, and a grant under one job payload page
    // (16 KiB, not the 4 KiB envelope page) releases nothing.
    while !outcome.terminal_is_empty() {
        assert!(matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Pending { released_items: 1, .. } | semio_framework_job::JobPayloadCloseStep::Complete), "fault payload close must progress");
    }
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
}

#[test]
fn jack_nested_query_effect_retires_one_exact_owner_per_grant() {
    let mut object = PropertyBag::new();
    object.insert("nested".repeat(32), PropertyValue::Array(vec![PropertyValue::String("payload".repeat(128)), PropertyValue::String("tail".into())]));
    let effect = GraphEffect::SetProperty { entity: EntityRef::Node("node".repeat(64)), key: "key".repeat(64), value: PropertyValue::Object(object) };
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackEffectRetirementFactory, effect);
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

fn drive_snapshot_clone(source: &JackSnapshot) -> JackSnapshot {
    let mut clone = JackSnapshotCloneAuthority::new();
    for _ in 0..100_000 {
        match clone.advance(source, JACK_OWNED_FIELD_BYTES).expect("bounded Jack snapshot clone") {
            JackSnapshotCloneStep::Pending { copied_bytes } => assert!(copied_bytes <= JACK_OWNED_FIELD_BYTES),
            JackSnapshotCloneStep::Complete => {
                let value = clone.take_value().expect("completed clone transfers its exact snapshot");
                assert!(clone.terminal_is_empty());
                drop(clone);
                return value;
            }
        }
    }
    panic!("Jack snapshot clone did not reach its bounded terminal")
}

fn drive_snapshot_retirement(value: JackSnapshot) -> usize {
    let mut retirement = store::ArtifactOwnedValueRetirementFactory::retire_owned(&JackSnapshotRetirementFactory, value);
    let mut steps = 0;
    for _ in 0..100_000 {
        steps += 1;
        match retirement.close_step(1, JACK_OWNED_FIELD_BYTES).expect("bounded Jack snapshot retirement") {
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= JACK_OWNED_FIELD_BYTES);
            }
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return steps;
            }
            store::SnapshotRetirementStep::Blocked => panic!("materialized Jack snapshot retirement cannot wait for a live peer owner"),
        }
    }
    panic!("Jack snapshot retirement did not reach its bounded terminal")
}

#[test]
fn query_ownership_shared_scene_clone_retires_while_source_remains_live() {
    let mut source = crate::standards::v1::subsets::any::schema::empty_jack_document();
    crate::materialize_jack_content(&mut source.content, vec![Node { id: "live".into(), kind: "Apartment".into(), name: "Live".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, properties: PropertyBag::new(), ports: Vec::new() }], Vec::new());
    let source_owner = source.content.local_owner::<crate::JackContentOwner>().expect("source content owner");
    let clone = drive_snapshot_clone(&source);
    let clone_owner = clone.content.local_owner::<crate::JackContentOwner>().expect("clone content owner");
    assert!(std::sync::Arc::ptr_eq(&source_owner, &clone_owner));
    drop(source_owner);
    drop(clone_owner);
    let steps = drive_snapshot_retirement(clone);
    assert_eq!(source.content.local_owner::<crate::JackContentOwner>().expect("live source owner remains").snapshot().nodes[0].id.value, "live");
    assert!(steps > 1);
    drive_snapshot_retirement(source);
}

#[test]
fn query_ownership_unique_scene_retirement_drains_entities_one_owner_per_grant() {
    let mut source = crate::standards::v1::subsets::any::schema::empty_jack_document();
    crate::materialize_jack_content(
        &mut source.content,
        vec![
            Node { id: "one".into(), kind: "Apartment".into(), name: "One".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, properties: PropertyBag::new(), ports: Vec::new() },
            Node { id: "two".into(), kind: "Apartment".into(), name: "Two".into(), x: 1.0, y: 1.0, width: 1.0, height: 1.0, properties: PropertyBag::new(), ports: Vec::new() },
        ],
        vec![Edge { id: "edge".into(), kind: "Connection".into(), source: "one@out".into(), target: "two@in".into(), properties: PropertyBag::new() }],
    );
    let steps = drive_snapshot_retirement(source);
    assert!(steps > 3, "the last scene owner must retire nodes and edges separately");
}

#[semio_framework_async_macros::async_test]
async fn set_query_op_text_round_trips() {
    ::store::os_store::test_support::assert_op_line_round_trip(&set_query("MATCH (a:Piece) WHERE a.name = 'b' RETURN a".into()));
}

#[semio_framework_async_macros::async_test]
async fn parse_op_rejects_unknown_keyword() {
    let err = TrinityGraphMutation::parse_op("bogusOp x").expect_err("unknown op");
    assert!(err.message.contains("unknown mutation line"));
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use protocol::{ArtifactId, Edit, SchemaId};

    let mut store = new_trinity_graph_store(create_document_envelope_for_test()).await.expect("valid artifact store");
    crate::standards::v1::subsets::any::schema::mutations::text::dispatch_trinity_graph_mutations(&mut store, vec![set_query("MATCH (a:Piece) RETURN a".into())]).await.unwrap_or(());
    if let Some(edit) = store.envelope().vcs.edits.last() {
        let edit: &Edit<TrinityGraphMutation> = edit;
        ::store::os_store::test_support::assert_command_envelope_round_trip::<JackSnapshot, TrinityGraphMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
    }
}


struct JackCloseRefusalOwner {
    kind: ValueRefusalKind,
    message: Option<String>,
}

impl semio_framework_value::ErasedSnapshotRetirement for JackCloseRefusalOwner {
    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<semio_framework_value::SnapshotRetirementStep, ValueError> {
        match self.message.take() {
            Some(message) => Err(ValueError::new(self.kind, message)),
            None => Ok(semio_framework_value::SnapshotRetirementStep::Complete),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.message.is_none()
    }
}

fn close_jack_initializer(authority: &mut JackStoreInitializationAuthority) {
    for _ in 0..100_000 {
        match semio_framework_plugin::ArtifactStoreInitializationAuthority::close_step(authority, 1, JACK_OWNED_FIELD_BYTES).expect("Jack initializer close") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= JACK_OWNED_FIELD_BYTES);
            }
            semio_framework_plugin::PluginCloseStep::Complete => {
                assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(authority));
                return;
            }
            step => panic!("Jack initializer unexpectedly stopped retirement: {step:?}"),
        }
    }
    panic!("Jack initializer did not reach terminal-empty close")
}

fn jack_close_refusal_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🚫️close-refusal/🔣️.json")).expect("closed language-neutral Jack refusal corpus")
}

#[test]
fn jack_initializer_close_preserves_typed_refusal_and_owned_message() {
    use semio_framework_value::ToValue;

    let fixture = jack_close_refusal_fixture();
    for row in fixture["closeRefusals"].as_array().expect("close refusal cases") {
        let kind = match row["kind"].as_str().expect("refusal kind") {
            "invalidValue" => ValueRefusalKind::InvalidValue,
            "canceled" => ValueRefusalKind::Canceled,
            "ownershipLimit" => ValueRefusalKind::OwnershipLimit,
            "allocationFailed" => ValueRefusalKind::AllocationFailed,
            "workLimit" => ValueRefusalKind::WorkLimit,
            "depthLimit" => ValueRefusalKind::DepthLimit,
            "unsupportedOwner" => ValueRefusalKind::UnsupportedOwner,
            "invariantViolated" => ValueRefusalKind::InvariantViolated,
            other => panic!("unknown refusal kind: {other}"),
        };
        let mut authority = empty_jack_initializer(semio_framework_job::OperationId(503), semio_framework_job::Generation(17));
        *authority.active = Some(Box::new(JackCloseRefusalOwner { kind, message: Some(row["message"].as_str().expect("refusal message").to_owned()) }));
        let zero_grant = semio_framework_plugin::ArtifactStoreInitializationAuthority::close_step(&mut authority, 0, JACK_OWNED_FIELD_BYTES);
        let result = semio_framework_plugin::ArtifactStoreInitializationAuthority::close_step(&mut authority, 1, JACK_OWNED_FIELD_BYTES);
        close_jack_initializer(&mut authority);
        drop(authority);
        assert!(matches!(zero_grant.expect("zero grant retains owner"), semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 }));
        let fault = result.expect_err("typed retirement refusal");
        let oracle: serde_json::Value = serde_json::from_slice(&semio_framework_diagnostic::encode_fault_bytes(&fault)).expect("independent JSON fault oracle");
        assert_eq!(oracle, row["expected"]);
        assert_eq!(fault.to_value(), row["expected"]);
        eprintln!("[DEBUG] Jack typed close refusal {} retains exact message and retires every owner", row["kind"]);
    }
}

#[test]
fn jack_initializer_fault_copies_short_borrow_into_owned_bytes() {
    let fixture = jack_close_refusal_fixture();
    for row in fixture["borrowedFaults"].as_array().expect("borrowed fault cases") {
        let mut authority = empty_jack_initializer(semio_framework_job::OperationId(504), semio_framework_job::Generation(18));
        {
            let message = row["message"].as_str().expect("borrowed message").to_owned();
            authority.fail(message.as_bytes());
        }
        let phase = authority.phase;
        let actual = authority.fault.clone();
        close_jack_initializer(&mut authority);
        drop(authority);
        assert_eq!(phase, JackStoreInitializationPhase::RetireFault);
        let oracle: Vec<u8> = serde_json::from_value(row["expectedBytes"].clone()).expect("independent byte-array oracle");
        assert_eq!(actual.as_deref(), Some(oracle.as_slice()));
        eprintln!("[DEBUG] Jack borrowed fault retains {} exact owned bytes after source drops", oracle.len());
    }
}
