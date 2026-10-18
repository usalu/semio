/// ⚔️ Modify versus delete, quarantine and testkit law wiring: the modify versus delete resolutions, the chronological determinism, the quarantine accept and discard, the ledger replay and the testkit laws over the real store.

/// 🛰️ A `DeleteN` at an earlier HLC and a `SetN` at a later HLC — replayed in HLC order, the
/// `SetN` lands on an already-deleted target and raises `mutation.target-missing` (Error).
async fn modify_vs_delete_envelopes() -> (semio_framework_os_spr::MutationEnvelope, semio_framework_os_spr::MutationEnvelope) {
    let delete = mutation_envelope_at("deleter", "op-delete", DemoMutation::DeleteN(DeleteN {}), HybridLogicalTimestamp::new(1, 100), Vec::new());
    let modify = mutation_envelope_at("modifier", "op-modify", DemoMutation::SetN(SetN { n: 42 }), HybridLogicalTimestamp::new(2, 200), Vec::new());
    (delete, modify)
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_quarantines_under_normal_and_vigilant() {
    for policy in [semio_framework_os_spr::MergePolicy::Normal, semio_framework_os_spr::MergePolicy::Vigilant] {
        let mut store = fresh_demo_store().await;
        store.set_merge_policy(policy);
        let (delete, modify) = modify_vs_delete_envelopes().await;
        store.ingest_remote(delete).await.expect("the delete alone raises no message and always applies");
        let pre_merge = store.snapshot().expect("pre-merge snapshot");
        let pre_merge_ids = store.applied_edit_ids().to_vec();
        let report = store.ingest_remote(modify).await.expect("a policy rejection is a MergeReport, not an Err");
        assert!(!report.accepted, "{policy:?} must reject an Error-level modify-vs-delete conflict");
        assert_eq!(report.worst, Some(semio_framework_diagnostic::Severity::Error));
        assert_eq!(store.snapshot().expect("unchanged snapshot"), pre_merge, "{policy:?}: state stays pre-merge on reject");
        assert_eq!(store.applied_edit_ids(), pre_merge_ids.as_slice(), "{policy:?}: applied_edit_ids stays pre-merge on reject");
        let conflict_id = report.conflict.clone().expect("a reject must raise a conflict");
        let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded on the store");
        assert_eq!(conflict.status, semio_framework_os_spr::ConflictStatus::Open);
        assert!(matches!(conflict.kind, semio_framework_os_spr::ConflictKind::Quarantined { .. }), "{policy:?}: a rejected batch quarantines, it never degrades");
        assert!(store.open_conflicts().any(|conflict| conflict.id == conflict_id));
    }
}

#[semio_framework_async_macros::async_test]
async fn modify_vs_delete_applies_under_laissez_faire_with_a_degraded_conflict() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let report = store.ingest_remote(modify).await.expect("LaissezFaire only rejects Fatal");
    assert!(report.accepted);
    assert_eq!(report.worst, Some(semio_framework_diagnostic::Severity::Error));
    assert_eq!(store.snapshot().expect("snapshot").n, None, "the modify's part is absent — LAW 2 (an Error message ⇒ no change to its target)");
    assert!(report.replayed.iter().any(|edit_messages| edit_messages.messages.iter().any(|message| message.code.0 == "mutation.target-missing")), "an Error message must be reported");
    let conflict_id = report.conflict.expect("worst >= Warning must raise a Degraded conflict");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded");
    assert!(matches!(conflict.kind, semio_framework_os_spr::ConflictKind::Degraded { .. }));
    assert_eq!(conflict.status, semio_framework_os_spr::ConflictStatus::Open);
}

