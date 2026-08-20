//! ⏱ `duration` — one named inference: the wav snapshot's real playback length, derived from the
//! `fmt ` chunk's real `sampleRate`/`channels` plus the real decoded `data` sample count
//! (per-variant: `Pcm16`/`Pcm8`/`Float32` element counts, or — for the honest `Raw` fallback,
//! anything this codec doesn't interpret sample-by-sample — byte length divided by `blockAlign`,
//! the same "bytes per interleaved frame" quantity the RIFF spec itself defines `data`'s size in
//! terms of). A pure whole-snapshot scalar — no `InferredField` needed.

use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{WavData, WavSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Duration
/// ⏱️ wav's `fmt`/`data`-derived playback duration.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WavDuration {
    pub duration_seconds: f64,
    pub frame_count: u64,
    pub bits_per_sample: u16,
}

/// 🌱 Hand-rolled (not derived) — `WavSnapshot::default()`'s `fmt` is a real 44.1kHz/16-bit PCM
/// form (`WavFmt::default()`'s own documented normal form), not a zeroed struct, so a derived
/// all-zero `Default` would disagree with `compute_wav_duration(&WavSnapshot::default())` and
/// silently break `inference_default_law` one level down from the family-root `Inference` type.
impl Default for WavDuration {
    fn default() -> Self {
        compute_wav_duration(&WavSnapshot::default())
    }
}

/// ⏱️ Computes [`WavDuration`] — `frameCount` is samples-per-channel (element count divided by
/// `fmt.channels`, `channels` floored to `1` to avoid a div-by-zero on a malformed `fmt`), and
/// `durationSeconds = frameCount / sampleRate` (`0.0` when `sampleRate` is `0`, an honest
/// degenerate case, not a panic).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_wav_duration(snapshot: &WavSnapshot) -> WavDuration {
    let total_samples: u64 = match &snapshot.data {
        WavData::Pcm16(v) => v.len() as u64,
        WavData::Pcm8(v) => v.len() as u64,
        WavData::Float32(v) => v.len() as u64,
        WavData::Raw(bytes) => {
            if snapshot.fmt.block_align > 0 {
                bytes.len() as u64 / snapshot.fmt.block_align as u64
            } else {
                0
            }
        }
    };
    let channels = (snapshot.fmt.channels as u64).max(1);
    let frame_count = total_samples / channels;
    let duration_seconds = if snapshot.fmt.sample_rate > 0 { frame_count as f64 / snapshot.fmt.sample_rate as f64 } else { 0.0 };
    WavDuration { duration_seconds, frame_count, bits_per_sample: snapshot.fmt.bits_per_sample }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot(sample_rate: u32, channels: u16, data: WavData) -> WavSnapshot {
        WavSnapshot { fmt: WavFmt { sample_rate, channels, ..WavFmt::default() }, data, ..WavSnapshot::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn interleaved_stereo_pcm16_divides_element_count_by_channel_count() {
        let duration = compute_wav_duration(&snapshot(4, 2, WavData::Pcm16(vec![1, -1, 2, -2, 3, -3, 4, -4])));
        assert_eq!(duration, WavDuration { duration_seconds: 1.0, frame_count: 4, bits_per_sample: 16 });
    }

    #[semio_framework_async_macros::async_test]
    async fn raw_fallback_uses_block_align_as_the_honest_bytes_per_frame_divisor() {
        let mut snapshot = snapshot(8, 1, WavData::Raw(vec![0u8; 24]));
        snapshot.fmt.block_align = 3;
        let duration = compute_wav_duration(&snapshot);
        assert_eq!(duration.frame_count, 8);
    }

    #[semio_framework_async_macros::async_test]
    async fn zero_sample_rate_yields_zero_duration_not_a_panic() {
        let duration = compute_wav_duration(&snapshot(0, 1, WavData::Pcm8(vec![0; 4])));
        assert_eq!(duration.duration_seconds, 0.0);
        assert_eq!(duration.frame_count, 4);
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = snapshot(44_100, 2, WavData::Float32(vec![0.0; 8]));
        assert_eq!(compute_wav_duration(&snapshot), compute_wav_duration(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_wav_duration(&WavSnapshot::default()), WavDuration::default());
    }
}
//#endregion 🧪️Tests
