// 🛤️ History lane, interaction store and immutable root laws: the history lane defaults, undo and redo within a lane, the interaction entries, the interaction store reset, the constant time snapshot, presence and transient roots.

//#region 🔖️HistoryLaneTests
#[semio_framework_async_macros::async_test]
async fn history_lane_defaults_to_document() {
    assert_eq!(HistoryLane::default(), HistoryLane::Document);
}

/// 🛤️ The design's headline acceptance case: undoing after an interleaved run of
/// document/interaction edits reverts the last DOCUMENT edit, skipping past trailing (and even
/// mid-history) `Interaction`-lane entries in both directions, which stay applied throughout.
///
/// Default undo skips the TRAILING interaction2 edit to revert doc2 instead.
///
/// A second default undo reverts doc1 — the only remaining Document-lane entry — even though
/// it now sits BEFORE two still-applied interaction edits in `applied_edit_ids`.
///
/// Default redo mirrors it: restores doc1 first (nearest Document entry in the redo stack),
/// then doc2, never touching either interaction edit's own applied/redo membership; each lands
/// back at its own HLC position in the applied order.
#[semio_framework_async_macros::async_test]
async fn history_lane_default_undo_and_redo_skip_interaction_entries() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply doc1");
    let doc1_id = store.applied_edit_ids()[0].clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 100 })], lane: HistoryLane::Interaction, transaction: None }).await.expect("apply interaction1");
    let interaction1_id = store.applied_edit_ids()[1].clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply doc2");
    let doc2_id = store.applied_edit_ids()[2].clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 200 })], lane: HistoryLane::Interaction, transaction: None }).await.expect("apply interaction2");
    let interaction2_id = store.applied_edit_ids()[3].clone();

    assert_eq!(store.envelope().lanes.get(&interaction1_id), Some(&HistoryLane::Interaction));
    assert_eq!(store.envelope().lanes.get(&interaction2_id), Some(&HistoryLane::Interaction));
    assert!(store.envelope().lanes.get(&doc1_id).is_none(), "an ordinary Document-lane edit never gets a `lanes` entry (sparse ledger)");
    assert!(store.envelope().lanes.get(&doc2_id).is_none());

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo skips interaction2 to revert doc2");
    assert_eq!(store.applied_edit_ids(), &[doc1_id.clone(), interaction1_id.clone(), interaction2_id.clone()], "doc2 removed; both interaction edits remain applied");
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&doc2_id));

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo doc1 despite interaction edits between it and the tail");
    assert_eq!(store.applied_edit_ids(), &[interaction1_id.clone(), interaction2_id.clone()]);
    assert_eq!(store.redo_edit_ids(), &[doc2_id.clone(), doc1_id.clone()]);

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo doc1");
    assert_eq!(store.applied_edit_ids(), &[doc1_id.clone(), interaction1_id.clone(), interaction2_id.clone()]);
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&doc2_id));
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo doc2");
    assert_eq!(store.applied_edit_ids(), &[doc1_id.clone(), interaction1_id.clone(), doc2_id.clone(), interaction2_id.clone()]);
    assert!(store.redo_edit_ids().is_empty());
}

/// 🛤️ The completing half of the mechanism: `UndoInLane`/`RedoInLane` walk a NON-`Document`
/// lane explicitly and independently of the document lane's own cursor position.
///
/// Explicit lane-scoped undo reverts ONLY the interaction edit, leaving the document edit
/// applied — the mirror image of default `Undo` skipping it.
///
/// Redoing the Document lane from here has nothing to redo — only the Interaction lane's
/// cursor moved, proving the two lanes' redo stacks are independent, not one shared position.
#[semio_framework_async_macros::async_test]
async fn history_lane_undo_in_lane_and_redo_in_lane_walk_only_the_requested_lane() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply doc");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 99 })], lane: HistoryLane::Interaction, transaction: None }).await.expect("apply interaction");
    let doc_id = store.applied_edit_ids()[0].clone();
    let interaction_id = store.applied_edit_ids()[1].clone();

    test_support::dispatch_test_command(&mut store, ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction }).await.expect("undo in interaction lane");
    assert_eq!(store.applied_edit_ids(), std::slice::from_ref(&doc_id));
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&interaction_id));
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "reverting the interaction edit restores the document edit's own value");

    assert_eq!(test_support::dispatch_test_command(&mut store, ArtifactCommand::RedoInLane { lane: HistoryLane::Document }).await.unwrap_err(), VcsError::NothingToRedo);

    test_support::dispatch_test_command(&mut store, ArtifactCommand::RedoInLane { lane: HistoryLane::Interaction }).await.expect("redo in interaction lane");
    assert_eq!(store.applied_edit_ids(), &[doc_id.clone(), interaction_id.clone()]);
    assert!(store.redo_edit_ids().is_empty());
}

