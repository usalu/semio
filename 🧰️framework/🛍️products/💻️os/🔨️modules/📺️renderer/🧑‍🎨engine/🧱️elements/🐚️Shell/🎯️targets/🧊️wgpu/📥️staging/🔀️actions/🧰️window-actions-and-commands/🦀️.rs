//! 🔀️ `shell-actions` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region ⌨️WindowScope

/// ⌨️ Resolves the window an app-wide chord addresses — twin of `resolveKeybindingTargetWindowV1`.
/// `mounted` is the ACTIVE MODE's window instances in layout order, never every declared kind: a verb
/// owned by a window this mode does not mount has no reachable owner and must say so.
pub(crate) fn resolve_keybinding_target_window_v1(app: &AppDefinition, mounted: &[WindowScopeInstanceV1], focused_window_id: Option<&str>, action_id: &str) -> (WindowScopeTargetKindV1, Option<String>) {
    let declares = |window_kind_id: &str| app.window_kinds.iter().any(|kind| kind.id == window_kind_id && semio_framework::window_kind_actions(app, kind).iter().any(|action| action.id == action_id));
    if let Some(focused) = focused_window_id.and_then(|id| mounted.iter().find(|instance| instance.id == id)) {
        if declares(&focused.window_kind_id) {
            return (WindowScopeTargetKindV1::Focused, Some(focused.id.clone()));
        }
    }
    if let Some(owner) = mounted.iter().find(|instance| declares(&instance.window_kind_id)) {
        return (WindowScopeTargetKindV1::Owner, Some(owner.id.clone()));
    }
    (WindowScopeTargetKindV1::Unowned, None)
}

/// 🚦️ `⌘⇧G · Add Generation — not available in this mode`, in the reader's language.
pub(crate) fn keybinding_unowned_text_v1(locale: &str, chord: &str, label: &str) -> String {
    let tail = if locale == "de" { KEYBINDING_UNOWNED_LABEL_DE } else { KEYBINDING_UNOWNED_LABEL_EN };
    format!("{chord} · {label} — {tail}")
}

