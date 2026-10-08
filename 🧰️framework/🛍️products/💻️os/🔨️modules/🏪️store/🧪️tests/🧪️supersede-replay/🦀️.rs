//! ✏️ Store laws of non-destructive history editing: effective forwards under supersessions, Report-mode replays with
//! per-mutation outcomes, the finalize gate, scoped and unscoped supersessions, replica convergence, cancellation,
//! persistence, convergence early exit and the prefix-snapshot ring (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
use super::*;

const FIXTURE_IDENTITY_BYTE_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;

pub(super) async fn fixture_author<P, M>(store: &mut ArtifactStore<P, M>, command: ArtifactCommand<M>) -> Result<CommandReceipt, VcsError>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let mut observed = 0;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
        assert!(progress.owned_bytes <= FIXTURE_IDENTITY_BYTE_CEILING);
        observed = progress.owned_bytes;
        true
    };
    let mut identity = crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(FIXTURE_IDENTITY_BYTE_CEILING, &mut observer).expect("declared fixture identity authority");
    let result = store.dispatch(command, &mut identity).await;
    drop(identity.pause().expect("original cumulative fixture identity receipt"));
    drop(observer);
    eprintln!("[DEBUG] fixture caller identity owned={observed} ceiling={FIXTURE_IDENTITY_BYTE_CEILING}");
    result
}

fn planning_grant(demand:semio_framework_value::RetirementDemand)->semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}

fn native_retirement_grant(owner: &dyn ErasedSnapshotRetirement, maximum_items: usize, release_granularity: usize) -> semio_framework_value::retained_clone::RetainedCloneGrant {
    let copy = owner.next_copy_byte_demand().expect("original native copy demand");
    let capacity = owner.next_capacity_byte_demand(copy).expect("original native capacity demand");
    assert!(copy.saturating_add(capacity) <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release_granularity.max(owner.next_release_byte_demand().expect("original native whole backing demand")), maximum_depth: owner.next_depth_demand().expect("original native depth demand") }
}

//#region 🧰️Harness
async fn demo_store(id: &str, n: Option<i32>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n }, None)).await
}

async fn apply(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, mutations: Vec<DemoMutation>) {
    fixture_author(store, ArtifactCommand::Apply { mutations, transaction: None }).await.expect("a clean edit applies");
}

fn set(n: i32) -> DemoMutation {
    DemoMutation::SetN(SetN { n })
}

fn add(delta: i32) -> DemoMutation {
    DemoMutation::AddN(AddN { delta })
}

fn operation_ids(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> Vec<MutationId> {
    store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect()
}

fn input(target: &MutationId, replacement: Option<DemoMutation>) -> SupersedeInput<DemoMutation> {
    SupersedeInput { target: target.clone(), replacement }
}

fn draft(replacement: Option<DemoMutation>) -> protocol::InputReplacement {
    match replacement {
        Some(operation) => protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: operation.encode_op().expect("demo operations encode") },
        None => protocol::InputReplacement::Withdrawn,
    }
}

/// 🔡️ The bytes a lowercase hex string spells.
fn hex(text: &str) -> Vec<u8> {
    text.as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("ascii hex"), 16).expect("hex digit pair")).collect()
}

fn drafts(entries: &[(&MutationId, Option<DemoMutation>)]) -> protocol::HistoryInputDrafts {
    entries.iter().map(|(target, replacement)| ((*target).clone(), draft(replacement.clone()))).collect()
}

fn finish(store: &ArtifactStore<DemoSnapshot, DemoMutation>, mut replay: EditReplay<DemoSnapshot, DemoMutation>) -> EditReplayResult<DemoSnapshot, DemoMutation> {
    drive_test_report_replay(&mut replay, store.replay_edits());
    replay.finish().expect("a finished replay yields its result")
}

/// 📋️ Per-mutation outcomes keyed by identity: what replicas with different edit boundaries must agree on.
fn outcomes_by_mutation(outcomes: &[protocol::MutationReplayOutcome]) -> BTreeMap<MutationId, (Option<semio_framework_diagnostic::Severity>, Vec<String>, bool, bool)> {
    outcomes.iter().map(|outcome| (outcome.mutation_id.clone(), (outcome.worst, outcome.messages.iter().map(|message| message.code.0.clone()).collect(), outcome.superseded, outcome.withdrawn))).collect()
}

/// 📋️ Per-mutation outcomes by applied position and operation index: what two independently authored stores agree on.
fn outcomes_by_position(store: &ArtifactStore<DemoSnapshot, DemoMutation>, outcomes: &[protocol::MutationReplayOutcome]) -> Vec<(usize, u32, Option<semio_framework_diagnostic::Severity>, Vec<String>, bool, bool)> {
    outcomes
        .iter()
        .map(|outcome| {
            (
                store.applied_edit_ids().iter().position(|id| *id == outcome.edit_id).expect("an outcome names an applied edit"),
                outcome.op_index,
                outcome.worst,
                outcome.messages.iter().map(|message| message.code.0.clone()).collect(),
                outcome.superseded,
                outcome.withdrawn,
            )
        })
        .collect()
}
//#endregion 🧰️Harness

//#region 🧪️Corpus
/// 🧫️ The language-agnostic corpus: the store's Report replay reproduces every case the independent fast-json-patch
/// oracle (`🟦️.ts` beside this file) reproduces, a clean case finalizes to the same state and durable outcomes, and a
/// blocking case is refused with nothing changed.
#[semio_framework_async_macros::async_test]
async fn supersede_replay_corpus_matches_the_store() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️supersede-replay/🔣️.json")).expect("corpus parses");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let initial: Option<i32> = serde_json::from_value(case["initial"]["n"].clone()).expect("initial n");
        let mut store = demo_store(name, initial).await;
        store.enable_convergence_early_exit();
        for edit in case["edits"].as_array().expect("edits") {
            apply(&mut store, edit.as_array().expect("edit operations").iter().map(|operation| DemoMutation::from_value(operation.clone().into()).expect("corpus operations decode")).collect()).await;
        }
        let operations: Vec<(usize, u32, MutationId)> = store.mutation_ops().expect("operations").into_iter().map(|operation| (operation.position, operation.op_index, operation.mutation_id)).collect();
        let id_of = |edit: u64, op: u64| operations.iter().find(|(position, index, _)| *position as u64 == edit && u64::from(*index) == op).map(|(_, _, id)| id.clone()).expect("the corpus names a recorded operation");
        let drafts: protocol::HistoryInputDrafts = case["supersessions"]
            .as_array()
            .expect("supersessions")
            .iter()
            .map(|supersession| {
                let replacement = match &supersession["replacement"] {
                    serde_json::Value::String(withdrawn) if withdrawn == "withdrawn" => protocol::InputReplacement::Withdrawn,
                    serde_json::Value::Object(input) if input.contains_key("invalid") => {
                        protocol::InputReplacement::Input { schema: input["invalid"]["schema"].as_str().expect("invalid schema").into(), payload: hex(input["invalid"]["payloadHex"].as_str().expect("invalid payload")) }
                    }
                    operation => draft(Some(DemoMutation::from_value(operation.clone().into()).expect("replacement decodes"))),
                };
                (id_of(supersession["edit"].as_u64().unwrap(), supersession["op"].as_u64().unwrap()), replacement)
            })
            .collect();
        let replay = store.begin_report_replay(&drafts, Some(&operations[0].2)).expect("the replay begins at the genesis");
        let result = finish(&store, replay);
        let report = store.replay_report(&result).expect("report");
        let expected = &case["expected"];
        let expected_n: Option<i32> = serde_json::from_value(expected["state"]["n"].clone()).expect("expected n");
        assert_eq!(result.state().expect("reached state").n, expected_n, "{name}: reached state");
        let observed: Vec<serde_json::Value> = outcomes_by_position(&store, &report.outcomes)
            .into_iter()
            .map(|(edit, op, worst, codes, superseded, withdrawn)| serde_json::json!({ "edit": edit, "op": op, "worst": worst.map(|level| format!("{level:?}").to_lowercase()), "codes": codes, "superseded": superseded, "withdrawn": withdrawn }))
            .collect();
        assert_eq!(serde_json::Value::Array(observed), expected["outcomes"], "{name}: per-mutation outcomes");
        assert_eq!(report.blocks_finalize(), expected["blocksFinalize"].as_bool().unwrap(), "{name}: finalize gate");
        let generation = store.generation();
        match store.commit_finished_replay(result, HistoryFinalization::Overwrite).await {
            Ok(_) => {
                assert!(!report.blocks_finalize(), "{name}: a blocking report never commits");
                assert_eq!(store.snapshot_ref().n, expected_n, "{name}: finalized state");
                test_support::assert_live_equals_replay(&store).await;
                let durable = store.mutation_outcomes().expect("durable outcomes");
                assert_eq!(outcomes_by_position(&store, &durable), outcomes_by_position(&store, &report.outcomes), "{name}: durable outcomes equal the report");
            }
            Err(VcsError::Rejected { policy, messages }) => {
                assert!(report.blocks_finalize(), "{name}: only a blocking report is refused");
                assert_eq!(policy, crate::os_spr::MergePolicy::Normal);
                assert!(messages.iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error));
                assert_eq!(store.generation(), generation, "{name}: a refused finalize changes nothing");
                assert!(store.supersessions().is_empty());
            }
            Err(error) => panic!("{name}: unexpected finalize failure {error}"),
        }
    }
}
//#endregion 🧪️Corpus

//#region 🧪️SupersedeLaws
/// ✏️ Superseding an interior operation keeps every slot, changes the revision, and leaves exactly the state and durable
/// messages a fresh store folding the edited log reaches; both persisted forms round-trip it.
#[semio_framework_async_macros::async_test]
async fn interior_supersede_equals_a_fresh_replay_of_the_edited_log() {
    let mut store = demo_store("interior", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3), add(4)]).await;
    let ids = operation_ids(&store);
    let applied = store.applied_edit_ids().to_vec();
    let revision = store.0.content_revision();
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("a clean supersession authors");
    assert_eq!(store.snapshot_ref().n, Some(19));
    assert_eq!(store.applied_edit_ids(), applied.as_slice(), "a supersession keeps every slot");
    assert_ne!(store.0.content_revision(), revision, "the revision names the effective forwards");
    assert_eq!(store.supersessions().len(), 1);
    assert_eq!(store.0.last_projection_cause, Some(ArtifactProjectionCause::Replay));
    let transition = store.envelope().transitions.last().expect("the supersession is an event");
    assert_eq!(transition.dependencies, vec![ids[0].clone()], "a supersession depends on its targets");
    assert_eq!(transition.target, vec!["n".to_string()], "a supersession declares the address it writes");
    test_support::assert_live_equals_replay(&store).await;
    let mut fresh = demo_store("interior-fresh", Some(0)).await;
    apply(&mut fresh, vec![set(10)]).await;
    apply(&mut fresh, vec![add(2)]).await;
    apply(&mut fresh, vec![add(3), add(4)]).await;
    assert_eq!(store.snapshot().expect("edited"), fresh.snapshot().expect("fresh"));
    assert_eq!(
        outcomes_by_position(&store, &store.mutation_outcomes().unwrap()).into_iter().map(|(edit, op, worst, codes, _, _)| (edit, op, worst, codes)).collect::<Vec<_>>(),
        outcomes_by_position(&fresh, &fresh.mutation_outcomes().unwrap()).into_iter().map(|(edit, op, worst, codes, _, _)| (edit, op, worst, codes)).collect::<Vec<_>>()
    );
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
    let files = print_document_pack(store.envelope()).await.expect("pack prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pack parses").into_envelope()).await;
    assert_eq!(reloaded.snapshot().expect("reloaded"), store.snapshot().expect("live"));
    assert_eq!(reloaded.supersessions().keys().collect::<Vec<_>>(), store.supersessions().keys().collect::<Vec<_>>());
    assert_eq!(reloaded.envelope().transitions, store.envelope().transitions, "the reload re-derives every supersession target");
    assert_eq!(reloaded.mutation_outcomes().unwrap(), store.mutation_outcomes().unwrap());
    let log = crate::os_spr::decode_history(&files.spr, &crate::os_spr::DecodeOptions::default()).await.expect("spr decodes");
    assert!(log.fold().expect("the persisted log folds").supersessions.contains_key(&ids[0]), "HistoryLog::fold resolves the supersession");
}

