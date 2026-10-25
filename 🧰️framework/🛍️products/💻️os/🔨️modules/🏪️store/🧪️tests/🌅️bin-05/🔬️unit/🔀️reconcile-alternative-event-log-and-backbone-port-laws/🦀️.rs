// 🔀️ Reconcile alternative, event log and backbone port laws: the reconcile alternative pins, the checkpoint minting, the merge base, the event log catch up, the structural history replication, the checkout and edit convergence, the space member checkout routing and the backbone ports.

//#region 🔖️ReconcileAlternative
#[semio_framework_async_macros::async_test]
async fn reconcile_alternative_requires_an_existing_checkpoint() {
    const CEILING:usize=201*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING,&mut observer).unwrap();
    let mut envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let error = reconcile_alternative(&mut envelope, "reconciled", None, Vec::new(), &mut identity).await.unwrap_err();
    assert_eq!(error, VcsError::NoCheckpoint);
    retire_demo_envelope(envelope);
}

#[semio_framework_async_macros::async_test]
async fn reconcile_alternative_pins_the_latest_checkpoint_and_optionally_records_a_reconciliation_checkpoint() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: Vec::new() }, &mut identity).await.expect("commit");
    let base_checkpoint_id = store.envelope().vcs.checkpoints[0].id.clone();

    let mut without_message = owned_test_envelope(&store).await;
    let alt_id = reconcile_alternative(&mut without_message, "no-record", None, Vec::new(), &mut identity).await.expect("reconcile without message");
    assert_eq!(without_message.vcs.alternatives.last().unwrap().checkpoint_ids, vec![base_checkpoint_id.clone()]);
    assert_eq!(without_message.vcs.checkpoints.len(), 1, "no checkpoint_message means no new checkpoint is recorded");
    assert!(!alt_id.is_empty());

    let mut with_message = owned_test_envelope(&store).await;
    let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
    reconcile_alternative(&mut with_message, "recorded", Some("merged concurrent work".into()), authors.clone(), &mut identity).await.expect("reconcile with message");
    assert_eq!(with_message.vcs.checkpoints.len(), 2, "a checkpoint_message appends one reconciliation checkpoint");
    let recorded_checkpoint = with_message.vcs.checkpoints.last().unwrap();
    assert_eq!(recorded_checkpoint.parent_id, Some(base_checkpoint_id));
    assert_eq!(recorded_checkpoint.authors, authors);
    assert_eq!(recorded_checkpoint.message, Some("reconciled".into()), "the reconciliation checkpoint's own message is fixed, distinct from the change description");
    assert_eq!(with_message.vcs.changes.last().unwrap().description, Some("merged concurrent work".into()), "the passed checkpoint_message becomes the change's description");
    retire_demo_envelope(without_message);
    retire_demo_envelope(with_message);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn commit_checkpoint_mints_distinct_content_addressed_ids_for_distinct_commits() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply 1");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("first".into()), authors: Vec::new() }, &mut identity).await.expect("commit 1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply 2");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("second".into()), authors: Vec::new() }, &mut identity).await.expect("commit 2");

    let ids: Vec<&str> = store.envelope().vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()).collect();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1], "two distinct commits must mint two distinct checkpoint ids");
    assert!(ids.iter().all(|id| id.starts_with("ck-")));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn merge_base_finds_the_nearest_common_ancestor_across_a_fork() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply root");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }, &mut identity).await.expect("commit root");
    let root_id = store.envelope().vcs.checkpoints[0].id.clone();

    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-a".into() }, &mut identity).await.expect("create feature-a");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply a");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("a1".into()), authors: Vec::new() }, &mut identity).await.expect("commit a1");
    let a1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

    store.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: root_id.clone() }, &mut identity).await.expect("checkout root");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature-b".into() }, &mut identity).await.expect("create feature-b");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None }, &mut identity).await.expect("apply b");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("b1".into()), authors: Vec::new() }, &mut identity).await.expect("commit b1");
    let b1_id = store.envelope().vcs.checkpoints.last().unwrap().id.clone();

    assert_eq!(merge_base(store.envelope(), &a1_id, &b1_id).await, Some(root_id.clone()), "a1 and b1 forked at root");
    assert_eq!(merge_base(store.envelope(), &a1_id, &root_id).await, Some(root_id.clone()), "root is its own descendant's merge-base");
    assert_eq!(merge_base(store.envelope(), &root_id, &root_id).await, Some(root_id), "a checkpoint is its own merge-base");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

