
use super::*;
use db_storage::MemoryStorage;

fn control() -> IndexCursorControl {
    IndexCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()
}

async fn retained(source: &[u8]) -> IndexBytes {
    let mut control = control();
    IndexBytes::try_admit(source.to_vec(), MAX_VALUE_LEN, &mut control).await.unwrap()
}

async fn retained_vec(source: Vec<u8>) -> IndexBytes {
    let mut control = control();
    IndexBytes::try_admit(source, MAX_VALUE_LEN, &mut control).await.unwrap()
}

async fn read_retained(bytes: &IndexBytes) -> Vec<u8> {
    let mut prepared = bytes.prepare_platform().await.unwrap();
    let output = prepared.as_slice().to_vec();
    while prepared.close_step().unwrap() {}
    output
}

async fn entry(key: &[u8], value: &[u8]) -> RunEntry {
    RunEntry { key: retained(key).await, value: RunValue::Put(retained(value).await) }
}

async fn tombstone(key: &[u8]) -> RunEntry {
    RunEntry { key: retained(key).await, value: RunValue::Tombstone }
}

async fn run(entries: impl IntoIterator<Item = RunEntry>) -> RunEntries {
    let mut run = RunEntries::new();
    for entry in entries {
        assert!(run.push(entry).is_ok());
    }
    run
}

async fn assert_entry(entry: &RunEntry, key: &[u8], value: Option<&[u8]>) {
    assert_eq!(read_retained(&entry.key).await, key);
    match (&entry.value, value) {
        (RunValue::Put(actual), Some(expected)) => assert_eq!(read_retained(actual).await, expected),
        (RunValue::Tombstone, None) => {}
        _ => panic!("retained run value shape mismatch"),
    }
}

async fn put_bytes<S: IndexStorage>(handle: &IndexHandle<'_, S>, key: &[u8], value: &[u8]) {
    let mut control = control();
    handle.put(retained(key).await, retained(value).await, &mut control).await.unwrap();
}

async fn delete_bytes<S: IndexStorage>(handle: &IndexHandle<'_, S>, key: &[u8]) {
    let mut control = control();
    handle.delete(retained(key).await, &mut control).await.unwrap();
}

async fn get_bytes<S: IndexStorage>(handle: &IndexHandle<'_, S>, key: &[u8]) -> Option<Vec<u8>> {
    let mut control = control();
    let mut key = retained(key).await;
    let result = handle.get(&key, &mut control).await.unwrap();
    while key.close_step().unwrap().is_some() {}
    match result {
        Some(mut bytes) => {
            let output = read_retained(&bytes).await;
            while bytes.close_step().unwrap().is_some() {}
            Some(output)
        }
        None => None,
    }
}

async fn stats<S: IndexStorage>(handle: &IndexHandle<'_, S>) -> IndexStats {
    let mut control = control();
    handle.stats(&mut control).await.unwrap()
}

