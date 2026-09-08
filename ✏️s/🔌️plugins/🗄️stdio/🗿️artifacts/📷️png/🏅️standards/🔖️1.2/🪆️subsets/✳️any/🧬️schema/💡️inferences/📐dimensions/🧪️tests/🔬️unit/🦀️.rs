
use super::*;

#[semio_framework_async_macros::async_test]
async fn derives_from_ihdr_fields() {
    let snapshot = PngSnapshot { width: 4, height: 3, bit_depth: 8, color_type: PngColorType::Rgba, ..PngSnapshot::default() };
    assert_eq!(compute_png_dimensions(&snapshot), PngDimensions { width: 4, height: 3, bit_depth: 8, has_alpha: true, pixel_count: 12 });
}

#[semio_framework_async_macros::async_test]
async fn rgb_has_no_alpha() {
    let snapshot = PngSnapshot { width: 2, height: 2, bit_depth: 8, color_type: PngColorType::Rgb, ..PngSnapshot::default() };
    assert!(!compute_png_dimensions(&snapshot).has_alpha);
}
