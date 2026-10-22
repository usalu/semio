/// 🧾️ Journal commit, cancellation and stage error laws: the owned three member bind, the private committed record, the group journal commit, the cancellation wait, the stage error and the uncertain journal error.

#[semio_framework_async_macros::async_test]
async fn durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners() {
    let (mut parent_store, mut drawing_store, mut value_store) = owned_three_stores().await;
    let bound = bound_three(&parent_store, &drawing_store, &value_store);
    let decision = bound.decision.clone();
    assert!(decision.validate().is_ok());
    assert_eq!(decision.parent.expected_revision, parent_store.content_revision_now());
    assert_eq!(decision.drawing.expected_revision, drawing_store.content_revision_now());
    assert_eq!(decision.value.expected_revision, value_store.content_revision_now());
    assert_eq!(bound.parent.prepared.edit().mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    assert_eq!(bound.drawing.prepared.edit().mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    assert_eq!(bound.value.prepared.edit().mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    for member in [&decision.parent, &decision.drawing, &decision.value] {
        assert!(member.recovery_pack.starts_with(b"\x89SEM\r\n\x1a\n"));
        assert_eq!(member.recovery_pack_sha256, semio_framework_hash::sha256_hex(&member.recovery_pack));
    }
    let recovered = match recover_decision_store_owned(&decision, &parent_store, &drawing_store, &value_store).expect("all-base recovery verifies every bound outcome") {
        DurableOwnedThreeStoreRecoveryV1::Apply(recovered) => recovered,
        DurableOwnedThreeStoreRecoveryV1::AlreadyApplied => panic!("base stores cannot report already applied"),
    };
    for prepared in [&recovered.parent.prepared, &recovered.drawing.prepared, &recovered.value.prepared] {
        prepared.authority().validate_prepared(prepared).expect("recovery reconstructs a valid private Store seal");
        assert_eq!(prepared.edit().mutation_meta[0].group_id.as_deref(), Some(decision.decision_sha256.as_str()));
    }
    let mut tampered = decision;
    tampered.value.recovery_pack[0] ^= 1;
    assert!(recover_decision_store_owned(&tampered, &parent_store, &drawing_store, &value_store).is_err());
    drop(recovered);
    drop(bound);
    close_demo_artifact_store(&mut value_store);
    close_demo_artifact_store(&mut drawing_store);
    close_demo_artifact_store(&mut parent_store);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_private_committed_record_recovers_all_three_stores_without_reappending_journal() {
    let (parent, drawing, value) = owned_three_stores().await;
    let bound = bound_three(&parent, &drawing, &value);
    let decision = bound.decision.clone();
    let record = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("committed recovery record remains canonical");
    let receipt = DurableOwnedGroupJournalReceiptV1 { anchor_sha256: decision.anchor_sha256.clone(), decision_sha256: decision.decision_sha256.clone(), transaction_id: 71, segment_index: 5 };
    drop(bound);
    let admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let mut host = match admission.restore_untrusted_committed_record(&record, receipt.clone()).unwrap_or_else(|_| panic!("base frontiers return every owner through a retained recovery host")) {
        DurableOwnedMapRecoveryStartV1::Apply(host) => host,
        DurableOwnedMapRecoveryStartV1::AlreadyApplied(_) => panic!("base frontiers cannot report an applied decision"),
    };
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::StagingParent));
    assert!(matches!(host.advance(grant), DurableOwnedMapRecoveryAdvanceV1::Progress(_)), "committed parent stages before a recoverable drawing dependency fault");
    assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing));
    let drawing_retirement_factory = host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).and_then(|drawing| drawing.owners.snapshot_retirement_factory.take());
    assert_eq!(host.advance(grant), DurableOwnedMapRecoveryAdvanceV1::Fault(DurableOwnedGroupDecisionError::InvalidFrontier));
    assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing));
    let all_old = host.capture_snapshot().expect("pre-visibility recovery fault keeps one complete old triple visible");
    assert_eq!([all_old.parent.n, all_old.drawing.n, all_old.value.n], [Some(0), Some(0), Some(0)]);
    *host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).expect("retained recovery keeps the exact drawing Store").snapshot_retirement_factory = drawing_retirement_factory;
    let mut observed_post_visibility_retry = false;
    for _ in 0..4096 {
        if !observed_post_visibility_retry && host.phase() == Some(DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing) {
            let drawing_retirement_factory = host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).and_then(|drawing| drawing.owners.snapshot_retirement_factory.take());
            assert_eq!(host.advance(grant), DurableOwnedMapRecoveryAdvanceV1::Fault(DurableOwnedGroupDecisionError::InvalidOutcome));
            assert_eq!(host.phase(), Some(DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing));
            let all_new = host.capture_snapshot().expect("post-visibility recovery fault keeps one complete new triple visible");
            assert_eq!([all_new.parent.n, all_new.drawing.n, all_new.value.n], [Some(7), Some(11), Some(13)]);
            *host.operation.as_mut().and_then(|operation| operation.drawing.as_mut()).expect("retained recovery keeps the exact drawing Store after visibility").snapshot_retirement_factory = drawing_retirement_factory;
            observed_post_visibility_retry = true;
        }
        match host.advance(grant) {
            DurableOwnedMapRecoveryAdvanceV1::AwaitingAcknowledgement => assert!(host.acknowledge_restoration(&receipt)),
            DurableOwnedMapRecoveryAdvanceV1::Complete => break,
            DurableOwnedMapRecoveryAdvanceV1::Progress(_) | DurableOwnedMapRecoveryAdvanceV1::DiagnosticProgress(_, _) | DurableOwnedMapRecoveryAdvanceV1::Blocked => {}
            DurableOwnedMapRecoveryAdvanceV1::Fault(error) => panic!("committed recovery retained an unexpected fault: {error}"),
        }
    }
    assert!(observed_post_visibility_retry);
    let DurableOwnedMapRecoveryOwnersV1 { parent, drawing, value } = host.take_terminal_owners().expect("terminal recovery ACK returns all three exact Stores");
    assert!(host.terminal_is_empty());
    assert_eq!([parent.fold_current().await.unwrap().n, drawing.fold_current().await.unwrap().n, value.fold_current().await.unwrap().n], [Some(7), Some(11), Some(13)]);
    let replay = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("same committed decision re-admits canonically");
    let replay_admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let DurableOwnedMapRecoveryStartV1::AlreadyApplied(DurableOwnedMapRecoveryOwnersV1 { parent, mut drawing, mut value }) =
        replay_admission.restore_untrusted_committed_record(&replay, receipt.clone()).unwrap_or_else(|_| panic!("all-post replay remains idempotent"))
    else {
        panic!("all three post-frontiers must not publish twice")
    };
    let (mut unused_base_parent, base_drawing, base_value) = owned_three_stores().await;
    let mixed_record = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("mixed-frontier recovery record remains canonical");
    let mixed_admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, base_drawing, base_value);
    let DurableOwnedMapRecoveryRejectedV1 { error, mut parent, drawing: mut mixed_drawing, value: mut mixed_value } = match mixed_admission.restore_untrusted_committed_record(&mixed_record, receipt.clone()) {
        Err(rejected) => rejected,
        Ok(_) => panic!("a committed decision cannot reconstruct over mixed base and post frontiers"),
    };
    assert_eq!(error, DurableOwnedGroupDecisionError::InvalidFrontier);
    close_demo_artifact_store(&mut mixed_value);
    close_demo_artifact_store(&mut mixed_drawing);
    close_demo_artifact_store(&mut parent);
    close_demo_artifact_store(&mut unused_base_parent);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    let (parent, drawing, value) = owned_three_stores().await;
    let mut foreign_receipt = receipt;
    foreign_receipt.decision_sha256 = "00".repeat(32);
    let foreign_record = DurableOwnedGroupJournalRecordV1::admit_canonical(decision.encode_pack()).expect("foreign-receipt recovery record remains canonical");
    let foreign_admission = DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let DurableOwnedMapRecoveryRejectedV1 { error, mut parent, mut drawing, mut value } = match foreign_admission.restore_untrusted_committed_record(&foreign_record, foreign_receipt) {
        Err(rejected) => rejected,
        Ok(_) => panic!("a foreign committed receipt cannot enter Store recovery"),
    };
    assert_eq!(error, DurableOwnedGroupDecisionError::InvalidHash);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::Commit, state: Arc::clone(&state) };
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::StartingJournal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("fixed-three staging turn");
        let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("pending partial staging reads all-old");
        assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
    }
    let decision_bytes = coordinator.decision_pack.as_ref().expect("starting journal retains decision bytes").len();
    let one_byte = copy_grant(1);
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, one_byte).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
    assert_eq!(state.lock().unwrap().begins, 0);
    let insufficient = copy_grant(decision_bytes - 1);
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, insufficient).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Blocked);
    assert_eq!(state.lock().unwrap().begins, 0);
    let exact = copy_grant(decision_bytes);
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, exact).unwrap(), DurableOwnedThreeStoreCommitAdvanceV1::Progress(DurableOwnedThreeStoreCommitPhaseV1::Journal));
    assert_eq!(state.lock().unwrap().begins, 1);
    let mut observed_pending = false;
    let mut observed_committed = false;
    let mut rejected_partial_committed_root = false;
    let mut rejected_foreign_visibility = false;
    let mut rejected_structurally_false_adoption = false;
    for _ in 0..4096 {
        let step = coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("retained group commit turn");
        let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("one captured group read");
        if coordinator.phase() == DurableOwnedThreeStoreCommitPhaseV1::Journal {
            assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
            assert_eq!(parent.local_actor_id().0.as_str(), fixture()["openedActor"].as_str().expect("declared opened group actor"));
            assert!(drawing.invalidate_after_replay().is_err());
            observed_pending = true;
        }
        if matches!(
            coordinator.phase(),
            DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingDrawingLease
                | DurableOwnedThreeStoreCommitPhaseV1::PublishingValueLease
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingParent
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::AdoptingValue
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingParent
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingDrawing
                | DurableOwnedThreeStoreCommitPhaseV1::ClearingValue
                | DurableOwnedThreeStoreCommitPhaseV1::AwaitingAck
        ) {
            assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(7), Some(11), Some(13)]);
            if !rejected_partial_committed_root {
                let retained_value_root = value.wire.durable_group_root.take().expect("committed value root");
                assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                *value.wire.durable_group_root = Some(retained_value_root);
                rejected_partial_committed_root = true;
            }
            if !rejected_foreign_visibility {
                let foreign_owner = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
                let original_visibility = std::mem::replace(&mut value.wire.durable_group_root.as_mut().expect("committed value root").visibility, foreign_owner.view());
                assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                value.wire.durable_group_root.as_mut().expect("committed value root").visibility = original_visibility;
                drop(foreign_owner);
                rejected_foreign_visibility = true;
            }
            if !rejected_structurally_false_adoption && coordinator.phase() == DurableOwnedThreeStoreCommitPhaseV1::PublishingParentLease {
                parent.wire.durable_group_root.as_mut().expect("committed parent root").adopted = true;
                assert!(capture_store_owned_three_snapshot(&parent, &drawing, &value).is_err());
                parent.wire.durable_group_root.as_mut().expect("committed parent root").adopted = false;
                rejected_structurally_false_adoption = true;
            }
            assert_erased_snapshot_authority(&mut parent, 7).await;
            assert_erased_snapshot_authority(&mut drawing, 11).await;
            assert_erased_snapshot_authority(&mut value, 13).await;
            observed_committed = true;
        }
        if let DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt) = step {
            assert!(coordinator.acknowledge(&receipt));
        }
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(observed_pending && observed_committed && rejected_partial_committed_root && rejected_foreign_visibility && rejected_structurally_false_adoption && coordinator.terminal_is_empty());
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [1, 1, 1]);
    assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [1, 1, 1]);
    let journal = state.lock().unwrap();
    assert_eq!((journal.begins, journal.advances), (1, 2));
    assert_eq!(journal.close_count, 1);
    assert!(!journal.cancelled && !journal.decision_pack.is_empty());
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::AbsentAfterCancel, state: Arc::clone(&state) };
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
    }
    assert!(coordinator.cancel());
    for _ in 0..4096 {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("cancel resolution turn");
        let read = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("cancelled captured group read");
        assert_eq!([read.parent.n, read.drawing.n, read.value.n], [Some(0), Some(0), Some(0)]);
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
    assert_eq!([parent.applied_edit_ids().len(), drawing.applied_edit_ids().len(), value.applied_edit_ids().len()], [0, 0, 0]);
    let journal = state.lock().unwrap();
    assert!(journal.cancelled);
    assert_eq!(journal.close_count, 1);
    assert_eq!(journal.advances, 2);
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::Commit, state: Arc::clone(&state) };
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("parent stages before injected drawing failure");
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::StagingDrawing);
        assert!(parent.durable_group_root.as_ref().unwrap().retirement.as_ref().unwrap().planning_output.is_some(),"staged candidate retains its original encoded packet for controlled retirement");
