/// 🔭️ Envelope close, field registry and snapshot read laws: the interactive envelope close, the field decoder registry, the snapshot read double return, the snapshot roots, the rejected history mutation and the member owned batch admission.

#[test]
fn interactive_envelope_close_reclaims_every_owner_through_the_job_protocol() {
    use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep};

    let mut caller=OriginalEnvelopeCaller::admit_cleanup("interactive-envelope");
    let mut cx=caller.context();
    let registry = ArtifactEnvelopeFieldDecoderRegistry::new();
    let mut job = ArtifactEnvelopeDecodeAuthority::<(), ()>::try_new(
        artifact_envelope_decode_test_cursor(&[br#"{"schema":"s","id":"i","vcs":{},"editMessages":[],"conflicts":[]}"#]),
        &registry,
        Box::new(TestEnvelopeFieldDecoder { terminal: false, accepted: 0 }),
    )
    .unwrap_or_else(|_| panic!("fixed close-protocol decoder admission"));
    let grant=envelope_field_close_grant();
    job.begin_close();
    let mut detached = None;
    for _ in 0..256 {
        drive_test_envelope_field_return(&registry, &mut detached);
        let before=cx.retained_progress();
        let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||cx.with_retained_loan(grant,|cx|job.close_step(cx)));
        if let Ok(step)=&result{assert_eq!(cx.retained_progress(),before.checked_add(step.progress()).unwrap());assert_eq!((born,freed),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
        match result {
            Ok(InteractiveJobCloseStep::Pending { progress }) => {
                assert!(progress.fits(envelope_field_close_grant()));
            }
            Ok(InteractiveJobCloseStep::Blocked) => {}
            Err(error) => panic!("interactive envelope close refused: {error:?}"),
            Ok(InteractiveJobCloseStep::Complete { .. }) => {
                assert!(job.terminal_is_empty());
                assert!(registry.terminal_is_empty());
                drop(cx);
                caller.retire(grant);
                return;
            }
        }
    }
    panic!("interactive envelope close did not reclaim every exact owner");
}

#[test]
fn envelope_field_registry_retains_late_return_and_generation_reuse_until_bounded_app_reclaim() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::<(), ()>::new();
    let first = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("first exact decoder lease"));
    let first_ticket = first.ticket();
    drop(first);
    assert_eq!(registry.next_returned_ticket(), Some(first_ticket));
    assert!(!registry.ticket_reclaimed(first_ticket));
    let mut detached = None;
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(first_ticket));
    assert!(registry.terminal_is_empty(), "the registry is empty only after exact ownership transferred to the app close pump");
    assert!(detached.is_some(), "the detached owner remains retained outside the registry until its terminal step");
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(detached.is_none());
    assert!(registry.terminal_is_empty());

    let second = registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("reused exact decoder lease"));
    let second_ticket = second.ticket();
    assert_eq!(second_ticket.index(), first_ticket.index());
    assert_ne!(second_ticket.generation(), first_ticket.generation());
    assert!(!registry.ticket_reclaimed(second_ticket), "an old reclaimed generation cannot certify a reused slot");
    drop(second);
    drive_test_envelope_field_return(&registry, &mut detached);
    drive_test_envelope_field_return(&registry, &mut detached);
    assert!(registry.ticket_reclaimed(second_ticket));
    assert!(registry.terminal_is_empty());
}

#[test]
fn envelope_field_registry_capacity_and_contention_return_the_exact_decoder_before_adoption() {
    let registry = ArtifactEnvelopeFieldDecoderRegistry::<(), ()>::new();
    let mut leases = Vec::with_capacity(ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY);
    for _ in 0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY {
        leases.push(registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })).unwrap_or_else(|_| panic!("fixed decoder capacity admits its exact maximum")));
    }
    let (fault, rejected) = match registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })) {
        Ok(_) => panic!("fixed decoder capacity plus one must fail closed"),
        Err(rejected) => rejected,
    };
    assert_eq!(fault, ArtifactEnvelopeFieldDecoderRegistryFault::Capacity);
    assert!(rejected.terminal_is_empty());
    drop(rejected);

    while let Some(lease) = leases.pop() {
        drop(lease);
        let mut detached = None;
        drive_test_envelope_field_return(&registry, &mut detached);
        drive_test_envelope_field_return(&registry, &mut detached);
        assert!(detached.is_none());
    }
    assert!(registry.terminal_is_empty());

    let state = registry.state.lock().expect("test owns exact registry contention guard");
    let (fault, rejected) = match registry.try_admit(Box::new(TestEnvelopeFieldDecoder { terminal: true, accepted: 0 })) {
        Ok(_) => panic!("contended decoder admission must fail closed"),
        Err(rejected) => rejected,
    };
    assert_eq!(fault, ArtifactEnvelopeFieldDecoderRegistryFault::Contended);
    assert!(rejected.terminal_is_empty());
    drop(rejected);
    drop(state);
    assert!(registry.terminal_is_empty());
}

