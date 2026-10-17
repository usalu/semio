//! 🔬️ Unit tests of the Store that exercise the demo document through the public API of the Store crates.
//!
//! 🧭️ Moved from the former store unit test module (chunk 4 of the cut by subject).

use super::*;

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

#[semio_framework_async_macros::async_test]
async fn apply_computes_backwards_from_pre_state() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None }).await.expect("apply");
    let edit = &store.envelope().vcs.edits[0];
    assert_eq!(edit.inverse.iter().cloned().collect::<Vec<_>>(), vec![DemoMutation::AssignN(AssignN { n: Some(0) })]);
}

#[semio_framework_async_macros::async_test]
async fn commit_checkpoint_wraps_edits_into_change() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("init".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).await.expect("commit");
    assert_eq!(store.envelope().vcs.changes.len(), 1);
    assert_eq!(store.envelope().vcs.checkpoints.len(), 1);
    assert_eq!(store.envelope().vcs.checkpoints[0].message, Some("init".into()));
}

#[semio_framework_async_macros::async_test]
async fn checkout_checkpoint_restores_applied_edits() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit");
    let checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }).await.expect("apply2");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(9));
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CheckoutCheckpoint { checkpoint_id }).await.expect("checkout");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));
}

#[semio_framework_async_macros::async_test]
async fn alternatives_switch_restores_checkpoint_chain() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CreateAlternative { name: "branch-a".into() }).await.expect("create alternative");
    let alt_id = store.envelope().vcs.alternatives.last().expect("the branched alternative follows the trunk").id.clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply on branch");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));
    {
        let trunk = store.trunk_alternative_id();
        test_support::dispatch_test_command(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk })
    }
    .await
    .expect("back to the trunk");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "the trunk tip does not show another line's uncommitted edit");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: alt_id }).await.expect("back to the alternative");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "the alternative tip keeps the edit that was authored on it");
}

#[semio_framework_async_macros::async_test]
async fn checkout_old_checkpoint_then_commit_creates_a_fork() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit c1");
    let c1 = store.envelope().vcs.checkpoints[0].id.clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).await.expect("commit c2");
    let transitions = store.envelope().transitions.len();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CheckoutCheckpoint { checkpoint_id: c1.clone() }).await.expect("checkout c1");
    assert_eq!(store.envelope().transitions.len(), transitions, "a checkout is this viewer's head and authors no shared event");
    assert_eq!(store.current_checkpoint_id().await, Some(c1.as_str()));
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "an explicit checkout hides the later trunk commit");
    {
        let trunk = store.trunk_alternative_id();
        test_support::dispatch_test_command(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk })
    }
    .await
    .expect("back to the trunk tip");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));
    let children: Vec<&Checkpoint> = store.envelope().vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(c1.as_str())).collect();
    assert_eq!(children.len(), 1, "looking at an old checkpoint does not fork the trunk");
}

#[semio_framework_async_macros::async_test]
async fn create_alternative_appends_commits_to_its_own_checkpoint_chain() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).await.expect("commit root");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CreateAlternative { name: "feature-a".into() }).await.expect("create alternative");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("branch commit".into()), authors: Vec::new() }).await.expect("commit on branch");
    assert_eq!(store.envelope().vcs.alternatives.last().expect("the branched alternative").checkpoint_ids.len(), 2);
    assert_eq!(store.envelope().vcs.alternatives.first().expect("the trunk").checkpoint_ids.len(), 1, "the branch commit never grows the trunk");
    assert_eq!(store.envelope().vcs.checkpoints.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn history_columns_orders_newest_first_and_labels_trunk_root() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit c1");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }).await.expect("commit c2");
    let columns = store.history_columns().await;
    assert_eq!(columns.len(), 2);
    assert_eq!(columns[0].description, Some("c2".into()), "newest checkpoint must be first");
    assert_eq!(columns[0].lane, 0);
    assert_eq!(columns[0].labels, vec!["main".to_string()], "newest unlabeled row falls back to main");
    assert!(columns[1].labels.is_empty(), "only the newest row gets the main fallback");
    let json = serde_json::to_string(&test_support::SerdeValue(&columns[0].to_value())).expect("serialize");
    assert!(json.contains("checkpointId"), "wire format must be camelCase: {json}");
}

#[semio_framework_async_macros::async_test]
async fn history_columns_assigns_distinct_lanes_and_pulls_main_only_descendants_to_trunk() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }).await.expect("commit root");
    let root = store.envelope().vcs.checkpoints[0].id.clone();

    test_support::dispatch_test_command(&mut store, ArtifactCommand::CreateAlternative { name: "feature-a".into() }).await.expect("create feature-a");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }).await.expect("commit a1");

    {
        let trunk = store.trunk_alternative_id();
        test_support::dispatch_test_command(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk })
    }
    .await
    .expect("back to the trunk");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CreateAlternative { name: "feature-b".into() }).await.expect("create feature-b");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }).await.expect("commit b1");

    {
        let trunk = store.trunk_alternative_id();
        test_support::dispatch_test_command(&mut store, ArtifactCommand::SwitchAlternative { alternative_id: trunk })
    }
    .await
    .expect("trunk again");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("main resumed".into()), authors: Vec::new() }).await.expect("commit main resumed");

    let columns = store.history_columns().await;
    assert_eq!(columns.len(), 4, "root + a1 + b1 + main-resumed");
    let by_message: HashMap<String, &HistoryColumn> = columns.iter().filter_map(|column| column.description.clone().map(|description| (description, column))).collect();
    assert_eq!(by_message["root"].lane, 0, "root has no parent, lane 0");
    assert_eq!(by_message["main resumed"].lane, 0, "commit with no alternative stays on the trunk");
    let a_lane = by_message["a1"].lane;
    let b_lane = by_message["b1"].lane;
    assert_ne!(a_lane, 0, "a1 belongs to an alternative, not the trunk");
    assert_ne!(b_lane, 0, "b1 belongs to an alternative, not the trunk");
    assert_ne!(a_lane, b_lane, "distinct alternatives must get distinct swimlanes");

    let root_children: Vec<&HistoryColumn> = columns.iter().filter(|column| column.parent_checkpoint_id.as_deref() == Some(root.as_str())).collect();
    assert_eq!(root_children.len(), 3, "root forked three ways: a1, b1, main-resumed");
}

#[semio_framework_async_macros::async_test]
async fn backbone_message_binary_round_trips_every_variant() {
    let genesis = BackboneMessage::Genesis { pack: vec![1, 2, 3] };
    assert_eq!(BackboneMessage::decode_op(&genesis.encode_op().unwrap()).unwrap(), genesis);

    let envelope = sample_envelope_for_backbone_test().await;
    let operations = BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&[envelope.clone(), envelope]) };
    assert_eq!(BackboneMessage::decode_op(&operations.encode_op().unwrap()).unwrap(), operations);

    let ack = BackboneMessage::Ack { op_ids: vec!["op-1".to_string(), "op-2".to_string()] };
    assert_eq!(BackboneMessage::decode_op(&ack.encode_op().unwrap()).unwrap(), ack);

    let empty_ack = BackboneMessage::Ack { op_ids: Vec::new() };
    assert_eq!(BackboneMessage::decode_op(&empty_ack.encode_op().unwrap()).unwrap(), empty_ack);

    for (retract, hex) in [(BackboneMessage::Retract { mutation_ids: Vec::new() }, "01040001000c00"), (BackboneMessage::Retract { mutation_ids: vec!["a".into()] }, "010401016101000c010600")] {
        let encoded = retract.encode_op().unwrap();
        assert_eq!(encoded.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), hex, "the retraction vector the TypeScript twin writes");
        assert_eq!(decode_hot_backbone_message_exact(&encoded).unwrap(), retract);
    }
}

async fn sample_envelope_for_backbone_test() -> semio_framework_os_spr::MutationEnvelope {
    semio_framework_os_spr::MutationEnvelope {
        mutation_id: MutationId("op-1".to_string()),
        document_id: ArtifactId("doc-1".to_string()),
        actor: ActorId("actor-1".to_string().into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: vec![1, 2, 3] },
        inverse: semio_framework_os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
        timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 },
        transaction: None,
        verb: None,
        line: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn no_backbone_by_default() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    assert!(envelope.backbone.is_none(), "a fresh document has no attached backbone");
    let store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert!(store.backbone_ref().is_none());
}

