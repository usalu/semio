//! 🪀️ `shell-view-panels` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
//#region 🧯️TransientNoticeAndAgentOverlays
/// ⏱️ `setTimeout(…, 4000)` in `showTransientNotice` — the auto-dismiss deadline.
pub(crate) const TRANSIENT_NOTICE_AUTO_DISMISS_MS: f64 = 4000.0;

/// 🆔️ The transient notice's own polite status node — the wgpu twin of React's `[data-semio-transient-notice]`.
pub(crate) const TRANSIENT_NOTICE_STATUS_ID: &str = "shell.notice";

/// ⏱️ Pure deadline check, factored out so the 4 s rule is testable without a frame clock.
pub(crate) fn transient_notice_expired(notice: &ShellTransientNotice, now_ms: f64) -> bool {
    now_ms - notice.shown_at_ms >= TRANSIENT_NOTICE_AUTO_DISMISS_MS
}

//#endregion 🧯️TransientNoticeAndAgentOverlays
//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