/// 🎟️ A lease is returned EXACTLY ONCE, on either of the registry's two return paths, and the
/// path a return takes is decided by whether its root is still aliased outside the registry.
///
/// An aliased root has nothing for the retirement pump to dispose, so `try_release_aliased`
/// reclaims its slot on the returning thread and the pump never sees it. A sole-owner root is the
/// last `Arc` in the process, so it is counted into `returned` and handed to the pump, which
/// retires its domain tree in bounded steps. Both paths must refuse a second return of the same
/// generation: a lease that answered twice once leaked a `returned` credit against a slot that had
/// already been freed and reissued.
#[test]
fn snapshot_read_double_return_is_counted_once_and_reclaimed_once() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let aliased_root = Arc::new(11u32);
    let mut aliased = registry.try_issue(aliased_root.clone()).expect("exact aliased lease");
    assert!(aliased.return_now());
    assert!(!aliased.return_now(), "the same generation cannot be returned twice on the aliased path");
    assert_eq!(registry.returned_count(), 0, "an aliased root is reclaimed on the returning thread, not charged to the pump");
    assert_eq!(Arc::strong_count(&aliased_root), 1, "the aliased slot released its own owner alias in the same call");
    assert!(registry.try_admit_one_returned::<u32,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed return probe").is_none(), "the pump has nothing to reclaim for an aliased return");
    drop(aliased);
    drop(aliased_root);
    assert!(registry.terminal_is_empty());

    let mut lease = registry.try_issue(Arc::new(11u32)).expect("exact sole-owner lease");
    assert!(lease.return_now());
    assert!(!lease.return_now(), "the same generation cannot be returned twice on the pump path");
    assert_eq!(registry.returned_count(), 1);
    let mut reclaimed = None;
    for _ in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        if let Some(owner) = registry.try_admit_one_returned::<u32,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed return probe") {
            reclaimed = Some(owner);
            break;
        }
    }
    assert_eq!(reclaimed.as_deref(), Some(&11));
    assert_eq!(registry.returned_count(), 0);
    assert!(registry.try_admit_one_returned::<u32,_>(physical_test_close_grant(),|original,_|Ok((original,RetainedCloneProgress{copied_items:1,..Default::default()}))).map(|(original,progress)|{assert!(progress.fits(physical_test_close_grant()));original}).expect("one fixed return probe").is_none(), "a reclaimed generation is never handed out a second time");
    drop(reclaimed);
    drop(lease);
    assert!(registry.terminal_is_empty());
}

#[test]
fn stale_snapshot_read_generation_cannot_aba_remove_a_reused_slot() {
    let registry = semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();
    let owner = Arc::new(1u16);
    let first = registry.try_issue(owner.clone()).expect("first exact lease");
    let first_read = ErasedSnapshotRead::new(owner, first);
    let (_, stale_index, stale_generation) = first_read.lease_identity().unwrap();
    drop(match first_read.into_typed::<u16>(&registry) {
        Ok(owner) => owner,
        Err(_) => panic!("first exact lease retires"),
    });

    let mut reads = Vec::with_capacity(SNAPSHOT_READ_LEASE_CAPACITY);
    let mut reused_generation = None;
    for value in 0..SNAPSHOT_READ_LEASE_CAPACITY {
        let owner = Arc::new(value as u16);
        let lease = registry.try_issue(owner.clone()).expect("fixed ring re-admits every slot");
        let read = ErasedSnapshotRead::new(owner, lease);
        let (_, index, generation) = read.lease_identity().unwrap();
        if index == stale_index {
            reused_generation = Some(generation);
        }
        reads.push(read);
    }
    assert!(reused_generation.is_some_and(|generation| generation != stale_generation));
    assert!(registry.try_take(stale_index, stale_generation).is_err(), "stale generation cannot remove the reused exact slot");
    for read in reads {
        drop(match read.into_typed::<u16>(&registry) {
            Ok(owner) => owner,
            Err(_) => panic!("every reused exact lease retires"),
        });
    }
    assert!(registry.terminal_is_empty());
}


