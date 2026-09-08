//! 📤️ Serialize `s.stdio.semio` (v1/audio) into `s.stdio.wav` (riff-pcm/✳️any) — ALWAYS writes
//! `WavData::Float32` (interleaved), regardless of `SemioAudioFormat`'s metadata label: `audio`'s
//! samples are always real `f32` (see the deserializer's own doc comment), and 32-bit IEEE float
//! is the only `WavData` encoding that can carry them back out with zero quantization/clipping
//! loss -- re-quantizing to `Pcm16`/`Pcm8` here would silently introduce lossy rounding this
//! bridge did not need to accept. `fmt.audio_format = 3` (`WAVE_FORMAT_IEEE_FLOAT`),
//! `bits_per_sample = 32`, `block_align`/`byte_rate` derived from real `channels.len()`.
//!
//! Honest, documented lossy field: `audio.tags` has no wav-subset counterpart to encode into (see
//! the deserializer's own doc comment on why this bridge doesn't synthesize a RIFF `LIST INFO`
//! chunk) -- `other_chunks` is always empty on export, so tags do not survive an
//! `audio→wav→audio` round trip; every numeric/sample field does.

use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_wav::standards::riff_pcm::subsets::any::schema::snapshot::{WavData, WavFmt};
use semio_s_artifact_stdio_wav::WavSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

pub struct SemioAudioToWav;

impl ArtifactSerializer for SemioAudioToWav {
    type From = SemioAudioSnapshot;
    type Into = WavSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let channels = from.channels.len().max(1) as u16;
        let frame_count = from.channels.iter().map(|c| c.samples.len()).max().unwrap_or(0);
        let mut interleaved = Vec::with_capacity(frame_count * channels as usize);
        for i in 0..frame_count {
            for ch in &from.channels {
                interleaved.push(ch.samples.get(i).copied().unwrap_or(0.0));
            }
        }
        let block_align = channels * 4;
        let fmt = WavFmt { audio_format: 3, channels, sample_rate: from.sample_rate, byte_rate: from.sample_rate * block_align as u32, block_align, bits_per_sample: 32, ext: None };
        Ok(WavSnapshot { schema: "stdio.wav".into(), fmt, data: WavData::Float32(interleaved), other_chunks: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