//#region 🔖️SortedRun
#[semio_framework_async_macros::async_test]
async fn run_round_trips_through_encode_and_decode() {
    let mut entries = run([entry(b"a", b"1").await, entry(b"b", b"2").await, tombstone(b"c").await]).await;
    let mut control = control();
    let encoded = encode_run_pages(IndexKind::Command, &entries, &mut control).await.unwrap();
    while entries.close_step().unwrap() {}
    let mut decoded = decode_run_pages(encoded, IndexKind::Command, &mut control).await.unwrap();
    assert_entry(decoded.get(0).unwrap(), b"a", Some(b"1")).await;
    assert_entry(decoded.get(1).unwrap(), b"b", Some(b"2")).await;
    assert_entry(decoded.get(2).unwrap(), b"c", None).await;
    while decoded.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn decode_run_detects_corruption_via_checksum() {
    let mut entries = run([entry(b"a", b"1").await]).await;
    let mut control = control();
    let encoded = encode_run_pages(IndexKind::Command, &entries, &mut control).await.unwrap();
    while entries.close_step().unwrap() {}
    let mut prepared = db_storage::db_io_prepare_platform(&encoded).unwrap().await.unwrap();
    let mut corrupt = prepared.as_slice().to_vec();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 0xFF;
    while prepared.close_step().unwrap() {}
    let mut encoded = encoded;
    while encoded.close_step().unwrap().is_some() {}
    let corrupt = retained_vec(corrupt).await;
    assert!(matches!(decode_run_pages(corrupt.pages, IndexKind::Command, &mut control).await, Err(DbError::Corrupt(_))));
}

#[semio_framework_async_macros::async_test]
async fn decode_run_rejects_kind_mismatch() {
    let mut entries = run([entry(b"a", b"1").await]).await;
    let mut control = control();
    let encoded = encode_run_pages(IndexKind::Command, &entries, &mut control).await.unwrap();
    while entries.close_step().unwrap() {}
    assert!(matches!(decode_run_pages(encoded, IndexKind::Commit, &mut control).await, Err(DbError::Corrupt(_))));
}

#[semio_framework_async_macros::async_test]
async fn decode_run_rejects_non_ascending_entries() {
    // Hand-build a malformed body bypassing `encode_run`'s own ordering check, to exercise
    // `decode_run`'s independent defensive re-validation.
    let mut writer = ByteWriter::new();
    writer.write_bytes(&RUN_MAGIC);
    writer.write_u8(RUN_VERSION);
    writer.write_u8(IndexKind::Command.tag());
    writer.write_varint_u64(2);
    for key in [b"b".as_slice(), b"a".as_slice()] {
        writer.write_varint_u64(key.len() as u64);
        writer.write_bytes(key);
        writer.write_u8(1);
        writer.write_varint_u64(1);
        writer.write_bytes(b"x");
    }
    let mut bytes = writer.into_bytes();
    let checksum = crc32c(&bytes);
    bytes.extend_from_slice(&checksum.to_le_bytes());
    let mut control = control();
    let bytes = retained_vec(bytes).await;
    assert!(matches!(decode_run_pages(bytes.pages, IndexKind::Command, &mut control).await, Err(DbError::Corrupt(_))));
}

#[semio_framework_async_macros::async_test]
async fn build_run_sorts_and_last_write_wins_on_duplicate_keys() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let handle = IndexHandle::new(&storage, ArtifactId::from("sort"), IndexKind::Command).await;
    let entries = run([entry(b"b", b"1").await, entry(b"a", b"2").await, entry(b"b", b"3").await]).await;
    let mut control = control();
    handle.put_batch(entries, &mut control).await.unwrap();
    assert_eq!(get_bytes(&handle, b"a").await, Some(b"2".to_vec()));
    assert_eq!(get_bytes(&handle, b"b").await, Some(b"3".to_vec()));
}
//#endregion 🔖️SortedRun

//#region 🔖️Merge
#[semio_framework_async_macros::async_test]
async fn merge_runs_prefers_newest_and_respects_drop_tombstones() {
    let older = run([entry(b"a", b"old-a").await, entry(b"b", b"old-b").await]).await;
    let newer = run([tombstone(b"b").await, entry(b"c", b"new-c").await]).await;
    let mut control = control();
    let mut merged = merge_run_entries(older, newer, false, &mut control).await.unwrap();
    assert_entry(merged.get(0).unwrap(), b"a", Some(b"old-a")).await;
    assert_entry(merged.get(1).unwrap(), b"b", None).await;
    assert_entry(merged.get(2).unwrap(), b"c", Some(b"new-c")).await;
    while merged.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn merge_runs_of_zero_runs_is_empty() {
    let mut control = control();
    assert!(merge_run_entries(RunEntries::new(), RunEntries::new(), true, &mut control).await.unwrap().is_empty());
}
//#endregion 🔖️Merge

//#region 🔖️IndexKind
#[semio_framework_async_macros::async_test]
async fn run_id_round_trips_kind_and_sequence_for_every_kind() {
    for kind in IndexKind::ALL {
        for sequence in [0u64, 1, SEQUENCE_MASK] {
            let run_id = make_run_id(kind, sequence).expect("make_run_id");
            assert_eq!(kind_tag_of_run_id(run_id), kind.tag());
            assert_eq!(sequence_of_run_id(run_id), sequence);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn run_id_rejects_sequence_overflowing_the_namespace() {
    assert!(matches!(make_run_id(IndexKind::Command, SEQUENCE_MASK + 1), Err(DbError::LimitExceeded(_))));
}
//#endregion 🔖️IndexKind

//#region 🔖️IndexHandle
#[semio_framework_async_macros::async_test]
async fn index_handle_put_get_delete_round_trips() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
    put_bytes(&handle, b"k1", b"v1").await;
    put_bytes(&handle, b"k2", b"v2").await;
    assert_eq!(get_bytes(&handle, b"k1").await, Some(b"v1".to_vec()));
    assert_eq!(get_bytes(&handle, b"k2").await, Some(b"v2".to_vec()));
    assert_eq!(get_bytes(&handle, b"missing").await, None);
    delete_bytes(&handle, b"k1").await;
    assert_eq!(get_bytes(&handle, b"k1").await, None);
    assert_eq!(get_bytes(&handle, b"k2").await, Some(b"v2".to_vec()));
}

#[semio_framework_async_macros::async_test]
async fn index_handle_put_overwrites_earlier_value_for_same_key() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
    put_bytes(&handle, b"k", b"first").await;
    put_bytes(&handle, b"k", b"second").await;
    assert_eq!(get_bytes(&handle, b"k").await, Some(b"second".to_vec()));
}

#[semio_framework_async_macros::async_test]
async fn index_handle_scan_prefix_returns_sorted_live_entries_only() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
    put_bytes(&handle, b"a/1", b"1").await;
    put_bytes(&handle, b"a/2", b"2").await;
    put_bytes(&handle, b"b/1", b"3").await;
    delete_bytes(&handle, b"a/2").await;
    let mut control = control();
    let mut prefix = retained(b"a/").await;
    let mut scanned = handle.scan_prefix(&prefix, &mut control).await.unwrap();
    assert_eq!(scanned.len(), 1);
    assert_entry(scanned.get(0).unwrap(), b"a/1", Some(b"1")).await;
    while scanned.close_step().unwrap() {}
    while prefix.close_step().unwrap().is_some() {}
}

#[semio_framework_async_macros::async_test]
async fn index_handle_auto_merges_to_stay_within_policy() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let policy = MergePolicy { max_runs_before_merge: 2 };
    let handle = IndexHandle::with_policy(&storage, ArtifactId::from("doc-1"), IndexKind::Command, policy).await;
    for i in 0..6u64 {
        put_bytes(&handle, format!("k{i:03}").as_bytes(), &i.to_le_bytes()).await;
    }
    let shape = stats(&handle).await;
    assert!(shape.run_count <= 2, "run_count {} should respect the merge policy", shape.run_count);
    for i in 0..6u64 {
        let value = get_bytes(&handle, format!("k{i:03}").as_bytes()).await.unwrap();
        assert_eq!(u64::from_le_bytes(value.try_into().expect("8 bytes")), i);
    }
}

