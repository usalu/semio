//! ⚓️ `shell-dock` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl ShellDockState {
    //#region 🔖️NativeBackboneSync
    /// 🚦️ Rewrites the live dock's sync leaf label from the CURRENT pill. The label is baked into the dock
    /// when it is (re)built, so without this the footer pill — and its accessible name — kept `Remote:
    /// detached` after the actor reported `Live` until something else rebuilt the dock (measured, run s12c).
    pub(crate) fn relabel_sync_tab(&mut self, prefs: &PrefsState, sync: &SyncState) {
        let label = shell_sync_pill_text(sync.sync_pill(), prefs.locale_id == "de");
        for anchor in PanelAnchor::ALL {
            if let Some(tab) = self.dock_tabs.tabs_mut(anchor).iter_mut().find(|tab| tab.id == FRAMEWORK_SYNC_PANEL_TAB_ID) {
                tab.label = label.clone();
            }
        }
    }
    //#endregion 🔖️NativeBackboneSync
}

//#endregion ShellActions