/// 🛂️ Hub check-in replays a ledger under `Normal`: a remote supersession whose downstream replay raises an `Error` is
/// refused as `Rejected { Normal }`, a clean one folds into the next pair.
#[semio_framework_async_macros::async_test]
async fn check_in_refuses_a_supersession_whose_replay_blocks_under_normal() {
    let mut store = demo_store("check-in", None).await;
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let supersede = |replacement: Option<DemoMutation>, logical: u64| {
        let transition = crate::os_spr::HistoryTransition::Supersede(protocol::TransitionSupersede { scope: None, inputs: vec![protocol::SupersededInput { target: ids[0].clone(), replacement: draft(replacement) }] });
        crate::os_spr::encode_envelopes(&[crate::os_spr::history_transition_envelope(
            &transition,
            &ArtifactId("check-in".into()),
            &ActorId("editor".into()),
            vec![ids[0].clone()],
            HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical },
        )])
    };
    let refused = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &supersede(None, 0), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "{refused:?}");
    assert!(refused.unwrap_err().to_string().contains("rejected by merge policy Normal"));
    let accepted = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &supersede(Some(DemoMutation::AssignN(AssignN { n: Some(40) })), 1), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>)
        .await
        .expect("a clean supersession checks in");
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&accepted.pack, &accepted.spr).await.expect("checked-in pair parses");
    assert_eq!(parsed.snapshot.n, Some(41));
    test_support::retire_parsed_document(parsed);
}

/// 🚦️ Warning, Error and Fatal downstream outcomes are reported per mutation; Error and Fatal refuse the supersession
/// atomically with `Rejected { Normal }`, a Warning commits and stays in the durable history.
#[semio_framework_async_macros::async_test]
async fn downstream_warning_error_and_fatal_outcomes_are_reported_per_mutation() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "severity", DemoSnapshot { n: Some(0) }, None)).await;
    for operation in [SeverityMutation::SetN(SeveritySetN { n: 1 }), SeverityMutation::SetN(SeveritySetN { n: 2 }), SeverityMutation::SetN(SeveritySetN { n: 3 })] {
        fixture_author(&mut store, ArtifactCommand::Apply { mutations: vec![operation], transaction: None }).await.expect("clean severity edit");
    }
    let ids: Vec<MutationId> = store.mutation_ops().unwrap().into_iter().map(|operation| operation.mutation_id).collect();
    let severity_draft = |operation: SeverityMutation| protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: operation.encode_op().unwrap() };
    let entries = protocol::HistoryInputDrafts::from([
        (ids[0].clone(), severity_draft(SeverityMutation::SetWarningN(SetWarningN { n: 5 }))),
        (ids[1].clone(), severity_draft(SeverityMutation::SetErrorN(SetErrorN { n: 6 }))),
        (ids[2].clone(), severity_draft(SeverityMutation::SetFatalN(SetFatalN { n: 7 }))),
    ]);
    let mut replay = store.begin_report_replay(&entries, None).expect("preview replay");
    drive_test_report_replay(&mut replay, store.replay_edits());
    let result = replay.finish().expect("finished");
    let report = store.replay_report(&result).expect("report");
    let levels: Vec<Option<semio_framework_diagnostic::Severity>> = report.outcomes.iter().map(|outcome| outcome.worst).collect();
    assert_eq!(levels, vec![Some(semio_framework_diagnostic::Severity::Warning), Some(semio_framework_diagnostic::Severity::Error), Some(semio_framework_diagnostic::Severity::Fatal)]);
    assert!(report.blocks_finalize());
    assert_eq!(report.worst, Some(semio_framework_diagnostic::Severity::Fatal));
    drop(result);
    let generation = store.generation();
    let snapshot = store.snapshot().unwrap();
    for (target, operation) in [(&ids[1], SeverityMutation::SetErrorN(SetErrorN { n: 6 })), (&ids[2], SeverityMutation::SetFatalN(SetFatalN { n: 7 }))] {
        let refused = fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: target.clone(), replacement: Some(operation) }] }).await;
        assert!(matches!(refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, ref messages }) if !messages.is_empty()), "{refused:?}");
        assert_eq!(store.generation(), generation, "a refused supersession changes nothing");
        assert_eq!(store.snapshot().unwrap(), snapshot);
        assert!(store.supersessions().is_empty());
    }
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: ids[0].clone(), replacement: Some(SeverityMutation::SetWarningN(SetWarningN { n: 5 })) }] }).await.expect("a warning never blocks");
    let durable = store.mutation_outcomes().unwrap();
    assert_eq!(durable[0].worst, Some(semio_framework_diagnostic::Severity::Warning), "the warning persists in history");
    assert!(durable[0].superseded);
    assert_eq!(store.snapshot_ref().n, Some(3), "downstream sets still win");
}

/// 🚫️ A withdrawn operation folds as a no-op, keeps its slot, and a later supersession restores it.
#[semio_framework_async_macros::async_test]
async fn withdrawing_an_operation_folds_it_as_a_no_op() {
    let mut store = demo_store("withdraw", Some(0)).await;
    apply(&mut store, vec![set(5)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], None)] }).await.expect("withdraw");
    assert_eq!(store.snapshot_ref().n, Some(6));
    let outcome = store.mutation_outcomes().unwrap().into_iter().find(|outcome| outcome.mutation_id == ids[1]).expect("withdrawn outcome");
    assert!(outcome.withdrawn && outcome.superseded && outcome.messages.is_empty());
    test_support::assert_live_equals_replay(&store).await;
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], Some(add(20)))] }).await.expect("a later supersession wins");
    assert_eq!(store.snapshot_ref().n, Some(26));
    test_support::assert_document_text_round_trip(&store).await;
}

/// 🌿️ A scoped supersession is effective exactly while its alternative is active; an unscoped one in every alternative.
#[semio_framework_async_macros::async_test]
async fn scoped_supersessions_follow_their_alternative_and_unscoped_ones_every_alternative() {
    let mut store = demo_store("scoped", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "variant".into(), inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("variant alternative");
    let variant = store.envelope().active_alternative_id.clone().expect("the variant is active");
    assert_eq!(store.snapshot_ref().n, Some(11));
    assert_eq!(store.supersessions().get(&ids[0]).and_then(|supersession| supersession.scope.clone()), Some(variant.clone()));
    let outbound = store.envelope().transitions.len();
    assert_eq!(outbound, 3, "commit, branch and scoped supersede are one authored batch");
    fixture_author(&mut store, ArtifactCommand::CreateAlternative { name: "other".into() }).await.expect("another alternative at the head");
    assert_eq!(store.snapshot_ref().n, Some(2), "the scoped supersession leaves with its alternative");
    assert!(store.supersessions().is_empty());
    test_support::assert_live_equals_replay(&store).await;
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: variant.clone() }).await.expect("switch back");
    assert_eq!(store.snapshot_ref().n, Some(11), "switching back re-applies the scoped supersession");
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], Some(add(5)))] }).await.expect("unscoped");
    assert_eq!(store.snapshot_ref().n, Some(15));
    let other = store.envelope().vcs.alternatives.iter().find(|alternative| alternative.name == "other").map(|alternative| alternative.id.clone()).expect("other alternative");
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: other }).await.expect("switch to other");
    assert_eq!(store.snapshot_ref().n, Some(6), "an unscoped supersession holds in every alternative");
    test_support::assert_live_equals_replay(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
    let refused = fixture_author(&mut store, ArtifactCommand::Supersede { scope: Some("missing".into()), inputs: vec![input(&ids[1], Some(add(5)))] }).await;
    assert!(matches!(refused, Err(VcsError::UnknownAlternative(_))));
}

/// 🔀️ A supersession converges on every replica whatever order its events arrive in, also against a concurrent edit.
#[semio_framework_async_macros::async_test]
async fn replicas_converge_on_a_supersession_under_shuffled_arrival() {
    let mut author = demo_store("converge", Some(0)).await;
    apply(&mut author, vec![set(1)]).await;
    apply(&mut author, vec![add(2)]).await;
    apply(&mut author, vec![add(3)]).await;
    let ids = operation_ids(&author);
    let mut peer = demo_store("converge", Some(0)).await;
    for event in author.event_log().expect("log") {
        peer.ingest_remote(event).await.expect("peer ingests the edits");
    }
    apply(&mut peer, vec![add(100)]).await;
    fixture_author(&mut author, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(7)))] }).await.expect("supersede");
    let log = author.event_log().expect("author log");
    for event in peer.event_log().expect("peer log") {
        author.ingest_remote(event).await.expect("author ingests the concurrent edit");
    }
    for event in log.iter().rev().cloned() {
        peer.ingest_remote(event).await.expect("peer ingests out of order");
    }
    assert_eq!(author.snapshot().unwrap(), peer.snapshot().unwrap());
    assert_eq!(author.snapshot_ref().n, Some(112));
    assert_eq!(author.applied_edit_ids().len(), peer.applied_edit_ids().len());
    assert_eq!(author.supersessions().keys().collect::<Vec<_>>(), peer.supersessions().keys().collect::<Vec<_>>());
    assert_eq!(outcomes_by_mutation(&author.mutation_outcomes().unwrap()), outcomes_by_mutation(&peer.mutation_outcomes().unwrap()));
    for rotation in 0..log.len() {
        let mut replica = demo_store("converge", Some(0)).await;
        let mut shuffled = log.clone();
        shuffled.rotate_left(rotation);
        for event in shuffled {
            replica.ingest_remote(event).await.expect("replica ingests");
        }
        assert_eq!(replica.applied_edit_ids().len(), 3, "rotation {rotation}");
        assert_eq!(replica.snapshot_ref().n, Some(12), "rotation {rotation}");
        assert!(replica.supersessions().contains_key(&ids[0]), "rotation {rotation}");
    }
}