/// 🧹️ Drives any space member's owned close to its terminal-empty witness on its own exact quoted per-axis demands.
fn drive_member_close<M: SpaceMember>(member: &mut M) {
    const CEILING: usize = 4096;
    let mut last = (semio_framework_value::RetirementDemand::default(), RetainedCloneProgress::default());
    for _ in 0..65_536 {
        if member.close_owned_terminal_is_empty() {
            return;
        }
        let demand = member.close_owned_demands(CEILING).expect("current independent member close frontier");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(CEILING), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = member.close_owned_step(grant).expect("member closes under its exact quoted owner grant");
        assert!(step.progress().fits(grant) && step.progress().copied_items <= 1);
        last = (demand, step.progress());
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(member.close_owned_terminal_is_empty());
            return;
        }
    }
    panic!("member retirement did not reach its terminal-empty witness; last frontier {:?} receipt {:?}", last.0, last.1);
}


#[semio_framework_async_macros::async_test]
async fn artifact_store_snapshot_roots_and_final_envelope_transfer_in_exact_close_order() {
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "root-close", DemoSnapshot { n: Some(7) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid empty-history store");
    store.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    *store.live.tail_undo_cache = Some(("tail-owner".repeat(64), Arc::clone(&*store.live.current)));
    let original = store.live.tail_undo_cache.as_ref().unwrap().0.as_ptr();
    let phase = ArtifactStoreCursorDisposerPhase::RuntimeOwners(5);
    let demand = store.close_cursor_demands(phase, 7).unwrap();
    let grant = RetainedCloneGrant { maximum_copy_bytes: 7, maximum_release_bytes: 0, ..physical_test_close_grant() };
    let (denied, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(phase, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }));
    assert!(denied.is_err());
    assert_eq!((requested, released), (0, 0));
    assert_eq!(store.live.tail_undo_cache.as_ref().unwrap().0.as_ptr(), original);
    let (admitted, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(phase, grant));
    let (tail_id, receipt) = admitted.unwrap();
    assert_eq!((requested, released), (receipt.retained_capacity_bytes, receipt.released_bytes));
    assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
    drive_retirement_terminal(tail_id.expect("original tail id retires before snapshot"));
    assert_eq!(store.live.tail_undo_cache.as_ref().unwrap().0.capacity(), 0);
    let (tail, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(ArtifactStoreCursorDisposerPhase::TailSnapshot, grant));
    assert!(tail.unwrap().0.is_none());
    assert_eq!((requested, released), (0, 0));
    for phase in [ArtifactStoreCursorDisposerPhase::CurrentSnapshot, ArtifactStoreCursorDisposerPhase::Envelope] {
        let demand = store.close_cursor_demands(phase, 7).unwrap();
        let grant = RetainedCloneGrant { maximum_copy_bytes: 7, maximum_release_bytes: 0, ..physical_test_close_grant() };
        let (admitted, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| store.close_cursor_admit(phase, grant));
        let (owner, receipt) = admitted.unwrap();
        assert_eq!((requested, released), (receipt.retained_capacity_bytes, receipt.released_bytes));
        assert_eq!(receipt.retained_capacity_bytes, demand.capacity_bytes);
        drive_retirement_terminal(owner.expect("current and whole envelope transfer exactly once"));
        assert!(store.close_cursor_admit(phase, grant).unwrap().0.is_none());
    }
    assert!(store.live.current_detached && store.log.envelope_detached);
    close_test_store(&mut store);
}


#[test]
fn rejected_history_mutation_and_metadata_owners_close_under_one_item_grants() {
    let edit = Edit {
        line: None,
        id: "rejected-edit".repeat(32),
        actor: Some("actor".repeat(64).into()),
        forwards: vec![DemoMutation::SetN(SetN { n: 7 })],
        inverse: vec![DemoMutation::SetN(SetN { n: 3 })].into(),
        mutation_meta: Vec::new(),
        verb: None,
        sequence_number: 1,
        started_at: "started".repeat(64),
        finished_at: Some("finished".repeat(64)),
    };
    drive_retirement_terminal(Box::new(ArtifactStoreDecodedEditRetirement::new(edit, Arc::new(DemoMutationRetirementFactory))));
    drive_retirement_terminal(Box::new(ArtifactStoreHistoryMetadataRetirement::change(Change {
        id: "change".repeat(64),
        edit_ids: vec!["edit-a".repeat(64), "edit-b".repeat(64)],
        description: Some("metadata".repeat(64)),
        saved_at: "timestamp".repeat(64),
    })));
}


