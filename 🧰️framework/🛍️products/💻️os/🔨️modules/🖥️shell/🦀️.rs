//! 🖥️ Shell UI state single source of truth. `ShellState` + `ShellCommand` + `ShellEvent` +
//! `ShellError` + the pure [`reduce`] function are the ONE place semantic shell UI state (which
//! windows exist, what is focused, active mode/tool/utility, panel and dock layout,
//! dialogs/overlays, sync/merge state, user-visible prefs) lives. The React `🐚️Shell/component.tsx`
//! reducer, the ShellHost `useState`s, and the wgpu `🐚️Shell/component.rs` struct are three
//! independent, drifting copies of this today; they become projections of this module in later
//! adoption packets (H1–H4). Nothing outside those host files can observe or drive shell state
//! today — that is exactly why the OS is not LLM-first. `shell_capabilities()` (in `🧬️schema`) is
//! what lets an MCP gateway (a later packet) advertise every `ShellCommand` variant as an
//! invocable, schema-described tool.
//!
//! This file owns ONLY the pure [`reduce`] transition and the fixture suite. Every wire type
//! (`ShellState`/`ShellCommand`/`ShellEvent`/`ShellError`/`ShellCapability` and their leaves), the
//! JSON-Schema derivation entry point for `ShellCommand`, and the Rust schema registry the
//! TypeScript mirror is rendered from live in the `🧬️schema` module below, whose `🔣️.json` is
//! the language-neutral authority; this file is a CONSUMER of them and re-exports them flat.
//!
//! Pure: no I/O, no clock (callers pass `now_ms`), no `wasm_bindgen`/`web_sys`/`winit`/`tokio`/
//! `std::thread`/`SystemTime`/`Instant::now`/`std::fs`/`std::net`. Compiles for native AND
//! `wasm32-unknown-unknown` — both the React host and wgpu-web run this crate directly.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️luna-shellstate-audit.md`
//! for the row-by-row classification this module is built from, and `📓️terra-P9-report.md` for
//! the coverage table (which audit row each variant below subsumes) and every scope decision this
//! file makes that the audit left open.

// 🧭️ Resolves relative to THIS file's own physical directory (🖥️shell/), independent of how this
// file itself got mounted into the crate from 📦️packages/🦀️rust/🦀️.rs.
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;

pub use schema::*;

//#region 🧮️reduce
fn require_non_empty(value: &str, field: &str) -> Result<(), ShellError> {
    if value.trim().is_empty() {
        Err(ShellError::EmptyIdentifier { field: field.to_string() })
    } else {
        Ok(())
    }
}

