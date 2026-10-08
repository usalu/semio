use super::*;
use crate::standards::v1::subsets::audio::io::text::snapshot::{enc_snapshot,dec_snapshot};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn channel(seed: f32, len: usize) -> SemioAudioChannel {
    SemioAudioChannel { samples: (0..len).map(|i| seed + i as f32 * 0.1).collect() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> SemioAudioSnapshot {
    SemioAudioSnapshot {
        sample_rate: 44_100,
        format: SemioAudioFormat::Pcm16,
        channels: vec![channel(0.0, 4), channel(1.0, 4), channel(2.0, 4)],
        tags: vec![SemioAudioTag { key: "title".into(), value: "one".into() }],
        ..SemioAudioSnapshot::default()
    }
}

/// 🧪️ Canonical absorb case 1: `InsertChannel(2,c)` then `RemoveChannel(0)` →
/// `{removed:[0], added:[(1,c)]}`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let c = channel(9.0, 2);
    let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: c.clone() }], ..Default::default() };
    let d2: SemioAudioChannelsDiff = IndexedTripleDiff { removed: vec![0], ..Default::default() };
    channels_absorb(&mut d1, d2);
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.added, vec![IndexAdded { index: 1, item: c }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `InsertChannel(2,c)` then `InsertChannel(2,d)` → BOTH survive.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let c = channel(1.0, 2);
    let d = channel(2.0, 2);
    let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: c.clone() }], ..Default::default() };
    let d2: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: d.clone() }], ..Default::default() };
    channels_absorb(&mut d1, d2);
    assert_eq!(d1.added, vec![IndexAdded { index: 2, item: d }, IndexAdded { index: 3, item: c }]);
}

/// 🧪️ Canonical absorb case 3: `InsertChannel(1,c)` then `SetChannelSamples(1,..)` patches
/// INTO the added payload — merged has only `added`, no separate `modified` entry.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let c = channel(1.0, 2);
    let mut d1: SemioAudioChannelsDiff = IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: c.clone() }], ..Default::default() };
    let d2: SemioAudioChannelsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: 1, diff: SemioAudioChannelDiff { samples: Some(vec![9.0, 9.0]) } }], ..Default::default() };
    channels_absorb(&mut d1, d2);
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert_eq!(d1.added[0].item.samples, vec![9.0, 9.0]);
    assert_eq!(d1.added[0].index, 1);
}

/// 🧪️ `DiffCodec` text/binary round-trip law — exercises scalars and both collection triples
/// (`removed`/`modified`/`added`) simultaneously via a real `between()` result.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioAudioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioAudioDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}

#[semio_framework_async_macros::async_test]
async fn snapshot_bracket_codec_round_trips() {
    let s = base_snapshot();
    let encoded = enc_snapshot(&s);
    let decoded = dec_snapshot(&encoded).expect("decode");
    assert_eq!(decoded, s);
}
