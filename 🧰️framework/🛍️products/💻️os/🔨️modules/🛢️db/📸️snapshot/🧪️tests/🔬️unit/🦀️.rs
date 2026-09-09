use super::*;
use db_storage::MemoryStorage;

fn pages(bytes: &[u8]) -> db_storage::DbIoPages {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).expect("test snapshot writer admitted");
    for fragment in bytes.chunks(db_storage::DB_IO_PAGE_BYTES) {
        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
    }
    writer.finish().unwrap()
}

//#region 🔖️Descriptor
async fn sample_descriptor(generation: u64, parent: Option<u64>) -> SnapshotDescriptor {
    SnapshotDescriptor {
        document: "doc-1".into(),
        generation,
        parent_generation: parent,
        head_seq: generation * 10,
        commit_seq: generation * 5,
        epoch: 1,
        chain_hash: [generation as u8; 32],
        protocol_version: 1,
        vcs_head: Some("ck-abcdef".to_string()),
        base_pack_hash: Some(ContentHash([9u8; 32])),
        roots: vec![ContentHash([1u8; 32]), ContentHash([2u8; 32])],
        new_pages: vec![],
        created_at_ms: 1_700_000_000_000 + generation,
    }
}

#[semio_framework_async_macros::async_test]
async fn descriptor_encode_decode_round_trips_all_fields() {
    let descriptor = sample_descriptor(3, Some(2)).await;
    let bytes = descriptor.encode().await;
    let decoded = SnapshotDescriptor::decode(&bytes).await.unwrap();
    assert_eq!(decoded, descriptor);
}

#[semio_framework_async_macros::async_test]
async fn descriptor_encode_decode_round_trips_none_optionals() {
    let mut descriptor = sample_descriptor(0, None).await;
    descriptor.vcs_head = None;
    descriptor.base_pack_hash = None;
    descriptor.roots.clear();
    let bytes = descriptor.encode().await;
    let decoded = SnapshotDescriptor::decode(&bytes).await.unwrap();
    assert_eq!(decoded, descriptor);
    assert_eq!(decoded.frontier().await.head_seq, 0);
}

#[semio_framework_async_macros::async_test]
async fn descriptor_decode_rejects_bad_version_tag() {
    let mut bytes = sample_descriptor(0, None).await.encode().await;
    bytes[0] = 0xFF;
    assert!(matches!(SnapshotDescriptor::decode(&bytes).await, Err(DbError::Corrupt(_))));
}

