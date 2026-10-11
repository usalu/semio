//! ⏱ `duration` — the mp4 snapshot's real presentation length.
//! The presentation clock follows the authoritative `mvhd` duration first, then `elst` edit-list
//! segment durations and `tkhd` durations in the movie timescale. Only containers without those
//! values fall back to per-track sample presentation spans derived from `stts` and `ctts`.

use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

//#region 🔖️Duration
/// ⏱️ mp4's movie-clock and edit-list-aware presentation duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct Mp4Duration {
    pub duration_seconds: f64,
    pub track_count: u32,
    pub sample_count: u32,
}

/// ⏱️ Computes [`Mp4Duration`] from `mvhd`, `elst`, `tkhd`, then sample presentation spans.
/// Zero timescales and absent timing remain an honest unknown duration of `0.0`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_mp4_duration(snapshot: &Mp4Snapshot) -> Mp4Duration {
    let movie_seconds = (snapshot.movie.timescale > 0 && snapshot.movie.duration > 0).then(|| snapshot.movie.duration as f64 / snapshot.movie.timescale as f64);
    let edit_seconds = (snapshot.movie.timescale > 0)
        .then(|| {
            snapshot
                .tracks
                .iter()
                .filter(|track| !track.metadata.edits.is_empty())
                .map(|track| track.metadata.edits.iter().fold(0_u64, |total, edit| total.saturating_add(edit.segment_duration)) as f64 / snapshot.movie.timescale as f64)
                .fold(0.0_f64, f64::max)
        })
        .filter(|seconds| *seconds > 0.0);
    let track_header_seconds = (snapshot.movie.timescale > 0)
        .then(|| snapshot.tracks.iter().map(|track| track.metadata.duration as f64 / snapshot.movie.timescale as f64).fold(0.0_f64, f64::max))
        .filter(|seconds| *seconds > 0.0);
    let sample_seconds = snapshot
        .tracks
        .iter()
        .filter_map(|track| {
            if track.timescale == 0 || track.samples.is_empty() {
                return None;
            }
            let mut decode_time = 0_i128;
            let mut first = i128::MAX;
            let mut end = i128::MIN;
            for sample in &track.samples {
                let presentation = decode_time + i128::from(sample.cts_offset);
                first = first.min(presentation);
                end = end.max(presentation + i128::from(sample.duration));
                decode_time += i128::from(sample.duration);
            }
            Some(end.saturating_sub(first).max(0) as f64 / track.timescale as f64)
        })
        .fold(0.0_f64, f64::max);
    let duration_seconds = movie_seconds.or(edit_seconds).or(track_header_seconds).unwrap_or(sample_seconds);
    let sample_count = snapshot.tracks.iter().map(|track| track.samples.len() as u32).sum();
    Mp4Duration { duration_seconds, track_count: snapshot.tracks.len() as u32, sample_count }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