//#endregion ⌨️WindowScope
impl<'ctx> ActionsCtx<'ctx> {
    /// 🚀️ Executes a staged action once (P2): validates required args, dispatches exactly one
    /// `ActionDescriptor` with the merged effective args, and keeps the staged values for tweak-and-
    /// repeat. No-operations when the active utility gates actions or a required arg is still unset.
    pub(crate) fn execute_staged_action<'a>(&'a mut self, window_id: &'a str, action_id: &'a str) -> ShellTurn<'a, Result<(), String>> {
        shell_turn(async move {
            let Some(session) = self.docs.session.clone() else {
                return Ok(());
            };
            if !self.windows.actions_enabled_for_window(&session.app, window_id) {
                return Ok(());
            }
            let Some(action) = window_action_definition(&session.app, window_id, action_id).cloned() else {
                return Ok(());
            };
            let staged = self.commands.staged_map_for(window_id, action_id);
            let Some(effective) = resolved_execute_args(&action.args, &staged) else {
                return Ok(());
            };
            let args = (!effective.is_empty()).then(|| semio_framework::DslValue::from(Value::Object(effective)));
            self.dispatch_action(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: action_id.to_string(), args }).await
        })
    }

    /// 🧭️ React's `RESET_DOCK` clears BOTH persisted dock layers — the arrangement override
    /// (`dockLayoutStore`) and the per-anchor chrome (`dockUiStateStore`) — so the panel dock
    /// returns to its computed default, not just the window layout.
    pub(crate) fn apply_os_command<'a>(&'a mut self, command_id: &'a str, option_value: Option<&'a str>) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            debug_log(&format!("[TRACE] wgpu-shell os command id={command_id} value={option_value:?}"));
            match command_id {
                "os.toggleFullscreen" => {
                    self.chrome.fullscreen_toggle_requested = true;
                    Ok(())
                }
                "os.openTaskManager" => {
                    self.as_session().sync_dock_tabs();
                    self.dock.reveal_dock_tab(FRAMEWORK_TASK_MANAGER_PANEL_ID);
                    Ok(())
                }
                "os.openHub" => {
                    self.opening.push_uri("/hub".to_string());
                    self.apply_shell_uri("/hub").await
                }
                "os.resetDock" => {
                    self.dock.layout_override = None;
                    self.as_session().sync_dock();
                    self.dock.dock_override = None;
                    self.dock.panel_anchors = Default::default();
                    self.dock.dock_collapsed_branches.clear();
                    self.as_session().sync_dock_tabs();
                    let app_id = self.docs.dock_store_app_id();
                    save_dock_skeleton_to_store(app_id.as_deref(), None);
                    save_dock_ui_to_store(app_id.as_deref(), None);
                    self.chrome.chrome_present.last_persisted_dock_skeleton = Some(None);
                    self.chrome.chrome_present.last_persisted_dock_ui = Some(self.dock.dock_ui_snapshot());
                    Ok(())
                }
                "os.setAppearance" | "os.setDriver" | "os.setLocale" | "os.setTerminology" | "os.setThemeId" => {
                    let Some(value) = option_value else {
                        return Ok(());
                    };
                    let action = match command_id {
                        "os.setAppearance" => "setAppearance",
                        "os.setDriver" => "setDriver",
                        "os.setLocale" => "setLocale",
                        "os.setTerminology" => "setTerminology",
                        "os.setThemeId" => "setThemeId",
                        _ => unreachable!(),
                    };
                    self.dispatch_action(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: crate::action_args_json!({ "value": value }) }).await
                }
                _ => Ok(()),
            }

        })

    }

    //#region ShellCommandHistory
    /// 🕒️ The `dispatch_action`-recursion delivery path (mechanism (a) — a `noteShellCommand`'s
    /// action id falls through every special-cased arm, including this one's own `"framework"`
    /// branch, straight to the normal plugin-forwarding tail) for the framework settings arms below.
    /// The recursion itself is exactly what the ticket calls for; `dispatch_action` is a [`ShellTurn`], boxed at its definition.
    /// No-ops without an active session: nothing to log a shell command against.
    pub(crate) async fn note_shell_setting_command(&mut self, command_id: &str, value: Option<&str>) -> Result<(), String> {
        let Some(controller_id) = self.plugins.host_controller_id() else {
            return Ok(());
        };
        let label = self.docs.shell_command_label_for_setting(command_id, &self.chrome, &self.prefs);
        let detail = value.map(|value| serde_json::json!({ "value": value }));
        let note = note_shell_command_action(&controller_id, command_id, &label, detail);
        self.dispatch_action(note).await
    }

    /// 🕒️ `handle_shell_hit`'s generic seam into `shell_command_for_control`'s `(commandId, label)`
    /// mapping — every discrete chrome-control arm that should log a history row calls this instead
    /// of hand-rolling the same `host_controller_id`/`dispatch_action` boilerplate. A silent no-op for
    /// any control id `shell_command_for_control` doesn't recognize, and without an active session
    /// (nothing to log a shell command against). Not itself inside `dispatch_action`, so a plain
    /// `.await` (no `Box::pin`) suffices here.
    pub(crate) async fn note_control_command(&mut self, control_id: &str, detail: Option<Value>) -> Result<(), String> {
        let Some((command_id, label)) = shell_command_for_control(control_id, self.prefs.locale_id == "de") else {
            return Ok(());
        };
        let Some(controller_id) = self.docs.shell_command_controller_id(&self.plugins) else {
            return Ok(());
        };
        let note = note_shell_command_action(&controller_id, command_id, &label, detail);
        self.dispatch_action(note).await
    }

    //#endregion ShellCommandHistory
}

//#endregion ActionPanelAndUtilities

