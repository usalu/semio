//! ⏱ `duration` — one named inference: the avi snapshot's real playback length, read straight
//! off the `avih` MainAVIHeader's own `dwTotalFrames` and `dwMicroSecPerFrame` fields (the exact
//! fields a real AVI player consults for total runtime — RIFF/AVI 1.0 defines duration at the
//! container level, not per-stream, unlike mp4's per-track `stts` tables). A pure whole-snapshot
//! scalar read — no `InferredField` needed.

use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Duration
/// ⏱️ avi's `avih` MainAVIHeader-derived playback duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AviDuration {
    pub duration_seconds: f64,
    pub stream_count: u32,
    pub total_frames: u32,
}

/// ⏱️ Computes [`AviDuration`] — `durationSeconds = totalFrames * microSecPerFrame / 1_000_000`,
/// the RIFF/AVI 1.0 MainAVIHeader's own defining relationship (both fields are `u32`, so the
/// product widens through `f64` rather than risking a `u32` overflow for a long high-framerate
/// capture). `streamCount` is a plain `streams.len()` — the number of `strl` stream lists the
/// container actually declared.
pub async fn compute_avi_duration(snapshot: &AviSnapshot) -> AviDuration {
    let duration_seconds = snapshot.main_header.total_frames as f64 * snapshot.main_header.micro_sec_per_frame as f64 / 1_000_000.0;
    AviDuration { duration_seconds, stream_count: snapshot.streams.len() as u32, total_frames: snapshot.main_header.total_frames }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::{AviMainHeader, AviStream};

    async fn snapshot(total_frames: u32, micro_sec_per_frame: u32, stream_count: usize) -> AviSnapshot {
        AviSnapshot { main_header: AviMainHeader { total_frames, micro_sec_per_frame, ..AviMainHeader::default() }, streams: (0..stream_count).map(|_| AviStream::default()).collect(), ..AviSnapshot::default() }
    }

    #[test]
    async fn duration_is_total_frames_times_micro_sec_per_frame() {
        // 10 fps (100_000 microseconds/frame), 2 frames => 0.2s.
        let duration = compute_avi_duration(&snapshot(2, 100_000, 1));
        assert_eq!(duration, AviDuration { duration_seconds: 0.2, stream_count: 1, total_frames: 2 });
    }

    #[test]
    async fn multi_stream_container_counts_every_declared_stream() {
        let duration = compute_avi_duration(&snapshot(0, 0, 3));
        assert_eq!(duration.stream_count, 3);
        assert_eq!(duration.duration_seconds, 0.0);
    }

    #[test]
    async fn inference_determinism_law() {
        let snapshot = snapshot(30, 33_333, 1);
        assert_eq!(compute_avi_duration(&snapshot), compute_avi_duration(&snapshot));
    }

    #[test]
    async fn inference_default_law() {
        assert_eq!(compute_avi_duration(&AviSnapshot::default()), AviDuration::default());
    }
}
//#endregion 🧪️Tests