#[semio_framework_async_macros::async_test]
async fn descriptor_decode_never_panics_on_truncated_input() {
    let bytes = sample_descriptor(1, Some(0)).await.encode().await;
    for len in 0..bytes.len() {
        assert!(SnapshotDescriptor::decode(&bytes[..len]).await.is_err(), "expected error at truncation length {len}");
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️Generation
async fn page(bytes: &[u8]) -> Page {
    Page::try_from_pages(pages(bytes)).await.unwrap()
}

#[semio_framework_async_macros::async_test]
async fn single_generation_round_trips_through_pack_public_api() {
    let pages = vec![page(b"page-zero").await, page(b"page-one").await];
    let mut descriptor = sample_descriptor(0, None).await;
    descriptor.new_pages = pages.iter().map(|p| p.hash).collect();
    descriptor.roots = vec![pages[0].hash];

    let bytes = build_generation(&descriptor, &pages, None).await.unwrap();

    // 🔬️ A real `pack::PackFile`, unrelated to this crate's own reader, must accept the bytes.
    let file = pack::PackFile::open_manifest(bytes.as_slice(), &pack::PackLimits::default(), pack::os_pack::VerificationLevel::Full).await.unwrap();
    assert_eq!(file.chunk_count(), 2);
    assert_eq!(file.manifest().unwrap().schema_name, "");

    let handle = open_latest(&bytes).await.unwrap();
    assert_eq!(handle.generation().await, 0);
    assert_eq!(handle.descriptor, descriptor);
    assert!(handle.parent_footer_offset().await.is_none());

    let read_back = read_page(&bytes, &handle, pages[1].hash).await.unwrap();
    assert_eq!(read_back, b"page-one");
}

#[semio_framework_async_macros::async_test]
async fn build_generation_rejects_new_pages_mismatched_with_descriptor() {
    let pages = vec![page(b"a").await];
    let descriptor = sample_descriptor(0, None).await; // descriptor.new_pages left empty
    assert!(matches!(build_generation(&descriptor, &pages, None).await, Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn footer_chain_flag_and_prev_offset_are_genuine_pack_wire_data() {
    let parent_pages = vec![page(b"gen0-page").await];
    let mut gen0_descriptor = sample_descriptor(0, None).await;
    gen0_descriptor.new_pages = parent_pages.iter().map(|p| p.hash).collect();
    let gen0_bytes = build_generation(&gen0_descriptor, &parent_pages, None).await.unwrap();

    let parent_footer_position = gen0_bytes.len() as u64 - pack::FOOTER_SIZE as u64;
    let child_pages = vec![page(b"gen1-page").await];
    let mut gen1_descriptor = sample_descriptor(1, Some(0)).await;
    gen1_descriptor.new_pages = child_pages.iter().map(|p| p.hash).collect();
    let gen1_bytes = build_generation(&gen1_descriptor, &child_pages, Some(parent_footer_position)).await.unwrap();

    // 🔬️ Read gen1's footer via `pack`'s own public superblock API, not this crate's helpers.
    let superblock = pack::PackFile::open_superblock(gen1_bytes.as_slice(), &pack::PackLimits::default()).await.unwrap();
    let footer = superblock.superblock().footer;
    assert_eq!(footer.required_flags & pack::REQUIRED_FOOTER_CHAIN, pack::REQUIRED_FOOTER_CHAIN);
    assert_eq!(footer.prev_footer_offset, parent_footer_position);
    assert_eq!(footer.file_len, gen1_bytes.len() as u64);

    let direct_footer = pack::read_footer_only(&gen1_bytes.as_slice()).await.unwrap();
    assert_eq!(direct_footer, footer);
}

#[semio_framework_async_macros::async_test]
async fn two_generation_incremental_chain_resolves_inherited_pages() {
    let gen0_pages = vec![page(b"root-page").await, page(b"stable-page").await];
    let mut gen0_descriptor = sample_descriptor(0, None).await;
    gen0_descriptor.new_pages = gen0_pages.iter().map(|p| p.hash).collect();
    gen0_descriptor.roots = vec![gen0_pages[0].hash, gen0_pages[1].hash];
    let gen0_bytes = build_generation(&gen0_descriptor, &gen0_pages, None).await.unwrap();

    let parent_footer_position = gen0_bytes.len() as u64 - pack::FOOTER_SIZE as u64;
    let gen1_pages = vec![page(b"changed-page").await];
    let mut gen1_descriptor = sample_descriptor(1, Some(0)).await;
    gen1_descriptor.new_pages = gen1_pages.iter().map(|p| p.hash).collect();
    gen1_descriptor.roots = vec![gen1_pages[0].hash, gen0_pages[1].hash];
    let gen1_bytes = build_generation(&gen1_descriptor, &gen1_pages, Some(parent_footer_position)).await.unwrap();

    let mut combined = Vec::new();
    combined.extend_from_slice(&gen0_bytes);
    combined.extend_from_slice(&gen1_bytes);

    let latest = open_latest(&combined).await.unwrap();
    assert_eq!(latest.generation().await, 1);
    let parent_offset = latest.parent_footer_offset().await.unwrap();
    assert_eq!(parent_offset, parent_footer_position);

    let ancestor = open_ancestor(&combined, parent_offset).await.unwrap();
    assert_eq!(ancestor.generation().await, 0);
    assert_eq!(ancestor.descriptor, gen0_descriptor);
    assert!(ancestor.parent_footer_offset().await.is_none());

    // ✅️ New-in-gen1 page resolves directly from gen1's own chunk table.
    let changed = read_page(&combined, &latest, gen1_pages[0].hash).await.unwrap();
    assert_eq!(changed, b"changed-page");

    // ✅️ Unchanged page (not listed in gen1) resolves by walking to gen0.
    let inherited = read_page(&combined, &latest, gen0_pages[1].hash).await.unwrap();
    assert_eq!(inherited, b"stable-page");

    // ❌️ A hash present in neither generation reports NotFound, not a panic.
    assert!(matches!(read_page(&combined, &latest, ContentHash([0xEE; 32])).await, Err(DbError::NotFound(_))));
}

#[semio_framework_async_macros::async_test]
async fn open_latest_rejects_truncated_buffer() {
    let pages = vec![page(b"only-page").await];
    let mut descriptor = sample_descriptor(0, None).await;
    descriptor.new_pages = pages.iter().map(|p| p.hash).collect();
    let bytes = build_generation(&descriptor, &pages, None).await.unwrap();
    for len in 0..pack::FOOTER_SIZE {
        assert!(open_latest(&bytes[..len]).await.is_err(), "expected error at truncation length {len}");
    }
    assert!(open_latest(&bytes).await.is_ok());
}
//#endregion 🔖️Generation

//#region 🔖️Manager
async fn body(head_seq: u64) -> SnapshotBody {
    SnapshotBody { head_seq, commit_seq: head_seq, epoch: 0, chain_hash: [0u8; 32], protocol_version: 1, vcs_head: None, base_pack_hash: None, roots: vec![], created_at_ms: head_seq * 1000 }
}

#[semio_framework_async_macros::async_test]
async fn manager_publishes_full_baseline_then_loads_it_back() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document: ArtifactId = "doc-a".into();
    let pages = vec![page(b"p0").await];

    let generation = db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &pages, body(10).await)).unwrap();
    assert_eq!(generation, 0);

    let (loaded_generation, descriptor) = db_actor::block_on(manager.load_latest(&document)).unwrap().unwrap();
    assert_eq!(loaded_generation, 0);
    assert_eq!(descriptor.parent_generation, None);
    assert_eq!(descriptor.head_seq, 10);
}

#[semio_framework_async_macros::async_test]
async fn manager_incremental_publish_without_prior_generation_errors() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document: ArtifactId = "doc-b".into();
    let result = db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(0).await));
    assert!(matches!(result, Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn manager_incremental_chain_materializes_and_resolves_inherited_pages() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document: ArtifactId = "doc-c".into();

    let gen0_pages = vec![page(b"base-a").await, page(b"base-b").await];
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &gen0_pages, body(0).await)).unwrap();

    let gen1_pages = vec![page(b"delta-a").await];
    let gen1 = db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &gen1_pages, body(5).await)).unwrap();
    assert_eq!(gen1, 1);

    let (latest_generation, descriptor) = db_actor::block_on(manager.load_latest(&document)).unwrap().unwrap();
    assert_eq!(latest_generation, 1);
    assert_eq!(descriptor.parent_generation, Some(0));

    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8_192).unwrap();
    let mut cursor = manager.chain_cursor(&document, 1, control);
    let mut inherited = cursor.read_page(gen0_pages[1].hash).await.unwrap();
    assert_eq!(inherited, b"base-b");
    while inherited.close_step().unwrap().is_some() {}
    let mut local = cursor.read_page(gen1_pages[0].hash).await.unwrap();
    assert_eq!(local, b"delta-a");
    while local.close_step().unwrap().is_some() {}
    while cursor.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn manager_retain_from_requires_full_baseline_floor() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document: ArtifactId = "doc-d".into();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(0).await)).unwrap();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(1).await)).unwrap();

    assert!(matches!(db_actor::block_on(manager.retain_from(&document, 1)), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn manager_retain_from_deletes_generations_below_a_valid_baseline_floor() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document: ArtifactId = "doc-e".into();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(0).await)).unwrap();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(1).await)).unwrap();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(2).await)).unwrap();

    db_actor::block_on(manager.retain_from(&document, 2)).unwrap();
    assert_eq!(db_actor::block_on(storage.list_generations(&document)).unwrap(), vec![2]);
}