/// 📏️ Closes an erased member publication on its exact quoted per-axis demands.
fn close_erased_publication_quoted(publication: &mut dyn ErasedMemberStoreOneItemPublication) {
    publication.begin_close();
    for _ in 0..65_536 {
        if publication.terminal_is_empty() {
            return;
        }
        let demand = publication.retirement_demands(4096).expect("current member publication close frontier");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = publication.close_step(grant).expect("member publication closes on its quoted grant");
        assert!(step.progress().fits(grant));
    }
    panic!("member publication did not retire its exact owners");
}


#[semio_framework_async_macros::async_test]
async fn member_owned_batch_admission_keeps_exact_source_on_wrong_type_stale_revision_and_metadata_refusal() {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧩️composition/📨️emission/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut member = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "owned-batch", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    member.install_document_store_owners_exact(retained_demo_member_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
    let make_request = |mutations| MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(715), expected_generation: member.generation_now(), expected_revision: member.content_revision_now(), actor: fixture["metadata"]["actor"].as_str().unwrap().into(), group_id: Some(fixture["metadata"]["groupId"].as_str().unwrap().into()), transaction: None, mutations };
    let wrong: Vec<i32> = serde_json::from_value(fixture["wrongType"].clone()).unwrap();
    let wrong_pointer = wrong.as_ptr();
    let (wrong, _) = MemberStoreOwnedBatch::try_new(wrong, grant).unwrap_or_else(|_| panic!("funded wrong-type owner"));
    let mut wrong = make_request(wrong);
    assert!(member.begin_one_item_owned_publication(&mut wrong, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(wrong.mutations.mutations::<i32>().unwrap().as_ptr(), wrong_pointer);
    assert_eq!(serde_json::to_value(wrong.mutations.mutations::<i32>().unwrap()).unwrap(), fixture["wrongType"]);
    for _ in 0..1024 { if wrong.mutations.terminal_is_empty() { break; } assert!(wrong.mutations.close_granted(grant).unwrap().progress().fits(grant)); }
    assert!(wrong.mutations.terminal_is_empty());
    let values = fixture["values"].as_array().unwrap().iter().map(|row| DemoMutation::SetN(SetN { n: row.as_i64().unwrap() as i32 })).collect::<Vec<_>>();
    let pointer = values.as_ptr();
    let (values, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("funded exact mutation vector"));
    let mut request = make_request(values);
    request.expected_generation += 1;
    assert!(member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
    request.expected_generation -= 1;
    request.expected_revision[0] ^= 1;
    assert!(member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
    request.expected_revision[0] ^= 1;
    let original_actor=std::mem::take(&mut request.actor);
    assert!(member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).is_err());
    assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
    request.actor = original_actor;
    for (index, mutation) in request.mutations.mutations::<DemoMutation>().unwrap().iter().enumerate() {
        match mutation { DemoMutation::SetN(row) => assert_eq!(row.n as i64, fixture["values"][index].as_i64().unwrap()), _ => panic!("exact mutation variant/order changed") }
    }
    let (mut publication,_) = member.begin_one_item_owned_publication(&mut request, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() }).expect("same original owner admits after exact authority restoration").expect("funded publication birth");
    assert!(request.mutations.mutations::<DemoMutation>().is_none());
    assert_eq!(member.snapshot_ref().n, Some(0));
    close_erased_member_publication(&mut *publication, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 4096, ..one_item_test_policy() });
    for _ in 0..1024 { if request.mutations.terminal_is_empty() { break; } assert!(request.mutations.close_granted(grant).unwrap().progress().fits(grant)); }
    assert!(request.mutations.terminal_is_empty());
    assert_eq!(member.snapshot_ref().n, Some(0));
    for _ in 0..65_536 { if matches!(SpaceMember::close_owned_step(&mut member, physical_test_close_grant()).unwrap(), RetainedCloneStep::Complete(_)) { break; } }
    assert!(SpaceMember::close_owned_terminal_is_empty(&member));
}
