use super::*;
use crate::standards::v87a::subsets::any::schema::snapshot::GifRgb;
use crate::STDIO_GIF_DOCUMENT_SCHEMA;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn img(seed: u8, w: u32, h: u32) -> GifImage {
    GifImage { left: 0, top: 0, width: w, height: h, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }), indices: vec![0u8; (w * h) as usize] }
}

/// 🧪️ Canonical absorb case 1: `Insert(2,f)` then `Remove(0)` → `{removed:[0], added:[(1,f)]}`.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_remove_before_shifts_index() {
    let f = img(9, 2, 2);
    let mut d1 = GifImagesDiff { added: vec![GifImageAdded { index: 2, image: f.clone() }], ..Default::default() };
    let d2 = GifImagesDiff { removed: vec![0], ..Default::default() };
    d1.absorb(d2);
    assert_eq!(d1.removed, vec![0]);
    assert_eq!(d1.added, vec![GifImageAdded { index: 1, image: f }]);
    assert!(d1.modified.is_empty());
}

/// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` → BOTH survive as
/// `added:[(2,g),(3,f)]` — the exact LWW-slot bug this recipe replaces.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_insert_same_index_both_survive() {
    let f = img(1, 2, 2);
    let g = img(2, 2, 2);
    let mut d1 = GifImagesDiff { added: vec![GifImageAdded { index: 2, image: f.clone() }], ..Default::default() };
    let d2 = GifImagesDiff { added: vec![GifImageAdded { index: 2, image: g.clone() }], ..Default::default() };
    d1.absorb(d2);
    assert_eq!(d1.added, vec![GifImageAdded { index: 2, image: g }, GifImageAdded { index: 3, image: f },]);
}

/// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,v)` patches INTO the added
/// payload — merged has only `added`, no separate `modified` entry.
#[semio_framework_async_macros::async_test]
async fn absorb_insert_then_set_field_patches_into_added() {
    let f = img(1, 2, 2);
    let mut d1 = GifImagesDiff { added: vec![GifImageAdded { index: 1, image: f.clone() }], ..Default::default() };
    let d2 = GifImagesDiff { modified: vec![GifImageModified { index: 1, diff: GifImageDiff { interlace: Some(true), ..Default::default() } }], ..Default::default() };
    d1.absorb(d2);
    assert!(d1.modified.is_empty());
    assert_eq!(d1.added.len(), 1);
    assert!(d1.added[0].image.interlace);
    assert_eq!(d1.added[0].index, 1);
}

/// 🧪️ Tri-state nullable field: `gct` going from `Some` to `None` must be `Some(None)`, not
/// absent from the diff.
#[semio_framework_async_macros::async_test]
async fn gct_tristate_removal_is_some_none() {
    let a = GifSnapshot { gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb::default(); 2] }), ..GifSnapshot::default() };
    let b = GifSnapshot { gct: None, ..GifSnapshot::default() };
    let d = GifDiff { gct: Some(None), ..GifDiff::default() };
    assert_eq!(d.gct, Some(None));
    assert_eq!(protocol::apply_diff(&d, &a).unwrap(), b);
}

/// 🧪️ F6: `DiffCodec` round-trip laws for the hand-rolled `GifDiff` text/binary grammar —
/// exercises scalars, both tri-states (`gct` at the top level, `lct` inside a modified image),
/// and the `images` collection triple (`removed`/`modified`/`added`) simultaneously via a real
/// `between()` result — mirrors gif89a's `diff_codec_text_binary_roundtrip_law`.
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
