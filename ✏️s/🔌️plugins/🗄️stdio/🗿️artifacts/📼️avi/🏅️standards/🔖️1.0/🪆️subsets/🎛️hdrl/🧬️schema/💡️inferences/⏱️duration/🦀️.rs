//! ⏱ `duration` — one named inference: the avi snapshot's real playback length, read straight
//! off the `avih` MainAVIHeader's own `dwTotalFrames` and `dwMicroSecPerFrame` fields (the exact
//! fields a real AVI player consults for total runtime — RIFF/AVI 1.0 defines duration at the
//! container level, not per-stream, unlike mp4's per-track `stts` tables). A pure whole-snapshot
//! scalar read — no `InferredField` needed.

use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;

//#region 🔖️Duration
/// ⏱️ avi's `avih` MainAVIHeader-derived playback duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_avi_duration(snapshot: &AviSnapshot) -> AviDuration {
    let duration_seconds = snapshot.main_header.total_frames as f64 * snapshot.main_header.micro_sec_per_frame as f64 / 1_000_000.0;
    AviDuration { duration_seconds, stream_count: snapshot.streams.len() as u32, total_frames: snapshot.main_header.total_frames }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
