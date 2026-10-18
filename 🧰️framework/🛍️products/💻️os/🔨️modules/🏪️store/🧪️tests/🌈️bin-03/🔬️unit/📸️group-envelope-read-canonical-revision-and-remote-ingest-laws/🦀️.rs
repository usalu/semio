// 📸️ Group envelope read, canonical revision and remote ingest laws: the compound envelope read fixtures and laws, the canonical runtime seed and revision, the rejected remote ingest and the checkpoint transitions.

//#region 📸️CompoundEnvelopeReadLaws
struct GroupReadTriggerSnapshot {
    value: i32,
    commit: Option<Arc<Mutex<semio_framework_os_vcs::ArtifactGroupVisibilityOwner>>>,
    reads: Arc<std::sync::atomic::AtomicUsize>,
}

impl ToValue for GroupReadTriggerSnapshot {
    fn to_value(&self) -> DslValue {
        self.reads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if let Some(owner) = self.commit.as_ref() {
            owner.lock().expect("injected serializer decision").commit();
        }
        self.value.to_value()
    }
}

fn group_read_fixture_edit(id: &str) -> Edit<()> {
    Edit { line: None, id: id.into(), actor: None, forwards: Vec::new(), inverse: Vec::new().into(), mutation_meta: Vec::new(), verb: None, sequence_number: 0, started_at: String::new(), finished_at: None }
}

fn group_read_fixture_envelope(snapshot: GroupReadTriggerSnapshot) -> ArtifactEnvelope<GroupReadTriggerSnapshot, ()> {
    let mut edits = ArtifactHistoryLedger::new();
    edits.try_push(group_read_fixture_edit("initial")).unwrap();
    ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
        schema: "group-read/v1".into(),
        id: "group-reader".into(),
        vcs: ArtifactVcs { genesis: AdmittedArtifactGenesis::from_decoded_pack(snapshot, vec![0]), edits, changes: ArtifactHistoryLedger::new(), checkpoints: ArtifactHistoryLedger::new(), alternatives: ArtifactHistoryLedger::new() },
        backbone: None,
        active_alternative_id: None,
        cursor: Some(ArtifactCursor::new(vec!["initial".into()], Vec::new(), None)),
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::HistoryFoldIndex::new(),
        viewer_checkpoint_id: None,
        edit_messages: ArtifactEditMessageLedger::new(),
        conflicts: Vec::new(),
        transitions: Vec::new(),
        history_shape: semio_framework_os_spr::HistoryShape::Document,
        open_transaction: None,
    })
}

/// 📏️ Drives an original owner within the caller's separately declared five-axis policy.
fn drive_retirement_within(owner: &mut dyn ErasedSnapshotRetirement, grant: RetainedCloneGrant) {
    for _ in 0..4096 {
        if owner.terminal_is_empty() { return; }
        owner.next_copy_byte_demand().expect("fallible current copy frontier");owner.next_capacity_byte_demand(grant.maximum_copy_bytes).expect("fallible current capacity frontier");owner.next_release_byte_demand().expect("fallible current release frontier");owner.next_depth_demand().expect("fallible current depth frontier");
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(grant).expect("supplied retirement turn"));
        assert!(step.progress().fits(grant));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        if matches!(step,RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty());return; }
    }
    panic!("original owner did not reach terminal empty within bounded supplied turns");
}

fn close_group_read_fixture(envelope: ArtifactEnvelope<GroupReadTriggerSnapshot, ()>) {
    let mut owners = envelope.into_owners();
    let mut cursor = ArtifactStoreCursorRetirement::new(owners.cursor.take().unwrap());
    test_support::drive_retirement(&mut cursor).expect("group read fixture cursor reaches its terminal-empty witness under its quoted demands");
    assert!(cursor.terminal_is_empty());
    while owners.vcs.edits.pop().is_some() {}
}

