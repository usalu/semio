//! ✏️ Store laws of non-destructive history editing: effective forwards under supersessions, Report-mode replays with
//! per-mutation outcomes, the finalize gate, scoped and unscoped supersessions, replica convergence, cancellation,
//! persistence, convergence early exit and the prefix-snapshot ring (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
use super::*;

//#region 🧰️Harness
async fn demo_store(id: &str, n: Option<i32>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n }, None)).await
}

async fn apply(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, mutations: Vec<DemoMutation>) {
    store.dispatch(ArtifactCommand::Apply { mutations, description: None, transaction: None }).await.expect("a clean edit applies");
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

fn drafts(entries: &[(&MutationId, Option<DemoMutation>)]) -> BTreeMap<MutationId, protocol::InputReplacement> {
    entries.iter().map(|(target, replacement)| ((*target).clone(), draft(replacement.clone()))).collect()
}

fn finish(store: &ArtifactStore<DemoSnapshot, DemoMutation>, mut replay: EditReplay<DemoSnapshot, DemoMutation>) -> EditReplayResult<DemoSnapshot, DemoMutation> {
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), ReplayStep::Finished(_)));
    replay.finish().expect("a finished replay yields its result")
}

/// 📋️ Per-mutation outcomes keyed by identity: what replicas with different edit boundaries must agree on.
fn outcomes_by_mutation(outcomes: &[protocol::MutationReplayOutcome]) -> BTreeMap<MutationId, (Option<crate::os_dsl::Severity>, Vec<String>, bool, bool)> {
    outcomes.iter().map(|outcome| (outcome.mutation_id.clone(), (outcome.worst, outcome.messages.iter().map(|message| message.code.0.clone()).collect(), outcome.superseded, outcome.withdrawn))).collect()
}

