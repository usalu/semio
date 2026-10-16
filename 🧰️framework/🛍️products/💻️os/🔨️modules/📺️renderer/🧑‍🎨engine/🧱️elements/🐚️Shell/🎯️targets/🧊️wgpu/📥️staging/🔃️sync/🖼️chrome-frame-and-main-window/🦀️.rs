//! 🔃️ `shell-sync` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl SyncState {
    /// 💓️ Coalesces one bounded presence preview page for the shared I/O lane. A shell with no sync
    /// channel has no peer to heartbeat to, so the step there would only clear its own flag.
    pub fn request_presence_preview(&mut self, chrome: &mut ChromeState) {
        if self.sync_channel.is_some() {
            chrome.chrome_present.maintenance.presence_requested = true;
        }
    }
}

//#endregion ShellChrome

