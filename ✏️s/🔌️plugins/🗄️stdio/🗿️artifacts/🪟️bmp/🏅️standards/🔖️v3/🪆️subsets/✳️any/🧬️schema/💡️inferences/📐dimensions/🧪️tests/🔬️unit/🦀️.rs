use super::*;

#[test]
fn derives_from_checked_canonical_layout() {
    let snapshot = crate::io::decode_bmp(include_bytes!("../../../../../🧫️fixtures/🧬️canonical-byte-authority/direct-rgb24-padding-gap-trailer.bmp")).unwrap();
    assert_eq!(compute_bmp_dimensions(&snapshot), BmpDimensions { width: 3, height: 2, bit_depth: 24, has_alpha: false, pixel_count: 6 });
}

#[test]
fn invalid_snapshot_has_no_inferred_dimensions() {
    let snapshot = BmpSnapshot { schema: crate::STDIO_BMP_DOCUMENT_SCHEMA.into(), bytes: vec![1, 2, 3] };
    assert_eq!(compute_bmp_dimensions(&snapshot), BmpDimensions::default());
}