#[semio_framework_async_macros::async_test]
async fn chronological_determinism_any_arrival_order_converges() {
    let a = mutation_envelope_at("actor-a", "op-a", DemoMutation::SetN(SetN { n: 10 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    let b = mutation_envelope_at("actor-b", "op-b", DemoMutation::SetN(SetN { n: 20 }), HybridLogicalTimestamp::new(2, 300), Vec::new());

    let mut forward = fresh_demo_store().await;
    forward.ingest_remote(a.clone()).await.expect("a");
    forward.ingest_remote(b.clone()).await.expect("b");

    let mut reversed = fresh_demo_store().await;
    reversed.ingest_remote(b).await.expect("b");
    reversed.ingest_remote(a).await.expect("a");

    assert_eq!(forward.snapshot().expect("snapshot"), reversed.snapshot().expect("snapshot"));
    assert_eq!(forward.applied_edit_ids(), reversed.applied_edit_ids(), "both must land in the same HLC order regardless of arrival order");
    assert_eq!(forward.applied_edit_ids(), &["op-a".to_string(), "op-b".to_string()]);
    assert_eq!(forward.conflicts().len(), reversed.conflicts().len());
    assert!(forward.conflicts().is_empty(), "two non-conflicting SetN edits raise no conflict");
}

#[semio_framework_async_macros::async_test]
async fn empty_store_replays_a_remote_event_log_in_hlc_order_and_keeps_its_local_policy() {
    let mut remote = fresh_demo_store().await;
    let later = mutation_envelope_at("later-peer", "later-op", DemoMutation::SetN(SetN { n: 20 }), HybridLogicalTimestamp::new(2, 300), Vec::new());
    let earlier = mutation_envelope_at("earlier-peer", "earlier-op", DemoMutation::SetN(SetN { n: 10 }), HybridLogicalTimestamp::new(1, 100), Vec::new());
    remote.ingest_remote(later).await.expect("remote receives later edit first");
    remote.ingest_remote(earlier).await.expect("remote receives earlier edit second");

    let mut local = fresh_demo_store().await;
    local.set_merge_policy(semio_framework_os_spr::MergePolicy::Vigilant);
    for event in remote.event_log().expect("remote event log") {
        local.ingest_remote(event).await.expect("local replays the remote events");
    }

    assert_eq!(local.0.merge_policy(), semio_framework_os_spr::MergePolicy::Vigilant, "the receiving store's local-only policy is never carried by events");
    assert_eq!(local.snapshot().expect("replayed snapshot"), DemoSnapshot { n: Some(20) });
    assert_eq!(local.applied_edit_ids(), &["earlier-op".to_string(), "later-op".to_string()], "replay orders events by HLC rather than by log or arrival order");
    assert_eq!(local.conflicts(), remote.conflicts());
}

#[semio_framework_async_macros::async_test]
async fn a_backbone_genesis_of_another_document_is_refused_without_mutation() {
    let foreign = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(5) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let (channel, remote) = ChannelBackbone::pair("genesis").await;
    remote.push(BackboneMessage::Genesis { pack: foreign.envelope().vcs.genesis.pack().to_vec() }).await.expect("push foreign genesis");
    let mut target = fresh_demo_store().await;
    let error = target.attach_backbone(Backbones::Channel(channel)).await.expect_err("a different genesis names a different document");
    assert!(matches!(error, VcsError::ValidationFailed(message) if message.contains("different initial snapshot")));
    assert_eq!(target.snapshot().expect("unchanged"), DemoSnapshot { n: Some(0) });

    let (channel, remote) = ChannelBackbone::pair("genesis-shared").await;
    remote.push(BackboneMessage::Genesis { pack: DemoSnapshot { n: Some(0) }.encode_pack() }).await.expect("push shared genesis");
    let mut peer = fresh_demo_store().await;
    peer.attach_backbone(Backbones::Channel(channel)).await.expect("a shared genesis is the same document");
}

#[semio_framework_async_macros::async_test]
async fn quarantine_accept_equals_laissez_faire_result() {
    let mut quarantined = fresh_demo_store().await;
    quarantined.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    quarantined.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    let reject_report = quarantined.ingest_remote(modify.clone()).await.expect("reject is a report");
    assert!(!reject_report.accepted);
    let conflict_id = reject_report.conflict.expect("conflict raised");
    quarantined.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Accept).await.expect("accept");

    let mut laissez_faire = fresh_demo_store().await;
    laissez_faire.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    laissez_faire.ingest_remote(delete).await.expect("delete applies cleanly");
    laissez_faire.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    assert_eq!(quarantined.snapshot().expect("snapshot"), laissez_faire.snapshot().expect("snapshot"));
    assert_eq!(quarantined.applied_edit_ids(), laissez_faire.applied_edit_ids());
    assert_eq!(quarantined.conflicts().iter().filter(|conflict| conflict.status == semio_framework_os_spr::ConflictStatus::Open).count(), 0, "no Open conflict remains — accept must not raise a second conflict");
    assert_eq!(quarantined.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict kept").status, semio_framework_os_spr::ConflictStatus::Accepted);
}

#[semio_framework_async_macros::async_test]
async fn quarantine_discard_preserves_state() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_discard = store.snapshot().expect("pre-discard snapshot");
    let pre_discard_ids = store.applied_edit_ids().to_vec();
    let reject_report = store.ingest_remote(modify).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    store.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Discard).await.expect("discard");
    assert_eq!(store.snapshot().expect("snapshot"), pre_discard, "a discarded batch must never be applied");
    assert_eq!(store.applied_edit_ids(), pre_discard_ids.as_slice());
    assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict kept").status, semio_framework_os_spr::ConflictStatus::Discarded);
}

#[semio_framework_async_macros::async_test]
async fn ledger_matches_a_fresh_replay_of_the_same_envelopes() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let modify_edit_id = modify.mutation_id.0.clone();

    let mut first = fresh_demo_store().await;
    first.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    first.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    first.ingest_remote(modify.clone()).await.expect("modify applies under LaissezFaire");

    let mut replay = fresh_demo_store().await;
    replay.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    replay.ingest_remote(delete).await.expect("delete applies cleanly");
    replay.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    assert_eq!(first.messages_for_edit(&modify_edit_id), replay.messages_for_edit(&modify_edit_id));
    assert!(!first.messages_for_edit(&modify_edit_id).is_empty(), "the modify edit must have raised a message");
    assert_eq!(first.snapshot().expect("snapshot"), replay.snapshot().expect("snapshot"));
}

