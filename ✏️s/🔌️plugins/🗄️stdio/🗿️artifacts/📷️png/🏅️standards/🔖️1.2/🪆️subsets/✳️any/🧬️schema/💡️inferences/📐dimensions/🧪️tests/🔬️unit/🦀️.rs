use super::*;

#[test]
fn derives_geometry_from_checked_ihdr_projection() {
    let snapshot = crate::io::decode_png(include_bytes!("../../../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-adam7.png")).unwrap();
    assert_eq!(compute_png_dimensions(&snapshot), PngDimensions { width: 3, height: 3, bit_depth: 8, has_alpha: true, pixel_count: 9 });
}

#[test]
fn grayscale_16_has_no_alpha() {
    let snapshot = crate::io::decode_png(include_bytes!("../../../../../../../../../🧫️fixtures/🧬️canonical-source/precision-16bit-gray.png")).unwrap();
    assert_eq!(compute_png_dimensions(&snapshot), PngDimensions { width: 2, height: 1, bit_depth: 16, has_alpha: false, pixel_count: 2 });
}
