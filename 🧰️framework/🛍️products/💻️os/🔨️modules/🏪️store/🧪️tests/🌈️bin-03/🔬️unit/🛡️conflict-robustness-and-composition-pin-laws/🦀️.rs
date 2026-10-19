// 🛡️ Conflict robustness and composition pin laws: the ghost edit id errors, the conflict retirement cursors, the open conflict capacity, the resolved conflict pruning, the quarantine message clearing and the composition pins.

//#region 🔖️RobustnessTests
// 🎯️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` J1 — security/
// robustness audit findings HIGH-1, HIGH-2, MEDIUM-3, MEDIUM-4 (`📓️j1-robustness-fixes.md`).
// Every test here FAILS without its corresponding fix.

/// 👻️ HIGH-1: an id named in `order[k..]` but absent from `edits` means `applied_edit_ids`/
/// `vcs.edits` fell out of sync (crash/recovery, a partially-applied ingest) — a bare
/// `continue` here used to silently compute a WRONG snapshot instead of failing loudly.
#[semio_framework_async_macros::async_test]
async fn replay_suffix_partitioned_errors_loudly_on_a_ghost_edit_id_instead_of_silently_dropping_it() {
    let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
    let order = vec!["ghost-edit".to_string()];
    let error = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::replay_suffix_partitioned(Arc::new(DemoSnapshot { n: Some(0) }), &order, 0, &edits, "demo/v1", &EffectiveSupersessions::new(), semio_framework_os_spr::MergePolicy::Normal)
        .err()
        .expect("a ghost edit id must be a loud, typed VcsError");
    assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
}

/// 🔦️ HIGH-2: minting a `ConflictId` from a `filter_map` that silently drops missing ids can
/// (if every id is missing) hash an EMPTY mutation-id set into a content-addressed conflict
/// id — a content address that addresses no content, so unrelated conflicts can collide on
/// it. `edits_for_ids` is the strict replacement both `ingest_remote` mint sites now use.
#[semio_framework_async_macros::async_test]
async fn edits_for_ids_errors_loudly_on_a_ghost_edit_id_instead_of_silently_filtering_it() {
    let edits: HashMap<String, Edit<DemoMutation>> = HashMap::new();
    let error = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::edits_for_ids(&["ghost-edit".to_string()], &edits).expect_err("an id that resolves to nothing must fail loudly, never vanish from the mutation-id set");
    assert_eq!(error, VcsError::UnknownEdit("ghost-edit".into()));
}

/// 🛰️ A synthetic `Open` conflict for capacity/pruning fixtures — content doesn't matter, only
/// that its `ConflictId`/timestamp are distinct per `seed` and it never gets touched by real
/// replay logic (these tests push it directly onto `envelope.conflicts`, bypassing `ingest_
/// remote`/`resolve_conflict` entirely, exactly like the existing hand-built conflict fixtures
/// in `🔖️PreviewWireTests` above).
async fn synthetic_open_conflict(seed: u64) -> semio_framework_os_spr::Conflict {
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![format!("synthetic-edit-{seed}")] };
    let artifact = ArtifactId("demo".into());
    let mutation_ids = vec![MutationId(format!("synthetic-op-{seed}"))];
    let timestamp = HybridLogicalTimestamp::new(seed, seed);
    semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &artifact, &mutation_ids, &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: Vec::new(),
        actors: vec![ActorId(format!("synthetic-actor-{seed}").into())],
        timestamp,
    }
}

#[semio_framework_async_macros::async_test]
async fn conflict_retirement_cursors_quarantined_payloads_messages_actors_and_ids() {
    let mut envelope = foreign_mutation_envelope("conflict-actor", DemoMutation::SetN(SetN { n: 9 })).await;
    envelope.dependencies = (0..32).map(|index| MutationId(format!("dependency-{index}-{}", "d".repeat(128)))).collect();
    envelope.diff.payload = vec![7; 16 * 1024];
    envelope.inverse.payload = vec![3; 16 * 1024];
    let kind = semio_framework_os_spr::ConflictKind::Quarantined { envelopes: vec![envelope] };
    let timestamp = HybridLogicalTimestamp::new(41, 43);
    let conflict = semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId("conflict-document".repeat(64)), &[MutationId("conflict-mutation".repeat(64))], &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: vec![semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", "message".repeat(256)).at(["target".repeat(256)])],
        actors: vec![ActorId("actor".repeat(256).into())],
        timestamp,
    };
    let mut retirement = ArtifactStoreConflictRetirement::new(conflict);
    test_support::drive_retirement(&mut retirement).expect("deep conflict retirement terminates across repeated interruption under its quoted demands");
    assert!(retirement.terminal_is_empty());
}