#[test]
fn retained_group_envelope_read_captures_history_and_cursor_before_decision() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📖️group-read.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let owner = Arc::new(Mutex::new(semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new()));
        let view = owner.lock().unwrap().view();
        let inject_commit = case["capture"] == "pending" && case["decision"] == "committed";
        let reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut envelope = group_read_fixture_envelope(GroupReadTriggerSnapshot { value: 0, commit: inject_commit.then(|| owner.clone()), reads: reads.clone() });
        let prepared_ids: Vec<String> = serde_json::from_value(fixture["prepared"]["appliedEditIds"].clone()).unwrap();
        envelope.cursor.as_mut().unwrap().stage_group_owned(ArtifactCursorOwners { applied_edit_ids: prepared_ids.clone().into(), redo_edit_ids: Vec::new().into(), checkpoint_id: None }, &view).unwrap();
        for id in &prepared_ids[1..] {
            let reservation = envelope.vcs.edits.reserve_group_one(&view).unwrap();
            envelope.vcs.edits.stage_group_reserved(reservation, group_read_fixture_edit(id), &view).unwrap();
        }
        if case["capture"] == "committed" {
            assert!(owner.lock().unwrap().commit());
        }
        let read = envelope.capture_read().unwrap();
        if case["decision"] == "aborted" {
            assert!(owner.lock().unwrap().abort());
        }
        if inject_commit { assert!(owner.lock().unwrap().commit()); }
        let captured: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&read)).unwrap();
        let fresh: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&envelope.capture_read().unwrap())).unwrap();
        assert_eq!(reads.load(std::sync::atomic::Ordering::SeqCst), 0);
        for (value, selector) in [(&captured, "captured"), (&fresh, "fresh")] {
            let expected = &fixture[case[selector].as_str().unwrap()];
            let actual_ids: Vec<_> = value["vcs"]["edits"].as_array().unwrap().iter().map(|edit| edit["id"].clone()).collect();
            assert_eq!(serde_json::json!(actual_ids), expected["history"], "{} {selector}", case["id"]);
            assert_eq!(value["cursor"]["appliedEditIds"], expected["appliedEditIds"], "{} {selector}", case["id"]);
        }
        drop(read);
        let retired_cursor = if view.committed() {
            envelope.vcs.edits.adopt_group(&view).unwrap();
            envelope.cursor.as_mut().unwrap().adopt_group_owned(&view).unwrap()
        } else {
            owner.lock().unwrap().abort();
            while envelope.vcs.edits.abort_group_one(&view).unwrap().is_some() {}
            envelope.cursor.as_mut().unwrap().abort_group_owned(&view).unwrap()
        };
        let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(retired_cursor));
        test_support::drive_retirement(&mut retirement).expect("retired group cursor reaches its terminal-empty witness under its quoted demands");
        assert!(retirement.terminal_is_empty());
        close_group_read_fixture(envelope);
    }
}

#[test]
fn retained_group_envelope_read_rejects_foreign_cursor_visibility_before_serialization() {
    let mut history_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let history = history_owner.view();
    let mut cursor_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let cursor = cursor_owner.view();
    let reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut envelope = group_read_fixture_envelope(GroupReadTriggerSnapshot { value: 0, commit: None, reads: reads.clone() });
    let reservation = envelope.vcs.edits.reserve_group_one(&history).unwrap();
    envelope.vcs.edits.stage_group_reserved(reservation, group_read_fixture_edit("foreign"), &history).unwrap();
    envelope.cursor.as_mut().unwrap().stage_group_owned(ArtifactCursorOwners::default(), &cursor).unwrap();
    assert!(envelope.capture_read().is_err());
    assert!(envelope.capture_read().map(|read| semio_framework_pack_json::to_json_string(&read)).is_err());
    assert_eq!(reads.load(std::sync::atomic::Ordering::SeqCst), 0);
    history_owner.abort();
    cursor_owner.abort();
    while envelope.vcs.edits.abort_group_one(&history).unwrap().is_some() {}
    drop(envelope.cursor.as_mut().unwrap().abort_group_owned(&cursor).unwrap());
    close_group_read_fixture(envelope);
}

//#endregion 📸️CompoundEnvelopeReadLaws