/// 📋️ Per-mutation outcomes by applied position and operation index: what two independently authored stores agree on.
fn outcomes_by_position(store: &ArtifactStore<DemoSnapshot, DemoMutation>, outcomes: &[protocol::MutationReplayOutcome]) -> Vec<(usize, u32, Option<crate::os_dsl::Severity>, Vec<String>, bool, bool)> {
    outcomes
        .iter()
        .map(|outcome| (store.applied_edit_ids().iter().position(|id| *id == outcome.edit_id).expect("an outcome names an applied edit"), outcome.op_index, outcome.worst, outcome.messages.iter().map(|message| message.code.0.clone()).collect(), outcome.superseded, outcome.withdrawn))
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
        let drafts: BTreeMap<MutationId, protocol::InputReplacement> = case["supersessions"]
            .as_array()
            .expect("supersessions")
            .iter()
            .map(|supersession| {
                let replacement = match &supersession["replacement"] {
                    serde_json::Value::String(withdrawn) if withdrawn == "withdrawn" => protocol::InputReplacement::Withdrawn,
                    serde_json::Value::Object(input) if input.contains_key("invalid") => protocol::InputReplacement::Input { schema: input["invalid"]["schema"].as_str().expect("invalid schema").into(), payload: hex(input["invalid"]["payloadHex"].as_str().expect("invalid payload")) },
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
                assert!(messages.iter().any(|message| message.level >= crate::os_dsl::Severity::Error));
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
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("a clean supersession authors");
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
    assert_eq!(outcomes_by_position(&store, &store.mutation_outcomes().unwrap()).into_iter().map(|(edit, op, worst, codes, _, _)| (edit, op, worst, codes)).collect::<Vec<_>>(), outcomes_by_position(&fresh, &fresh.mutation_outcomes().unwrap()).into_iter().map(|(edit, op, worst, codes, _, _)| (edit, op, worst, codes)).collect::<Vec<_>>());
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
    apply(&mut store, vec![DemoMutation::RestoreN(RestoreN { n: Some(5) })]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let supersede = |replacement: Option<DemoMutation>, logical: u64| {
        let transition = crate::os_spr::HistoryTransition::Supersede(protocol::TransitionSupersede { scope: None, inputs: vec![protocol::SupersededInput { target: ids[0].clone(), replacement: draft(replacement) }] });
        crate::os_spr::encode_envelopes(&[crate::os_spr::history_transition_envelope(&transition, &ArtifactId("check-in".into()), &ActorId("editor".into()), vec![ids[0].clone()], HybridLogicalTimestamp { actor: 9, physical_ms: u64::MAX / 2, logical })])
    };
    let refused = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &supersede(None, 0), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "{refused:?}");
    assert!(refused.unwrap_err().to_string().contains("rejected by merge policy Normal"));
    let accepted = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &supersede(Some(DemoMutation::RestoreN(RestoreN { n: Some(40) })), 1), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await.expect("a clean supersession checks in");
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
        store.dispatch(ArtifactCommand::Apply { mutations: vec![operation], description: None, transaction: None }).await.expect("clean severity edit");
    }
    let ids: Vec<MutationId> = store.mutation_ops().unwrap().into_iter().map(|operation| operation.mutation_id).collect();
    let severity_draft = |operation: SeverityMutation| protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: operation.encode_op().unwrap() };
    let entries = BTreeMap::from([(ids[0].clone(), severity_draft(SeverityMutation::SetWarningN(SetWarningN { n: 5 }))), (ids[1].clone(), severity_draft(SeverityMutation::SetErrorN(SetErrorN { n: 6 }))), (ids[2].clone(), severity_draft(SeverityMutation::SetFatalN(SetFatalN { n: 7 })))]);
    let mut replay = store.begin_report_replay(&entries, None).expect("preview replay");
    replay.step(store.replay_edits(), &mut || false).expect("steps");
    let result = replay.finish().expect("finished");
    let report = store.replay_report(&result).expect("report");
    let levels: Vec<Option<crate::os_dsl::Severity>> = report.outcomes.iter().map(|outcome| outcome.worst).collect();
    assert_eq!(levels, vec![Some(crate::os_dsl::Severity::Warning), Some(crate::os_dsl::Severity::Error), Some(crate::os_dsl::Severity::Fatal)]);
    assert!(report.blocks_finalize());
    assert_eq!(report.worst, Some(crate::os_dsl::Severity::Fatal));
    drop(result);
    let generation = store.generation();
    let snapshot = store.snapshot().unwrap();
    for (target, operation) in [(&ids[1], SeverityMutation::SetErrorN(SetErrorN { n: 6 })), (&ids[2], SeverityMutation::SetFatalN(SetFatalN { n: 7 }))] {
        let refused = store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: target.clone(), replacement: Some(operation) }] }).await;
        assert!(matches!(refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, ref messages }) if !messages.is_empty()), "{refused:?}");
        assert_eq!(store.generation(), generation, "a refused supersession changes nothing");
        assert_eq!(store.snapshot().unwrap(), snapshot);
        assert!(store.supersessions().is_empty());
    }
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: ids[0].clone(), replacement: Some(SeverityMutation::SetWarningN(SetWarningN { n: 5 })) }] }).await.expect("a warning never blocks");
    let durable = store.mutation_outcomes().unwrap();
    assert_eq!(durable[0].worst, Some(crate::os_dsl::Severity::Warning), "the warning persists in history");
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
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], None)] }).await.expect("withdraw");
    assert_eq!(store.snapshot_ref().n, Some(6));
    let outcome = store.mutation_outcomes().unwrap().into_iter().find(|outcome| outcome.mutation_id == ids[1]).expect("withdrawn outcome");
    assert!(outcome.withdrawn && outcome.superseded && outcome.messages.is_empty());
    test_support::assert_live_equals_replay(&store).await;
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], Some(add(20)))] }).await.expect("a later supersession wins");
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
    store.dispatch(ArtifactCommand::CreateAlternativeWithSupersede { name: "variant".into(), inputs: vec![input(&ids[0], Some(set(10)))] }).await.expect("variant alternative");
    let variant = store.envelope().active_alternative_id.clone().expect("the variant is active");
    assert_eq!(store.snapshot_ref().n, Some(11));
    assert_eq!(store.supersessions().get(&ids[0]).and_then(|supersession| supersession.scope.clone()), Some(variant.clone()));
    let outbound = store.envelope().transitions.len();
    assert_eq!(outbound, 3, "commit, branch and scoped supersede are one authored batch");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "other".into() }).await.expect("another alternative at the head");
    assert_eq!(store.snapshot_ref().n, Some(2), "the scoped supersession leaves with its alternative");
    assert!(store.supersessions().is_empty());
    test_support::assert_live_equals_replay(&store).await;
    store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: variant.clone() }).await.expect("switch back");
    assert_eq!(store.snapshot_ref().n, Some(11), "switching back re-applies the scoped supersession");
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[1], Some(add(5)))] }).await.expect("unscoped");
    assert_eq!(store.snapshot_ref().n, Some(15));
    let other = store.envelope().vcs.alternatives.iter().find(|alternative| alternative.name == "other").map(|alternative| alternative.id.clone()).expect("other alternative");
    store.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: other }).await.expect("switch to other");
    assert_eq!(store.snapshot_ref().n, Some(6), "an unscoped supersession holds in every alternative");
    test_support::assert_live_equals_replay(&store).await;
    test_support::assert_document_text_round_trip(&store).await;
    let refused = store.dispatch(ArtifactCommand::Supersede { scope: Some("missing".into()), inputs: vec![input(&ids[1], Some(add(5)))] }).await;
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
    author.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(7)))] }).await.expect("supersede");
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
    let probe = store.begin_report_replay(&entries, None).expect("probe");
    let total = probe.progress().total;
    probe.cancel();
    assert_eq!(total, 4);
    for stop in 0..=total {
        let (generation, revision, snapshot, applied, transitions, outcomes) = (store.generation(), store.0.content_revision(), store.snapshot().unwrap(), store.applied_edit_ids().to_vec(), store.envelope().transitions.len(), store.mutation_outcomes().unwrap());
        let mut replay = store.begin_report_replay(&entries, None).expect("replay");
        if stop > 0 {
            let mut done = 0;
            let step = replay.step(store.replay_edits(), &mut || {
                done += 1;
                done >= stop
            });
            assert!(step.is_ok());
            assert_eq!(replay.progress().done, stop);
        }
        replay.cancel();
        assert_eq!(store.generation(), generation);
        assert_eq!(store.0.content_revision(), revision);
        assert_eq!(store.snapshot().unwrap(), snapshot);
        assert_eq!(store.applied_edit_ids(), applied.as_slice());
        assert_eq!(store.envelope().transitions.len(), transitions);
        assert_eq!(store.mutation_outcomes().unwrap(), outcomes);
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
        store.state_before(&ids[6], &BTreeMap::new()).map(|preview| drive_retirement_terminal(store.retire_snapshot_alias(preview).expect("preview alias retires"))).expect("the preview populates the prefix ring");
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
        store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&target, Some(set(50)))] }).await.expect("supersede");
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
    let mut observe = |target: &MutationId, entries: BTreeMap<MutationId, protocol::InputReplacement>| {
        let preview = store.state_before(target, &entries).expect("preview");
        let n = preview.n;
        drive_retirement_terminal(store.retire_snapshot_alias(preview).expect("alias retires"));
        n
    };
    assert_eq!(observe(&ids[0], BTreeMap::new()), Some(0));
    assert_eq!(observe(&ids[3], BTreeMap::new()), Some(6));
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
    let preview = store.state_before(&ids[5], &BTreeMap::new()).expect("preview");
    drive_retirement_terminal(store.retire_snapshot_alias(preview).expect("alias retires"));
    let retained: Vec<usize> = store.prefix_ring.iter().map(|entry| entry.length).collect();
    assert!(!retained.is_empty() && retained.iter().all(|length| *length >= 1));
    let displaced = store.displaced_retirements.owners.len();
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(40)))] }).await.expect("supersede");
    assert!(store.displaced_retirements.owners.len() > displaced, "stale ring entries retire through displaced owners");
    let live = forward_prefix_digests::<DemoSnapshot, DemoMutation>(store.revision_accumulator.identity_digest, store.applied_edit_ids(), &store.envelope().vcs.edits, store.supersessions()).expect("digests");
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
    store.dispatch(ArtifactCommand::Apply { mutations: vec![set(3), add(1)], description: None, transaction: Some(transaction.clone()) }).await.expect("apply in a transaction");
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
        ArtifactCommand::Apply { mutations: vec![set(1)], description: Some("drag".into()), transaction: Some(transaction.clone()) },
        ArtifactCommand::ApplyInLane { mutations: vec![add(1)], description: None, lane: HistoryLane::Interaction, transaction: Some(transaction) },
    ];
    for command in commands {
        let text = print_command(&command).await.expect("prints");
        assert_eq!(parse_command::<DemoMutation>(&text).await.expect("parses"), command, "{text}");
        let bytes = command.encode_op().expect("encodes");
        assert_eq!(ArtifactCommand::<DemoMutation>::decode_op(&bytes).expect("decodes"), command);
        assert_eq!(ArtifactCommand::<DemoMutation>::from_value(command.to_value()).expect("value decodes"), command);
    }
    assert!(parse_command::<DemoMutation>("supersede\n  replace m-1\n").await.is_err(), "a replace line needs its replacement");
    assert!(parse_command::<DemoMutation>("supersede\n").await.is_err(), "a supersede needs an input");
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
    store.envelope().vcs.edits.iter().map(|edit| (edit.id.clone(), edit.inverse.clone())).collect()
}

