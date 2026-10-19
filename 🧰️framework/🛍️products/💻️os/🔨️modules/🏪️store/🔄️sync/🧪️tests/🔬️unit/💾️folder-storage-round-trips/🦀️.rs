/// 💾️ Folder storage round trips: the folder event log, document archive, text and pack storage round trips, the blob store and the incomplete tail.

/// 🎯️ `FolderEventLogStorage` is a pure `(pack, spr)` event store — schema-agnostic,
/// no JSON/codec involvement at this layer (that lives one level up, in `FolderEndpoint`, tested
/// via `folder_external_edit_delivers_remote_operations`). This test exercises exactly the
/// storage mechanics: per-id folding, append-only replacement, and the folder-wide index.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_event_log_storage_round_trips_by_document_id() {
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
    assert_eq!(storage.read("doc-a").await.expect("read empty"), None, "absent document reads as None");

    storage.write("doc-a", "demo/v1", b"pack-a", b"spr-a").await.expect("write a");
    storage.write("doc-b", "demo/v1", b"pack-b", b"spr-b").await.expect("write b");
    assert_eq!(storage.read("doc-a").await.expect("read a").expect("some a"), (b"pack-a".to_vec(), b"spr-a".to_vec()), "documents are keyed independently");
    assert_eq!(storage.read("doc-b").await.expect("read b").expect("some b"), (b"pack-b".to_vec(), b"spr-b".to_vec()));

    storage.write("doc-a", "demo/v1", b"pack-a2", b"spr-a2").await.expect("upsert a");
    assert_eq!(storage.read("doc-a").await.expect("reread a").expect("some a2"), (b"pack-a2".to_vec(), b"spr-a2".to_vec()), "the latest snapshot event replaces the projection");

    let mut ids = storage.document_ids().await.expect("document ids");
    ids.sort();
    assert_eq!(ids, vec!["doc-a".to_string(), "doc-b".to_string()], "folder indexes every document");
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn document_archive_event_log_storage_round_trips_complete_owned_bytes() {
    let archive = persisted_recursive_document_archive();
    let bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode recursive archive");
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
    assert_eq!(storage.read_archive("root-1").await.expect("read absent archive"), None);
    storage.write_archive("root-1", "s.test.root/1/*", &bytes).await.expect("write recursive archive");
    let persisted = storage.read_archive("root-1").await.expect("read recursive archive").expect("archive exists");
    assert_eq!(persisted, bytes);
    assert_eq!(semio_framework_os_spr::decode_document_archive_bytes(&persisted).await.expect("decode persisted recursive archive"), archive);
    assert!(storage.document_ids().await.expect("archive document index").contains(&"root-1".to_string()));
    let mut trailing = persisted.clone();
    trailing.push(0);
    assert!(storage.write_archive("root-1", "s.test.root/1/*", &trailing).await.is_err());
    assert_eq!(storage.read_archive("root-1").await.expect("read after malformed refusal").expect("prior archive remains"), persisted);
    let oversized = vec![1; semio_framework_os_spr::DOCUMENT_ARCHIVE_MAXIMUM_BYTES + 1];
    assert!(storage.write_archive("root-1", "s.test.root/1/*", &oversized).await.is_err());
    assert_eq!(storage.read_archive("root-1").await.expect("read after over-limit refusal").expect("prior archive remains"), bytes);
}

