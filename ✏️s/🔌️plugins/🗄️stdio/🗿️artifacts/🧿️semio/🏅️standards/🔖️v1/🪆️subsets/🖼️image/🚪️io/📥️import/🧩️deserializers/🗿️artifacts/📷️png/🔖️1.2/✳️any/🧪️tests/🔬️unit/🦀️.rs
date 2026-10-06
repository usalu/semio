use super::*;
use semio_s_artifact_stdio_png::{io::PngProjection, schema::snapshot::{PngChunkMarker, PngTextChunk, PngTextKind}};

fn sample_png() -> PngSnapshot {
    let projection = PngProjection {
        width: 2, height: 1, bit_depth: 8, color_type: PngColorType::Rgba, interlace: false,
        plte: None, trns: None, gama: None, chrm: None, srgb: None, phys: None, time: None, bkgd: None,
        text_chunks: vec![PngTextChunk { keyword: "Title".into(), value: "semio fixture".into(), kind: PngTextKind::Text, ..Default::default() }],
        pixels: vec![255, 0, 0, 255, 0, 255, 0, 255],
        chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Text { index: 0 }, PngChunkMarker::Idat, PngChunkMarker::Iend],
        unknown_chunks: Vec::new(),
    };
    let bytes = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::author_png_projection(&projection).unwrap();
    semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::decode_png(&bytes).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn maps_checked_projection_pixels_metadata_and_source_profile() {
    let semio = ::semio_framework_async::poll::resolve_ready(SemioImageFromPng::deserialize(&sample_png())).unwrap();
    assert_eq!((semio.width, semio.height, semio.colorspace, semio.bit_depth), (2, 1, SemioColorspace::Rgba, 8));
    assert_eq!(semio.frames[0].rgba8, vec![255, 0, 0, 255, 0, 255, 0, 255]);
    assert_eq!((semio.metadata[0].key.as_str(), semio.metadata[0].value.as_str()), ("Title", "semio fixture"));
}

#[semio_framework_async_macros::async_test]
async fn rejects_invalid_source_authority() {
    let mut bad = sample_png();
    bad.bytes[20] ^= 1;
    assert!(::semio_framework_async::poll::resolve_ready(SemioImageFromPng::deserialize(&bad)).is_err());
}