#[semio_framework_async_macros::async_test]
async fn memory_backbone_pair_propagates_edits_bidirectionally() {
    let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b").await;
    let envelope_a: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let envelope_b: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store_a = ArtifactStore::new(envelope_a, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut store_b = ArtifactStore::new(envelope_b, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store_a.attach_backbone(Backbones::Memory(backbone_a)).await.expect("attach a");
    store_b.attach_backbone(Backbones::Memory(backbone_b)).await.expect("attach b");

    test_support::dispatch_test_command(&mut store_a, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply on a");
    store_b.tick().await.expect("tick b");
    assert_eq!(store_b.snapshot().expect("snapshot b").n, Some(1), "b receives a's edit");

    test_support::dispatch_test_command(&mut store_b, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply on b");
    store_a.tick().await.expect("tick a");
    assert_eq!(store_a.snapshot().expect("snapshot a").n, Some(2), "a receives b's edit");
}

#[semio_framework_async_macros::async_test]
async fn backbone_retirement_blocks_for_live_peer_then_drains_one_owned_message_or_byte_grant() {
    let (mut local, peer) = MemoryBackbone::pair("close-local", "close-peer").await;
    local.send(BackboneMessage::Genesis { pack: vec![1; 64 * 1024] }).await.expect("seed deep close queue");
    let mut retirement = ArtifactStoreBackboneRetirement::new(Backbones::Memory(local));
    let mut observed_blocked = false;
    for _ in 0..32 {
        let grant = RetainedCloneGrant { maximum_copy_bytes: 17, maximum_capacity_bytes: 0, ..physical_test_close_grant() };
        assert!(retirement.next_release_byte_demand().expect("original backing release demand") <= grant.maximum_release_bytes);
        assert!(retirement.next_depth_demand().expect("original backing depth demand") <= grant.maximum_depth);
        match retirement.close_step(grant).expect("bounded pre-peer close") {
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => {
                observed_blocked = true;
                break;
            }
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => assert!(progress.fits(grant)),
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => panic!("a live peer still retains both exact queue authorities"),
        }
    }
    assert!(observed_blocked);
    drop(peer);
    let mut turns = 0;
    while !retirement.terminal_is_empty() {
        let grant = RetainedCloneGrant { maximum_copy_bytes: 17, maximum_capacity_bytes: 0, ..physical_test_close_grant() };
        assert!(retirement.next_release_byte_demand().expect("original backing release demand") <= grant.maximum_release_bytes);
        assert!(retirement.next_depth_demand().expect("original backing depth demand") <= grant.maximum_depth);
        match retirement.close_step(grant).expect("bounded post-peer close") {
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => { assert!(progress.fits(grant)); assert!(progress != RetainedCloneProgress::default(), "no external queue owner remains after peer close"); },
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(retirement.terminal_is_empty()); },
        }
        turns += 1;
        assert!(turns < 100_000, "backbone queue and payload retirement terminates across interruptions");
    }
}

#[semio_framework_async_macros::async_test]
async fn detach_backbone_stops_synchronizing_but_keeps_the_wip_graph() {
    let (backbone_a, backbone_b) = MemoryBackbone::pair("peer-a", "peer-b").await;
    let mut store_a = ArtifactStore::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut store_b: ArtifactStore<DemoSnapshot, DemoMutation> = ArtifactStore::new(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store_a.attach_backbone(Backbones::Memory(backbone_a)).await.expect("attach a");
    store_b.attach_backbone(Backbones::Memory(backbone_b)).await.expect("attach b");
    store_a.detach_backbone().expect("detach source backbone");
    assert!(store_a.backbone_ref().is_none());

    test_support::dispatch_test_command(&mut store_a, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None }).await.expect("apply after detach still works on the in-memory graph");
    assert_eq!(store_a.snapshot().expect("snapshot a").n, Some(9));
    store_b.tick().await.expect("tick b");
    assert_eq!(store_b.snapshot().expect("snapshot b").n, Some(0), "detached edits never reach the peer");
}

#[semio_framework_async_macros::async_test]
async fn loaded_envelope_with_stale_backbone_ref_never_auto_attaches() {
    let stale = || create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, Some(ArtifactBackboneRef { uri: "folder:///nonexistent/path".into() }), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(stale(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert!(!store.tick().await.expect("tick with no live backbone is a no-operation"), "no backbone was ever attached, so there is nothing to pump");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply works purely against the in-memory graph");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1));

    store.reset(stale()).await.expect("reset");
    assert!(!store.tick().await.expect("tick after set_state with no live backbone is a no-operation"), "set_state must not resurrect IO from a stale backbone descriptor either");
}

/// 🧬️ A component codec that answers `codec.print-mirror` with the pair's byte counts, so a law can
/// see the pair it was handed.
struct EchoComponentCodec {
    schema: &'static str,
}

impl ComponentDocumentCodec for EchoComponentCodec {
    fn schema(&self) -> &str {
        self.schema
    }

    fn pack_schema_hash(&self) -> ComponentDocumentCodecFuture<'_, [u8; 32]> {
        Box::pin(async move { Ok([9; 32]) })
    }

    fn print_mirror<'a>(&'a self, pack: &'a [u8], spr: &'a [u8]) -> ComponentDocumentCodecFuture<'a, ArtifactTextFiles> {
        Box::pin(async move { Ok(ArtifactTextFiles { dsl: pack.len().to_string(), ops: spr.len().to_string() }) })
    }
}

/// 🧭️ One resolution per schema, in the hub's trusted-catalog order: the linked Rust codec whenever
/// this binary links one — a mounted component owning the same kind never displaces it — and the
/// mounted component's codec for a kind nothing links, answering identity and the validation mirror
/// itself; a kind neither owns resolves to nothing (ticket 26/09/23 slice WG8, B2).
#[semio_framework_async_macros::async_test]
async fn a_kind_resolves_to_its_linked_codec_before_its_mounted_component() {
    let linked = artifact_codec::<DemoSnapshot, DemoMutation>("test.kind-codec-linked/v1");
    let linked_hash = linked.pack_schema_hash;
    register_document_codec(linked).expect("linked codec registers");
    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.kind-codec-linked/v1" })).expect("component codec registers");
    register_component_document_codec(Arc::new(EchoComponentCodec { schema: "test.kind-codec-component/v1" })).expect("component codec registers");

    let resolved = document_kind_codec("test.kind-codec-linked/v1").await.expect("registry").expect("linked kind resolves");
    assert!(matches!(resolved, DocumentKindCodec::Linked(_)), "a linked codec outranks a mounted component");
    assert_eq!(resolved.pack_schema_hash().await.expect("linked identity"), linked_hash);

    let resolved = document_kind_codec("test.kind-codec-component/v1").await.expect("registry").expect("component kind resolves");
    assert!(matches!(resolved, DocumentKindCodec::Component(_)));
    assert_eq!(resolved.pack_schema_hash().await.expect("component identity"), [9; 32]);
    assert_eq!(resolved.print_mirror(&[1, 2, 3], &[4]).await.expect("component mirror"), ArtifactTextFiles { dsl: "3".into(), ops: "1".into() }, "the mirror is the component's answer for exactly the pair handed over");

    assert!(document_kind_codec("test.kind-codec-unowned/v1").await.expect("registry").is_none());
}

#[semio_framework_async_macros::async_test]
async fn document_codec_of_round_trips_dsl_and_pack_and_edit_text() {
    let codec = artifact_codec::<DemoSnapshot, DemoMutation>("test.document-codec-roundtrip/v1");
    assert_eq!(codec.schema, "test.document-codec-roundtrip/v1");
    assert_eq!(codec.extension, "demo");

    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("test.document-codec-roundtrip/v1", "demo", DemoSnapshot { n: Some(4) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let text_files = print_document_text(&envelope).await;
    drop(envelope.into_owners());
    let text_files = text_files.expect("print document text");

    let (pack_files, dsl_mirror) = (codec.compile_dsl)(&text_files.dsl, &text_files.ops).await.expect("codec compile_dsl");
    assert_eq!(dsl_mirror, DemoSnapshot { n: Some(4) }.print_dsl(), "dsl mirror matches the initial snapshot's print_dsl");

    let mirrored = (codec.print_mirror)(&pack_files.pack, &pack_files.spr).await.expect("codec print_mirror");
    assert_eq!(mirrored.dsl, dsl_mirror, "print_mirror's dsl text agrees with compile_dsl's own mirror, no JSON round trip");

    let document_id = ArtifactId("demo".to_string());
    let schema = SchemaId("test.document-codec-roundtrip/v1".to_string());
    let edit = Edit {
        line: None,
        id: "edit-1".into(),
        actor: Some("peer".into()),
        forwards: vec![DemoMutation::SetN(SetN { n: 9 })],
        inverse: vec![DemoMutation::SetN(SetN { n: 4 })].into(),
        mutation_meta: Vec::new(),
        verb: None,
        sequence_number: 1,
        started_at: "0".into(),
        finished_at: None,
    };
    let mut op_envelopes = semio_framework_os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("op envelopes");
    let op_envelope = op_envelopes.pop().expect("exactly one op envelope for a single-op edit");
    let edit_text = (codec.edit_text_from_envelope)(&op_envelope).await.expect("codec edit_text_from_envelope");
    let lines: Vec<&str> = edit_text.lines().collect();
    assert_eq!(lines.len(), 4, "one complete append unit: edit header, forward op, inverse record, one metadata record: {edit_text:?}");
    assert!(lines[0].starts_with("edit "), "edit header first: {edit_text:?}");
    assert_eq!(lines[1], "  set-n n=9", "forward op line: {edit_text:?}");
    assert!(lines[2].starts_with("inverse ") && lines[2].contains("set-n n=4"), "inverse record carries the edit's inverse: {edit_text:?}");
    assert!(lines[3].starts_with("metadata "), "one metadata record per forward: {edit_text:?}");
    let replayed = parse_document_text::<DemoSnapshot, DemoMutation>(&text_files.dsl, &format!("{}{edit_text}", text_files.ops), &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("strict replay accepts the codec's append unit appended to the document's ops");
    assert_eq!(replayed.into_snapshot().n, Some(9), "the appended edit replays forward");

    preflight_document_codecs(std::slice::from_ref(&codec)).await.expect("preflight accepts an unclaimed full descriptor without publishing it");
    assert!(document_codec("test.document-codec-roundtrip/v1").await.expect("registry availability").is_none(), "preflight must not publish a codec");
    register_document_codec(codec).expect("first document codec registration");
    assert!(document_codec("test.document-codec-roundtrip/v1").await.expect("registry availability").is_some(), "registered codec is discoverable by schema string");
    assert!(document_codec("no-such-schema").await.expect("registry availability").is_none());
}

/// 🪪 Drives a linked codec's apply-ops thunk under a fixture caller-owned identity authority and returns its original cumulative receipt.
async fn apply_ops_under_fixture_identity(codec: &ArtifactCodec, pack: &[u8], spr: &[u8], ops: &[u8]) -> Result<(Vec<u8>, Vec<u8>, String), VcsError> {
    const CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
        assert!(progress.owned_bytes <= CEILING);
        true
    };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING, &mut observer).expect("declared fixture identity authority");
    let result = (codec.apply_ops_binary)(pack, spr, ops, &mut identity).await;
    drop(identity.pause().expect("original cumulative fixture identity receipt"));
    result
}

/// 🧩️ The NATIVE twin of the guest's `codec.apply-ops`: a LINKED Rust codec reducing a NONEMPTY
/// batch. `apply_ops_binary` is the path stdio and gis take on a hub — the hub links their codecs
/// and never asks their components — and it builds a throwaway `ArtifactStore` to reduce into.
///
/// 🪦️ `ArtifactStore::new` installs NO owner catalogue, and the close cursor at the end of that
/// thunk answers `artifact store has no owner-supplied bounded disposer` without one, so the first
/// nonempty batch through a linked codec could never return. The empty batch returns BEFORE a store
/// exists, which is why every previous law over this thunk (and the guest twin's, until ticket
/// 26/09/18 slice TC3e) passed while the real path was dead. This law drives one op through it and
/// reads the edit back out of the applied history, so a store that cannot close fails here rather
/// than in a hub's creation log.
#[semio_framework_async_macros::async_test]
async fn document_codec_apply_ops_binary_reduces_a_nonempty_batch_and_closes_its_store() {
    let codec = artifact_codec::<DemoSnapshot, DemoMutation>("test.document-codec-apply-ops/v1");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("test.document-codec-apply-ops/v1", "demo-apply-ops", DemoSnapshot { n: Some(4) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let baseline = print_document_pack(&envelope).await;
    drop(envelope.into_owners());
    let baseline = baseline.expect("print document pack");

    let empty = apply_ops_under_fixture_identity(&codec, &baseline.pack, &baseline.spr, &semio_framework_os_spr::encode_ops_vec(&[])).await.expect("an empty batch returns the baseline");
    assert!(!empty.0.is_empty() && !empty.1.is_empty(), "an empty apply-ops batch must return the baseline pair, not an empty one");

    let op = <DemoMutation as semio_framework_os_spr::OpBinary>::encode_op(&DemoMutation::SetN(SetN { n: 9 })).expect("encode set-n");
    let ops = semio_framework_os_spr::encode_ops_vec(&[op]);
    let applied = apply_ops_under_fixture_identity(&codec, &baseline.pack, &baseline.spr, &ops).await.expect("a nonempty batch reduces and closes its store");
    assert!(!applied.0.is_empty() && !applied.1.is_empty(), "apply_ops_binary produced an empty pair");
    let history = semio_framework_os_spr::decode_history(&applied.1, &semio_framework_os_spr::DecodeOptions::default()).await.expect("applied history");
    assert_eq!(history.doc_id, "demo-apply-ops");
    assert_eq!(history.schema, "test.document-codec-apply-ops/v1");
    assert_eq!(history.edits.len(), 1, "one op in the batch must land exactly one edit, got {}", history.edits.len());
    let passed = apply_ops_under_fixture_identity(&codec, &applied.0, &applied.1, &semio_framework_os_spr::encode_ops_vec(&[])).await.expect("an empty batch passes a populated pair through");
    assert!(!passed.0.is_empty() && !passed.1.is_empty(), "an empty batch over a populated pair returns that pair");
    let mirror = (codec.print_mirror)(&applied.0, &applied.1).await.expect("a populated pair mirrors");
    assert!(mirror.ops.contains("set-n"), "the mirror prints the pair's edit: {}", mirror.ops);
}

/// 📜️ The hub's Check In fold is a replica's own fold: replaying an author's whole ledger (two
/// edits, an undo, a checkpoint) onto the genesis pair prints exactly the pair of a replica that
/// ingested the same ledger remotely (the author's own pair differs only in author-local edit
/// timestamps no envelope carries), keeps the author's edit identity and snapshot, a ledger prefix
/// plus its tail equals the whole fold, and a corrupt stream or a stream missing a causal dependency
/// is refused while the reduction store still closes.
#[semio_framework_async_macros::async_test]
async fn replay_envelopes_onto_pair_equals_the_replica_that_folded_the_same_ledger() {
    let codec = artifact_codec::<DemoSnapshot, DemoMutation>("demo/v1");
    let genesis = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let genesis_pair = print_document_pack(&genesis).await;
    drop(genesis.into_owners());
    let genesis_pair = genesis_pair.expect("genesis pair");

    let mut author = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("first edit");
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("second edit");
    let prefix_events = author.event_log().expect("prefix ledger");
    test_support::dispatch_test_command(&mut author, ArtifactCommand::Undo).await.expect("undo the second edit");
    test_support::dispatch_test_command(&mut author, ArtifactCommand::CommitCheckpoint { message: Some("check in".into()), authors: Vec::new() }).await.expect("checkpoint");
    let events = author.event_log().expect("whole ledger");
    let author_pair = print_document_pack(author.envelope()).await.expect("author pair");
    let author_history = semio_framework_os_spr::decode_history(&author_pair.spr, &semio_framework_os_spr::DecodeOptions::default()).await.expect("author history");

    let mut replica = fresh_demo_store().await;
    for event in events.clone() {
        replica.ingest_remote(event).await.expect("the replica ingests the author's ledger");
    }
    let replica_pair = print_document_pack(replica.envelope()).await.expect("replica pair");

    let replayed = (codec.replay_envelopes)(&genesis_pair.pack, &genesis_pair.spr, &semio_framework_os_spr::encode_envelopes(&events)).await.expect("the whole ledger folds");
    assert_eq!((replayed.0.as_slice(), replayed.1.as_slice()), (replica_pair.pack.as_slice(), replica_pair.spr.as_slice()), "the fold prints the remote replica's exact pair");
    assert_eq!(replayed.0, author_pair.pack, "the fold keeps the author's snapshot container");
    let history = semio_framework_os_spr::decode_history(&replayed.1, &semio_framework_os_spr::DecodeOptions::default()).await.expect("replayed history");
    assert_eq!(history.edits.iter().map(|edit| edit.id.clone()).collect::<Vec<_>>(), author_history.edits.iter().map(|edit| edit.id.clone()).collect::<Vec<_>>(), "both edits keep the author's identity");
    assert_eq!(history.transitions.len(), 2, "the undo and the checkpoint transitions both fold");
    assert_eq!(replica.snapshot().expect("replica snapshot"), author.snapshot().expect("author snapshot"), "the fold converges on the author's content");

    let prefix = (codec.replay_envelopes)(&genesis_pair.pack, &genesis_pair.spr, &semio_framework_os_spr::encode_envelopes(&prefix_events)).await.expect("a ledger prefix folds");
    let resumed = (codec.replay_envelopes)(&prefix.0, &prefix.1, &semio_framework_os_spr::encode_envelopes(&events[prefix_events.len()..])).await.expect("the tail folds onto the prefix pair");
    assert_eq!((resumed.0.as_slice(), resumed.1.as_slice()), (replica_pair.pack.as_slice(), replica_pair.spr.as_slice()), "prefix pair + tail equals the whole fold");

    assert!((codec.replay_envelopes)(&genesis_pair.pack, &genesis_pair.spr, &[0xff, 0x01, 0x02]).await.is_err(), "a corrupt stream is refused");
    let missing_dependency = semio_framework_os_spr::encode_envelopes(&events[prefix_events.len()..]);
    assert!((codec.replay_envelopes)(&genesis_pair.pack, &genesis_pair.spr, &missing_dependency).await.is_err(), "transitions naming edits the pair never held are refused");
    assert!((codec.replay_envelopes)(&[], &[], &semio_framework_os_spr::encode_envelopes(&events)).await.is_err(), "no baseline is refused");
    close_test_store(&mut replica);
    close_test_store(&mut author);
}

#[semio_framework_async_macros::async_test]
async fn register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first() {
    let first = artifact_codec::<DemoSnapshot, DemoMutation>("test.duplicate-id-probe/v1");
    let second = ArtifactCodec { pack_schema_hash: [7u8; 32], ..first.clone() };
    assert_ne!(first.pack_schema_hash, second.pack_schema_hash, "fixture precondition: the two codecs must be distinguishable");

    register_document_codec(first.clone()).expect("first registration");
    register_document_codec(first.clone()).expect("an identical descriptor and executable is idempotent");
    let conflict = match register_document_codec(second).expect_err("a schema collision must reject rather than replace") {
        DocumentCodecRegistryError::Conflict(conflict) => conflict,
        DocumentCodecRegistryError::Unavailable => panic!("document codec registry unavailable"),
        DocumentCodecRegistryError::Binding(error) => panic!("document codec binding refused: {error:?}"),
    };
    assert_eq!(conflict.schema, "test.duplicate-id-probe/v1");
    let resolved = document_codec("test.duplicate-id-probe/v1").await.expect("registry availability").expect("still registered after the second call");
    assert_eq!(resolved.pack_schema_hash, first.pack_schema_hash, "the first codec remains authoritative after a conflict");
}

/// 🚫️async: E4 fn-pointer slot — `DialectMigration.migrate_pack` is `fn(&[u8]) ->
/// Result<Vec<u8>, String>` (unnameable if async) — see R9/E4.
#[semio_framework_async_macros::async_test]
async fn dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop() {
    fn append_marker(bytes: &[u8]) -> Result<Vec<u8>, String> {
        Ok([bytes, b"-migrated"].concat())
    }

    let from = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "test.runtime-migration".into(), standard: "1".into(), subset: "*".into() };
    let to = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "test.runtime-migration".into(), standard: "2".into(), subset: "*".into() };
    let migration = DialectMigration { from: from.clone(), to: to.clone(), lossless: true, migrate_pack: append_marker };
    preflight_dialect_migrations(std::slice::from_ref(&migration)).await.expect("preflight accepts an unclaimed dialect pair without mutation");
    assert!(matches!(migrate_document(&from, &to, b"seed").await, Err(DialectMigrationError::Missing { .. })), "preflight must not publish a migration");
    register_dialect_migrations(vec![migration.clone()]).expect("batch migration registration");
    assert_eq!(migrate_document(&from, &to, b"seed").await.expect("registered migration"), b"seed-migrated");

    let conflict = DialectMigration { lossless: false, ..migration };
    assert!(matches!(preflight_dialect_migrations(std::slice::from_ref(&conflict)).await, Err(DialectMigrationRegistryError::Conflict { .. })), "preflight must reject a descriptor change before batch commit");
    assert_eq!(migrate_document(&from, &to, b"seed").await.expect("first migration remains authoritative"), b"seed-migrated");
}

async fn projection_probe(store: &ArtifactStore<DemoSnapshot, DemoMutation>, cause: ArtifactProjectionCause) -> ArtifactProjectionResult<Option<i32>, DemoDiff> {
    let event = store.projection_event(cause, Some(DemoSnapshot { n: Some(-1) }), ArtifactProjectionCacheMode::ValidatePrevious, "deterministic").expect("capture projection event");
    assert_eq!(event.previous, Some(DemoSnapshot { n: Some(-1) }));
    assert_eq!(event.cache_mode, ArtifactProjectionCacheMode::ValidatePrevious);
    event.result(event.state.n, None).await
}

async fn assert_projection_is_stale(store: &ArtifactStore<DemoSnapshot, DemoMutation>, result: ArtifactProjectionResult<Option<i32>, DemoDiff>) {
    assert!(matches!(store.accept_projection_result(result), Err(StaleArtifactProjection { .. })), "a result for an older projection stamp must be rejected");
}

#[semio_framework_async_macros::async_test]
async fn projection_result_gate_rejects_results_after_every_invalidating_store_transition() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "projection", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let before_apply = projection_probe(&store, ArtifactProjectionCause::Apply).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    assert_projection_is_stale(&store, before_apply).await;

    let before_undo = projection_probe(&store, ArtifactProjectionCause::Undo).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
    assert_projection_is_stale(&store, before_undo).await;

    let before_redo = projection_probe(&store, ArtifactProjectionCause::Redo).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo");
    assert_projection_is_stale(&store, before_redo).await;

    let before_remote = projection_probe(&store, ArtifactProjectionCause::RemoteIngest).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::IngestRemote { envelope: foreign_mutation_envelope("projection-peer", DemoMutation::SetN(SetN { n: 2 })).await }).await.expect("remote ingest");
    assert_projection_is_stale(&store, before_remote).await;

    let before_reset = projection_probe(&store, ArtifactProjectionCause::Reset).await;
    let reset_envelope = owned_test_envelope(&store).await;
    store.reset(reset_envelope).await.expect("reset");
    assert_projection_is_stale(&store, before_reset).await;

    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("checkpoint");
    let checkpoint_id = store.current_checkpoint_id().await.expect("checkpoint id").to_string();
    let before_checkout = projection_probe(&store, ArtifactProjectionCause::Checkout).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CheckoutCheckpoint { checkpoint_id }).await.expect("checkout");
    assert_projection_is_stale(&store, before_checkout).await;
}

