
use super::*;
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_semio() -> SemioImageSnapshot {
    SemioImageSnapshot {
        colorspace: SemioColorspace::Indexed,
        bit_depth: 8,
        frames: vec![SemioImageFrame { delay_ms: 100, rgba8: vec![255, 0, 0, 255, 0, 255, 0, 255] }],
        icc: None,
        metadata: vec![SemioImageMetadataEntry { key: "comment".into(), value: "semio fixture".into() }, SemioImageMetadataEntry { key: "loopCount".into(), value: "0".into() }],
        width: 2,
        height: 1,
        ..SemioImageSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn quantizes_and_real_byte_round_trips_through_gif_codec() {
    let semio = sample_semio();
    let gif = semio_framework_plugin::resolve_ready(SemioImageToGif::serialize(&semio)).expect("serialize");
    assert_eq!(gif.frames.len(), 1);
    assert_eq!(gif.frames[0].indices.len(), 2);
    assert_eq!(gif.loop_count, Some(0));
    assert_eq!(gif.comments, vec!["semio fixture".to_string()]);

    let bytes = semio_s_artifact_stdio_gif::standards::v89a::engine::encode_gif(&gif).expect("encode real gif bytes");
    let decoded = semio_s_artifact_stdio_gif::standards::v89a::engine::decode_gif(&bytes).expect("decode real gif bytes");
    assert_eq!(decoded.width, semio.width);
    assert_eq!(decoded.height, semio.height);
    assert_eq!(decoded.frames.len(), 1);
    let rgba_back = decoded.frames[0].rgba(decoded.gct.as_ref());
    assert_eq!(rgba_back, semio.frames[0].rgba8, "1:1 quantization must be lossless for <=256 distinct colors");
    assert_eq!(decoded.comments, semio.metadata.iter().filter(|m| m.key == "comment").map(|m| m.value.clone()).collect::<Vec<_>>());
}

#[semio_framework_async_macros::async_test]
async fn errors_past_256_distinct_colors() {
    let mut rgba = Vec::new();
    for i in 0..257u32 {
        rgba.extend_from_slice(&[(i % 256) as u8, ((i / 2) % 256) as u8, ((i / 3) % 256) as u8, 255]);
    }
    let semio = SemioImageSnapshot { width: 257, height: 1, frames: vec![SemioImageFrame { delay_ms: 0, rgba8: rgba }], ..SemioImageSnapshot::default() };
    assert!(semio_framework_plugin::resolve_ready(SemioImageToGif::serialize(&semio)).is_err());
}