#[semio_framework_async_macros::async_test]
async fn index_handle_compact_collapses_to_one_run_and_drops_tombstones() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
    put_bytes(&handle, b"a", b"1").await;
    put_bytes(&handle, b"b", b"2").await;
    delete_bytes(&handle, b"a").await;
    let mut control = control();
    let shape = handle.compact(&mut control).await.unwrap();
    assert_eq!(shape.run_count, 1);
    assert_eq!(shape.entry_count, 1);
    assert_eq!(get_bytes(&handle, b"a").await, None);
    assert_eq!(get_bytes(&handle, b"b").await, Some(b"2".to_vec()));
    handle.verify(&mut control).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn index_handle_compact_of_one_run_is_a_no_op() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let handle = IndexHandle::new(&storage, ArtifactId::from("doc-1"), IndexKind::Command).await;
    put_bytes(&handle, b"a", b"1").await;
    let before = stats(&handle).await;
    let mut control = control();
    let after = handle.compact(&mut control).await.unwrap();
    assert_eq!(before, after);
}

#[semio_framework_async_macros::async_test]
async fn different_kinds_do_not_collide_for_the_same_document() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let document = ArtifactId::from("doc-1");
    let commands = IndexHandle::new(&storage, document.clone(), IndexKind::Command).await;
    let regions = IndexHandle::new(&storage, document, IndexKind::TouchedRegion).await;

    put_bytes(&commands, b"shared-key", b"command-value").await;
    put_bytes(&regions, b"shared-key", b"region-value").await;
    assert_eq!(get_bytes(&commands, b"shared-key").await, Some(b"command-value".to_vec()));
    assert_eq!(get_bytes(&regions, b"shared-key").await, Some(b"region-value".to_vec()));
    assert_eq!(stats(&commands).await.run_count, 1);
    assert_eq!(stats(&regions).await.run_count, 1);
}
//#endregion 🔖️IndexHandle

//#region 🔖️TypedIndexes
#[semio_framework_async_macros::async_test]
async fn command_index_records_and_looks_up_locations() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = CommandIndex::new(&storage, ArtifactId::from("doc-1")).await;
    let location = RecordLocation { segment: 3, offset: 128, len: 64 };
    db_actor::block_on(index.record(42, location)).expect("record");
    assert_eq!(db_actor::block_on(index.lookup(42)).expect("lookup"), Some(location));
    assert_eq!(db_actor::block_on(index.lookup(43)).expect("lookup"), None);
    db_actor::block_on(index.remove(42)).expect("remove");
    assert_eq!(db_actor::block_on(index.lookup(42)).expect("lookup"), None);
}

