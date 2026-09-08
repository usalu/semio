
use super::*;

#[semio_framework_async_macros::async_test]
async fn derives_from_header_fields() {
    let snapshot = BmpSnapshot { width: 5, height: 2, bits_per_pixel: 24, ..BmpSnapshot::default() };
    assert_eq!(compute_bmp_dimensions(&snapshot), BmpDimensions { width: 5, height: 2, bit_depth: 24, has_alpha: false, pixel_count: 10 });
}

#[semio_framework_async_macros::async_test]
async fn thirty_two_bpp_is_treated_as_alpha_capable() {
    let snapshot = BmpSnapshot { width: 1, height: 1, bits_per_pixel: 32, ..BmpSnapshot::default() };
    assert!(compute_bmp_dimensions(&snapshot).has_alpha);
}
