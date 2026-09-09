use super::*;
use crate::standards::v87a::subsets::any::schema::snapshot::GifRgb;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_image(seed: u8) -> GifImage {
    GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }), indices: vec![0, 1, 1, 0] }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn base_snapshot() -> GifSnapshot {
    GifSnapshot { schema: "stdio.gif".into(), width: 2, height: 2, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, images: vec![sample_image(1), sample_image(2), sample_image(3)] }
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

/// 🧪️ `mutation_diff_law`: every variant's `diff()` matches what `apply_gif_mutation` returns
/// and applying it reproduces the same mutated state.
#[semio_framework_async_macros::async_test]
async fn mutation_diff_law() {
    let base = base_snapshot();
    for mutation in [
        GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { background_color_index: 9, ..base.clone() } }),
        GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb::default(); 2] }) }),
        GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
        GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
        GifMutation::InsertImage(insert_image::InsertImage { index: 1, image: sample_image(9) }),
        GifMutation::RemoveImage(remove_image::RemoveImage { index: 1 }),
        GifMutation::MoveImage(move_image::MoveImage { from: 0, to: 2 }),
        GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
        GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] }),
        GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 0, interlace: true }),
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
    round_trips(&base, GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { background_color_index: 5, ..base.clone() } }));
    round_trips(&base, GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 8, height: 6 }));
    round_trips(&base, GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb::default(); 4] }) }));
    round_trips(&base, GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 2 }));
    round_trips(&base, GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 9 }));
    round_trips(&base, GifMutation::InsertImage(insert_image::InsertImage { index: 1, image: sample_image(9) }));
    round_trips(&base, GifMutation::RemoveImage(remove_image::RemoveImage { index: 1 }));
    round_trips(&base, GifMutation::MoveImage(move_image::MoveImage { from: 0, to: 2 }));
    round_trips(&base, GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }));
    round_trips(&base, GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] }));
    round_trips(&base, GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 2, interlace: true }));
}

/// 🧪️ Wave 7: `KINDS` must name exactly the enum's variants (kebab-cased, declaration order),
/// and exactly the manifest's `mutationCatalogs[].kinds` — the framework never parses Rust, so
/// this is what keeps the oracle catalog declaration honest against a drifted enum.
#[semio_framework_async_macros::async_test]
async fn kinds_match_enum_variants_and_manifest_catalog() {
    assert_eq!(
        KINDS,
        ["set-snapshot", "set-screen-size", "set-global-color-table", "set-background-color-index", "set-pixel-aspect-ratio", "insert-image", "remove-image", "move-image", "set-image-geometry", "set-image-pixels", "set-image-interlace"]
    );
    assert_eq!(KINDS.len(), 11, "one kebab-case entry per GifMutation variant");
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "manifest mutationCatalogs[].kinds must list {kind:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn remove_image_out_of_range_is_noop_not_panic() {
    let base = base_snapshot();
    let mut snap = base.clone();
    apply_gif_mutation(&mut snap, &GifMutation::RemoveImage(remove_image::RemoveImage { index: 99 }));
    assert_eq!(snap, base);
}

/// 🧪️ F6: `OpText`/`OpBinary` round-trip laws over the full 11-variant vocabulary (handcrafted
/// impls over the `dsl::DslOps`-derived `DslVariants`, mirroring gif89a's `GifMutation`).
#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    let base = base_snapshot();
    for mutation in [
        GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { background_color_index: 9, ..base.clone() } }),
        GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb::default(); 2] }) }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: None }),
        GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
        GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
        GifMutation::InsertImage(insert_image::InsertImage { index: 1, image: sample_image(9) }),
        GifMutation::RemoveImage(remove_image::RemoveImage { index: 1 }),
        GifMutation::MoveImage(move_image::MoveImage { from: 0, to: 2 }),
        GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
        GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] }),
        GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 0, interlace: true }),
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