#[semio_framework_async_macros::async_test]
async fn projection_result_gate_rejects_results_after_dependency_and_checkpoint_transitions() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "projection-dependencies", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    let before_replay = projection_probe(&store, ArtifactProjectionCause::Replay).await;
    assert_eq!(store.invalidate_after_replay().expect("replay invalidation").cause, ArtifactProjectionCause::Replay);
    assert_projection_is_stale(&store, before_replay).await;

    let before_policy = projection_probe(&store, ArtifactProjectionCause::PolicyChange).await;
    assert_eq!(store.invalidate_after_policy_change().expect("policy invalidation").cause, ArtifactProjectionCause::PolicyChange);
    assert_projection_is_stale(&store, before_policy).await;

    let before_resource = projection_probe(&store, ArtifactProjectionCause::ExternalResourceChange).await;
    assert_eq!(store.invalidate_after_external_resource_change().expect("resource invalidation").cause, ArtifactProjectionCause::ExternalResourceChange);
    assert_projection_is_stale(&store, before_resource).await;

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply before checkpoint");
    let before_checkpoint = projection_probe(&store, ArtifactProjectionCause::Checkpoint).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("non-empty checkpoint");
    assert_projection_is_stale(&store, before_checkpoint).await;
    assert_eq!(store.last_projection_invalidation().expect("checkpoint invalidation").cause, ArtifactProjectionCause::Checkpoint);

    let generation = store.generation();
    let error = test_support::dispatch_test_command(&mut store, ArtifactCommand::PruneDrafts).await.expect_err("draft pruning is explicitly unavailable");
    assert!(matches!(error, VcsError::ValidationFailed(_)));
    assert_eq!(store.generation(), generation, "a rejected prune cannot invalidate or report a success");
}

#[semio_framework_async_macros::async_test]
// 🎞️ The final paragraph this test used to have (`ValidatedMutation`/`Mutation::validate`
// rejecting `n < 0` before persisting) is gone — `Mutation::validate` is deleted (§C4/C10); the
// real outcome-messages/`MergePolicy` rejection this replaces is lane 1-A's C6 work.
async fn reset_and_apply_reject_malformed_history_before_persisting() {
    let fresh = || create_document_envelope("demo/v1", "reset-invalid", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut malformed_constructor = fresh();
    malformed_constructor.cursor = Some(ArtifactCursor::new(vec!["missing".into()], Vec::new(), None));
    assert!(matches!(semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(malformed_constructor, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await, Err(VcsError::UnknownEdit(id)) if id == "missing"), "construction must reject malformed cursor history before any mutation applies");

    let mut legacy_seed = ArtifactStore::new(fresh(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut legacy_seed, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }).await.expect("seed edit");
    let files = print_document_pack(legacy_seed.envelope()).await.expect("owned history encode");
    let mut cursorless = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("owned history decode").into_envelope();
    cursorless.cursor = None;
    let cursorless_store = ArtifactStore::new(cursorless, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(cursorless_store.snapshot().expect("cursorless snapshot"), DemoSnapshot { n: Some(3) });
    drop(cursorless_store);
    let mut duplicate_history = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("second owned history decode").into_envelope();
    duplicate_history.cursor = None;
    let duplicate_edit = duplicate_history.vcs.edits[0].clone();
    duplicate_history.vcs.edits.try_push(duplicate_edit).expect("duplicate test edit fits the fixed history ledger");
    assert!(matches!(semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(duplicate_history, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await, Err(VcsError::ValidationFailed(message)) if message.contains("repeats authoritative edit")), "duplicate authoritative edits cannot be hidden by first-match replay");

    let mut store = ArtifactStore::new(fresh(), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let generation = store.generation();
    let mut dangling = fresh();
    dangling.transitions.push(semio_framework_os_spr::history_transition_envelope(
        &semio_framework_os_spr::HistoryTransition::Revert { mutation_ids: vec![MutationId("missing".into())] },
        &ArtifactId("demo".into()),
        &ActorId("local".into()),
        Vec::new(),
        HybridLogicalTimestamp::new(0, 1),
    ));
    assert!(matches!(store.reset(dangling).await, Err(VcsError::ValidationFailed(message)) if message.contains("missing")), "a transition naming an unknown operation cannot be folded");
    assert_eq!(store.generation(), generation, "failed reset must preserve the live store");
}

#[semio_framework_async_macros::async_test]
async fn attach_folds_a_pushed_event_log() {
    let (channel, remote) = ChannelBackbone::pair("chan").await;
    let mut seed_store = fresh_demo_store().await;
    test_support::dispatch_test_command(&mut seed_store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut seed_store, ArtifactCommand::CommitCheckpoint { message: Some("seed".into()), authors: Vec::new() }).await.expect("commit");
    remote.push(BackboneMessage::Genesis { pack: seed_store.envelope().vcs.genesis.pack().to_vec() }).await.expect("push genesis");
    remote.push(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&seed_store.event_log().expect("seed log")) }).await.expect("push events");

    let mut store = fresh_demo_store().await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach folds the pushed events");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(5), "the pushed edit is projected");
    assert_eq!(store.current_checkpoint_id().await, seed_store.current_checkpoint_id().await, "the pushed commit is projected");
}

#[semio_framework_async_macros::async_test]
async fn channel_backbone_round_trips_between_store_and_actor() {
    let (channel, remote) = ChannelBackbone::pair("chan").await;
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let attach_flush = drain_channel_for_test(&remote).expect("drain attach");
    assert!(matches!(attach_flush.as_slice(), [BackboneMessage::Genesis { .. }]), "attach announces the genesis of an event-free document and nothing else: {attach_flush:?}");

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], transaction: None }).await.expect("apply");
    let outbound = drain_channel_for_test(&remote).expect("drain apply");
    assert!(outbound.iter().any(|message| matches!(message, BackboneMessage::Mutations { .. })), "a local apply is sent outbound as mutations: {outbound:?}");

    remote.push(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&[foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 8 })).await]) }).await.expect("push inbound operations");
    store.tick().await.expect("tick");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(8), "store ingests the actor's inbound operations");
}

