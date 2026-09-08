
use super::*;
use crate::standards::v1::subsets::audio::schema::snapshot::{STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA, SemioAudioChannel};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(sample_rate: u32, channel_lengths: &[usize]) -> SemioAudioSnapshot {
    SemioAudioSnapshot { schema: STDIO_SEMIOAUDIO_DOCUMENT_SCHEMA.into(), sample_rate, format: Default::default(), channels: channel_lengths.iter().map(|&len| SemioAudioChannel { samples: vec![0.0; len] }).collect(), tags: Vec::new() }
}

#[semio_framework_async_macros::async_test]
async fn duration_uses_the_longest_channel_and_sample_rate() {
    let duration = compute_semio_audio_duration(&snapshot(4, &[8, 12]));
    assert_eq!(duration, SemioAudioDuration { duration_seconds: 3.0, sample_count: 12, channel_count: 2 });
}

#[semio_framework_async_macros::async_test]
async fn zero_sample_rate_yields_zero_duration_not_a_panic() {
    let duration = compute_semio_audio_duration(&snapshot(0, &[8]));
    assert_eq!(duration.duration_seconds, 0.0);
    assert_eq!(duration.sample_count, 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot(44100, &[44100, 22050]);
    assert_eq!(compute_semio_audio_duration(&snapshot), compute_semio_audio_duration(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_semio_audio_duration(&SemioAudioSnapshot::default()), SemioAudioDuration::default());
}
