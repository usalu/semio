use super::*;

const FIXTURE_MANIFEST: &str = include_str!("../../../🧫️fixtures/🧬️canonical-byte-authority/🔣️.json");

fn accepted() -> [(&'static str, BmpProfile, &'static [u8]); 9] {
    [
        ("direct-rgb24-padding-gap-trailer", BmpProfile::DirectRgb24, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")),
        ("indexed-rgb1-duplicate-palette", BmpProfile::IndexedRgb1, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb1-duplicate-palette.bmp")),
        ("indexed-rgb4-duplicate-palette", BmpProfile::IndexedRgb4, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb4-duplicate-palette.bmp")),
        ("indexed-rgb8-duplicate-palette", BmpProfile::IndexedRgb8, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb8-duplicate-palette.bmp")),
        ("direct-rgb16-555", BmpProfile::DirectRgb16, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb16-555.bmp")),
        ("direct-rgb32-reserved-sample", BmpProfile::DirectRgb32, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb32-reserved-sample.bmp")),
        ("direct-bitfields16-565", BmpProfile::DirectBitfields16, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields16-565.bmp")),
        ("direct-bitfields32", BmpProfile::DirectBitfields32, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields32.bmp")),
        ("direct-rgb24-top-down", BmpProfile::DirectRgb24, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-top-down.bmp")),
    ]
}

#[test]
fn every_declared_v3_profile_round_trips_exact_source_bytes() {
    let manifest: serde_json::Value = serde_json::from_str(FIXTURE_MANIFEST).expect("neutral fixture manifest");
    assert_eq!(manifest["accepted"].as_array().expect("accepted cases").len(), accepted().len());
    for (id, profile, source) in accepted() {
        assert!(manifest["accepted"].as_array().unwrap().iter().any(|case| case["id"] == id), "{id}: missing from neutral manifest");
        let snapshot = decode_bmp(source).unwrap_or_else(|failure| panic!("{id}: {failure}"));
        let layout = bmp_layout(&snapshot).unwrap();
        assert_eq!(layout.profile, profile, "{id}: profile");
        assert_eq!(encode_bmp(&snapshot).unwrap(), source, "{id}: no-op save must preserve every byte");
        assert_eq!(bmp_rgba8_preview(&snapshot).unwrap().len(), layout.width as usize * layout.height as usize * 4, "{id}: display projection");
    }
}

#[test]
fn rgba8_conversion_authors_a_checked_direct_rgb24_profile() {
    let source = [255, 0, 0, 255, 0, 255, 0, 255];
    let snapshot = bmp_direct_rgb24_from_rgba8(2, 1, &source, 2835, -2835).unwrap();
    let layout = bmp_layout(&snapshot).unwrap();
    assert_eq!(layout.profile, BmpProfile::DirectRgb24);
    assert_eq!((layout.width, layout.height, layout.row_stride), (2, 1, 8));
    assert_eq!((layout.x_pixels_per_meter, layout.y_pixels_per_meter), (2835, -2835));
    assert_eq!(&snapshot.bytes[60..62], &[0, 0]);
    assert_eq!(bmp_rgba8_preview(&snapshot).unwrap(), [255, 0, 0, 255, 0, 255, 0, 255]);
    assert!(bmp_direct_rgb24_from_rgba8(2, 1, &source[..4], 0, 0).unwrap_err().contains("expected 8"));
    assert!(bmp_direct_rgb24_from_rgba8(1, 1, &[1, 2, 3, 0], 0, 0).unwrap_err().contains("cannot represent nonopaque"));
    assert!(bmp_direct_rgb24_from_rgba8(1, 1, &[1, 2, 3, 128], 0, 0).unwrap_err().contains("cannot represent nonopaque"));
}

#[test]
fn rgba8_alpha_policy_is_neutral_fixture_driven() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️canonical-byte-authority/rgba8-direct-rgb24.json")).unwrap();
    let rgb = fixture["rgb"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect::<Vec<_>>();
    for case in fixture["cases"].as_array().unwrap() {
        let rgba = [rgb[0], rgb[1], rgb[2], case["alpha"].as_u64().unwrap() as u8];
        let result = bmp_direct_rgb24_from_rgba8(1, 1, &rgba, 2835, 2835);
        if case["outcome"] == "accepted" {
            assert_eq!(result.unwrap().bytes, include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/rgba8-opaque-direct-rgb24.bmp"), "{}", case["id"]);
        } else {
            assert!(result.unwrap_err().contains(case["error"].as_str().unwrap()), "{}", case["id"]);
        }
    }
}

#[test]
fn foreign_dib_profiles_are_rejected_before_document_construction() {
    for (id, source) in [
        ("12", include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-12.bmp").as_slice()),
        ("52", include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-52.bmp").as_slice()),
        ("56", include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-56.bmp").as_slice()),
        ("108", include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-108.bmp").as_slice()),
        ("124", include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/reject-dib-124.bmp").as_slice()),
    ] {
        let failure = decode_bmp(source).unwrap_err();
        assert!(failure.contains("40-byte BITMAPINFOHEADER"), "DIB {id}: {failure}");
    }
}

#[test]
fn indexed_paint_preserves_duplicate_palette_identity_and_unaddressed_bytes() {
    let source = include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/indexed-rgb1-duplicate-palette.bmp");
    let before = decode_bmp(source).unwrap();
    let before_preview = bmp_rgba8_preview(&before).unwrap();
    let revision = bmp_revision(&before);
    let mut progress = Vec::new();
    let after = paint_indexed_region_controlled(&before, &revision, BmpRegion { x: 0, y: 0, width: 1, height: 1 }, 1, &mut |done, total| {
        progress.push((done, total));
        true
    })
    .unwrap();
    let changed: Vec<_> = before.bytes.iter().zip(&after.bytes).enumerate().filter_map(|(index, (before, after))| (before != after).then_some(index)).collect();
    assert_eq!(changed, vec![62]);
    assert_eq!(bmp_rgba8_preview(&after).unwrap(), before_preview, "two equal-colour palette indices remain independently addressable");
    assert_eq!(progress, vec![(0, 1), (1, 1)]);
    assert_eq!(before.bytes.as_slice(), source);
}

#[test]
fn direct_paint_changes_only_addressed_samples() {
    let source = include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp");
    let before = decode_bmp(source).unwrap();
    let after = paint_direct_region_controlled(&before, &bmp_revision(&before), BmpRegion { x: 1, y: 0, width: 1, height: 1 }, BmpColor { red: 9, green: 8, blue: 7, alpha: 6 }, &mut |_, _| true).unwrap();
    let layout = bmp_layout(&before).unwrap();
    let sample = layout.data_offset + layout.row_stride + 3;
    let changed: Vec<_> = before.bytes.iter().zip(&after.bytes).enumerate().filter_map(|(index, (before, after))| (before != after).then_some(index)).collect();
    assert_eq!(changed, vec![sample, sample + 1, sample + 2]);
    assert_eq!(&after.bytes[sample..sample + 3], &[7, 8, 9]);
    assert_eq!(&after.bytes[54..57], &[0xde, 0xad, 0xbe], "pre-pixel gap");
    assert_eq!(&after.bytes[after.bytes.len() - 4..], &[0xfe, 0xed, 0xfa, 0xce], "trailer");
}

#[test]
fn cancelled_and_stale_paints_publish_nothing() {
    let source = include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp");
    let before = decode_bmp(source).unwrap();
    let cancelled = paint_direct_region_controlled(&before, &bmp_revision(&before), BmpRegion { x: 0, y: 0, width: 3, height: 2 }, BmpColor { red: 1, green: 2, blue: 3, alpha: 4 }, &mut |done, _| done == 0).unwrap_err();
    assert!(cancelled.contains("cancelled"));
    assert!(paint_direct_region_controlled(&before, "stale", BmpRegion { x: 0, y: 0, width: 1, height: 1 }, BmpColor::default(), &mut |_, _| true).unwrap_err().contains("stale"));
    assert_eq!(before.bytes.as_slice(), source);
}

#[test]
fn bitfield_paint_preserves_unmasked_precision_bits() {
    let source = include_bytes!("../../../🧫️fixtures/🧬️canonical-byte-authority/direct-bitfields32.bmp");
    let before = decode_bmp(source).unwrap();
    let layout = bmp_layout(&before).unwrap();
    let after = paint_direct_region_controlled(&before, &bmp_revision(&before), BmpRegion { x: 0, y: 0, width: 1, height: 1 }, BmpColor { red: 90, green: 80, blue: 70, alpha: 0 }, &mut |_, _| true).unwrap();
    assert_eq!(after.bytes[layout.data_offset + 3], 0x7f, "unmasked high sample byte");
    assert_eq!(&after.bytes[layout.data_offset + 4..layout.data_offset + 8], &before.bytes[layout.data_offset + 4..layout.data_offset + 8], "unaddressed sample");
}

#[test]
fn canonical_history_outputs_are_exact_mutation_results() {
    let direct_before = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(include_str!("../../../🧫️fixtures/🧬️history-edits/🖌️paint-direct-region/🎯️direct/📸️snapshot/⬅️before/🗣️.dsl.semio")).unwrap();
    let direct_after = paint_direct_region_controlled(&direct_before, &bmp_revision(&direct_before), BmpRegion { x: 0, y: 0, width: 1, height: 1 }, BmpColor { red: 17, green: 34, blue: 51, alpha: 255 }, &mut |_, _| true).unwrap();
    let direct_expected = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(include_str!("../../../🧫️fixtures/🧬️history-edits/🖌️paint-direct-region/🎯️direct/📸️snapshot/➡️after/🗣️.dsl.semio")).unwrap();
    assert_eq!(direct_after, direct_expected);

    let indexed_before = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(include_str!("../../../🧫️fixtures/🧬️history-edits/🎨️paint-indexed-region/🎯️direct/📸️snapshot/⬅️before/🗣️.dsl.semio")).unwrap();
    let indexed_after = paint_indexed_region_controlled(&indexed_before, &bmp_revision(&indexed_before), BmpRegion { x: 0, y: 0, width: 1, height: 1 }, 1, &mut |_, _| true).unwrap();
    let indexed_expected = <BmpSnapshot as store::ArtifactDsl>::parse_dsl(include_str!("../../../🧫️fixtures/🧬️history-edits/🎨️paint-indexed-region/🎯️direct/📸️snapshot/➡️after/🗣️.dsl.semio")).unwrap();
    assert_eq!(indexed_after, indexed_expected);
}
