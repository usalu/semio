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

#[semio_framework_async_macros::async_test]
async fn absorb_law_holds_over_curated_ops() {
    let base = GifSnapshot { frames: vec![frame(1, 2, 2), frame(2, 2, 2), frame(3, 2, 2)], loop_count: Some(0), ..GifSnapshot::default() };
    let mid = {
        let mut s = base.clone();
        s.frames.insert(1, frame(9, 2, 2));
        s.frames.remove(0);
        s.comments.push("hello".into());
        s
    };
    let after = {
        let mut s = mid.clone();
        s.frames[0].disposal = GifDisposal::RestoreToBackground;
        s.frames.push(frame(5, 2, 2));
        s.app_extensions.push(GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1, 2, 3] });
        s
    };
    let mut d1 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &mid);
    let d2 = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&mid, &after);
    d1.absorb(d2);
    assert_eq!(d1.apply(&base).unwrap(), after);
}

#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = GifSnapshot { width: 4, height: 4, frames: vec![frame(1, 4, 4)], ..GifSnapshot::default() };
    let b = GifSnapshot { width: 4, height: 4, frames: vec![frame(1, 4, 4), frame(2, 2, 2)], loop_count: Some(0), ..GifSnapshot::default() };
    let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &b);
    assert_eq!(ab.apply(&a).unwrap(), b);
    let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&b, &a);
    assert_eq!(ba.apply(&b).unwrap(), a);
    assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&a, &a).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn inverse_law() {
    let base = GifSnapshot { frames: vec![frame(1, 2, 2), frame(2, 2, 2)], loop_count: Some(0), ..GifSnapshot::default() };
    let next = {
        let mut s = base.clone();
        s.frames[0].disposal = GifDisposal::RestoreToPrevious;
        s.frames.remove(1);
        s.frames.push(frame(7, 3, 3));
        s.background_color_index = 5;
        s.loop_count = None;
        s.comments.push("hi".into());
        s
    };
    let d = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&base, &next);
    let mutated = d.apply(&base).unwrap();
    let inv = d.inverse(&base);
    assert_eq!(inv.apply(&mutated).unwrap(), base);
}

/// 🧪️ Field sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in EVERY mutable
/// field, incl. every tri-state exercising `Some(None)`, with asymmetric collection lengths
/// (F1's structural trap: a single index-keyed `between()` call can show `removed` XOR
/// `added`, never both — split across both directions, per `f1-closer-report.md` §4.4).
#[semio_framework_async_macros::async_test]
async fn field_sweep_covers_every_mutable_field() {
    let mut fa = frame(1, 2, 2);
    fa.transparent_index = Some(0);
    let sweep_a = GifSnapshot {
        schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width: 10,
        height: 8,
        gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }),
        background_color_index: 0,
        pixel_aspect_ratio: 0,
        loop_count: None,
        frames: vec![fa, frame(2, 2, 2)],
        comments: vec!["first".into()],
        app_extensions: vec![GifAppExtension { identifier: *b"NETSCAPX", auth_code: *b"2.0", data: vec![9] }],
    };
    let mut fb0 = frame(1, 2, 2);
    fb0.disposal = GifDisposal::RestoreToPrevious;
    fb0.interlace = true;
    fb0.transparent_index = None;
    fb0.plain_text = Some(GifPlainText { left: 1, top: 1, width: 2, height: 2, cell_width: 4, cell_height: 4, fg_color_index: 0, bg_color_index: 1, text: "hi".into() });
    let sweep_b = GifSnapshot {
        schema: STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width: 20,
        height: 16,
        gct: None,
        background_color_index: 3,
        pixel_aspect_ratio: 7,
        loop_count: Some(5),
        frames: vec![fb0, frame(6, 3, 3), frame(7, 3, 3)],
        comments: vec![],
        app_extensions: vec![],
    };

    let ab = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_b);
    assert_eq!(ab.apply(&sweep_a).unwrap(), sweep_b);
    assert!(ab.width.is_some());
    assert!(ab.height.is_some());
    assert_eq!(ab.gct, Some(None), "gct going Some->None must be tri-state Some(None)");
    assert!(ab.background_color_index.is_some());
    assert!(ab.pixel_aspect_ratio.is_some());
    assert_eq!(ab.loop_count, Some(Some(5)));
    let frames_ab = ab.frames.as_ref().expect("frames must differ");
    assert!(!frames_ab.modified.is_empty(), "sweep must exercise a modified frame");
    assert!(!frames_ab.added.is_empty(), "sweep must exercise an added frame (b is longer)");
    let modified_diff = &frames_ab.modified[0].diff;
    assert!(modified_diff.disposal.is_some());
    assert!(modified_diff.interlace.is_some());
    assert_eq!(modified_diff.transparent_index, Some(None), "transparent_index Some->None must be tri-state Some(None)");
    assert!(modified_diff.plain_text.is_some(), "plain_text None->Some must be captured");
    let comments_ab = ab.comments.as_ref().expect("comments must differ");
    assert!(!comments_ab.removed.is_empty(), "sweep must exercise a removed comment (b has none)");
    let app_ext_ab = ab.app_extensions.as_ref().expect("app_extensions must differ");
    assert!(!app_ext_ab.removed.is_empty(), "sweep must exercise a removed app extension (b has none)");

    let ba = <GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_b, &sweep_a);
    assert_eq!(ba.apply(&sweep_b).unwrap(), sweep_a);
    let frames_ba = ba.frames.as_ref().expect("frames must differ");
    assert!(!frames_ba.removed.is_empty(), "reverse direction must exercise a removed frame (a is shorter)");
    let comments_ba = ba.comments.as_ref().expect("comments must differ");
    assert!(!comments_ba.added.is_empty(), "reverse direction must exercise an added comment");
    let app_ext_ba = ba.app_extensions.as_ref().expect("app_extensions must differ");
    assert!(!app_ext_ba.added.is_empty(), "reverse direction must exercise an added app extension");

    assert!(<GifDiff as DiffAlgebra<GifSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
}

/// 🧪️ F6-PILOT: `DiffCodec` round-trip laws for the hand-rolled `GifDiff` text/binary grammar
/// — exercises scalars, both tri-states (`gct`/`loop_count` at the top level, `lct`/
/// `transparent_index`/`plain_text` inside a modified frame), and all three collection triples
/// (`removed`/`modified`/`added`) simultaneously via a real `between()` result.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let mut fa = frame(1, 2, 2);
    fa.transparent_index = Some(0);
    let a = GifSnapshot {
        width: 10,
        height: 8,
        gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 1, g: 2, b: 3 }; 2] }),
        loop_count: None,
        frames: vec![fa, frame(2, 2, 2)],
        comments: vec!["first".into()],
        app_extensions: vec![GifAppExtension { identifier: *b"NETSCAPX", auth_code: *b"2.0", data: vec![9] }],
        ..GifSnapshot::default()
    };
    let mut fb0 = frame(1, 2, 2);
    fb0.disposal = GifDisposal::RestoreToPrevious;
    fb0.transparent_index = None;
    fb0.plain_text = Some(GifPlainText { left: 1, top: 1, width: 2, height: 2, cell_width: 4, cell_height: 4, fg_color_index: 0, bg_color_index: 1, text: "hi".into() });
    let b = GifSnapshot { width: 20, height: 16, gct: None, loop_count: Some(5), frames: vec![fb0, frame(6, 3, 3), frame(7, 3, 3)], comments: vec![], app_extensions: vec![], ..GifSnapshot::default() };
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
