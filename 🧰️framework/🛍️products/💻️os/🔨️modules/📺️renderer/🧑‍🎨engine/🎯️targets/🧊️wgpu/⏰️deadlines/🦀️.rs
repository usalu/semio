//! ⏰️ Bridges accepted control deadlines and finite animation phases onto the host scheduler.

use ui_render::InvalidationReason;

pub(crate) const RETAINED_CONTROL_CLOCK: ui_render::DeadlineKey = ui_render::DeadlineKey(1);

const UI_ANIMATION_CLOCK: ui_render::DeadlineKey = ui_render::DeadlineKey(2);

/// 🖼️ Retains only the last accepted packet's animation demand across candidate work and discard.
#[derive(Default)]
pub(crate) struct AcceptedAnimationClock {
    active: bool,
    next_seconds: Option<f64>,
}

impl AcceptedAnimationClock {
    pub(crate) fn accept(&mut self, active: bool) {
        self.active = active;
        if !active {
            self.next_seconds = None;
        }
    }

    pub(crate) fn sync(&mut self, scheduler: &mut ui_render::FrameScheduler, now_seconds: f64) {
        self.next_seconds = if self.active && now_seconds.is_finite() { Some(self.next_seconds.filter(|due| *due > now_seconds).unwrap_or(now_seconds + 1.0 / 60.0)) } else { None };
        scheduler.replace_deadline(UI_ANIMATION_CLOCK, self.next_seconds.map(|due| ui_render::Deadline { due, reason: InvalidationReason::ANIMATION }));
    }
}

/// 🅿️ Parks accepted presentation clocks while an external runtime owner prevents publication.
pub(crate) fn sync_presented_deadlines(scheduler: &mut ui_render::FrameScheduler, animation: &mut AcceptedAnimationClock, control: Option<ui_render::Deadline>, now_seconds: f64, awaiting_runtime: bool) {
    if awaiting_runtime {
        scheduler.replace_deadline(UI_ANIMATION_CLOCK, None);
        scheduler.replace_deadline(RETAINED_CONTROL_CLOCK, None);
    } else {
        animation.sync(scheduler, now_seconds);
        scheduler.replace_deadline(RETAINED_CONTROL_CLOCK, control);
    }
}

/// 🎞️ Keeps the shared 1.6/3.2-second shader phase precise at every monotonic clock magnitude.
pub(crate) fn ui_animation_seconds(now_us: Option<u64>) -> f32 {
    now_us.map_or(0.0, |now| (now % 3_200_000) as f32 / 1_000_000.0)
}

/// 🕰️ Translates a worker monotonic deadline into the host scheduler's independent origin.
pub(crate) fn retained_control_deadline(due_us: Option<u64>, now_us: Option<u64>, host_seconds: f64) -> Option<ui_render::Deadline> {
    if !host_seconds.is_finite() {
        return None;
    }
    let remaining_us = due_us?.saturating_sub(now_us?);
    Some(ui_render::Deadline { due: host_seconds + remaining_us as f64 / 1_000_000.0, reason: InvalidationReason::INPUT_STATE })
}

//#region 🔖️Deadlines

//#region ⏳️Constants

/// 🧩️ Native plugin hot-swap mtime poll cadence — this packet's own replacement for the old
/// every-`frame()`-tick plugin-artifact scan storm; a coarse ~1 s poll per the packet brief.
pub const NATIVE_HOT_SWAP_POLL_SECONDS: f64 = 1.0;

//#endregion ⏳️Constants

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
