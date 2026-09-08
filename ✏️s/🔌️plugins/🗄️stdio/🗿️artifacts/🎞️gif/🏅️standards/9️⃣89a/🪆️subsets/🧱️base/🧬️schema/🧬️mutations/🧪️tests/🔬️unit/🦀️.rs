
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_frame(seed: u8) -> GifFrame {
    GifFrame {
        left: 0,
        top: 0,
        width: 2,
        height: 2,
        interlace: false,
        lct: Some(GifColorTable { sorted: false, colors: vec![crate::standards::v89a::subsets::any::schema::snapshot::GifRgb { r: seed, g: seed, b: seed }; 2] }),
        indices: vec![0, 1, 1, 0],
        delay_cs: 10,
        disposal: GifDisposal::DoNotDispose,
        transparent_index: None,
        user_input: false,
        plain_text: None,
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> GifSnapshot {
    GifSnapshot {
        schema: "stdio.gif.89a".into(),
        width: 2,
        height: 2,
        gct: None,
        background_color_index: 0,
        pixel_aspect_ratio: 0,
        loop_count: Some(0),
        frames: vec![sample_frame(1), sample_frame(2), sample_frame(3)],
        comments: vec!["c0".into()],
        app_extensions: vec![GifAppExtension { identifier: *b"EXISTING", auth_code: *b"ext", data: vec![0] }],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trips(base: &GifSnapshot, mutation: GifMutation) {
    let diff = mutation.diff(base);
    let mutated = diff.diff().apply(base).expect("diff must apply to base");
    let inverses = mutation.inverse(base);
    let mut restored = mutated.clone();
    for inv in &inverses {
        let inv_diff = inv.diff(&restored);
        restored = inv_diff.diff().apply(&restored).expect("inverse diff must apply to restored");
    }
    assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
}

/// 🧪️ `mutation_diff_law`: every variant's `diff()` matches what `apply_gif_mutation` returns.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    for mutation in [
        GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { loop_count: Some(9), ..base.clone() } }),
        GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![Default::default(); 2] }) }),
        GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
        GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
        GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: None }),
        GifMutation::InsertFrame(insert_frame::InsertFrame { index: 1, frame: sample_frame(9) }),
        GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: 1 }),
        GifMutation::MoveFrame(move_frame::MoveFrame { from: 0, to: 2 }),
        GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
        GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] }),
        GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index: 0, interlace: true }),
        GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_cs: 77 }),
        GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index: 0, disposal: GifDisposal::RestoreToBackground }),
        GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: 0, transparent_index: Some(1) }),
        GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index: 0, user_input: true }),
        GifMutation::InsertComment(insert_comment::InsertComment { index: 0, text: "new".into() }),
        GifMutation::RemoveComment(remove_comment::RemoveComment { index: 0 }),
        GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1] } }),
        GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: 0 }),
    ] {
        let mut snap = base.clone();
        let returned_diff = apply_gif_mutation(&mut snap, &mutation);
        let expected_diff = mutation.diff(&base);
        assert_eq!(returned_diff, expected_diff, "returned diff must equal mutation.diff(base) for {mutation:?}");
        assert_eq!(snap, expected_diff.diff().apply(&base).expect("diff must apply to base"), "apply_gif_mutation must match diff.diff().apply(base) for {mutation:?}");
    }
}

/// 🧪️ `inverse_law` (mutation-level): every variant round-trips.
#[semio_framework_async_macros::async_test]
async fn mutation_apply_inverse_round_trips_every_variant() {
    let base = base_snapshot();
    round_trips(&base, GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { loop_count: Some(5), ..base.clone() } }));
    round_trips(&base, GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 8, height: 6 }));
    round_trips(&base, GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: false, colors: vec![Default::default(); 4] }) }));
    round_trips(&base, GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 2 }));
    round_trips(&base, GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 9 }));
    round_trips(&base, GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: Some(3) }));
    round_trips(&base, GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: None }));
    round_trips(&base, GifMutation::InsertFrame(insert_frame::InsertFrame { index: 1, frame: sample_frame(9) }));
    round_trips(&base, GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: 1 }));
    round_trips(&base, GifMutation::MoveFrame(move_frame::MoveFrame { from: 0, to: 2 }));
    round_trips(&base, GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }));
    round_trips(&base, GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] }));
    round_trips(&base, GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index: 2, interlace: true }));
    round_trips(&base, GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_cs: 42 }));
    round_trips(&base, GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index: 2, disposal: GifDisposal::RestoreToBackground }));
    round_trips(&base, GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: 0, transparent_index: Some(1) }));
    round_trips(&base, GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index: 0, user_input: true }));
    round_trips(&base, GifMutation::InsertComment(insert_comment::InsertComment { index: 0, text: "new".into() }));
    round_trips(&base, GifMutation::RemoveComment(remove_comment::RemoveComment { index: 0 }));
    round_trips(&base, GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1] } }));
    round_trips(&base, GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: 0 }));
}

#[semio_framework_async_macros::async_test]
async fn remove_frame_out_of_range_is_noop_not_panic() {
    let base = base_snapshot();
    let mut snap = base.clone();
    apply_gif_mutation(&mut snap, &GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: 99 }));
    assert_eq!(snap, base);
}

/// 🧪️ F6-PILOT: `OpText`/`OpBinary` round-trip laws over the full ~20-variant vocabulary
/// (handcrafted impls over the `dsl::DslOps`-derived `DslVariants`).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = base_snapshot();
    for mutation in [
        GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { loop_count: Some(9), ..base.clone() } }),
        GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![Default::default(); 2] }) }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: None }),
        GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
        GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
        GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: None }),
        GifMutation::InsertFrame(insert_frame::InsertFrame { index: 1, frame: sample_frame(9) }),
        GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: 1 }),
        GifMutation::MoveFrame(move_frame::MoveFrame { from: 0, to: 2 }),
        GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
        GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] }),
        GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index: 0, interlace: true }),
        GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_cs: 77 }),
        GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index: 0, disposal: GifDisposal::RestoreToBackground }),
        GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: 0, transparent_index: Some(1) }),
        GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: 0, transparent_index: None }),
        GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index: 0, user_input: true }),
        GifMutation::InsertComment(insert_comment::InsertComment { index: 0, text: "new".into() }),
        GifMutation::RemoveComment(remove_comment::RemoveComment { index: 0 }),
        GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1] } }),
        GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: 0 }),
    ] {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = GifMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = GifMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