#[semio_framework_async_macros::async_test]
async fn inverse_index_records_and_looks_up_locations() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = InverseIndex::new(&storage, ArtifactId::from("doc-1")).await;
    let location = RecordLocation { segment: 1, offset: 0, len: 16 };
    db_actor::block_on(index.record(7, location)).expect("record");
    assert_eq!(db_actor::block_on(index.lookup(7)).expect("lookup"), Some(location));
}

#[semio_framework_async_macros::async_test]
async fn actor_seq_index_resolves_and_tracks_latest_per_actor() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = ActorSeqIndex::new(&storage, ArtifactId::from("doc-1")).await;
    let alice = ActorId::from("alice");
    let bob = ActorId::from("bob");
    db_actor::block_on(index.record(&alice, 1, 100)).expect("record");
    db_actor::block_on(index.record(&alice, 2, 101)).expect("record");
    db_actor::block_on(index.record(&bob, 1, 200)).expect("record");

    assert_eq!(db_actor::block_on(index.lookup(&alice, 1)).expect("lookup"), Some(100));
    assert_eq!(db_actor::block_on(index.lookup(&alice, 2)).expect("lookup"), Some(101));
    assert_eq!(db_actor::block_on(index.latest_for_actor(&alice)).expect("latest"), Some((2, 101)));
    assert_eq!(db_actor::block_on(index.latest_for_actor(&bob)).expect("latest"), Some((1, 200)));
    assert_eq!(db_actor::block_on(index.latest_for_actor(&ActorId::from("carol"))).expect("latest"), None);
}

#[semio_framework_async_macros::async_test]
async fn actor_seq_index_rejects_actor_id_with_embedded_nul() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = ActorSeqIndex::new(&storage, ArtifactId::from("doc-1")).await;
    let unsafe_actor = ActorId::from("bad\u{0}actor");
    assert!(matches!(db_actor::block_on(index.record(&unsafe_actor, 1, 1)), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn frontier_index_round_trips_and_tracks_latest() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = FrontierIndex::new(&storage, ArtifactId::from("doc-1")).await;
    let first = Frontier { document: ArtifactId::from("doc-1"), head_seq: 1, commit_seq: 1, chain_hash: [1u8; 32], epoch: 0 };
    let second = Frontier { document: ArtifactId::from("doc-1"), head_seq: 5, commit_seq: 2, chain_hash: [2u8; 32], epoch: 1 };
    db_actor::block_on(index.record(&first)).expect("record");
    db_actor::block_on(index.record(&second)).expect("record");

    assert_eq!(db_actor::block_on(index.lookup(1)).expect("lookup"), Some(first));
    assert_eq!(db_actor::block_on(index.latest()).expect("latest"), Some(second));
}

#[semio_framework_async_macros::async_test]
async fn touched_region_index_accumulates_sorted_unique_seqs() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = TouchedRegionIndex::new(&storage, ArtifactId::from("doc-1")).await;
    db_actor::block_on(index.record_touch(b"region-a", 5)).expect("record_touch");
    db_actor::block_on(index.record_touch(b"region-a", 2)).expect("record_touch");
    db_actor::block_on(index.record_touch(b"region-a", 5)).expect("record_touch");
    assert_eq!(db_actor::block_on(index.touching(b"region-a")).expect("touching"), vec![2, 5]);
    assert_eq!(db_actor::block_on(index.touching(b"region-b")).expect("touching"), Vec::<u64>::new());
}

#[semio_framework_async_macros::async_test]
async fn commit_index_round_trips() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = CommitIndex::new(&storage, ArtifactId::from("doc-1")).await;
    db_actor::block_on(index.record("ck-abc123", 9)).expect("record");
    assert_eq!(db_actor::block_on(index.lookup("ck-abc123")).expect("lookup"), Some(9));
    assert_eq!(db_actor::block_on(index.lookup("ck-missing")).expect("lookup"), None);
}

#[semio_framework_async_macros::async_test]
async fn full_text_index_search_finds_indexed_documents() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = FullTextIndex::new(&storage, ArtifactId::from("doc-1")).await;
    db_actor::block_on(index.index_document(1, "The Quick Brown Fox")).expect("index");
    db_actor::block_on(index.index_document(2, "quick jumps")).expect("index");

    assert_eq!(db_actor::block_on(index.search("quick")).expect("search"), vec![1, 2]);
    assert_eq!(db_actor::block_on(index.search("QUICK")).expect("search"), vec![1, 2]);
    assert_eq!(db_actor::block_on(index.search("fox")).expect("search"), vec![1]);
    assert_eq!(db_actor::block_on(index.search("absent")).expect("search"), Vec::<u64>::new());
}