#[semio_framework_async_macros::async_test]
async fn merge_base_is_none_for_a_dangling_unknown_checkpoint_id() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("root".into()), authors: Vec::new() }, &mut identity).await.expect("commit");
    let root_id = store.envelope().vcs.checkpoints[0].id.clone();

    assert_eq!(merge_base(store.envelope(), &root_id, "unknown-checkpoint").await, None, "an id absent from the checkpoint list shares no ancestry with anything");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}


//#region 🔖️EventLogCatchUp
#[semio_framework_async_macros::async_test]
async fn event_log_catch_up_adds_only_unknown_events_and_derives_their_records() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = fresh_demo_store().await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("local apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("local".into()), authors: Vec::new() }, &mut identity).await.expect("local commit");

    let mut remote_store = fresh_demo_store().await;
    for event in store.event_log().expect("local log") {
        remote_store.ingest_remote(event).await.expect("remote replica ingests the local log");
    }
    remote_store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("remote apply");
    remote_store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("remote".into()), authors: Vec::new() }, &mut identity).await.expect("remote commit");

    let (channel, remote_end) = ChannelBackbone::pair("chan").await;
    store.attach_backbone(Backbones::Channel(channel)).await.expect("attach");
    let _ = drain_channel_for_test(&remote_end).expect("drain attach announcement");
    remote_end.push(BackboneMessage::Mutations { envelopes: semio_framework_os_spr::encode_envelopes(&remote_store.event_log().expect("remote log")) }).await.expect("push the whole remote log");
    store.tick().await.expect("tick folds the pushed events");

    assert_eq!(store.envelope().vcs.edits.len(), 2, "the shared original edit is deduplicated, only the new remote edit is added");
    assert_eq!(store.envelope().vcs.checkpoints.len(), 2, "the remote's checkpoint is derived from its commit transition");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(2), "current folds in the new remote edit");
    assert_eq!(store.current_checkpoint_id().await, remote_store.current_checkpoint_id().await);
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🧾️ Every observable position of a store: payload, applied order, redo stack, checkpoint and alternative.
async fn history_position(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> (DemoSnapshot, usize, usize, Option<String>, Option<String>) {
    (store.snapshot().expect("snapshot"), store.applied_edit_ids().len(), store.redo_edit_ids().len(), store.current_checkpoint_id().await.map(str::to_string), store.envelope().active_alternative_id.clone())
}

/// 🔀️ Undo, redo, checkpoint, alternative and checkout on one replica reach the other as events over a
/// live backbone — the receiver folds them into exactly the author's positions, step by step.
#[semio_framework_async_macros::async_test]
async fn structural_history_replicates_as_events_over_a_backbone() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut a = fresh_demo_store().await;
    let mut b = fresh_demo_store().await;
    let (near, far) = MemoryBackbone::pair("structural-a", "structural-b").await;
    a.attach_backbone(Backbones::Memory(near)).await.expect("attach a");
    b.attach_backbone(Backbones::Memory(far)).await.expect("attach b");
    let steps: Vec<ArtifactCommand<DemoMutation>> = vec![
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 5 })], transaction: None },
        ArtifactCommand::Undo,
        ArtifactCommand::Redo,
        ArtifactCommand::Undo,
        ArtifactCommand::CommitCheckpoint { message: Some("one".into()), authors: vec![Author { id: "u1".into(), name: "Ada".into(), avatar: Some("a.png".into()) }] },
        ArtifactCommand::CreateAlternative { name: "variant".into() },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 9 })], transaction: None },
        ArtifactCommand::CommitCheckpoint { message: Some("two".into()), authors: Vec::new() },
    ];
    let (shared, local) = steps.split_at(6);
    for command in shared {
        a.dispatch(command.clone(), &mut identity).await.expect("a authors the step");
        b.tick().await.expect("b folds the step's events");
        assert_eq!(history_position(&a).await, history_position(&b).await, "shared trunk steps land on both replicas");
    }
    for command in local {
        a.dispatch(command.clone(), &mut identity).await.expect("a authors on the new alternative");
        b.tick().await.expect("b folds the shared events and stays on the trunk");
    }
    assert_eq!(a.snapshot().expect("a"), DemoSnapshot { n: Some(9) });
    assert_eq!(b.snapshot().expect("b"), DemoSnapshot { n: Some(1) }, "b never left the trunk");
    assert!(a.envelope().active_alternative_id.is_some());
    assert!(b.envelope().active_alternative_id.is_none());
    let first = a.envelope().vcs.checkpoints[0].id.clone();
    let transitions = a.envelope().transitions.len();
    a.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: first }, &mut identity).await.expect("a checks out the first checkpoint locally");
    b.tick().await.expect("b has no checkout event to fold");
    assert_eq!(a.envelope().transitions.len(), transitions);
    assert_eq!(a.snapshot().expect("a"), DemoSnapshot { n: Some(1) });
    assert_eq!(b.snapshot().expect("b"), DemoSnapshot { n: Some(1) });
    assert_ne!(a.envelope().active_alternative_id, b.envelope().active_alternative_id);
    assert_eq!(a.envelope().vcs.alternatives, b.envelope().vcs.alternatives, "alternative chains are folded facts on both replicas");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🔀️ A checkout and a concurrent edit converge whichever replica receives the other's events first.