/// 💧️ A member document whose history holds supersessions hydrates — the retained composition member open — to exactly
/// the store a `.pack`/`.spr` parse builds: the superseded state, the effective supersessions, the recomputed inverses,
/// the durable outcomes and the content revision. The `.ops` text mirror round-trips the same history (audit F-C1).
#[semio_framework_async_macros::async_test]
async fn a_member_document_with_supersessions_hydrates_to_its_superseded_state() {
    let dialect = crate::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() };
    let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-superseded", DemoSnapshot { n: Some(0) }, None);
    envelope.dialect = Some(dialect.clone());
    let mut store = ArtifactStore::new(envelope).await;
    apply(&mut store, vec![set(1)]).await;
    apply(&mut store, vec![add(2)]).await;
    apply(&mut store, vec![add(3), add(4)]).await;
    let ids = operation_ids(&store);
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(set(10))), input(&ids[2], None)] }).await.expect("a clean supersession");
    assert_eq!(store.snapshot_ref().n, Some(16));
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reference = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("pair parses").into_envelope()).await;
    let history = crate::os_spr::decode_history(&files.spr, &crate::os_spr::DecodeOptions::default()).await.expect("history decodes");
    let expected = crate::os_io::ArtifactRef { artifact_id: "member-superseded".into(), dialect };
    let (operation, generation) = (semio_framework_job::OperationId(3), semio_framework_job::Generation(5));
    let mut hydration = RetainedPersistedDocumentHydration::<DemoSnapshot, DemoMutation>::from_pack(files.pack.clone(), history, expected, None, "demo/v1".into(), DemoSnapshot::member_store_owners(), operation, generation, u64::MAX, PersistedDocumentHydrationTarget::Store { generation: 0 });
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
        assert_eq!(outcomes[target], (Some(crate::os_dsl::Severity::Fatal), vec!["mutation.invariant".to_string()], true, false));
    }
    assert_eq!(store.state_before(&ids[2], &BTreeMap::new()).expect("state before").n, Some(1));
    assert_eq!(materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("materialized").n, Some(1));
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr).await.expect("the pair loads").into_envelope()).await;
    assert_eq!(reloaded.snapshot_ref().n, Some(1));
    assert_eq!(outcomes_by_mutation(&reloaded.mutation_outcomes().unwrap()), outcomes);
    let text = print_document_text(store.envelope()).await.expect("text prints");
    let mirrored = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops).await.expect("the text mirror loads").into_envelope()).await;
    assert_eq!(mirrored.snapshot_ref().n, Some(1));
    let refused = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&baseline.pack, &baseline.spr, &crate::os_spr::encode_envelopes(std::slice::from_ref(&garbage)), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.invariant")), "{refused:?}");
    let repair = remote_supersession("garbage", &ids[1], draft(Some(add(5))), 2);
    let repaired = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&baseline.pack, &baseline.spr, &crate::os_spr::encode_envelopes(&[garbage, repair]), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await.expect("a repaired ledger checks in");
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
        assert_eq!((summary.code.0.as_str(), summary.level), ("mutation.cascade", crate::os_dsl::Severity::Info));
        summary.message.strip_suffix(" more messages").expect("the summary counts what it drops").parse().expect("a count")
    };
    let mut store = demo_store("bounded", None).await;
    apply(&mut store, vec![DemoMutation::RestoreN(RestoreN { n: Some(5) })]).await;
    apply(&mut store, (0..OPERATIONS).map(|_| add(1)).collect()).await;
    let ids = operation_ids(&store);
    let bulk = store.applied_edit_ids()[1].clone();
    let applied = store.messages_for_edit(&bulk).to_vec();
    assert!(bytes(&applied, &bulk) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES);
    assert_eq!(applied.len() - 1 + dropped(&applied), OPERATIONS, "kept and summarized messages account for every operation");
    store.dispatch(ArtifactCommand::Supersede { scope: None, inputs: vec![input(&ids[0], Some(DemoMutation::RestoreN(RestoreN { n: Some(7) })))] }).await.expect("an oversize but clean replay installs");
    assert_eq!(store.snapshot_ref().n, Some(207));
    assert_eq!(store.messages_for_edit(&bulk), applied.as_slice(), "the replayed entry bounds exactly like the applied one");
    store.dispatch(ArtifactCommand::CreateAlternativeWithSupersede { name: "bounded-variant".into(), inputs: vec![input(&ids[0], Some(DemoMutation::RestoreN(RestoreN { n: Some(9) })))] }).await.expect("an oversize replay branches");
    assert_eq!(store.envelope().vcs.alternatives.iter().filter(|alternative| alternative.name == "bounded-variant").count(), 1);
    assert_eq!(store.snapshot_ref().n, Some(209));
    let withdraw = remote_supersession("bounded", &ids[0], protocol::InputReplacement::Withdrawn, 0);
    let report = store.ingest_remote(withdraw).await.expect("a remote supersession is never refused for size");
    assert!(report.accepted && report.conflict.is_none());
    assert_eq!(store.snapshot_ref().n, None);
    let failed = store.messages_for_edit(&bulk).to_vec();
    assert!(bytes(&failed, &bulk) <= ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES);
    assert!(failed[..failed.len() - 1].iter().all(|message| message.code.0 == "mutation.target-missing" && message.level == crate::os_dsl::Severity::Error), "the worst messages are the ones kept");
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
    apply(&mut store, vec![DemoMutation::RestoreN(RestoreN { n: Some(5) })]).await;
    apply(&mut store, vec![add(1)]).await;
    let ids = operation_ids(&store);
    let files = print_document_pack(store.envelope()).await.expect("pair prints");
    let breaking = remote_supersession("intermediate", &ids[0], protocol::InputReplacement::Withdrawn, 0);
    let healing = remote_supersession("intermediate", &ids[0], draft(Some(DemoMutation::RestoreN(RestoreN { n: Some(7) }))), 1);
    let refused = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &crate::os_spr::encode_envelopes(std::slice::from_ref(&breaking)), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await;
    assert!(matches!(&refused, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "{refused:?}");
    let healed = replay_envelopes_onto_pair::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &crate::os_spr::encode_envelopes(&[breaking, healing]), test_support::plain_document_store_owners::<DemoSnapshot, DemoMutation>).await.expect("a ledger whose final history is clean checks in");
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
    apply(&mut store, vec![DemoMutation::RestoreN(RestoreN { n: Some(5) })]).await;
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
    };
    let report = store.ingest_remote(peer).await.expect("the peer edit ingests");
    assert!(report.accepted);
    assert_eq!(store.snapshot_ref().n, None);
    store.dispatch(ArtifactCommand::Undo).await.expect("the interior delete reverts");
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
    let restate = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| drafts(&[(&operation_ids(store)[0], Some(DemoMutation::RestoreN(RestoreN { n: Some(5) })))]);
    let converged = finish(&early, early.begin_report_replay(&restate(&early), None).expect("early replay"));
    assert!(converged.converged_at().is_some(), "an equal prefix state converges");
    let replayed = finish(&full, full.begin_report_replay(&restate(&full), None).expect("full replay"));
    assert!(replayed.converged_at().is_none());
    assert_eq!(outcomes_by_position(&early, &early.replay_report(&converged).unwrap().outcomes), outcomes_by_position(&full, &full.replay_report(&replayed).unwrap().outcomes));
    drop((converged, replayed));
    early.dispatch(ArtifactCommand::Redo).await.expect("redo re-applies the interior delete");
    full.dispatch(ArtifactCommand::Redo).await.expect("redo re-applies the interior delete");
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
    author.dispatch(ArtifactCommand::Undo).await.expect("undo add 3");
    author.dispatch(ArtifactCommand::Undo).await.expect("undo add 2");
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
