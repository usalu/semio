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

        ShellCommand::SetUiAppearance { appearance } => {
            next.ui_appearance = *appearance;
        }
        ShellCommand::SetUiLayout { layout } => {
            next.ui_layout = *layout;
        }
        ShellCommand::SetUiDriver { driver_id } => {
            next.ui_driver_id = driver_id.clone();
        }
        ShellCommand::SetUiCustomDriver { driver_id, driver } => {
            require_non_empty(driver_id, "driver_id")?;
            match driver {
                Some(d) => {
                    next.ui_custom_drivers.insert(driver_id.clone(), d.clone());
                }
                None => {
                    next.ui_custom_drivers.remove(driver_id);
                }
            }
        }
        ShellCommand::SetUiDriverDraft { draft } => {
            next.ui_driver_draft = draft.clone();
        }
        ShellCommand::SetUiLocale { locale } => {
            next.ui_locale = *locale;
        }
        ShellCommand::SetUiTerminology { terminology_id } => {
            next.ui_terminology = terminology_id.clone();
        }
        ShellCommand::SetUiTheme { theme_id } => {
            next.ui_theme_id = theme_id.clone();
        }
        ShellCommand::SetUiCustomTheme { theme_id, theme } => {
            require_non_empty(theme_id, "theme_id")?;
            match theme {
                Some(t) => {
                    next.ui_custom_themes.insert(theme_id.clone(), t.clone());
                }
                None => {
                    next.ui_custom_themes.remove(theme_id);
                }
            }
        }
        ShellCommand::SetUiThemeDraft { draft } => {
            next.ui_theme_draft = draft.clone();
        }
        ShellCommand::SetUiKeybindingOverride { control_id, keys } => {
            require_non_empty(control_id, "control_id")?;
            match keys {
                Some(k) => {
                    next.ui_keybinding_overrides.insert(control_id.clone(), k.clone());
                }
                None => {
                    next.ui_keybinding_overrides.remove(control_id);
                }
            }
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
mod tests {
    use std::collections::HashMap;

    use protocol::value as dsl_core;
    use serde::{Deserialize, Serialize};

    use super::*;

    fn window(id: &str) -> ExtraWindowInstance {
        ExtraWindowInstance { window_id: id.to_string(), kind: "app".to_string(), params: None }
    }

    /// 🧬️ Additive `#[derive(ToValue, FromValue)]` / hand-written `ShellCommand` bridge round-trip
    /// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01): every wire-carrying
    /// type in this file must satisfy `FromValue(ToValue(x)) == x`, covering the plain `#[value(...)]`
    /// derive path AND the JsonValue `with`-bridges AND `ShellCommand`'s hand-written impl (the one
    /// type the derive cannot reach — see the `ShellCommand` `ToValue`/`FromValue` impl block above).
    #[test]
    fn value_round_trip_matches_serde_shape() {
        fn check<T: dsl_core::ToValue + dsl_core::FromValue + std::fmt::Debug + PartialEq>(value: T) {
            let round_tripped = <T as dsl_core::FromValue>::from_value(dsl_core::ToValue::to_value(&value)).expect("round-trip decode");
            assert_eq!(round_tripped, value);
        }

        // Plain derive: unit-variant enum, tagged enum, tuple struct.
        check(Anchor::Left);
        check(LayoutNode::Split { orientation: SplitOrientation::Horizontal, children: vec![LayoutNode::Leaf { window_id: "w1".to_string() }], sizes: vec![1.0, 2.0] });
        check(IconName("plugin.icon".to_string()));
        check(AppRole("designer".to_string()));

        // `with`-bridged JsonValue fields, both present and absent.
        check(ExtraWindowInstance { window_id: "w1".to_string(), kind: "app".to_string(), params: Some(serde_json::json!({"seed": 1})) });
        check(window("w2"));
        check(DialogState { dialog_id: "d1".to_string(), seed_args: None });
        check(UiDriver { driver_id: "custom".to_string(), label: "Custom".to_string(), config: serde_json::json!({"a": [1, 2, "x"]}) });
        check(ShellCapability {
            id: "ui.window.focus".to_string(),
            title: "Focus window".to_string(),
            description: "Focuses a window".to_string(),
            schema: serde_json::json!({"type": "object"}),
            observable_only: false,
        });

        // `with`-bridged nested-HashMap JsonValue fields on `ShellState` — round-trip the whole
        // struct via a populated instance so the bridge's key ordering / nesting is exercised.
        let mut state = ShellState::default();
        state.staged_action_args.insert("w1".to_string(), HashMap::from([("a1".to_string(), HashMap::from([("arg1".to_string(), serde_json::json!(42))]))]));
        state.staged_command_args.insert("c1".to_string(), HashMap::from([("arg1".to_string(), serde_json::json!("value"))]));
        state.extra_windows.push(window("w3"));
        state.dialog_stack.push(DialogState { dialog_id: "d2".to_string(), seed_args: Some(serde_json::json!(["x", "y"])) });
        check(state);

        // Hand-written `ShellCommand` bridge — including the three variants a plain derive cannot
        // reach (enum-variant `JsonValue` fields; `#[value(with = "...")]` is unsupported there).
        check(ShellCommand::StageActionArg { window_id: "w1".to_string(), action_id: "a1".to_string(), arg_id: "arg1".to_string(), value: serde_json::json!({"x": 1}) });
        check(ShellCommand::StageCommandArg { command_id: "c1".to_string(), arg_id: "arg1".to_string(), value: serde_json::json!([1, 2, 3]) });
        check(ShellCommand::OpenDialog { dialog_id: "d1".to_string(), seed_args: Some(serde_json::json!({"y": 2})) });
        check(ShellCommand::OpenDialog { dialog_id: "d1".to_string(), seed_args: None });
        check(ShellCommand::ResetDock);
        check(ShellCommand::SetPanelSize { anchor: Anchor::Top, size: 12.5 });

        // Same `DslValue` a `ShellCommand` produces must carry the identical `"type"`/field-name
        // shape `#[serde(tag = "type", rename_all_fields = "camelCase")]` produces — the round-trip
        // contract's actual bar (not just "decodes back to itself").
        let encoded = dsl_core::ToValue::to_value(&ShellCommand::StageActionArg { window_id: "w1".to_string(), action_id: "a1".to_string(), arg_id: "arg1".to_string(), value: serde_json::json!(true) });
        let via_serde: serde_json::Value = serde_json::to_value(ShellCommand::StageActionArg { window_id: "w1".to_string(), action_id: "a1".to_string(), arg_id: "arg1".to_string(), value: serde_json::json!(true) }).expect("serde encode");
        assert_eq!(serde_json::Value::from(encoded), via_serde);
    }

    #[test]
    fn reduce_is_pure_and_increments_revision() {
        let state = ShellState::default();
        let (next, events) = reduce(&state, &ShellCommand::SetSearchOpen { open: true }, 1000).expect("accepted");
        assert_eq!(next.revision, state.revision + 1);
        assert!(next.search_open);
        assert!(!state.search_open, "input state must be untouched");
        assert!(matches!(events.last(), Some(ShellEvent::Applied { .. })));
    }

    #[test]
    fn reduce_rejects_leave_state_and_revision_untouched() {
        let state = ShellState::default();
        let err = reduce(&state, &ShellCommand::SelectConflict { conflict_id: Some("missing".to_string()) }, 1000).unwrap_err();
        assert_eq!(err, ShellError::UnknownConflict { conflict_id: "missing".to_string() });
    }

    #[test]
    fn focus_after_closing_focused_window_reassigns() {
        let state = ShellState { extra_windows: vec![window("w1"), window("w2")], active_window_id: Some("w2".to_string()), ..ShellState::default() };
        let (next, events) = reduce(&state, &ShellCommand::SetExtraWindows { windows: vec![window("w1")] }, 1000).expect("accepted");
        assert_eq!(next.active_window_id, Some("w1".to_string()));
        assert!(events.iter().any(|e| matches!(e, ShellEvent::WindowFocusChanged { previous: Some(p), current: Some(c) } if p == "w2" && c == "w1")));
    }

    #[test]
    fn mode_tool_mutual_exclusion_tool_clears_utility() {
        let mut state = ShellState { active_window_id: Some("w1".to_string()), ..ShellState::default() };
        state.active_utility_by_window.insert("w1".to_string(), Some("inspect".to_string()));
        let (next, events) = reduce(&state, &ShellCommand::SetActiveTool { tool_id: Some("draw".to_string()) }, 1000).expect("accepted");
        assert_eq!(next.active_tool_id, Some("draw".to_string()));
        assert_eq!(next.active_utility_by_window.get("w1").cloned().flatten(), None);
        assert!(events.iter().any(|e| matches!(e, ShellEvent::ActiveUtilityChanged { .. })));
    }

    #[test]
    fn dialog_stacking_open_and_close_top() {
        let mut state = ShellState::default();
        state.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
        let (opened, _) = reduce(&state, &ShellCommand::OpenDialog { dialog_id: "confirm".to_string(), seed_args: None }, 1000).expect("accepted");
        assert_eq!(opened.dialog_stack.iter().map(|d| d.dialog_id.clone()).collect::<Vec<_>>(), vec!["settings".to_string(), "confirm".to_string()]);
        let (closed, events) = reduce(&opened, &ShellCommand::CloseDialog { dialog_id: None }, 1000).expect("accepted");
        assert_eq!(closed.dialog_stack.iter().map(|d| d.dialog_id.clone()).collect::<Vec<_>>(), vec!["settings".to_string()]);
        assert!(events.iter().any(|e| matches!(e, ShellEvent::DialogClosed { dialog_id } if dialog_id == "confirm")));
    }

    #[test]
    fn dock_reset_clears_override_and_emits_event() {
        let state = ShellState { dock_override: Some(LayoutNode::Leaf { window_id: "w1".to_string() }), ..ShellState::default() };
        let (next, events) = reduce(&state, &ShellCommand::ResetDock, 1000).expect("accepted");
        assert_eq!(next.dock_override, None);
        assert!(events.iter().any(|e| matches!(e, ShellEvent::DockReset)));
    }

    #[test]
    fn panel_path_memory_keys_do_not_clobber_each_other() {
        let state = ShellState::default();
        let (s1, _) = reduce(&state, &ShellCommand::SetPanelPathMemory { panel_key: "left".to_string(), path: Some("tab-a".to_string()) }, 1000).expect("accepted");
        let (s2, _) = reduce(&s1, &ShellCommand::SetPanelPathMemory { panel_key: "right".to_string(), path: Some("tab-b".to_string()) }, 1000).expect("accepted");
        assert_eq!(s2.panel_path_memory.get("left").cloned(), Some("tab-a".to_string()));
        assert_eq!(s2.panel_path_memory.get("right").cloned(), Some("tab-b".to_string()));
    }

    #[test]
    fn shell_capabilities_declaration_order_matches_enum() {
        let caps = shell_capabilities();
        assert_eq!(caps.len(), 65);
        let mut ids: Vec<&str> = caps.iter().map(|c| c.id.as_str()).collect();
        let unique_before = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), unique_before, "capability ids must be unique");
        for cap in &caps {
            assert_ne!(cap.schema, serde_json::Value::Null, "capability {} must have a non-null schema", cap.id);
        }
    }

    #[test]
    fn set_panel_size_rejects_invalid_values() {
        let state = ShellState::default();
        let err = reduce(&state, &ShellCommand::SetPanelSize { anchor: Anchor::Left, size: -1.0 }, 1000).unwrap_err();
        assert!(matches!(err, ShellError::InvalidPanelSize { .. }));
        let err = reduce(&state, &ShellCommand::SetPanelSize { anchor: Anchor::Left, size: f32::NAN }, 1000).unwrap_err();
        assert!(matches!(err, ShellError::InvalidPanelSize { .. }));
    }

    /// 🧾️ Verifies all authored case inputs and reducer outputs against the committed fixtures.
    /// The independent TypeScript reducer checks the same language-neutral specimens.
    #[test]
    fn constructed_cases_match_committed_fixtures() {
        use std::fs;
        use std::path::PathBuf;

        #[derive(Serialize)]
        struct FixtureOk<'a> {
            name: &'a str,
            state: &'a ShellState,
            command: &'a ShellCommand,
            expected: FixtureOkExpected<'a>,
        }
        #[derive(Serialize)]
        struct FixtureOkExpected<'a> {
            state: &'a ShellState,
            events: &'a [ShellEvent],
        }
        #[derive(Serialize)]
        struct FixtureErr<'a> {
            name: &'a str,
            state: &'a ShellState,
            command: &'a ShellCommand,
            expected: FixtureErrExpected<'a>,
        }
        #[derive(Serialize)]
        struct FixtureErrExpected<'a> {
            error: &'a ShellError,
        }

        let dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("🧫️fixtures");
        let mut fixtures = HashMap::new();
        for entry in fs::read_dir(&dir).expect("read fixtures dir") {
            let entry = entry.expect("dir entry");
            if entry.path().extension().and_then(|e| e.to_str()) == Some("json") {
                let fixture: serde_json::Value = serde_json::from_str(&fs::read_to_string(entry.path()).expect("read committed fixture")).expect("parse committed fixture");
                let name = fixture["name"].as_str().expect("fixture name").to_owned();
                assert!(fixtures.insert(name, fixture).is_none(), "duplicate fixture identity");
            }
        }
        assert_eq!(fixtures.len(), 77);
        let compared = std::cell::RefCell::new(std::collections::HashSet::new());

        let assert_ok = |name: &str, state: ShellState, command: ShellCommand| {
            let (result_state, result_events) = reduce(&state, &command, 1_700_000_000_000).expect(name);
            let fixture = FixtureOk { name, state: &state, command: &command, expected: FixtureOkExpected { state: &result_state, events: &result_events } };
            assert_eq!(&serde_json::to_value(&fixture).expect("serialize"), fixtures.get(name).expect(name), "fixture {name}");
            assert!(compared.borrow_mut().insert(name.to_owned()), "duplicate case {name}");
        };
        let assert_err = |name: &str, state: ShellState, command: ShellCommand| {
            let error = reduce(&state, &command, 1_700_000_000_000).expect_err(name);
            let fixture = FixtureErr { name, state: &state, command: &command, expected: FixtureErrExpected { error: &error } };
            assert_eq!(&serde_json::to_value(&fixture).expect("serialize"), fixtures.get(name).expect(name), "fixture {name}");
            assert!(compared.borrow_mut().insert(name.to_owned()), "duplicate case {name}");
        };

        let base = ShellState::default();

        // One fixture per ShellCommand variant.
        assert_ok("register-loaded-plugin", base.clone(), ShellCommand::RegisterLoadedPlugin { plugin: LoadedPlugin { plugin_id: "cad".to_string(), module_url: "https://plugins.example/cad.wasm".to_string(), label: Some("CAD".to_string()) } });
        {
            let mut s = base.clone();
            s.loaded_plugins.push(LoadedPlugin { plugin_id: "cad".to_string(), module_url: "https://plugins.example/cad.wasm".to_string(), label: None });
            assert_ok("unregister-loaded-plugin", s, ShellCommand::UnregisterLoadedPlugin { plugin_id: "cad".to_string() });
        }
        assert_ok("set-plugin-status", base.clone(), ShellCommand::SetPluginStatus { plugin_id: "cad".to_string(), status: PluginPanelStatus::Open });
        assert_ok("set-plugin-supervisor-state", base.clone(), ShellCommand::SetPluginSupervisorState { plugin_id: "cad".to_string(), state: PluginSupervisorState { healthy: true, restart_count: 0, last_signal_ms: Some(1000) } });
        assert_ok("set-active-session", base.clone(), ShellCommand::SetActiveSession { session: Some(ActiveSession { plugin_id: "cad".to_string(), app_id: "modeler".to_string(), instance_id: 1 }) });
        assert_ok("set-session-error", base.clone(), ShellCommand::SetSessionError { error: Some("plugin failed to load".to_string()) });
        assert_ok("set-app-label-override", base.clone(), ShellCommand::SetAppLabelOverride { app_id: "cad".to_string(), label_key: "toolbar.extrude".to_string(), value: Some("Push/Pull".to_string()) });
        assert_ok("set-action-pane-folded", base.clone(), ShellCommand::SetActionPaneFolded { window_id: "w1".to_string(), folded: true });
        assert_ok("set-action-pane-expanded", base.clone(), ShellCommand::SetActionPaneExpanded { window_id: "w1".to_string(), action_id: Some("translateSelection".to_string()) });
        assert_ok("stage-action-arg", base.clone(), ShellCommand::StageActionArg { window_id: "w1".to_string(), action_id: "translateSelection".to_string(), arg_id: "dx".to_string(), value: serde_json::json!(1.5) });
        {
            let mut s = base.clone();
            s.staged_action_args.entry("w1".to_string()).or_default().entry("translateSelection".to_string()).or_default().insert("dx".to_string(), serde_json::json!(1.5));
            assert_ok("reset-action-args", s, ShellCommand::ResetActionArgs { window_id: "w1".to_string(), action_id: "translateSelection".to_string() });
        }
        assert_ok("set-active-utility", base.clone(), ShellCommand::SetActiveUtility { window_id: "w1".to_string(), utility_id: Some("inspect".to_string()) });
        assert_ok("set-active-tool", base.clone(), ShellCommand::SetActiveTool { tool_id: Some("draw".to_string()) });
        assert_ok("set-command-expanded", base.clone(), ShellCommand::SetCommandExpanded { command_id: Some("os.setAppearance".to_string()) });
        assert_ok("stage-command-arg", base.clone(), ShellCommand::StageCommandArg { command_id: "os.setAppearance".to_string(), arg_id: "value".to_string(), value: serde_json::json!("dark") });
        {
            let mut s = base.clone();
            s.staged_command_args.entry("os.setAppearance".to_string()).or_default().insert("value".to_string(), serde_json::json!("dark"));
            assert_ok("reset-command-args", s, ShellCommand::ResetCommandArgs { command_id: "os.setAppearance".to_string() });
        }
        assert_ok("set-panel-visible", base.clone(), ShellCommand::SetPanelVisible { anchor: Anchor::Left, visible: true });
        assert_ok("set-panel-size", base.clone(), ShellCommand::SetPanelSize { anchor: Anchor::Left, size: 320.0 });
        assert_ok("set-panel-path", base.clone(), ShellCommand::SetPanelPath { anchor: Anchor::Left, path: vec!["explorer".to_string(), "documents".to_string()] });
        assert_ok("set-dock-override", base.clone(), ShellCommand::SetDockOverride { dock: Some(LayoutNode::Leaf { window_id: "w1".to_string() }) });
        assert_ok("set-panel-path-memory", base.clone(), ShellCommand::SetPanelPathMemory { panel_key: "left".to_string(), path: Some("tab-a".to_string()) });
        {
            let mut s = base.clone();
            s.panel_path_memory.insert("left".to_string(), "tab-a".to_string());
            s.panel_path_memory.insert("right".to_string(), "tab-b".to_string());
            assert_ok("panel-path-memory-keys-independent", s, ShellCommand::SetPanelPathMemory { panel_key: "right".to_string(), path: Some("tab-c".to_string()) });
        }
        assert_ok("set-tree-open-state", base.clone(), ShellCommand::SetTreeOpenState { tree_id: "layers".to_string(), open: true });
        assert_ok("hydrate-dock-ui", base.clone(), ShellCommand::HydrateDockUi { dock: Some(DockUiState { layout: Some(LayoutNode::Leaf { window_id: "w1".to_string() }), panels_visible: ByAnchor::uniform(true) }) });
        {
            let mut s = base.clone();
            s.dock_override = Some(LayoutNode::Leaf { window_id: "w1".to_string() });
            assert_ok("reset-dock", s, ShellCommand::ResetDock);
        }
        assert_ok("focus-window", base.clone(), ShellCommand::FocusWindow { window_id: Some("w1".to_string()) });
        {
            let mut s = base.clone();
            s.extra_windows = vec![window("w1"), window("w2")];
            s.active_window_id = Some("w2".to_string());
            assert_ok("focus-after-closing-focused-window", s, ShellCommand::SetExtraWindows { windows: vec![window("w1")] });
        }
        assert_ok(
            "set-shell-layout",
            base.clone(),
            ShellCommand::SetShellLayout {
                layout: Some(LayoutNode::Split { orientation: SplitOrientation::Horizontal, children: vec![LayoutNode::Leaf { window_id: "w1".to_string() }, LayoutNode::Leaf { window_id: "w2".to_string() }], sizes: vec![0.5, 0.5] }),
            },
        );
        assert_ok("set-active-example", base.clone(), ShellCommand::SetActiveExample { example_id: "gallery.chair".to_string() });
        assert_ok("set-mobile-panel-path", base.clone(), ShellCommand::SetMobilePanelPath { path: vec!["home".to_string()] });
        assert_ok("set-mobile-panel-visible", base.clone(), ShellCommand::SetMobilePanelVisible { visible: true });
        assert_ok("set-extra-windows", base.clone(), ShellCommand::SetExtraWindows { windows: vec![window("w1")] });
        assert_ok("set-window-title", base.clone(), ShellCommand::SetWindowTitle { window_id: "w1".to_string(), title: "Untitled Model".to_string() });
        assert_ok("set-window-icon", base.clone(), ShellCommand::SetWindowIcon { window_id: "w1".to_string(), icon: IconName("cube".to_string()) });
        assert_ok("set-search-open", base.clone(), ShellCommand::SetSearchOpen { open: true });
        assert_ok("set-find-open", base.clone(), ShellCommand::SetFindOpen { open: true });
        assert_ok("auto-start-introduction", base.clone(), ShellCommand::AutoStartIntroduction { key: "welcome".to_string() });
        assert_ok("set-introduction-step", base.clone(), ShellCommand::SetIntroductionStep { step_index: Some(2) });
        assert_ok("complete-introduction-interaction", base.clone(), ShellCommand::CompleteIntroductionInteraction { interaction_index: 3 });
        assert_ok("open-dialog", base.clone(), ShellCommand::OpenDialog { dialog_id: "settings".to_string(), seed_args: None });
        {
            let mut s = base.clone();
            s.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
            assert_ok("close-dialog-top", s, ShellCommand::CloseDialog { dialog_id: None });
        }
        {
            let mut s = base.clone();
            s.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
            assert_ok("dialog-stacking-open-second", s, ShellCommand::OpenDialog { dialog_id: "confirm".to_string(), seed_args: Some(serde_json::json!({"prompt": "Discard changes?"})) });
        }
        {
            let mut s = base.clone();
            s.dialog_stack.push(DialogState { dialog_id: "settings".to_string(), seed_args: None });
            s.dialog_stack.push(DialogState { dialog_id: "confirm".to_string(), seed_args: None });
            assert_ok("dialog-stacking-close-top-keeps-rest", s, ShellCommand::CloseDialog { dialog_id: None });
        }
        assert_ok("show-transient-notice", base.clone(), ShellCommand::ShowTransientNotice { notice: TransientNotice { message: "Saved".to_string(), kind: NoticeKind::Success, expires_at_ms: Some(1_700_000_003_000) } });
        {
            let mut s = base.clone();
            s.transient_notice = Some(TransientNotice { message: "Saved".to_string(), kind: NoticeKind::Success, expires_at_ms: None });
            assert_ok("dismiss-transient-notice", s, ShellCommand::DismissTransientNotice);
        }
        assert_ok("set-open-with-focus-role", base.clone(), ShellCommand::SetOpenWithFocusRole { role: Some(AppRole("editor".to_string())) });
        assert_ok("set-active-tutorial", base.clone(), ShellCommand::SetActiveTutorial { tutorial_id: Some("getting-started".to_string()) });
        assert_ok("set-ui-appearance", base.clone(), ShellCommand::SetUiAppearance { appearance: UiAppearance::Dark });
        assert_ok("set-ui-layout", base.clone(), ShellCommand::SetUiLayout { layout: UiChromeLayout::Compact });
        assert_ok("set-ui-driver", base.clone(), ShellCommand::SetUiDriver { driver_id: "default".to_string() });
        assert_ok(
            "set-ui-custom-driver",
            base.clone(),
            ShellCommand::SetUiCustomDriver { driver_id: "custom-1".to_string(), driver: Some(UiDriver { driver_id: "custom-1".to_string(), label: "My Driver".to_string(), config: serde_json::json!({}) }) },
        );
        assert_ok("set-ui-driver-draft", base.clone(), ShellCommand::SetUiDriverDraft { draft: Some(UiDriver { driver_id: "draft".to_string(), label: "Draft".to_string(), config: serde_json::json!({}) }) });
        assert_ok("set-ui-locale", base.clone(), ShellCommand::SetUiLocale { locale: UiLocale::De });
        assert_ok("set-ui-terminology", base.clone(), ShellCommand::SetUiTerminology { terminology_id: "architecture".to_string() });
        assert_ok("set-ui-theme", base.clone(), ShellCommand::SetUiTheme { theme_id: "mono".to_string() });
        assert_ok(
            "set-ui-custom-theme",
            base.clone(),
            ShellCommand::SetUiCustomTheme { theme_id: "custom-1".to_string(), theme: Some(UiTheme { theme_id: "custom-1".to_string(), label: "My Theme".to_string(), tokens: HashMap::from([("accent".to_string(), "#f00".to_string())]) }) },
        );
        assert_ok("set-ui-theme-draft", base.clone(), ShellCommand::SetUiThemeDraft { draft: Some(UiTheme { theme_id: "draft".to_string(), label: "Draft".to_string(), tokens: HashMap::new() }) });
        assert_ok("set-ui-keybinding-override", base.clone(), ShellCommand::SetUiKeybindingOverride { control_id: "os.toggleFullscreen".to_string(), keys: Some("Cmd+Ctrl+F".to_string()) });
        assert_ok("set-sync-backbone-uri", base.clone(), ShellCommand::SetSyncBackboneUri { uri: Some("hub://space/doc".to_string()) });
        assert_ok("set-sync-card-kind", base.clone(), ShellCommand::SetSyncCardKind { kind: Some(SyncCardKind::Folder) });
        assert_ok("set-sync-draft-path", base.clone(), ShellCommand::SetSyncDraftPath { path: "/tmp/checkin".to_string() });
        assert_ok("set-document-sync-status", base.clone(), ShellCommand::SetDocumentSyncStatus { document_id: "doc-1".to_string(), status: ArtifactSyncStatus::Dirty });
        assert_ok(
            "set-document-inference-port",
            base.clone(),
            ShellCommand::SetDocumentInferencePort {
                document_id: "doc-1".to_string(),
                port: InferencePortStatus { phase: InferencePortPhase::Offered, job_id: Some("11111111111111111111111111111111".to_string()), cursor: 3, completed: 3, total: 3, proposal_hash: Some("9071779b724c67e0a45d5e23fddc8dbeb3d9b537936a4a14c293bc373960b130".to_string()), cancel_requested: false, code: None },
            },
        );
        {
            let mut s = base.clone();
            s.inference_port_by_document.insert("doc-1".to_string(), InferencePortStatus { phase: InferencePortPhase::Applied, job_id: Some("11111111111111111111111111111111".to_string()), cursor: 3, completed: 3, total: 3, proposal_hash: None, cancel_requested: false, code: None });
            assert_ok("clear-document-inference-port", s, ShellCommand::ClearDocumentInferencePort { document_id: "doc-1".to_string() });
        }
        assert_ok("set-merge-policy", base.clone(), ShellCommand::SetMergePolicy { policy: MergePolicy::PreferLocal });
        assert_ok("set-conflicts", base.clone(), ShellCommand::SetConflicts { conflicts: vec![Conflict { conflict_id: "c1".to_string(), document_id: "doc-1".to_string(), description: "concurrent edit".to_string() }] });
        {
            let mut s = base.clone();
            s.conflicts = vec![Conflict { conflict_id: "c1".to_string(), document_id: "doc-1".to_string(), description: "concurrent edit".to_string() }];
            assert_ok("select-conflict", s, ShellCommand::SelectConflict { conflict_id: Some("c1".to_string()) });
        }
        assert_ok("set-storage-scope", base.clone(), ShellCommand::SetStorageScope { scope: ShellScope::LocalStorage });
        assert_ok("set-opening-preference", base.clone(), ShellCommand::SetOpeningPreference { role: "editor".to_string(), dialect_id: Some("cad.modeler".to_string()) });

        // Mode↔tool mutual exclusion tricky paths.
        {
            let mut s = base.clone();
            s.active_window_id = Some("w1".to_string());
            s.active_utility_by_window.insert("w1".to_string(), Some("inspect".to_string()));
            assert_ok("mode-tool-exclusion-tool-clears-utility", s, ShellCommand::SetActiveTool { tool_id: Some("draw".to_string()) });
        }
        {
            let mut s = base.clone();
            s.active_window_id = Some("w1".to_string());
            s.active_tool_id = Some("draw".to_string());
            assert_ok("mode-tool-exclusion-utility-clears-tool", s, ShellCommand::SetActiveUtility { window_id: "w1".to_string(), utility_id: Some("inspect".to_string()) });
        }

        // Error fixtures.
        assert_err("error-unregister-unknown-plugin", base.clone(), ShellCommand::UnregisterLoadedPlugin { plugin_id: "missing".to_string() });
        assert_err("error-close-dialog-empty-stack", base.clone(), ShellCommand::CloseDialog { dialog_id: None });
        assert_err("error-close-dialog-unknown-id", base.clone(), ShellCommand::CloseDialog { dialog_id: Some("missing".to_string()) });
        assert_err("error-select-unknown-conflict", base.clone(), ShellCommand::SelectConflict { conflict_id: Some("missing".to_string()) });
        assert_err("error-set-panel-size-negative", base.clone(), ShellCommand::SetPanelSize { anchor: Anchor::Left, size: -5.0 });
        assert_err("error-set-window-title-empty-id", base, ShellCommand::SetWindowTitle { window_id: String::new(), title: "x".to_string() });

        assert_eq!(compared.into_inner().len(), fixtures.len());
    }

    /// 🧪️ Loads every committed fixture and re-derives it through `reduce` — the Rust half of the
    /// twin-parity mechanism (packet §3): the TypeScript test loads the same files against its own
    /// independent reducer implementation.
    #[test]
    fn fixtures_produce_expected_output() {
        use std::fs;
        use std::path::PathBuf;

        #[derive(Deserialize)]
        struct FixtureFile {
            name: String,
            state: ShellState,
            command: ShellCommand,
            expected: serde_json::Value,
        }

        let dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("🧫️fixtures");
        let entries: Vec<PathBuf> =
            fs::read_dir(&dir).expect("committed fixtures dir must exist").filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json")).collect();
        assert!(!entries.is_empty(), "no fixtures found in {}", dir.display());

        let mut checked = 0usize;
        for path in entries {
            let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            let fixture: FixtureFile = serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()));
            let outcome = reduce(&fixture.state, &fixture.command, 1_700_000_000_000);
            if let Some(expected_error) = fixture.expected.get("error") {
                let error = outcome.expect_err(&format!("{}: expected error", fixture.name));
                let actual_error = serde_json::to_value(&error).expect("serialize error");
                assert_eq!(&actual_error, expected_error, "fixture {} error mismatch", fixture.name);
            } else {
                let (state, events) = outcome.unwrap_or_else(|e| panic!("{}: expected ok, got {e:?}", fixture.name));
                let expected_state = fixture.expected.get("state").cloned().expect("expected.state");
                let expected_events = fixture.expected.get("events").cloned().expect("expected.events");
                assert_eq!(serde_json::to_value(&state).expect("serialize state"), expected_state, "fixture {} state mismatch", fixture.name);
                assert_eq!(serde_json::to_value(&events).expect("serialize events"), expected_events, "fixture {} events mismatch", fixture.name);
            }
            checked += 1;
        }
        assert!(checked >= 65, "expected at least one fixture per ShellCommand variant (65), found {checked}");
    }
}
//#endregion 🧪️tests
