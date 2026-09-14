use super::*;

#[semio_framework_async_macros::async_test]
async fn next_remodeling_id_is_monotonic_and_prefixed() {
    let a = next_remodeling_id("stream");
    let b = next_remodeling_id("stream");
    assert!(a.starts_with("stream-"));
    assert!(b.starts_with("stream-"));
    assert_ne!(a, b);
}


#[semio_framework_async_macros::async_test]
async fn video_codec_from_label_recognizes_common_aliases() {
    assert_eq!(video_codec_from_label("h264"), VideoCodec::Avc);
    assert_eq!(video_codec_from_label("h.265"), VideoCodec::Hevc);
    assert_eq!(video_codec_from_label("mjpg"), VideoCodec::Mjpeg);
    assert_eq!(video_codec_from_label("weird"), VideoCodec::Unknown);
}