#[semio_framework_async_macros::async_test]
async fn manager_select_generation_picks_highest_head_seq_at_most_target() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document: ArtifactId = "doc-f".into();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &[], body(0).await)).unwrap();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(10).await)).unwrap();
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::Incremental, &[], body(20).await)).unwrap();

    assert_eq!(db_actor::block_on(manager.select_generation(&document, 15)).unwrap(), Some(1));
    assert_eq!(db_actor::block_on(manager.select_generation(&document, 25)).unwrap(), Some(2));
    assert_eq!(db_actor::block_on(manager.select_generation(&document, 5)).unwrap(), Some(0));

    let empty_document: ArtifactId = "doc-empty".into();
    assert_eq!(db_actor::block_on(manager.select_generation(&empty_document, 100)).unwrap(), None);
}

#[semio_framework_async_macros::async_test]
async fn manager_verify_accepts_intact_and_rejects_corrupted_generation() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let manager = SnapshotManager::new(&storage).await;
    let document: ArtifactId = "doc-g".into();
    let source_pages = vec![page(b"verify-me").await];
    db_actor::block_on(manager.publish(&document, SnapshotOrigin::FullBaseline, &source_pages, body(0).await)).unwrap();

    db_actor::block_on(manager.verify(&document, 0, pack::os_pack::VerificationLevel::Full)).unwrap();

    let mut retained = db_actor::block_on(storage.read_generation(&document, 0)).unwrap();
    let mut prepared = db_storage::db_io_prepare_platform(&retained).unwrap().await.unwrap();
    let mut corrupted = prepared.as_slice().to_vec();
    while prepared.close_step().unwrap() {}
    while retained.close_step().unwrap().is_some() {}
    let last = corrupted.len() - 1;
    corrupted[last] ^= 0xFF;
    db_actor::block_on(storage.write_generation(&document, 0, pages(&corrupted))).unwrap();
    assert!(db_actor::block_on(manager.verify(&document, 0, pack::os_pack::VerificationLevel::Standard)).is_err());
}
//#endregion 🔖️Manager

