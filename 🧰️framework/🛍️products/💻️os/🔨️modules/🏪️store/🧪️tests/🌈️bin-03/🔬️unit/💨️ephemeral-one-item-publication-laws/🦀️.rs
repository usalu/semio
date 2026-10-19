/// 💨️ Ephemeral one item publication laws: the faulting ephemeral task, the transfer preparation faults, the monotone progress, the retained stale safe publications, the unclosed owner drop and the panicking body.

struct FaultingEphemeralTask {
    invalid_receipt: bool,
    closing: bool,
}

impl ArtifactEphemeralPreparationTask<DemoSnapshot, DemoMutation> for FaultingEphemeralTask {
    fn advance(&mut self, _: &DemoSnapshot, _: &mut Option<DemoMutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactEphemeralPreparationTaskStep<DemoSnapshot>, ValueError> {
        if !self.invalid_receipt {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"injected construction failure"));
        }
        let root=Arc::new(DemoSnapshot { n: Some(99) });
        let ownership=RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes+1,retained_capacity_bytes:semio_framework_value::shared_retirement_allocation_bytes::<DemoSnapshot>(),released_bytes:0};
        Ok(ArtifactEphemeralPreparationTaskStep::Prepared { root,ownership, checkpoint: ArtifactStoreOneItemCheckpoint { completed_items: 1, completed_bytes: grant.maximum_copy_bytes as u64 + 1, ..Default::default() } })
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }
}

impl ErasedSnapshotRetirement for FaultingEphemeralTask {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        assert!(self.closing);
        let empty = RetainedCloneProgress::default();
        Ok(if grant.maximum_items == 0 { RetainedCloneStep::Progress(empty) } else { RetainedCloneStep::Complete(empty) })
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }
    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {
        Ok(0)
    }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(1)
    }
}

/// 🚫️ A refused ephemeral preparation either raises its fault or answers `PreparationRefused` with the fault retained on the publication.
fn ephemeral_refusal(advance: Result<ArtifactStoreOneItemAdvance, String>, retained_fault: Option<&str>) -> String {
    match advance {
        Err(error) => error,
        Ok(ArtifactStoreOneItemAdvance::PreparationRefused) => retained_fault.expect("a refused ephemeral preparation retains its fault").to_string(),
        Ok(other) => panic!("a faulting ephemeral preparation advanced instead of refusing: {other:?}"),
    }
}

#[test]
fn ephemeral_transfer_preparation_faults_close_presence_and_transient_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🫧️preparation-fault/🔣️.json")).unwrap();
    let factory = ArtifactEphemeralTaskPreparationFactory::new(
        |_| Ok(ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: 64 }),
        |_, mutation| Ok(Box::new(FaultingEphemeralTask { invalid_receipt: matches!(mutation, DemoMutation::SetN(SetN { n: 1 })), closing: false })),
        Arc::new(DemoInitialSnapshotRetirementFactory),
        Arc::new(DemoMutationRetirementFactory),
    );
    for row in fixture["cases"].as_array().unwrap() {
        let mutation = || DemoMutation::SetN(SetN { n: row["mutation"].as_i64().unwrap() as i32 });
        let grant = ArtifactStoreOneItemGrant { maximum_items: 1,maximum_copy_bytes: 7,maximum_capacity_bytes: 7,maximum_release_bytes: 7,maximum_depth: 64 };
        let mut presence = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
        let presence_root_factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
        presence.install_local_retirement_factory(presence_root_factory.clone()).unwrap();
        let mut presence_publication = presence.begin_publish_one(semio_framework_job::OperationId(1), 0, mutation(), Some(&factory), Some(presence_root_factory)).unwrap();
        let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
        let mut transient_publication = transient.begin_publish_one_leased(semio_framework_job::OperationId(2), 0, mutation(), &factory, Arc::new(DemoInitialSnapshotRetirementFactory)).unwrap();
        let presence_advance = presence.advance_publish_one(&mut presence_publication, grant);
        let presence_error = ephemeral_refusal(presence_advance, presence_publication.fault());
        let transient_advance = transient.advance_publish_one(&mut transient_publication, grant);
        let transient_error = ephemeral_refusal(transient_advance, transient_publication.fault());
        assert_eq!(presence_error, row["fault"].as_str().unwrap());
        assert_eq!(transient_error, presence_error);
        for publication in [&mut presence_publication, &mut transient_publication] {
            assert_eq!(publication.fault(), Some(presence_error.as_str()));
            assert_eq!(format!("{:?}", publication.phase()), fixture["phase"].as_str().unwrap());
            assert_eq!(publication.close_step(ArtifactStoreOneItemGrant { maximum_items: 0,maximum_copy_bytes: 4096,maximum_capacity_bytes: 4096,maximum_release_bytes: 4096,maximum_depth: 64 }).unwrap().progress(), RetainedCloneProgress::default());
            close_ephemeral_publication(publication);
            assert!(publication.terminal_is_empty());
        }
        assert_eq!(serde_json::to_value(presence.local().n).unwrap(), fixture["initial"]);
        assert_eq!(serde_json::to_value(transient.current_root().n).unwrap(), fixture["initial"]);
        let mut retirement = presence.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
        test_support::drive_retirement(&mut retirement).expect("presence retirement reaches its terminal-empty witness under its quoted demands");
        assert!(retirement.terminal_is_empty());
    }
}

