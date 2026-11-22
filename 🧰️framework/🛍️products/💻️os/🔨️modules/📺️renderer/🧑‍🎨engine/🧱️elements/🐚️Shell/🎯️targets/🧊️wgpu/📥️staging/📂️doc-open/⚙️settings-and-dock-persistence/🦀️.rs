//! 📂️ `shell-doc-open` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl OpeningState {
    /// 🧵️ React's `TaskManagerWindow`: the running tasks ([`Self::import_task_nodes`]: this shell's transfers, then the runtime's jobs,
    /// installations and tool calls, each with progress, Cancel and, where it can be held, Suspend/Resume) above the actors section
    /// ([`Self::task_manager_actor_nodes`]: the actor table, or the distinct `no-runtime` / `no-actors` notice).
    pub fn build_task_manager_ui(&self, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("os.task-manager.panel".into()),
            children: vec![self.import_task_nodes(prefs), self.task_manager_actor_nodes(is_de)],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }
}

//#endregion ShellLifecycle