/// 🧮️ Total, pure state transition: same inputs → same outputs, never panics. `now_ms` is the
/// caller's clock reading (this crate never reads a clock itself); it is threaded through only to
/// timestamp events/notices that need a wall-clock moment (`TransientNotice::expires_at_ms`, etc.)
/// — it is the caller's responsibility to advance it monotonically. On success, `state.revision`
/// in the returned state is exactly `state.revision + 1`; on error, the input `state` is
/// unchanged (this function takes `&ShellState` and always returns a fresh value, so "unchanged"
/// is enforced by never touching the borrowed input).
pub fn reduce(state: &ShellState, command: &ShellCommand, now_ms: u64) -> Result<(ShellState, Vec<ShellEvent>), ShellError> {
    let mut next = state.clone();
    let mut events: Vec<ShellEvent> = Vec::new();
    let capability_id = capability_id_for(command);

    match command {
        ShellCommand::RegisterLoadedPlugin { plugin } => {
            require_non_empty(&plugin.plugin_id, "plugin.plugin_id")?;
            next.loaded_plugins.retain(|existing| existing.plugin_id != plugin.plugin_id);
            next.loaded_plugins.push(plugin.clone());
        }
        ShellCommand::UnregisterLoadedPlugin { plugin_id } => {
            require_non_empty(plugin_id, "plugin_id")?;
            let before = next.loaded_plugins.len();
            next.loaded_plugins.retain(|existing| &existing.plugin_id != plugin_id);
            if next.loaded_plugins.len() == before {
                return Err(ShellError::UnknownPlugin { plugin_id: plugin_id.clone() });
            }
            next.plugin_status_by_id.remove(plugin_id);
            next.plugin_supervisor_by_id.remove(plugin_id);
        }
        ShellCommand::SetPluginStatus { plugin_id, status } => {
            require_non_empty(plugin_id, "plugin_id")?;
            next.plugin_status_by_id.insert(plugin_id.clone(), status.clone());
        }
        ShellCommand::SetPluginSupervisorState { plugin_id, state: supervisor } => {
            require_non_empty(plugin_id, "plugin_id")?;
            next.plugin_supervisor_by_id.insert(plugin_id.clone(), supervisor.clone());
        }
        ShellCommand::SetActiveSession { session } => {
            next.active_session = session.clone();
        }
        ShellCommand::SetSessionError { error } => {
            next.session_error = error.clone();
        }

        ShellCommand::SetAppLabelOverride { app_id, label_key, value } => {
            require_non_empty(app_id, "app_id")?;
            require_non_empty(label_key, "label_key")?;
            let entry = next.app_labels_overlay.entry(app_id.clone()).or_default();
            match value {
                Some(v) => {
                    entry.insert(label_key.clone(), v.clone());
                }
                None => {
                    entry.remove(label_key);
                    if entry.is_empty() {
                        next.app_labels_overlay.remove(app_id);
                    }
                }
            }
        }

        ShellCommand::SetActionPaneFolded { window_id, folded } => {
            require_non_empty(window_id, "window_id")?;
            next.action_pane_folded_by_window.insert(window_id.clone(), *folded);
        }
        ShellCommand::SetActionPaneExpanded { window_id, action_id } => {
            require_non_empty(window_id, "window_id")?;
            next.action_pane_expanded_by_window.insert(window_id.clone(), action_id.clone());
        }
        ShellCommand::StageActionArg { window_id, action_id, arg_id, value } => {
            require_non_empty(window_id, "window_id")?;
            require_non_empty(action_id, "action_id")?;
            require_non_empty(arg_id, "arg_id")?;
            next.staged_action_args.entry(window_id.clone()).or_default().entry(action_id.clone()).or_default().insert(arg_id.clone(), value.clone());
        }
        ShellCommand::ResetActionArgs { window_id, action_id } => {
            require_non_empty(window_id, "window_id")?;
            require_non_empty(action_id, "action_id")?;
            if let Some(by_action) = next.staged_action_args.get_mut(window_id) {
                by_action.remove(action_id);
                if by_action.is_empty() {
                    next.staged_action_args.remove(window_id);
                }
            }
        }
        ShellCommand::SetActiveUtility { window_id, utility_id } => {
            require_non_empty(window_id, "window_id")?;
            next.active_utility_by_window.insert(window_id.clone(), utility_id.clone());
            // Mode↔tool mutual exclusion (tricky path, packet §5): a utility taking control of the
            // currently-focused window releases the global active tool.
            if utility_id.is_some() && next.active_window_id.as_deref() == Some(window_id.as_str()) && next.active_tool_id.is_some() {
                let previous = next.active_tool_id.take();
                events.push(ShellEvent::ActiveToolChanged { previous, current: None });
            }
        }
        ShellCommand::SetActiveTool { tool_id } => {
            let previous_tool = next.active_tool_id.clone();
            next.active_tool_id = tool_id.clone();
            // Mode↔tool mutual exclusion: a global tool taking control releases the focused
            // window's active utility.
            if tool_id.is_some() {
                if let Some(window_id) = next.active_window_id.clone() {
                    if let Some(slot) = next.active_utility_by_window.get_mut(&window_id) {
                        if slot.is_some() {
                            let previous = slot.take();
                            events.push(ShellEvent::ActiveUtilityChanged { window_id, previous, current: None });
                        }
                    }
                }
            }
            let _ = previous_tool;
        }

        ShellCommand::SetCommandExpanded { command_id } => {
            next.command_panel_expanded = command_id.clone();
        }
        ShellCommand::StageCommandArg { command_id, arg_id, value } => {
            require_non_empty(command_id, "command_id")?;
            require_non_empty(arg_id, "arg_id")?;
            next.staged_command_args.entry(command_id.clone()).or_default().insert(arg_id.clone(), value.clone());
        }
        ShellCommand::ResetCommandArgs { command_id } => {
            require_non_empty(command_id, "command_id")?;
            next.staged_command_args.remove(command_id);
        }

        ShellCommand::SetPanelVisible { anchor, visible } => {
            next.panels_visible.set(*anchor, *visible);
        }
        ShellCommand::SetPanelSize { anchor, size } => {
            if !size.is_finite() || *size < 0.0 {
                return Err(ShellError::InvalidPanelSize { anchor: *anchor, size: *size });
            }
            next.panels_size.set(*anchor, *size);
        }
        ShellCommand::SetPanelPath { anchor, path } => {
            next.panels_path.set(*anchor, path.clone());
        }
        ShellCommand::SetDockOverride { dock } => {
            next.dock_override = dock.clone();
        }
        ShellCommand::SetPanelPathMemory { panel_key, path } => {
            require_non_empty(panel_key, "panel_key")?;
            match path {
                Some(p) => {
                    next.panel_path_memory.insert(panel_key.clone(), p.clone());
                }
                None => {
                    next.panel_path_memory.remove(panel_key);
                }
            }
        }
        ShellCommand::SetTreeOpenState { tree_id, open } => {
            require_non_empty(tree_id, "tree_id")?;
            next.tree_open_states.insert(tree_id.clone(), *open);
        }
        ShellCommand::HydrateDockUi { dock } => {
            next.dock_override = dock.as_ref().and_then(|d| d.layout.clone());
            if let Some(d) = dock {
                next.panels_visible = d.panels_visible.clone();
            }
        }
        ShellCommand::ResetDock => {
            next.dock_override = None;
            events.push(ShellEvent::DockReset);
        }
        ShellCommand::FocusWindow { window_id } => {
            let previous = next.active_window_id.clone();
            next.active_window_id = window_id.clone();
            if previous != next.active_window_id {
                events.push(ShellEvent::WindowFocusChanged { previous, current: next.active_window_id.clone() });
            }
        }
        ShellCommand::SetShellLayout { layout } => {
            next.shell_layout = layout.clone();
        }
        ShellCommand::SetActiveExample { example_id } => {
            next.active_example_id = example_id.clone();
        }
        ShellCommand::SetMobilePanelPath { path } => {
            next.mobile_panel_path = path.clone();
        }
        ShellCommand::SetMobilePanelVisible { visible } => {
            next.mobile_panel_visible = *visible;
        }
        ShellCommand::SetExtraWindows { windows } => {
            next.extra_windows = windows.clone();
            // Focus-after-close (tricky path, packet §5): if the focused window was an extra
            // window and it is no longer present, refocus the last remaining extra window, or
            // clear focus if none remain.
            if let Some(active) = next.active_window_id.clone() {
                let was_extra = state.extra_windows.iter().any(|w| w.window_id == active);
                let still_present = next.extra_windows.iter().any(|w| w.window_id == active);
                if was_extra && !still_present {
                    let fallback = next.extra_windows.last().map(|w| w.window_id.clone());
                    next.active_window_id = fallback.clone();
                    events.push(ShellEvent::WindowFocusChanged { previous: Some(active), current: fallback });
                }
            }
        }
        ShellCommand::SetWindowTitle { window_id, title } => {
            require_non_empty(window_id, "window_id")?;
            next.window_titles_by_id.insert(window_id.clone(), title.clone());
        }
        ShellCommand::SetWindowIcon { window_id, icon } => {
            require_non_empty(window_id, "window_id")?;
            next.window_icons_by_id.insert(window_id.clone(), icon.clone());
        }

        ShellCommand::SetSearchOpen { open } => {
            next.search_open = *open;
        }
        ShellCommand::SetFindOpen { open } => {
            next.find_open = *open;
        }
        ShellCommand::AutoStartIntroduction { key } => {
            require_non_empty(key, "key")?;
            if !next.introduction_auto_started_keys.iter().any(|k| k == key) {
                next.introduction_auto_started_keys.push(key.clone());
            }
        }
        ShellCommand::SetIntroductionStep { step_index } => {
            next.introduction_step_index = *step_index;
        }
        ShellCommand::CompleteIntroductionInteraction { interaction_index } => {
            if !next.introduction_completed_interactions.contains(interaction_index) {
                next.introduction_completed_interactions.push(*interaction_index);
            }
        }
        ShellCommand::OpenDialog { dialog_id, seed_args } => {
            require_non_empty(dialog_id, "dialog_id")?;
            next.dialog_stack.push(DialogState { dialog_id: dialog_id.clone(), seed_args: seed_args.clone() });
            events.push(ShellEvent::DialogOpened { dialog_id: dialog_id.clone() });
        }
        ShellCommand::CloseDialog { dialog_id } => {
            let closed_id = match dialog_id {
                Some(id) => {
                    require_non_empty(id, "dialog_id")?;
                    let position = next.dialog_stack.iter().position(|d| &d.dialog_id == id).ok_or_else(|| ShellError::UnknownDialog { dialog_id: id.clone() })?;
                    next.dialog_stack.remove(position).dialog_id
                }
                None => next.dialog_stack.pop().ok_or_else(|| ShellError::UnknownDialog { dialog_id: String::new() })?.dialog_id,
            };
            events.push(ShellEvent::DialogClosed { dialog_id: closed_id });
        }
        ShellCommand::ShowTransientNotice { notice } => {
            next.transient_notice = Some(notice.clone());
        }
        ShellCommand::DismissTransientNotice => {
            next.transient_notice = None;
        }
        ShellCommand::SetOpenWithFocusRole { role } => {
            next.open_with_focus_role = role.clone();
        }

        ShellCommand::SetActiveTutorial { tutorial_id } => {
            next.active_tutorial_id = tutorial_id.clone();
        }

        ShellCommand::SetUiDriverDraft { draft } => {
            next.ui_driver_draft = draft.clone();
        }
        ShellCommand::SetUiThemeDraft { draft } => {
            next.ui_theme_draft = draft.clone();
        }

        ShellCommand::SetSyncBackboneUri { uri } => {
            next.sync_backbone_uri = uri.clone();
        }
        ShellCommand::SetSyncCardKind { kind } => {
            next.sync_card_kind = *kind;
        }
        ShellCommand::SetSyncDraftPath { path } => {
            next.sync_draft_path = path.clone();
        }
        ShellCommand::SetDocumentSyncStatus { document_id, status } => {
            require_non_empty(document_id, "document_id")?;
            next.sync_status_by_document.insert(document_id.clone(), status.clone());
        }

        ShellCommand::SetDocumentInferencePort { document_id, port } => {
            require_non_empty(document_id, "document_id")?;
            next.inference_port_by_document.insert(document_id.clone(), port.clone());
        }
        ShellCommand::ClearDocumentInferencePort { document_id } => {
            require_non_empty(document_id, "document_id")?;
            next.inference_port_by_document.remove(document_id);
        }

        ShellCommand::SetMergePolicy { policy } => {
            next.merge_policy = *policy;
        }
        ShellCommand::SetConflicts { conflicts } => {
            next.conflicts = conflicts.clone();
            if let Some(selected) = &next.selected_conflict_id {
                if !next.conflicts.iter().any(|c| &c.conflict_id == selected) {
                    next.selected_conflict_id = None;
                }
            }
        }
        ShellCommand::SelectConflict { conflict_id } => {
            if let Some(id) = conflict_id {
                require_non_empty(id, "conflict_id")?;
                if !next.conflicts.iter().any(|c| &c.conflict_id == id) {
                    return Err(ShellError::UnknownConflict { conflict_id: id.clone() });
                }
            }
            next.selected_conflict_id = conflict_id.clone();
        }

        ShellCommand::SetStorageScope { scope } => {
            next.storage_scope = *scope;
        }
        ShellCommand::SetOpeningPreference { role, dialect_id } => {
            require_non_empty(role, "role")?;
            match dialect_id {
                Some(id) => {
                    next.opening_preferences.insert(role.clone(), id.clone());
                }
                None => {
                    next.opening_preferences.remove(role);
                }
            }
        }
    }

    let _ = now_ms;
    next.revision = state.revision + 1;
    events.push(ShellEvent::Applied { capability_id, revision: next.revision });
    Ok((next, events))
}
//#endregion 🧮️reduce

//#region 🧪️tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️tests
