//! ⏰️ Every source of "something is due later" this crate's own `AppRuntime` used to track as raw
//! `app_now_ms()`-stamped fields (`wheel_zoom_deadline_ms`, `world3d_camera_dispatch_deadlines_ms`,
//! `caret_blink_at_ms`), swept unconditionally on **every** `frame()` call under the old
//! `ControlFlow::Poll` loop. This file is the ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-
//! FAMILY` (packet `os-host`) replacement: each source below is either a token-keyed re-armable
//! deadline (camera settle, wheel-zoom settle — "the token is replaced when re-armed" per the packet
//! brief) or a one-shot policy object (`CaretBlink`, `HotSwapPoll`) that a caller feeds into
//! `ui_render::FrameScheduler` so `winit_app.rs`'s event loop wakes exactly when — and only when —
//! one of these is actually due, never on a fixed per-frame cadence.
//!
//! **Clock-ownership gap, read before wiring `arm_*` call sites (see
//! `📓️terra-os-host-report.md`'s "what remains blocked" section for the full writeup):**
//! `ui_render::FrameScheduler::request_deadline`'s `due` and `FrameScheduler::should_render`'s `now`
//! must be the *same* monotonic clock. `ui_host::window::native::NativeHost`/`CanvasHost` own that
//! clock privately (`MonotonicClock`/`BrowserClock`) and never hand `now` to the
//! `WindowDelegate` — there is no accessor. Every function below therefore takes `now_seconds`
//! as a plain parameter rather than reading any clock itself, so it stays testable with a synthetic
//! clock; the caller (`os_host::OsHost`, see that file's `OsClock`) is responsible for supplying a
//! `now_seconds` that tracks the *same* wall-time source `ui_host` uses (`Instant::now()` native,
//! `performance.now()` wasm) closely enough for these sub-second deadlines — see `OsClock`'s own
//! docstring for why a separately-constructed clock with the same origin epoch is an accepted
//! approximation, not a bug.

use std::collections::HashMap;
use ui_render::{FrameScheduler, InvalidationReason};

//#region 🔖️Deadlines

//#region ⏳️Constants

/// 🕒️ World3D wheel-zoom's settled `setCamera` dispatch — ported verbatim from `AppRuntime`'s old
/// `world3d_camera_dispatch_deadlines_ms` 350 ms constant.
pub const CAMERA_SETTLE_SECONDS: f64 = 0.350;

/// 🕒️ Node-graph wheel-zoom's settle window — ported verbatim from `AppRuntime`'s old
/// `wheel_zoom_deadline_ms` 120 ms constant.
pub const WHEEL_ZOOM_SETTLE_SECONDS: f64 = 0.120;

/// ⌨️ Caret blink half-period — ported verbatim from `AppRuntime`'s old `caret_blink_at_ms` 500 ms
/// constant.
pub const CARET_BLINK_SECONDS: f64 = 0.500;

/// 🧩️ Native plugin hot-swap mtime poll cadence — this packet's own replacement for the old
/// every-`frame()`-tick `std::fs::metadata` stat storm; a coarse ~1 s poll per the packet brief.
pub const NATIVE_HOT_SWAP_POLL_SECONDS: f64 = 1.0;

//#endregion ⏳️Constants

//#region 🔁️TokenDeadlines

/// 🔁️ Sweeps a per-key deadline map (camera settle / wheel-zoom settle both use this exact shape —
/// `AppRuntime`'s own `world3d_camera_dispatch_deadlines_ms` precedent, generalized) and returns the
/// keys at-or-past `now_seconds`, removing them. Ported from `📦️glue.rs`'s
/// `sweep_expired_camera_dispatch_deadlines` (moved here verbatim, generalized off any single
/// product concept — the four `camera_dispatch_deadline_tests` below are the same tests, ported).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn sweep_expired(pending: &mut HashMap<String, f64>, now_seconds: f64) -> Vec<String> {
    let expired: Vec<String> = pending.iter().filter(|(_, due)| now_seconds >= **due).map(|(key, _)| key.clone()).collect();
    for key in &expired {
        pending.remove(key);
    }
    expired
}

/// 🔁️ Arms (or **replaces** — the packet brief's own words) `key`'s deadline at `now_seconds +
/// settle_seconds`, and requests the matching `FrameScheduler` wake so the host's event loop actually
/// resumes at that instant instead of only on the next unrelated invalidation. Calling this again for
/// the same `key` before it fires — the debounce case every wheel tick hits — overwrites the map entry
/// (the "replace" half of "the token is replaced when re-armed"); the *scheduler* still ends up with
/// one stale extra deadline entry from the earlier call, which is harmless: `should_render` only folds
/// due deadlines into the dirty mask, and `sweep_expired` above is what actually decides whether a
/// surface fires, so a spurious extra wake that finds nothing due in `pending` is a no-op frame, not a
/// double dispatch.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn arm(pending: &mut HashMap<String, f64>, scheduler: &mut FrameScheduler, key: &str, now_seconds: f64, settle_seconds: f64, reason: InvalidationReason) {
    let due = now_seconds + settle_seconds;
    pending.insert(key.to_string(), due);
    scheduler.request_deadline(due, reason);
}