/// 🛑️ Cancelling a Report replay after any number of steps leaves the store exactly as it was, and every scratch owner
/// retires (the test store closes to its terminal-empty witness).
#[semio_framework_async_macros::async_test]
async fn cancelling_a_report_replay_at_every_step_leaves_the_store_untouched() {
    let mut store = demo_store("cancel", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2), add(3)]).await;
    apply(&mut store, vec![add(4)]).await;
    let ids = operation_ids(&store);
    let entries = drafts(&[(&ids[0], Some(set(9)))]);
    let mut probe = store.begin_report_replay(&entries, None).expect("probe").with_retirement_factories(Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoSnapshot>::default()), Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoMutation>::default()));
    let total = probe.progress().total;
    probe.cancel();
    super::super::raw_replay_retirement::tests::close(&mut probe);
    assert_eq!(total, 7);
    for stop in 0..=total {
        let (generation, revision, snapshot, applied, transitions, outcomes) =
            (store.generation(), store.0.content_revision(), store.snapshot().unwrap(), store.applied_edit_ids().to_vec(), store.envelope().transitions.len(), store.mutation_outcomes().unwrap());
        let mut replay = store.begin_report_replay(&entries, None).expect("replay").with_retirement_factories(Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoSnapshot>::default()), Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<DemoMutation>::default()));
        if stop > 0 {
            while replay.progress().done < stop {
                assert!(replay.step(store.replay_edits(), &mut || true).is_ok());
            }
            assert_eq!(replay.progress().done, stop);
        }
        replay.cancel();
        assert_eq!(store.generation(), generation);
        assert_eq!(store.0.content_revision(), revision);
        assert_eq!(store.snapshot().unwrap(), snapshot);
        assert_eq!(store.applied_edit_ids(), applied.as_slice());
        assert_eq!(store.envelope().transitions.len(), transitions);
        assert_eq!(store.mutation_outcomes().unwrap(), outcomes);
        super::super::raw_replay_retirement::tests::close(&mut replay);
    }
}

/// 🏁️ Convergence early exit reports and finalizes exactly what the full replay does.
#[semio_framework_async_macros::async_test]
async fn convergence_early_exit_equals_the_full_replay() {
    let mut stores = Vec::new();
    for early in [true, false] {
        let mut store = demo_store(if early { "early" } else { "full" }, Some(0)).await;
        if early {
            store.enable_convergence_early_exit();
        }
        apply(&mut store, vec![set(1)]).await;
        apply(&mut store, vec![set(2)]).await;
        for _ in 0..5 {
            apply(&mut store, vec![add(1)]).await;
        }
        let ids = operation_ids(&store);
        store.state_before(&ids[6], &protocol::HistoryInputDrafts::new()).map(|preview| drive_retirement_terminal(admit_demo_alias_retirement(&store,preview))).expect("the preview populates the prefix ring");
        stores.push((store, ids));
    }
    let mut reports = Vec::new();
    for (store, ids) in &stores {
        let replay = store.begin_report_replay(&drafts(&[(&ids[0], Some(set(50)))]), None).expect("replay");
        let result = finish(store, replay);
        reports.push((result.converged_at(), result.state().map(|state| state.n), outcomes_by_position(store, &store.replay_report(&result).unwrap().outcomes)));
    }
    assert!(reports[0].0.is_some_and(|length| length < 7), "the early store converges before the head: {:?}", reports[0].0);
    assert_eq!(reports[1].0, None, "the full store replays every edit");
    assert_eq!(reports[0].1, reports[1].1);
    assert_eq!(reports[0].2, reports[1].2);
    for (store, ids) in &mut stores {
        let target = ids[0].clone();
        fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&target, Some(set(50)))] }).await.expect("supersede");
    }
    assert_eq!(stores[0].0.snapshot().unwrap(), stores[1].0.snapshot().unwrap());
    assert_eq!(outcomes_by_position(&stores[0].0, &stores[0].0.mutation_outcomes().unwrap()), outcomes_by_position(&stores[1].0, &stores[1].0.mutation_outcomes().unwrap()));
    test_support::assert_live_equals_replay(&stores[0].0).await;
}

/// 🕰️ `state_before` is the projection before an operation, with drafts over the effective inputs and nothing downstream.
#[semio_framework_async_macros::async_test]
async fn state_before_answers_the_projection_before_an_operation_under_drafts() {
    let mut store = demo_store("before", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3), add(4)]).await;
    let ids = operation_ids(&store);
    let mut observe = |target: &MutationId, entries: protocol::HistoryInputDrafts| {
        let preview = store.state_before(target, &entries).expect("preview");
        let n = preview.n;
        drive_retirement_terminal(admit_demo_alias_retirement(&store,preview));
        n
    };
    assert_eq!(observe(&ids[0], protocol::HistoryInputDrafts::new()), Some(0));
    assert_eq!(observe(&ids[3], protocol::HistoryInputDrafts::new()), Some(6));
    assert_eq!(observe(&ids[3], drafts(&[(&ids[0], Some(set(10)))])), Some(15));
    assert_eq!(observe(&ids[2], drafts(&[(&ids[1], None)])), Some(1));
    assert!(store.prefix_ring.iter().all(|entry| entry.length <= 2), "only live prefixes are retained");
    assert!(!store.prefix_ring.is_empty());
    assert_eq!(store.snapshot_ref().n, Some(10), "a preview never touches the live projection");
}

/// 🏁️ A finished session replay commits without replaying again, refuses when stale or blocking, and finalizes as a new
/// alternative just like an overwrite.
#[semio_framework_async_macros::async_test]
async fn finished_replays_commit_atomically_and_refuse_stale_or_blocking_ones() {
    let mut store = demo_store("finish", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    let ids = operation_ids(&store);
    let stale = finish(&store, store.begin_report_replay(&drafts(&[(&ids[0], Some(set(10)))]), None).expect("stale replay"));
    apply(&mut store, vec![add(3)]).await;
    assert!(matches!(store.commit_finished_replay(stale, HistoryFinalization::Overwrite).await, Err(VcsError::Stale { .. })));
    let blocking = finish(&store, store.begin_report_replay(&drafts(&[(&ids[0], Some(DemoMutation::DeleteN(DeleteN {})))]), None).expect("blocking replay"));
    let generation = store.generation();
    assert!(matches!(store.commit_finished_replay(blocking, HistoryFinalization::Overwrite).await, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, .. })));
    assert_eq!(store.generation(), generation);
    let clean = finish(&store, store.begin_report_replay(&drafts(&[(&ids[0], Some(set(10)))]), None).expect("clean replay"));
    let receipt = store.commit_finished_replay(clean, HistoryFinalization::Overwrite).await.expect("overwrite");
    assert!(receipt.generation > generation);
    assert_eq!(store.snapshot_ref().n, Some(15));
    test_support::assert_live_equals_replay(&store).await;
    let variant = finish(&store, store.begin_report_replay(&drafts(&[(&ids[1], Some(add(20)))]), None).expect("variant replay"));
    store.commit_finished_replay(variant, HistoryFinalization::Alternative { name: "variant".into() }).await.expect("new alternative");
    assert_eq!(store.snapshot_ref().n, Some(33));
    assert!(store.envelope().active_alternative_id.is_some());
    test_support::assert_live_equals_replay(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
}

/// 🧭️ Ring entries a supersession makes stale are evicted through the snapshot retirement factory — never dropped — and
/// the store still closes to its exact terminal-empty witness.
#[semio_framework_async_macros::async_test]
async fn prefix_ring_evictions_retire_through_the_snapshot_factory() {
    let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "ring", DemoSnapshot { n: Some(0) }, None)).await;
    store.install_document_store_owners_exact(DocumentStoreOwners::new(
        Arc::new(ExactDemoSnapshotRetirementFactory(Arc::clone(&completed))),
        Arc::new(DemoInitialSnapshotRetirementFactory),
        Arc::new(DemoMutationRetirementFactory),
        Box::new(ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new()),
    ));
    for n in 1..=6 {
        apply(&mut store, vec![set(n)]).await;
    }
    let ids = operation_ids(&store);
    let preview = store.state_before(&ids[5], &protocol::HistoryInputDrafts::new()).expect("preview");
    drive_retirement_terminal(admit_demo_alias_retirement(&store,preview));
    let retained: Vec<usize> = store.prefix_ring.iter().map(|entry| entry.length).collect();
    assert!(!retained.is_empty() && retained.iter().all(|length| *length >= 1));
    let displaced = store.displaced_retirements.owners.len();
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(40)))] }).await.expect("supersede");
    assert!(store.displaced_retirements.owners.len() > displaced, "stale ring entries retire through displaced owners");
    let live = forward_prefix_digests::<DemoSnapshot, DemoMutation, _, _>(store.revision_accumulator.identity_digest, store.applied_edit_ids(), &store.envelope().vcs.edits, store.supersessions()).expect("digests");
    assert!(store.prefix_ring.iter().all(|entry| live.get(entry.length) == Some(&entry.digest)), "no stale entry survives");
    close_demo_artifact_store(&mut store);
    assert!(completed.load(std::sync::atomic::Ordering::SeqCst) > retained.len(), "every evicted and live root retired exactly");
}
//#endregion 🧪️SupersedeLaws

//#region 🧪️Metadata
/// 🧾️ Local and remote operations carry their semantic kind, descriptor label and tool transaction, on the wire and
/// through `.spr` persistence.
#[semio_framework_async_macros::async_test]
async fn operations_carry_semantic_kind_label_and_transaction_everywhere() {
    let transaction = protocol::TransactionRef::mint(&ActorId("author".into()), &HybridLogicalTimestamp::new(1, 1), "demo#drag");
    let mut store = demo_store("metadata", Some(0)).await;
    fixture_author(&mut store, ArtifactCommand::Apply { mutations: vec![set(3), add(1)], transaction: Some(transaction.clone()) }).await.expect("apply in a transaction");
    let expect = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| {
        let operations = store.mutation_ops().expect("operations");
        assert_eq!(operations.len(), 2);
        for (operation, (kind, label)) in operations.iter().zip([("demo/v1#set-n", "Set N"), ("demo/v1#add-n", "Add N")]) {
            let meta = operation.meta.expect("metadata");
            assert_eq!(meta.semantic_kind.as_ref().map(|kind| kind.0.as_str()), Some(kind));
            assert_eq!(meta.label.as_deref(), Some(label));
            assert_eq!(operation.transaction, Some(&transaction));
        }
    };
    expect(&store);
    let mut peer = demo_store("metadata", Some(0)).await;
    for event in store.event_log().expect("log") {
        assert_eq!(event.transaction.as_ref(), Some(&transaction), "the wire carries the transaction");
        peer.ingest_remote(event).await.expect("peer ingests");
    }
    expect(&peer);
    let files = print_document_pack(store.envelope()).await.expect("pack prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pack parses").into_envelope()).await;
    expect(&reloaded);
}

/// 🎞️ `Supersede`, `CreateAlternativeWithSupersede` and a transaction-carrying `Apply` round-trip the value, text and
/// binary command codecs.
#[semio_framework_async_macros::async_test]
async fn supersede_commands_round_trip_every_codec() {
    let transaction = protocol::TransactionRef { id: "tx-0123456789abcdef".into(), tool: "demo#drag".into() };
    let commands: Vec<ArtifactCommand<DemoMutation>> = vec![
        ArtifactCommand::Supersede { scope: Some("alternative-1".into()), inputs: vec![input(&MutationId("m-1".into()), Some(set(3))), input(&MutationId("m-2".into()), None)] },
        ArtifactCommand::Supersede { scope: None, inputs: vec![input(&MutationId("m-3".into()), None)] },
        ArtifactCommand::CreateAlternativeWithSupersede { name: "variant".into(), inputs: vec![input(&MutationId("m-1".into()), Some(add(2)))] },
        ArtifactCommand::Apply { mutations: vec![set(1)], transaction: Some(transaction.clone()) },
        ArtifactCommand::ApplyInLane { mutations: vec![add(1)], lane: HistoryLane::Interaction, transaction: Some(transaction) },
    ];
    for command in commands {
        let text = print_command(&command).await.expect("prints");
        assert_eq!(parse_command::<DemoSnapshot, DemoMutation>(&text).await.expect("parses"), command, "{text}");
        let bytes = command.encode_command().expect("encodes");
        assert_eq!(ArtifactCommand::<DemoMutation>::decode_command::<DemoSnapshot>(&bytes).expect("decodes"), command);
        assert_eq!(ArtifactCommand::<DemoMutation>::from_value(command.to_value()).expect("value decodes"), command);
    }
    assert!(parse_command::<DemoSnapshot, DemoMutation>("supersede\n  replace m-1\n").await.is_err(), "a replace line needs its replacement");
    assert!(parse_command::<DemoSnapshot, DemoMutation>("supersede\n").await.is_err(), "a supersede needs an input");
}
//#endregion 🧪️Metadata

