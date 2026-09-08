
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
    check(ShellCapability { id: "ui.window.focus".to_string(), title: "Focus window".to_string(), description: "Focuses a window".to_string(), schema: serde_json::json!({"type": "object"}), observable_only: false });

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
    assert_ok("set-command-expanded", base.clone(), ShellCommand::SetCommandExpanded { command_id: Some("ui.driver.setDraft".to_string()) });
    assert_ok("stage-command-arg", base.clone(), ShellCommand::StageCommandArg { command_id: "ui.driver.setDraft".to_string(), arg_id: "value".to_string(), value: serde_json::json!("dark") });
    {
        let mut s = base.clone();
        s.staged_command_args.entry("ui.driver.setDraft".to_string()).or_default().insert("value".to_string(), serde_json::json!("dark"));
        assert_ok("reset-command-args", s, ShellCommand::ResetCommandArgs { command_id: "ui.driver.setDraft".to_string() });
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
    assert_ok("set-ui-driver-draft", base.clone(), ShellCommand::SetUiDriverDraft { draft: Some(UiDriver { driver_id: "draft".to_string(), label: "Draft".to_string(), config: serde_json::json!({}) }) });
    assert_ok("set-ui-theme-draft", base.clone(), ShellCommand::SetUiThemeDraft { draft: Some(UiTheme { theme_id: "draft".to_string(), label: "Draft".to_string(), config: serde_json::json!({}) }) });
    assert_ok("set-sync-backbone-uri", base.clone(), ShellCommand::SetSyncBackboneUri { uri: Some("hub://space/doc".to_string()) });
    assert_ok("set-sync-card-kind", base.clone(), ShellCommand::SetSyncCardKind { kind: Some(SyncCardKind::Folder) });
    assert_ok("set-sync-draft-path", base.clone(), ShellCommand::SetSyncDraftPath { path: "/tmp/checkin".to_string() });
    assert_ok("set-document-sync-status", base.clone(), ShellCommand::SetDocumentSyncStatus { document_id: "doc-1".to_string(), status: ArtifactSyncStatus::Dirty });
    assert_ok(
        "set-document-inference-port",
        base.clone(),
        ShellCommand::SetDocumentInferencePort {
            document_id: "doc-1".to_string(),
            port: InferencePortStatus {
                phase: InferencePortPhase::Offered,
                job_id: Some("11111111111111111111111111111111".to_string()),
                cursor: 3,
                completed: 3,
                total: 3,
                proposal_hash: Some("9071779b724c67e0a45d5e23fddc8dbeb3d9b537936a4a14c293bc373960b130".to_string()),
                cancel_requested: false,
                code: None,
            },
        },
    );
    {
        let mut s = base.clone();
        s.inference_port_by_document.insert(
            "doc-1".to_string(),
            InferencePortStatus { phase: InferencePortPhase::Applied, job_id: Some("11111111111111111111111111111111".to_string()), cursor: 3, completed: 3, total: 3, proposal_hash: None, cancel_requested: false, code: None },
        );
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
    let entries: Vec<PathBuf> = fs::read_dir(&dir).expect("committed fixtures dir must exist").filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json")).collect();
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
