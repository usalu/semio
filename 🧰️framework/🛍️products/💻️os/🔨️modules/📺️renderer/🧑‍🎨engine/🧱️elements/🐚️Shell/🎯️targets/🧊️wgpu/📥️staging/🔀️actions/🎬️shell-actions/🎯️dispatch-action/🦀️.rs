//! 🎯️ Dispatch action: the single funnel that every shell action crosses, `ActionsCtx::dispatch_action`. See [`super`].

use super::*;

#[path = "🎛️settings/🦀️.rs"]
mod settings;

#[path = "🎨️theme-actions/🦀️.rs"]
mod theme_actions;

#[path = "⌨️staging/🦀️.rs"]
mod staging;

#[path = "🪟️layouts/🦀️.rs"]
mod layouts;

#[path = "🧩️plugins-chat/🦀️.rs"]
mod plugins_chat;

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
                    "setAppearance" => return self.framework_set_appearance(&action).await,
                    "setDriver" => return self.framework_set_driver(&action).await,
                    "setDriverField" => return self.framework_set_driver_field(&action).await,
                    "setDriverSaveLabel" => return self.framework_set_driver_save_label(&action).await,
                    "saveDriver" => return self.framework_save_driver().await,
                    "deleteDriver" => return self.framework_delete_driver(&action).await,
                    "setLocale" => return self.framework_set_locale(&action).await,
                    "setTerminology" => return self.framework_set_terminology(&action).await,
                    "setThemeId" => return self.framework_set_theme_id(&action).await,
                    "runOsCommand" => return self.framework_run_os_command(&action).await,
                    "setLayout" => return self.framework_set_layout(&action).await,
                    "setDefaultApp" => return self.framework_set_default_app(&action).await,
                    "captureKeybinding" => return self.framework_capture_keybinding(&action).await,
                    "resetKeybinding" => return self.framework_reset_keybinding(&action).await,
                    "setCommandExpanded" => return self.framework_set_command_expanded(&action).await,
                    "executeResolvedCommand" => return self.framework_execute_resolved_command(&action).await,
                    "stageCommandArg" => return self.framework_stage_command_arg(&action).await,
                    "resetStagedCommand" => return self.framework_reset_staged_command(&action).await,
                    "executeStagedCommand" => return self.framework_execute_staged_command(&action).await,
                    "setActionExpanded" => return self.framework_set_action_expanded(&action).await,
                    IMPORT_TRANSFER_CANCEL_ACTION => {
                        if let Some(transfer) = action.args.as_ref().map(dsl_value_as_json).and_then(|args| args.get("transfer").and_then(Value::as_u64)) {
                            self.as_session().cancel_import_transfer(transfer);
                        }
                        return Ok(());
                    }
                    "stageActionArg" => return self.framework_stage_action_arg(&action).await,
                    "resetActionArgs" => return self.framework_reset_action_args(&action).await,
                    "executeStagedAction" => return self.framework_execute_staged_action(&action).await,
                    "setEngagementInput" => return self.framework_set_engagement_input(&action).await,
                    "toggleSearchPossibles" => return self.framework_toggle_search_possibles(&action).await,
                    "setWorldProjectionTemplate" => return self.framework_set_world_projection_template(&action).await,
                    "setLayoutSaveLabel" => return self.framework_set_layout_save_label(&action).await,
                    "saveCurrentLayout" => return self.framework_save_current_layout().await,
                    "applyNamedLayout" => return self.framework_apply_named_layout(&action).await,
                    "deleteUserLayout" => return self.framework_delete_user_layout(&action).await,
                    "openDisplayWindow" => return self.framework_open_display_window(&action).await,
                    "selectConflict" => return self.framework_select_conflict(&action).await,
                    "resolveConflict" => return self.framework_resolve_conflict(&action).await,
                    "installPlugin" | "reloadPlugin" => return self.framework_install_or_reload_plugin(&action).await,
                    "uninstallPlugin" => return self.framework_uninstall_plugin(&action).await,
                    "installExtensionUrl" => return self.framework_install_extension_url(&action).await,
                    "installExtensionFile" => return self.framework_install_extension_file().await,
                    "setExtensionEnabled" => return self.framework_set_extension_enabled(&action).await,
                    "uninstallExtension" => return self.framework_uninstall_extension(&action).await,
                    "setChatDraft" => return self.framework_set_chat_draft(&action).await,
                    "sendChatDraft" => return self.framework_send_chat_draft().await,
                    "cancelToolCall" => return self.framework_cancel_tool_call(&action).await,
                    other if other.starts_with("hub") => {
                        self.handle_hub_workspace_action(&action.action.clone(), action.args.clone()).await;
                        return Ok(());
                    }
                    "setMergePolicy" => return self.framework_set_merge_policy(&action).await,
                    "setThemeColor" | "setThemeSpacing" | "setThemeFontStack" | "setThemeStroke" | "setThemeRadius" | "setThemeOpacity" | "setThemeMetric" | "setThemeAppearancePaint" => return self.framework_set_theme_token(&action).await,
                    "setThemeSaveLabel" => return self.framework_set_theme_save_label(&action).await,
                    "saveTheme" => return self.framework_save_theme().await,
                    "exportTheme" => return self.framework_export_theme().await,
                    "importTheme" => return self.framework_import_theme(&action).await,
                    "applyImportedTheme" => return self.framework_apply_imported_theme(&action).await,
                    "resetThemeId" => return self.framework_reset_theme_id().await,
                    "deleteThemeId" => return self.framework_delete_theme_id(&action).await,
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