//#region 🧪️FoldSiteLaws
/// 📨️ A supersession `remote` authored on another replica: one input replacing `target`, stamped past every local clock.
fn remote_supersession(document: &str, target: &MutationId, replacement: protocol::InputReplacement, logical: u64) -> crate::os_spr::MutationEnvelope {
    let transition = crate::os_spr::HistoryTransition::Supersede(protocol::TransitionSupersede { scope: None, inputs: vec![protocol::SupersededInput { target: target.clone(), replacement }] });
    crate::os_spr::history_transition_envelope(&transition, &ArtifactId(document.into()), &ActorId("remote-editor".into()), vec![target.clone()], HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical })
}

/// 🧵️ Every edit's inverse by edit id: what a load recomputes and every fold site must agree on.
fn inverses(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> BTreeMap<String, Vec<DemoMutation>> {
    store.envelope().vcs.edits.iter().map(|edit| (edit.id.clone(), edit.inverse.iter().cloned().collect())).collect()
}

/// 💧️ A member document whose history holds supersessions hydrates — the retained composition member open — to exactly
/// the store a `.pack`/`.spr` parse builds: the superseded state, the effective supersessions, the recomputed inverses,
/// the durable outcomes and the content revision. The `.ops` text mirror round-trips the same history (audit F-C1).
#[semio_framework_async_macros::async_test]
async fn a_member_document_with_supersessions_hydrates_to_its_superseded_state() {
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() };
    let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-superseded", DemoSnapshot { n: Some(0) }, None);
    envelope.dialect = Some(dialect.clone());
    let mut store = ArtifactStore::new(envelope).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3), add(4)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10))), input(&ids[2], None)] }).await.expect("a clean supersession");
    assert_eq!(store.snapshot_ref().n, Some(16));
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reference = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pair parses").into_envelope()).await;
    let history = crate::os_spr::decode_history(&files.spr, &crate::os_spr::DecodeOptions::default()).await.expect("history decodes");
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: "member-superseded".into(), dialect };
    let (operation, generation) = (semio_framework_job::OperationId(3), semio_framework_job::Generation(5));
    let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_pack(
        files.pack.clone(),
        history,
        expected,
        None,
        "demo/v1".into(),
        DemoSnapshot::member_store_owners(),
        operation,
        generation,
        u64::MAX,
        PersistedDocumentHydrationTarget::Store { generation: 0 },
        store.local_actor_id().clone(),
    );
    let mut sequence = 0;
    let mut member = None;
    for _ in 0..100_000 {
        let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4, 999), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
        match hydration.step(&mut cx) {
            PersistedDocumentHydrationStep::Pending(_) => {}
            PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Store(hydrated)) => {
                member = Some(*hydrated);
                break;
            }
            PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Envelope(_)) => panic!("a store hydration hands out a store"),
            PersistedDocumentHydrationStep::Rejected(diagnostic) => panic!("hydration refused a superseded history: {diagnostic:?}"),
        }
    }
    let member = ArtifactStore(member.expect("hydration converges"), Some(close_test_store::<DemoSnapshot, DemoMutation>));
    assert_eq!(member.snapshot_ref().n, Some(16), "the hydrated member folds the effective forwards");
    assert_eq!(member.supersessions(), reference.supersessions());
    assert_eq!(inverses(&member), inverses(&reference));
    assert_eq!(outcomes_by_position(&member, &member.mutation_outcomes().unwrap()), outcomes_by_position(&reference, &reference.mutation_outcomes().unwrap()));
    assert_eq!(member.0.content_revision(), reference.0.content_revision(), "hydration and parse build the same canonical identity");
    assert_eq!(member.0.content_revision(), store.0.content_revision(), "and the live store's");
    let text = print_document_text(store.envelope()).await.expect("text prints");
    assert!(text.ops.lines().any(|line| line.starts_with("supersede")), "the text mirror carries the supersede line");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops).await.expect("the text mirror parses").into_envelope()).await;
    assert_eq!(mirrored.snapshot_ref().n, Some(16));
    assert_eq!(mirrored.supersessions(), store.supersessions());
    assert_eq!(mirrored.0.content_revision(), store.0.content_revision());
}

#[semio_framework_async_macros::async_test]
async fn retained_hydration_preserves_the_original_opened_actor_across_stored_authors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️opened-hydration-actor/🔣️.json")).unwrap();
    let opened = fixture["openedActor"].as_str().unwrap();
    let author = fixture["historyAuthor"].as_str().unwrap();
    assert_ne!(opened, author);
    for config in [false, true] {
        let initial = DemoSnapshot { n: Some(fixture["initial"].as_i64().unwrap() as i32) };
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "opened-actor-hydration", initial.clone(), None);
        if config { envelope.history_shape = crate::os_spr::HistoryShape::Config; }
        else { envelope.dialect = Some(semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() }); }
        let mut source = super::super::ArtifactStore::new(envelope, ActorId(author.into())).await.unwrap();
        source.install_document_store_owners_exact(DemoSnapshot::member_store_owners());
        let mut source = ArtifactStore(source, Some(close_test_store::<DemoSnapshot, DemoMutation>));
        apply(&mut source, vec![set(fixture["stored"].as_i64().unwrap() as i32)]).await;
        let files = print_document_pack(source.envelope()).await.unwrap();
        let history = crate::os_spr::decode_history(&files.spr, &crate::os_spr::DecodeOptions::default()).await.unwrap();
        assert_eq!(history.edits.last().unwrap().actor.as_deref(), Some(author));
        let mut hydrated = None;
        if config {
            let mut hydration = RetainedConfigStoreHydration::<DemoSnapshot, DemoMutation>::from_snapshots(source.envelope().vcs.genesis.clone(), initial.clone(), initial, history, source.envelope().id.clone(), "demo/v1".into(), DemoSnapshot::member_store_owners(), 0, 65536, ActorId(opened.into()));
            for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
                match hydration.advance(1, 65536) {
                    ConfigStoreHydrationStep::Pending(_) => {}
                    ConfigStoreHydrationStep::Ready(store) => { hydrated = Some(*store); break; }
                    ConfigStoreHydrationStep::Rejected(diagnostic) => panic!("original config actor hydration refused: {diagnostic:?}"),
                }
            }
            assert!(hydration.terminal_is_empty());
        } else {
            let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: source.envelope().id.clone(), dialect: source.envelope().dialect.clone().unwrap() };
            let (operation, generation) = (semio_framework_job::OperationId(700), semio_framework_job::Generation(1));
            let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_pack(files.pack, history, expected, None, "demo/v1".into(), DemoSnapshot::member_store_owners(), operation, generation, u64::MAX, PersistedDocumentHydrationTarget::Store { generation: 0 }, ActorId(opened.into()));
            let mut sequence = 0;
            for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
                let mut cx = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4, u64::MAX), semio_framework_job::root_cancel_token(), || Some(1), &mut sequence);
                match hydration.step(&mut cx) {
                    PersistedDocumentHydrationStep::Pending(_) => {}
                    PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Store(store)) => { hydrated = Some(*store); break; }
                    PersistedDocumentHydrationStep::Rejected(diagnostic) => panic!("original member actor hydration refused: {diagnostic:?}"),
                    _ => panic!("original member actor hydration returned another output"),
                }
            }
            assert!(hydration.terminal_is_empty());
        }
        let hydrated = ArtifactStore(hydrated.expect("original actor hydration converges"), Some(close_test_store::<DemoSnapshot, DemoMutation>));
        assert_eq!(hydrated.snapshot_ref().n, source.snapshot_ref().n);
        assert_eq!(serde_json::to_value(hydrated.local_actor_id()).unwrap(), fixture["openedActor"]);
        assert_eq!(semio_framework_pack_json::to_json_string(hydrated.local_actor_id()), serde_json::to_string(&fixture["openedActor"]).unwrap());
        assert_eq!(hydrated.envelope().vcs.edits.last().unwrap().actor.as_deref(), Some(author));
    }
    println!("[DEBUG] Original persisted/config hydrators preserve opened actor={opened} independently of stored author={author}");
}

#[semio_framework_async_macros::async_test]
async fn cancelled_hydration_retires_the_same_original_opened_actor_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️opened-hydration-actor/🔣️.json")).unwrap();
    let source = demo_store("cancelled-opened-actor", Some(0)).await;
    let files = print_document_pack(source.envelope()).await.unwrap();
    let history = crate::os_spr::decode_history(&files.spr, &crate::os_spr::DecodeOptions::default()).await.unwrap();
    let expected = semio_framework_artifact_reference::ArtifactRef { artifact_id: source.envelope().id.clone(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
    let mut receipts = Vec::new();
    for actor in fixture["retiredActors"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()) {
        let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_pack(files.pack.clone(), history.clone(), expected.clone(), None, "demo/v1".into(), DemoSnapshot::member_store_owners(), semio_framework_job::OperationId(701), semio_framework_job::Generation(1), u64::MAX, PersistedDocumentHydrationTarget::Store { generation: 0 }, ActorId(actor.into()));
        assert_eq!(hydration.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        let mut charged = 0;
        for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
            match hydration.close_step(fixture["grant"]["items"].as_u64().unwrap() as usize, fixture["grant"]["bytes"].as_u64().unwrap() as usize).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1 && released_bytes <= 3); charged += released_bytes; }
                SnapshotRetirementStep::Complete => break,
                SnapshotRetirementStep::Blocked => panic!("opened actor owner retirement blocked"),
            }
        }
        assert!(hydration.terminal_is_empty());
        receipts.push(charged);
    }
    assert_eq!(receipts[1] - receipts[0], fixture["retiredActorByteDifference"].as_u64().unwrap() as usize);
    let oracle: Vec<String> = serde_json::from_value(fixture["retiredActors"].clone()).unwrap();
    assert_eq!(oracle[1].len() - oracle[0].len(), receipts[1] - receipts[0]);
    println!("[DEBUG] Original cancelled hydration actor byte receipts={receipts:?}, exact actor-only difference={}", receipts[1] - receipts[0]);
}

