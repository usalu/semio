//! @emoji ⏱️ `InvalidationReason` and the `FrameScheduler` that makes idle windows cost zero frames.
//!
//! `should_render(now) -> Option<InvalidationReason>` returning `None` is the whole point of this
//! file: the defect this program replaces is `wgpu-old`'s `🦀️.rs` calling `request_redraw` every
//! frame unconditionally (`ControlFlow::Poll`). A caller (the not-yet-landed `os_host`/`winit_app`
//! region) is expected to poll this once per wake and only build/present a frame when it returns
//! `Some`.
//!
//! `InvalidationReason` is a hand-rolled bitflag `u32`, not the `bitflags` crate — this crate's
//! `Cargo.toml` is registrar-only (ruling U7) and already carries no such dependency; the pattern
//! mirrors the wgpu-old target's own hand-rolled `tree::NodeFlags`.

//#region 🔖️Schedule

//#region 🚩️InvalidationReason

/// 🚩️ Why a window needs another frame. A single [`FrameScheduler::invalidate`] call may carry
/// several reasons at once (bitwise-OR'd); [`FrameScheduler::should_render`] hands back every reason
/// accumulated since the last frame was built, which is what lets N invalidations in one wake cycle
/// coalesce into exactly one frame instead of one each.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct InvalidationReason(u32);

impl InvalidationReason {
    pub const NONE: Self = Self(0);
    pub const STRUCTURE: Self = Self(1 << 0);
    pub const LAYOUT: Self = Self(1 << 1);
    pub const PAINT: Self = Self(1 << 2);
    pub const ANIMATION: Self = Self(1 << 3);
    pub const THEME: Self = Self(1 << 4);
    pub const VIEWPORT: Self = Self(1 << 5);
    pub const RESOURCE_READY: Self = Self(1 << 6);
    pub const INPUT_STATE: Self = Self(1 << 7);
    pub const SURFACE: Self = Self(1 << 8);
    pub const ACCESSIBILITY: Self = Self(1 << 9);

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl std::ops::BitOr for InvalidationReason {
    type Output = Self;
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

//#endregion 🚩️InvalidationReason

//#region ⏰️Deadline

/// ⏰️ A single scheduled wake: at `due` (seconds, same monotonic clock [`FrameScheduler::should_render`]'s
/// `now` uses), invalidate with `reason`. Engine-neutral — this crate has no notion of *why* a product
/// wants a deadline (camera settle, wheel-zoom debounce, caret blink); those policies belong to
/// whatever product/host code calls [`FrameScheduler::request_deadline`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Deadline {
    pub due: f64,
    pub reason: InvalidationReason,
}

//#endregion ⏰️Deadline

//#region 📅️FrameScheduler

/// 📅️ Per-window scheduling state: an accumulated dirty mask, a due-time-ordered deadline queue, and
/// a visibility flag. `should_render` is the only method that matters to a caller deciding whether to
/// build a frame; everything else feeds it.
#[derive(Default)]
pub struct FrameScheduler {
    dirty: InvalidationReason,
    deadlines: Vec<Deadline>,
    visible: bool,
}

impl FrameScheduler {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self { dirty: InvalidationReason::NONE, deadlines: Vec::new(), visible: true }
    }

    /// 🚩️ Marks the window dirty for `reason`, effective the next [`Self::should_render`] call.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn invalidate(&mut self, reason: InvalidationReason) {
        self.dirty.insert(reason);
    }

    /// ⏰️ Schedules a future wake. Multiple deadlines may be pending at once (e.g. a caret blink and
    /// a camera-settle debounce running concurrently); each fires independently at its own `due` time.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn request_deadline(&mut self, due: f64, reason: InvalidationReason) {
        self.deadlines.push(Deadline { due, reason });
    }

    /// 👁️ A hidden window still tracks deadlines (see [`Self::should_render`]'s doc) but never
    /// renders while hidden — set by whatever host code owns the window's OS-level visibility state.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// ⏰️ The soonest still-pending deadline, if any — a caller (the OS event loop) uses this to size
    /// its next `WaitUntil`, independent of whether the window is currently visible.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn next_deadline(&self) -> Option<Deadline> {
        self.deadlines.iter().copied().min_by(|a, b| a.due.total_cmp(&b.due))
    }

    /// 🏁️ Folds every deadline due at or before `now` into the dirty mask (regardless of visibility —
    /// deadlines keep firing while hidden, so nothing is lost or double-counted once the window
    /// becomes visible again), then returns the accumulated mask and clears it — but only if the
    /// window is visible and the mask is non-empty. A hidden window, or a window with nothing dirty,
    /// gets `None`: this is the single method that makes an idle window cost zero frames.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn should_render(&mut self, now: f64) -> Option<InvalidationReason> {
        let mut remaining = Vec::with_capacity(self.deadlines.len());
        for deadline in self.deadlines.drain(..) {
            if deadline.due <= now {
                self.dirty.insert(deadline.reason);
            } else {
                remaining.push(deadline);
            }
        }
        self.deadlines = remaining;

        if !self.visible || self.dirty.is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.dirty))
    }
}

//#endregion 📅️FrameScheduler

//#endregion 🔖️Schedule

#[cfg(test)]
#[path = "../🧪️tests/🔬️schedule-unit/🦀️.rs"]
mod tests;