#[semio_framework_async_macros::async_test]
async fn channel_backbone_remote_pops_one_exact_owner_in_fifo_order_and_preserves_contention() {
    let (mut channel, remote) = ChannelBackbone::pair("chan").await;
    channel.send(BackboneMessage::Ack { op_ids: vec!["first".into()] }).await.expect("queue first exact owner");
    channel.send(BackboneMessage::Ack { op_ids: vec!["second".into()] }).await.expect("queue second exact owner");
    let guard = remote.outbound.lock().expect("hold outbound authority for contention fixture");
    assert!(matches!(remote.try_pop_front(), Err(VcsError::Backbone(message)) if message.contains("contended")));
    assert_eq!(guard.len(), 2, "contention leaves both exact owners untouched");
    drop(guard);
    assert!(matches!(remote.try_pop_front().expect("first bounded pop"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["first"]));
    assert!(matches!(remote.try_pop_front().expect("second bounded pop"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["second"]));
    assert!(remote.try_pop_front().expect("terminal bounded pop").is_none());
}

#[semio_framework_async_macros::async_test]
async fn pump_acks_ingested_operations() {
    let (channel, remote) = ChannelBackbone::pair("chan").await;
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let _ = drain_channel_for_test(&remote).expect("drain attach snapshot");

    let inbound = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 7 })).await;
    let mutation_id = inbound.mutation_id.0.clone();
    remote.push(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&[inbound]) }).await.expect("push inbound operations");
    store.tick().await.expect("tick");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(7), "ingested the inbound operation");

    let outbound = drain_channel_for_test(&remote).expect("drain ack");
    assert!(outbound.iter().any(|message| matches!(message, BackboneMessage::Ack { op_ids } if op_ids == &vec![mutation_id.clone()])), "successful operations ingest emits an Ack for the ingested operation ids: {outbound:?}");
}

#[semio_framework_async_macros::async_test]
async fn exact_base_only_undo_refuses_a_foreign_tail() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("local apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::IngestRemote { envelope: foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 2 })).await }).await.expect("ingest foreign");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "foreign edit sits at the tail");

    let error = test_support::dispatch_test_command(&mut store, ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::ExactBaseOnly, semantic_command: None }).await.expect_err("undo must refuse a foreign tail");
    assert!(matches!(error, VcsError::ForeignEdit(_)), "got {error:?}");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "the timeline is untouched after refusal");
}

#[semio_framework_async_macros::async_test]
async fn transform_against_concurrent_undo_skips_over_a_foreign_tail() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("local apply");
    let local_edit_id = store.applied_edit_ids()[0].clone();
    let foreign = foreign_mutation_envelope("peer", DemoMutation::SetN(SetN { n: 2 })).await;
    let foreign_id = foreign.mutation_id.0.clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::IngestRemote { envelope: foreign }).await.expect("ingest foreign");
    assert_eq!(store.applied_edit_ids().len(), 2, "local + foreign are both applied");

    test_support::dispatch_test_command(&mut store, ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::TransformAgainstConcurrent, semantic_command: None }).await.expect("transform undo removes the local edit from mid-timeline");
    assert_eq!(store.applied_edit_ids(), std::slice::from_ref(&foreign_id), "only the local edit is removed; the concurrent foreign edit stays applied");
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&local_edit_id), "the local edit is on the redo stack");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "snapshot re-materializes from the foreign edit alone");

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo brings the local edit back");
    assert_eq!(store.applied_edit_ids(), &[local_edit_id, foreign_id], "redo reinstates the local edit at its own HLC position, before the later concurrent edit — where every replica folds it");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "the later concurrent edit still wins");
}

/// ↩️ Builds on the existing Revert/Reinstate ledger (S11/S12 history_row_applied_v1): plain Undo is
/// selective; Pack/SPR reload folds the same cursor with the supplied opened actor, so Redo
/// reinstates this author's entry from the shared redo stack.
#[semio_framework_async_macros::async_test]
async fn plain_undo_is_selective_and_durable_across_event_log_reload() {
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("local a1");
    let local_a1 = store.applied_edit_ids()[0].clone();
    let author = store.local_actor_id().0.clone();
    let foreign = foreign_mutation_envelope("author-b", DemoMutation::SetN(SetN { n: 2 })).await;
    let foreign_id = foreign.mutation_id.0.clone();
    test_support::dispatch_test_command(&mut store, ArtifactCommand::IngestRemote { envelope: foreign }).await.expect("ingest foreign");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }).await.expect("local a2");
    let local_a2 = store.applied_edit_ids().iter().find(|id| **id != local_a1 && **id != foreign_id).expect("a2").clone();

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("plain undo skips foreign and undoes local a2");
    assert_eq!(store.applied_edit_ids(), &[local_a1.clone(), foreign_id.clone()]);
    assert_eq!(store.redo_edit_ids(), std::slice::from_ref(&local_a2));

    let files = print_document_pack(store.envelope()).await.expect("print pack");
    let parsed = parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse pack");
    let mut reloaded = ArtifactStore::new_with_actor(parsed.envelope, ActorId(author), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.applied_edit_ids(), &[local_a1.clone(), foreign_id.clone()], "reload preserves selective undo projection");
    assert_eq!(reloaded.redo_edit_ids(), std::slice::from_ref(&local_a2), "durable collaborative redo stack survives reload");
    test_support::dispatch_test_command(&mut reloaded, ArtifactCommand::Redo).await.expect("redo after reload reinstates the local edit");
    assert_eq!(reloaded.applied_edit_ids(), &[local_a1, foreign_id, local_a2]);
    assert!(reloaded.redo_edit_ids().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn compensating_undo_dispatches_semantic_command() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None }).await.expect("apply");
    let undo_apply = ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 0 })], transaction: None };
    test_support::dispatch_test_command(&mut store, ArtifactCommand::UndoWithPolicy { policy: UndoPolicy::CompensatingAction, semantic_command: Some(Box::new(undo_apply)) }).await.expect("compensating undo");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(0));
}

#[semio_framework_async_macros::async_test]
async fn edit_mutations_exposes_the_latest_edit() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert!(store.edit_mutations().is_none(), "no edits yet");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None }).await.expect("apply");
    let (forwards, inverse, meta) = store.edit_mutations().expect("edit operations");
    assert_eq!(forwards, &[DemoMutation::SetN(SetN { n: 5 })]);
    assert_eq!(inverse.iter().cloned().collect::<Vec<_>>(), vec![DemoMutation::AssignN(AssignN { n: Some(0) })], "inverse restores the pre-state");
    assert_eq!(meta.len(), 1);
}

/// 🪢️ A streamed tool transaction (design §15) grows ONE open edit tick by tick without re-replaying history: after 50 ticks
/// and the commit the edit carries every forward, inverse and metadata row, and one undo reverts the whole gesture.
#[semio_framework_async_macros::async_test]
async fn a_streamed_transaction_grows_one_edit_over_many_ticks() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let transaction = semio_framework_os_spr::TransactionRef { id: "tx-00000000000000d1".into(), tool: "s.demo@1/*#editor#drag".into() };
    for n in 1..=50 {
        test_support::dispatch_test_command(&mut store, ArtifactCommand::AppendTransaction { mutations: vec![DemoMutation::SetN(SetN { n })], transaction: transaction.clone() }).await.expect("tick");
    }
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitTransaction { transaction_id: transaction.id.clone() }).await.expect("commit");
    assert_eq!(store.envelope().vcs.edits.len(), 1, "one gesture is one edit");
    let edit = store.envelope().vcs.edits.last().expect("edit");
    assert_eq!((edit.forwards.len(), edit.inverse.len(), edit.mutation_meta.len()), (50, 50, 50));
    assert_eq!(store.snapshot().expect("snapshot").n, Some(50));
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot after undo").n, Some(0), "one undo reverts the whole 50-tick gesture");
}

/// 🔏️ LAW (ticket 26/09/23 C12): a streamed edit's revision identity is a pure function of the edit. Each tick extends the
/// tail's digest chains by only the operations it appended, yet after a long run the live revision equals the revision a
/// store loaded from the same envelope derives from scratch, and every tick still moves the revision.
#[semio_framework_async_macros::async_test]
async fn a_streamed_edit_revision_is_its_own_from_scratch_digest() {
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let transaction = semio_framework_os_spr::TransactionRef { id: "tx-00000000000000d2".into(), tool: "s.demo@1/*#editor#type".into() };
    let mut revisions = std::collections::HashSet::new();
    for n in 1..=256 {
        test_support::dispatch_test_command(&mut store, ArtifactCommand::AppendTransaction { mutations: vec![DemoMutation::SetN(SetN { n })], transaction: transaction.clone() }).await.expect("tick");
        assert!(revisions.insert(store.content_revision_now()), "tick {n} moves the revision");
    }
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitTransaction { transaction_id: transaction.id.clone() }).await.expect("commit");
    assert_eq!(store.envelope().vcs.edits.len(), 1, "the run is one edit");
    let files = print_document_pack(store.envelope()).await.expect("owned document encode");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("owned document decode").envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(reloaded.content_revision_now(), store.content_revision_now(), "the incrementally extended revision equals the from-scratch one");
}

#[semio_framework_async_macros::async_test]
async fn assert_subset_harness_fidelity_and_inference_helpers() {
    test_support::assert_import_export_fidelity_bytes(b"fixture", b"fixture", test_support::IoFidelityClass::Exact).await;
    test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Canonical).await;
    test_support::assert_inference_determinism(&7_i32, &7_i32).await;
}

#[semio_framework_async_macros::async_test]
#[should_panic(expected = "exact fidelity requires byte-identical export")]
async fn assert_import_export_fidelity_bytes_exact_rejects_divergence() {
    test_support::assert_import_export_fidelity_bytes(b"fixture", b"other", test_support::IoFidelityClass::Exact).await;
}

