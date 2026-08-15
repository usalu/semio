//! ⏱ `duration` — one named inference: the mp4 snapshot's real container playback length. Each
//! `Mp4Track` already carries its `stts`-flattened per-sample `duration` (in the track's own
//! `timescale` units — real ISO-BMFF box fields, not fabricated); this leaf sums those per track
//! and reports the LONGEST track's duration as the container's own (the same "bounded by the
//! slowest-ending track" convention `🧿️semio/✳️animation`'s clip-duration facet already
//! established for gltf-style multi-channel timing). A pure whole-snapshot fold — no
//! `InferredField` needed.

use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Duration
/// ⏱️ mp4's per-track `stts`-derived container duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mp4Duration {
    pub duration_seconds: f64,
    pub track_count: u32,
    pub sample_count: u32,
}

/// ⏱️ Computes [`Mp4Duration`] — per track, `durationSeconds` = `sum(samples[].duration) /
/// timescale` (`0.0` for a `timescale` of `0`, an honest degenerate case, not a panic);
/// `durationSeconds` reports the MAXIMUM across tracks (the container plays until its longest
/// track ends). `sampleCount` sums every track's sample count.
pub fn compute_mp4_duration(snapshot: &Mp4Snapshot) -> Mp4Duration {
    let duration_seconds = snapshot
        .tracks
        .iter()
        .map(|track| {
            if track.timescale > 0 {
                let ticks: u64 = track.samples.iter().map(|s| s.duration as u64).sum();
                ticks as f64 / track.timescale as f64
            } else {
                0.0
            }
        })
        .fold(0.0_f64, f64::max);
    let sample_count = snapshot.tracks.iter().map(|t| t.samples.len() as u32).sum();
    Mp4Duration { duration_seconds, track_count: snapshot.tracks.len() as u32, sample_count }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Sample, Mp4Track};

    fn track(timescale: u32, sample_durations: &[u32]) -> Mp4Track {
        Mp4Track { timescale, samples: sample_durations.iter().map(|&d| Mp4Sample { duration: d, ..Mp4Sample::default() }).collect(), ..Mp4Track::default() }
    }

    #[test]
    fn container_duration_is_bounded_by_the_slowest_ending_track() {
        let snapshot = Mp4Snapshot {
            tracks: vec![
                track(1000, &[33, 33]),         // 0.066s
                track(1000, &[33, 33, 33, 33]), // 0.132s — the real container duration
            ],
            ..Mp4Snapshot::default()
        };
        let duration = compute_mp4_duration(&snapshot);
        assert_eq!(duration.track_count, 2);
        assert_eq!(duration.sample_count, 6);
        assert!((duration.duration_seconds - 0.132).abs() < 1e-9, "got {duration:?}");
    }

    #[test]
    fn zero_timescale_track_contributes_zero_not_a_panic() {
        let snapshot = Mp4Snapshot { tracks: vec![track(0, &[10, 10])], ..Mp4Snapshot::default() };
        let duration = compute_mp4_duration(&snapshot);
        assert_eq!(duration.duration_seconds, 0.0);
        assert_eq!(duration.sample_count, 2);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = Mp4Snapshot { tracks: vec![track(600, &[600])], ..Mp4Snapshot::default() };
        assert_eq!(compute_mp4_duration(&snapshot), compute_mp4_duration(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_mp4_duration(&Mp4Snapshot::default()), Mp4Duration::default());
    }
}
//#endregion 🧪️Tests
