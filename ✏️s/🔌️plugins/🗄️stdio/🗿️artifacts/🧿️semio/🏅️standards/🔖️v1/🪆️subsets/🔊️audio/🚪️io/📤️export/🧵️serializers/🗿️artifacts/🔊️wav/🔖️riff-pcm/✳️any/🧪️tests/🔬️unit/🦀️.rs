use super::*;
use crate::standards::v1::subsets::audio::io::wav_deserializer::SemioAudioFromWav;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioTag, STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
#[semio_framework_async_macros::async_test]
async fn audio_to_wav_to_audio_round_trips_losslessly_for_samples_and_rate() {
    let original = real_world_audio_no_tags();
    let wav = semio_framework_plugin::resolve_ready(SemioAudioToWav::serialize(&original)).expect("serialize");
    assert_eq!(wav.fmt.channels, 2);
    assert_eq!(wav.fmt.sample_rate, 44_100);
    assert_eq!(wav.fmt.audio_format, 3);
    let back = semio_framework_plugin::resolve_ready(SemioAudioFromWav::deserialize(&wav)).expect("deserialize");
    assert_eq!(back.sample_rate, original.sample_rate);
    assert_eq!(back.channels, original.channels);
    assert_eq!(back.format, SemioAudioFormat::Float32); // normalized, documented
}

#[semio_framework_async_macros::async_test]
async fn tags_are_intentionally_dropped_on_export_documented_lossy() {
    let mut snap = real_world_audio_no_tags();
    snap.tags = vec![SemioAudioTag { key: "title".into(), value: "clean".into() }];
    let wav = semio_framework_plugin::resolve_ready(SemioAudioToWav::serialize(&snap)).expect("serialize");
    assert!(wav.other_chunks.is_empty());
    let back = semio_framework_plugin::resolve_ready(SemioAudioFromWav::deserialize(&wav)).expect("deserialize");
    assert!(back.tags.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn mismatched_channel_lengths_pad_shorter_channel_with_silence_not_panic() {
    let snap = SemioAudioSnapshot { channels: vec![SemioAudioChannel { samples: vec![1.0, 2.0, 3.0] }, SemioAudioChannel { samples: vec![1.0] }], ..real_world_audio_no_tags() };
    let wav = semio_framework_plugin::resolve_ready(SemioAudioToWav::serialize(&snap)).expect("serialize");
    match &wav.data {
        WavData::Float32(v) => assert_eq!(v.len(), 6),
        other => panic!("expected Float32, got {other:?}"),
    }
}
