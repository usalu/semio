
use super::*;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioTag};

#[semio_framework_async_macros::async_test]
async fn real_samples_honestly_error_rather_than_fabricate_compressed_frames() {
    let snap = SemioAudioSnapshot {
        sample_rate: 44_100,
        format: SemioAudioFormat::Float32,
        channels: vec![SemioAudioChannel { samples: vec![0.0, 0.5, -0.5] }],
        tags: vec![SemioAudioTag { key: "title".into(), value: "x".into() }],
        ..SemioAudioSnapshot::default()
    };
    let result = semio_framework_plugin::resolve_ready(SemioAudioToMp3::serialize(&snap));
    assert!(result.is_err(), "must not silently fabricate MP3 frame payloads from raw samples");
}

#[semio_framework_async_macros::async_test]
async fn empty_sample_content_round_trips_to_an_empty_container_without_erroring() {
    let snap = SemioAudioSnapshot { sample_rate: 44_100, channels: vec![SemioAudioChannel { samples: vec![] }], ..SemioAudioSnapshot::default() };
    let mp3 = semio_framework_plugin::resolve_ready(SemioAudioToMp3::serialize(&snap)).expect("no real content -- nothing to fabricate");
    assert!(mp3.frames.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn default_empty_snapshot_serializes_cleanly() {
    let snap = SemioAudioSnapshot::default();
    assert!(semio_framework_plugin::resolve_ready(SemioAudioToMp3::serialize(&snap)).is_ok());
}
