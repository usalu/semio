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
    let jpg = ::semio_framework_async::poll::resolve_ready(SemioImageToJpg::serialize(&semio)).expect("serialize");
    assert_eq!(jpg.image.width, 2);
    assert_eq!(jpg.image.height, 1);
    assert_eq!(jpg.image.other_segments.len(), 1);
    let bytes = semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::io::encode_jpg(&jpg, &semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::io::JpgEncodeOptions::from_frame(None)).expect("encode real jpg bytes");
    let decoded = semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::io::decode_jpg(&bytes).expect("decode real jpg bytes");
    assert_eq!(decoded.image.width, semio.width);
    assert_eq!(decoded.image.height, semio.height);
    assert_eq!(decoded.image.pixels.len(), semio.frames[0].rgba8.len(), "lossy DCT — length matches, exact bytes need not");
}
