//! ⏱ `duration` — one named inference: the mp4 snapshot's real container playback length. Each
//! `Mp4Track` already carries its `stts`-flattened per-sample `duration` (in the track's own
//! `timescale` units — real ISO-BMFF box fields, not fabricated); this leaf sums those per track
//! and reports the LONGEST track's duration as the container's own (the same "bounded by the
//! slowest-ending track" convention `🧿️semio/✳️animation`'s clip-duration facet already
//! established for gltf-style multi-channel timing). A pure whole-snapshot fold — no
//! `InferredField` needed.

use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

//#region 🔖️Duration
/// ⏱️ mp4's per-track `stts`-derived container duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Mp4Duration {
    pub duration_seconds: f64,
    pub track_count: u32,
    pub sample_count: u32,
}

/// ⏱️ Computes [`Mp4Duration`] — per track, `durationSeconds` = `sum(samples[].duration) /
/// timescale` (`0.0` for a `timescale` of `0`, an honest degenerate case, not a panic);
/// `durationSeconds` reports the MAXIMUM across tracks (the container plays until its longest
/// track ends). `sampleCount` sums every track's sample count.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
