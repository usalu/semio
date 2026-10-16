//! 🚶️ `shell-session` staging of `shell-actions`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellActions
impl<'ctx> SessionCtx<'ctx> {
    //#region 🔖️CheckIn
    /// 🧾️ ticket §C5 — resets the local history projection and re-seeds it from a full
    /// `ProgramBridgeEntry::read_history` snapshot (`replace=true`), called right after a document
    /// attaches (`open_document`/`attach_sync_backbone`'s native branch) so a just-opened document's
    /// own uncommitted-edit count starts from its OWN history, never a leftover projection from
    /// whatever was open before. Best-effort: a read failure just leaves the projection empty (the
    /// same posture every other native exchange call in this file already takes on error — logged, not
    /// propagated, since a stale/absent history must never block opening a document).
    pub async fn refresh_history_snapshot(&mut self) {
        self.history.history_cursor = 0;
        self.history.history_entries.clear();
        self.history.history_current_checkpoint_id = None;
        self.history.history_time_travel = None;
        self.history.history_reprojection = None;
        self.sync.last_uncommitted_edit_at_ms = None;
        self.sync.auto_checkin_pending = false;
        self.sync.checkpoint_dispatched = false;
        self.seed_history_snapshot().await;
    }

    /// 📖️ Reads the session program's whole history projection over the local one (`replace = true`): its rows, head,
    /// history-edit session and waiting reprojection. `false` when no session's program could be asked. What a session
    /// start seeds from ([`Self::seed_history_snapshot`]) and what a lossy publication lane falls back to
    /// ([`Self::drain_operation_publications`]).
    pub async fn reread_history_projection(&mut self) -> bool {
        let Some(session) = self.docs.session.as_ref() else { return false };
        let Some(plugin) = self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id) else { return false };
        match plugin.read_history(session.instance_id).await {
            Ok(patch) => {
                fold_history_patch(&mut self.history.history_entries, &mut self.history.history_cursor, &patch, true);
                self.observe_history_time_travel(patch.time_travel.as_ref());
                self.history.observe_history_reprojection(patch.reprojection.as_ref());
                self.history.history_current_checkpoint_id = patch.current_checkpoint_id;
            }
            Err(error) => { let detail = self.prefs.note_refused_open(error, &mut self.chrome); debug_log(&format!("[TRACE] wgpu shell read_history failed: {detail}")); },
        }
        true
    }

    /// 🧾️ The seeding half of [`Self::refresh_history_snapshot`]: the program's whole history projection, read through
    /// the native exchange's `ReadHistory` or the browser bridge's `readHistory` alike, so a session start or a restored
    /// archive shows its rows and head on both builds before the next `history_patch` folds in.
    ///
    /// ⚔️ A freshly attached document brings its own open conflicts with it — React seeds the
    /// Conflicts panel on the same session-start/switch edge; the `ReadConflicts` exchange is native-only.
    pub async fn seed_history_snapshot(&mut self) {
        if !self.reread_history_projection().await {
            return;
        }
        self.prefs.conflicts_seeded = false;
        self.prefs.open_conflicts.clear();
        self.prefs.selected_conflict_id = None;
        #[cfg(not(target_arch = "wasm32"))]
        self.docs.seed_open_conflicts(&mut self.chrome, &self.plugins, &mut self.prefs).await;
    }
    //#endregion 🔖️CheckIn
}

//#endregion ShellActions

