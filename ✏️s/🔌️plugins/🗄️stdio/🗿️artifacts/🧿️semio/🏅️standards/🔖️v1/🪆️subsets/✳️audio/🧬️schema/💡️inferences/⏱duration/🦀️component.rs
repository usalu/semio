//! ⏱ `duration` — one named inference: the semio audio snapshot's real playback length, derived
//! from `sampleRate` and the longest decoded `channels[].samples` sequence (real audio may carry
//! channels of unequal length if a decoder under-ran one channel — the longest one is the honest
//! playback length, matching how a real player would report duration). A pure whole-snapshot
//! scalar (one max-length fold over `channels`) — no `InferredField` needed.

use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Duration
/// ⏱️ Semio audio's sample-count-derived playback duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAudioDuration {
    pub duration_seconds: f64,
    pub sample_count: u64,
    pub channel_count: u32,
}

/// ⏱️ Computes [`SemioAudioDuration`] — `sample_count` is the LONGEST channel's sample count
/// (not the sum, which would overcount a multi-channel file); `duration_seconds` is
/// `sample_count / sample_rate`, `0.0` when `sample_rate` is `0` (an honest degenerate case, not
/// a division panic — `sample_rate: u32` cannot be negative).
pub fn compute_semio_audio_duration(snapshot: &SemioAudioSnapshot) -> SemioAudioDuration {
    let sample_count = snapshot.channels.iter().map(|channel| channel.samples.len() as u64).max().unwrap_or(0);
    let duration_seconds = if snapshot.sample_rate > 0 { sample_count as f64 / snapshot.sample_rate as f64 } else { 0.0 };
    SemioAudioDuration { duration_seconds, sample_count, channel_count: snapshot.channels.len() as u32 }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};

    fn snapshot(sample_rate: u32, channel_lengths: &[usize]) -> SemioAudioSnapshot {
        SemioAudioSnapshot { schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(), sample_rate, format: Default::default(), channels: channel_lengths.iter().map(|&len| SemioAudioChannel { samples: vec![0.0; len] }).collect(), tags: Vec::new() }
    }

    #[test]
    fn duration_uses_the_longest_channel_and_sample_rate() {
        let duration = compute_semio_audio_duration(&snapshot(4, &[8, 12]));
        assert_eq!(duration, SemioAudioDuration { duration_seconds: 3.0, sample_count: 12, channel_count: 2 });
    }

    #[test]
    fn zero_sample_rate_yields_zero_duration_not_a_panic() {
        let duration = compute_semio_audio_duration(&snapshot(0, &[8]));
        assert_eq!(duration.duration_seconds, 0.0);
        assert_eq!(duration.sample_count, 8);
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = snapshot(44100, &[44100, 22050]);
        assert_eq!(compute_semio_audio_duration(&snapshot), compute_semio_audio_duration(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_semio_audio_duration(&SemioAudioSnapshot::default()), SemioAudioDuration::default());
    }
}
//#endregion 🧪️Tests