/// 🛤️ Acceptance: a history made ENTIRELY of `Interaction`-lane edits is a no-op for
/// default `Undo` (no `Document`-lane entry exists at all), while the lane-scoped API still
/// reaches them.
///
/// The explicit lane-scoped API can still walk them.
#[semio_framework_async_macros::async_test]
async fn history_lane_default_undo_is_a_no_op_when_every_edit_is_interaction_lane() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], lane: HistoryLane::Interaction, transaction: None }).await.expect("apply interaction1");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], lane: HistoryLane::Interaction, transaction: None }).await.expect("apply interaction2");
    assert_eq!(store.applied_edit_ids().len(), 2);

    let error = test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.unwrap_err();
    assert_eq!(error, VcsError::NothingToUndo, "no Document-lane entry exists to undo; both interaction edits must stay untouched");
    assert_eq!(store.applied_edit_ids().len(), 2, "default undo must not remove either interaction edit");

    test_support::dispatch_test_command(&mut store, ArtifactCommand::UndoInLane { lane: HistoryLane::Interaction }).await.expect("undo in interaction lane");
    assert_eq!(store.applied_edit_ids().len(), 1);
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}

/// 🛤️ `Interaction`-lane entries are ordinary persisted `Edit`s — they survive a plain
/// owned pack+SPR round trip, and a reloaded store's default undo still skips them.
#[semio_framework_async_macros::async_test]
async fn history_lane_interaction_entries_survive_owned_document_round_trip() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply doc");
    let doc_id = store.applied_edit_ids()[0].clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::ApplyInLane { mutations: vec![DemoMutation::SetN(SetN { n: 42 })], lane: HistoryLane::Interaction, transaction: None }).await.expect("apply interaction");
    let interaction_id = store.applied_edit_ids()[1].clone();
    assert_eq!(store.envelope().lanes.get(&interaction_id), Some(&HistoryLane::Interaction));

    let files = print_document_pack(store.envelope()).await.expect("owned document encode");
    let reloaded_envelope = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("owned document decode").envelope;
    assert_eq!(reloaded_envelope.lanes.get(&interaction_id), Some(&HistoryLane::Interaction), "lane tag must survive the owned document round trip");

    let mut reloaded = ArtifactStore::new(reloaded_envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.applied_edit_ids(), store.applied_edit_ids(), "reload seeds applied_edit_ids from the persisted cursor, same as any other edit");
    test_support::dispatch_test_command(&mut reloaded, ArtifactCommand::Undo).await.expect("undo on the reloaded store still skips the interaction edit");
    assert_eq!(reloaded.applied_edit_ids(), std::slice::from_ref(&interaction_id), "the document edit was removed; the interaction edit is the only one left applied");
    assert!(reloaded.redo_edit_ids().contains(&doc_id), "the reverted document edit now sits on the redo stack");
}

//#endregion 🔖️HistoryLaneTests

//#region 🔖️InteractionStoreTests
/// 🕹️ `InteractionStore::apply` mutates the local hover value and bumps `generation`,
/// mirroring `PresenceStore`/`TransientStore` — the same `Mutation<S>::diff().apply()` seam,
/// reused here with the file's existing `DemoSnapshot`/`DemoMutation` fixtures standing in for
/// an app's hover-shaped type.
#[semio_framework_async_macros::async_test]
async fn interaction_store_apply_updates_hover_and_bumps_generation() {
    let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    assert_eq!(store.generation().await, 0);
    assert_eq!(store.hover().await.n, Some(0));

    store.apply(&[DemoMutation::SetN(SetN { n: 7 })]).await.expect("valid interaction mutation");
    assert_eq!(store.hover().await.n, Some(7), "apply routes through Mutation::diff/Diff::apply like PresenceStore");
    assert_eq!(store.generation().await, 1);

    store.apply(&[]).await.expect("empty interaction batch");
    assert_eq!(store.generation().await, 1, "an empty mutation batch must not bump generation, same as PresenceStore/TransientStore");
}