/// ⏱️ Local edits get large physical-ms HLCs (the local clock ticks off the real wall clock).
///
/// A remote edit stamped with a tiny HLC — guaranteed to sort before both local edits.
#[semio_framework_async_macros::async_test]
async fn applied_edit_ids_stay_sorted_by_hlc_after_a_backdated_remote_insert() {
    let mut store = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("local apply 1");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("local apply 2");
    let local_ids = store.applied_edit_ids().to_vec();

    let backdated = mutation_envelope_at("backdated-actor", "op-backdated", DemoMutation::SetN(SetN { n: 99 }), HybridLogicalTimestamp::new(9, 1), Vec::new());
    store.ingest_remote(backdated).await.expect("backdated insert");

    assert_eq!(store.applied_edit_ids()[0], "op-backdated", "the backdated edit must sort before both local edits");
    assert!(store.applied_edit_ids().iter().skip(1).eq(local_ids.iter()));

    let envelope = store.envelope();
    let hlcs: Vec<HybridLogicalTimestamp> = store.applied_edit_ids().iter().map(|id| envelope.vcs.edits.iter().find(|edit| edit.id == *id).and_then(|edit| edit.mutation_meta.first()).map(|meta| meta.timestamp).expect("meta")).collect();
    let mut cmp_keys = Vec::with_capacity(hlcs.len());
    for hlc in &hlcs {
        cmp_keys.push(hlc.cmp_key());
    }
    assert!(cmp_keys.windows(2).all(|pair| pair[0] <= pair[1]), "applied_edit_ids must stay HLC-sorted: {hlcs:?}");
}

//#region 🔖️TestkitLawWiring
// 🎯️ G2 verification barrier (26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
// CONFLICTS) — the tests above assert the same scenarios by hand; these route the SAME
// scenarios through the frozen `📡️spr/🧫️fixtures` `🔖️Merge`/`🔖️Conflict` law helpers so the
// laws are proven to hold against this store's real `ingest_remote`/`resolve_conflict`/
// `messages_for_edit`/`.spr` codec, not only against testkit's own synthetic self-tests.