#[cfg(not(target_arch = "wasm32"))]
fn persisted_recursive_document_archive() -> semio_framework_os_spr::DocumentArchivePack {
    let root = semio_framework_os_spr::DocumentArchiveArtifactRef { artifact_id: "root-1".into(), artifact_kind: "s.test.root".into(), standard: "1".into(), subset: "*".into() };
    let child = semio_framework_os_spr::DocumentArchiveArtifactRef { artifact_id: "child-1".into(), artifact_kind: "s.test.child".into(), standard: "1".into(), subset: "native".into() };
    let leaf = semio_framework_os_spr::DocumentArchiveArtifactRef { artifact_id: "leaf-1".into(), artifact_kind: "s.test.child".into(), standard: "1".into(), subset: "native".into() };
    semio_framework_os_spr::DocumentArchivePack {
        parent_pack: vec![1, 2],
        parent_spr: vec![3],
        members: vec![
            semio_framework_os_spr::OwnedDocumentMemberPackEntry {
                ordinal: 0,
                reference: child.clone(),
                owner: semio_framework_os_spr::DocumentArchiveOwnerRef { parent: root, slot: "children".into(), child_id: child.artifact_id.clone() },
                envelope_pack: vec![4, 5],
            },
            semio_framework_os_spr::OwnedDocumentMemberPackEntry {
                ordinal: 1,
                reference: leaf.clone(),
                owner: semio_framework_os_spr::DocumentArchiveOwnerRef { parent: child, slot: "nested".into(), child_id: leaf.artifact_id.clone() },
                envelope_pack: vec![6],
            },
        ],
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn document_archive_actor_refuses_malformed_and_over_limit_candidates_without_displacing_live_bytes() {
    let live = semio_framework_os_spr::encode_document_archive_bytes(&persisted_recursive_document_archive()).expect("encode live recursive archive");
    let (_, remote) = ChannelBackbone::pair("document-archive-refusal").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(8);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "root-1".into(), schema: "s.test.root/1/*".into(), bindings: Vec::new(), watch_external: false, actor: "local".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    actor.handle_test_document_archive(live.clone()).await;
    assert_eq!(actor.current_archive_test(), Some(live.clone()));

    let mut trailing = live.clone();
    trailing.push(0);
    assert!(artifact_actor_message_bytes(&ArtifactActorMsg::LocalDocumentArchive { archive: trailing.clone() }).is_some());
    actor.handle_test_document_archive(trailing).await;
    assert_eq!(actor.current_archive_test(), Some(live.clone()));

    let unsupported = vec![2];
    assert!(artifact_actor_message_bytes(&ArtifactActorMsg::LocalDocumentArchive { archive: unsupported.clone() }).is_none());
    actor.handle_test_document_archive(unsupported).await;
    assert_eq!(actor.current_archive_test(), Some(live.clone()));

    let mut oversized = vec![0; semio_framework_os_spr::DOCUMENT_ARCHIVE_MAXIMUM_BYTES + 1];
    oversized[0] = 1;
    assert!(artifact_actor_message_bytes(&ArtifactActorMsg::LocalDocumentArchive { archive: oversized.clone() }).is_none());
    actor.handle_test_document_archive(oversized).await;
    assert_eq!(actor.current_archive_test(), Some(live));
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn document_archive_text_storage_round_trips_complete_owned_bytes() {
    let archive = persisted_recursive_document_archive();
    let bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode recursive archive");
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let storage = FolderTextStorage::new(dir.path().to_path_buf()).await;
    let files = ArtifactPackFiles { pack: archive.parent_pack.clone(), spr: archive.parent_spr.clone(), ops: String::new() };
    assert_eq!(storage.read_archive("root-1", "test").await.expect("read absent archive"), None);
    storage.write_archive("root-1", "test", &bytes, &files, "root = 1\n").await.expect("write recursive archive");
    let persisted = storage.read_archive("root-1", "test").await.expect("read recursive archive").expect("archive exists");
    assert_eq!(persisted, bytes);
    assert_eq!(semio_framework_os_spr::decode_document_archive_bytes(&persisted).await.expect("decode persisted recursive archive"), archive);
    let mut malformed = persisted.clone();
    malformed.push(0);
    assert!(storage.write_archive("root-1", "test", &malformed, &files, "root = 2\n").await.is_err());
    assert_eq!(storage.read_archive("root-1", "test").await.expect("read after malformed refusal").expect("prior archive remains"), persisted);
}

/// 🔐️ The endpoint-level save→load→undo proof: a store's undo/redo position survives a
/// full write/read cycle through the ACTUAL `FolderEventLogStorage` byte storage (`store`'s own
/// `save_load_undo_proof_pack_spr_round_trip_preserves_undo_redo_position` proves the pure
/// in-memory pack/spr encoding; this proves the folder persistence layer built on top of it).
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_event_log_storage_round_trips_undo_position_through_pack_spr() {
    const IDENTITY_CEILING:usize=201*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());

    let mut store = semio_framework_os_store_test_support::plain_test_store(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "doc-a", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], transaction: None }, &mut identity).await.expect("apply e1");
    let post_e1 = store.snapshot().expect("post-e1");
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], transaction: None }, &mut identity).await.expect("apply e2");
    store.dispatch(ArtifactCommand::Undo, &mut identity).await.expect("undo e2");
    assert_eq!(store.snapshot().expect("live"), post_e1, "precondition: live store is back at post-e1");

    let files = print_document_pack(store.envelope()).await.expect("print document pack");
    storage.write("doc-a", "demo/v1", &files.pack, &files.spr).await.expect("write");

    let (pack, spr) = storage.read("doc-a").await.expect("read").expect("some");
    let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&pack, &spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse: {error}"));
    assert_eq!(parsed.snapshot, post_e1, "loaded snapshot must equal post-e1 through the folder storage layer");
    let mut reloaded = semio_framework_os_store_test_support::plain_test_store(parsed.envelope).await;
    assert_eq!(reloaded.snapshot().expect("reloaded"), post_e1);

    reloaded.dispatch(ArtifactCommand::Redo, &mut identity).await.expect("redo e2 after folder reload");
    assert_eq!(reloaded.snapshot().expect("post-redo"), DemoSnapshot { n: 2 });
    semio_framework_os_store_test_support::close_plain_test_store(&mut store);
    semio_framework_os_store_test_support::close_plain_test_store(&mut reloaded);
}

