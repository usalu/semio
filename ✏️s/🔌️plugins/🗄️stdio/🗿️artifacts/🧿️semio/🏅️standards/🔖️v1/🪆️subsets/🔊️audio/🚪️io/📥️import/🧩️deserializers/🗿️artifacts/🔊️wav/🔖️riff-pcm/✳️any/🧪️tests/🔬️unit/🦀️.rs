use super::*;
use semio_s_artifact_stdio_wav::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_wav() -> WavSnapshot {
    WavSnapshot {
        schema: "stdio.wav".into(),
        fmt: WavFmt { audio_format: 1, channels: 2, sample_rate: 44_100, byte_rate: 176_400, block_align: 4, bits_per_sample: 16, ext: None },
        data: WavData::Pcm16(vec![0, 0, 16_384, -16_384, 32_767, -32_768]), // interleaved L/R, 3 frames
        other_chunks: vec![],
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_deinterleaves_pcm16_into_real_f32_channels() {
    let audio = semio_framework_plugin::resolve_ready(SemioAudioFromWav::deserialize(&real_world_wav())).expect("deserialize");
    assert_eq!(audio.sample_rate, 44_100);
    assert_eq!(audio.format, SemioAudioFormat::Pcm16);
    assert_eq!(audio.channels.len(), 2);
    assert_eq!(audio.channels[0].samples, vec![0.0, 0.5, 32_767.0 / 32_768.0]);
    assert_eq!(audio.channels[1].samples, vec![0.0, -0.5, -1.0]);
}

#[semio_framework_async_macros::async_test]
async fn raw_fallback_data_yields_correct_channel_count_with_no_fabricated_samples() {
    let mut wav = real_world_wav();
    wav.fmt.bits_per_sample = 24;
    wav.data = WavData::Raw(vec![0u8; 18]);
    let audio = semio_framework_plugin::resolve_ready(SemioAudioFromWav::deserialize(&wav)).expect("deserialize");
    assert_eq!(audio.format, SemioAudioFormat::Pcm24);
    assert_eq!(audio.channels.len(), 2);
    for ch in &audio.channels {
        assert!(ch.samples.is_empty());
    }
}
