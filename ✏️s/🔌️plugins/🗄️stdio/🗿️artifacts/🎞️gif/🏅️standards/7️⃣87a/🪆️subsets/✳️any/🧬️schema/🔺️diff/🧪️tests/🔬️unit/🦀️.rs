
use super::*;
use crate::STDIO_GIF_DOCUMENT_SCHEMA;
use crate::standards::v87a::subsets::any::schema::snapshot::GifRgb;

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

#[semio_framework_async_macros::async_test]
async fn absorb_law_holds_over_curated_ops() {
    let base = GifSnapshot { images: vec![img(1, 2, 2), img(2, 2, 2), img(3, 2, 2)], ..GifSnapshot::default() };
    let mid = {
        let mut s = base.clone();
        s.images.insert(1, img(9, 2, 2));
        s.images.remove(0);
        s
    };
    let after = {
        let mut s = mid.clone();
        s.images[0].interlace = true;
        s.images.push(img(5, 2, 2));
        s
    };
    let mut d1 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &mid);
    let d2 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert_eq!(d1.apply(&base).unwrap(), after);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = GifSnapshot { width: 4, height: 4, images: vec![img(1, 4, 4)], ..GifSnapshot::default() };
    let b = GifSnapshot { width: 4, height: 4, images: vec![img(1, 4, 4), img(2, 2, 2)], ..GifSnapshot::default() };
    let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b);
    assert_eq!(ab.apply(&a).unwrap(), b);
    let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&b, &a);
    assert_eq!(ba.apply(&b).unwrap(), a);
    assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = GifSnapshot { images: vec![img(1, 2, 2), img(2, 2, 2)], ..GifSnapshot::default() };
    let next = {
        let mut s = base.clone();
        s.images[0].interlace = true;
        s.images.remove(1);
        s.images.push(img(7, 3, 3));
        s.background_color_index = 5;
        s
    };
    let d = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &next);
    let mutated = d.apply(&base).unwrap();
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&mutated).unwrap(), base);
}

/// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
/// field, with asymmetric image-collection lengths (F1's structural trap: a single
/// index-keyed `between()` call can show `removed` XOR `added`, never both — so assertions are
/// split across both directions, per `f1-closer-report.md` §4.4).
#[semio_framework_async_macros::async_test]
async fn field_sweep_covers_every_mutable_field() {
    let sweep_a = GifSnapshot {
        schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        width: 10,
        height: 8,
        gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }),
        background_color_index: 0,
        pixel_aspect_ratio: 0,
        images: vec![img(1, 2, 2), img(2, 2, 2)],
    };
    let mut sweep_b = GifSnapshot {
        schema: STDIO_GIF_DOCUMENT_SCHEMA.into(),
        width: 20,
        height: 16,
        gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb { r: 9, g: 9, b: 9 }; 4] }),
        background_color_index: 3,
        pixel_aspect_ratio: 7,
        images: vec![img(1, 2, 2)],
    };
    sweep_b.images[0].interlace = true;
    sweep_b.images.push(img(5, 3, 3));
    sweep_b.images.push(img(6, 3, 3));

    let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_b);
    assert_eq!(ab.apply(&sweep_a).unwrap(), sweep_b);
    assert!(ab.width.is_some());
    assert!(ab.height.is_some());
    assert!(ab.gct.is_some());
    assert!(ab.background_color_index.is_some());
    assert!(ab.pixel_aspect_ratio.is_some());
    let images_ab = ab.images.as_ref().expect("images must differ");
    assert!(!images_ab.modified.is_empty(), "sweep must exercise a modified image");
    assert!(!images_ab.added.is_empty(), "sweep must exercise an added image (b is longer)");

    let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_b, &sweep_a);
    assert_eq!(ba.apply(&sweep_b).unwrap(), sweep_a);
    let images_ba = ba.images.as_ref().expect("images must differ");
    assert!(!images_ba.removed.is_empty(), "reverse direction must exercise a removed image (a is shorter)");

    assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
}

/// 🧪️ Tri-state nullable field: `gct` going from `Some` to `None` must be `Some(None)`, not
/// absent from the diff.
#[semio_framework_async_macros::async_test]
async fn gct_tristate_removal_is_some_none() {
    let a = GifSnapshot { gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb::default(); 2] }), ..GifSnapshot::default() };
    let b = GifSnapshot { gct: None, ..GifSnapshot::default() };
    let d = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b);
    assert_eq!(d.gct, Some(None));
    assert_eq!(d.apply(&a).unwrap(), b);
}

/// 🧪️ F6: `DiffCodec` round-trip laws for the hand-rolled `GifDiff` text/binary grammar —
/// exercises scalars, both tri-states (`gct` at the top level, `lct` inside a modified image),
/// and the `images` collection triple (`removed`/`modified`/`added`) simultaneously via a real
/// `between()` result — mirrors gif89a's `diff_codec_text_binary_roundtrip_law`.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = GifSnapshot { width: 10, height: 8, gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }), images: vec![img(1, 2, 2), img(2, 2, 2)], ..GifSnapshot::default() };
    let mut ib0 = img(1, 2, 2);
    ib0.interlace = true;
    ib0.lct = None;
    let b = GifSnapshot { width: 20, height: 16, gct: None, images: vec![ib0, img(6, 3, 3), img(7, 3, 3)], ..GifSnapshot::default() };
    let cases = vec![GifDiff::default(), <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b), <GifDiff as DiffAlgebra<GifSnapshot>>::between(&b, &a)];
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