/// 🚧️ MEDIUM-3: a peer that keeps sending a batch this replica keeps quarantining can grow
/// `envelope.conflicts` without bound (the dag never advances on quarantine, so the SAME
/// envelope is eligible for redelivery forever). Hitting the open-conflict cap must be a
/// loud, typed refusal — atomic, nothing applied — never a silent drop or overwrite.
#[semio_framework_async_macros::async_test]
async fn ingest_remote_refuses_a_new_open_conflict_once_the_backlog_is_at_capacity() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("the delete alone raises no message and always applies");

    let cap = semio_framework_os_store_engine::lifted_store::StoreLog::<DemoSnapshot, DemoMutation>::OPEN_CONFLICT_CAP;
    for seed in 0..cap as u64 {
        store.0.log.envelope.conflicts.push(synthetic_open_conflict(seed).await);
    }
    assert_eq!(store.open_conflicts().count(), cap, "fixture must actually be at capacity for this test to be meaningful");

    let before_edits = store.envelope().vcs.edits.len();
    let before_snapshot = store.snapshot().expect("pre-attempt snapshot");
    let before_ids = store.applied_edit_ids().to_vec();

    let error = store.ingest_remote(modify).await.expect_err("minting one more Open conflict past the cap must be a loud, typed refusal");
    assert!(matches!(&error, VcsError::ValidationFailed(message) if message.contains("capacity")), "got {error:?}");
    assert_eq!(store.envelope().vcs.edits.len(), before_edits, "a refused mint must be fully atomic — nothing about the attempted batch applied");
    assert_eq!(store.snapshot().expect("snapshot"), before_snapshot);
    assert_eq!(store.applied_edit_ids(), before_ids.as_slice());
    assert_eq!(store.open_conflicts().count(), cap, "the backlog must stay exactly at capacity, never silently grow past it");
}

/// ✂️ MEDIUM-3: resolved (`Accepted`/`Discarded`) conflicts are closed historical facts, so
/// unlike `Open` ones they are prunable — oldest push order evicted first, capped, while an
/// `Open` conflict is never touched no matter how far over cap the resolved backlog grows.
#[semio_framework_async_macros::async_test]
async fn resolved_conflicts_are_pruned_oldest_first_once_the_ledger_exceeds_its_cap_while_open_ones_survive() {
    let mut store = fresh_demo_store().await;
    let cap = semio_framework_os_store_engine::lifted_store::StoreOwners::<DemoSnapshot, DemoMutation>::RESOLVED_CONFLICT_CAP;
    let open = synthetic_open_conflict(u64::MAX).await;
    store.0.log.envelope.conflicts.push(open.clone());
    let overflow: u64 = 20;
    let resolved_count = cap as u64 + overflow;
    for seed in 0..resolved_count {
        let mut resolved = synthetic_open_conflict(seed).await;
        resolved.status = semio_framework_os_spr::ConflictStatus::Accepted;
        store.0.log.envelope.conflicts.push(resolved);
    }
    assert_eq!(store.conflicts().len(), resolved_count as usize + 1);

    store.0.prune_resolved_conflicts().expect("bounded resolved-conflict retirement");

    assert_eq!(store.conflicts().len(), cap, "pruning must bring the ledger back down to the cap");
    assert!(store.conflicts().iter().any(|conflict| conflict.id == open.id && conflict.status == semio_framework_os_spr::ConflictStatus::Open), "the Open conflict must never be evicted, no matter how far over cap the resolved backlog grows");
    let surviving_seeds: HashSet<u64> = store.conflicts().iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Accepted).map(|conflict| conflict.timestamp.physical_ms).collect();
    assert_eq!(surviving_seeds.len(), cap - 1);
    for evicted_seed in 0..=overflow {
        assert!(!surviving_seeds.contains(&evicted_seed), "seed {evicted_seed} was among the oldest resolved conflicts and must be pruned first");
    }
    assert!(surviving_seeds.contains(&(resolved_count - 1)), "the newest resolved conflict must survive pruning");
}