#[semio_framework_async_macros::async_test]
async fn retained_group_cursor_shares_history_visibility_and_retires_displaced_roots() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🎯️group-cursor.json")).expect("group cursor fixture");
    let mut cursor: ArtifactCursor = serde_json::from_value(fixture["before"].clone()).expect("independent old cursor");
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["after"].clone()).expect("independent next cursor");
    let mut owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let mut history = ArtifactHistoryLedger::new();
    history.try_push("initial".to_string()).expect("old history");
    cursor.stage_group_owned(next, &view).expect("retained prepared cursor");
    for id in ["member-a", "member-b"] {
        let reservation = history.reserve_group_one(&view).expect("one history reservation");
        history.stage_group_reserved(reservation, id.to_string(), &view).expect("one prepared history record");
        assert_eq!(serde_json::to_value(&cursor).expect("old cursor serialization"), fixture["before"]);
        assert_eq!(cursor.applied_edit_ids.len(), 1);
        assert_eq!(history.len(), 1);
    }
    let held_before_commit: &ArtifactCursorOwners = &cursor;
    let held_history = history.iter();
    assert!(owner.commit());
    assert_eq!(held_before_commit.applied_edit_ids, ["initial".to_string()]);
    assert_eq!(held_history.cloned().collect::<Vec<_>>(), vec!["initial"]);
    assert_eq!(serde_json::to_value(&cursor).expect("committed cursor serialization"), fixture["after"]);
    assert_eq!(cursor.applied_edit_ids, history.iter().cloned().collect::<Vec<_>>());
    assert!(cursor.abort_group_owned(&view).is_err());
    let displaced = cursor.adopt_group_owned(&view).expect("exact displaced cursor owner");
    history.adopt_group(&view).expect("nonpublishing history adoption");
    assert_eq!(serde_json::to_value(&cursor).expect("adopted cursor serialization"), fixture["after"]);
    let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(displaced));
    drive_retirement_within(&mut retirement, RetainedCloneGrant { maximum_items:fixture["maximumItems"].as_u64().expect("item grant") as usize,maximum_copy_bytes:fixture["maximumBytes"].as_u64().expect("logical byte work") as usize,..physical_test_close_grant() });
    assert!(retirement.terminal_is_empty());
    while history.pop().is_some() {}
    assert!(history.terminal_is_empty());
    let mut abort_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let abort_view = abort_owner.view();
    let mut wrong_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let wrong_view = wrong_owner.view();
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["before"].clone()).expect("cancelled cursor owner");
    cursor.stage_group_owned(next, &abort_view).expect("unpublished next cursor");
    assert!(wrong_owner.abort());
    assert!(cursor.abort_group_owned(&wrong_view).is_err());
    assert!(abort_owner.abort());
    let cancelled = cursor.abort_group_owned(&abort_view).expect("returned exact uncommitted owner");
    assert_eq!(serde_json::to_value(cancelled).expect("cancelled owner oracle"), fixture["before"]);
    assert_eq!(serde_json::to_value(cursor).expect("unchanged committed cursor"), fixture["after"]);
}