#[semio_framework_async_macros::async_test]
async fn conflict_index_accumulates_multiple_records_per_command() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = ConflictIndex::new(&storage, ArtifactId::from("doc-1")).await;
    index.record_conflict(5, retained(b"region-collision").await).await.unwrap();
    index.record_conflict(5, retained(b"constraint-violation").await).await.unwrap();
    index.record_conflict(6, retained(b"other").await).await.unwrap();
    let mut records = index.conflicts_for(5).await.unwrap();
    assert_eq!(read_retained(records.get(0).unwrap()).await, b"region-collision");
    assert_eq!(read_retained(records.get(1).unwrap()).await, b"constraint-violation");
    while records.close_step().unwrap() {}
    let mut records = index.conflicts_for(6).await.unwrap();
    assert_eq!(read_retained(records.get(0).unwrap()).await, b"other");
    while records.close_step().unwrap() {}
    let mut records = index.conflicts_for(7).await.unwrap();
    assert_eq!(records.len(), 0);
    while records.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn projection_index_resolves_exact_and_floor_lookups_scoped_to_projection_id() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = ProjectionIndex::new(&storage, ArtifactId::from("doc-1")).await;
    index.record("by-author", 10, retained(b"state-10").await).await.unwrap();
    index.record("by-author", 20, retained(b"state-20").await).await.unwrap();
    let mut exact = index.at("by-author", 10).await.unwrap().unwrap();
    assert_eq!(read_retained(&exact).await, b"state-10");
    while exact.close_step().unwrap().is_some() {}
    assert!(index.at("by-author", 15).await.unwrap().is_none());
    let (sequence, mut floor) = index.latest_at_or_before("by-author", 15).await.unwrap().unwrap();
    assert_eq!(sequence, 10);
    assert_eq!(read_retained(&floor).await, b"state-10");
    while floor.close_step().unwrap().is_some() {}
    let (sequence, mut floor) = index.latest_at_or_before("by-author", 20).await.unwrap().unwrap();
    assert_eq!(sequence, 20);
    assert_eq!(read_retained(&floor).await, b"state-20");
    while floor.close_step().unwrap().is_some() {}
    assert_eq!(db_actor::block_on(index.latest_at_or_before("by-author", 5)).expect("latest_at_or_before"), None);
    // 🎯️ "by-color" sorts after "by-author" but has no entries at all — must not fall back to
    // a lexicographically-earlier projection's entry.
    assert_eq!(db_actor::block_on(index.latest_at_or_before("by-color", 100)).expect("latest_at_or_before"), None);
}

#[semio_framework_async_macros::async_test]
async fn projection_index_rejects_projection_id_with_embedded_nul() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = ProjectionIndex::new(&storage, ArtifactId::from("doc-1")).await;
    assert!(matches!(index.record("bad\u{0}id", 1, retained(&[1]).await).await, Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn preview_index_coalesces_latest_publish_or_withdraw_per_actor_and_key() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = PreviewIndex::new(&storage, ArtifactId::from("doc-1")).await;
    let alice = ActorId::from("alice");

    index.publish(&alice, "drag-ghost", retained(&[1]).await).await.unwrap();
    let mut latest = index.latest(&alice, "drag-ghost").await.unwrap().unwrap();
    assert_eq!(read_retained(&latest).await, [1]);
    while latest.close_step().unwrap().is_some() {}
    index.publish(&alice, "drag-ghost", retained(&[2]).await).await.unwrap();
    index.publish(&alice, "cursor", retained(&[9]).await).await.unwrap();
    let mut for_alice = index.for_actor(&alice).await.unwrap();
    assert_eq!(for_alice.len(), 2);
    while for_alice.close_step().unwrap() {}

    db_actor::block_on(index.withdraw(&alice, "drag-ghost")).expect("withdraw");
    assert!(index.latest(&alice, "drag-ghost").await.unwrap().is_none());
    let mut for_alice = index.for_actor(&alice).await.unwrap();
    assert_eq!(for_alice.len(), 1);
    while for_alice.close_step().unwrap() {}
}

#[semio_framework_async_macros::async_test]
async fn preview_index_rejects_actor_id_with_embedded_nul() {
    let storage = MemoryStorage::new(db_storage::db_io_test_pool()).await.unwrap();
    let index = PreviewIndex::new(&storage, ArtifactId::from("doc-1")).await;
    let unsafe_actor = ActorId::from("bad\u{0}actor");
    assert!(matches!(index.publish(&unsafe_actor, "k", retained(&[1]).await).await, Err(DbError::InvalidArgument(_))));
}
//#endregion 🔖️TypedIndexes
