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

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use crate::artifacts::wav::WavSnapshot;
use crate::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{WavData, WavFmt};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("audio") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.wav", standard: StandardId("riff-pcm"), subset: SubsetId("*") };

pub struct SemioAudioToWav;

impl ArtifactSerializer for SemioAudioToWav {
    type From = SemioAudioSnapshot;
    type Into = WavSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioTag, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
    use crate::artifacts::semio::standards::v1::subsets::audio::io::wav_deserializer::SemioAudioFromWav;
    use semio_framework_plugin::ArtifactDeserializer;

    fn real_world_audio_no_tags() -> SemioAudioSnapshot {
        SemioAudioSnapshot {
            schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(),
            sample_rate: 44_100,
            format: SemioAudioFormat::Pcm16,
            channels: vec![SemioAudioChannel { samples: vec![0.0, 0.5, -0.5, 1.0] }, SemioAudioChannel { samples: vec![0.0, -0.5, 0.5, -1.0] }],
            tags: Vec::new(),
        }
    }

    /// 🧪️ codec_retention_law: audio → wav → audio is a LOSSLESS fixpoint for every field except
    /// `format` (always normalizes to `Float32`, documented above) and `tags` (dropped,
    /// documented above) -- constructed with neither here so equality holds field-for-field.
    #[test]
    fn audio_to_wav_to_audio_round_trips_losslessly_for_samples_and_rate() {
        let original = real_world_audio_no_tags();
        let wav = SemioAudioToWav::serialize(&original).expect("serialize");
        assert_eq!(wav.fmt.channels, 2);
        assert_eq!(wav.fmt.sample_rate, 44_100);
        assert_eq!(wav.fmt.audio_format, 3);
        let back = SemioAudioFromWav::deserialize(&wav).expect("deserialize");
        assert_eq!(back.sample_rate, original.sample_rate);
        assert_eq!(back.channels, original.channels);
        assert_eq!(back.format, SemioAudioFormat::Float32); // normalized, documented
    }

    #[test]
    fn tags_are_intentionally_dropped_on_export_documented_lossy() {
        let mut snap = real_world_audio_no_tags();
        snap.tags = vec![SemioAudioTag { key: "title".into(), value: "clean".into() }];
        let wav = SemioAudioToWav::serialize(&snap).expect("serialize");
        assert!(wav.other_chunks.is_empty());
        let back = SemioAudioFromWav::deserialize(&wav).expect("deserialize");
        assert!(back.tags.is_empty());
    }

    #[test]
    fn mismatched_channel_lengths_pad_shorter_channel_with_silence_not_panic() {
        let snap = SemioAudioSnapshot {
            channels: vec![SemioAudioChannel { samples: vec![1.0, 2.0, 3.0] }, SemioAudioChannel { samples: vec![1.0] }],
            ..real_world_audio_no_tags()
        };
        let wav = SemioAudioToWav::serialize(&snap).expect("serialize");
        match &wav.data {
            WavData::Float32(v) => assert_eq!(v.len(), 6),
            other => panic!("expected Float32, got {other:?}"),
        }
    }
}
//#endregion 🔖️Tests