/// 🔄️ `reset` discards the current hover outright (a host clears hover when a
/// view/window closes) and still bumps `generation` so a pending broadcast reflects the clear.
#[semio_framework_async_macros::async_test]
async fn interaction_store_reset_discards_hover_and_bumps_generation() {
    let mut store = InteractionStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(0) });
    store.apply(&[DemoMutation::SetN(SetN { n: 3 })]).await.expect("valid interaction mutation");
    assert_eq!(store.generation().await, 1);

    store.reset(DemoSnapshot { n: Some(0) }).await;
    assert_eq!(store.hover().await.n, Some(0));
    assert_eq!(store.generation().await, 2, "reset bumps generation even though the value returns to default");
}

/// 🏗️ `Default` seeds from `S::default()`, same convention as `PresenceStore`/`TransientStore`.
#[semio_framework_async_macros::async_test]
async fn interaction_store_default_seeds_from_hover_default() {
    let store = InteractionStore::<DemoSnapshot, DemoMutation>::default();
    assert_eq!(store.hover().await.n, Some(0));
    assert_eq!(store.generation().await, 0);
}

//#endregion 🔖️InteractionStoreTests

//#region 🔖️ImmutableOperationRoots
#[semio_framework_async_macros::async_test]
async fn artifact_snapshot_root_is_o1_and_generation_stable_until_the_next_event() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let generation = store.generation_now();
    let before = store.snapshot_root();
    let same = store.snapshot_root();
    assert!(Arc::ptr_eq(&before, &same), "capturing an immutable operation root must only retain the existing Arc");
    assert_eq!(generation, store.generation_now());

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }).await.expect("apply");
    let after = store.snapshot_root();
    assert!(!Arc::ptr_eq(&before, &after), "a committed event publishes a new immutable root");
    assert_eq!(before.n, Some(0), "an already-admitted operation retains its exact pre-event snapshot");
    assert_eq!(after.n, Some(7));
    assert_eq!(store.content_revision_now(), store.content_revision().await);
}

#[semio_framework_async_macros::async_test]
async fn presence_local_read_is_o1_and_never_clones_the_payload_at_capture() {
    let mut store = PresenceStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(1) });
    let factory: Arc<dyn SnapshotRetirementFactory<DemoSnapshot>> = Arc::new(DemoSnapshotRetirementFactory);
    store.install_local_retirement_factory(factory.clone()).unwrap();
    let before = store.local_read().unwrap();
    assert!(std::ptr::eq(before.get(), store.local()));
    assert_eq!(store.generation_now(), 0);

    store.apply(&[DemoMutation::SetN(SetN { n: 2 })]).await.expect("presence apply");
    let after = store.local_read().unwrap();
    assert!(!std::ptr::eq(before.get(), after.get()));
    assert_eq!(before.n, Some(1));
    assert_eq!(after.n, Some(2));
    assert_eq!(store.generation_now(), 1);
    drop(before);
    drop(after);
    let mut close = store.begin_retirement(Arc::new(DemoSnapshot { n: Some(0) }), |value| value.n == Some(0)).ok().unwrap();
    test_support::drive_retirement(&mut close).expect("presence retirement reaches its terminal-empty witness under its quoted demands");
    assert!(close.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn transient_root_is_o1_and_retains_the_exact_pre_reset_value() {
    let mut store = TransientStore::<DemoSnapshot, DemoMutation>::new(DemoSnapshot { n: Some(3) });
    let before = store.current_root();
    assert!(Arc::ptr_eq(&before, &store.current_root()));

    store.reset(DemoSnapshot { n: Some(4) }).await;
    let after = store.current_root();
    assert!(!Arc::ptr_eq(&before, &after));
    assert_eq!(before.n, Some(3));
    assert_eq!(after.n, Some(4));
    assert_eq!(store.generation_now(), 1);
}

//#endregion 🔖️ImmutableOperationRoots