/// 🧯️ A replacement input breaking the supersede law — garbage bytes and a foreign schema from a remote replica — folds
/// its target as a no-op with one `Fatal` `mutation.invariant` outcome, identically at every fold site: ingest, the
/// durable outcomes, `state_before`, both persisted forms and `materialize_document_snapshot`. Hub check-in refuses it as
/// a blocking report — never a decode failure — and takes the ledger once a later supersession repairs it (audit F-M2).
#[semio_framework_async_macros::async_test]
async fn an_input_breaking_the_supersede_law_folds_as_a_fatal_no_op_at_every_site() {
    let mut store = demo_store("garbage", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3)]).await;
    let ids = operation_ids(&store);
    let baseline = print_document_pack(store.envelope()).await.expect("baseline pair prints");
    let garbage = remote_supersession("garbage", &ids[1], protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: vec![0xff, 0x13, 0x37] }, 0);
    let foreign = remote_supersession("garbage", &ids[2], protocol::InputReplacement::Input { schema: "other/v1".into(), payload: add(30).encode_op().unwrap() }, 1);
    for envelope in [garbage.clone(), foreign] {
        let report = store.ingest_remote(envelope).await.expect("a replica never refuses a supersession it cannot decode");
        assert!(report.accepted && report.conflict.is_none());
    }
    assert_eq!(store.snapshot_ref().n, Some(1), "both superseded operations fold as no-ops");
    let outcomes = outcomes_by_mutation(&store.mutation_outcomes().expect("durable outcomes"));
    for target in [&ids[1], &ids[2]] {
        assert_eq!(outcomes[target], (Some(semio_framework_diagnostic::Severity::Fatal), vec!["mutation.invariant".to_string()], true, false));
    }
    assert_eq!(store.state_before(&ids[2], &protocol::HistoryInputDrafts::new()).expect("state before").n, Some(1));
    assert_eq!(materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("materialized").n, Some(1));
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("the pair loads").into_envelope()).await;
    assert_eq!(reloaded.snapshot_ref().n, Some(1));
    assert_eq!(outcomes_by_mutation(&reloaded.mutation_outcomes().unwrap()), outcomes);
    let text = print_document_text(store.envelope()).await.expect("text prints");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops).await.expect("the text mirror loads").into_envelope()).await;
    assert_eq!(mirrored.snapshot_ref().n, Some(1));
    let refused =
        replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&baseline.pack, &baseline.spr, &crate::os_spr::encode_envelopes(std::slice::from_ref(&garbage)), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.invariant")), "{refused:?}");
    let repair = remote_supersession("garbage", &ids[1], draft(Some(add(5))), 2);
    let repaired = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&baseline.pack, &baseline.spr, &crate::os_spr::encode_envelopes(&[garbage, repair]), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>)
        .await
        .expect("a repaired ledger checks in");
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&repaired.pack, &repaired.spr).await.expect("checked-in pair parses");
    assert_eq!(parsed.snapshot.n, Some(9));
    test_support::retire_parsed_document(parsed);
}

/// ✂️ A replay never yields an unstorable message ledger: an edit whose outcomes exceed one entry's byte authority keeps
/// its worst messages and summarizes the rest deterministically — at local apply, at a local supersession's install, at
/// a remote supersession's ingest and at load — so no supersession is refused or wedged for size, and an alternative
/// branched over such an edit is authored exactly once (audit F-M3).
#[semio_framework_async_macros::async_test]
async fn oversize_replay_messages_are_bounded_deterministically_everywhere() {
    const OPERATIONS: usize = 200;
    let bytes = |entry: &[crate::os_spr::MutationMessage], edit_id: &str| entry.iter().fold(edit_id.len(), |total, message| total + message.code.0.len() + message.message.len() + message.target.iter().map(String::len).sum::<usize>());
    let dropped = |entry: &[crate::os_spr::MutationMessage]| -> usize {
        let summary = entry.last().expect("a bounded entry ends with its summary");
        assert_eq!((summary.code.0.as_str(), summary.level), ("mutation.cascade", semio_framework_diagnostic::Severity::Info));
        summary.message.strip_suffix(" more messages").expect("the summary counts what it drops").parse().expect("a count")
    };
    let mut store = demo_store("bounded", None).await;
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, (0..OPERATIONS).map(|_| add(1)).collect()).await;
    let ids = operation_ids(&store);
    let bulk = store.applied_edit_ids()[1].clone();
    let applied = store.messages_for_edit(&bulk).to_vec();
    assert!(bytes(&applied, &bulk) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES);
    assert_eq!(applied.len() - 1 + dropped(&applied), OPERATIONS, "kept and summarized messages account for every operation");
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(DemoMutation::AssignN(AssignN { n: Some(7) })))] }).await.expect("an oversize but clean replay installs");
    assert_eq!(store.snapshot_ref().n, Some(207));
    assert_eq!(store.messages_for_edit(&bulk), applied.as_slice(), "the replayed entry bounds exactly like the applied one");
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "bounded-variant".into(), inputs: vec![input(&ids[0], Some(DemoMutation::AssignN(AssignN { n: Some(9) })))] }).await.expect("an oversize replay branches");
    assert_eq!(store.envelope().vcs.alternatives.iter().filter(|alternative| alternative.name == "bounded-variant").count(), 1);
    assert_eq!(store.snapshot_ref().n, Some(209));
    let withdraw = remote_supersession("bounded", &ids[0], protocol::InputReplacement::Withdrawn, 0);
    let report = store.ingest_remote(withdraw).await.expect("a remote supersession is never refused for size");
    assert!(report.accepted && report.conflict.is_none());
    assert_eq!(store.snapshot_ref().n, None);
    let failed = store.messages_for_edit(&bulk).to_vec();
    assert!(bytes(&failed, &bulk) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES);
    assert!(failed[..failed.len() - 1].iter().all(|message| message.code.0 == "mutation.target-missing" && message.level == semio_framework_diagnostic::Severity::Error), "the worst messages are the ones kept");
    let kept = failed.len() - 1;
    assert_eq!(kept + dropped(&failed), OPERATIONS);
    assert_eq!(failed.last().unwrap().op_index, Some(kept as u32), "the summary sits at the first dropped operation");
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("a bounded ledger loads").into_envelope()).await;
    assert_eq!(reloaded.messages_for_edit(&bulk), failed.as_slice(), "load bounds the recomputed messages identically");
    assert_eq!(outcomes_by_mutation(&reloaded.mutation_outcomes().unwrap()), outcomes_by_mutation(&store.mutation_outcomes().unwrap()));
}

/// ⚖️ Hub check-in judges the history a ledger folds to, never the states in between: a supersession that makes a
/// downstream operation fail is refused alone, and checks in once a later supersession heals it (audit F-M4).
#[semio_framework_async_macros::async_test]
async fn check_in_judges_the_folded_ledger_not_its_intermediate_states() {
    let mut store = demo_store("intermediate", None).await;
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let breaking = remote_supersession("intermediate", &ids[0], protocol::InputReplacement::Withdrawn, 0);
    let healing = remote_supersession("intermediate", &ids[0], draft(Some(DemoMutation::AssignN(AssignN { n: Some(7) }))), 1);
    let refused = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &crate::os_spr::encode_envelopes(std::slice::from_ref(&breaking)), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "{refused:?}");
    let healed = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &crate::os_spr::encode_envelopes(&[breaking, healing]), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>)
        .await
        .expect("a ledger whose final history is clean checks in");
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&healed.pack, &healed.spr).await.expect("checked-in pair parses");
    assert_eq!(parsed.snapshot.n, Some(8));
    test_support::retire_parsed_document(parsed);
}

/// 🔁️ A replica with an interior revert — an undo of an edit a later foreign edit follows — at the given early-exit
/// setting: `restore 5`, `delete`, then the peer's `add 1` (kept with its error under a laissez-faire policy), then undo.
async fn interior_revert_store(early_exit: bool) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    let mut store = demo_store("interior", None).await;
    store.set_merge_policy(crate::os_spr::MergePolicy::LaissezFaire);
    if early_exit {
        store.enable_convergence_early_exit();
    }
    apply(&mut store, vec![DemoMutation::AssignN(AssignN { n: Some(5) })]).await;
    apply(&mut store, vec![DemoMutation::DeleteN(DeleteN {})]).await;
    let peer = crate::os_spr::MutationEnvelope {
        mutation_id: MutationId("peer-add".into()),
        document_id: ArtifactId("interior".into()),
        actor: ActorId("peer".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: SchemaId("demo/v1".into()), payload: add(1).encode_op().unwrap() },
        inverse: crate::os_spr::InverseMutation { schema: SchemaId("demo/v1".into()), payload: Vec::new() },
        timestamp: HybridLogicalTimestamp { actor: 4, physical_ms: u64::MAX / 2, logical: 0 },
        transaction: None,
        verb: None,
        line: None,
    };
    let report = store.ingest_remote(peer).await.expect("the peer edit ingests");
    assert!(report.accepted);
    assert_eq!(store.snapshot_ref().n, None);
    fixture_author(&mut store, ArtifactCommand::Undo).await.expect("the interior delete reverts");
    store
}

/// 🏁️ An interior re-projection keeps the durable history replay-consistent: after an interior revert the downstream
/// edit's durable outcome is what a replay says, so a supersession replay converging early reports exactly what the full
/// replay reports (audit F-M6).
#[semio_framework_async_macros::async_test]
async fn an_interior_revert_keeps_the_early_exit_equal_to_the_full_replay() {
    let mut early = interior_revert_store(true).await;
    let mut full = interior_revert_store(false).await;
    for store in [&early, &full] {
        assert_eq!(store.snapshot_ref().n, Some(6), "the peer edit applies once the delete is reverted");
        let ids = operation_ids(store);
        assert_eq!(outcomes_by_mutation(&store.mutation_outcomes().unwrap())[&ids[1]].1, vec!["mutation.cascade".to_string()], "the reverted interior refreshed the downstream outcome");
    }
    let restate = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| drafts(&[(&operation_ids(store)[0], Some(DemoMutation::AssignN(AssignN { n: Some(5) })))]);
    let converged = finish(&early, early.begin_report_replay(&restate(&early), None).expect("early replay"));
    assert!(converged.converged_at().is_some(), "an equal prefix state converges");
    let replayed = finish(&full, full.begin_report_replay(&restate(&full), None).expect("full replay"));
    assert!(replayed.converged_at().is_none());
    assert_eq!(outcomes_by_position(&early, &early.replay_report(&converged).unwrap().outcomes), outcomes_by_position(&full, &full.replay_report(&replayed).unwrap().outcomes));
    drop((converged, replayed));
    fixture_author(&mut early, ArtifactCommand::Redo).await.expect("redo re-applies the interior delete");
    fixture_author(&mut full, ArtifactCommand::Redo).await.expect("redo re-applies the interior delete");
    for store in [&early, &full] {
        assert_eq!(store.snapshot_ref().n, None);
        let ids = operation_ids(store);
        assert_eq!(outcomes_by_mutation(&store.mutation_outcomes().unwrap())[&ids[2]].1, vec!["mutation.target-missing".to_string()], "the interior redo refreshed it again");
    }
}

