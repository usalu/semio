use super::*;

const SCAN: &[u8] = include_bytes!("../../../🧫️fixtures/🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff");

/// 🖼️ The real scan read by the `tiff` crate sits inside the Baseline class, strip-organized.
#[test]
fn the_real_scan_reads_inside_the_baseline_class() {
    let axes = read_axes(SCAN).expect("the tiff reader decodes the committed scan");
    assert!(axes.raster, "the scan's raster decodes");
    assert!(axes.strip_offsets.is_some() && axes.tile_width.is_none(), "the scan is strip-organized");
    assert_eq!(verdict(&axes), Vec::<&str>::new(), "the committed scan is a Baseline TIFF");
}

/// 🛡️ Every kind moves its own axis and raises exactly the code the specification table names.
#[test]
fn every_kind_moves_its_axis_and_the_verdict_the_table_names() {
    let axes = read_axes(SCAN).expect("reads");
    let json = |text: &str| semio_repo_test_host::parse_json(text).expect("params");
    for (kind, params, code) in [
        ("set-compression", r#"{"compression": 5}"#, Some("stdio.tiff.baseline.unsupported-compression")),
        ("set-photometric-interpretation", r#"{"photometric": 6}"#, Some("stdio.tiff.baseline.unsupported-photometric")),
        ("set-bits-per-sample", r#"{"bits": [16, 16, 16]}"#, Some("stdio.tiff.baseline.unsupported-bits-per-sample")),
        ("insert-tile-tags", r#"{"tileWidth": 256, "tileLength": 256}"#, Some("stdio.tiff.baseline.tiled-not-baseline")),
        ("remove-strip-offsets", "{}", Some("stdio.tiff.baseline.missing-strip-offsets")),
        ("set-strip-offsets", r#"{"offsets": [8, 65536]}"#, None),
    ] {
        let next = apply(&axes, kind, &json(params)).expect("applies");
        assert_ne!(project(&next), project(&axes), "{kind} must move the projection");
        assert_eq!(verdict(&next).first().copied(), code, "{kind}");
    }
    assert!(apply(&axes, "set-byte-order", &json("{}")).is_err(), "a kind outside the vocabulary is refused");
}
