//! ⏱ `duration` — one named inference: the mp3 snapshot's real playback length, computed by
//! walking every decoded `Mp3FrameHeader` and looking up its REAL MPEG-1/2/2.5 Layer I/II/III
//! samples-per-frame constant plus its real sample rate (via the engine's own `sample_rate_hz`
//! table, reused — not re-declared — so the two never drift). A pure whole-snapshot fold over
//! `frames` — no `InferredField` needed.

use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::sample_rate_hz;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;

//#region 🔖️Duration
/// ⏱️ mp3's frame-header-derived playback duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Mp3Duration {
    pub duration_seconds: f64,
    pub frame_count: u32,
    pub channel_count: u32,
}

/// 📐️ MPEG Layer samples-per-frame, per ISO/IEC 11172-3 §2.4.2.3 / 13818-3: Layer I (header
/// `layer == 3`) is always 384 samples/frame; Layer II (`layer == 2`) is always 1152; Layer III
/// (`layer == 1`) is 1152 for MPEG1 (`version_id == 3`) but only 576 for MPEG2/2.5
/// (`version_id == 0 | 2`, the halved-rate LSF extension). `layer == 0` is the header's own
/// reserved value — honestly contributes `0` (not fabricated), matching how a real decoder would
/// reject the frame rather than guess a duration for it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn samples_per_frame(version_id: u8, layer: u8) -> u32 {
    match layer {
        3 => 384,
        2 => 1152,
        1 if version_id == 3 => 1152,
        1 => 576,
        _ => 0,
    }
}

/// ⏱️ Computes [`Mp3Duration`] — `durationSeconds` sums, per frame, `samples_per_frame /
/// sample_rate_hz` (frames with a reserved layer/version/sample-rate-index contribute `0`
/// seconds but are still counted in `frameCount`, the same honest-decode-failure treatment the
/// engine's own `bitrate_kbps`/`sample_rate_hz` already give reserved table indices).
/// `channelCount` reads the FIRST frame's `channel_mode` (`3` = mono ⇒ `1` channel, anything else
/// ⇒ `2`) — `0` when there are no frames at all (an honest "unknown", never a fabricated stereo
/// guess).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_mp3_duration(snapshot: &Mp3Snapshot) -> Mp3Duration {
    let duration_seconds: f64 = snapshot
        .frames
        .iter()
        .map(|frame| {
            let h = &frame.header;
            let samples = samples_per_frame(h.mpeg_version_id, h.layer) as f64;
            match sample_rate_hz(h.mpeg_version_id, h.sample_rate_index) {
                Some(rate) if rate > 0 => samples / rate as f64,
                _ => 0.0,
            }
        })
        .sum();
    let channel_count = match snapshot.frames.first() {
        Some(frame) if frame.header.channel_mode == 3 => 1,
        Some(_) => 2,
        None => 0,
    };
    Mp3Duration { duration_seconds, frame_count: snapshot.frames.len() as u32, channel_count }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Mp3Frame, Mp3FrameHeader};

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
}
//#endregion 🧪️Tests
