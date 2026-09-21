use super::*;

//#region ⌨️CaretBlink

#[test]
fn caret_blink_never_arms_without_a_caret() {
    let mut scheduler = FrameScheduler::new();
    let mut blink = CaretBlink::new();
    blink.sync(&mut scheduler, 0.0, false);
    assert_eq!(scheduler.next_deadline(), None, "no caret, no timer — a blink timer with nothing focused is a frame generator");
}

#[test]
fn caret_blink_arms_exactly_once_while_present() {
    let mut scheduler = FrameScheduler::new();
    let mut blink = CaretBlink::new();
    blink.sync(&mut scheduler, 0.0, true);
    let first = scheduler.next_deadline();
    blink.sync(&mut scheduler, 0.1, true);
    assert_eq!(scheduler.next_deadline(), first, "a still-present caret does not re-arm a second deadline");
}

/// 🔥️ `fire` is only ever reached AFTER the event loop has consumed the deadline that woke it —
/// `FrameScheduler::should_render` drains every `due <= now` entry into `dirty`
/// (`🖌️render/⏱️schedule/🦀️.rs:133-142`) and only then does the frame call `fire`. Without that
/// drain the scheduler's append-only list still holds the half-period this law just consumed, and
/// `next_deadline` answers the OLD 0.5 instead of the re-armed 1.0
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY wave 2–6 integration; W1's §3.A2 hand-off).
#[test]
fn caret_blink_toggles_and_rearms_on_fire() {
    let mut scheduler = FrameScheduler::new();
    let mut blink = CaretBlink::new();
    blink.sync(&mut scheduler, 0.0, true);
    assert!(blink.is_visible());
    assert!(scheduler.should_render(CARET_BLINK_SECONDS).is_some(), "the half-period is what woke this frame");
    blink.fire(&mut scheduler, CARET_BLINK_SECONDS);
    assert!(!blink.is_visible());
    assert_eq!(scheduler.next_deadline().map(|deadline| deadline.due), Some(CARET_BLINK_SECONDS * 2.0));
}

#[test]
fn caret_disappearing_disarms_and_resets_visible() {
    let mut scheduler = FrameScheduler::new();
    let mut blink = CaretBlink::new();
    blink.sync(&mut scheduler, 0.0, true);
    blink.fire(&mut scheduler, CARET_BLINK_SECONDS);
    assert!(!blink.is_visible());
    blink.sync(&mut scheduler, CARET_BLINK_SECONDS, false);
    assert!(blink.is_visible(), "losing the caret resets to solid, not mid-blink invisible");
    blink.sync(&mut scheduler, CARET_BLINK_SECONDS, true);
    assert!(blink.is_visible(), "a freshly re-armed caret starts solid");
}

//#endregion ⌨️CaretBlink

//#region 🧩️NativeHotSwapPoll

#[test]
fn hot_swap_poll_is_due_immediately_on_first_call() {
    let mut poll = HotSwapPoll::new();
    assert!(poll.is_due(0.0));
}

#[test]
fn hot_swap_poll_is_not_due_again_inside_the_window() {
    let mut poll = HotSwapPoll::new();
    assert!(poll.is_due(0.0));
    assert!(!poll.is_due(NATIVE_HOT_SWAP_POLL_SECONDS - 0.001));
}

#[test]
fn hot_swap_poll_is_due_again_once_the_window_elapses() {
    let mut poll = HotSwapPoll::new();
    assert!(poll.is_due(0.0));
    assert!(poll.is_due(NATIVE_HOT_SWAP_POLL_SECONDS));
}

//#endregion 🧩️NativeHotSwapPoll
