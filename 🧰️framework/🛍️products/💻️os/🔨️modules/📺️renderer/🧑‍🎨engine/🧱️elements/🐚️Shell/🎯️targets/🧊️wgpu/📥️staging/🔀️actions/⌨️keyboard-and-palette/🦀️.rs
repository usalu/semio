//! 🔀️ `shell-actions` staging of `keyboard-and-palette`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellInput
impl<'ctx> ActionsCtx<'ctx> {
    /// 🖐️ Runs the work a browser only permits inside the gesture that asked for it — today exactly
    /// the parked file-open pickers — and nothing else.
    ///
    /// ⚖️ Everything a gesture arms goes to the settle pump, which takes it on a LATER frame. A file
    /// picker cannot wait for one: `input.click()` is gated on USER ACTIVATION, which the browser
    /// grants to the task that handles the click and to no task after it. This is the one exception,
    /// and it is deliberately the narrowest possible: only `pending_file_opens`, only from a pointer
    /// gesture, and still bounded (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎨️ The theme IMPORT picker is the second gesture-bound door (packet W15d) and obeys the same
    /// rule: opened from the gesture that armed it, never from a later settle step.
    #[cfg(target_arch = "wasm32")]
    pub async fn drain_gesture_bound_work(&mut self) {
        if std::mem::take(&mut self.prefs.pending_theme_import) {
            self.run_theme_import_request().await;
        }
        for _ in 0..SHELL_DEFERRED_CHAIN_ROUNDS {
            let file_opens = std::mem::take(&mut self.opening.pending_file_opens);
            if file_opens.is_empty() {
                break;
            }
            for request in file_opens {
                self.opening.run_file_open_request(request, &self.docs).await;
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn drain_gesture_bound_work(&mut self) {}

    /// 📑️ Journals what React journals for the same gesture: `shell.panelTab` when the anchor's path
    /// moves, `shell.panelToggle` when its visibility flips (`🏛️ShellHost/🟦️.tsx:10014`, `:10042`).
    ///
    /// 🩸️ This renderer used to journal NOTHING for a panel press — the surface appeared and
    /// disappeared in `dumpStructure` with no action crossing `dispatch_action` at all, so the parity
    /// probe read every panel step as a difference (`📓️w5b-interaction-parity-probe.md` §4.4).
    pub async fn journal_panel_selection(&mut self, anchor: PanelAnchor, tab_id: &str, was_visible: bool, was_path: Option<String>) -> Result<(), String> {
        let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) else { return Ok(()) };
        let is_de = self.prefs.locale_id == "de";
        let visible = self.dock.anchor_open(anchor);
        let path = self.dock.anchor_state(anchor).path.first().cloned();
        if visible != was_visible {
            let label = shell_chrome_string("shellCommand.panelToggle", is_de);
            let detail = serde_json::json!({ "anchor": anchor.as_str(), "visible": visible });
            self.dispatch_action(note_shell_command_action(&controller_id, "shell.panelToggle", label, Some(detail))).await?;
        }
        if path != was_path {
            let label = shell_chrome_string("shellCommand.panelTab", is_de);
            let detail = serde_json::json!({ "anchor": anchor.as_str(), "tabId": tab_id });
            self.dispatch_action(note_shell_command_action(&controller_id, "shell.panelTab", label, Some(detail))).await?;
        }
        Ok(())
    }

    /// 🌳️ A branch already showing its children row folds that row; otherwise the press opens
    /// the anchor at that branch — React's `PanelTabRow` drill-down, one press per level.
    pub async fn select_panel_tab_step(&mut self, anchor: PanelAnchor, tab_id: &str) -> Result<(), String> {
        if self.dock.dock_tabs.locate(tab_id).and_then(|(found, path)| self.dock.dock_tabs.node_at(found, &path)).is_some_and(DockTabNode::is_branch) {
            if self.dock.anchor_open(anchor) && self.dock.anchor_state(anchor).path.first().is_some_and(|id| id == tab_id) && !self.dock.dock_branch_collapsed(anchor, tab_id) {
                self.dock.toggle_dock_branch(anchor, tab_id);
            } else {
                self.dock.dock_collapsed_branches.remove(&format!("{}/{tab_id}", anchor.as_str()));
                self.dock.toggle_anchor_tab(anchor, tab_id);
            }
            return Ok(());
        }
        let claimed = self.docs.session.as_ref().filter(|_| self.plugins.host_config().is_some()).map(|session| session.app.controller_id.clone());
        let is_app_leaf = self.docs.session.as_ref().is_some_and(|session| flatten_panel_tab_leaves(&session.app.panel_tabs).into_iter().any(|leaf| leaf.id() == tab_id));
        if let Some(controller_id) = claimed.filter(|_| is_app_leaf) {
            self.dispatch_action(ActionDescriptor { controller_id, action: "setActivePanelTab".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "tabId": tab_id }) }).await?;
            return Ok(());
        }
        self.dock.toggle_anchor_tab(anchor, tab_id);
        Ok(())
    }

}

//#endregion ShellInput