#[semio_framework_async_macros::async_test]
async fn test_support_round_trip_helpers_pass_for_demo_operation() {
    test_support::assert_operation_round_trip(&DemoSnapshot { n: Some(4) }, DemoMutation::SetN(SetN { n: 9 })).await;
    test_support::assert_store_roundtrip(DemoSnapshot { n: Some(4) }, DemoMutation::SetN(SetN { n: 9 })).await;

    let edit = Edit::<DemoMutation> {
        line: None,
        id: "edit-command-envelope".into(),
        actor: Some("actor-fallback".into()),
        forwards: vec![DemoMutation::SetN(SetN { n: 9 })],
        inverse: vec![DemoMutation::SetN(SetN { n: 4 })].into(),
        mutation_meta: vec![MutationMeta {
            mutation_id: Some(MutationId("op-a".into())),
            dependencies: vec![MutationId("op-0".into())],
            base_version: 0,
            author_id: Some(ActorId("actor-explicit".into())),
            timestamp: HybridLogicalTimestamp::new(1, 1000),
            undo_policy: UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
            transaction: None,
        }],
        verb: None,
        sequence_number: 1,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    test_support::assert_command_envelope_round_trip::<DemoSnapshot, DemoMutation>(&edit, &ArtifactId("doc-command-envelope".into()), &SchemaId("demo/v1".into())).await;
}

/// 🪤️ Proves `assert_command_envelope_round_trip` is not a trivially-true check: a hand-rolled
/// `Mutation` whose `Deserialize` impl silently drops its own field (encodes `n` faithfully but
/// always decodes to `n: 0`) must trip law (2) of the doc comment on
/// `assert_command_envelope_round_trip` — the same "deliberately lossy impl" pattern
/// `protocol_testkit`'s `op_text_round_trip_panics_on_a_lossy_impl` uses for `assert_op_text_round_trip`.
#[semio_framework_async_macros::async_test]
#[should_panic(expected = "did not decode back into an equal forward operation")]
async fn command_envelope_round_trip_panics_on_a_lossy_operation() {
    let edit = Edit::<LossyMutation> {
        line: None,
        id: "edit-lossy".into(),
        actor: None,
        forwards: vec![LossyMutation::SetN(LossySetN { n: 7 })],
        inverse: vec![].into(),
        mutation_meta: vec![],
        verb: None,
        sequence_number: 0,
        started_at: "2026-07-27T00:00:00Z".into(),
        finished_at: None,
    };
    test_support::assert_command_envelope_round_trip::<DemoSnapshot, LossyMutation>(&edit, &ArtifactId("doc-lossy".into()), &SchemaId("lossy/v1".into())).await;
}

// `DemoSnapshot`'s `pack::snapshot::ArtifactDsl` impl and `DemoMutation`'s `crate::os_store::OpText` impl are now
// generated by `#[derive(crate::os_dsl::DslArtifact)]`/`#[derive(crate::os_dsl::DslOps)]` on the type definitions
// themselves (see `DemoSnapshot`/`DemoMutation` above) — the `dsl_schema` grammar replaces
// this crate's own hand-rolled `"n <value>"`/`"set-n <value>"` printer/parser.

#[semio_framework_async_macros::async_test]
async fn demo_dsl_round_trips() {
    test_support::assert_dsl_round_trip(&DemoSnapshot { n: Some(42) });
}

#[semio_framework_async_macros::async_test]
async fn demo_dsl_pack_equivalence() {
    test_support::assert_dsl_pack_equivalence(&DemoSnapshot { n: Some(42) });
}

#[semio_framework_async_macros::async_test]
async fn demo_op_text_round_trips() {
    test_support::assert_op_line_round_trip(&DemoMutation::SetN(SetN { n: 7 }));
}

#[semio_framework_async_macros::async_test]
async fn demo_op_binary_round_trips_and_matches_text() {
    let operation = DemoMutation::SetN(SetN { n: 7 });
    let encoded = operation.encode_op().expect("op encode");
    let encoded_again = operation.encode_op().expect("op re-encode");
    assert_eq!(encoded, encoded_again, "op binary encoding must be deterministic");
    assert_eq!(encoded[0], pack_rt::OP_BINARY_FORMAT);
    let decoded = DemoMutation::decode_op(&encoded).expect("op decode");
    assert_eq!(decoded, operation);
    let via_text = DemoMutation::parse_op(&operation.print_op()).expect("op parse");
    assert_eq!(via_text, decoded, "binary and text round trips diverged");
}

#[semio_framework_async_macros::async_test]
async fn demo_op_binary_rejects_unknown_format_and_ordinal() {
    let operation = DemoMutation::SetN(SetN { n: 7 });
    let mut wrong_format = operation.encode_op().expect("op encode");
    wrong_format[0] = 9;
    assert!(DemoMutation::decode_op(&wrong_format).is_err(), "format 9 must be rejected");
    let out_of_range = [pack_rt::OP_BINARY_FORMAT, 0x7E];
    assert!(DemoMutation::decode_op(&out_of_range).is_err(), "ordinal beyond declared variants must be rejected");
}

#[semio_framework_async_macros::async_test]
async fn print_edit_lines_emits_one_indented_line_per_forward_op() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply");
    let edit = store.envelope().vcs.edits.last().expect("edit");
    let printed = print_edit_lines(edit).await.expect("print edit lines");
    assert!(printed.starts_with("edit "), "got {printed:?}");
    assert!(printed.contains("\n  set-n n=1\n"));
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_after_apply_and_checkpoint() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }).await.expect("apply");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).await.expect("commit");
    test_support::assert_document_text_round_trip(&store).await;
    test_support::assert_document_pack_round_trip(&store).await;
}

#[semio_framework_async_macros::async_test]
async fn parse_document_text_rejects_invalid_op_line_with_span() {
    let files = ArtifactTextFiles { dsl: "n=0\n".to_string(), ops: "doc demo schema=demo/v1\nedit e1 sequence=1 started=\"1\" line=[]\n  not-an-op\n".to_string() };
    let error = parse_document_text::<DemoSnapshot, DemoMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_err();
    assert_eq!(error.span.line, 3);
}

/// 🩺️ Stresses the stateful `current`/`tail_undo_cache` fast paths — multi-op edits, streamed
/// transactions, undo/redo, and a checkpoint (cold-path recompute) all interleaved — against the
/// full-replay differential oracle, so any divergence between the incremental paths and a
/// from-scratch replay fails loudly here rather than surfacing as a silent snapshot bug later.
///
/// Multi-operation edit: current must fold both ops, matching a from-scratch replay.
///
/// Streamed gesture: its first tick opens a NEW edit after the `Apply`-created one and its second tick grows that open
/// edit — two edits total, the second carrying both ticks (3 then 4) once the commit closes it.
///
/// Undo the whole streamed edit (O(1) tail-cache path) restores the `Apply`-edit's state, not
/// the initial snapshot — only the gesture's edit is undone here.
///
/// Checkpoint (cold path through `checkout_checkpoint_internal` is NOT exercised by commit
/// itself, but a following apply + a second, older undo still must agree with replay).
#[semio_framework_async_macros::async_test]
async fn stateful_current_matches_full_replay_across_interleaved_commands() {
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 }), DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply multi-op edit");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));

    let transaction = semio_framework_os_spr::TransactionRef { id: "tx-00000000000000d3".into(), tool: "s.demo@1/*#editor#drag".into() };
    test_support::dispatch_test_command(&mut store, ArtifactCommand::AppendTransaction { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: transaction.clone() }).await.expect("tick 1");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::AppendTransaction { mutations: vec![DemoMutation::SetN(SetN { n: 4 })], transaction: transaction.clone() }).await.expect("tick 2");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitTransaction { transaction_id: transaction.id }).await.expect("commit");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(4));
    assert_eq!(store.envelope().vcs.edits.len(), 2, "the streamed gesture started its own edit, not a third");

    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2));
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Redo).await.expect("redo");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(4));

    test_support::dispatch_test_command(&mut store, ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }).await.expect("commit");
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None }).await.expect("apply after checkpoint");
    test_support::assert_live_equals_replay(&store).await;
    test_support::dispatch_test_command(&mut store, ArtifactCommand::Undo).await.expect("undo after checkpoint");
    test_support::assert_live_equals_replay(&store).await;
    assert_eq!(store.snapshot().expect("snapshot").n, Some(4));
}



/// 🪄️ Downcasts a registered `dyn SpaceMember` back to its concrete demo store.
async fn demo_member<'a, Mutation: self::Mutation<DemoSnapshot> + 'static, M: SpaceMember + 'static>(host: &'a mut SpaceHost<M>, document_id: &str) -> &'a mut ArtifactStore<DemoSnapshot, Mutation> {
    host.member_mut(document_id).await.expect("member registered").as_any_mut().await.downcast_mut::<ArtifactStore<DemoSnapshot, Mutation>>().expect("concrete member type matches")
}

/// 🎯️ Every member below gets at least one uncommitted edit (dirty), mirroring
/// `space_checkpoint_commits_dirty_members_and_pins_their_checkpoints`'s `member_a` — a fresh
/// member with zero edits and zero checkpoints has no `current_checkpoint_id` yet, which
/// `commit_space_checkpoint` requires of every registered member (dirty ones are auto-committed,
/// already-clean ones just need a prior checkpoint).
#[semio_framework_async_macros::async_test]
async fn register_space_documents_registers_manifest_collections_and_artifacts_together() {
    let mut manifest = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "space-manifest", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut manifest, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }).await.expect("apply manifest edit");
    let mut collection_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "collection-a", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut collection_a, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }).await.expect("apply collection a edit");
    let mut collection_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "collection-b", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut collection_b, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }).await.expect("apply collection b edit");
    let mut artifact_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "artifact-a", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    test_support::dispatch_test_command(&mut artifact_a, ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }).await.expect("apply artifact edit");

    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid space host history");
    host.register_space_documents(manifest, vec![collection_a, collection_b], vec![artifact_a]).await;

    assert!(host.member("space-manifest").await.is_some(), "manifest registered");
    assert!(host.member("collection-a").await.is_some(), "collection a registered");
    assert!(host.member("collection-b").await.is_some(), "collection b registered");
    assert!(host.member("artifact-a").await.is_some(), "artifact registered");

    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
        assert!(progress.owned_bytes <= IDENTITY_CEILING);
        true
    };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut observer).expect("declared fixture identity authority");
    let space_checkpoint_id = host.commit_space_checkpoint("initial space checkpoint".into(), Vec::new(), &mut identity).await.expect("commit space checkpoint");
    drop(identity.pause().expect("original cumulative fixture identity receipt"));
    let snapshot = host.meta_snapshot().await.expect("meta snapshot");
    let checkpoint = snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == space_checkpoint_id).expect("checkpoint recorded");
    assert_eq!(checkpoint.members.len(), 4, "manifest + 2 collections + 1 artifact all pinned atomically in one space checkpoint");
    let pinned_ids: HashSet<&str> = checkpoint.members.iter().map(|pin| pin.document_id.as_str()).collect();
    assert_eq!(pinned_ids, HashSet::from(["space-manifest", "collection-a", "collection-b", "artifact-a"]));
}