#[semio_framework_async_macros::async_test]
async fn retained_group_cursor_empty_base_and_dropped_publisher_return_every_staged_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🎯️group-cursor.json")).expect("group cursor fixture");
    let mut cursor = ArtifactCursor::default();
    let mut history = ArtifactHistoryLedger::<String>::new();
    let owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
    let view = owner.view();
    let next: ArtifactCursorOwners = serde_json::from_value(fixture["after"].clone()).expect("prepared cursor");
    cursor.stage_group_owned(next, &view).expect("staged empty-base cursor");
    #[cfg(not(target_arch = "wasm32"))]
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cursor.applied_edit_ids.push("forbidden".into()))).is_err(), "DerefMut cannot expose either root while staged");
    let reservation = history.reserve_group_one(&view).expect("reserved but not staged slot");
    assert!(cursor.applied_edit_ids.is_empty() && history.is_empty());
    assert_eq!(serde_json::to_value(&cursor).expect("empty cursor"), serde_json::json!({}));
    drop(owner);
    assert!(!view.pending() && !view.committed(), "dropping the unique publisher aborts the decision without dropping staged payloads");
    history.cancel_reservation(reservation).expect("exact cancelled reservation");
    assert_eq!(history.abort_group_one(&view), Ok(None));
    assert!(history.terminal_is_empty());
    let cancelled = cursor.abort_group_owned(&view).expect("exact staged cursor survives publisher drop");
    assert_eq!(serde_json::to_value(&cancelled).expect("cancelled owner"), fixture["after"]);
    let mut retirement = ArtifactStoreCursorRetirement::new(ArtifactCursor::from_owners(cancelled));
    drive_retirement_within(&mut retirement, physical_test_close_grant());
    assert!(retirement.terminal_is_empty());
    assert!(cursor.applied_edit_ids.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn canonical_runtime_seed_retains_duplicate_owners_and_preflights_before_building() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🌱️runtime-seed.json")).expect("runtime seed fixture");
    let ids: Vec<MutationId> = serde_json::from_value::<Vec<String>>(fixture["identities"].clone()).expect("fixture identities").into_iter().map(MutationId).collect();
    let expected: Vec<String> = serde_json::from_value(fixture["applied"].clone()).expect("independent fixture applied identities");
    let retired: Vec<String> = serde_json::from_value(fixture["retired"].clone()).expect("independent fixture duplicate identities");
    semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&ids).expect("duplicate identities are valid existing authority");
    let (mut dag, duplicates) = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::adopt_runtime_seed(ids);
    assert_eq!(duplicates, retired);
    let mut applied = Vec::new();
    while let Some(semio_framework_os_spr::MutationDagCloseOwner::Identity(id)) = dag.take_one_close_owner() {
        applied.push(id);
    }
    applied.reverse();
    assert_eq!(applied, expected);
    assert!(dag.terminal_is_empty());
    let mut retirement = ArtifactStoreStringVectorRetirement::new(duplicates);
    drive_retirement_within(&mut retirement, RetainedCloneGrant { maximum_items:fixture["maximumItems"].as_u64().expect("item grant") as usize,maximum_copy_bytes:fixture["maximumBytes"].as_u64().expect("logical byte work") as usize,..physical_test_close_grant() });
    assert!(retirement.terminal_is_empty());
    let oversized = vec![MutationId("x".repeat(semio_framework_os_spr::causal::MUTATION_DAG_IDENTIFIER_BYTES + 1))];
    assert!(semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&oversized).is_err());
    let saturated: Vec<_> = (0..=semio_framework_os_spr::causal::MUTATION_DAG_CAPACITY).map(|index| MutationId(index.to_string())).collect();
    assert!(semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::preflight_runtime_seed(&saturated).is_err());
}

#[semio_framework_async_macros::async_test]
async fn canonical_revision_distinguishes_interior_aba_across_load_and_reset() {
    let mut original = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "revision-aba", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    original.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    for n in [1, 2, 3] {
        test_support::dispatch_test_command(&mut original, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n })], transaction: None }).await.expect("seed revision edit");
    }
    let original_revision = original.content_revision().await;
    let mut changed = owned_test_envelope(&original).await;
    changed.vcs.edits[1].forwards = vec![DemoMutation::SetN(SetN { n: 99 })];
    changed.vcs.edits[1].inverse = vec![DemoMutation::AssignN(AssignN { n: Some(1) })].into();
    changed.vcs.edits[2].inverse = vec![DemoMutation::AssignN(AssignN { n: Some(99) })].into();

    let mut loaded = ArtifactStore::bare(changed).await;
    loaded.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    assert_eq!(original.snapshot().expect("original snapshot").n, loaded.snapshot().expect("loaded snapshot").n, "interior ABA keeps the same materialized endpoint");
    assert_ne!(original_revision, loaded.content_revision().await, "canonical identity must cover the changed interior edit, not only cursor endpoints");

    original.reset(owned_test_envelope(&loaded).await).await.expect("reset changed history");
    assert_eq!(original.content_revision().await, loaded.content_revision().await, "reset reconstruction must recover the canonical loaded identity");
    close_demo_artifact_store(&mut loaded);
    close_demo_artifact_store(&mut original);
}