/// ✂️ Drops `key`'s pending deadline outright — the immediate-dispatch case (a pointer-release orbit
/// beats any still-pending wheel-settle deadline) that `AppRuntime`'s own doc comment on
/// `world3d_camera_dispatch_deadlines_ms` already calls out: dispatch immediately, then cancel so the
/// debounce sweep doesn't re-dispatch a now-stale pose a moment later.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn cancel(pending: &mut HashMap<String, f64>, key: &str) {
    pending.remove(key);
}

//#endregion 🔁️TokenDeadlines

//#region ⌨️CaretBlink

/// ⌨️ A repeating deadline registered **only while the presented frame actually shows a visible
/// editable caret** — the packet brief's own requirement, verbatim: "a blink timer that runs when
/// nothing is focused is a frame generator". `sync` is called once per `redraw()` with whatever the
/// just-built frame determined about caret presence; `fire` is called only when the scheduler's own
/// `CARET_BLINK` deadline was actually the reason a redraw happened.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CaretBlink {
    visible: bool,
    armed: bool,
}

impl CaretBlink {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self { visible: true, armed: false }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// 👁️ Reconciles the blink timer against whether a caret is present in the frame just built. A
    /// caret appearing arms the repeating deadline; a caret disappearing disarms it and resets to
    /// visible (so the next time a caret appears it starts solid, matching the old
    /// `caret_blink_visible: true` boot default) rather than possibly resuming mid-blink invisible.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn sync(&mut self, scheduler: &mut FrameScheduler, now_seconds: f64, caret_present: bool) {
        if !caret_present {
            self.armed = false;
            self.visible = true;
            return;
        }
        if !self.armed {
            self.armed = true;
            scheduler.request_deadline(now_seconds + CARET_BLINK_SECONDS, InvalidationReason::PAINT);
        }
    }

    /// 🔥️ The blink deadline actually firing: toggles visibility and re-arms for the next half-period.
    /// A caller only invokes this when it already knows a caret is still present this frame (`sync`
    /// having just confirmed it) — calling it on a frame with no caret would re-arm a timer `sync`
    /// itself would immediately disarm again next call, so callers order `sync` before `fire`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn fire(&mut self, scheduler: &mut FrameScheduler, now_seconds: f64) {
        self.visible = !self.visible;
        scheduler.request_deadline(now_seconds + CARET_BLINK_SECONDS, InvalidationReason::PAINT);
    }
}

impl Default for CaretBlink {
    fn default() -> Self {
        Self::new()
    }
}

//#endregion ⌨️CaretBlink

//#region 🎬️TutorialKeyframes

/// 🎬️ Tutorial playback's per-keyframe wake — the tutorial track already knows its own next keyframe
/// timestamp (`ShellState::tutorial_tick`'s playhead), so this is a thin one-line adapter rather than
/// a new scheduling concept: whoever owns the playhead calls this with that timestamp converted to
/// `now_seconds`-relative seconds.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn arm_next_keyframe(scheduler: &mut FrameScheduler, due_seconds: f64) {
    scheduler.request_deadline(due_seconds, InvalidationReason::ANIMATION);
}

//#endregion 🎬️TutorialKeyframes

//#region 📦️AssetFetch

/// 📦️ Asset fetches (glb/map-tile/ui-image) have no deadline of their own — they invalidate the
/// instant they land, not on a timer — so this is `FrameScheduler::invalidate`, not
/// `request_deadline`; kept here rather than inlined at each call site so every invalidation reason
/// this crate uses has exactly one named entry point, matching the other sources in this file.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn on_asset_ready(scheduler: &mut FrameScheduler) {
    scheduler.invalidate(InvalidationReason::RESOURCE_READY);
}

//#endregion 📦️AssetFetch

//#region 🧩️NativeHotSwapPoll

/// 🧩️ Coarse ~1 s poll gate for native plugin hot-swap mtime checks — replaces the old
/// `poll_native_plugin_hot_swap` call sitting unconditionally at the top of every `frame()` (a
/// `std::fs::metadata` stat per plugin, every single tick under `ControlFlow::Poll`). `is_due` is a
/// pure predicate over an explicit `now_seconds` — deliberately **not** coupled to any
/// `FrameScheduler` (unlike this file's other deadline sources): `AppRuntime` (native-only,
/// `app_now_ms()`-clocked, no access to `OsHost`'s scheduler — see `os_host.rs`'s own docstring on why
/// the two live in different ownership scopes) calls this with its own self-consistent clock purely
/// to gate the `std::fs::metadata` calls; `OsHost::redraw` separately re-arms a plain periodic
/// scheduler deadline (its own clock) so a fully idle window still wakes roughly every
/// `NATIVE_HOT_SWAP_POLL_SECONDS` to give this gate a chance to open at all — two independent,
/// individually-correct pieces rather than one that needs both clocks to agree.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HotSwapPoll {
    last_checked_seconds: f64,
}

