//! 🖐️ `shell-pointer` staging of `storage-prefs-and-i18n`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔑️StorageKeys
/// 🆔️ The introduction chrome's control ids, byte-identical to React's own
/// (`UIIntroduction`'s `close={{ id: "ui.introduction.skip" }}`, `<Button id="ui.introduction.back">`,
/// `<Button id="ui.introduction.next">`, and the keybinding rows in
/// `🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`).
///
/// 🩸️ This renderer used to mint `shell.tour.{skip,next,back}` — ids React has never had, so a parity
/// probe driving ONE journey across both renderers had to carry an alias table and every tour step read
/// as a difference (`📓️w5b-interaction-parity-probe.md` §4.1).
pub(crate) const UI_INTRODUCTION_SKIP_CONTROL_ID: &str = "ui.introduction.skip";

pub(crate) const UI_INTRODUCTION_NEXT_CONTROL_ID: &str = "ui.introduction.next";

pub(crate) const UI_INTRODUCTION_BACK_CONTROL_ID: &str = "ui.introduction.back";

/// 🧯️ The blocking veil itself. React's scrim is a real `pointer-events-auto` element with its own
/// `onPointerDown={skip}`; giving it a control id is what lets `pointer_press_belongs_to_shell_chrome`
/// claim the press before any 3D surface under it can.
pub(crate) const UI_INTRODUCTION_VEIL_CONTROL_ID: &str = "ui.introduction.veil";

//#endregion 🔑️StorageKeys
//#endregion ShellChrome

