/// 📮️ Document codec, replay and projection gate laws: the document codec round trips, the fixture identity apply, the replica replay onto a pair, the codec registration, the dialect migration and the projection result gates.

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
    const CEILING: usize = 201 * semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
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
async fn original_envelope_parse_preserves_genesis_without_a_discarded_projection_clone(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🎮️one-item/🛤️lane/🔣️.json")).unwrap();
 for row in law["envelope"].as_array().unwrap(){
  let value=row["initial"].as_i64().unwrap()as i32;let envelope=create_document_envelope::<DemoSnapshot,DemoMutation>("demo/v1",row["name"].as_str().unwrap(),DemoSnapshot{n:Some(value)},None);let files=print_document_pack(&envelope).await.unwrap();envelope.retire_unadopted();
  DEMO_PROJECTION_CLONES.with(|count|count.set(0));
  let parsed=text_format::parse_document_pack_envelope::<DemoSnapshot,DemoMutation>(&files.pack,&files.spr).await.unwrap();
  assert_eq!(parsed.vcs.genesis.facts().snapshot().n,Some(row["expected"].as_i64().unwrap()as i32));assert_eq!(parsed.vcs.genesis.pack(),files.pack.as_slice());assert!(parsed.vcs.edits.is_empty());assert_eq!(DEMO_PROJECTION_CLONES.with(std::cell::Cell::get),row["discardedProjectionClones"].as_u64().unwrap()as usize);parsed.retire_unadopted();
 }
 println!("[DEBUG] original envelope parser: three neutral genesis cases retain pack facts and metadata validation with discardedProjectionClones0; semantic replay stays owned by receiving Store");
}
