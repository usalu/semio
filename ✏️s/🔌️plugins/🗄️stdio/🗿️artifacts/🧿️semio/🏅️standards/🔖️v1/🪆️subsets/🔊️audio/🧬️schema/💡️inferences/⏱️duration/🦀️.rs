//! ⏱ `duration` — one named inference: the semio audio snapshot's real playback length, derived
//! from `sampleRate` and the longest decoded `channels[].samples` sequence (real audio may carry
//! channels of unequal length if a decoder under-ran one channel — the longest one is the honest
//! playback length, matching how a real player would report duration). A pure whole-snapshot
//! scalar (one max-length fold over `channels`) — no `InferredField` needed.

use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;

//#region 🔖️Duration
/// ⏱️ Semio audio's sample-count-derived playback duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioAudioDuration {
    pub duration_seconds: f64,
    pub sample_count: u64,
    pub channel_count: u32,
}

/// ⏱️ Computes [`SemioAudioDuration`] — `sample_count` is the LONGEST channel's sample count
/// (not the sum, which would overcount a multi-channel file); `duration_seconds` is
/// `sample_count / sample_rate`, `0.0` when `sample_rate` is `0` (an honest degenerate case, not
/// a division panic — `sample_rate: u32` cannot be negative).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_audio_duration(snapshot: &SemioAudioSnapshot) -> SemioAudioDuration {
    let sample_count = snapshot.channels.iter().map(|channel| channel.samples.len() as u64).max().unwrap_or(0);
    let duration_seconds = if snapshot.sample_rate > 0 { sample_count as f64 / snapshot.sample_rate as f64 } else { 0.0 };
    SemioAudioDuration { duration_seconds, sample_count, channel_count: snapshot.channels.len() as u32 }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
