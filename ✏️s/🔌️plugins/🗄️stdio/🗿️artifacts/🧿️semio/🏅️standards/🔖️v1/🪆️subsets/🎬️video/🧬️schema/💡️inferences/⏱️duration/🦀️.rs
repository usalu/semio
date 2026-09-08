//! ⏱ `duration` — one named inference: the container's real elapsed time, the max across every
//! stream's own `(max pts + its own rate) -> seconds` — same shape `animation`'s/`audio`'s own
//! duration facets establish for their own multi-track fold (the longest track bounds the
//! container, matching gltf-style clip duration and the audio facet's own "longest channel"
//! reasoning). `data` (the opaque compressed payload) is never read — pts/rate alone are enough,
//! honoring this subset's own opaque-payload boundary.

use crate::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot, SemioVideoStream};

//#region 🔖️Duration
/// ⏱️ Semio video container duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioVideoDuration {
    pub duration_seconds: f64,
    pub stream_count: u32,
    pub sample_count: u32,
}

/// ⏱️ One stream's own elapsed time: `(max pts among its samples) * (rate.den / rate.num)` — `pts`
/// is expressed in units of `rate` ticks per second, so dividing by the rate converts to seconds.
/// `0.0` for an empty stream or a zero numerator (honest degenerate case, not a panic — matches
/// `audio`'s own `sampleRate == 0` handling).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn stream_duration_seconds(stream: &SemioVideoStream) -> f64 {
    if stream.rate.num == 0 {
        return 0.0;
    }
    let max_pts = stream.samples.iter().map(|s| s.pts).max().unwrap_or(0);
    max_pts as f64 * (stream.rate.den as f64 / stream.rate.num as f64)
}

/// ⏱️ Computes [`SemioVideoDuration`] — pure, total, O(streams + samples).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_video_duration(snapshot: &SemioVideoSnapshot) -> SemioVideoDuration {
    let duration_seconds = snapshot.streams.iter().map(stream_duration_seconds).fold(0.0_f64, f64::max);
    let sample_count = snapshot.streams.iter().map(|s| s.samples.len() as u32).sum();
    SemioVideoDuration { duration_seconds, stream_count: snapshot.streams.len() as u32, sample_count }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