/// 🔏️ Two replicas holding the same events report the same content revision whatever order a supersession of a redo
/// edit and the reverts that moved it there arrived in (audit F-m1).
#[semio_framework_async_macros::async_test]
async fn a_supersession_of_a_redo_edit_keeps_revisions_order_independent() {
    let mut author = demo_store("redo-revision", Some(0)).await;
    apply(&mut author, vec![set(1)]).await;
    apply(&mut author, vec![add(2)]).await;
    apply(&mut author, vec![add(3)]).await;
    let ids = operation_ids(&author);
    let log = author.event_log().expect("edit log");
    fixture_author(&mut author, ArtifactCommand::Undo).await.expect("undo add 3");
    fixture_author(&mut author, ArtifactCommand::Undo).await.expect("undo add 2");
    let reverts: Vec<_> = author.event_log().expect("log").into_iter().filter(|event| !log.iter().any(|known| known.mutation_id == event.mutation_id)).collect();
    assert_eq!(reverts.len(), 2);
    let supersede = remote_supersession("redo-revision", &ids[2], draft(Some(add(30))), 0);
    let mut late = demo_store("redo-revision", Some(0)).await;
    for event in log.iter().cloned().chain(reverts.iter().cloned()).chain([supersede.clone()]) {
        late.ingest_remote(event).await.expect("reverts first, then the supersession");
    }
    let mut early = demo_store("redo-revision", Some(0)).await;
    for event in log.iter().cloned().chain([supersede]).chain(reverts.iter().cloned()) {
        early.ingest_remote(event).await.expect("the supersession first, then the reverts");
    }
    assert_eq!(late.redo_edit_ids(), early.redo_edit_ids());
    assert_eq!(late.redo_edit_ids().len(), 2);
    assert_eq!(late.0.content_revision(), early.0.content_revision());
    let files = print_document_pack(late.envelope()).await.expect("pair prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pair parses").into_envelope()).await;
    assert_eq!(reloaded.0.content_revision(), late.0.content_revision(), "and a fresh load agrees");
}
//#endregion 🧪️FoldSiteLaws

//#region 🧪️TrunkLaws
/// 🚉️ The alternatives a store lists as `(id, name, chain length)` and the alternative it is on.
fn alternative_view(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> (Vec<(String, String, usize)>, String) {
    (store.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.len())).collect(), store.active_line_id())
}

/// 🌳️ A new alternative preserves the trunk it branched from: after finalizing a history edit as a new alternative the
/// trunk is listed first (empty name, localized by the UI), switching to it restores the original positions with only
/// unscoped and trunk-scoped supersessions, switching back restores the edited ones, the log never names the trunk,
/// replicas holding the same events agree, and `.spr` and `.ops` reloads keep all of it.
#[semio_framework_async_macros::async_test]
async fn a_new_alternative_preserves_the_trunk_it_branched_from() {
    let mut store = demo_store("trunk", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    let ids = operation_ids(&store);
    let trunk = store.trunk_alternative_id();
    assert_eq!(store.active_line_id(), trunk, "every document starts on its trunk");
    assert!(store.envelope().vcs.alternatives.is_empty(), "a trunk without a checkpoint is not listed yet");
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "edited".into(), inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("finalize as a new alternative");
    assert_eq!(store.snapshot_ref().n, Some(12));
    let (alternatives, current) = alternative_view(&store);
    assert_eq!(alternatives.iter().map(|(id, name, _)| (id.as_str(), name.as_str())).collect::<Vec<_>>()[0], (trunk.as_str(), ""), "the trunk is listed first");
    let edited = alternatives.iter().find(|(_, name, _)| name == "edited").map(|(id, _, _)| id.clone()).expect("the new alternative");
    assert_eq!(current, edited);
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk.clone() }).await.expect("switch back to the trunk");
    assert_eq!(store.snapshot_ref().n, Some(3), "the trunk keeps the original positions");
    assert_eq!((store.active_line_id(), store.envelope().active_alternative_id.clone()), (trunk.clone(), None));
    assert!(store.supersessions().is_empty(), "the edited alternative's scoped supersession stays on it");
    assert!(
        store.envelope().transitions.iter().all(|envelope| {
            match crate::os_spr::history_transition_from_envelope(envelope).expect("transition").expect("history") {
                crate::os_spr::HistoryTransition::Checkout { .. } => false,
                crate::os_spr::HistoryTransition::Branch { alternative_id, .. } => alternative_id != trunk,
                crate::os_spr::HistoryTransition::Commit(checkpoint) => checkpoint.line_id.as_deref() != Some(trunk.as_str()),
                _ => true,
            }
        }),
        "a head switch is local, and the log never names the trunk"
    );
    fixture_author(&mut store, ArtifactCommand::Supersede { scope: Some(trunk.clone()), inputs: vec![input(&ids[1], Some(add(5)))] }).await.expect("a supersession scoped to the trunk");
    assert_eq!(store.snapshot_ref().n, Some(6));
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: edited.clone() }).await.expect("switch to the edited alternative");
    assert_eq!(store.snapshot_ref().n, Some(12), "the edited alternative keeps its positions and ignores the trunk's edit");
    fixture_author(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk.clone() }).await.expect("and back again");
    assert_eq!(store.snapshot_ref().n, Some(6));
    let expected = alternative_view(&store);
    let log = store.event_log().expect("log");
    let mut peer = demo_store("trunk", Some(0)).await;
    for event in log.iter().rev().cloned() {
        peer.ingest_remote(event).await.expect("the peer ingests every event");
    }
    let mut other = demo_store("trunk", Some(0)).await;
    for event in log {
        other.ingest_remote(event).await.expect("another peer ingests them in authoring order");
    }
    for replica in [&peer, &other] {
        assert_eq!(alternative_view(replica), expected, "replicas list the same alternatives and stand on the same one");
        assert_eq!(replica.snapshot_ref().n, Some(6));
        assert_eq!(replica.supersessions(), store.supersessions());
    }
    assert_eq!(peer.0.content_revision(), other.0.content_revision(), "arrival order never changes a replica's identity");
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let mut reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pair parses").into_envelope()).await;
    assert_eq!(alternative_view(&reloaded), expected, ".spr keeps the trunk");
    assert_eq!(reloaded.snapshot_ref().n, Some(6));
    let text = print_document_text(store.envelope()).await.expect("text prints");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops).await.expect("text parses").into_envelope()).await;
    assert_eq!(alternative_view(&mirrored), expected, ".ops keeps the trunk");
    assert_eq!(mirrored.snapshot_ref().n, Some(6));
    fixture_author(&mut reloaded, ArtifactCommand::SwitchAlternative { alternative_id: edited }).await.expect("a reloaded store switches to the edited alternative");
    assert_eq!(reloaded.snapshot_ref().n, Some(12));
}

/// 👁️ A replica standing on another alternative at an explicit checkpoint keeps that head through its `.spr` pack
/// (`REC_VIEWER`, audit W1G-9): the persisted log names the head, the reloaded store stands on the same alternative and
/// checkpoint with the same projection and supersessions, and the `.ops` text — the shared log, which carries no viewer head
/// — hydrates to the trunk tip.
#[semio_framework_async_macros::async_test]
async fn a_viewer_head_round_trips_through_the_spr_pack() {
    let mut store = demo_store("viewer", Some(0)).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    let ids = operation_ids(&store);
    fixture_author(&mut store, ArtifactCommand::CreateAlternativeWithSupersede { name: "edited".into(), inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("finalize as a new alternative");
    apply(&mut store, vec![add(3)]).await;
    fixture_author(&mut store, ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a checkpoint on the edited alternative");
    let edited = store.envelope().active_alternative_id.clone().expect("the edited alternative is active");
    let checkpoint = store.envelope().vcs.alternatives.iter().find(|alternative| alternative.id == edited).and_then(|alternative| alternative.checkpoint_ids.last().cloned()).expect("the edited alternative's newest checkpoint");
    apply(&mut store, vec![add(4)]).await;
    assert_eq!(store.snapshot_ref().n, Some(19));
    fixture_author(&mut store, ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint.clone() }).await.expect("look at the committed checkpoint");
    let head = (store.envelope().active_alternative_id.clone(), store.envelope().viewer_checkpoint_id.clone());
    assert_eq!(head, (Some(edited.clone()), Some(checkpoint.clone())), "the replica stands on the edited alternative at its checkpoint");
    assert_eq!(store.snapshot_ref().n, Some(15), "an explicit checkpoint hides the uncommitted edit after it");
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let log = crate::os_spr::decode_history(&files.spr, &crate::os_spr::DecodeOptions::default()).await.expect("spr decodes");
    assert_eq!((log.viewer_line.clone(), log.viewer_checkpoint.clone()), head, "the persisted log names the viewer head");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pair parses").into_envelope()).await;
    assert_eq!((reloaded.envelope().active_alternative_id.clone(), reloaded.envelope().viewer_checkpoint_id.clone()), head, ".spr keeps the viewer head");
    assert_eq!((reloaded.snapshot_ref().n, reloaded.supersessions()), (Some(15), store.supersessions()), ".spr reloads the viewer's projection");
    test_support::assert_document_pack_round_trip(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
}
//#endregion 🧪️TrunkLaws

//#region 🧪️HistoryShapeLaws
/// 🗂️ A config store's history holds undo and redo only: every command authoring another transition is refused at
/// dispatch with the typed `HistoryShape` refusal before anything runs — the store untouched, every replacement
/// retired — and so is a remote transition of another kind; undo and redo keep working. The store refuses exactly the
/// kinds the language-agnostic shape table excludes (`🧫️history-transition`, twinned in TypeScript and Python), and a
/// document store admits all of them.
#[semio_framework_async_macros::async_test]
async fn a_config_store_holds_undo_and_redo_only() {
    use crate::os_spr::HistoryTransitionKind::{Checkout, Commit, Supersede};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/📡️replication/🔗️causal/🧫️fixtures/🧫️history-transition/🔣️.json")).expect("history transition fixture parses");
    let admitted = |shape: &str| -> Vec<String> {
        let row = fixture["shapes"].as_array().expect("shape rows").iter().find(|row| row["shape"] == shape).expect("a shape row");
        row["admits"].as_array().expect("admits").iter().map(|kind| kind.as_str().expect("kind").to_string()).collect()
    };
    let mut config = ArtifactStore::new(create_config_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "config", DemoSnapshot { n: Some(0) }, None).await).await;
    apply(&mut config, vec![set(1)]).await;
    apply(&mut config, vec![add(2)]).await;
    let ids = operation_ids(&config);
    let trunk = config.trunk_alternative_id();
    let untouched = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| (store.generation(), store.0.content_revision(), store.envelope().transitions.len(), store.snapshot_ref().n, store.supersessions().len());
    let before = untouched(&config);
    let refused = [
        (ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }, Commit),
        (ArtifactCommand::CreateAlternative { name: "b".into() }, Commit),
        (ArtifactCommand::SwitchAlternative { alternative_id: trunk.clone() }, Checkout),
        (ArtifactCommand::CheckoutCheckpoint { checkpoint_id: "c".into() }, Checkout),
        (ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(9)))] }, Supersede),
        (ArtifactCommand::Supersede { scope: Some(trunk), inputs: vec![input(&ids[1], None)] }, Supersede),
        (ArtifactCommand::CreateAlternativeWithSupersede { name: "b".into(), inputs: vec![input(&ids[0], Some(set(9)))] }, Commit),
    ];
    for (command, kind) in refused {
        let kinds = command.history_transition_kinds();
        assert!(kinds.iter().any(|kind| !admitted("config").contains(&kind.name().to_string())), "{kinds:?}: the shape table excludes a kind");
        assert!(kinds.iter().all(|kind| admitted("document").contains(&kind.name().to_string())), "{kinds:?}: a document holds every kind");
        assert_eq!(fixture_author(&mut config, command).await.err(), Some(VcsError::HistoryShape { shape: crate::os_spr::HistoryShape::Config, kind }));
        assert_eq!(untouched(&config), before, "a refused command leaves the store untouched");
    }
    let remote = remote_supersession("config", &ids[0], draft(Some(set(9))), 0);
    assert_eq!(config.ingest_remote(remote).await.err(), Some(VcsError::HistoryShape { shape: crate::os_spr::HistoryShape::Config, kind: Supersede }));
    assert_eq!(untouched(&config), before, "a refused remote transition is never recorded");
    for command in [ArtifactCommand::<DemoMutation>::Undo, ArtifactCommand::Redo] {
        assert!(command.history_transition_kinds().iter().all(|kind| admitted("config").contains(&kind.name().to_string())));
    }
    fixture_author(&mut config, ArtifactCommand::Undo).await.expect("a config store undoes");
    assert_eq!(config.snapshot_ref().n, Some(1));
    fixture_author(&mut config, ArtifactCommand::Redo).await.expect("and redoes");
    assert_eq!(config.snapshot_ref().n, Some(3));
    let mut document = demo_store("config", Some(0)).await;
    apply(&mut document, vec![set(1)]).await;
    let target = operation_ids(&document)[0].clone();
    fixture_author(&mut document, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&target, Some(set(9)))] }).await.expect("a document store supersedes");
    assert_eq!(document.snapshot_ref().n, Some(9));
}
//#endregion 🧪️HistoryShapeLaws

