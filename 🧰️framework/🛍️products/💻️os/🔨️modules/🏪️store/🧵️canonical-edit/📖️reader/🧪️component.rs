//! 🧪️ Typed reader byte parity, exact Arc ownership, cancellation, and worker laws.

use super::*;
use super::super::borrowed_tests::{fixture, MapLifetime, MapMutation, MapRetirementFactory};
use std::sync::atomic::Ordering;

//#region 📦️ReaderFixtures
struct RootRetirementFactory;
struct EmptyRetirement;
impl ErasedSnapshotRetirement for EmptyRetirement {
    fn close_step(&mut self, _items: usize, _bytes: usize) -> Result<SnapshotRetirementStep, String> { Ok(SnapshotRetirementStep::Complete) }
    fn terminal_is_empty(&self) -> bool { true }
}
impl SnapshotRetirementFactory<Edit<MapMutation>> for RootRetirementFactory {
    fn retire(&self, root: Arc<Edit<MapMutation>>) -> Box<dyn ErasedSnapshotRetirement> {
        match Arc::into_inner(root) {
            Some(edit) => Box::new(ArtifactStoreDecodedEditRetirement::new(edit, Arc::new(MapRetirementFactory))),
            None => Box::new(EmptyRetirement),
        }
    }
}

fn make_reader() -> (ArtifactCanonicalJsonReader<Edit<MapMutation>>, serde_json::Value, Arc<MapLifetime>) {
    let (edit, fixture, lifetime) = fixture();
    (ArtifactCanonicalJsonReader::new(Arc::new(edit), Arc::new(RootRetirementFactory)), fixture, lifetime)
}

fn finish(reader: &mut ArtifactCanonicalJsonReader<Edit<MapMutation>>, bytes: usize) -> Vec<u8> {
    let mut result = Vec::new();
    for _ in 0..100_000 {
        let prior = reader.completed_bytes();
        let mut output = [0; 512];
        let count = reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }, &mut output).unwrap();
        assert!(count <= bytes.min(256));
        assert_eq!(reader.completed_bytes() - prior, count as u64);
        result.extend_from_slice(&output[..count]);
        if reader.is_complete() { return result; }
    }
    panic!("canonical reader did not finish");
}

fn close(reader: &mut ArtifactCanonicalJsonReader<Edit<MapMutation>>) {
    reader.begin_close();
    assert!(matches!(reader.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }).unwrap(), SnapshotRetirementStep::Blocked));
    for _ in 0..100_000 {
        match reader.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).unwrap() {
            SnapshotRetirementStep::Complete => { assert!(reader.terminal_is_empty()); return; }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 1),
            SnapshotRetirementStep::Blocked => panic!("positive reader retirement grant blocked"),
        }
    }
    panic!("canonical reader did not retire");
}

fn retire(root: Arc<Edit<MapMutation>>, lifetime: &MapLifetime) {
    let mut retirement = RootRetirementFactory.retire(root);
    for _ in 0..100_000 {
        if matches!(retirement.close_step(1, 1).unwrap(), SnapshotRetirementStep::Complete) {
            assert!(retirement.terminal_is_empty());
            assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 1);
            return;
        }
    }
    panic!("transferred reader root did not retire");
}
//#endregion 📦️ReaderFixtures

//#region 🧪️ReaderLifecycle
#[test]
fn canonical_reader_large_borrowed_map_matches_serde_and_transfers_exact_root() {
    let reader_fixture: serde_json::Value = serde_json::from_str(include_str!("../🧪️fixtures/🔣️canonical-reader.json")).unwrap();
    for bytes in [1, 7, 4096] {
        let (mut reader, fixture, lifetime) = make_reader();
        let address = Arc::as_ptr(reader.owned.root.as_ref().unwrap());
        let expected = serde_json::to_vec(reader.owned.root.as_ref().unwrap().as_ref()).unwrap();
        assert_eq!(expected, fixture["expectedJson"].as_str().unwrap().as_bytes());
        assert_eq!(expected.len() as u64, reader_fixture["expectedByteLength"].as_u64().unwrap());
        assert!(reader.take_root().is_none());
        assert_eq!(reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: bytes }, &mut [0; 256]).unwrap(), 0);
        assert_eq!(reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }, &mut [0; 256]).unwrap(), 0);
        assert_eq!(reader.completed_bytes(), 0);
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        let actual = finish(&mut reader, bytes);
        assert_eq!(actual, expected);
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(&actual);
        assert_eq!(hash.finalize().iter().map(|byte| format!("{byte:02x}")).collect::<String>(), reader_fixture["expectedJsonSha256"].as_str().unwrap());
        let root = reader.take_root().unwrap();
        assert_eq!(Arc::as_ptr(&root), address);
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 0);
        close(&mut reader);
        retire(root, &lifetime);
    }
}

#[test]
fn canonical_reader_cancel_before_poll_mid_key_and_after_completion_retires_exact_root() {
    for stage in 0..3 {
        let (mut reader, fixture, lifetime) = make_reader();
        if stage == 1 {
            let target = fixture["expectedJson"].as_str().unwrap().find("key-").unwrap() as u64 + 128;
            while reader.completed_bytes() < target { reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }, &mut [0; 1]).unwrap(); }
            assert!(lifetime.active_iterators.load(Ordering::SeqCst) > 0);
            assert!(reader.take_root().is_none());
            reader = std::thread::spawn(move || { reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }, &mut [0; 1]).unwrap(); reader }).join().unwrap();
        } else if stage == 2 { finish(&mut reader, 7); }
        reader.cancel();
        let prior = reader.completed_bytes();
        assert_eq!(reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }, &mut [0; 256]).unwrap(), 0);
        assert_eq!(reader.completed_bytes(), prior);
        close(&mut reader);
        assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
        assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn canonical_reader_rebound_root_rejected_before_borrowed_reference_use() {
    let (mut reader, _, lifetime) = make_reader();
    for _ in 0..100 { reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }, &mut [0; 7]).unwrap(); }
    let original = reader.owned.root.replace(Arc::new(fixture().0)).unwrap();
    assert_eq!(reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }, &mut [0; 7]).unwrap_err(), "canonical-edit.borrowed-root-rebound");
    reader.owned.root = Some(original);
    close(&mut reader);
    assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 1);

    let (mut reader, _, lifetime) = make_reader();
    for _ in 0..100 { reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }, &mut [0; 7]).unwrap(); }
    reader.cancel();
    reader.begin_close();
    while !reader.owned.encoder.terminal_is_empty() {
        assert!(!reader.is_complete());
        assert!(reader.take_root().is_none());
        reader.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).unwrap();
    }
    let root = reader.take_root().unwrap();
    assert_eq!(lifetime.active_iterators.load(Ordering::SeqCst), 0);
    close(&mut reader);
    retire(root, &lifetime);
}

#[test]
fn canonical_reader_unclosed_drop_preserves_owned_root_and_does_not_double_panic() {
    let (mut reader, _, lifetime) = make_reader();
    for _ in 0..100 { reader.encode_chunk(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }, &mut [0; 7]).unwrap(); }
    assert!(lifetime.active_iterators.load(Ordering::SeqCst) > 0);
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(reader))).is_err());
    assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 0);
    assert!(lifetime.active_iterators.load(Ordering::SeqCst) > 0);
    let (reader, _, lifetime) = make_reader();
    assert!(std::thread::spawn(move || { let _reader = reader; panic!("primary reader failure"); }).join().is_err());
    assert_eq!(lifetime.root_drops.load(Ordering::SeqCst), 0);
}
//#endregion 🧪️ReaderLifecycle
