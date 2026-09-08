
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, SemioImageMetadataEntry};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioImageSnapshot {
    SemioImageSnapshot {
        width: 2,
        height: 1,
        colorspace: SemioColorspace::Rgba,
        bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "semio fixture".into() }],
        ..SemioImageSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn maps_pixels_and_metadata_to_png() {
    let semio = sample_semio();
    let png = semio_framework_plugin::resolve_ready(SemioImageToPng::serialize(&semio)).expect("serialize");
    assert_eq!(png.width, 2);
    assert_eq!(png.height, 1);
    assert_eq!(png.pixels, semio.frames[0].rgba8);
    assert_eq!(png.text_chunks.len(), 1);
    assert_eq!(png.text_chunks[0].keyword, "Title");
}

/// 🧪️ Real round trip: semio → (this leaf) → `PngSnapshot` → (png's own real codec) → bytes →
/// (png's own real codec) → `PngSnapshot` — proves the serializer produces a genuinely
/// encodable/decodable PNG, not just a plausible-looking struct.
#[semio_framework_async_macros::async_test]
async fn real_byte_round_trip_through_png_codec() {
    let semio = sample_semio();
    let png = semio_framework_plugin::resolve_ready(SemioImageToPng::serialize(&semio)).expect("serialize");
    let bytes = semio_s_artifact_stdio_png::engine::encode_png(&png).expect("encode real png bytes");
    let decoded = semio_s_artifact_stdio_png::engine::decode_png(&bytes).expect("decode real png bytes");
    assert_eq!(decoded.pixels, semio.frames[0].rgba8);
    assert_eq!(decoded.width, semio.width);
    assert_eq!(decoded.height, semio.height);
    assert_eq!(decoded.text_chunks.len(), 1);
    assert_eq!(decoded.text_chunks[0].keyword, "Title");
    assert_eq!(decoded.text_chunks[0].value, "semio fixture");
}
