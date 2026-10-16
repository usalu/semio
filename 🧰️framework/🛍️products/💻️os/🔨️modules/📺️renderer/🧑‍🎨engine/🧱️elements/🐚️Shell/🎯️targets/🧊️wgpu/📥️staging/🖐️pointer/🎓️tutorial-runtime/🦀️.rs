//! 🖐 `shell-pointer` staging of `tutorial-runtime`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
//#region 🎬️Lifecycle
impl<'ctx> PointerCtx<'ctx> {
    /// 🎬️ Drains `tutorial_pending_document_ops` (queued by a tick/seek this frame) and applies each
    /// through the existing plugin-bridge document mechanisms — the async counterpart to the sync
    /// tick/seek above, called from `AppRuntime::frame` right after `render_chrome` (mirrors how `frame`
    /// already defers `scene_events` the same way, for the same sync/async split).
    ///
    /// 🚧️ `TutorialBase.document_json` is always `None` fleet-wide today (no tutorial
    /// definition populates it, and `last_envelope_dsl` — its only non-`None` source — is
    /// itself never set past its `None` default); a real loader needs the tutorial-content
    /// dsl-text conversion this plan's B5 tutorial-track bullet scopes separately, not a
    /// whole-envelope JSON reader (deleted with `PluginApp::load_document`/`document_dsl`).
    pub async fn tutorial_flush_pending_document_ops(&mut self) {
        if self.tutorial.tutorial_pending_document_ops.is_empty() {
            return;
        }
        let ops = std::mem::take(&mut self.tutorial.tutorial_pending_document_ops);
        self.chrome.chrome_build.tutorial_dispatch_internal = true;
        for op in ops {
            match op {
                TutorialPendingDocOp::LoadArtifactDsl(_json) => {
                    debug_log(&format!("[TRACE] tutorial load document (json) not wired to the pack-only plugin bridge"));
                }
                TutorialPendingDocOp::ApplyOperations(operations) => {
                    if let Err(err) = self.as_actions().apply_mutations(&operations, UiDirtyScope::Full).await {
                        debug_log(&format!("[TRACE] tutorial apply operations failed: {err}"));
                    }
                }
                TutorialPendingDocOp::HistoryAction { action_id, args } => {
                    if let Some(session) = self.docs.session.clone() {
                        let descriptor = ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action_id, args: args.map(semio_framework::DslValue::from) };
                        if let Err(err) = self.as_actions().dispatch_action(descriptor).await {
                            debug_log(&format!("[TRACE] tutorial history action failed: {err}"));
                        }
                    }
                }
            }
        }
        self.chrome.chrome_build.tutorial_dispatch_internal = false;
    }
}
//#endregion 🎬️Lifecycle
//#endregion 🎬️Tutorial
//#endregion ShellChrome
