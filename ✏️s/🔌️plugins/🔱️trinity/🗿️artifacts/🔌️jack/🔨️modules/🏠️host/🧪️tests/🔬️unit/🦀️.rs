use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::standards::v1::subsets::any::schema::graph::GraphEffect;
use crate::standards::v1::subsets::any::schema::graph::{Edge, EntityRef, Node};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller, OriginalRetirementPump, ReplacementIdentity, StepContext};
use semio_framework_value::RetainedWorkPoll;
use crate::standards::v1::subsets::any::io::binary::mutations::{encode_op, decode_op};
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::graph::Port;
#[allow(unused_imports)]
use semio_framework_graph_manifest::PropertyDef;
#[allow(unused_imports)]
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
#[allow(unused_imports)]
use semio_framework_diagnostic::TextError;
#[allow(unused_imports)]
use semio_framework_value::retained_clone::admit_retained_clone_close;
#[allow(unused_imports)]
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::standards::v1::subsets::any::schema::host::*;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use crate::standards::v1::subsets::any::io::binary::mutations::new_trinity_graph_store;
use crate::standards::v1::subsets::any::schema::graph::TRINITY_GRAPH_SCHEMA;

#[semio_framework_async_macros::async_test]
async fn set_query_op_binary_round_trips_and_agrees_with_text() {
    let operation = set_query("MATCH (a:Piece) RETURN a.name".into());
    ::semio_os_store_test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}

#[semio_framework_async_macros::async_test]
async fn nakagin_document_text_round_trips_store_with_applied_operation() {
    let mut identity = semio_plugin::artifact_app_laws::fixture_identity();
    let envelope = create_document_envelope_for_test();
    let mut doc_store = new_trinity_graph_store(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store fixture");
    doc_store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![set_query("MATCH (a:Piece) RETURN a.name".into())], transaction: None }, &mut identity).await.expect("apply set-query");
    ::semio_os_store_test_support::assert_document_text_round_trip(&doc_store).await;
    ::semio_os_store_test_support::assert_document_pack_round_trip(&doc_store).await;
}

/// 🧫️ The empty Nakagin-manifest document the round-trip laws edit the query of.
fn create_document_envelope_for_test() -> store::os_store::store_schema::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation> {
    create_document_envelope::<JackSnapshot, TrinityGraphMutation>(TRINITY_GRAPH_SCHEMA, "doc-text-test", crate::standards::v1::subsets::any::schema::empty_jack_document::<semio_os_store_test_support::FixtureComposition>(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)
}
use store::os_store::document_materialize::create_document_envelope;

type JackStoreInitializationAuthority = store::ArtifactStoreReplayInitializer<JackSnapshot, TrinityGraphMutation, store::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>;

fn empty_jack_initializer(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> JackStoreInitializationAuthority {
    let envelope = create_document_envelope(TRINITY_GRAPH_SCHEMA, "jack-retained-load", crate::standards::v1::subsets::any::schema::empty_jack_document::<semio_os_store_test_support::FixtureComposition>(), None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    JackStoreInitializationAuthority::new(envelope, TRINITY_GRAPH_SCHEMA, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()))
}

fn step_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 131_072, maximum_capacity_bytes: 16_777_216, maximum_release_bytes: 1_048_576, maximum_depth: 64 }
}

/// 🧭️ The terminal a driven initializer lent, resolved through its own descriptor.
#[derive(Debug, PartialEq, Eq)]
enum DrivenOutcome {
    Complete,
    Cancelled,
    Fault,
}

struct InitializerCaller {
    caller: OriginalJobFixtureCaller,
    decoder: OriginalJobDecodeOwner,
}