//#region 🧪️CommandDecodeLaws
std::thread_local! {
    static WITNESS_RETIRED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static WITNESS_DROPPED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// 🧿️ A demo operation with a fail-closed owner's discipline: its cold disposal is counted, and a bare drop — what a
/// partial decode used to do — is counted too, so a law can prove none happened.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct WitnessOp(DemoMutation, bool);

impl WitnessOp {
    pub(super) fn live(operation: DemoMutation) -> Self {
        Self(operation, true)
    }

    /// 📊️ `(retired, dropped)` so far on this thread.
    pub(super) fn tally() -> (usize, usize) {
        (WITNESS_RETIRED.with(std::cell::Cell::get), WITNESS_DROPPED.with(std::cell::Cell::get))
    }
}

impl Drop for WitnessOp {
    fn drop(&mut self) {
        if self.1 {
            WITNESS_DROPPED.with(|dropped| dropped.set(dropped.get() + 1));
        }
    }
}

impl ToValue for WitnessOp {
    fn to_value(&self) -> DslValue {
        self.0.to_value()
    }
}

impl FromValue for WitnessOp {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        DemoMutation::from_value(value).map(Self::live)
    }
}

impl OpBinary for WitnessOp {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        self.0.encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        DemoMutation::decode_op(bytes).map(Self::live)
    }
}

impl OpText for WitnessOp {
    fn print_op(&self) -> String {
        self.0.print_op()
    }

    fn parse_op(line: &str) -> Result<Self, TextError> {
        DemoMutation::parse_op(line).map(Self::live)
    }
}

impl Mutation<DemoSnapshot> for WitnessOp {
    type Diff = <DemoMutation as Mutation<DemoSnapshot>>::Diff;
    const DESCRIPTORS: &'static [crate::os_spr::MutationLeafDescriptor] = <DemoMutation as Mutation<DemoSnapshot>>::DESCRIPTORS;

    fn descriptor(&self) -> &'static crate::os_spr::MutationLeafDescriptor {
        self.0.descriptor()
    }

    fn diff(&self, base: &DemoSnapshot) -> crate::os_spr::MutationOutcome<Self::Diff> {
        self.0.diff(base)
    }

    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok({
        self.0.inverse(base)?.into_iter().map(Self::live).collect()
    
    })
}

    fn retire_cold(mut self) {
        self.1 = false;
        WITNESS_RETIRED.with(|retired| retired.set(retired.get() + 1));
    }
}

/// 🧷️ Command decoding is all-or-nothing, binary and text alike: a command whose operation list, supersede inputs or
/// nested command fails midway — or that carries bytes past its end — is refused with the typed `CommandDecodeError`
/// naming the failing entry, and every operation decoded before the failure retires through the technology's cold
/// disposal; none is ever dropped bare (audit F-m11b). A hostile operation count never allocates beyond the input.
#[semio_framework_async_macros::async_test]
async fn a_refused_command_decode_retires_every_operation_it_decoded() {
    let binary = |command: ArtifactCommand<DemoMutation>| command.encode_command().expect("encodes");
    let garbled = |command: ArtifactCommand<DemoMutation>, last: DemoMutation| {
        let mut bytes = binary(command);
        let at = bytes.len() - last.encode_op().expect("encodes").len();
        bytes[at] = 0xff;
        bytes
    };
    let decode = |bytes: &[u8]| ArtifactCommand::<WitnessOp>::decode_command::<DemoSnapshot>(bytes);
    let apply = |mutations: Vec<DemoMutation>| ArtifactCommand::Apply { mutations, transaction: None };
    let start = WitnessOp::tally();
    let cases: Vec<(Vec<u8>, usize, Option<usize>)> = vec![
        (garbled(apply(vec![set(1), add(2), set(3)]), set(3)), 2, Some(2)),
        (garbled(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&MutationId("m-1".into()), Some(set(9))), input(&MutationId("m-2".into()), None), input(&MutationId("m-3".into()), Some(add(4)))] }, add(4)), 1, Some(2)),
        (garbled(ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: Some(Box::new(apply(vec![set(5), add(6)]))) }, add(6)), 1, Some(1)),
        ([binary(apply(vec![set(7), add(8)])), vec![0]].concat(), 2, None),
    ];
    let mut retired = 0;
    for (bytes, decoded, failing) in cases {
        let refused = decode(&bytes).expect_err("a broken command is refused");
        retired += decoded;
        match failing {
            Some(index) => assert!(matches!(refused, CommandDecodeError::Operation { index: at, .. } if at == index), "{refused:?}"),
            None => assert!(matches!(refused, CommandDecodeError::Layout(_)), "{refused:?}"),
        }
        assert_eq!(WitnessOp::tally(), (start.0 + retired, start.1), "every decoded operation retired, none dropped");
    }
    let mut hostile = vec![COMMAND_BINARY_FORMAT, 0, 0];
    crate::os_pack::write_varint_u64(&mut hostile, u64::MAX >> 1);
    assert!(matches!(decode(&hostile), Err(CommandDecodeError::Operation { index: 0, .. })));
    let one = print_command(&apply(vec![set(1)])).await.expect("prints");
    let op_line = one.lines().find(|line| line.starts_with("  ")).expect("an op line").to_string();
    for (command, decoded) in [(format!("{}\n{op_line}\n  not an operation\n", one.lines().next().unwrap()), 1), (format!("supersede\n  replace m-1\n  {op_line}\n  replace m-2\n    not an operation\n"), 1)] {
        assert!(parse_command::<DemoSnapshot, WitnessOp>(&command).await.is_err(), "{command}");
        retired += decoded;
        assert_eq!(WitnessOp::tally(), (start.0 + retired, start.1), "{command}");
    }
    let accepted = decode(&binary(apply(vec![set(1), add(2)]))).expect("a whole command decodes");
    assert_eq!(WitnessOp::tally(), (start.0 + retired, start.1), "an accepted command's operations stay live");
    retire_command::<DemoSnapshot, WitnessOp>(accepted);
    assert_eq!(WitnessOp::tally(), (start.0 + retired + 2, start.1));
}
//#endregion 🧪️CommandDecodeLaws