#[semio_framework_async_macros::async_test]
async fn space_checkpoint_commits_dirty_members_and_pins_their_checkpoints() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    member_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply a");

    let mut member_b = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-b", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    member_b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None }, &mut identity).await.expect("apply b");
    member_b.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b-init".into()), authors: Vec::new() }, &mut identity).await.expect("commit b upfront, so it starts clean");
    let member_b_checkpoint = member_b.current_checkpoint_id().await.expect("b checkpoint").to_string();

    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid space host history");
    host.register_member(member_a).await;
    host.register_member(member_b).await;

    let space_checkpoint_id = host.commit_space_checkpoint("studio init".into(), vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }], &mut identity).await.expect("commit space checkpoint");

    let snapshot = host.meta_snapshot().await.expect("meta snapshot");
    assert_eq!(snapshot.checkpoints.len(), 1);
    let checkpoint = &snapshot.checkpoints[0];
    assert_eq!(checkpoint.id, space_checkpoint_id);
    assert_eq!(checkpoint.members.len(), 2, "pins one entry per registered member");
    let pin_b = checkpoint.members.iter().find(|pin| pin.document_id == "member-b").expect("pin b");
    assert_eq!(pin_b.checkpoint_id, member_b_checkpoint, "clean member reuses its existing checkpoint");
    assert!(!host.member("member-a").await.expect("member a").is_dirty().await, "dirty member-a is committed (and therefore clean) by the space checkpoint");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn space_vcs_host_meta_document_is_backbone_attachable_and_detachable() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let (backbone_a, backbone_b) = MemoryBackbone::pair("studio-a", "studio-b").await;
    let mut host_a = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid first space host history");
    let mut host_b = SpaceHost::<ArtifactStore<DemoSnapshot, DemoMutation>>::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid second space host history");
    assert!(host_a.backbone_ref().await.is_none(), "default is unattached, like any other ArtifactStore");

    host_a.attach_backbone(Backbones::Memory(backbone_a)).await.expect("attach a");
    host_b.attach_backbone(Backbones::Memory(backbone_b)).await.expect("attach b");
    assert!(host_a.backbone_ref().await.is_some());

    let mut member = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    member.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply on member, so it's dirty and can be committed");
    host_a.register_member(member).await;
    host_a.commit_space_checkpoint("studio init".into(), Vec::new(), &mut identity).await.expect("commit space checkpoint on a");

    host_b.tick().await.expect("tick b");
    assert_eq!(host_b.meta_snapshot().await.expect("meta snapshot b").checkpoints.len(), 1, "the space-wide checkpoint replicates through the meta-document's backbone");

    host_a.detach_backbone().await.expect("detach source host backbone");
    assert!(host_a.backbone_ref().await.is_none());
    host_a.commit_space_checkpoint("studio offline".into(), Vec::new(), &mut identity).await.expect("meta history keeps working purely in memory once detached");
    host_b.tick().await.expect("tick b again");
    assert_eq!(host_b.meta_snapshot().await.expect("meta snapshot b unchanged").checkpoints.len(), 1, "detached space edits never reach the peer");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn space_checkout_checkpoint_fans_out_and_restores_pinned_member_state() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid space host history");
    host.register_member(member_a).await;

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply 1");
    let space_checkpoint_1 = host.commit_space_checkpoint("first".into(), Vec::new(), &mut identity).await.expect("commit 1");

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply 2");
    host.commit_space_checkpoint("second".into(), Vec::new(), &mut identity).await.expect("commit 2");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(2), "member reflects the second space checkpoint before checking out the first");

    host.checkout_space_checkpoint(&space_checkpoint_1, &mut identity).await.expect("checkout space checkpoint 1");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(1), "checking out the first space checkpoint fans out and restores member-a's pinned state");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn space_switch_alternative_fans_out_and_restores_pinned_member_state() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let member_a = ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "member-a", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid space host history");
    host.register_member(member_a).await;

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply 1");
    host.commit_space_checkpoint("root".into(), Vec::new(), &mut identity).await.expect("commit root");

    let alt_id = host.create_space_alternative("branch-a".into(), &mut identity).await.expect("create alternative");

    demo_member::<DemoMutation, _>(&mut host, "member-a").await.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply 2 (uncommitted at the studio level)");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(2), "uncommitted edit is live before switching");

    host.switch_space_alternative(&alt_id, &mut identity).await.expect("switch alternative fans out to its pinned checkpoint");
    assert_eq!(demo_member::<DemoMutation, _>(&mut host, "member-a").await.snapshot().expect("snapshot").n, Some(1), "switching alternatives restores each member to its pinned checkpoint, discarding the uncommitted edit");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn space_undo_and_redo_target_the_member_with_the_most_recent_local_edit_by_hlt() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut member_early = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-early", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    member_early.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN(TimestampedSetN { n: 1, physical_ms: 1_000 })], transaction: None }, &mut identity).await.expect("apply early");

    let mut member_late = ArtifactStore::new(create_document_envelope::<DemoSnapshot, TimestampedMutation>("demo-ts/v1", "member-late", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    member_late.dispatch(ArtifactCommand::Apply { mutations: vec![TimestampedMutation::SetN(TimestampedSetN { n: 9, physical_ms: 2_000 })], transaction: None }, &mut identity).await.expect("apply late");

    let mut host = SpaceHost::new(create_document_envelope(&format!("{S_SPACE_HISTORY_SCHEMA}/v1"), "studio", SpaceHistorySnapshot::default(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.expect("valid space host history");
    host.register_member(member_early).await;
    host.register_member(member_late).await;

    host.undo(&mut identity).await.expect("space undo targets the member with the higher HLT");
    assert_eq!(demo_member::<TimestampedMutation, _>(&mut host, "member-early").await.snapshot().expect("early snapshot").n, Some(1), "earlier local edit (lower HLT) is untouched");
    assert_eq!(demo_member::<TimestampedMutation, _>(&mut host, "member-late").await.snapshot().expect("late snapshot").n, Some(0), "later local edit (higher HLT) is the one undone");

    host.redo(&mut identity).await.expect("studio redo targets the most recently undone edit");
    assert_eq!(demo_member::<TimestampedMutation, _>(&mut host, "member-late").await.snapshot().expect("late snapshot after redo").n, Some(9), "redo restores the member's most recently undone edit");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
// 🎞️ `Mutation::reconcile`/`reconcile_with_last`/`snapshot_with_conflicts` are all deleted
// (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C4/C6/C10) — concurrent-
// merge arbitration is now `ingest_remote`/`resolve_conflict` against first-class `Conflict`s.
async fn snapshot_matches_materialize_and_conflicts_stay_empty_absent_remote_ingestion() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply");
    let replayed = materialize_document_snapshot(store.envelope(), store.applied_edit_ids()).await.expect("replay");
    assert_eq!(replayed.n, Some(3));
    assert_eq!(store.snapshot().expect("snapshot").n, Some(3));
    assert!(store.conflicts().is_empty(), "no remote ingestion happened, so the store's conflict buffer stays empty");
    assert!(store.open_conflicts().next().is_none());
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn space_history_op_round_trips() {
    let checkpoint = SpaceCheckpoint {
        id: "sc-1".into(),
        parent_id: None,
        message: "root".into(),
        authors: Vec::new(),
        timestamp: HybridLogicalTimestamp::new(0, 1),
        members: vec![SpaceMemberPin { document_id: "member-a".into(), checkpoint_id: "cp-1".into(), alternative_id: String::new() }],
    };
    test_support::assert_operation_round_trip(&SpaceHistorySnapshot::default(), SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone() })).await;

    let with_checkpoint = SpaceHistorySnapshot { checkpoints: vec![checkpoint], alternatives: Vec::new(), active_alternative_id: None };
    let alternative = SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] };
    test_support::assert_operation_round_trip(&with_checkpoint, SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative })).await;

    let with_alternative_active = SpaceHistorySnapshot {
        alternatives: vec![SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] }, SpaceAlternative { id: "sa-other".into(), name: "other".into(), checkpoint_ids: vec!["sc-1".into()] }],
        active_alternative_id: Some("sa-1".into()),
        ..with_checkpoint.clone()
    };
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: "sa-other".into() })).await;
    test_support::assert_operation_round_trip(&with_checkpoint, SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: "sc-1".into() })).await;
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "sa-other".into() })).await;
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "sa-1".into() })).await;
    test_support::assert_operation_round_trip(&with_alternative_active, SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: None })).await;
}

#[semio_framework_async_macros::async_test]
async fn demo_fixture_inverse_diffs_sum_to_the_negative_diff() {
    use semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law as law;
    let base = DemoSnapshot { n: Some(3) };
    law(&DemoMutation::AddN(AddN { delta: 4 }), &base).await;
    law(&DemoMutation::SetN(SetN { n: 9 }), &base).await;
    law(&DemoMutation::DeleteN(DeleteN {}), &base).await;
    law(&DemoMutation::AssignN(AssignN { n: None }), &base).await;
    law(&DemoMutation::AssignN(AssignN { n: Some(8) }), &DemoSnapshot { n: None }).await;
}

#[semio_framework_async_macros::async_test]
async fn space_history_inverse_diffs_sum_to_the_negative_diff() {
    use semio_framework_os_spr::protocol_laws::assert_mutation_inverse_sum_law as law;
    let checkpoint = SpaceCheckpoint {
        id: "sc-1".into(),
        parent_id: None,
        message: "root".into(),
        authors: Vec::new(),
        timestamp: HybridLogicalTimestamp::new(0, 1),
        members: vec![SpaceMemberPin { document_id: "member-a".into(), checkpoint_id: "cp-1".into(), alternative_id: String::new() }],
    };
    law(&SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint: checkpoint.clone() }), &SpaceHistorySnapshot::default()).await;
    let with_checkpoint = SpaceHistorySnapshot { checkpoints: vec![checkpoint], alternatives: Vec::new(), active_alternative_id: None };
    let alternative = SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] };
    law(&SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative }), &with_checkpoint).await;
    let with_alternative_active = SpaceHistorySnapshot {
        alternatives: vec![SpaceAlternative { id: "sa-1".into(), name: "branch".into(), checkpoint_ids: vec!["sc-1".into()] }, SpaceAlternative { id: "sa-other".into(), name: "other".into(), checkpoint_ids: vec!["sc-1".into()] }],
        active_alternative_id: Some("sa-1".into()),
        ..with_checkpoint.clone()
    };
    law(&SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: "sa-other".into() }), &with_alternative_active).await;
    law(&SpaceHistoryMutation::RemoveSpaceCheckpoint(RemoveSpaceCheckpoint { checkpoint_id: "sc-1".into() }), &with_checkpoint).await;
    law(&SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "sa-other".into() }), &with_alternative_active).await;
    law(&SpaceHistoryMutation::RemoveSpaceAlternative(RemoveSpaceAlternative { alternative_id: "sa-1".into() }), &with_alternative_active).await;
    law(&SpaceHistoryMutation::SetActiveSpaceAlternative(SetActiveSpaceAlternative { alternative_id: None }), &with_alternative_active).await;
}


/// 🛰️ Builds an explicitly stamped Severity envelope for policy and quarantine law tests.
fn severity_mutation_envelope_at(document_id: &str, actor: &str, mutation_id: &str, operation: SeverityMutation, timestamp: HybridLogicalTimestamp) -> semio_framework_os_spr::MutationEnvelope {
    semio_framework_os_spr::MutationEnvelope {
        mutation_id: MutationId(mutation_id.to_string()),
        document_id: ArtifactId(document_id.to_string()),
        actor: ActorId(actor.to_string().into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: SchemaId("demo/v1".to_string()), payload: operation.encode_op().expect("encode severity mutation") },
        inverse: semio_framework_os_spr::InverseMutation { schema: SchemaId("demo/v1".to_string()), payload: Vec::new() },
        timestamp,
        transaction: None,
        verb: None,
        line: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn document_text_round_trips_authoritative_metadata_messages_conflicts_and_cursor() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "severity-text", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], transaction: None }, &mut identity).await.expect("apply warning");
    let edit = store.envelope().vcs.edits.last().expect("one durable edit").clone();
    let messages = store.envelope().edit_messages.last().expect("durable outcome ledger").messages.clone();
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
    let mutation_ids = stable_mutation_ids_for_edit(&edit).await.expect("stable operation identity");
    let timestamp = edit.mutation_meta.first().expect("metadata timestamp").timestamp;
    let document_id = ArtifactId(store.envelope().id.clone());
    store.0.envelope.conflicts.push(semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages,
        actors: vec![ActorId(edit.actor.clone().expect("edit actor"))],
        timestamp,
    });
    let files = print_document_text(store.envelope()).await.expect("print full-fidelity text");
    for record in ["inverse ", "metadata ", "message ", "conflict "] {
        assert!(files.ops.lines().any(|line| line.starts_with(record)), "missing {record:?} record: {}", files.ops);
    }
    let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse full-fidelity text");
    assert_eq!(&parsed.envelope, store.envelope());
    assert_eq!(parsed.snapshot, store.snapshot().expect("live snapshot"));
    test_support::retire_parsed_document(parsed);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🔬️ `print_ops_log`'s `metadata `/`message `/`conflict ` records now build their `data`