#[semio_framework_async_macros::async_test]
async fn rejected_remote_ingest_keeps_state_and_dag_unpoisoned() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let valid = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 7 })).await;
    let mut malformed = valid.clone();
    malformed.diff.payload = vec![0xff];
    let before = owned_test_envelope(&store).await;

    assert!(store.ingest_remote(malformed).await.is_err(), "malformed remote data must reject before committing the DAG or history");
    assert_eq!(store.envelope(), &before);
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(0) });

    store.ingest_remote(valid).await.expect("the rejected envelope must not poison its mutation id in the DAG");
    assert_eq!(store.snapshot().expect("accepted snapshot"), DemoSnapshot { n: Some(7) });
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn remote_ingest_requires_duplicate_mutation_payload_equivalence() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let accepted = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 2 })).await;
    store.ingest_remote(accepted.clone()).await.expect("first remote envelope");
    let before = owned_test_envelope(&store).await;

    store.ingest_remote(accepted.clone()).await.expect("an exact duplicate is idempotent");
    let mut conflict = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 9 })).await;
    conflict.mutation_id = accepted.mutation_id.clone();
    let error = store.ingest_remote(conflict).await.expect_err("the same mutation id may not carry a different payload");
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("conflicts with its established payload")));
    assert_eq!(store.envelope(), &before);
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(2) });
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn a_remote_transition_repeating_a_known_id_with_another_payload_is_refused_without_mutation() {
    let mut author = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("author edit");
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Undo).await.expect("author undo");
    let events = author.event_log().expect("author event log");
    let mut replica = fresh_demo_store().await;
    for event in events.clone() {
        replica.ingest_remote(event).await.expect("replica ingests the author's events");
    }
    assert_eq!(replica.snapshot().expect("replica snapshot"), DemoSnapshot { n: Some(0) }, "the replicated revert withdraws the edit");
    let before = owned_test_envelope(&replica).await;
    let mut forged = events.last().expect("the revert transition").clone();
    let semio_framework_os_spr::HistoryTransition::Revert { mutation_ids } = semio_framework_os_spr::history_transition_from_envelope(&forged).expect("decode").expect("a transition") else { panic!("the last event is the revert") };
    forged.diff.payload = semio_framework_os_spr::encode_history_transition(&semio_framework_os_spr::HistoryTransition::Reinstate { mutation_ids });
    let error = replica.ingest_remote(forged).await.expect_err("a known transition id may not carry another payload");
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("conflicts with its established payload")));
    assert_eq!(replica.envelope(), &before);
    retire_demo_envelope(before);
}

/// 🐛️ Two replicas each commit a checkpoint over the same edit: every change on either replica must
/// name edits that replica actually holds, because a `Commit` transition names wire operations and
/// each replica resolves them against its own edit ledger.
#[semio_framework_async_macros::async_test]
async fn checkpoint_transitions_resolve_against_each_replicas_own_edits() {
    let mut a = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut a, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 }), DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("a's two-operation edit");
    let mut b = fresh_demo_store().await;
    for event in a.event_log().expect("a's log") {
        b.ingest_remote(event).await.expect("b ingests a's operations");
    }
    test_support::dispatch_test_command(&mut b, ArtifactCommand::CommitCheckpoint { message: Some("b checkpoint".into()), authors: Vec::new() }).await.expect("b's own checkpoint");
    test_support::dispatch_test_command(&mut a, ArtifactCommand::CommitCheckpoint { message: Some("a checkpoint".into()), authors: Vec::new() }).await.expect("a's own checkpoint");
    for event in a.event_log().expect("a's log") {
        b.ingest_remote(event).await.expect("b ingests a's checkpoint transition");
    }
    for event in b.event_log().expect("b's log") {
        a.ingest_remote(event).await.expect("a ingests b's checkpoint transition");
    }
    for store in [&a, &b] {
        assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "both checkpoints are facts on every replica");
        for change in &store.envelope().vcs.changes {
            for edit_id in &change.edit_ids {
                assert!(store.envelope().vcs.edits.iter().any(|edit| edit.id == *edit_id), "change {} references edit {edit_id} the replica does not hold", change.id);
            }
        }
    }
    assert_eq!(a.snapshot().expect("a"), b.snapshot().expect("b"));
    assert_eq!(a.current_checkpoint_id().await, b.current_checkpoint_id().await, "the later commit in HLC order is current on both replicas");
}