//#region 🔖️Policy
#[semio_framework_async_macros::async_test]
async fn snapshot_policy_fires_on_any_threshold_alone() {
    let policy = SnapshotPolicy { max_ops_since_last: 100, max_bytes_since_last: 1_000_000, max_ms_since_last: 60_000 };
    assert!(!policy.should_snapshot(10, 10, 10).await);
    assert!(policy.should_snapshot(100, 0, 0).await);
    assert!(policy.should_snapshot(0, 1_000_000, 0).await);
    assert!(policy.should_snapshot(0, 0, 60_000).await);
}
//#endregion 🔖️Policy

//#region 🔖️Lease
#[semio_framework_async_macros::async_test]
async fn snapshot_lease_round_trips_acquire_renew_release_via_memory_storage() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();

    let fence = db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-a", 1_000, 0)).unwrap();
    assert_eq!(fence, EpochFence::INITIAL);
    assert!(db_actor::block_on(SnapshotLease::current(&storage, &document, 0)).unwrap().is_some());

    db_actor::block_on(SnapshotLease::renew(&storage, &document, "actor-a", fence, 1_000, 500)).unwrap();

    // ❌️ A second holder can't acquire while actor-a's lease is still unexpired.
    assert!(matches!(db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-b", 1_000, 500)), Err(DbError::Conflict(_))));

    db_actor::block_on(SnapshotLease::release(&storage, &document, "actor-a", fence)).unwrap();
    assert!(db_actor::block_on(SnapshotLease::current(&storage, &document, 500)).unwrap().is_none());

    // ✅️ Now free, actor-b can acquire fresh (an explicit release clears the record entirely,
    // per `LeaseStorage::acquire`'s own doc — only a hand-off from an EXPIRED-but-still-present
    // lease bumps the epoch fence, exercised next).
    let after_release = db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-b", 1_000, 500)).unwrap();
    assert_eq!(after_release, EpochFence::INITIAL);

    // ✅️ A hand-off from an unreleased but time-expired lease DOES bump the fence.
    let expired_handoff = db_actor::block_on(SnapshotLease::acquire(&storage, &document, "actor-c", 1_000, 1_600)).unwrap();
    assert_eq!(expired_handoff, after_release.next());
}

#[semio_framework_async_macros::async_test]
async fn snapshot_lease_resource_name_is_scoped_per_document() {
    let a: ArtifactId = "doc-a".into();
    let b: ArtifactId = "doc-b".into();
    assert_ne!(SnapshotLease::resource(&a), SnapshotLease::resource(&b));
}
//#endregion 🔖️Lease
