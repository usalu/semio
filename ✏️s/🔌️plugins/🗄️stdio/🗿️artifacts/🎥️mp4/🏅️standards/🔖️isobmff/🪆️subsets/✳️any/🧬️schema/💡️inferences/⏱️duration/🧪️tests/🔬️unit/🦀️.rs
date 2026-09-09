use super::*;
use crate::standards::isobmff::subsets::any::schema::snapshot::{Mp4Sample, Mp4Track};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn track(timescale: u32, sample_durations: &[u32]) -> Mp4Track {
    Mp4Track { timescale, samples: sample_durations.iter().map(|&d| Mp4Sample { duration: d, ..Mp4Sample::default() }).collect(), ..Mp4Track::default() }
}

#[semio_framework_async_macros::async_test]
async fn container_duration_is_bounded_by_the_slowest_ending_track() {
    let snapshot = Mp4Snapshot {
        tracks: vec![
            track(1000, &[33, 33]),         // 0.066s
            track(1000, &[33, 33, 33, 33]), // 0.132s — the real container duration
        ],
        ..Mp4Snapshot::default()
    };
    let duration = compute_mp4_duration(&snapshot);
    assert_eq!(duration.track_count, 2);
    assert_eq!(duration.sample_count, 6);
    assert!((duration.duration_seconds - 0.132).abs() < 1e-9, "got {duration:?}");
}

#[semio_framework_async_macros::async_test]
async fn zero_timescale_track_contributes_zero_not_a_panic() {
    let snapshot = Mp4Snapshot { tracks: vec![track(0, &[10, 10])], ..Mp4Snapshot::default() };
    let duration = compute_mp4_duration(&snapshot);
    assert_eq!(duration.duration_seconds, 0.0);
    assert_eq!(duration.sample_count, 2);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = Mp4Snapshot { tracks: vec![track(600, &[600])], ..Mp4Snapshot::default() };
    assert_eq!(compute_mp4_duration(&snapshot), compute_mp4_duration(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_mp4_duration(&Mp4Snapshot::default()), Mp4Duration::default());
}