/// payload with `crate::os_pack::json::to_json_string` instead of `serde_json::to_string`
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02) — direct proof the two
/// encode the same ordered first-party values byte-identically, exercising both the
/// sparse (every `Option`/default field omitted) and dense (every optional field populated,
/// including `MutationOrigin`'s two non-default enum-variant-with-fields shapes) ends of each
/// type's `skip_serializing_if` surface.
#[test]
fn ops_log_records_to_json_string_match_serde_json_byte_for_byte() {
    let sparse_meta = MutationMeta {
        mutation_id: None,
        dependencies: Vec::new(),
        base_version: 1,
        author_id: None,
        timestamp: HybridLogicalTimestamp::new(1, 100),
        undo_policy: UndoPolicy::ExactBaseOnly,
        payload_hash: None,
        semantic_kind: None,
        label: None,
        group_id: None,
        origin: semio_framework_os_spr::MutationOrigin::Owner,
        transaction: None,
    };
    let dense_meta = MutationMeta {
        mutation_id: Some(MutationId("mutation-1".into())),
        dependencies: vec![MutationId("mutation-0".into())],
        base_version: 7,
        author_id: Some(ActorId("actor-1".into())),
        timestamp: HybridLogicalTimestamp::new(9, 300),
        undo_policy: UndoPolicy::TransformAgainstConcurrent,
        payload_hash: Some(semio_framework_os_spr::PayloadHash([7u8; 32])),
        semantic_kind: Some(SchemaId("demo/v1#set-n".into())),
        label: Some("set n".into()),
        group_id: Some("group-1".into()),
        origin: semio_framework_os_spr::MutationOrigin::Contributed { plugin_id: "plugin-a".into(), mutation_id: SchemaId("mutation-src".into()), payload_hash: semio_framework_os_spr::PayloadHash([3u8; 32]) },
        transaction: None,
    };
    let transaction_meta =
        MutationMeta { origin: semio_framework_os_spr::MutationOrigin::Transaction { initiator: semio_framework_os_spr::ForeignTarget { artifact_id: "artifact-1".into(), artifact_kind: "demo".into(), dialect: Some("v1".into()) } }, ..dense_meta.clone() };
    for meta in [&sparse_meta, &dense_meta, &transaction_meta] {
        let mine = semio_framework_pack_json::to_json_string(meta);
        let theirs = serde_json::to_string(&test_support::SerdeValue(&meta.to_value())).expect("serde_json encodes MutationMeta fields");
        assert_eq!(mine, theirs, "MutationMeta's ToValue/pack::json bridge diverged from serde_json for origin={:?}", meta.origin);
    }

    let sparse_message = semio_framework_os_spr::MutationMessage { level: semio_framework_diagnostic::Severity::Warning, code: semio_framework_diagnostic::FaultCode("mutation.no-op".into()), message: "no-op".into(), target: Vec::new(), op_index: None };
    let dense_message = semio_framework_os_spr::MutationMessage {
        level: semio_framework_diagnostic::Severity::Fatal,
        code: semio_framework_diagnostic::FaultCode("mutation.invariant".into()),
        message: "n invariant violated".into(),
        target: vec!["n".into()],
        op_index: Some(2),
    };
    for message in [&sparse_message, &dense_message] {
        let mine = semio_framework_pack_json::to_json_string(message);
        let theirs = serde_json::to_string(&test_support::SerdeValue(&message.to_value())).expect("serde_json encodes MutationMessage fields");
        assert_eq!(mine, theirs, "MutationMessage's ToValue/pack::json bridge diverged from serde_json for op_index={:?}", message.op_index);
    }

    let conflict = semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId("conflict-1".into()),
        kind: semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec!["edit-1".into(), "edit-2".into()] },
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: vec![sparse_message, dense_message],
        actors: vec![ActorId("actor-1".into()), ActorId("actor-2".into())],
        timestamp: HybridLogicalTimestamp::new(9, 300),
    };
    let mine = semio_framework_pack_json::to_json_string(&conflict);
    let theirs = serde_json::to_string(&test_support::SerdeValue(&conflict.to_value())).expect("serde_json encodes Conflict fields");
    assert_eq!(mine, theirs, "Conflict's ToValue/pack::json bridge diverged from serde_json");
}

