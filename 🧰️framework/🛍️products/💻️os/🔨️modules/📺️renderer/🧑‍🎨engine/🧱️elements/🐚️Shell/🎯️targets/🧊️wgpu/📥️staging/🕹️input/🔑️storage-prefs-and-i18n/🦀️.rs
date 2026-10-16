//! 🕹️ `shell-input` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔑️StorageKeys
/// 🆔️ The tour as ONE dialog-level surface in the chrome census. React's `UIIntroduction` renders its
/// veil and card as several `[data-level="dialog"]` nodes (one of them anonymous), which is a DOM
/// composition, not a behaviour — a probe compares dialog PRESENCE, so one row states it.
pub const UI_INTRODUCTION_DIALOG_ELEMENT_ID: &str = "ui.introduction";

//#endregion 🔑️StorageKeys
//#endregion ShellChrome

