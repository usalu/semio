//! 🔀️ `shell-actions` staging of `time-travel`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

impl<'ctx> ActionsCtx<'ctx> {
    /// 🏁️ Applies what the session's guest published between two dispatches — the wgpu twin of React's
    /// `subscribeOperationCompletions` and `subscribeOperationProgress` passes. A typed operation that outlives the host
    /// call that started it (an example load of a few hundred steps, the replay behind an accepted history edit) ends
    /// on the bridge's standing drain, and its completion is the only carrier of its dirty scope and command-log
    /// delta. Each publication's history patch takes the reply's own stale-guarded fold
    /// ([`ShellState::observe_invocation_history`]) and the union of what they owe ([`operation_publication_refresh`])
    /// goes to the settle lane ([`ShellState::owe_refresh`]) — the surface refreshes and the rows land with no further
    /// dispatch. A drain `fault` is told through the dispatch-fault funnel; `resync` re-reads the whole projection.
    /// `true` when the session, the rows or the owed refresh changed, or a fault was told.
    pub async fn drain_operation_publications(&mut self) -> bool {
        let publications = self.docs.session.as_ref().and_then(|session| self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|plugin| plugin.take_operation_publications(session.instance_id))).unwrap_or_default();
        if publications.is_empty() {
            return false;
        }
        let before = (self.history.history_time_travel.clone(), self.history.history_reprojection.clone(), self.history.history_cursor, self.history.history_entries.len());
        let mut owed = UiDirtyScope::None;
        let mut told = false;
        for publication in &publications {
            if let Some(fault) = publication.fault.as_deref() {
                debug_log(&format!("[TRACE] wgpu shell operation publication fault: {fault}"));
                self.prefs.note_dispatch_fault(fault, &mut self.chrome);
                told = true;
            }
            if publication.resync {
                self.as_session().reread_history_projection().await;
            }
            self.observe_invocation_history(publication.history_patch.as_ref()).await;
            owed = owed.merged_with(operation_publication_refresh(publication));
        }
        let owes = !owed.asks_for_nothing();
        self.settle.owe_refresh(owed);
        told || owes || before != (self.history.history_time_travel.clone(), self.history.history_reprojection.clone(), self.history.history_cursor, self.history.history_entries.len())
    }
}