#[semio_framework_async_macros::async_test]
async fn document_text_rejects_missing_metadata_and_dangling_transitions_without_synthesis() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-text", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], transaction: None }, &mut identity).await.expect("apply warning");
    let files = print_document_text(store.envelope()).await.expect("print strict text");
    let missing_metadata = files.ops.lines().filter(|line| !line.starts_with("metadata ")).collect::<Vec<_>>().join("\n");
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &missing_metadata, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("no metadata records")));

    let dangling = semio_framework_os_spr::history_transition_envelope(
        &semio_framework_os_spr::HistoryTransition::Revert { mutation_ids: vec![MutationId("unknown-edit".into())] },
        &ArtifactId("strict-text".into()),
        &ActorId("local".into()),
        Vec::new(),
        HybridLogicalTimestamp::new(0, 1),
    );
    let dangling_text = format!("{}\n{}", files.ops.trim_end(), ops_line_from_transition(&dangling).expect("transition line").print_op());
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &dangling_text, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("unknown operation unknown-edit")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn document_text_preserves_non_contiguous_edit_sequences_and_rejects_invalid_ones() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "strict-sequence", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetN(SeveritySetN { n: 1 })], transaction: None }, &mut identity).await.expect("first edit");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetN(SeveritySetN { n: 2 })], transaction: None }, &mut identity).await.expect("second edit");
    let mut renumbered = owned_test_envelope(&store).await;
    renumbered.vcs.edits[0].sequence_number = 4;
    renumbered.vcs.edits[1].sequence_number = 17;

    let files = print_document_text(&renumbered).await.expect("print strict text");
    retire_demo_envelope(renumbered);
    let parsed = parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &files.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse strict text").into_envelope();
    assert_eq!(parsed.vcs.edits.iter().map(|edit| edit.sequence_number).collect::<Vec<_>>(), vec![4, 17]);
    retire_demo_envelope(parsed);

    let invalid = files.ops.replacen("sequence=4", "sequence=-1", 1);
    assert!(matches!(parse_document_text::<DemoSnapshot, SeverityMutation>(&files.dsl, &invalid, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await, Err(error) if error.message.contains("invalid edit sequence -1")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn persisted_conflict_requires_a_global_owned_operation_index() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "conflict-index", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 3 })], transaction: None }, &mut identity).await.expect("apply warning");
    let edit = store.envelope().vcs.edits.last().expect("one edit").clone();
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit.id.clone()] };
    let mutation_ids = stable_mutation_ids_for_edit(&edit).await.expect("stable mutation ids");
    let timestamp = edit.mutation_meta.first().expect("timestamp").timestamp;
    let mut messages = store.envelope().edit_messages.last().expect("outcome message").messages.clone();
    messages[0].op_index = Some(1);
    let document_id = ArtifactId(store.envelope().id.clone());
    store.0.envelope.conflicts.push(semio_framework_os_spr::Conflict {
        id: semio_framework_os_spr::ConflictId::new(&kind, &document_id, &mutation_ids, &timestamp).await,
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages,
        actors: vec![ActorId(edit.actor.clone().expect("actor"))],
        timestamp,
    });
    assert!(matches!(validate_persisted_conflicts(store.envelope()).await, Err(VcsError::ValidationFailed(message)) if message.contains("operation index")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn conflict_generation_and_validation_canonicalize_repeated_actors() {
    let document_id = "repeated-conflict-actor";
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let clean = severity_mutation_envelope_at(document_id, "same-peer", "same-clean", SeverityMutation::SetN(SeveritySetN { n: 1 }), HybridLogicalTimestamp::new(1, 100));
    let mut fatal = severity_mutation_envelope_at(document_id, "same-peer", "same-fatal", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    fatal.dependencies = vec![clean.mutation_id.clone()];

    store.ingest_remote(fatal).await.expect("dependent fatal is buffered");
    let report = store.ingest_remote(clean).await.expect("ready batch is reported");
    let conflict_id = report.conflict.expect("fatal batch is quarantined");
    assert!(!report.accepted);
    let conflict = store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("durable conflict");
    assert_eq!(conflict.actors, vec![ActorId("same-peer".into())]);
    validate_persisted_conflicts(store.envelope()).await.expect("generated actors use the canonical unique participant set");

    let mut malformed = owned_test_envelope(&store).await;
    malformed.conflicts[0].actors.push(ActorId("same-peer".into()));
    assert!(matches!(validate_persisted_conflicts(&malformed).await, Err(VcsError::ValidationFailed(message)) if message.contains("malformed actor identities")));
    retire_demo_envelope(malformed);
}

#[semio_framework_async_macros::async_test]
async fn quarantined_accept_is_atomic_when_a_later_envelope_remains_fatal() {
    let document_id = "severity-atomic";
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let clean = severity_mutation_envelope_at(document_id, "clean-peer", "clean-op", SeverityMutation::SetN(SeveritySetN { n: 1 }), HybridLogicalTimestamp::new(1, 100));
    let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    let kind = semio_framework_os_spr::ConflictKind::Quarantined { envelopes: vec![clean.clone(), fatal.clone()] };
    let timestamp = HybridLogicalTimestamp::new(9, 300);
    let mutation_ids = vec![clean.mutation_id.clone(), fatal.mutation_id.clone()];
    let conflict_id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(document_id.to_string()), &mutation_ids, &timestamp).await;
    store.0.envelope.conflicts.push(semio_framework_os_spr::Conflict {
        id: conflict_id.clone(),
        kind,
        status: semio_framework_os_spr::ConflictStatus::Open,
        messages: vec![semio_framework_os_spr::MutationMessage {
            level: semio_framework_diagnostic::Severity::Fatal,
            code: semio_framework_diagnostic::FaultCode("mutation.invariant".to_string()),
            message: "n invariant violated".to_string(),
            target: vec!["n".to_string()],
            op_index: Some(0),
        }],
        actors: vec![clean.actor.clone(), fatal.actor.clone()],
        timestamp,
    });
    validate_persisted_conflicts(store.envelope()).await.expect("well-formed quarantine fixture");
    let before = owned_test_envelope(&store).await;
    let generation = store.generation();

    let report = store.resolve_conflict(&conflict_id.0, semio_framework_os_spr::ConflictResolution::Accept).await.expect("fatal outcome is reported, not an infrastructure error");

    assert!(!report.accepted);
    assert_eq!(report.replayed.len(), 2, "the aggregate report must retain the clean and fatal replay reports");
    assert_eq!(store.envelope(), &before, "the candidate's earlier clean mutation must never leak through a later fatal rejection");
    assert_eq!(store.snapshot().expect("unchanged snapshot"), DemoSnapshot { n: Some(0) });
    assert_eq!(store.generation(), generation, "a rejected candidate does not invalidate the source store");
    assert_eq!(store.conflicts().iter().find(|conflict| conflict.id == conflict_id).expect("original conflict retained").status, semio_framework_os_spr::ConflictStatus::Open);
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn a_remote_event_rejected_by_policy_stays_quarantined_out_of_history() {
    let document_id = "severity-events";
    let fatal = severity_mutation_envelope_at(document_id, "fatal-peer", "fatal-op", SeverityMutation::SetFatalN(SetFatalN { n: 2 }), HybridLogicalTimestamp::new(2, 200));
    let mut local = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", document_id, DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let report = local.ingest_remote(fatal).await.expect("a policy rejection is a report");
    assert!(!report.accepted);
    assert_eq!(local.snapshot().expect("local content remains unchanged"), DemoSnapshot { n: Some(0) });
    assert!(local.envelope().vcs.edits.is_empty(), "rejected remote edits are never adopted into local history");
    assert_eq!(local.conflicts().len(), 1);
    assert_eq!(local.conflicts()[0].status, semio_framework_os_spr::ConflictStatus::Open);
}

/// 🧪️ `preview_wire`'s headline law (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-
/// FIRST-CLASS-CONFLICTS` §C6): its output is exactly what independently folding the same ops
/// through the diff engine (`apply_mutation`, the same primitive `replay_mutations` itself calls
/// on the real apply path) computes, stamped with `op_index`, and the live store is byte-
/// identical before and after — a pure dry run all the way through, never applying anything even
/// though it threads state forward internally to preview op `i` against `0..i`'s outcome.
///
/// 🧮️ Independently folds the SAME ops through the exact primitive the real apply path
/// (`replay_mutations`) itself calls (`apply_mutation`) — the ground truth `preview_wire`
/// must match. `replay_mutations` does not yet surface its own messages (lane 1-A's pending
/// C6 work, see `📓️w1-e-report.md`), so this recomputation is the closest available proof.
#[semio_framework_async_macros::async_test]
async fn preview_wire_reports_the_same_messages_the_real_apply_would_produce_and_changes_nothing() {
    let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let before = owned_test_envelope(&store).await;

    let ops: Vec<Vec<u8>> = vec![
        SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode clean"),
        SeverityMutation::SetWarningN(SetWarningN { n: 2 }).encode_op().expect("encode warn"),
        SeverityMutation::SetErrorN(SetErrorN { n: 99 }).encode_op().expect("encode error"),
        SeverityMutation::SetFatalN(SetFatalN { n: 99 }).encode_op().expect("encode fatal"),
    ];

    let messages = store.preview_wire(&ops).await;

    let mut expected = Vec::new();
    let mut running = DemoSnapshot { n: Some(0) };
    for (index, op) in ops.iter().enumerate() {
        let mutation = SeverityMutation::decode_op(op).expect("decode");
        let (next, op_messages) = apply_mutation(&running, &mutation).expect("preview fixture diff applies");
        for message in op_messages {
            expected.push(message.at_op(index as u32));
        }
        running = next;
    }
    assert_eq!(messages, expected, "preview_wire must report exactly the messages the real diff engine computes");
    assert_eq!(messages.len(), 3, "one message each for warn/error/fatal; the clean op is silent");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning);
    assert_eq!(messages[0].op_index, Some(1));
    assert_eq!(messages[1].level, semio_framework_diagnostic::Severity::Error);
    assert_eq!(messages[1].op_index, Some(2));
    assert_eq!(messages[2].level, semio_framework_diagnostic::Severity::Fatal);
    assert_eq!(messages[2].op_index, Some(3));

    assert_eq!(store.envelope(), &before, "preview_wire is a pure dry run: the live store is byte-identical afterward");
    assert_eq!(store.snapshot().expect("snapshot unaffected"), DemoSnapshot { n: Some(0) }, "preview_wire never advances the live cursor");
    retire_demo_envelope(before);
}

#[semio_framework_async_macros::async_test]
async fn preview_wire_reports_a_fatal_message_for_undecodable_op_bytes_and_stops_there() {
    let envelope: ArtifactEnvelope<DemoSnapshot, SeverityMutation> = create_document_envelope("demo/v1", "preview-malformed", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    let ops: Vec<Vec<u8>> = vec![SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode"), vec![0xff, 0xff, 0xff]];

    let messages = store.preview_wire(&ops).await;

    assert_eq!(messages.len(), 1, "the clean op is silent; the malformed op reports one structural message and stops");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Fatal);
    assert_eq!(messages[0].code.0, "mutation.invariant");
    assert_eq!(messages[0].op_index, Some(1));
}

#[semio_framework_async_macros::async_test]
async fn spr_round_trip_preserves_edit_messages_and_conflicts() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initial = DemoSnapshot { n: Some(0) };
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "durable-outcomes", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let receipt = store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 2 })], transaction: None }, &mut identity).await.expect("warning is accepted");
    let edit_id = receipt.edit_ids.first().expect("one durable edit").clone();
    let messages = store.messages_for_edit(&edit_id).to_vec();
    let edit = store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id).expect("durable edit");
    let mutation_ids = stable_mutation_ids_for_edit(edit).await.expect("durable edit carries operation identity");
    let actors = vec![ActorId(edit.actor.clone().expect("durable edit has an actor"))];
    let timestamp = edit.mutation_meta.first().expect("durable edit carries timestamp").timestamp;
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit_id.clone()] };
    let artifact_id = ArtifactId(store.envelope().id.clone());
    let conflict_id = semio_framework_os_spr::ConflictId::new(&kind, &artifact_id, &mutation_ids, &timestamp).await;
    store.0.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: conflict_id, kind, status: semio_framework_os_spr::ConflictStatus::Open, messages: messages.clone(), actors, timestamp });

    let pack = initial.encode_pack();
    let spr = print_document_spr(store.envelope()).await.expect("outcome history encodes");
    let parsed = parse_document_spr::<DemoSnapshot, SeverityMutation>(&pack, &spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("outcome history decodes");
    assert_eq!(parsed.envelope.edit_messages, store.envelope().edit_messages);
    assert_eq!(parsed.envelope.conflicts, store.envelope().conflicts);

    let restored = ArtifactStore::new(parsed.envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(restored.messages_for_edit(&edit_id), messages);
    assert_eq!(restored.conflicts(), store.conflicts());
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧾️ The two state-dependent `Error` codes of the outcome vocabulary — `mutation.target-referenced` (the target is still
/// referenced) and `mutation.target-mismatch` (the payload contradicts the target's current state) — survive the persisted
/// `.spr` history round trip, and the persisted-message gate refuses either one at any other level.
#[semio_framework_async_macros::async_test]
async fn spr_round_trip_preserves_state_dependent_error_codes() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let initial = DemoSnapshot { n: Some(0) };
    let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, SeverityMutation>("demo/v1", "state-dependent-codes", initial.clone(), None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.set_merge_policy(semio_framework_os_spr::MergePolicy::LaissezFaire);
    let receipt = store.dispatch(ArtifactCommand::Apply { mutations: vec![SeverityMutation::SetWarningN(SetWarningN { n: 2 })], transaction: None }, &mut identity).await.expect("warning is accepted");
    let edit_id = receipt.edit_ids.first().expect("one durable edit").clone();
    let edit = store.envelope().vcs.edits.iter().find(|edit| edit.id == edit_id).expect("durable edit");
    let mutation_ids = stable_mutation_ids_for_edit(edit).await.expect("durable edit carries operation identity");
    let actors = vec![ActorId(edit.actor.clone().expect("durable edit has an actor"))];
    let timestamp = edit.mutation_meta.first().expect("durable edit carries timestamp").timestamp;
    let kind = semio_framework_os_spr::ConflictKind::Degraded { edit_ids: vec![edit_id.clone()] };
    let conflict_id = semio_framework_os_spr::ConflictId::new(&kind, &ArtifactId(store.envelope().id.clone()), &mutation_ids, &timestamp).await;
    let messages: Vec<semio_framework_os_spr::MutationMessage> = ["mutation.target-referenced", "mutation.target-mismatch"].into_iter().map(|code| semio_framework_os_spr::MutationMessage::error(code, format!("{code} persisted")).at(["node-1"]).at_op(0)).collect();
    store.0.envelope.conflicts.push(semio_framework_os_spr::Conflict { id: conflict_id, kind, status: semio_framework_os_spr::ConflictStatus::Open, messages, actors, timestamp });

    let pack = initial.encode_pack();
    let spr = print_document_spr(store.envelope()).await.expect("state-dependent codes encode");
    let parsed = parse_document_spr::<DemoSnapshot, SeverityMutation>(&pack, &spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("state-dependent codes decode");
    assert_eq!(parsed.envelope.conflicts, store.envelope().conflicts);
    let restored = ArtifactStore::new(parsed.envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(restored.conflicts(), store.conflicts());
    let codes: Vec<&str> = restored.conflicts().last().expect("the conflict survives").messages.iter().map(|message| message.code.0.as_str()).collect();
    assert_eq!(codes, ["mutation.target-referenced", "mutation.target-mismatch"]);

    for message in &mut store.0.envelope.conflicts.last_mut().expect("the conflict").messages {
        message.level = semio_framework_diagnostic::Severity::Fatal;
    }
    let error = print_document_spr(store.envelope()).await.expect_err("a state-dependent code is an Error, never Fatal");
    assert!(matches!(&error, VcsError::ValidationFailed(detail) if detail.contains("malformed mutation message")), "{error:?}");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 📖️ The persisted-message gate walks the language-agnostic outcome vocabulary (`📡️replication/🎮️mutation/🧪️tests/🧫️fixtures/
/// 🧫️outcome-code`): every code persists at exactly its level and at no other, the apply family persists `Fatal`, and every
/// code the vocabulary rejects is refused as unknown.
#[semio_framework_async_macros::async_test]
async fn persisted_messages_admit_exactly_the_outcome_vocabulary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧫️fixtures/🧫️outcome-code/🔣️.json")).expect("outcome code fixture parses");
    let levels = [("info", semio_framework_diagnostic::Severity::Info), ("warning", semio_framework_diagnostic::Severity::Warning), ("error", semio_framework_diagnostic::Severity::Error), ("fatal", semio_framework_diagnostic::Severity::Fatal)];
    let level_of = |name: &str| levels.iter().find(|(known, _)| *known == name).map(|(_, level)| *level).expect("fixture level");
    let persisted = |code: &str, level: semio_framework_diagnostic::Severity| semio_framework_os_spr::MutationMessage {
        level,
        code: semio_framework_diagnostic::FaultCode::new(code),
        message: format!("{code} persisted"),
        target: vec!["node-1".to_string()],
        op_index: Some(0),
    };
    let mut admitted: Vec<(String, semio_framework_diagnostic::Severity)> = fixture["codes"].as_array().expect("codes").iter().map(|row| (row["code"].as_str().expect("code").to_string(), level_of(row["level"].as_str().expect("level")))).collect();
    admitted.extend(fixture["apply"]["accepted"].as_array().expect("apply codes").iter().map(|code| (code.as_str().expect("apply code").to_string(), level_of(fixture["apply"]["level"].as_str().expect("apply level")))));
    for (code, level) in &admitted {
        assert_eq!(expected_mutation_message_level(code).await, Some(*level), "{code}");
        validate_persisted_message(&persisted(code, *level), Some(1)).await.unwrap_or_else(|error| panic!("{code} persists at its level: {error:?}"));
        for (_, other) in levels.iter().filter(|(_, other)| other != level) {
            assert!(validate_persisted_message(&persisted(code, *other), Some(1)).await.is_err(), "{code} is refused at {other:?}");
        }
    }
    for code in fixture["rejected"].as_array().expect("rejected codes").iter().map(|code| code.as_str().expect("rejected code")) {
        assert_eq!(expected_mutation_message_level(code).await, None, "{code}");
        assert!(validate_persisted_message(&persisted(code, semio_framework_diagnostic::Severity::Error), Some(1)).await.is_err(), "{code} is refused");
    }
}

#[semio_framework_async_macros::async_test]
async fn spr_parse_rejects_history_without_authoritative_operation_metadata() {
    let initial = DemoSnapshot { n: Some(0) };
    let history = semio_framework_os_spr::HistoryLog {
        doc_id: "metadata-required".to_string(),
        schema: "demo/v1".to_string(),
        edits: [semio_framework_os_spr::HistoryEdit {
            line: None,
            id: "edit-1".to_string(),
            actor: Some("author".into()),
            started_at: String::new(),
            finished_at: None,
            verb: None,
            ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(SeverityMutation::SetN(SeveritySetN { n: 1 }).encode_op().expect("encode")) }],
            inverse: Vec::new().into(),
            meta: None,
            lane: None,
        }].into_iter().collect(),
        ..Default::default()
    };
    let spr = semio_framework_os_spr::encode_history(&history, &semio_framework_os_spr::EncodeOptions::default()).await.expect("encode fixture");
    let error = parse_document_spr::<DemoSnapshot, SeverityMutation>(&initial.encode_pack(), &spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect_err("authoritative history must never synthesize operation identity");
    assert!(error.message.contains("authoritative operation metadata"));
}


//#region 🔖️TextFormatHelpers
#[semio_framework_async_macros::async_test]
async fn ops_author_conversion_keeps_the_avatar_a_commit_transition_covers() {
    let author = Author { id: "a1".into(), name: "Alice".into(), avatar: Some("http://example/a1.png".into()) };
    let round_tripped: Author = OpsAuthor::from(&author).into();
    assert_eq!(round_tripped, author, "the avatar is part of the commit payload the transition id covers, so the .ops text must carry it");
}

#[semio_framework_async_macros::async_test]
async fn ops_header_line_commit_round_trips_including_delimiter_and_quote_characters_in_authors() {
    let header = OpsHeaderLine::Commit {
        id: "transition-1".to_string(),
        actor: "actor-a".into(),
        clock: "18.0.0".to_string(),
        after: vec!["op-1".to_string()],
        checkpoint: "c1".to_string(),
        parent: None,
        change: "ch1".to_string(),
        operations: vec!["op-1".to_string(), "op-2".to_string()],
        description: None,
        saved: "17".to_string(),
        by: vec![OpsAuthor { id: "a:1,x".to_string(), name: "Alice, A. \"the great\"".to_string(), avatar: None }, OpsAuthor { id: "b2".to_string(), name: "Bob".to_string(), avatar: Some("b.png".to_string()) }],
        message: Some("first \"checkpoint\"".to_string()),
        at: "18".to_string(),
        line: Vec::new(),
    };
    let printed = header.print_op();
    assert!(!printed.contains('\n'), "print_op must be one line: {printed:?}");
    assert!(!printed.contains("parent="), "an absent optional field must be omitted, not printed as a '-' placeholder: {printed}");
    let parsed = OpsHeaderLine::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed for {printed:?}: {e}"));
    assert_eq!(parsed, header, "OpsHeaderLine::Commit round trip diverged for {printed:?}");
}
//#endregion 🔖️TextFormatHelpers
