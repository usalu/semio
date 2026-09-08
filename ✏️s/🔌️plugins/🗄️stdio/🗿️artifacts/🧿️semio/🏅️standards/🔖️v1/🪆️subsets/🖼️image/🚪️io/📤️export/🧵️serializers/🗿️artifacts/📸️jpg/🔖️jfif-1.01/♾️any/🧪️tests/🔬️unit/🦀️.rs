
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioImageSnapshot {
    SemioImageSnapshot {
        width: 2,
        height: 1,
        colorspace: SemioColorspace::Rgb,
        bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "comment".into(), value: "semio fixture".into() }],
        ..SemioImageSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn real_byte_round_trip_through_jpg_codec() {
    let semio = sample_semio();
    let jpg = semio_framework_plugin::resolve_ready(SemioImageToJpg::serialize(&semio)).expect("serialize");
    assert_eq!(jpg.width, 2);
    assert_eq!(jpg.height, 1);
    assert_eq!(jpg.other_segments.len(), 1);
    let bytes = semio_s_artifact_stdio_jpg::engine::encode_jpg(&jpg).expect("encode real jpg bytes");
    let decoded = semio_s_artifact_stdio_jpg::engine::decode_jpg(&bytes).expect("decode real jpg bytes");
    assert_eq!(decoded.width, semio.width);
    assert_eq!(decoded.height, semio.height);
    assert_eq!(decoded.pixels.len(), semio.frames[0].rgba8.len(), "lossy DCT — length matches, exact bytes need not");
}
