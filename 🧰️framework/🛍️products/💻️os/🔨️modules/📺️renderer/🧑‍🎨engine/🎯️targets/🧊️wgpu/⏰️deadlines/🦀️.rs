//! ⏰️ Every source of "something is due later" this crate's own `AppRuntime` used to track as raw
//! `app_now_ms()`-stamped fields such as `caret_blink_at_ms`, swept unconditionally on **every**
//! `frame()` call under the old
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

use ui_render::{FrameScheduler, InvalidationReason};

//#region 🔖️Deadlines

//#region ⏳️Constants

/// ⌨️ Caret blink half-period — ported verbatim from `AppRuntime`'s old `caret_blink_at_ms` 500 ms
/// constant.
pub const CARET_BLINK_SECONDS: f64 = 0.500;

/// 🧩️ Native plugin hot-swap mtime poll cadence — this packet's own replacement for the old
/// every-`frame()`-tick plugin-artifact scan storm; a coarse ~1 s poll per the packet brief.
pub const NATIVE_HOT_SWAP_POLL_SECONDS: f64 = 1.0;

//#endregion ⏳️Constants

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
    #[cfg(test)]
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
    #[cfg(test)]
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


//#endregion 🎬️TutorialKeyframes

//#region 📦️AssetFetch


//#endregion 📦️AssetFetch

//#region 🧩️NativeHotSwapPoll

/// 🧩️ Coarse ~1 s poll gate for native plugin hot-swap mtime checks — replaces the old
/// `poll_native_plugin_hot_swap` call sitting unconditionally at the top of every `frame()` (a
/// plugin-artifact metadata request per plugin, every single tick under `ControlFlow::Poll`). `is_due` is a
/// pure predicate over an explicit `now_seconds` — deliberately **not** coupled to any
/// `FrameScheduler` (unlike this file's other deadline sources): `AppRuntime` (native-only,
/// `app_now_ms()`-clocked, no access to `OsHost`'s scheduler — see `os_host.rs`'s own docstring on why
/// the two live in different ownership scopes) calls this with its own self-consistent clock purely
/// to gate the worker I/O-lane scan submissions; `OsHost::redraw` separately re-arms a plain periodic
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
#[path = "../../../🧪️tests/🔬️wgpu-deadlines-unit/🦀️.rs"]
mod tests;