/// 🧼️ MEDIUM-4: one `ingest_remote` batch that quarantines BOTH a brand-new edit (`op-c`, never
/// committed before) AND a previously-committed edit a rewind now retroactively invalidates
/// (`op-a`, which carried a REAL non-empty `mutation.cascade` message from its first commit)
/// — proving `replace_edit_messages(.., empty)` clears the stale ledger entry correctly even
/// when it runs in the same pass as a never-populated one, regardless of which kind an id is.
///
/// `op-b` (earlier HLC than `op-a`) forces a rewind that replays `op-a` against a DELETED
/// target the second time around. `op-c` (later HLC, brand new) is submitted FIRST but
/// depends on `op-b`, so it buffers in the dag and is released alongside `op-b` in the SAME
/// `ingest_remote` call/batch — the mixed-kind scenario MEDIUM-4 asks for.
#[semio_framework_async_macros::async_test]
async fn quarantine_message_clearing_is_correct_for_a_mixed_new_and_retroactive_batch() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);

    let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::AddN(AddN { delta: 5 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    store.ingest_remote(a).await.expect("op-a commits cleanly the first time, on a fresh n=0 target");
    assert!(!store.messages_for_edit("op-a").is_empty(), "fixture must carry real prior ledger content for this test to be meaningful");
    assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: Some(5) });

    let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::DeleteN(DeleteN {}), HybridLogicalTimestamp::new(1, 50), Vec::new());
    let c = mutation_envelope_at("actor-c", "op-c", DemoMutation::AddN(AddN { delta: 1 }), HybridLogicalTimestamp::new(2, 150), vec![MutationId("op-b".into())]);
    store.ingest_remote(c).await.expect("op-c buffers behind its unmet dependency on op-b");
    let report = store.ingest_remote(b).await.expect("op-b's arrival releases op-b and op-c into the same batch");

    assert!(!report.accepted, "op-c never committed");
    assert_eq!(store.applied_edit_ids(), &["op-b".to_string()], "only op-b committed — op-a dropped out on retroactive invalidation, op-c never entered");
    assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: None });

    assert!(store.messages_for_edit("op-a").is_empty(), "op-a's stale non-empty ledger entry must be cleared, not left stale, once it is retroactively quarantined");
    assert!(store.messages_for_edit("op-c").is_empty(), "op-c never committed, so it must never gain a ledger entry");
    assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-a"), "a cleared entry must be fully removed from the durable ledger, not left behind as an empty Vec");
    assert!(store.envelope().edit_messages.iter().all(|entry| entry.edit_id != "op-c"), "op-c must never appear in the durable ledger at all");

    let conflict_id = report.conflict.expect("the mixed quarantine batch must raise one conflict");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
    assert_eq!(conflict.status, semio_framework_os_spr::ConflictStatus::Open);
    match &conflict.kind {
        semio_framework_os_spr::ConflictKind::Quarantined { envelopes } => assert_eq!(envelopes.len(), 2, "both op-a (retroactive) and op-c (new) must be quarantined TOGETHER in one conflict"),
        other => panic!("expected a Quarantined conflict, got {other:?}"),
    }
    assert_eq!(store.conflicts().len(), 1, "op-a's clean first commit never raised a conflict of its own — only this one mixed-batch conflict must exist");
}

//#endregion 🔖️RobustnessTests

#[semio_framework_async_macros::async_test]
async fn composition_pins_rederive_checkpoint_identity_without_partial_mutation() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("checkpoint".into()), authors: Vec::new() }).await.expect("checkpoint");
    let original_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
    let before = owned_test_envelope(&store).await;
    let invalid = semio_framework_os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };

    assert!(store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![invalid]).await.is_err());
    assert_eq!(store.envelope(), &before);

    let pin = semio_framework_os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef { artifact_id: "child".into(), dialect: demo_child_dialect() }, checkpoint_id: "child-checkpoint".into() };
    store.set_checkpoint_composition_pins(&original_checkpoint_id, vec![pin]).await.expect("valid pin update");
    let rederived = &store.envelope().vcs.checkpoints[0];
    assert_ne!(rederived.id, original_checkpoint_id);
    assert_eq!(rederived.composition_pins.len(), 1);
    let mut revalidated = semio_framework_os_store_engine::lifted_store::ArtifactStore::<DemoSnapshot, DemoMutation>::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(owned_test_envelope(&store).await, store.local_actor_id().clone()).await.expect("a persisted pinned checkpoint must validate its rederived identity");
    revalidated.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    close_test_store(&mut revalidated);
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn materialize_replays_forward_mutations() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
    assert_eq!(store.envelope().vcs.edits.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(0));
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}
