use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame(seed: u8, len: usize) -> SemioImageFrame {
    SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
}

/// 🧪️ Canonical absorb case 1: `InsertFrame(2,f)` then `RemoveFrame(0)` → `{removed:[0],
/// added:[(1,f)]}`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let f = frame(9, 4);
    let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
    let d2 = SemioImageFramesDiff { removed: vec![0], ..Default::default() };
    let absorbed = frames_absorb(d1, d2);
    assert_eq!(absorbed.removed, vec![0]);
    assert_eq!(absorbed.added, vec![IndexAdded { index: 1, item: f }]);
    assert!(absorbed.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `InsertFrame(2,f)` then `InsertFrame(2,g)` → both survive.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let f = frame(1, 4);
    let g = frame(2, 4);
    let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
    let d2 = SemioImageFramesDiff { added: vec![IndexAdded { index: 2, item: g.clone() }], ..Default::default() };
    let absorbed = frames_absorb(d1, d2);
    assert_eq!(absorbed.added, vec![IndexAdded { index: 2, item: g }, IndexAdded { index: 3, item: f }]);
}

/// 🧪️ Canonical absorb case 3: `InsertFrame(1,f)` then `SetFrameDelay(1,42)` patches INTO the
/// added payload.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let f = frame(1, 4);
    let d1 = SemioImageFramesDiff { added: vec![IndexAdded { index: 1, item: f.clone() }], ..Default::default() };
    let d2 = SemioImageFramesDiff { modified: vec![IndexModified { index: 1, diff: SemioImageFrameDiff { delay_ms: Some(42), rgba8: None } }], ..Default::default() };
    let absorbed = frames_absorb(d1, d2);
    assert!(absorbed.modified.is_empty());
    assert_eq!(absorbed.added.len(), 1);
    assert_eq!(absorbed.added[0].item.delay_ms, 42);
    assert_eq!(absorbed.added[0].index, 1);
}

/// 🧪️ Canonical absorb case 4: Modify+Remove annihilates the modify.
#[semio_framework_async_macros::async_test]
async fn absorb_modify_then_remove_drops_modify() {
    let d1 = SemioImageFramesDiff { modified: vec![IndexModified { index: 1, diff: SemioImageFrameDiff { delay_ms: Some(50), rgba8: None } }], ..Default::default() };
    let d2 = SemioImageFramesDiff { removed: vec![1], ..Default::default() };
    let absorbed = frames_absorb(d1, d2);
    assert!(absorbed.modified.is_empty());
    assert_eq!(absorbed.removed, vec![1]);
}

/// 🧪️ `DiffCodec` round-trip laws for the hand-rolled `SemioImageDiff` text/binary grammar —
/// scalars, the `icc` tri-state, and both collection triples simultaneously via a real
/// `between()` result.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    // 🌱 Reuses `demo_diff_cases()` (single source of truth, also feeds
    // `diff_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs`)
    // rather than an independent copy of the same base/other fixture pair.
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioImageDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioImageDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
