/// 🏁️ Checkpoint, alternative, history column and backbone laws: the backward apply, the checkpoint commit and checkout, the alternatives, the history columns, the backbone message round trips, the memory backbone pair and the echo component codec resolution.

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
    let operations = BackboneMessage::from_envelopes(&[envelope.clone(), envelope]);
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
