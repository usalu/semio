
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioImageSnapshot {
    SemioImageSnapshot {
        width: 2,
        height: 1,
        colorspace: SemioColorspace::Rgb,
        bit_depth: 24,
        frames: vec![SemioImageFrame { delay_ms: 0, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "xPixelsPerMeter".into(), value: "2835".into() }],
        ..SemioImageSnapshot::default()
    }
}

/// 🧪️ Real round trip through bmp's own codec — alpha is expected to drop (documented, the
/// codec's own real behavior), RGB channels must survive exactly.
#[semio_framework_async_macros::async_test]
async fn real_byte_round_trip_through_bmp_codec() {
    let semio = sample_semio();
    let bmp = semio_framework_plugin::resolve_ready(SemioImageToBmp::serialize(&semio)).expect("serialize");
    assert_eq!(bmp.x_pixels_per_meter, 2835);
    let bytes = semio_s_artifact_stdio_bmp::engine::encode_bmp(&bmp).expect("encode real bmp bytes");
    let decoded = semio_s_artifact_stdio_bmp::engine::decode_bmp(&bytes).expect("decode real bmp bytes");
    assert_eq!(decoded.width, semio.width);
    assert_eq!(decoded.height, semio.height);
    for (a, b) in decoded.pixels.chunks_exact(4).zip(semio.frames[0].rgba8.chunks_exact(4)) {
        assert_eq!(&a[0..3], &b[0..3], "RGB must survive exactly");
    }
}
