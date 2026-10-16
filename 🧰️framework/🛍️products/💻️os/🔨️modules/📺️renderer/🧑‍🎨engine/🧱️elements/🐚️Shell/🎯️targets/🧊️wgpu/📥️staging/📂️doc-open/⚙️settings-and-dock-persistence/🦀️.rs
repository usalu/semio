//! 📂️ `shell-doc-open` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl OpeningState {
    /// 🧵️ The truthful Task Manager body for this target — React's `TaskManagerWindow`: the running tasks this shell owns
    /// (picked-file imports, [`ShellState::import_task_nodes`]) with their progress and a Cancel each, above the actors. WGPU
    /// has no actor-registry metrics or dispatch bridge, so the actors read React's distinct `no-runtime` state and publish no
    /// actor verbs.
    pub fn build_task_manager_ui(&self, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some("os.task-manager.panel".into()),
            children: vec![self.import_task_nodes(prefs), UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some("os.task-manager.no-runtime".into()),
                children: vec![settings_text_row(shell_chrome_string("taskManager.noRuntime", is_de))],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            })],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }
}

//#endregion ShellLifecycle

