//! 🪀️ `shell-view-panels` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl<'ctx> PanelsViewCtx<'ctx> {
    //#region 🔖️NativeBackboneSync
    /// 👥️ The roster `#s-presence-peers` paints, scoped to the attached surface. A document opened
    /// without a hub binding carries no surface and answers an empty roster — exactly the state
    /// React's footer renders as `No one else is here`, not a reason to omit the pill.
    ///
    /// ⏪️ A peer editing the history wears its time-travel activity (`time_travel_peer_presence`).
    pub fn footer_presence_rows(&self) -> Vec<ui_wgpu::wgpu::PresencePeerRow> {
        let Some(surface) = self.directory.presence_surface.as_deref() else { return Vec::new() };
        let mut rows = presence_peer_rows_for_surface(&self.directory.presence_peers, Some(surface), surface);
        let mut activities = self.directory.peer_time_travel_presence(&self.chrome, &self.history, &self.prefs).activities;
        if !activities.is_empty() {
            for row in &mut rows {
                if let Some(index) = activities.iter().position(|(actor, _)| *actor == row.actor) {
                    row.activity = Some(activities.swap_remove(index).1);
                }
            }
        }
        rows
    }
    //#endregion 🔖️NativeBackboneSync
}

//#endregion ShellActions

