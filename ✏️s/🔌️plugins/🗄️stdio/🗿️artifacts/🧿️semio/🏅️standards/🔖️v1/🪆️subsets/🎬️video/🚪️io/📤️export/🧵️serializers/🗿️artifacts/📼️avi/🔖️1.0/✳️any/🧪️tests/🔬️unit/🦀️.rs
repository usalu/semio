
use super::*;
use crate::standards::v1::subsets::video::io::avi_deserializer::SemioVideoFromAvi;
use crate::standards::v1::subsets::video::schema::snapshot::{STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA, SemioRational, SemioVideoSample, SemioVideoStream};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_video() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![SemioVideoStream {
            kind: SemioVideoStreamKind::Video,
            codec: "MJPG".into(),
            width: 16,
            height: 16,
            rate: SemioRational { num: 10, den: 1 },
            samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3, 4] }, SemioVideoSample { pts: 1, key: false, data: vec![5, 6, 7, 8] }],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn video_to_avi_to_video_round_trips_everything_the_video_subset_can_represent() {
    let original = real_world_video();
    let avi = semio_framework_plugin::resolve_ready(SemioVideoToAvi::serialize(&original)).expect("serialize");
    assert_eq!(avi.streams.len(), 1);
    assert_eq!(avi.streams[0].strh.fcc_type, "vids");
    assert_eq!(avi.streams[0].strh.scale, 1);
    assert_eq!(avi.streams[0].strh.rate, 10);
    let back = semio_framework_plugin::resolve_ready(SemioVideoFromAvi::deserialize(&avi)).expect("deserialize");
    assert_eq!(back, original);
}

#[semio_framework_async_macros::async_test]
async fn subtitle_kind_folds_to_auds_fcc_type_honestly_documented() {
    let mut snap = real_world_video();
    snap.streams[0].kind = SemioVideoStreamKind::Subtitle;
    let avi = semio_framework_plugin::resolve_ready(SemioVideoToAvi::serialize(&snap)).expect("serialize");
    assert_eq!(avi.streams[0].strh.fcc_type, "auds");
}
