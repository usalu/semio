
use super::*;

#[test]
fn should_render_returns_none_for_a_clean_window() {
    let mut scheduler = FrameScheduler::new();
    assert_eq!(scheduler.should_render(0.0), None);
}

#[test]
fn an_invalidation_makes_the_next_should_render_call_return_it() {
    let mut scheduler = FrameScheduler::new();
    scheduler.invalidate(InvalidationReason::LAYOUT);
    let reason = scheduler.should_render(0.0).expect("dirty window must render");
    assert!(reason.contains(InvalidationReason::LAYOUT));
    assert_eq!(scheduler.should_render(0.0), None, "the dirty mask must be drained by the first should_render call");
}

#[test]
fn n_invalidations_coalesce_into_one_frame() {
    let mut scheduler = FrameScheduler::new();
    scheduler.invalidate(InvalidationReason::LAYOUT);
    scheduler.invalidate(InvalidationReason::PAINT);
    scheduler.invalidate(InvalidationReason::THEME);
    let reason = scheduler.should_render(0.0).expect("dirty window must render");
    assert!(reason.contains(InvalidationReason::LAYOUT));
    assert!(reason.contains(InvalidationReason::PAINT));
    assert!(reason.contains(InvalidationReason::THEME));
    assert_eq!(scheduler.should_render(0.0), None);
}

#[test]
fn a_deadline_fires_once_at_its_due_time() {
    let mut scheduler = FrameScheduler::new();
    scheduler.request_deadline(10.0, InvalidationReason::ANIMATION);
    assert_eq!(scheduler.should_render(5.0), None, "not due yet");
    let reason = scheduler.should_render(10.0).expect("due now");
    assert!(reason.contains(InvalidationReason::ANIMATION));
    assert_eq!(scheduler.should_render(10.0), None, "must not fire a second time");
    assert_eq!(scheduler.should_render(20.0), None, "must not fire a second time even later");
}

#[test]
fn a_hidden_window_does_not_render_but_still_tracks_deadlines() {
    let mut scheduler = FrameScheduler::new();
    scheduler.set_visible(false);
    scheduler.request_deadline(10.0, InvalidationReason::ANIMATION);
    assert_eq!(scheduler.should_render(10.0), None, "hidden window never renders");
    assert_eq!(scheduler.next_deadline(), None, "the due deadline was still tracked (folded into dirty) despite not rendering");

    scheduler.set_visible(true);
    let reason = scheduler.should_render(10.0).expect("now visible with accumulated dirt from the hidden period");
    assert!(reason.contains(InvalidationReason::ANIMATION));
}

#[test]
fn next_deadline_reports_the_soonest_pending_one() {
    let mut scheduler = FrameScheduler::new();
    scheduler.request_deadline(50.0, InvalidationReason::VIEWPORT);
    scheduler.request_deadline(10.0, InvalidationReason::ANIMATION);
    scheduler.request_deadline(30.0, InvalidationReason::THEME);
    let soonest = scheduler.next_deadline().expect("deadlines pending");
    assert_eq!(soonest.due, 10.0);
    assert_eq!(soonest.reason, InvalidationReason::ANIMATION);
}

#[test]
fn invalidation_reason_bitor_combines_flags() {
    let combined = InvalidationReason::LAYOUT | InvalidationReason::PAINT;
    assert!(combined.contains(InvalidationReason::LAYOUT));
    assert!(combined.contains(InvalidationReason::PAINT));
    assert!(!combined.contains(InvalidationReason::THEME));
}