#[semio_framework_async_macros::async_test]
async fn a_concurrent_checkout_and_edit_converge_in_either_arrival_order() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut a = fresh_demo_store().await;
    a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("seed edit");
    a.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }, &mut identity).await.expect("seed checkpoint");
    a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("a's later edit");
    let mut b = fresh_demo_store().await;
    for event in a.event_log().expect("a's log") {
        b.ingest_remote(event).await.expect("b catches up");
    }
    let known: HashSet<MutationId> = a.event_log().expect("a's log").into_iter().map(|event| event.mutation_id).collect();
    let checkpoint_id = a.envelope().vcs.checkpoints[0].id.clone();
    let transitions = a.envelope().transitions.len();
    a.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.clone() }, &mut identity).await.expect("a checks out the checkpoint");
    assert_eq!(a.envelope().transitions.len(), transitions, "a's checkout authors nothing a peer could fold");
    b.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 7 })], transaction: None }, &mut identity).await.expect("b's concurrent edit");
    let fresh_from = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| store.event_log().expect("log").into_iter().filter(|event| !known.contains(&event.mutation_id)).collect::<Vec<_>>();
    let from_b = fresh_from(&b);
    assert!(fresh_from(&a).is_empty());
    for event in from_b {
        a.ingest_remote(event).await.expect("a folds b's edit");
    }
    assert_eq!(a.snapshot().expect("a"), DemoSnapshot { n: Some(1) }, "a's explicit checkout hides uncommitted edits");
    assert_eq!(b.snapshot().expect("b"), DemoSnapshot { n: Some(7) });
    b.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id }, &mut identity).await.expect("b looks at the same checkpoint");
    assert_eq!(history_position(&a).await, history_position(&b).await, "equal heads over the same events agree");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 💾️ A reload replays the persisted events into the exact positions — through `.pack`+`.spr` and
/// through the `.ops` text mirror alike; no cursor is persisted.
#[semio_framework_async_macros::async_test]
async fn reload_replays_the_event_log_into_the_same_positions() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = fresh_demo_store().await;
    for command in [
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None },
        ArtifactCommand::CommitCheckpoint { message: Some("base".into()), authors: Vec::new() },
        ArtifactCommand::CreateAlternative { name: "variant".into() },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 8 })], transaction: None },
        ArtifactCommand::Undo,
    ] {
        store.dispatch(command, &mut identity).await.expect("history step");
    }
    let expected = history_position(&store).await;
    let files = print_document_pack(store.envelope()).await.expect("print pack");
    let reloaded = ArtifactStore::new(parse_document_pack::<DemoSnapshot, DemoMutation>(&files.pack, &files.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse pack").envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(history_position(&reloaded).await, expected, ".pack+.spr reload restores every position by folding events");
    let text = print_document_text(store.envelope()).await.expect("print text");
    assert!(text.ops.contains("\ncommit ") && text.ops.contains("\nbranch ") && text.ops.contains("\nrevert "), "the text mirror states the semantic transitions: {}", text.ops);
    let from_text = ArtifactStore::new(parse_document_text::<DemoSnapshot, DemoMutation>(&text.dsl, &text.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.expect("parse text").envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    assert_eq!(from_text.snapshot().expect("text snapshot"), expected.0, "the ops text folds the shared events");
    assert!(from_text.envelope().active_alternative_id.is_none(), "the ops text has no viewer head and hydrates to the trunk");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

/// 🎯️ A reload adopts the central fold: after `reset` the store's current projection is exactly the genesis snapshot taken through
/// [`protocol::apply_diff_cold`] once per applied operation, in applied order — the oracle below calls the central applier directly and
/// the store holds no other route from an event log to a projection.
#[semio_framework_async_macros::async_test]
async fn a_reload_adopts_the_central_apply_fold_of_the_applied_event_log() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let mut store = fresh_demo_store().await;
    for command in [
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 3 })], transaction: None },
        ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 8 })], transaction: None },
        ArtifactCommand::Undo,
    ] {
        store.dispatch(command, &mut identity).await.expect("history step");
    }
    let candidate = owned_test_envelope(&store).await;
    let mut reloaded = fresh_demo_store().await;
    reloaded.reset(candidate).await.expect("reload");

    let envelope = reloaded.envelope();
    let mut oracle = envelope.vcs.genesis.facts().snapshot().clone();
    for edit_id in reloaded.applied_edit_ids().iter() {
        let edit = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id).expect("the applied edit is in the log");
        for operation in edit.forwards.iter() {
            oracle = protocol::apply_diff_cold(operation.diff(&oracle).diff(), &oracle).expect("the central applier folds the applied operation");
        }
    }
    assert_eq!(reloaded.snapshot().expect("snapshot"), oracle, "the reloaded projection is the central apply fold of the applied operations");
    assert_eq!(oracle.n, Some(3), "the undone edit is not part of the fold");
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#endregion 🔖️EventLogCatchUp

