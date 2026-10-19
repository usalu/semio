//! 🎯️ Dispatch action: the single funnel that every shell action crosses, `ActionsCtx::dispatch_action`. See [`super`].

use super::*;

//#region ShellActions
impl<'ctx> ActionsCtx<'ctx> {
    /** 🪪️ One action a live ENGINE SURFACE derived from raw pointer/wheel input — the world pane's
     * hover, pick, navigation note and camera report, the board's and the map's twins. Identical to
     * [`Self::dispatch_action`] except for the provenance the chrome ledger records, which is what
     * separates a gesture stream from a control press on both renderers. */

    /// 🎯️ The chrome ledger's one tap: this fn is the single funnel EVERY input's action crosses, so
    /// recording here is the target's complete answer to "what did that click do"
    /// (`crate::interpreter`'s 🎯️ChromeLedger, diagnostics-gated and bounded).
    ///
    /// 🎬️ Tutorial playback remains shell-local. Recording is armed only after the plugin's shared
    /// retained `recordTutorial` route accepts and commits below.
    ///
    /// 🎬️ Deviation detection + recorder tap — every OTHER real dispatch funnels through here exactly
    /// once, before any of this function's own side effects, and skips itself while the tutorial
    /// player's own history-action replay is mid-flight (`tutorial_flush_pending_document_ops`'s
    /// `TutorialDispatchGuard`).
    ///
    /// 🔁️ An `Effect::DispatchAction` whose id is an APP COMMAND crosses as a command, never as an
    /// action — React's `makeEffectDispatchOne` picks `handleCommand` on exactly this predicate
    /// (`🛠️ShellHelpers/🟦️.tsx`, `isAppCommand`). The guest's re-arm chain is `flowEvalTick`, an
    /// app command with no action route at all, so sending it as an `ActionInvocation` faulted the
    /// whole dispatch and — through `flush_deferred_actions` — the boot
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎨️ Backs `build_settings_theme_ui`'s theme select/reset/delete in the owned build
    /// snapshot and mirrors the value to the present-side bridge consumed by `frame()`.
    ///
    /// 🎨️ React drops the draft on every selection change (`setThemeId` dispatches
    /// `SET_UI_THEME_DRAFT: null` first) — otherwise the previous theme's unsaved
    /// edits would follow the user onto the newly picked one.
    ///
    /// 🎛️ The bottom-middle Commands panel's zero-arg rows: every os command that carries no
    /// argument fires straight through `apply_os_command`, the same entry point the ⌘️K
    /// search's `"os-command:"` redirect uses — so `os.resetDock` is reachable from the dock
    /// panel itself now, matching React's always-fireable command rows.
    ///
    /// ⚙️ The Settings General leaf's layout row — React's `setLayout` + its
    /// `os.setLayout` history note.
    ///
    /// 👁️✏️ The Default Apps leaf's per-(dialect × role) pin — `"none"` clears it, React's
    /// `clearDefault`/`setDefault` pair over one `Select`.
    ///
    /// ⌨️ Arms (or disarms) chord capture for one shortcut row — the next key event becomes
    /// that row's override instead of a shell verb.
    ///
    /// ⌨️ Drops one row's override, which restores its `SHELL_SHORTCUT_ROWS` default —
    /// React's `resetKeybindingOverride`, whose fold is a `delete` on the override map.
    ///
    /// 🎛️ A command row with arguments owns expansion only. The stable key is re-resolved
    /// against the current palette on every click so an old mode/session cannot open a stale form.
    ///
    /// 🎛️ A zero-argument tree row executes through the same resolved-command authority as
    /// staged forms. The opaque owner-qualified key is never parsed into an invocation.
    ///
    /// 📝️ Buffers one staged command argument — React's `STAGE_COMMAND_ARG`, which writes
    /// only to the buffer and dispatches nothing.
    ///
    /// 🎛️ Execute of the staged form — the one place the buffered arguments leave the buffer,
    /// React's `onExecute(expanded, effective)`.
    ///
    /// 🎛️ `apply_os_command` takes the ONE option value every os command
    /// carries (React's own single-`value` shape), so the staged form's
    /// first argument is what it receives.
    ///
    /// 📇️ The per-window Actions pane's own four verbs — React's `SET_ACTION_PANE_EXPANDED`,
    /// `STAGE_ACTION_ARG`, `RESET_ACTION_ARGS` and `onExecute` (`windowActionPaneNode`,
    /// `🛠️ShellHelpers/🟦️.tsx:4221`). Only the last one dispatches anything to the app.
    ///
    /// 🔎️ The per-window Search pane's own two local verbs — the typed line's buffer and the
    /// possibles chevron. React keeps both in the `Search` component's own state.
    ///
    /// 🔀️ One retained Projection Tree row. Both pointer Activate and Enter/Space emit
    /// this exact action, so the camera, selected row, icon, title and AX snapshot advance
    /// through one accepted per-window boundary.
    ///
    /// 💬️ The chat panel's own two verbs — React's composer `setDraft`/`submit`. There is no
    /// clear verb: the feed is live bridge state, not a local buffer a shell may discard.
    /// 🖥️ The two Display leaves' own verbs — React's `DisplayHostApi`
    /// (`📌️ChromePanels/🟦️.tsx`): the save-name box, Save current layout, apply and delete.
    ///
    /// 🖥️ React's `NamedLayoutStore::save` persists on the same call that mutates its
    /// roster — a layout saved and not written would be gone on the next boot.
    ///
    /// 🖥️ React's `NamedLayoutStore::remove` persists the shortened roster too.
    ///
    /// 🖥️ One Display "Windows" row — React drags this payload onto the dock; with no panel
    /// drag lane here the press opens the window directly, carrying the same
    /// `{windowKindId, templateId}` the drag would have.
    ///
    /// ⚔️ The Conflicts leaf's two verbs — React's `onSelect`/`onResolve`.
    ///
    /// 🛍️ The Marketplace leaf's three verbs — React's
    /// `installPlugin`/`reloadPlugin`/`uninstallPlugin`.
    ///
    /// 🔐️ Slice WG6: every `hub*` verb the hub workspace names goes through ONE lane, so
    /// this dispatch carries one arm rather than eighteen and the surface's vocabulary and
    /// the shell's cannot drift apart.
    ///
    /// ⚖️ React's `setMergePolicy` — local authority state, never wire-carried, so it moves
    /// no document and notes no history entry beyond the shell setting itself.
    ///
    /// 🎨️ The eight theme-editor verbs — React's `setThemeColor`/`setThemeSpacing`/
    /// `setThemeFontStack`/`setThemeStroke`/`setThemeRadius`/`setThemeOpacity`/
    /// `setThemeMetric`/`setThemeAppearancePaint`, each a `draftThemePatch` over the SAME
    /// document. The key rides the row's authored args (merged with the committed value by
    /// `merge_committed_args`), where React's closure captured it.
    ///
    /// 💾️ React's `saveTheme(label)`: slug the trimmed name, store the canonical document
    /// under `custom.<slug>`, select it, and drop the draft.
    ///
    /// ⬇️ React's `exportTheme`: `downloadMediaExport(`${id}.theme.dsl`, "text/plain",
    /// serializeUiTheme(theme))` — the same filename, mime and canonical 2-space JSON.
    ///
    /// ⬆️ React's `importTheme`: `requestFileOpen(".theme.dsl,.dsl,text/plain")`, parse,
    /// then save under the parsed label. The picker is user-activation gated, so it parks
    /// for the gesture lane rather than opening from this async funnel.
    ///
    /// ⬆️ The picked file's text, parsed and saved — React's
    /// `saveTheme(parsed.label || parsed.id)`. An invalid file is ignored, exactly as
    /// React's `catch {}` ignores it.
    ///
    /// 📌️ ticket §C5 item 3 — explicit check-in's own dedicated controller (`#s-checkin`'s
    /// open/cancel/submit dialog funnel), same shell-owned-chrome treatment as `framework.sync`
    /// above.
    ///
    /// 🧰️ Intercept the framework `setActiveUtility` View action to update the host-owned active-utility
    /// map before forwarding to the plugin (which reacts by clearing its live-preview scratch). The
    /// authoritative state is the shell map + the `ViewModel.active_utility_id` it injects on render.
    ///
    /// 🧾️ ticket §C5 — fold this dispatch's own `history_patch` into the check-in projection (idle
    /// clock, checkpoint-landed detection + `TouchArtifact`) before anything else touches `self`.
    ///
    /// 🎓️ Advance-by-doing: this action was actually performed (the plugin call above succeeded), so
    /// a tour step whose `advance` targets it moves on now — see `chrome_tour_note_action_performed`.
    ///
    /// 🧾️ ONE host-effect funnel, byte for byte the fold `dispatch_command` performs. This arm used
    /// to be a SECOND, shorter, hand-rolled match ending in `_ => {}`, and every effect it did not
    /// name was dropped WITHOUT A TRACE — not even the funnel's own `effect dropped` line. That is
    /// how `Export Document…` ended: the guest ran it, answered `DownloadMediaExport`, the bridge
    /// handed it back as `requested_effects`, and this silent arm threw it away, so no plugin could
    /// export anything through an ACTION on this renderer while the command path exported fine
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
    /// Only the two effects `queue_host_effects` cannot own stay here — a `Navigate` that must also
    /// RESOLVE the uri, and the replay relay — because both are `async` and that funnel is
    /// deliberately a plain `fn`.
    pub fn dispatch_action<'a>(&'a mut self, mut action: ActionDescriptor) -> ShellTurn<'a, Result<(), String>> {

        shell_turn(async move {
            self.chrome.chrome_build.refused_guest_fault = None;
            if let Some(owner) = action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str).and_then(window_pane_owner).map(str::to_string) {
                scope_action_to_window(&mut action, &owner);
            }
            semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_dispatched_action(&action, self.chrome.dispatch_origin);
            if action.action == semio_framework::onboarding::START_TUTORIAL_ACTION_ID {
                if let Some(tutorial_id) = action.args.as_ref().and_then(|args| args.get("tutorialId")).and_then(|v| v.as_str()) {
                    self.as_window_actions().tutorial_start(tutorial_id);
                }
                return Ok(());
            }
            let record_tutorial_after_acceptance = action.action == semio_framework::onboarding::RECORD_TUTORIAL_ACTION_ID;
            if !self.chrome.chrome_build.tutorial_dispatch_internal {
                self.tutorial.tutorial_note_real_dispatch(&action);
            }
            if let Some(result) = self.as_session().apply_host_panel_selection(&action) {
                return result;
            }
            if !record_tutorial_after_acceptance && action.controller_id != "framework" {
                if let Some(session) = self.docs.action_session(&action, &self.plugins).cloned() {
                    if app_owns_command(&session.app, &action.action) {
                        if action.args.as_ref().and_then(|args| args.get("windowId")).and_then(semio_framework_value::DslValue::as_str) == Some("spawned") {
                            if let Some(id) = session.view_state.window_id.as_deref() { scope_action_to_window(&mut action, id); }
                        }
                        let arguments = action.args.as_ref().and_then(semio_framework_value::DslValue::as_object).map(|entries| entries.iter().cloned().collect()).unwrap_or_default();
                        return self
                            .dispatch_session_command(semio_framework::manifest::CommandInvocation {
                                address: semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: session.plugin_id.clone(), app_id: session.app.id.clone() }, command_id: action.action.clone() },
                                arguments,
                            }, session)
                            .await;
                    }
                }
            }
            if action.controller_id == "framework" {
                match action.action.as_str() {
                    "setAppearance" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.prefs.appearance_id = value.to_string();
                            self.note_shell_setting_command("os.setAppearance", Some(value)).await?;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDriver" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.prefs.driver_id = value.to_string();
                            self.prefs.clear_driver_draft(&mut self.chrome);
                            self.note_shell_setting_command("os.setDriver", Some(value)).await?;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDriverField" => {
                        if let (Some(key), Some(value)) = (action.args.as_ref().and_then(|args| args.get("key")).and_then(|value| value.as_str()), action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str())) {
                            if self.prefs.set_driver_field(key, value, &mut self.chrome) {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setDriverSaveLabel" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
                            if self.prefs.driver_save_label != value {
                                self.prefs.driver_save_label = value.to_string();
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "saveDriver" => {
                        let label = self.prefs.driver_save_label.trim().to_string();
                        if let Some(id) = custom_ui_driver_id(&label) {
                            let mut driver = self.prefs.driver_document(&self.chrome);
                            driver.driver_id = id.clone();
                            driver.label = label;
                            self.chrome.chrome_build.preferences.custom_drivers.insert(id.clone(), driver);
                            self.prefs.driver_id = id;
                            self.prefs.driver_save_label.clear();
                            self.prefs.clear_driver_draft(&mut self.chrome);
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "deleteDriver" => {
                        if let Some(id) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).filter(|id| id.starts_with("custom.")) {
                            let changed = self.chrome.chrome_build.preferences.custom_drivers.remove(id).is_some() || self.prefs.driver_id == id || self.prefs.driver_draft.is_some();
                            if self.prefs.driver_id == id {
                                self.prefs.driver_id = "default".into();
                            }
                            self.prefs.clear_driver_draft(&mut self.chrome);
                            if changed {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setLocale" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.prefs.locale_id != value;
                            self.prefs.locale_id = value.to_string();
                            semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::install_ui_engine_locale(self.prefs.active_locale());
                            if let Some(status) = self.prefs.plugin_fault_status(&self.plugins) {
                                self.chrome.error = Some(status);
                            }
                            self.docs.trace_resolved_locale("setLocale", &self.chrome, &self.prefs);
                            self.note_shell_setting_command("os.setLocale", Some(value)).await?;
                            if changed {
                                self.as_session().sync_dock_tabs();
                                self.settle.owe_refresh(UiDirtyScope::Full);
                                self.settle.owe_settle();
                            }
                        }
                        return Ok(());
                    }
                    "setTerminology" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.prefs.terminology_id != value;
                            self.prefs.terminology_id = value.to_string();
                            self.note_shell_setting_command("os.setTerminology", Some(value)).await?;
                            if changed {
                                self.as_session().sync_dock_tabs();
                                self.settle.owe_refresh(UiDirtyScope::Full);
                                self.settle.owe_settle();
                            }
                        }
                        return Ok(());
                    }
                    "setThemeId" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let changed = self.chrome.chrome_build.preferences.theme_id != value || self.prefs.theme_draft.is_some();
                            set_active_theme_id(value);
                            self.prefs.clear_theme_draft();
                            self.chrome.chrome_build.preferences.theme_id = value.to_string();
                            self.note_shell_setting_command("os.setThemeId", Some(value)).await?;
                            if changed {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "runOsCommand" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let command_id = value.to_string();
                            self.apply_os_command(&command_id, None).await?;
                        }
                        return Ok(());
                    }
                    "setLayout" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.chrome.chrome_build.preferences.ui_layout = if value == "tablet" { "tablet".to_string() } else { "desktop".to_string() };
                            self.note_shell_setting_command("os.setLayout", Some(value)).await?;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setDefaultApp" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let key = action.args.as_ref().and_then(|args| args.get("row")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                            if value == DEFAULT_APP_NONE_VALUE {
                                self.plugins.default_app_pins.remove(&key);
                            } else {
                                self.plugins.default_app_pins.insert(key, value.to_string());
                            }
                        }
                        return Ok(());
                    }
                    "captureKeybinding" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            let already = self.input.keybinding_capture_control_id.as_deref() == Some(value);
                            self.input.keybinding_capture_control_id = (!already).then(|| value.to_string());
                        }
                        return Ok(());
                    }
                    "resetKeybinding" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.chrome.chrome_build.preferences.keybinding_overrides.remove(value);
                        }
                        return Ok(());
                    }
                    "setCommandExpanded" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()) {
                            let live = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().any(|entry| entry.definition.in_palette && !entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key);
                            if live {
                                self.commands.expanded_command_id = (self.commands.expanded_command_id.as_deref() != Some(key)).then(|| key.to_string());
                            }
                        }
                        return Ok(());
                    }
                    "executeResolvedCommand" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            if let Some(entry) = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().find(|entry| entry.definition.in_palette && entry.definition.args.is_empty() && command_address_stable_key(&entry.address) == key) {
                                match &entry.address.owner {
                                    semio_framework::manifest::CommandOwnerAddress::Os => {
                                        self.apply_os_command(&entry.definition.id, None).await?;
                                    }
                                    _ => {
                                        self.dispatch_command(semio_framework::manifest::CommandInvocation { address: entry.address, arguments: Default::default() }).await?;
                                    }
                                }
                            }
                        }
                        return Ok(());
                    }
                    "stageCommandArg" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(command), Some(arg)) = (args.get("command").and_then(Value::as_str), args.get("arg").and_then(Value::as_str)) {
                            let staged = self.commands.staged_command_args.entry(command.to_string()).or_default();
                            let value = staged_arg_value(&args, staged.get(arg));
                            staged.insert(arg.to_string(), value);
                        }
                        return Ok(());
                    }
                    "resetStagedCommand" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.commands.staged_command_args.remove(value);
                        }
                        return Ok(());
                    }
                    "executeStagedCommand" => {
                        if let Some(key) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            let staged = self.commands.staged_command_args.get(&key).cloned().unwrap_or_default();
                            if let Some(entry) = self.docs.resolved_commands(&self.chrome, &self.plugins).into_iter().find(|entry| entry.definition.in_palette && command_address_stable_key(&entry.address) == key) {
                                let Some(effective) = resolved_execute_args(&entry.definition.args, &staged) else { return Ok(()) };
                                match &entry.address.owner {
                                    semio_framework::manifest::CommandOwnerAddress::Os => {
                                        let command_id = entry.definition.id.clone();
                                        let option = entry.definition.args.first().and_then(|arg| effective.get(&arg.id)).and_then(|value| value.as_str().map(ToOwned::to_owned));
                                        self.apply_os_command(&command_id, option.as_deref()).await?;
                                    }
                                    _ => {
                                        let arguments = effective.into_iter().filter_map(|(id, value)| serde_json::from_value::<DslValue>(value).ok().map(|value| (id, value))).collect();
                                        self.dispatch_command(semio_framework::manifest::CommandInvocation { address: entry.address.clone(), arguments }).await?;
                                    }
                                }
                            }
                            self.commands.staged_command_args.remove(&key);
                            self.commands.expanded_command_id = None;
                        }
                        return Ok(());
                    }
                    "setActionExpanded" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            if self.windows.action_panel_expanded.get(window_id).map(String::as_str) == Some(action_id) {
                                self.windows.action_panel_expanded.remove(window_id);
                            } else {
                                self.windows.action_panel_expanded.insert(window_id.to_string(), action_id.to_string());
                            }
                        }
                        return self.as_window_actions().republish_window_action_panes();
                    }
                    IMPORT_TRANSFER_CANCEL_ACTION => {
                        if let Some(transfer) = action.args.as_ref().map(dsl_value_as_json).and_then(|args| args.get("transfer").and_then(Value::as_u64)) {
                            self.as_session().cancel_import_transfer(transfer);
                        }
                        return Ok(());
                    }
                    "stageActionArg" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id), Some(arg_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str), args.get("arg").and_then(Value::as_str)) {
                            let (window_id, action_id, arg_id) = (window_id.to_string(), action_id.to_string(), arg_id.to_string());
                            let value = staged_arg_value(&args, self.commands.staged_map_for(&window_id, &action_id).get(&arg_id));
                            self.commands.stage_arg(&window_id, &action_id, &arg_id, value);
                        }
                        return self.as_window_actions().republish_window_action_panes();
                    }
                    "resetActionArgs" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
                            self.commands.reset_staged_args(&window_id, &action_id);
                        }
                        return self.as_window_actions().republish_window_action_panes();
                    }
                    "executeStagedAction" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(action_id)) = (args.get("window").and_then(Value::as_str), args.get("action").and_then(Value::as_str)) {
                            let (window_id, action_id) = (window_id.to_string(), action_id.to_string());
                            self.execute_staged_action(&window_id, &action_id).await?;
                        }
                        return Ok(());
                    }
                    "setEngagementInput" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let Some(window_id) = args.get("window").and_then(Value::as_str) {
                            let value = args.get("value").and_then(Value::as_str).unwrap_or_default().to_string();
                            self.windows.engagement_inputs.insert(window_id.to_string(), value);
                        }
                        return Ok(());
                    }
                    "toggleSearchPossibles" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let Some(window_id) = args.get("window").and_then(Value::as_str) {
                            let open = self.commands.search_possibles_open.get(window_id).copied().unwrap_or(false);
                            self.commands.search_possibles_open.insert(window_id.to_string(), !open);
                        }
                        return self.as_window_actions().republish_window_action_panes();
                    }
                    "setWorldProjectionTemplate" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        if let (Some(window_id), Some(template_id)) = (args.get("windowId").and_then(Value::as_str), args.get("templateId").and_then(Value::as_str)) {
                            let (window_id, template_id) = (window_id.to_string(), template_id.to_string());
                            self.dock.select_world_projection_template(&window_id, &template_id, &mut self.surfaces, &mut self.docs, &mut self.windows)?;
                        }
                        return Ok(());
                    }
                    "setLayoutSaveLabel" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.dock.layout_save_label = value.to_string();
                        }
                        return Ok(());
                    }
                    "saveCurrentLayout" => {
                        let label = self.dock.layout_save_label.trim().to_string();
                        if label.is_empty() {
                            return Ok(());
                        }
                        if let Some(layout) = self.dock.current_named_layout(&label, &self.docs) {
                            let id = layout.id.clone();
                            self.dock.user_layouts.retain(|entry| entry.id != id);
                            self.dock.user_layouts.push(layout);
                            self.dock.layout_save_label.clear();
                            self.dock.persist_named_layouts(&self.docs);
                            self.note_shell_setting_command("os.saveNamedLayout", Some(&id)).await?;
                        }
                        return Ok(());
                    }
                    "applyNamedLayout" => {
                        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if self.as_session().apply_named_layout(&layout_id) {
                                self.note_shell_setting_command("os.applyNamedLayout", Some(&layout_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "deleteUserLayout" => {
                        if let Some(layout_id) = action.args.as_ref().and_then(|args| args.get("layoutId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            self.dock.user_layouts.retain(|entry| entry.id != layout_id);
                            self.dock.persist_named_layouts(&self.docs);
                            self.note_shell_setting_command("os.deleteNamedLayout", Some(&layout_id)).await?;
                        }
                        return Ok(());
                    }
                    "openDisplayWindow" => {
                        let window_kind_id = action.args.as_ref().and_then(|args| args.get("windowKindId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        let template_id = action.args.as_ref().and_then(|args| args.get("templateId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        if let Some(window_kind_id) = window_kind_id {
                            self.dock.open_display_window(&window_kind_id, template_id.as_deref(), &mut self.settle, &self.docs, &mut self.windows);
                        }
                        return Ok(());
                    }
                    "selectConflict" => {
                        if let Some(conflict_id) = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()) {
                            self.prefs.selected_conflict_id = (self.prefs.selected_conflict_id.as_deref() != Some(conflict_id)).then(|| conflict_id.to_string());
                        }
                        return Ok(());
                    }
                    "resolveConflict" => {
                        let conflict_id = action.args.as_ref().and_then(|args| args.get("conflictId")).and_then(|v| v.as_str()).map(ToOwned::to_owned);
                        let accept = action.args.as_ref().and_then(|args| args.get("accept")).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true);
                        if let Some(conflict_id) = conflict_id {
                            self.docs.resolve_open_conflict(&conflict_id, accept, &mut self.chrome, &self.plugins, &mut self.prefs).await;
                        }
                        return Ok(());
                    }
                    "installPlugin" | "reloadPlugin" => {
                        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if action.action == "reloadPlugin" {
                                self.docs.uninstall_plugin(&plugin_id, &mut self.plugins, &mut self.directory);
                            }
                            let _ = self.plugins.install_plugin(&plugin_id).await;
                            self.note_shell_setting_command(if action.action == "reloadPlugin" { "os.reloadPlugin" } else { "os.installPlugin" }, Some(&plugin_id)).await?;
                        }
                        return Ok(());
                    }
                    "uninstallPlugin" => {
                        if let Some(plugin_id) = action.args.as_ref().and_then(|args| args.get("pluginId")).and_then(|v| v.as_str()).map(ToOwned::to_owned) {
                            if self.docs.uninstall_plugin(&plugin_id, &mut self.plugins, &mut self.directory) {
                                self.note_shell_setting_command("os.uninstallPlugin", Some(&plugin_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "installExtensionUrl" => {
                        let url = action.args.as_ref().and_then(|args| args.get("url")).and_then(|value| value.as_str()).filter(|url| !url.is_empty()).map(ToOwned::to_owned);
                        let mut request = serde_json::json!({
                            "op": "extension-store-install-url",
                            "prompt": shell_chrome_string("plugins.extension.urlPrompt", self.prefs.locale_id == "de")
                        });
                        if let Some(url) = url.as_deref() {
                            request["url"] = serde_json::Value::String(url.to_string());
                        }
                        if self.install_extension_from_store(request).await? {
                            self.note_shell_setting_command("os.installExtensionUrl", url.as_deref()).await?;
                        }
                        return Ok(());
                    }
                    "installExtensionFile" => {
                        if self.install_extension_from_store(serde_json::json!({ "op": "extension-store-install-file" })).await? {
                            self.note_shell_setting_command("os.installExtensionFile", None).await?;
                        }
                        return Ok(());
                    }
                    "setExtensionEnabled" => {
                        let extension_id = action.args.as_ref().and_then(|args| args.get("extensionId")).and_then(|value| value.as_str()).map(ToOwned::to_owned);
                        let enabled = action.args.as_ref().and_then(|args| args.get("enabled")).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(true);
                        if let Some(extension_id) = extension_id {
                            if self.set_extension_enabled(&extension_id, enabled).await? {
                                self.note_shell_setting_command(if enabled { "os.enableExtension" } else { "os.disableExtension" }, Some(&extension_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "uninstallExtension" => {
                        if let Some(extension_id) = action.args.as_ref().and_then(|args| args.get("extensionId")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            if self.uninstall_extension(&extension_id).await? {
                                self.note_shell_setting_command("os.uninstallExtension", Some(&extension_id)).await?;
                            }
                        }
                        return Ok(());
                    }
                    "setChatDraft" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            self.directory.agent_chat_draft = value.to_string();
                        }
                        return Ok(());
                    }
                    "sendChatDraft" => {
                        self.directory.send_agent_chat_draft(&mut self.chrome);
                        return Ok(());
                    }
                    "cancelToolCall" => {
                        if let Some(invocation_id) = action.args.as_ref().and_then(|args| args.get("invocationId")).and_then(|value| value.as_str()).map(ToOwned::to_owned) {
                            self.chrome.cancel_agent_tool_call(&invocation_id);
                        }
                        return Ok(());
                    }
                    other if other.starts_with("hub") => {
                        self.handle_hub_workspace_action(&action.action.clone(), action.args.clone()).await;
                        return Ok(());
                    }
                    "setMergePolicy" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            if SHELL_MERGE_POLICY_OPTIONS.iter().any(|(option, _)| *option == value) {
                                self.prefs.merge_policy = value.to_string();
                                self.note_shell_setting_command("os.setMergePolicy", Some(value)).await?;
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_GENERAL_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "setThemeColor" | "setThemeSpacing" | "setThemeFontStack" | "setThemeStroke" | "setThemeRadius" | "setThemeOpacity" | "setThemeMetric" | "setThemeAppearancePaint" => {
                        let args = action.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                        let verb = action.action.clone();
                        let before = self.prefs.theme_document(&self.chrome);
                        self.prefs.apply_theme_editor_commit(&verb, &args, &self.chrome);
                        if self.prefs.theme_document(&self.chrome) != before {
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "setThemeSaveLabel" => {
                        let value = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                        if self.prefs.theme_save_label != value {
                            self.prefs.theme_save_label = value;
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "saveTheme" => {
                        let label = self.prefs.theme_save_label.trim().to_string();
                        if let Some(id) = custom_theme_id_for_label(&label) {
                            let mut document = self.prefs.theme_document(&self.chrome);
                            document.id = id.clone();
                            document.label = label;
                            if let Some(raw) = save_custom_theme_document(&id, &document) {
                                self.chrome.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                                self.chrome.chrome_build.preferences.theme_id = id.clone();
                                self.prefs.theme_draft = None;
                                self.prefs.theme_save_label.clear();
                                self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    "exportTheme" => {
                        let document = self.prefs.theme_document(&self.chrome);
                        if let Some(text) = document.canonical_json() {
                            download_media_export(&format!("{}.theme.dsl", document.id), "text/plain", &text, None);
                        }
                        return Ok(());
                    }
                    "importTheme" => {
                        #[cfg(target_arch = "wasm32")]
                        {
                            self.prefs.pending_theme_import = true;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        self.settle.submit_shell_io_future(async move {
                            let opened = request_file_open(SHELL_THEME_IMPORT_ACCEPT, Some("text"), false).await;
                            ShellIoCompletion::Actions(
                                opened.into_iter().take(1).map(|file| ActionDescriptor { controller_id: "framework".into(), action: "applyImportedTheme".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "value": file.contents }) }).collect(),
                            )
                        });
                        return Ok(());
                    }
                    "applyImportedTheme" => {
                        if let Some(text) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            if let Some(document) = ThemeDocument::parse(text) {
                                let label = if document.label.is_empty() { document.id.clone() } else { document.label.clone() };
                                if let Some(id) = custom_theme_id_for_label(&label) {
                                    let saved = ThemeDocument { id: id.clone(), label, ..document };
                                    if let Some(raw) = save_custom_theme_document(&id, &saved) {
                                        self.chrome.chrome_build.preferences.custom_themes.insert(id.clone(), raw);
                                        self.chrome.chrome_build.preferences.theme_id = id.clone();
                                        self.prefs.theme_draft = None;
                                        self.note_shell_setting_command("os.setThemeId", Some(id.as_str())).await?;
                                        self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                                    }
                                }
                            }
                        }
                        return Ok(());
                    }
                    "resetThemeId" => {
                        let changed = self.chrome.chrome_build.preferences.theme_id != "semio" || self.prefs.theme_draft.is_some();
                        set_active_theme_id("semio");
                        self.prefs.clear_theme_draft();
                        self.chrome.chrome_build.preferences.theme_id = "semio".to_string();
                        self.note_shell_setting_command("os.resetThemeId", Some("semio")).await?;
                        if changed {
                            self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                        }
                        return Ok(());
                    }
                    "deleteThemeId" => {
                        if let Some(value) = action.args.as_ref().and_then(|args| args.get("value")).and_then(|v| v.as_str()) {
                            delete_custom_theme(value);
                            let removed = self.chrome.chrome_build.preferences.custom_themes.remove(value).is_some();
                            let selected = self.chrome.chrome_build.preferences.theme_id == value;
                            if selected {
                                self.chrome.chrome_build.preferences.theme_id = "semio".to_string();
                            }
                            self.note_shell_setting_command("os.deleteThemeId", Some(value)).await?;
                            if removed || selected {
                                self.as_session().republish_shell_panel_document(FRAMEWORK_SETTINGS_THEME_TAB_ID)?;
                            }
                        }
                        return Ok(());
                    }
                    _ => {}
                }
            }
            if action.controller_id == "framework.sync" {
                self.handle_sync_action(action).await?;
                return self.as_session().republish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID);
            }
            if action.controller_id == "framework.checkin" {
                return self.handle_checkin_action(action).await;
            }
            if action.action == semio_framework::SET_ACTIVE_UTILITY_ACTION_ID {
                if let Some(session) = self.docs.session.clone() {
                    if action.controller_id == session.app.controller_id {
                        if let Some(utility_id) = action.args.as_ref().and_then(|args| args.get("utilityId")).and_then(|value| value.as_str()) {
                            let window_id = action
                                .args
                                .as_ref()
                                .and_then(|args| args.get("windowId"))
                                .and_then(|value| value.as_str())
                                .map(String::from)
                                .or_else(|| self.windows.active_window_id.clone())
                                .unwrap_or_else(|| self.dock.active_utility_bar_window_kind(&session, &self.windows).id.clone());
                            self.windows.toggle_active_utility(&window_id, utility_id, &mut self.chrome, &mut self.docs);
                        }
                    }
                }
            }
            let Some(session) = self.docs.action_session(&action, &self.plugins).cloned() else {
                return Err("action has no mounted app owner".into());
            };
            let program = self.plugins.plugins.iter().find(|p| p.plugin_id == session.plugin_id).ok_or("action program missing")?;
            let live_view_state = self.as_session().live_view_state(&session);
            let invocation = self.prefs.session_action_invocation(&session, &action, &live_view_state)?;
            let action_json = semio_framework_pack_json::to_json_string(&invocation);
            let answer = program.handle_action(session.instance_id, &action_json, &live_view_state).await;
            let mut result = answer.map_err(|refusal| self.prefs.refused_guest_call(refusal, &session.app, &mut self.chrome))?;
            self.observe_invocation_history(result.history_patch.as_ref()).await;
            self.input.observe_interaction_output(&result.output);
            if record_tutorial_after_acceptance {
                self.as_window_actions().tutorial_start_recording();
            }
            self.docs.chrome_tour_note_action_performed(&action.action, &mut self.chrome);
            let mut queued = Vec::with_capacity(result.requested_effects.len());
            for effect in core::mem::take(&mut result.requested_effects) {
                match effect {
                    semio_framework::kernel::Effect::Navigate { uri } => {
                        self.opening.push_uri(uri.clone());
                        if let Err(error) = self.apply_shell_uri(&uri).await {
                            debug_log(&format!("[TRACE] wgpu shell navigate effect failed: {error}"));
                        }
                    }
                    semio_framework::kernel::Effect::ReplayShellCommand { action_id, args } => {
                        self.handle_replay_shell_command(&action_id, args.as_ref()).await;
                    }
                    other => queued.push(other),
                }
            }
            let scope = result.ui_scope.clone().merged_with(self.docs.host_effect_earned_scope(&queued));
            debug_log(&format!("[TRACE] wgpu-shell dispatch action={} scope={}", action.action, refresh_scope_label(&scope)));
            self.as_window_actions().queue_host_effects(&action.controller_id.clone(), queued);
            let operations: Vec<String> = result.mutations.iter().filter_map(|operation| serde_json::to_string(&operation.diff.payload).ok()).collect();
            self.apply_ops_inner(&operations, true, scope, Some(session)).await

        })

    }
}
//#endregion ShellActions
