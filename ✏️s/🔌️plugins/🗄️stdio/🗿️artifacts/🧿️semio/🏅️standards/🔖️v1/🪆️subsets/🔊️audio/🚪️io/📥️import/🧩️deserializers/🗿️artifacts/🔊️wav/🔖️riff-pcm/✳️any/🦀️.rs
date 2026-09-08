//! 📥️ Deserialize `s.stdio.wav` (riff-pcm/✳️any) into `s.stdio.semio` (v1/audio) — the LOSSLESS
//! half of this ticket's audio bridges (per the master plan: "wav's PCM data maps directly and
//! losslessly"). `WavData::{Pcm16,Pcm8,Float32}` de-interleave exactly into `channels[i].samples`
//! (real f32, widened/rescaled with the standard PCM↔float conventions -- signed 16-bit divides by
//! 32768, unsigned 8-bit centers on 128 then divides by 128, Float32 copies through bit-for-bit).
//!
//! Honest, documented lossy fields (real, unavoidable):
//! - `WavData::Raw` (24-bit/32-bit-int/ADPCM/WAVE_FORMAT_EXTENSIBLE payloads this codec's own
//!   honest boundary declines to interpret sample-by-sample -- see `WavData`'s own doc comment)
//!   has no sample content to de-interleave; `channels` still gets the right COUNT (from
//!   `fmt.channels`) with every channel's `samples` left empty, never fabricated.
//! - `WavFmt.ext`/`other_chunks` (RIFF `LIST`/`INFO`/`cue `/… chunks, `WAVE_FORMAT_EXTENSIBLE`
//!   tail bytes) have no `audio`-subset counterpart and are dropped -- `audio.tags` is NOT
//!   populated from these, since `wav`'s own snapshot keeps them raw/undecoded (see
//!   `RiffChunk`'s own doc comment) and decoding a `LIST INFO` sub-chunk here would be re-parsing
//!   bytes this bridge's job explicitly excludes ("zero codec reimplementation").

use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_wav::standards::riff_pcm::subsets::any::schema::snapshot::WavData;
use semio_s_artifact_stdio_wav::WavSnapshot;
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };

pub struct SemioAudioFromWav;

impl ArtifactDeserializer for SemioAudioFromWav {
    type From = WavSnapshot;
    type Into = SemioAudioSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let channel_count = from.fmt.channels.max(1) as usize;
        let (format, interleaved): (SemioAudioFormat, Vec<f32>) = match &from.data {
            WavData::Pcm16(samples) => (SemioAudioFormat::Pcm16, samples.iter().map(|&s| s as f32 / 32_768.0).collect()),
            WavData::Pcm8(samples) => (SemioAudioFormat::Pcm8, samples.iter().map(|&s| (s as f32 - 128.0) / 128.0).collect()),
            WavData::Float32(samples) => (SemioAudioFormat::Float32, samples.clone()),
            WavData::Raw(_) => (
                if from.fmt.bits_per_sample == 24 {
                    SemioAudioFormat::Pcm24
                } else if from.fmt.bits_per_sample >= 32 && from.fmt.audio_format == 3 {
                    SemioAudioFormat::Float64
                } else {
                    SemioAudioFormat::Pcm32
                },
                Vec::new(),
            ),
        };
        let mut channels: Vec<SemioAudioChannel> = (0..channel_count).map(|_| SemioAudioChannel { samples: Vec::new() }).collect();
        for (i, &sample) in interleaved.iter().enumerate() {
            channels[i % channel_count].samples.push(sample);
        }
        Ok(SemioAudioSnapshot { schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(), sample_rate: from.fmt.sample_rate, format, channels, tags: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