/// 🎯️ Seeds the write from a ZERO-edit envelope (no cursor line — a cursor is only
/// synced once an edit is dispatched, see `ArtifactStore::sync_cursor`) so both edits are then
/// added purely via the raw `append_ops` hot path with no cursor line ever written; a cursor
/// pinned to an earlier edit count would otherwise cap the reconstructed snapshot at that
/// edit (see `document_text_round_trips_a_cursor_after_undo_then_apply_interleaving` in
/// `store`'s own test suite for that law, exercised correctly there).
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_text_storage_round_trips_dsl_and_appends_ops() {
    const IDENTITY_CEILING:usize=201*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let storage = FolderTextStorage::new(dir.path().to_path_buf()).await;
    assert_eq!(storage.read("demo", "demo").await.expect("read empty"), None, "absent document reads as None");

    let mut seed = semio_framework_os_store_test_support::plain_test_store::<DemoSnapshot, DemoMutation>(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    let files = print_document_text(seed.envelope()).await.expect("print document text");
    storage.write("demo", "demo", &files).await.expect("write");

    let mut store = semio_framework_os_store_test_support::plain_test_store(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], transaction: None }, &mut identity).await.expect("apply 1");
    let first_edit = store.envelope().vcs.edits.last().expect("first edit");
    storage.append_ops("demo", "demo", &print_edit_lines(first_edit).await.expect("print edit lines")).await.expect("append ops 1");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], transaction: None }, &mut identity).await.expect("apply 2");
    let second_edit = store.envelope().vcs.edits.last().expect("second edit");
    storage.append_ops("demo", "demo", &print_edit_lines(second_edit).await.expect("print edit lines")).await.expect("append ops 2");

    let reloaded = storage.read("demo", "demo").await.expect("read").expect("some");
    let parsed: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded.dsl, &reloaded.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse: {error}"));
    assert_eq!(parsed.snapshot.n, 2, "write + append reconstructs every edit in order");
    let _ = parsed.into_snapshot();

    assert_eq!(storage.document_ids("demo").await.expect("document ids"), vec!["demo".to_string()]);
    semio_framework_os_store_test_support::close_plain_test_store(&mut seed);
    semio_framework_os_store_test_support::close_plain_test_store(&mut store);
}

