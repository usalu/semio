use super::*;
use crate::standards::v1_0::subsets::any::schema::snapshot::{AviMainHeader, AviStream};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(total_frames: u32, micro_sec_per_frame: u32, stream_count: usize) -> AviSnapshot {
    AviSnapshot { main_header: AviMainHeader { total_frames, micro_sec_per_frame, ..AviMainHeader::default() }, streams: (0..stream_count).map(|_| AviStream::default()).collect(), ..AviSnapshot::default() }
}

#[semio_framework_async_macros::async_test]
async fn duration_is_total_frames_times_micro_sec_per_frame() {
    // 10 fps (100_000 microseconds/frame), 2 frames => 0.2s.
    let duration = compute_avi_duration(&snapshot(2, 100_000, 1));
    assert_eq!(duration, AviDuration { duration_seconds: 0.2, stream_count: 1, total_frames: 2 });
}

#[semio_framework_async_macros::async_test]
async fn multi_stream_container_counts_every_declared_stream() {
    let duration = compute_avi_duration(&snapshot(0, 0, 3));
    assert_eq!(duration.stream_count, 3);
    assert_eq!(duration.duration_seconds, 0.0);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = snapshot(30, 33_333, 1);
    assert_eq!(compute_avi_duration(&snapshot), compute_avi_duration(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(compute_avi_duration(&AviSnapshot::default()), AviDuration::default());
}
