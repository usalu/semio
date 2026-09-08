
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
    let base = SemioImageSnapshot { frames: vec![frame(1, 4), frame(2, 4)], ..SemioImageSnapshot::default() };
    let mid = {
        let mut s = base.clone();
        s.frames[1].delay_ms = 50;
        s
    };
    let after = {
        let mut s = mid.clone();
        s.frames.remove(1);
        s
    };
    let d1 = frames_between(&base.frames, &mid.frames).expect("modify diff");
    let d2 = frames_between(&mid.frames, &after.frames).expect("remove diff");
    let absorbed = frames_absorb(d1, d2);
    assert!(absorbed.modified.is_empty());
    assert_eq!(absorbed.removed, vec![1]);
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_holds_over_curated_ops() {
    let base = SemioImageSnapshot { frames: vec![frame(1, 4), frame(2, 4), frame(3, 4)], metadata: vec![SemioImageMetadataEntry { key: "a".into(), value: "1".into() }], ..SemioImageSnapshot::default() };
    let mid = {
        let mut s = base.clone();
        s.frames.insert(1, frame(9, 4));
        s.frames.remove(0);
        s.metadata.push(SemioImageMetadataEntry { key: "b".into(), value: "2".into() });
        s
    };
    let after = {
        let mut s = mid.clone();
        s.frames[0].delay_ms = 250;
        s.frames.push(frame(5, 4));
        s.metadata[0].value = "1-changed".into();
        s
    };
    let mut d1 = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&base, &mid);
    let d2 = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = SemioImageSnapshot { width: 4, height: 4, frames: vec![frame(1, 16)], ..SemioImageSnapshot::default() };
    let b = SemioImageSnapshot { width: 4, height: 4, frames: vec![frame(1, 16), frame(2, 4)], colorspace: SemioColorspace::Grayscale, ..SemioImageSnapshot::default() };
    let ab = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &b);
    assert_eq!(ab.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    let ba = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&b, &a);
    assert_eq!(ba.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = SemioImageSnapshot { frames: vec![frame(1, 4), frame(2, 4)], icc: Some(vec![1, 2]), ..SemioImageSnapshot::default() };
    let next = {
        let mut s = base.clone();
        s.frames[0].delay_ms = 400;
        s.frames.remove(1);
        s.frames.push(frame(7, 9));
        s.icc = None;
        s.metadata.push(SemioImageMetadataEntry { key: "k".into(), value: "v".into() });
        s
    };
    let d = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&base, &next);
    let mutated = d.apply(&base).expect("apply must succeed for a well-formed fixture");
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&mutated).expect("apply must succeed for a well-formed fixture"), base);
}

/// 🧪️ field_sweep — THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable
/// field, including the `icc` tri-state exercising BOTH `Some(Some(_))` and `Some(None)`, and
/// asymmetric collection lengths (a single same-direction `between()` shows removed XOR
/// added, never both — split across both directions).
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let sweep_a = SemioImageSnapshot {
        schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
        width: 10,
        height: 8,
        colorspace: SemioColorspace::Rgb,
        bit_depth: 8,
        frames: vec![frame(1, 4), frame(2, 4)],
        icc: Some(vec![1, 2, 3]),
        metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "old".into() }, SemioImageMetadataEntry { key: "gone".into(), value: "bye".into() }],
    };
    let sweep_b = SemioImageSnapshot {
        schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
        width: 20,
        height: 16,
        colorspace: SemioColorspace::GrayscaleAlpha,
        bit_depth: 16,
        frames: vec![
            {
                let mut f = frame(1, 4);
                f.delay_ms = 500;
                f
            },
            frame(6, 9),
            frame(7, 9),
        ],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "keep".into(), value: "new".into() }, SemioImageMetadataEntry { key: "fresh".into(), value: "hi".into() }],
    };

    let ab = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_a, &sweep_b);
    assert_eq!(ab.apply(&sweep_a).expect("apply must succeed for a well-formed fixture"), sweep_b);
    assert!(ab.width.is_some());
    assert!(ab.height.is_some());
    assert!(ab.colorspace.is_some());
    assert!(ab.bit_depth.is_some());
    assert_eq!(ab.icc, Some(None), "icc Some->None must be tri-state Some(None)");
    let frames_ab = ab.frames.as_ref().expect("frames must differ");
    assert!(!frames_ab.modified.is_empty(), "sweep must exercise a modified frame");
    assert!(!frames_ab.added.is_empty(), "sweep must exercise an added frame (b is longer)");
    assert!(frames_ab.modified[0].diff.delay_ms.is_some());
    let metadata_ab = ab.metadata.as_ref().expect("metadata must differ");
    assert!(!metadata_ab.modified.is_empty(), "metadata: modified not exercised");
    assert!(!metadata_ab.removed.is_empty(), "metadata: removed not exercised");
    assert!(!metadata_ab.added.is_empty(), "metadata: added not exercised");

    let ba = <SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_b, &sweep_a);
    assert_eq!(ba.apply(&sweep_b).expect("apply must succeed for a well-formed fixture"), sweep_a);
    assert_eq!(ba.icc, Some(Some(vec![1, 2, 3])), "icc None->Some must be tri-state Some(Some(_))");
    let frames_ba = ba.frames.as_ref().expect("frames must differ");
    assert!(!frames_ba.removed.is_empty(), "reverse direction must exercise a removed frame (a is shorter)");

    assert!(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
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