/// 🎯️ Unlike the `.ops`-text hot path (`append_ops`, tested above), `.pack`+`.spr` have
/// no incremental-append primitive wired up yet (that's `crate::os_spr::HistoryAppender`, a future
/// wave's job to thread through this storage layer) — `write_pack` is the whole-file cold path
/// for the AUTHORITATIVE pair, called again after every edit. `append_ops` still keeps the
/// `.ops` TEXT MIRROR current independently (it is never read by the pack+spr-first
/// `parse_document_pack`/`read_pack` path — see `ArtifactPackFiles`'s doc — only by
/// `parse_document_text`), which this test verifies explicitly: appending ops text alone,
/// without an accompanying `write_pack`, does NOT change what `read_pack`/`parse_document_pack`
/// reconstructs, because pack+spr (not ops text) are authoritative for that path.
///
/// Text mirror is current (both edits landed via append_ops).
///
/// pack+spr are unaffected by ops-text-only appends — still the zero-edit snapshot from
/// the initial write_pack, proving read_pack/parse_document_pack never reads .ops.
///
/// A fresh whole-file write_pack (the actual cold-path persistence flow) brings pack+spr
/// current with the live store.
///
/// The always-written DSL mirror must also be on disk and agree with the initial-snapshot.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_text_storage_round_trips_pack() {
    const IDENTITY_CEILING:usize=201*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let storage = FolderTextStorage::new(dir.path().to_path_buf()).await;
    assert_eq!(storage.read_pack("demo", "demo").await.expect("read empty"), None, "absent pack reads as None");

    let mut seed = semio_framework_os_store_test_support::plain_test_store::<DemoSnapshot, DemoMutation>(create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    let files = print_document_pack(seed.envelope()).await.expect("print document pack");
    let dsl_mirror = seed.envelope().vcs.genesis.facts().snapshot().print_dsl();
    storage.write_pack("demo", "demo", &files, &dsl_mirror).await.expect("write pack");

    let mut store = semio_framework_os_store_test_support::plain_test_store(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], transaction: None }, &mut identity).await.expect("apply 1");
    let first_edit = store.envelope().vcs.edits.last().expect("first edit");
    storage.append_ops("demo", "demo", &print_edit_lines(first_edit).await.expect("print edit lines")).await.expect("append ops 1");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 2 }], transaction: None }, &mut identity).await.expect("apply 2");
    let second_edit = store.envelope().vcs.edits.last().expect("second edit");
    storage.append_ops("demo", "demo", &print_edit_lines(second_edit).await.expect("print edit lines")).await.expect("append ops 2");

    let reloaded_text = storage.read("demo", "demo").await.expect("read text").expect("some text");
    let parsed_text: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_text(&reloaded_text.dsl, &reloaded_text.ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse text: {error}"));
    assert_eq!(parsed_text.snapshot.n, 2, "the .ops text mirror reflects every appended edit");
    let _ = parsed_text.into_snapshot();

    let reloaded_pack = storage.read_pack("demo", "demo").await.expect("read pack").expect("some pack");
    let parsed_pack: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack.pack, &reloaded_pack.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse pack: {error}"));
    assert_eq!(parsed_pack.snapshot.n, 0, "pack+spr are authoritative and independent of ops-text-only appends");
    let _ = parsed_pack.into_snapshot();

    let files2 = print_document_pack(store.envelope()).await.expect("print document pack 2");
    let dsl_mirror2 = store.envelope().vcs.genesis.facts().snapshot().print_dsl();
    storage.write_pack("demo", "demo", &files2, &dsl_mirror2).await.expect("write pack 2");
    let reloaded_pack2 = storage.read_pack("demo", "demo").await.expect("read pack 2").expect("some pack 2");
    let parsed_pack2: ParsedDocumentText<DemoSnapshot, DemoMutation> = parse_document_pack(&reloaded_pack2.pack, &reloaded_pack2.spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.unwrap_or_else(|error| panic!("parse pack 2: {error}"));
    assert_eq!(parsed_pack2.snapshot.n, 2, "a fresh write_pack brings pack+spr current with the live store");
    let _ = parsed_pack2.into_snapshot();

    let mirror_path = dir.path().join(semio_framework_os_semio::semio_filename("demo", "demo", semio_framework_os_semio::Component::Dsl));
    let mirror = std::fs::read_to_string(&mirror_path).expect("dsl mirror on disk");
    assert_eq!(DemoSnapshot::parse_dsl(&mirror).expect("parse mirror").n, 0, "mirror captures the initial snapshot, not later edits");
    semio_framework_os_store_test_support::close_plain_test_store(&mut seed);
    semio_framework_os_store_test_support::close_plain_test_store(&mut store);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn blob_store_put_get_dedupes_idempotently() {
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
    let bytes = b"hello content-addressed world";
    assert!(!storage.has("not-a-real-hash").await.expect("has on empty store"));

    let first = storage.put(bytes, "text/plain").await.expect("first put");
    let second = storage.put(bytes, "text/plain").await.expect("second put");
    assert_eq!(first, second, "putting identical bytes twice is idempotent and dedupes by hash");
    assert_eq!(first.size, bytes.len() as u64);
    assert_eq!(first.media_type, "text/plain");

    assert!(storage.has(&first.hash).await.expect("has after put"));
    let fetched = storage.get(&first.hash).await.expect("get").expect("blob present");
    assert_eq!(fetched, bytes);

    let other = storage.put(b"different content", "text/plain").await.expect("put other");
    assert_ne!(other.hash, first.hash, "different bytes hash differently");

    storage.delete(&first.hash).await.expect("delete");
    assert!(!storage.has(&first.hash).await.expect("has after delete"));
    assert_eq!(storage.get(&first.hash).await.expect("get after delete"), None);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn folder_event_log_ignores_an_incomplete_tail_and_coordinates_handles() {
    use std::io::Write;

    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    let first = FolderEventLogStorage::new(dir.path().to_path_buf());
    let second = FolderEventLogStorage::new(dir.path().to_path_buf());
    first.write("doc-a", "demo/v1", b"pack-a", b"spr-a").await.expect("first handle write");
    second.write("doc-b", "demo/v1", b"pack-b", b"spr-b").await.expect("second handle write");

    let mut file = std::fs::OpenOptions::new().append(true).open(first.event_path()).expect("open event log");
    file.write_all(&FOLDER_EVENT_MAGIC[..3]).expect("partial crash tail");
    file.sync_data().expect("sync crash tail");

    assert_eq!(first.read("doc-a").await.expect("read a").expect("doc a"), (b"pack-a".to_vec(), b"spr-a".to_vec()));
    assert_eq!(second.read("doc-b").await.expect("read b").expect("doc b"), (b"pack-b".to_vec(), b"spr-b".to_vec()));
}
