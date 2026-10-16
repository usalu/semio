//! ⏳️ `shell-async` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour
/// 🖱️ Edge-detects "pointer just went down this frame", computed once per `render_chrome` call —
/// `InputState::pointer_down` is level-triggered (true every frame the button is held), so the
/// chrome-owned click handling below (dialogs, tour controls, the tour trigger) that lives outside the
/// `ActionDescriptor`/`ShellActions` dispatch pipeline needs its own edge tracking to avoid re-firing on
/// every held frame. Reads afterward (`chrome_clicked_this_frame`) are pure.
/// ⏱️ Self-contained wall-clock reader for the tooltip hover-delay timer — deliberately not sharing the
/// file's other `*_now_ms` helper (its exact module/scope kept shifting under concurrent edits from
/// `w3-prefs-i18n-themes` while this region was being written), same cfg-gated `js_sys::Date::now()` /
/// `SystemTime` split as the rest of this file already uses elsewhere.
#[cfg(target_arch = "wasm32")]
pub(crate) fn chrome_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn chrome_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

