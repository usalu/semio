
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_jpg() -> JpgSnapshot {
    JpgSnapshot { width: 2, height: 1, pixels: vec![255, 0, 0, 255, 0, 255, 0, 255], other_segments: vec![JpgSegment { marker: COM_MARKER, data: b"semio fixture".to_vec() }], ..JpgSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn maps_pixels_and_comment() {
    let semio = semio_framework_plugin::resolve_ready(SemioImageFromJpg::deserialize(&sample_jpg())).expect("deserialize");
    assert_eq!(semio.width, 2);
    assert_eq!(semio.height, 1);
    assert_eq!(semio.colorspace, SemioColorspace::Rgb);
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert_eq!(semio.metadata.len(), 1);
    assert_eq!(semio.metadata[0].key, "comment");
    assert_eq!(semio.metadata[0].value, "semio fixture");
}
