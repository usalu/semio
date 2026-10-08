use super::*;
use crate::standards::v89a::subsets::any::schema::snapshot::{GifRgb, STDIO_GIF89A_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame(seed: u8, w: u32, h: u32) -> GifFrame {
    GifFrame {
        left: 0,
        top: 0,
        width: w,
        height: h,
        interlace: false,
        lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }),
        indices: vec![0u8; (w * h) as usize],
        delay_cs: 10,
        disposal: GifDisposal::DoNotDispose,
        transparent_index: None,
        user_input: false,
        plain_text: None,
    }
}

/// 🧪️ Canonical absorb case 1: `InsertFrame(2,f)` then `RemoveFrame(0)` →
/// `{removed:[0], added:[(1,f)]}`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let f = frame(9, 2, 2);
    let mut d1 = GifFramesDiff { added: vec![GifFrameAdded { index: 2, frame: f.clone() }], ..Default::default() };
    let d2 = GifFramesDiff { removed: vec![0], ..Default::default() };
    d1.absorb(d2);
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.added, vec![GifFrameAdded { index: 1, frame: f }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `InsertFrame(2,f)` then `InsertFrame(2,g)` → BOTH survive as
/// `added:[(2,g),(3,f)]` — the exact LWW-slot bug this recipe replaces.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let f = frame(1, 2, 2);
    let g = frame(2, 2, 2);
    let mut d1 = GifFramesDiff { added: vec![GifFrameAdded { index: 2, frame: f.clone() }], ..Default::default() };
    let d2 = GifFramesDiff { added: vec![GifFrameAdded { index: 2, frame: g.clone() }], ..Default::default() };
    d1.absorb(d2);
    assert_eq!(d1.added, vec![GifFrameAdded { index: 2, frame: g }, GifFrameAdded { index: 3, frame: f },]);
}

/// 🧪️ Canonical absorb case 3: `InsertFrame(1,f)` then `SetFrameDelay(1,42)` patches INTO the
/// added payload — merged has only `added`, no separate `modified` entry.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let f = frame(1, 2, 2);
    let mut d1 = GifFramesDiff { added: vec![GifFrameAdded { index: 1, frame: f.clone() }], ..Default::default() };
    let d2 = GifFramesDiff { modified: vec![GifFrameModified { index: 1, diff: GifFrameDiff { delay_cs: Some(42), ..Default::default() } }], ..Default::default() };
    d1.absorb(d2);
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert_eq!(d1.added[0].frame.delay_cs, 42);
    assert_eq!(d1.added[0].index, 1);
}

/// 🧪️ F6-PILOT: `DiffCodec` round-trip laws for the hand-rolled `GifDiff` text/binary grammar
/// — exercises scalars, both tri-states (`gct`/`loop_count` at the top level, `lct`/
/// `transparent_index`/`plain_text` inside a modified frame), and all three collection triples
/// (`removed`/`modified`/`added`) simultaneously via a real `between()` result.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let cases = demo_diff_cases();
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = GifDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = GifDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