impl HotSwapPoll {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self { last_checked_seconds: f64::NEG_INFINITY }
    }

    /// ⏱️ `true` at most once per `NATIVE_HOT_SWAP_POLL_SECONDS` window.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_due(&mut self, now_seconds: f64) -> bool {
        if now_seconds - self.last_checked_seconds < NATIVE_HOT_SWAP_POLL_SECONDS {
            return false;
        }
        self.last_checked_seconds = now_seconds;
        true
    }
}

impl Default for HotSwapPoll {
    fn default() -> Self {
        Self::new()
    }
}

//#endregion 🧩️NativeHotSwapPoll

//#endregion 🔖️Deadlines

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️TokenDeadlines — ported verbatim from glue.rs's camera_dispatch_deadline_tests

    #[test]
    fn not_yet_expired_deadline_is_left_pending() {
        let mut pending = HashMap::from([("s1".to_string(), 1_000.0)]);
        let expired = sweep_expired(&mut pending, 999.0);
        assert!(expired.is_empty());
        assert_eq!(pending.get("s1"), Some(&1_000.0));
    }

    #[test]
    fn deadline_exactly_at_now_is_expired() {
        let mut pending = HashMap::from([("s1".to_string(), 1_000.0)]);
        let expired = sweep_expired(&mut pending, 1_000.0);
        assert_eq!(expired, vec!["s1".to_string()]);
        assert!(pending.is_empty(), "an expired surface is removed from the map");
    }

    #[test]
    fn already_expired_deadline_is_swept() {
        let mut pending = HashMap::from([("s1".to_string(), 500.0)]);
        let expired = sweep_expired(&mut pending, 1_000.0);
        assert_eq!(expired, vec!["s1".to_string()]);
        assert!(pending.is_empty());
    }

    #[test]
    fn multiple_surfaces_expire_independently() {
        let mut pending = HashMap::from([("expired-a".to_string(), 100.0), ("expired-b".to_string(), 200.0), ("still-pending".to_string(), 5_000.0)]);
        let mut expired = sweep_expired(&mut pending, 1_000.0);
        expired.sort();
        assert_eq!(expired, vec!["expired-a".to_string(), "expired-b".to_string()]);
        assert_eq!(pending.len(), 1, "only the still-pending surface remains");
        assert!(pending.contains_key("still-pending"));
    }

    #[test]
    fn arming_a_key_makes_it_the_schedulers_next_deadline() {
        let mut pending = HashMap::new();
        let mut scheduler = FrameScheduler::new();
        arm(&mut pending, &mut scheduler, "s1", 0.0, WHEEL_ZOOM_SETTLE_SECONDS, InvalidationReason::SURFACE);
        assert_eq!(scheduler.next_deadline().map(|deadline| deadline.due), Some(WHEEL_ZOOM_SETTLE_SECONDS));
        assert_eq!(pending.get("s1"), Some(&WHEEL_ZOOM_SETTLE_SECONDS));
    }

    #[test]
    fn re_arming_before_it_fires_replaces_the_due_time() {
        let mut pending = HashMap::new();
        let mut scheduler = FrameScheduler::new();
        arm(&mut pending, &mut scheduler, "s1", 0.0, CAMERA_SETTLE_SECONDS, InvalidationReason::SURFACE);
        arm(&mut pending, &mut scheduler, "s1", 0.1, CAMERA_SETTLE_SECONDS, InvalidationReason::SURFACE);
        assert_eq!(pending.get("s1"), Some(&(0.1 + CAMERA_SETTLE_SECONDS)), "the later arm wins the token");
    }

    #[test]
    fn cancel_drops_a_pending_key_with_no_dispatch() {
        let mut pending = HashMap::from([("s1".to_string(), 1_000.0)]);
        cancel(&mut pending, "s1");
        assert!(pending.is_empty());
        assert!(sweep_expired(&mut pending, 1_000.0).is_empty(), "a cancelled key never fires");
    }

    //#endregion 🔖️TokenDeadlines

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

    #[test]
    fn caret_blink_toggles_and_rearms_on_fire() {
        let mut scheduler = FrameScheduler::new();
        let mut blink = CaretBlink::new();
        blink.sync(&mut scheduler, 0.0, true);
        assert!(blink.is_visible());
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
}