#[semio_framework_async_macros::async_test]
async fn testkit_law_modify_vs_delete_holds_under_normal_and_vigilant() {
    for policy in [semio_framework_os_spr::MergePolicy::Normal, semio_framework_os_spr::MergePolicy::Vigilant] {
        let mut store = fresh_demo_store().await;
        store.set_merge_policy(policy);
        let (delete, modify) = modify_vs_delete_envelopes().await;
        store.ingest_remote(delete).await.expect("delete applies cleanly");
        let pre_merge = store.snapshot().expect("pre-merge snapshot");
        let report = store.ingest_remote(modify).await.expect("a policy rejection is a MergeReport, not an Err");
        let post_merge = store.snapshot().expect("post-merge snapshot");
        semio_framework_os_spr::protocol_laws::assert_modify_vs_delete(policy, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n.is_some()).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_modify_vs_delete_holds_under_laissez_faire() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_merge = store.snapshot().expect("pre-merge snapshot");
    let report = store.ingest_remote(modify).await.expect("LaissezFaire only rejects Fatal");
    let post_merge = store.snapshot().expect("post-merge snapshot");
    semio_framework_os_spr::protocol_laws::assert_modify_vs_delete(semio_framework_os_spr::MergePolicy::LaissezFaire, &pre_merge, &post_merge, &report, store.conflicts(), |snapshot: &DemoSnapshot| snapshot.n.is_some()).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_chronological_determinism_holds_for_a_real_modify_vs_delete_batch() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let envelopes = [delete, modify];
    semio_framework_os_spr::protocol_laws::assert_chronological_determinism(envelopes.len(), 7, 6, async |order| {
        let mut store = fresh_demo_store().await;
        for &index in order {
            store.ingest_remote(envelopes[index].clone()).await.expect("real store ingest must not hard-error even when the batch ends up quarantined");
        }
        let snapshot = store.snapshot().expect("snapshot");
        let applied = store.applied_edit_ids().to_vec();
        let conflict_ids: Vec<semio_framework_os_spr::ConflictId> = store.conflicts().iter().map(|conflict| conflict.id.clone()).collect();
        (snapshot, applied, conflict_ids)
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_quarantine_accept_equals_laissez_faire_via_real_store() {
    let mut quarantined = fresh_demo_store().await;
    quarantined.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    quarantined.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    let reject_report = quarantined.ingest_remote(modify.clone()).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    quarantined.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Accept).await.expect("accept");

    let mut laissez_faire = fresh_demo_store().await;
    laissez_faire.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    laissez_faire.ingest_remote(delete).await.expect("delete applies cleanly");
    laissez_faire.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    let accepted_state = quarantined.snapshot().expect("accepted snapshot");
    let laissez_faire_state = laissez_faire.snapshot().expect("laissez-faire snapshot");
    semio_framework_os_spr::protocol_laws::assert_quarantine_accept_equals_laissez_faire(&accepted_state, &laissez_faire_state).await;
}

/// 🗃️ `relayed`: every edit id this store's persisted history (`applied_edit_ids`) could ever
/// ship onward via `flush_outbound`/`snapshot_pack` — a discarded batch is only `seed_
/// applied` on the dag, never added to `applied_edit_ids`/`vcs.edits`, so it can never appear
/// here; this is the real set flush_outbound draws from, not a fabricated stand-in.
#[semio_framework_async_macros::async_test]
async fn testkit_law_quarantine_discard_preserves_state_via_real_store() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let pre_discard = store.snapshot().expect("pre-discard snapshot");
    let reject_report = store.ingest_remote(modify.clone()).await.expect("reject is a report");
    let conflict_id = reject_report.conflict.expect("conflict raised");
    store.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Discard).await.expect("discard");
    let post_discard = store.snapshot().expect("post-discard snapshot");
    let relayed = store.applied_edit_ids().to_vec();
    semio_framework_os_spr::protocol_laws::assert_quarantine_discard_preserves_state(&pre_discard, &post_discard, std::slice::from_ref(&modify.mutation_id.0), &relayed).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_ledger_matches_replay_via_real_store() {
    let (delete, modify) = modify_vs_delete_envelopes().await;
    let modify_edit_id = modify.mutation_id.0.clone();

    let mut first = fresh_demo_store().await;
    first.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    first.ingest_remote(delete.clone()).await.expect("delete applies cleanly");
    first.ingest_remote(modify.clone()).await.expect("modify applies under LaissezFaire");

    let mut replay = fresh_demo_store().await;
    replay.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    replay.ingest_remote(delete).await.expect("delete applies cleanly");
    replay.ingest_remote(modify).await.expect("modify applies under LaissezFaire");

    let mut ledger = HashMap::new();
    ledger.insert(modify_edit_id.clone(), first.messages_for_edit(&modify_edit_id).to_vec());
    let mut replayed = HashMap::new();
    replayed.insert(modify_edit_id.clone(), replay.messages_for_edit(&modify_edit_id).to_vec());
    assert!(!ledger[&modify_edit_id].is_empty(), "the modify edit must have raised a message for this law to be meaningful");

    semio_framework_os_spr::protocol_laws::assert_ledger_matches_replay(&ledger, &replayed).await;
}

#[semio_framework_async_macros::async_test]
async fn testkit_law_conflict_spr_round_trip_via_real_store() {
    let mut store = fresh_demo_store().await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::Normal);
    let (delete, modify) = modify_vs_delete_envelopes().await;
    store.ingest_remote(delete).await.expect("delete applies cleanly");
    let report = store.ingest_remote(modify).await.expect("reject is a report");
    let conflict_id = report.conflict.expect("conflict raised");
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("conflict recorded").clone();

    let pack_bytes = DemoSnapshot { n: Some(0) }.encode_pack();
    let encode = async |conflict: &semio_framework_os_spr::Conflict| -> Vec<u8> {
        let mut envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        envelope.conflicts = vec![conflict.clone()];
        let bytes = print_document_spr(&envelope).await.expect("encode conflict via the real .spr codec");
        retire_demo_envelope(envelope);
        bytes
    };
    let decode = async |bytes: &[u8]| -> semio_framework_os_spr::Conflict {
        let parsed = parse_document_spr::<DemoSnapshot, DemoMutation>(&pack_bytes, bytes, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("decode conflict via the real .spr codec");
        let conflict = parsed.envelope.conflicts.first().cloned().expect("one conflict round-tripped");
        retire_demo_envelope(parsed.envelope);
        conflict
    };
    semio_framework_os_spr::protocol_laws::assert_conflict_spr_round_trip(&conflict, encode, decode).await;
}

//#endregion 🔖️TestkitLawWiring