/// 📍️ An ephemeral one-item publication's `progress()` is MONOTONE across its whole life. It used to
/// answer `ArtifactStoreOneItemCheckpoint::default()` — all zeros — the moment `close_step` released
/// its preparation owner, so a host polling a gesture saw its progress snap back to 0 between items
/// (measured on `🌊️flow` as `preparation progress changed from 22013 to 0`, ticket 26/09/19
/// `📓️flow.md` §5.4). Every window-transient, presence and transient publication polls through this
/// exact reader, and its batched twin `ArtifactStoreBatchPublication::progress()` already retains its
/// checkpoint the same way.
#[semio_framework_async_macros::async_test]
async fn an_ephemeral_one_item_publication_reports_monotone_progress_across_owner_release() {
    let grant = ephemeral_test_close_grant();
    let factory = DemoEphemeralPreparationFactory::admissible();
    let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    let mut publication = transient.begin_publish_one(semio_framework_job::OperationId(41), 0, DemoMutation::SetN(SetN { n: 4 }), Some(&factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("ephemeral publication admits");
    let mut seen = publication.progress();
    let mut check = |next: ArtifactStoreOneItemCheckpoint, seen: &mut ArtifactStoreOneItemCheckpoint, phase: &str| {
        assert!(
            next.completed_items >= seen.completed_items && next.completed_bytes >= seen.completed_bytes,
            "ephemeral progress went backwards at {phase}: {:?} items / {:?} bytes after {:?} items / {:?} bytes",
            next.completed_items,
            next.completed_bytes,
            seen.completed_items,
            seen.completed_bytes
        );
        *seen = next;
    };
    loop {
        let advance = transient.advance_publish_one(&mut publication, grant).expect("ephemeral step");
        check(publication.progress(), &mut seen, "advance");
        if matches!(advance, ArtifactStoreOneItemAdvance::Published(_)) {
            break;
        }
    }
    let published = publication.progress();
    assert!(published.completed_items > 0, "a published ephemeral gesture reports the work it actually did, not zero");
    assert!(publication.acknowledge());
    for _ in 0..2048 {
        let step = publication.close_step(ephemeral_test_close_grant()).expect("ephemeral close advances");
        check(publication.progress(), &mut seen, "close");
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert_eq!(publication.progress(), published, "a fully retired ephemeral publication still reports the gesture it completed");
    close_ephemeral_publication(&mut publication);
}

#[semio_framework_async_macros::async_test]
async fn presence_and_transient_one_item_publications_are_retained_stale_safe_cancelable_and_exactly_closed() {
    let grant = ephemeral_test_close_grant();
    let presence_factory = DemoEphemeralPreparationFactory::admissible();
    let mut presence = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    let root_factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
    presence.install_local_retirement_factory(root_factory.clone()).unwrap();
    let old_presence_root = Arc::downgrade(presence.local_root().expect("live local presence owner"));
    let mut presence_publication = presence.begin_publish_one(semio_framework_job::OperationId(5), 0, DemoMutation::SetN(SetN { n: 5 }), Some(&presence_factory), Some(root_factory.clone())).expect("presence factory admits");
    let presence_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = presence.advance_publish_one(&mut presence_publication, grant).expect("presence step") {
            break receipt;
        }
    };
    assert_eq!(presence_receipt, LaneItemReceipt { generation_before: 0, generation_after: 1 });
    assert_eq!(presence.local().n, Some(5));
    let presence_root = presence_factory.published_root.lock().expect("presence witness").as_ref().and_then(std::sync::Weak::upgrade).expect("presence root");
    assert!(std::ptr::eq(presence_root.as_ref(), presence.local()));
    assert!(presence_publication.acknowledge());
    assert!(old_presence_root.upgrade().is_some(), "displaced presence root remains retained before bounded close");
    let _ = presence_publication.close_step(ephemeral_test_close_grant()).expect("presence close advances one owner");
    assert!(old_presence_root.upgrade().is_some(), "first close unit cannot destroy the displaced root");
    close_ephemeral_publication(&mut presence_publication);
    for _ in 0..2048 {
        if matches!(presence.maintenance_local_reads_step(physical_test_close_grant()).unwrap(), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(old_presence_root.upgrade().is_none(), "bounded retirement releases the displaced presence root");
    assert!(presence.begin_publish_one(semio_framework_job::OperationId(6), 0, DemoMutation::SetN(SetN { n: 9 }), Some(&presence_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());

    let transient_factory = DemoEphemeralPreparationFactory::admissible();
    let mut transient = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(2) });
    let root = transient.current_root();
    let mut cancelled = transient.begin_publish_one(semio_framework_job::OperationId(7), 0, DemoMutation::SetN(SetN { n: 6 }), Some(&transient_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("transient factory admits");
    assert!(transient.cancel_publish_one(&mut cancelled));
    close_ephemeral_publication(&mut cancelled);
    assert_eq!(transient.generation_now(), 0);
    assert!(Arc::ptr_eq(&root, &transient.current_root()));
    drop(root);
    let old_transient_root = Arc::downgrade(&transient.current_root());
    let mut transient_publication =
        transient.begin_publish_one(semio_framework_job::OperationId(8), 0, DemoMutation::SetN(SetN { n: 11 }), Some(&transient_factory), Some(Arc::new(DemoSnapshotRetirementFactory))).expect("transient publication admits");
    let transient_receipt = loop {
        if let ArtifactStoreOneItemAdvance::Published(receipt) = transient.advance_publish_one(&mut transient_publication, grant).expect("transient step") {
            break receipt;
        }
    };
    assert_eq!(transient_receipt, LaneItemReceipt { generation_before: 0, generation_after: 1 });
    assert_eq!(transient.current().await.n, Some(11));
    let transient_root = transient_factory.published_root.lock().expect("transient witness").as_ref().and_then(std::sync::Weak::upgrade).expect("transient root");
    assert!(Arc::ptr_eq(&transient_root, &transient.current_root()));
    assert!(transient_publication.acknowledge());
    assert!(old_transient_root.upgrade().is_some(), "displaced transient root remains retained before bounded close");
    let _ = transient_publication.close_step(ephemeral_test_close_grant()).expect("transient close advances one owner");
    assert!(old_transient_root.upgrade().is_some(), "first close unit cannot destroy the displaced transient root");
    close_ephemeral_publication(&mut transient_publication);
    assert!(old_transient_root.upgrade().is_none(), "bounded retirement releases the displaced transient root");
    let oversized =
        DemoEphemeralPreparationFactory { footprint: ArtifactStoreOneItemFootprint { work_items: ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS + 1, retained_bytes: ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES }, published_root: Arc::new(DemoPublishedRootWitness::new()), snapshots: Arc::new(DemoSnapshotRetirementFactory), mutations: Arc::new(DemoMutationRetirementFactory) };
    assert!(transient.begin_publish_one(semio_framework_job::OperationId(9), 1, DemoMutation::SetN(SetN { n: 7 }), Some(&oversized), Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());
    assert!(transient.begin_publish_one(semio_framework_job::OperationId(10), 1, DemoMutation::SetN(SetN { n: 7 }), None, Some(Arc::new(DemoSnapshotRetirementFactory))).is_err());
    drop(presence_root);
    let mut close = presence.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
    test_support::drive_retirement(&mut close).expect("presence retirement reaches its terminal-empty witness under its quoted demands");
    assert!(close.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "terminal-empty witness")]
async fn artifact_store_one_item_drop_rejects_an_unclosed_publication_owner() {
    let store = Box::leak(Box::new(ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "retained-drop", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await));
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let publication = store
        .begin_apply_batch(
            semio_framework_job::OperationId(10),
            store.generation_now(),
            store.content_revision_now(),
            "retained-test".into(),
            vec![DemoMutation::SetN(SetN { n: 1 })],
            HistoryLane::Document,
            Some(&(Arc::new(DemoOneItemPreparationFactory::admissible()) as Arc<dyn ArtifactStoreOneItemPreparationFactory<DemoSnapshot, DemoMutation>>)),
            None,
        )
        .expect("publication admits");
    drop(publication);
}

/// 🪲️ A failing body that still holds a live, non-terminal `ArtifactStore` must surface as ONE
/// ordinary panic. Unguarded, the `Drop` witness fired during the unwind, and that double panic
/// became `thread caused non-unwinding panic. aborting.` — a `SIGABRT` that erased libtest's
/// summary for every other test in the binary. Sibling law to
/// `artifact_store_one_item_drop_rejects_an_unclosed_publication_owner` directly above, which
/// proves the witness stays strict when nothing is unwinding.
#[semio_framework_async_macros::async_test]
async fn a_panicking_body_holding_a_live_store_unwinds_instead_of_aborting_the_process() {
    let store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "unwind-witness", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _live = store;
        panic!("simulated assertion failure with a live store in scope");
    }));
    std::panic::set_hook(previous);
    let payload = outcome.expect_err("the simulated failure must unwind, not abort the process");
    let message = payload.downcast_ref::<String>().map(String::as_str).or_else(|| payload.downcast_ref::<&'static str>().copied());
    assert_eq!(message, Some("simulated assertion failure with a live store in scope"));
}
