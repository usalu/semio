use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_bmp() -> BmpSnapshot {
    BmpSnapshot { width: 2, height: 1, bits_per_pixel: 24, row_order: BmpRowOrder::BottomUp, x_pixels_per_meter: 2835, y_pixels_per_meter: 2835, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255], ..BmpSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn maps_pixels_and_resolution() {
    let semio = semio_framework_plugin::resolve_ready(SemioImageFromBmp::deserialize(&sample_bmp())).expect("deserialize");
    assert_eq!(semio.width, 2);
    assert_eq!(semio.height, 1);
    assert_eq!(semio.colorspace, SemioColorspace::Rgb);
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert!(semio.metadata.iter().any(|m| m.key == "xPixelsPerMeter" && m.value == "2835"));
}
