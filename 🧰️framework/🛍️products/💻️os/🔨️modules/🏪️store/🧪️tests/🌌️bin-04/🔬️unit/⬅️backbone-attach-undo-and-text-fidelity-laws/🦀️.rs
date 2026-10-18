/// ⬅️ Backbone attach, undo and text fidelity laws: the attached event log, the channel backbone, the pump acks, the exact base undo, the streamed edit revision, the import and export fidelity, the demo dsl and op round trips and the document text parse.

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