//#region 🔖️SpaceMemberCheckoutRouting
#[semio_framework_async_macros::async_test]
async fn space_member_checkout_switches_at_the_alternative_tip_and_falls_back_to_checkout_when_stale() {
    const IDENTITY_CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| { assert!(progress.owned_bytes <= IDENTITY_CEILING); true };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING, &mut identity_observer).expect("declared Unit caller identity authority");
    let envelope: ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut store = ArtifactStore::new(envelope, &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 1 })], transaction: None }, &mut identity).await.expect("apply");
    store.dispatch(ArtifactCommand::CreateAlternative { name: "feature".into() }, &mut identity).await.expect("create alternative (auto-commits since no checkpoint existed yet)");
    let alt_id = store.envelope().vcs.alternatives.last().expect("the branched alternative").id.clone();
    let tip = store.envelope().vcs.alternatives.last().expect("the branched alternative").checkpoint_ids.last().expect("alt has a tip").clone();

    ReferenceSpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout at the tip routes through SwitchAlternative");
    assert_eq!(store.envelope().active_alternative_id, Some(alt_id.clone()), "switching to the tip keeps it active");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN(SetN { n: 2 })], transaction: None }, &mut identity).await.expect("apply on branch");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c2".into()), authors: Vec::new() }, &mut identity).await.expect("commit c2, advancing the alt's tip past `tip`");

    ReferenceSpaceMember::checkout(&mut store, &tip, &alt_id, &mut identity).await.expect("checkout of the now-stale tip falls back to CheckoutCheckpoint");
    assert_eq!(store.snapshot().expect("snapshot").n, Some(1), "restored the old checkpoint's state");
    assert_eq!(store.envelope().active_alternative_id.as_deref(), Some(alt_id.as_str()), "the checkpoint is still on this alternative, so the line stays");
    assert_eq!(store.envelope().viewer_checkpoint_id.as_deref(), Some(tip.as_str()));
    identity.encode(|control| { assert!(control.owned_bytes() <= IDENTITY_CEILING); Ok::<(), semio_framework_value::ValueError>(()) }).expect("same original cumulative identity observation");
    drop(identity.pause().expect("original Unit cumulative identity receipt"));
}

//#endregion 🔖️SpaceMemberCheckoutRouting

//#region 🔖️BackbonePorts
#[semio_framework_async_macros::async_test]
async fn memory_backbone_port_round_trips_and_reports_a_missing_file() {
    let port = MemoryBackbonePort::new().await;
    let error = port.read("file://nowhere").await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    port.write("file://a", "payload-1").await.expect("write");
    assert_eq!(port.read("file://a").await.expect("read"), "payload-1");
    port.write("file://a", "payload-2").await.expect("overwrite");
    assert_eq!(port.read("file://a").await.expect("read after overwrite"), "payload-2", "write is an upsert");
}

#[semio_framework_async_macros::async_test]
async fn local_storage_backbone_port_falls_back_to_its_in_memory_store() {
    let port = LocalStorageBackbonePort::new().await;
    let error = port.read("local://missing").await.unwrap_err();
    assert!(matches!(error, VcsError::Backbone(_)), "got {error:?}");
    port.write("local://a", "value").await.expect("write falls back to the in-memory store");
    assert_eq!(port.read("local://a").await.expect("read falls back too"), "value");

    let defaulted = LocalStorageBackbonePort::default();
    assert!(defaulted.read("local://a").await.is_err(), "Default constructs its own independent fallback store");
}

//#endregion 🔖️BackbonePorts
//#endregion 🔖️ReconcileAlternative
