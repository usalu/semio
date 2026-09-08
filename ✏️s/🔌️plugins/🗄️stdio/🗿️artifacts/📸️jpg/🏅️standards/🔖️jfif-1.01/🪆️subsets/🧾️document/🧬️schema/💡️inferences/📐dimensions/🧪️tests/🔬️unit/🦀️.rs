
use super::*;

#[semio_framework_async_macros::async_test]
async fn falls_back_to_canonical_8bit_precision_without_a_decoded_frame() {
    let snapshot = JpgSnapshot { width: 4, height: 2, ..JpgSnapshot::default() };
    assert_eq!(compute_jpg_dimensions(&snapshot), JpgDimensions { width: 4, height: 2, bit_depth: 8, has_alpha: false, pixel_count: 8 });
}

#[semio_framework_async_macros::async_test]
async fn never_reports_alpha() {
    assert!(!compute_jpg_dimensions(&JpgSnapshot::default()).has_alpha);
}
