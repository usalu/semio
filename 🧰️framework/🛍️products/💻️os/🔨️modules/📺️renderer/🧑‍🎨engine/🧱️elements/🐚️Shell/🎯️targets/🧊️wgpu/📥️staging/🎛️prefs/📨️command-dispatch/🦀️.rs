//! 🎛️ `shell-prefs` staging of `command-dispatch`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl PrefsState {
    //#region 🔖️DirectoryAndIdentity
    //#region 📇️DirectoryLane
    /// 📣️ Refuses one guest `replayShellCommand` out loud: the shared vocabulary's warning notice in this shell's tongue,
    /// carrying the reason's fault code — React's `refuseReplay`.
    pub(super) fn refuse_replay(&mut self, reason: ReplayRefusalReasonV1, chrome: &mut ChromeState) {
        let code = reason.code();
        chrome.show_transient_notice(reason.notice_text(&self.locale_id), semio_framework::Severity::Warning, Some(code.as_str()));
    }
    //#endregion 📇️DirectoryLane
    //#endregion 🔖️DirectoryAndIdentity
}

//#endregion ShellActions

