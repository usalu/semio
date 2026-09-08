
use super::*;
use crate::standards::riff_pcm::subsets::any::schema::snapshot::WavFmt;

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