impl InitializerCaller {
    fn new(operation: OperationId, generation: Generation) -> Self {
        let stock = RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 };
        let mut caller = OriginalJobFixtureCaller::admit(operation, generation, stock, || Some(1), u64::MAX, 1 << 24);
        caller.birth_returns(stock);
        caller.retain_dispatch().expect("original initializer dispatch");
        let decoder = caller.admitted_decoder();
        Self { caller, decoder }
    }

    fn retire(mut self) {
        let grant = step_grant();
        for _ in 0..100_000 {
            if self.decoder.terminal_is_empty() {
                self.caller.retire(grant);
                return;
            }
            let (quote, _) = semio_framework_job::original_decoder_teardown_quote(&self.decoder).expect("original initializer decoder close quote");
            self.caller.turn(quote, |cx| cx.with_original_retirement(|cx| self.decoder.close_on_source(cx))).expect("original initializer decoder retirement");
        }
        panic!("Jack initializer decoder did not reach terminal-empty close");
    }
}

fn drive_jack_initializer(authority: &mut JackStoreInitializationAuthority, caller: &mut InitializerCaller, current: Option<ReplacementIdentity>) -> DrivenOutcome {
    for _ in 0..100_000 {
        let outcome = caller.caller.refreshed_turn_with_decoder(step_grant(), &mut caller.decoder, |cx| {
            let mut step = |cx: &mut StepContext<'_>| {
                use semio_framework_job::JobOutcomeBorrow;
                let outcome = semio_plugin::ArtifactStoreInitializationAuthority::step(authority, cx)?;
                Ok(match outcome {
                    Some(JobOutcomeBorrow::Complete { .. }) => Some(DrivenOutcome::Complete),
                    Some(JobOutcomeBorrow::Cancelled { .. }) => Some(DrivenOutcome::Cancelled),
                    Some(JobOutcomeBorrow::Fault { .. }) => Some(DrivenOutcome::Fault),
                    _ => None,
                })
            };
            match current {
                Some(current) => cx.with_replacement_current(current, step),
                None => step(cx),
            }
        }).expect("Jack retained initializer turn");
        if let Some(outcome) = outcome {
            return outcome;
        }
    }
    panic!("Jack retained initializer did not reach a bounded terminal from {:?}", authority.phase)
}