let drawing_retirement_factory = drawing.owners.snapshot_retirement_factory.take();
    assert_eq!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant), Err(DurableOwnedGroupDecisionError::InvalidFrontier));
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::AbortingValue);
    *drawing.owners.snapshot_retirement_factory = drawing_retirement_factory;
    let value_addresses=coordinator.value.as_ref().map(|outcome|[outcome.prepared.edit()as*const _ as usize,Arc::as_ptr(outcome.prepared.post_snapshot())as usize,outcome.recovery_pack.as_ptr()as usize]).unwrap();
    let(denied,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||coordinator.advance(&mut parent,&mut drawing,&mut value,&mut sink,copy_grant(0)).unwrap());assert!(matches!(denied,DurableOwnedThreeStoreCommitAdvanceV1::Blocked));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(coordinator.value.as_ref().map(|outcome|[outcome.prepared.edit()as*const _ as usize,Arc::as_ptr(outcome.prepared.post_snapshot())as usize,outcome.recovery_pack.as_ptr()as usize]).unwrap(),value_addresses);
    let(receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||coordinator.advance(&mut parent,&mut drawing,&mut value,&mut sink,grant).unwrap());let DurableOwnedThreeStoreCommitAdvanceV1::DiagnosticProgress(_,receipt)=receipt else{panic!("unstaged original transfer must report actual ownership work");};assert!(receipt.fits(grant.retained_grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
    for _ in 0..4096 {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("retained stage-error owner continues abort cleanup");
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    assert!(parent.wire.durable_group_root.is_none() && drawing.wire.durable_group_root.is_none() && value.wire.durable_group_root.is_none());
    assert_eq!([parent.generation(), drawing.generation(), value.generation()], [0, 0, 0]);
    let journal = state.lock().unwrap();
    assert_eq!((journal.begins, journal.close_count), (0, 0));
    drop(journal);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}

#[semio_framework_async_macros::async_test]
async fn durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change() {
    let (mut parent, mut drawing, mut value) = owned_three_stores().await;
    let mut coordinator = bound_three(&parent, &drawing, &value).begin_retained_commit().expect("Store creates one retained group coordinator");
    let state = Arc::new(std::sync::Mutex::new(FakeJournalState::default()));
    let mut sink = FakeJournalSink { resolution: FakeJournalResolution::ErrorThenCommit, state: Arc::clone(&state) };
    let grant = copy_grant(DURABLE_OWNED_GROUP_EVENT_MAX_BYTES);
    while coordinator.phase() != DurableOwnedThreeStoreCommitPhaseV1::Journal {
        coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("pre-journal stage");
    }
    coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("journal first pending turn");
    assert!(coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).is_err());
    assert_eq!(coordinator.phase(), DurableOwnedThreeStoreCommitPhaseV1::Journal);
    let pending = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("uncertain journal read remains coherent");
    assert_eq!([pending.parent.n, pending.drawing.n, pending.value.n], [Some(0), Some(0), Some(0)]);
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances), (1, 2));
    }
    for _ in 0..4096 {
        let step = coordinator.advance(&mut parent, &mut drawing, &mut value, &mut sink, grant).expect("same journal owner retries");
        if let DurableOwnedThreeStoreCommitAdvanceV1::AwaitingAck(receipt) = step {
            assert!(coordinator.acknowledge(&receipt));
        }
        if coordinator.terminal_is_empty() {
            break;
        }
    }
    assert!(coordinator.terminal_is_empty());
    {
        let journal = state.lock().unwrap();
        assert_eq!((journal.begins, journal.advances), (1, 3));
    }
    let committed = capture_store_owned_three_snapshot(&parent, &drawing, &value).expect("retried journal commit read");
    assert_eq!([committed.parent.n, committed.drawing.n, committed.value.n], [Some(7), Some(11), Some(13)]);
    close_demo_artifact_store(&mut value);
    close_demo_artifact_store(&mut drawing);
    close_demo_artifact_store(&mut parent);
}
