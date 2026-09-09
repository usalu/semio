use super::*;
use crate::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "h264".into(),
                width: 1920,
                height: 1080,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![] }, SemioVideoSample { pts: 59, key: false, data: vec![] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1 }, samples: vec![SemioVideoSample { pts: 96_000, key: true, data: vec![] }] },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn duration_is_the_max_across_every_stream() {
    let duration = compute_semio_video_duration(&populated());
    // video stream: 59 / (30/1) ≈ 1.9667s; audio stream: 96000 / (48000/1) = 2.0s — audio wins.
    assert!((duration.duration_seconds - 2.0).abs() < 1e-9, "expected audio stream's 2.0s to win, got {}", duration.duration_seconds);
    assert_eq!(duration.stream_count, 2);
    assert_eq!(duration.sample_count, 3);
}

#[semio_framework_async_macros::async_test]
async fn zero_rate_stream_contributes_zero_not_a_panic() {
    let snapshot = SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 0, den: 1 }, samples: vec![SemioVideoSample { pts: 5, key: false, data: vec![] }] }],
    };
    let duration = compute_semio_video_duration(&snapshot);
    assert_eq!(duration.duration_seconds, 0.0);
    assert_eq!(duration.sample_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated();
    assert_eq!(compute_semio_video_duration(&snapshot), compute_semio_video_duration(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_video_duration(&SemioVideoSnapshot::default()), SemioVideoDuration::default());
}
