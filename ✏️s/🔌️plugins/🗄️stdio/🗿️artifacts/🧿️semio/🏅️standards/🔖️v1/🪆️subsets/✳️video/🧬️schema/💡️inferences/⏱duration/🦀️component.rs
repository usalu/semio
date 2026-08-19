//! ⏱ `duration` — one named inference: the container's real elapsed time, the max across every
//! stream's own `(max pts + its own rate) -> seconds` — same shape `animation`'s/`audio`'s own
//! duration facets establish for their own multi-track fold (the longest track bounds the
//! container, matching gltf-style clip duration and the audio facet's own "longest channel"
//! reasoning). `data` (the opaque compressed payload) is never read — pts/rate alone are enough,
//! honoring this subset's own opaque-payload boundary.

use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStream};
use serde::{Deserialize, Serialize};

//#region 🔖️Duration
/// ⏱️ Semio video container duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioVideoDuration {
    pub duration_seconds: f64,
    pub stream_count: u32,
    pub sample_count: u32,
}

/// ⏱️ One stream's own elapsed time: `(max pts among its samples) * (rate.den / rate.num)` — `pts`
/// is expressed in units of `rate` ticks per second, so dividing by the rate converts to seconds.
/// `0.0` for an empty stream or a zero numerator (honest degenerate case, not a panic — matches
/// `audio`'s own `sampleRate == 0` handling).
async fn stream_duration_seconds(stream: &SemioVideoStream) -> f64 {
    if stream.rate.num == 0 {
        return 0.0;
    }
    let max_pts = stream.samples.iter().map(|s| s.pts).max().unwrap_or(0);
    max_pts as f64 * (stream.rate.den as f64 / stream.rate.num as f64)
}

/// ⏱️ Computes [`SemioVideoDuration`] — pure, total, O(streams + samples).
pub async fn compute_semio_video_duration(snapshot: &SemioVideoSnapshot) -> SemioVideoDuration {
    let duration_seconds = snapshot.streams.iter().map(stream_duration_seconds).fold(0.0_f64, f64::max);
    let sample_count = snapshot.streams.iter().map(|s| s.samples.len() as u32).sum();
    SemioVideoDuration { duration_seconds, stream_count: snapshot.streams.len() as u32, sample_count }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};

    async fn populated() -> SemioVideoSnapshot {
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

    #[test]
    async fn duration_is_the_max_across_every_stream() {
        let duration = compute_semio_video_duration(&populated());
        // video stream: 59 / (30/1) ≈ 1.9667s; audio stream: 96000 / (48000/1) = 2.0s — audio wins.
        assert!((duration.duration_seconds - 2.0).abs() < 1e-9, "expected audio stream's 2.0s to win, got {}", duration.duration_seconds);
        assert_eq!(duration.stream_count, 2);
        assert_eq!(duration.sample_count, 3);
    }

    #[test]
    async fn zero_rate_stream_contributes_zero_not_a_panic() {
        let snapshot = SemioVideoSnapshot {
            schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
            streams: vec![SemioVideoStream { kind: SemioVideoStreamKind::Subtitle, codec: "srt".into(), width: 0, height: 0, rate: SemioRational { num: 0, den: 1 }, samples: vec![SemioVideoSample { pts: 5, key: false, data: vec![] }] }],
        };
        let duration = compute_semio_video_duration(&snapshot);
        assert_eq!(duration.duration_seconds, 0.0);
        assert_eq!(duration.sample_count, 1);
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = populated();
        assert_eq!(compute_semio_video_duration(&snapshot), compute_semio_video_duration(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_video_duration(&SemioVideoSnapshot::default()), SemioVideoDuration::default());
    }
}
//#endregion 🧪️Tests
