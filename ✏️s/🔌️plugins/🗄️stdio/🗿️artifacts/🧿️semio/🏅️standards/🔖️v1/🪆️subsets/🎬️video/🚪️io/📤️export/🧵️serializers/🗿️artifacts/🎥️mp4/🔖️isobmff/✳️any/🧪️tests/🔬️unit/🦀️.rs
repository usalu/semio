
use super::*;
use crate::standards::v1::subsets::video::io::mp4_deserializer::SemioVideoFromMp4;
use crate::standards::v1::subsets::video::schema::snapshot::{STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA, SemioRational, SemioVideoSample, SemioVideoStream, SemioVideoStreamKind};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_video() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![SemioVideoStream {
            kind: SemioVideoStreamKind::Video,
            codec: "avc1".into(),
            width: 640,
            height: 480,
            rate: SemioRational { num: 30, den: 1 },
            samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }, SemioVideoSample { pts: 1, key: false, data: vec![4, 5] }, SemioVideoSample { pts: 2, key: false, data: vec![6] }],
        }],
    }
}

/// 🧪️ codec_retention_law-style round trip FROM the semio side: video -> mp4 -> video must be
/// a clean fixpoint (everything `video` can represent survives), even though mp4 -> video ->
/// mp4 is documented-lossy (sps/pps/cts_offset) and therefore not the direction under test.
#[semio_framework_async_macros::async_test]
async fn video_to_mp4_to_video_round_trips_everything_the_video_subset_can_represent() {
    let original = real_world_video();
    let mp4 = semio_framework_plugin::resolve_ready(SemioVideoToMp4::serialize(&original)).expect("serialize");
    assert_eq!(mp4.tracks.len(), 1);
    assert_eq!(mp4.tracks[0].timescale, 30);
    assert_eq!(mp4.tracks[0].width, 640);
    assert_eq!(mp4.tracks[0].height, 480);
    let back = semio_framework_plugin::resolve_ready(SemioVideoFromMp4::deserialize(&mp4)).expect("deserialize");
    assert_eq!(back, original);
}

#[semio_framework_async_macros::async_test]
async fn codec_name_longer_than_four_bytes_is_truncated_not_panicking() {
    let mut snap = real_world_video();
    snap.streams[0].codec = "hevc-main10".into();
    let mp4 = semio_framework_plugin::resolve_ready(SemioVideoToMp4::serialize(&snap)).expect("serialize");
    assert_eq!(mp4.tracks[0].codec.nal_length_size, 4);
}

#[semio_framework_async_macros::async_test]
async fn empty_stream_list_serializes_to_zero_tracks() {
    let snap = SemioVideoSnapshot { schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(), streams: Vec::new() };
    let mp4 = semio_framework_plugin::resolve_ready(SemioVideoToMp4::serialize(&snap)).expect("serialize");
    assert!(mp4.tracks.is_empty());
}