fn close_jack_candidate(mut candidate: store::ArtifactStore<JackSnapshot, TrinityGraphMutation>) {
    use semio_plugin::ArtifactOwnedDisposer;

    let mut disposer = semio_plugin::ArtifactDocumentStoreDisposer::<JackSnapshot, TrinityGraphMutation>::new();
    for _ in 0..100_000 {
        let demand = disposer.retirement_demands(&candidate, semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES).expect("Jack candidate close quote");
        let grant = semio_plugin::plugin_self_funded_grant(demand, semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
        match disposer.close_step_cold(&mut candidate, grant).expect("Jack candidate close step") {
            semio_plugin::PluginLifecycleStep::Progress(progress) => assert!(progress.fits(grant)),
            semio_plugin::PluginLifecycleStep::Blocked { reason } => panic!("fresh Jack candidate close unexpectedly blocked: {reason}"),
            semio_plugin::PluginLifecycleStep::AwaitingInput { reason } => panic!("fresh Jack candidate close unexpectedly needs input: {reason}"),
            semio_plugin::PluginLifecycleStep::Complete(progress) => {
                assert!(progress.fits(grant));
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
fn jack_history_edit_initializer_aliases_genesis_and_publishes_exact_next_generation_and_candidate_closes_incrementally() {
    let operation = semio_framework_job::OperationId(501);
    let generation = semio_framework_job::Generation(13);
    let mut authority = empty_jack_initializer(operation, generation);
    let mut caller = InitializerCaller::new(operation, generation);
    let genesis = authority.envelope.as_ref().expect("retained Jack envelope").vcs.genesis.facts().share_snapshot();
    assert_eq!(drive_jack_initializer(&mut authority, &mut caller, None), DrivenOutcome::Complete);
    let candidate = semio_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("exact Jack candidate");
    assert_eq!(candidate.generation_now(), 14);
    assert!(std::sync::Arc::ptr_eq(&genesis, &candidate.snapshot_owner()));
    drop(genesis);
    close_jack_initializer(&mut authority, &mut caller);
    assert!(semio_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    close_jack_candidate(candidate);
    caller.retire();
}

#[test]
fn jack_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty() {
    let operation = semio_framework_job::OperationId(502);
    let generation = semio_framework_job::Generation(15);
    let mut cancelled = empty_jack_initializer(operation, generation);
    let mut caller = InitializerCaller::new(operation, generation);
    semio_plugin::ArtifactStoreInitializationAuthority::request_cancel(&mut cancelled);
    assert_eq!(drive_jack_initializer(&mut cancelled, &mut caller, None), DrivenOutcome::Cancelled);
    assert!(semio_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&cancelled));
    drop(cancelled);

    let saved = ReplacementIdentity::new(1, operation, generation);
    let current = ReplacementIdentity::new(1, operation, Generation(generation.0 + 1));
    let mut stale = empty_jack_initializer(operation, generation).with_replacement(Some(saved));
    assert_eq!(drive_jack_initializer(&mut stale, &mut caller, Some(current)), DrivenOutcome::Cancelled);
    close_jack_initializer(&mut stale, &mut caller);
    assert!(semio_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&stale));
    drop(stale);
    caller.retire();
}

/// ♻️ Drives one admitted owned value turn by turn under exactly its own published quote and counts the turns.
fn drive_owned_retirement<T: semio_framework_value::retirement::RetireOwned>(value: T) -> usize {
    use semio_framework_value::retirement::{admit_owned_retirement, controlled::controlled_retirement_birth_demand};
    let pump = OriginalRetirementPump::new();
    let birth = semio_plugin::plugin_self_funded_grant(controlled_retirement_birth_demand(&value).expect("owned retirement birth quote"), semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
    let mut original = Some(value);
    let (turn, _) = pump.turn(8, u64::MAX, || Some(0), birth).expect("owned retirement birth turn")
        .with_effect(|effect| admit_owned_retirement(&mut original, effect)).expect("owned retirement birth effect")
        .expect("funded owned retirement birth");
    let (mut retirement, receipt) = turn.result.expect("owned retirement admission").expect("owned retirement cursor");
    assert!(receipt.fits(birth));
    for steps in 0..100_000 {
        if retirement.terminal_is_empty() {
            return steps;
        }
        let body = semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES;
        let grant = semio_plugin::plugin_self_funded_grant(retirement.next_demand(body).expect("owned retirement quote"), semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
        let work = retirement.next_computational_work(body).expect("owned retirement work quote");
        let (poll, _) = pump.turn(work.max(1), u64::MAX, || Some(0), grant).expect("owned retirement turn")
            .with_effect(|effect| retirement.close_step(body, effect)).expect("owned retirement effect")
            .expect("funded owned retirement turn");
        if let RetainedWorkPoll::Complete(turn) = poll {
            assert!(turn.result.expect("bounded owned retirement").progress().fits(grant));
        }
    }
    panic!("owned retirement did not reach its bounded terminal")
}

#[test]
fn jack_nested_query_effect_retires_one_exact_owner_per_grant() {
    let mut object = PropertyBag::new();
    object.insert("nested".repeat(32), PropertyValue::Array(vec![PropertyValue::String("payload".repeat(128)), PropertyValue::String("tail".into())]));
    let effect = GraphEffect::SetProperty { entity: EntityRef::Node("node".repeat(64)), key: "key".repeat(64), value: PropertyValue::Object(object) };
    assert!(drive_owned_retirement(effect) > 1);
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
    drive_owned_retirement(value)
}

#[test]
fn query_ownership_shared_scene_clone_retires_while_source_remains_live() {
    let mut source = crate::standards::v1::subsets::any::schema::empty_jack_document::<semio_os_store_test_support::FixtureComposition>();
    semio_os_store_test_support::materialize_fixture(&mut source.content, vec![Node { id: "live".into(), kind: "Apartment".into(), name: "Live".into(), x: 0.0, y: 0.0, width: 1.0, height: 1.0, properties: PropertyBag::new(), ports: Vec::new() }], Vec::new());
    let source_owner = source.content.local_owner::<crate::standards::v1::subsets::any::schema::content::JackContentOwner>().expect("source content owner");
    let clone = drive_snapshot_clone(&source);
    let clone_owner = clone.content.local_owner::<crate::standards::v1::subsets::any::schema::content::JackContentOwner>().expect("clone content owner");
    assert!(std::sync::Arc::ptr_eq(&source_owner, &clone_owner));
    drop(source_owner);
    drop(clone_owner);
    let steps = drive_snapshot_retirement(clone);
    assert_eq!(source.content.local_owner::<crate::standards::v1::subsets::any::schema::content::JackContentOwner>().expect("live source owner remains").read().node_id(0).unwrap(), "live");
    assert!(steps > 1);
    drive_snapshot_retirement(source);
}

/// ⏱️ A snapshot names its interned manifest instead of owning a copy, so cloning and retiring a Nakagin document never walks the 42 kinds a retired snapshot used to carry.
#[test]
fn nakagin_snapshot_clone_and_retirement_do_not_walk_the_manifest() {
    let manifest = crate::standards::v1::subsets::any::schema::manifest::Manifest::nakagin_default();
    let manifest_steps = drive_owned_retirement(manifest);
    let source = crate::standards::v1::subsets::any::schema::empty_jack_document::<semio_os_store_test_support::FixtureComposition>();
    assert!(std::sync::Arc::ptr_eq(&source.manifest().unwrap(), &source.manifest().unwrap()));
    let clone = drive_snapshot_clone(&source);
    assert!(std::sync::Arc::ptr_eq(&clone.manifest().unwrap(), &source.manifest().unwrap()));
    let snapshot_steps = drive_snapshot_retirement(clone);
    assert!(snapshot_steps * 8 < manifest_steps, "{snapshot_steps} snapshot turns against {manifest_steps} manifest turns");
    drive_snapshot_retirement(source);
}

#[test]
fn query_ownership_unique_scene_retirement_drains_entities_one_owner_per_grant() {
    let mut source = crate::standards::v1::subsets::any::schema::empty_jack_document::<semio_os_store_test_support::FixtureComposition>();
    semio_os_store_test_support::materialize_fixture(
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
    ::semio_os_store_test_support::assert_op_line_round_trip(&set_query("MATCH (a:Piece) WHERE a.name = 'b' RETURN a".into()));
}

#[semio_framework_async_macros::async_test]
async fn parse_op_rejects_unknown_keyword() {
    let err = TrinityGraphMutation::parse_op("bogusOp x").expect_err("unknown op");
    assert!(err.message.contains("unknown mutation line"));
}

#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    let mut identity = semio_plugin::artifact_app_laws::fixture_identity();
    use protocol::{ArtifactId, Edit, SchemaId};

    let mut store = new_trinity_graph_store(create_document_envelope_for_test(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("valid artifact store");
    crate::standards::v1::subsets::any::schema::operations::dispatch_trinity_graph_mutations(&mut store, vec![set_query("MATCH (a:Piece) RETURN a".into())], &mut identity).await.expect("apply set query");
    let edit: &Edit<TrinityGraphMutation> = store.envelope().vcs.edits.last().expect("published query edit");
    ::semio_os_store_test_support::assert_command_envelope_round_trip::<JackSnapshot, TrinityGraphMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone())).await;
}


fn close_jack_initializer(authority: &mut JackStoreInitializationAuthority, caller: &mut InitializerCaller) {
    for _ in 0..100_000 {
        if semio_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(authority) {
            return;
        }
        semio_plugin::ArtifactStoreInitializationAuthority::begin_close(authority);
        let demand = semio_plugin::ArtifactStoreInitializationAuthority::retirement_demands(authority, semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES).expect("Jack initializer close quote");
        let grant = semio_plugin::plugin_self_funded_grant(demand, semio_plugin::PLUGIN_SELF_FUNDED_BODY_BYTES);
        let step = caller.caller.refreshed_turn_with_decoder(grant, &mut caller.decoder, |cx| cx.with_original_retirement(|cx| semio_plugin::ArtifactStoreInitializationAuthority::close_step(authority, cx))).expect("Jack initializer close");
        assert!(step.progress().fits(grant));
    }
    panic!("Jack initializer did not reach terminal-empty close")
}
