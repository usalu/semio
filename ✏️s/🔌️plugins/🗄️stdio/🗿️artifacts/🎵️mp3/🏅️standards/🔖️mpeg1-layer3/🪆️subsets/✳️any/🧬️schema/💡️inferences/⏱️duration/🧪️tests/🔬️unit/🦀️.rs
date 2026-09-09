use super::*;
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Mp3Frame, Mp3FrameHeader};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn frame(mpeg_version_id: u8, layer: u8, sample_rate_index: u8, channel_mode: u8) -> Mp3Frame {
    Mp3Frame {
        header: Mp3FrameHeader { mpeg_version_id, layer, protection_bit: true, bitrate_index: 9, sample_rate_index, padding: false, private_bit: false, channel_mode, mode_extension: 0, copyright: false, original: false, emphasis: 0 },
        payload: Vec::new(),
    }
}

#[semio_framework_async_macros::async_test]
async fn two_mpeg1_layer3_frames_at_44100hz_sum_to_the_real_1152_sample_duration() {
    // MPEG1 (version_id=3) Layer III (layer=1), sample_rate_index=0 => 44100Hz, mono (channel_mode=3).
    let snapshot = Mp3Snapshot { frames: vec![frame(3, 1, 0, 3), frame(3, 1, 0, 3)], ..Mp3Snapshot::default() };
    let duration = compute_mp3_duration(&snapshot);
    assert_eq!(duration.frame_count, 2);
    assert_eq!(duration.channel_count, 1);
    assert!((duration.duration_seconds - (2.0 * 1152.0 / 44_100.0)).abs() < 1e-9, "got {duration:?}");
}

#[semio_framework_async_macros::async_test]
async fn mpeg2_layer3_frame_uses_the_halved_576_sample_count_and_stereo_channel_mode() {
    // MPEG2 (version_id=2) Layer III, sample_rate_index=0 => 22050Hz, stereo (channel_mode=0).
    let snapshot = Mp3Snapshot { frames: vec![frame(2, 1, 0, 0)], ..Mp3Snapshot::default() };
    let duration = compute_mp3_duration(&snapshot);
    assert_eq!(duration.channel_count, 2);
    assert!((duration.duration_seconds - (576.0 / 22_050.0)).abs() < 1e-9, "got {duration:?}");
}

#[semio_framework_async_macros::async_test]
async fn no_frames_yields_an_honest_zero_not_a_fabricated_channel_count() {
    let duration = compute_mp3_duration(&Mp3Snapshot::default());
    assert_eq!(duration, Mp3Duration::default());
    assert_eq!(duration.channel_count, 0);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Mp3Snapshot { frames: vec![frame(3, 1, 0, 3)], ..Mp3Snapshot::default() };
    assert_eq!(compute_mp3_duration(&snapshot), compute_mp3_duration(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_mp3_duration(&Mp3Snapshot::default()), Mp3Duration::default());
}