//#region 🧪️RetractionLaws
/// 🔙️ A hub-refused local transition retracts (audit F-m8): the author's refused supersession, its refused
/// finalize-as-alternative (commit, branch, scoped supersession) leave the author's log through `BackboneMessage::Retract`
/// naming the supersessions and the commit — the branch depends on the commit and retracts with it — and the author
/// converges with the hub,
/// which never took them — the same event log, snapshot, supersessions, alternatives and outcomes, and the author's own
/// content revision from before its refused steps. Retracting again, retracting an unknown name or a transition another
/// replica authored changes nothing, and a persisted `.spr` retracts exactly the same transitions.
#[semio_framework_async_macros::async_test]
async fn a_refused_local_transition_retracts_and_the_author_converges_with_the_hub() {
    let (channel, remote) = ChannelBackbone::pair("retract").await;
    let mut author = demo_store("retract", Some(0)).await;
    author.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    apply(&mut author, vec![set(1)]).await;
    apply(&mut author, vec![add(2)]).await;
    apply(&mut author, vec![add(3)]).await;
    let ids = operation_ids(&author);
    let mut hub = demo_store("retract", Some(0)).await;
    for event in author.event_log().expect("log") {
        hub.ingest_remote(event).await.expect("the hub takes the edits");
    }
    let foreign = remote_supersession("retract", &ids[2], draft(Some(add(30))), 0);
    for replica in [&mut author, &mut hub] {
        replica.ingest_remote(foreign.clone()).await.expect("a peer's accepted supersession reaches both");
    }
    let accepted = (author.0.content_revision(), author.snapshot_ref().n, author.envelope().transitions.len());
    let before_refusal = print_document_pack(author.envelope()).await.expect("pair prints");
    let known = author.envelope().transitions.len();
    fixture_author(&mut author, ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("a local supersession");
    fixture_author(&mut author, ArtifactCommand::CreateAlternativeWithSupersede { name: "edited".into(), inputs: vec![input(&ids[1], None)] }).await.expect("finalize as a new alternative");
    let trunk = author.trunk_alternative_id();
    fixture_author(&mut author, ArtifactCommand::SwitchAlternative { alternative_id: trunk }).await.expect("back to the trunk");
    let authored: Vec<String> = author.envelope().transitions[known..].iter().map(|transition| transition.mutation_id.0.clone()).collect();
    assert_eq!(authored.len(), 4, "supersede, commit, branch, scoped supersede; switching moves this viewer's head only");
    let refused = vec![authored[0].clone(), authored[1].clone(), authored[3].clone()];
    let refused_log = print_document_pack(author.envelope()).await.expect("pair prints");
    let _ = drain_channel_for_test(&remote).expect("drain outbound");
    remote.push(BackboneMessage::Retract { mutation_ids: refused.clone() }).await.expect("push retraction");
    author.tick().await.expect("tick");
    let log_ids = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| store.event_log().expect("log").into_iter().map(|event| event.mutation_id.0).collect::<Vec<_>>();
    assert_eq!(log_ids(&author), log_ids(&hub), "the branch retracts with the commit it depends on");
    assert_eq!((author.0.content_revision(), author.snapshot_ref().n, author.envelope().transitions.len()), accepted, "the author is back where the hub took it");
    assert_eq!(author.snapshot_ref().n, hub.snapshot_ref().n);
    assert_eq!(author.supersessions(), hub.supersessions());
    assert_eq!(alternative_view(&author), alternative_view(&hub));
    assert_eq!(outcomes_by_mutation(&author.mutation_outcomes().expect("outcomes")), outcomes_by_mutation(&hub.mutation_outcomes().expect("outcomes")));
    let generation = author.generation();
    remote.push(BackboneMessage::Retract { mutation_ids: [refused.clone(), vec!["unknown".into(), foreign.mutation_id.0.clone()]].concat() }).await.expect("push again");
    author.tick().await.expect("tick");
    assert_eq!((author.generation(), author.0.content_revision(), log_ids(&author)), (generation, accepted.0, log_ids(&hub)), "a known, unknown or foreign name retracts nothing");
    let (spr, retracted) = retract_history_transitions_from_spr(&refused_log.spr, &refused).await.expect("the persisted log retracts");
    assert_eq!(retracted, authored, "the persisted log retracts the same closure");
    let persisted = |bytes: Vec<u8>| async move { crate::os_spr::decode_history(&bytes, &crate::os_spr::DecodeOptions::default()).await.expect("log decodes").transitions.into_iter().map(|record| record.id).collect::<Vec<_>>() };
    assert_eq!(persisted(spr).await, persisted(before_refusal.spr).await);
}
//#endregion 🧪️RetractionLaws

/// ⌛️ Planning, prefix folding and replay each yield after one unit; cancelled and stale cursors never publish.
#[semio_framework_async_macros::async_test]
async fn bounded_history_read_cursors_obey_the_neutral_law() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️supersede-replay/🔣️.json")).unwrap();
    let law = &corpus["boundedHistoryRead"];
    let close_items = law["closeGrant"]["maximumItems"].as_u64().unwrap() as usize;
    let release_bytes = law["closeGrant"]["maximumReleaseBytes"].as_u64().unwrap() as usize;
    assert_eq!(close_items, 1);
    assert_eq!(release_bytes, 4096);
    let mut store = demo_store("bounded-history-read", Some(law["initial"]["n"].as_i64().unwrap() as i32)).await;
    for edit in law["edits"].as_array().unwrap() {
        apply(&mut store, edit.as_array().unwrap().iter().map(|op| DemoMutation::from_value(op.clone().into()).unwrap()).collect()).await;
    }
    let ids = store.mutation_ops().unwrap();
    let id_of = |position: &serde_json::Value| ids.iter().find(|op| op.position == position["edit"].as_u64().unwrap() as usize && op.op_index == position["op"].as_u64().unwrap() as u32).unwrap().mutation_id.clone();
    let target = id_of(&law["target"]);
    let accepted: protocol::HistoryInputDrafts = law["accepted"].as_array().unwrap().iter().map(|input| (id_of(input), draft(Some(DemoMutation::from_value(input["replacement"].clone().into()).unwrap())))).collect();
    for row in &ids { assert_eq!(store.mutation_position(&row.mutation_id).unwrap(), (row.position, row.op_index as usize)); }
    let head = store.snapshot_ref().n;
    hot_path_census::take();
    let mut preview = Some(store.begin_derived_history_preview(target.clone(), accepted.clone()).unwrap());
    assert_eq!(hot_path_census::take(), hot_path_census::HotPathCensus::default(), "starting does not scan/hash/fold history");
    let mut pending_steps = 0;
    loop {
        let mut work = 0;
        let grant=planning_grant(preview.as_ref().unwrap().planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).unwrap());
        let step = store.step_derived_history_preview(preview.as_mut().unwrap(), grant, &mut || { work += 1; true }).unwrap();
        assert!(work <= law["maximumWorkPerStep"].as_u64().unwrap() as usize);
        assert_eq!(hot_path_census::take().digested, 0, "trusted prefixes need no history digest scan");
        assert_eq!(store.snapshot_ref().n, head);
        if matches!(step, ReplayStep::Finished(_)) { break; }
        pending_steps += 1;
    }
    assert!(pending_steps > 2, "planning and same-edit prefix operations yield separately");
    let before = store.finish_derived_history_preview(&mut preview).unwrap();
    assert!(preview.as_ref().unwrap().state.is_none());
    let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:ArtifactStore::<DemoSnapshot,DemoMutation>::history_read_retirement_birth_bytes(),maximum_depth:1,..Default::default()};
    let (residue,receipt)=store.retire_derived_history_preview(&mut preview,grant).unwrap().unwrap();assert!(receipt.fits(grant));drive_retirement_terminal(residue);
    assert_eq!(before.snapshot().n, Some(law["expected"]["before"].as_i64().unwrap() as i32));
    let replacement = DemoMutation::from_value(law["replacement"].clone().into()).unwrap();
    let (next, _) = store.derive_history_snapshot(&before, &replacement).unwrap();
    assert_eq!(next.unwrap().snapshot().n, Some(law["expected"]["preview"].as_i64().unwrap() as i32));
    let mut changed = accepted.clone();
    changed.insert(target.clone(), draft(Some(replacement)));
    let mut replay = Some(store.begin_derived_report_replay(changed, Some(target.clone())).unwrap());
    let mut slices = 0;
    loop {
        let mut work = 0;
        let grant=planning_grant(replay.as_ref().unwrap().planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).unwrap());
        let step = store.step_derived_report_replay(replay.as_mut().unwrap(), grant, &mut || { work += 1; true }).unwrap();
        assert!(work <= 1);
        assert_eq!(store.snapshot_ref().n, head);
        slices += 1;
        if matches!(step, ReplayStep::Finished(_)) { break; }
    }
    assert!(slices > 5, "the planner and replay are both sliced");
    let (result, head) = store.finish_derived_report_replay(&mut replay).unwrap();
    let head = head.unwrap();
    assert_eq!(head.snapshot().n, Some(law["expected"]["head"].as_i64().unwrap() as i32));
    let mut result = Some(result);
    let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ArtifactStore::<DemoSnapshot,DemoMutation>::history_read_retirement_birth_bytes(),maximum_depth:1,..Default::default()};
    let(mut retirement,birth)=store.retire_finished_history_replay(&mut result,grant).unwrap().unwrap();assert!(birth.fits(grant));
    assert!(result.is_none());
    let mut turns = 0;
    let head_identity = Arc::as_ptr(head.snapshot_owner());
    let head_value = head.snapshot().n;
    let mut held_head = Some(head);
    let mut physical_page_seen = false;
    let mut physical_demands = std::collections::BTreeSet::new();
    let mut physical_releases = std::collections::BTreeSet::new();
    let mut blocked_on_head = false;
    loop {
        turns += 1;
        assert!(turns < 100_000);
        let physical_demand = retirement.next_release_byte_demand().unwrap();
        physical_demands.insert(physical_demand);
        if physical_demand == law["physicalInversePageBytes"].as_u64().unwrap() as usize { physical_page_seen = true; }
        let grant = native_retirement_grant(retirement.as_ref(), close_items, release_bytes);
        assert!(physical_demand <= grant.maximum_release_bytes, "explicit physical release grant admits the exact original owner extent");
        if let Some(head) = held_head.as_ref() { assert_eq!(Arc::as_ptr(head.snapshot_owner()), head_identity); assert_eq!(head.snapshot().n, head_value); }
        match retirement.close_step(grant).unwrap() {
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(retirement.terminal_is_empty()); break; }
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => { physical_releases.insert(progress.released_bytes); assert!(progress.fits(grant)); }
            semio_framework_value::retained_clone::RetainedCloneStep::Blocked => {
                let head = held_head.take().expect("only the exact captured review head may block retirement");
                drive_retirement_terminal(admit_demo_alias_retirement(&store,head.into_snapshot_owner()));
                blocked_on_head = true;
            },
        }
    }
    println!("[DEBUG] History reader exact physical census demands={physical_demands:?} releases={physical_releases:?} headBlocked={blocked_on_head} turns={turns}");
    assert!(physical_page_seen, "actual retained inverse4096-byte allocation is observed before release");
    println!("[DEBUG] History read physical inverse page4096 released under exact whole-backing authority with exact captured head preserved");
    assert!(turns > 1 && law["expected"]["cancelRetirementCompletes"].as_bool().unwrap());
    assert!(!blocked_on_head, "retiring this read cursor releases its own alias without consuming the independently captured head");
    let held_head = held_head.take().expect("the external readable head remains owned after cursor retirement");
    assert_eq!(Arc::as_ptr(held_head.snapshot_owner()), head_identity);
    assert_eq!(held_head.snapshot().n, head_value);
    drive_retirement_terminal(admit_demo_alias_retirement(&store,held_head.into_snapshot_owner()));
    for stop in law["cancelAfterSteps"].as_array().unwrap() {
        let mut cancelled = Some(store.begin_derived_history_preview(target.clone(), accepted.clone()).unwrap());
        for _ in 0..stop.as_u64().unwrap() {
            let grant=planning_grant(cancelled.as_ref().unwrap().planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).unwrap());
            if matches!(store.step_derived_history_preview(cancelled.as_mut().unwrap(), grant, &mut || true).unwrap(), ReplayStep::Finished(_)) { break; }
        }
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ArtifactStore::<DemoSnapshot,DemoMutation>::history_read_retirement_birth_bytes(),maximum_depth:1,..Default::default()};
        let(mut retirement,birth)=store.retire_derived_history_preview(&mut cancelled,grant).unwrap().unwrap();assert!(birth.fits(grant));
        assert!(cancelled.is_none(), "cancellation transfers ownership before any metadata drain");
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 100_000);
            let grant = native_retirement_grant(retirement.as_ref(), close_items, release_bytes);
            match retirement.close_step(grant).unwrap() {
                semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(retirement.terminal_is_empty()); break; }
                semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
                semio_framework_value::retained_clone::RetainedCloneStep::Blocked => panic!("unpublished preview owns no outstanding reads"),
            }
            assert_eq!(store.snapshot_ref().n, Some(99));
        }
        assert!(turns > 1 && law["expected"]["cancelRetirementCompletes"].as_bool().unwrap());
        let mut cancelled = Some(store.begin_derived_report_replay(accepted.clone(), Some(target.clone())).unwrap());
        for _ in 0..stop.as_u64().unwrap() {
            let grant=planning_grant(cancelled.as_ref().unwrap().planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).unwrap());
            if matches!(store.step_derived_report_replay(cancelled.as_mut().unwrap(), grant, &mut || true).unwrap(), ReplayStep::Finished(_)) { break; }
        }
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:ArtifactStore::<DemoSnapshot,DemoMutation>::history_read_retirement_birth_bytes(),maximum_depth:1,..Default::default()};
        let(mut retirement,birth)=store.retire_derived_report_replay(&mut cancelled,grant).unwrap().unwrap();assert!(birth.fits(grant));
        assert!(cancelled.is_none());
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 100_000);
            let grant = native_retirement_grant(retirement.as_ref(), close_items, release_bytes);
            match retirement.close_step(grant).unwrap() {
                semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(retirement.terminal_is_empty()); break; }
                semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
                semio_framework_value::retained_clone::RetainedCloneStep::Blocked => panic!("unpublished replay owns no outstanding reads"),
            }
            assert_eq!(store.snapshot_ref().n, Some(99));
        }
        assert!(turns > 1);
    }
    assert_eq!(store.snapshot_ref().n, Some(99));
    let mut stale = store.begin_derived_history_preview(target, accepted).unwrap();
    apply(&mut store, vec![add(1)]).await;
    let grant=planning_grant(stale.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).unwrap());
    assert!(store.step_derived_history_preview(&mut stale, grant, &mut || true).is_err());
    let tail = operation_ids(&store).last().unwrap().clone();
    assert_eq!(store.mutation_position(&tail).unwrap(), (2, 0));
    fixture_author(&mut store, ArtifactCommand::Undo).await.unwrap();
    assert!(store.mutation_position(&tail).is_err(), "the exact authority refuses an undone identity");
    fixture_author(&mut store, ArtifactCommand::Redo).await.unwrap();
    assert_eq!(store.mutation_position(&tail).unwrap(), (2, 0));
    eprintln!("[DEBUG] Bounded history planning, prefix folding, replay and cancelled ownership retirement match the neutral law without publishing");
}
