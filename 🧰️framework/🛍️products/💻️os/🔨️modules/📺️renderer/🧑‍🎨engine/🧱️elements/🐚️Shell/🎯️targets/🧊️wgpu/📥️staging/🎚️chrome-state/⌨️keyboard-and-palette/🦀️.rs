//! 🎚️ `shell-chrome-state` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl ChromeState {
    //#region ⌨️ShellShortcutDispatch
    /// ⌨️ This session's REMAPPED shortcut table — [`SHELL_SHORTCUT_ROWS`] with the persisted
    /// `keybindingOverrides` applied. Every key-event reader goes through this, so a remap takes effect
    /// for dispatch, for reservation and for the shortcut rows W1c paints, from one source.
    pub(crate) fn shortcut_table(&self) -> Vec<(String, String)> {
        shell_shortcut_table(&self.chrome_build.preferences.keybinding_overrides)
    }
    //#endregion ⌨️ShellShortcutDispatch
}

//#endregion ShellInput

