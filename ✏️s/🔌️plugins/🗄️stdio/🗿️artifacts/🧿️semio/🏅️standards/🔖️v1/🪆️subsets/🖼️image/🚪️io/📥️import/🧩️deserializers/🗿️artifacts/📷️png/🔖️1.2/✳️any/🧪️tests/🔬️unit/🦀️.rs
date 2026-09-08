
use super::*;
use semio_s_artifact_stdio_png::schema::snapshot::PngRgb;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_png() -> PngSnapshot {
    PngSnapshot {
        width: 2,
        height: 1,
        bit_depth: 8,
        color_type: PngColorType::Rgba,
        pixels: vec![255, 0, 0, 255, 0, 255, 0, 255],
        text_chunks: vec![PngTextChunk { keyword: "Title".into(), value: "semio fixture".into(), kind: PngTextKind::Text, ..Default::default() }],
        chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Text { index: 0 }, PngChunkMarker::Idat, PngChunkMarker::Iend],
        plte: Some(vec![PngRgb::default()]),
        ..PngSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_pixels_and_metadata() {
    let semio = semio_framework_plugin::resolve_ready(SemioImageFromPng::deserialize(&sample_png())).expect("deserialize");
    assert_eq!(semio.width, 2);
    assert_eq!(semio.height, 1);
    assert_eq!(semio.colorspace, SemioColorspace::Rgba);
    assert_eq!(semio.frames.len(), 1);
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert_eq!(semio.icc, None, "png codec does not model iCCP — documented loss");
    assert_eq!(semio.metadata.len(), 1);
    assert_eq!(semio.metadata[0].key, "Title");
    assert_eq!(semio.metadata[0].value, "semio fixture");
}

#[semio_framework_async_macros::async_test]
async fn rejects_pixel_length_mismatch() {
    let mut bad = sample_png();
    bad.pixels.pop();
    assert!(semio_framework_plugin::resolve_ready(SemioImageFromPng::deserialize(&bad)).is_err());
}
