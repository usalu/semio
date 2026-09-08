
use super::*;
use ui_render::{PointerId, PointerInfo, PointerKind};

#[test]
fn mounted_pointer_storm_callback_p99_stays_below_two_milliseconds() {
    let mut events = ui_host::EventQueue::new();
    let mut scheduler = ui_render::FrameScheduler::new();
    let token = ui_host::UiThreadToken::mint_for_host();
    let pointer = PointerInfo { id: PointerId(1), kind: PointerKind::Mouse, pressure: None, tilt: None };
    let mut generation = 0;
    for sample in 0..20_000 {
        assert_eq!(enqueue_host_event(&mut events, &mut scheduler, token, &mut generation, DispatchEvent::PointerMove { pointer, x: sample as f32, y: 0.0 },), ui_host::EnqueueOutcome::Accepted);
    }
    let (_, _, p99_us) = semio_framework_trace::site_percentiles("os_renderer_event").expect("mounted event callback samples");
    assert!(p99_us < 2_000, "mounted event callback p99 was {p99_us} µs");
    assert_eq!(events.pending_discrete_len(), 0);
}

#[test]
fn mounted_resize_storm_callback_p99_stays_below_two_milliseconds() {
    let mut events = ui_host::EventQueue::new();
    let mut scheduler = ui_render::FrameScheduler::new();
    let token = ui_host::UiThreadToken::mint_for_host();
    let mut generation = 0;
    for sample in 0..20_000 {
        enqueue_host_metrics(&mut events, &mut scheduler, token, &mut generation, 800 + sample % 32, 600 + sample % 32, 2.0);
    }
    let (_, _, p99_us) = semio_framework_trace::site_percentiles("os_renderer_metrics").expect("mounted resize callback samples");
    assert!(p99_us < 2_000, "mounted resize callback p99 was {p99_us} µs");
    let drained = events.drain_page(ui_host::WorkerContext::new(events.current_generation()));
    let latest = drained.metrics.expect("coalesced resize sample");
    assert_eq!((latest.physical_width, latest.physical_height), (831, 631));
}

#[test]
fn mounted_frame_generation_exhaustion_is_permanent_and_non_wrapping() {
    let mut generation = u64::MAX - 1;
    assert!(advance_frame_generation(&mut generation));
    assert_eq!(generation, u64::MAX);
    assert!(!advance_frame_generation(&mut generation));
    assert_eq!(generation, u64::MAX);
}
