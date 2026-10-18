/// 🌌️ Stateful current and space history laws: the stateful current against a full replay, the space document registration, the space checkpoint, checkout, switch, undo and redo and the space history op round trips and inverse diffs.

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
