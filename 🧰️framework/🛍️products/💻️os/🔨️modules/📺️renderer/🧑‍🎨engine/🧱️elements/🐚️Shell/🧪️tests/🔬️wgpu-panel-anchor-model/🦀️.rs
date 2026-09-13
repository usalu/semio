
use super::*;
// 🩹️ `PanelTabKind` stopped being reachable through the shell module's own `use` list, and the
// `📌️panel-state` fixture moved from `🧑‍🎨engine/🧫️fixtures/` into this element's own `🧫️fixtures/`
// (panel-tab lane, 2026-09-13) — both are re-pointed here so the crate's test target builds again.
use semio_framework::PanelTabKind;

fn fresh_state() -> ShellState {
    // 🧪️ `ShellState::new` calls `load_persisted_panel_layout`, which — on native — reads whatever
    // happens to be at `~/.semio/panel-layout.json` on the machine running the test. Every assertion
    // below explicitly sets the fields it exercises afterward, so the outcome never depends on that.
    ShellState::new(Vec::new(), String::new())
}

fn host_test_apps() -> (AppDefinition, AppDefinition) {
    let mut home = super::command_registry_tests::test_app(Vec::new(), Vec::new());
    home.id = "home".into();
    home.controller_id = "space.home".into();
    home.panel_tabs = vec![PanelTabDefinition { kind: PanelTabKind::App("home-library".into()), label: LocalizedLabel::data("Home Library"), group: PanelGroup::Display, body_key: Some("home.library".into()), children: vec![] }];
    let mut studio = super::command_registry_tests::test_app(Vec::new(), Vec::new());
    studio.id = "studio".into();
    studio.controller_id = "space.studio".into();
    studio.panel_tabs = vec![
        PanelTabDefinition { kind: PanelTabKind::App("s-play-catalogue".into()), label: LocalizedLabel::data("Catalogue"), group: PanelGroup::Workbench, body_key: Some("s.play.catalogue".into()), children: vec![] },
        PanelTabDefinition { kind: PanelTabKind::App("s-play-inspector".into()), label: LocalizedLabel::data("Inspector"), group: PanelGroup::Details, body_key: Some("s.play.inspector".into()), children: vec![] },
        PanelTabDefinition {
            kind: PanelTabKind::App("studio-settings".into()),
            label: LocalizedLabel::data("Studio Settings"),
            group: PanelGroup::Settings,
            body_key: None,
            children: vec![PanelTabDefinition { kind: PanelTabKind::App("studio-settings-leaf".into()), label: LocalizedLabel::data("Settings"), group: PanelGroup::Settings, body_key: Some("studio.settings".into()), children: vec![] }],
        },
    ];
    (home, studio)
}

fn host_test_shell() -> ShellState {
    let (home, studio) = host_test_apps();
    let manifest = semio_framework::PluginManifest {
        plugin_id: "space".into(),
        label: "Space".into(),
        version: "1".into(),
        apps: vec![home, studio.clone()],
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![],
        artifact_kinds: vec![],
        dependencies: vec![],
        contributions: vec![],
    };
    let bridge = ProgramBridgeEntry::from_wasm("space".into(), None, std::path::PathBuf::from("missing-host-panel-guest.wasm"), manifest).expect("nonrunnable host bridge");
    let mut shell = ShellState::new(vec![bridge], "space".into());
    let base = SpacePanelState {
        active_panel_tab: "s-play-catalogue".into(),
        spawned_apps: vec![
            SpawnedAppEntry { id: "spawned-model".into(), plugin_id: "model".into(), instance_id: 41, app_id: "model@1/any#editor".into(), label: "Model".into(), breadcrumb: vec!["Workspace".into(), "Model".into()] },
            SpawnedAppEntry { id: "spawned-note".into(), plugin_id: "note".into(), instance_id: 42, app_id: "note@1/any#editor".into(), label: "Note".into(), breadcrumb: vec!["Workspace".into(), "Note".into()] },
        ],
        active_spawned_id: Some("spawned-model".into()),
    };
    let mut view_state = ViewModel::default();
    view_state.active_mode_id = Some(studio.default_mode_id.clone());
    view_state.active_window_kind_id = Some(studio.window_kinds.first().id.clone());
    view_state.panel_json = Some(ShellState::panel_json(&base).expect("strict base panel"));
    shell.session = Some(ActiveSession { plugin_id: "space".into(), instance_id: 77, app: studio, view_state });
    shell
}

#[test]
fn host_panel_json_codec_matches_the_neutral_fixture_and_rejects_invalid_carriage() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/📌️panel-state/🔣️.json")).expect("panel fixture");
    let base: SpacePanelState = serde_json::from_value(fixture["base"].clone()).expect("fixture base");
    let encoded = ShellState::panel_json(&base).expect("strict panel JSON");
    assert!(encoded.starts_with('{'));
    let mut view = ViewModel::default();
    view.panel_json = Some(encoded.clone());
    assert_eq!(ShellState::panel_state_from_view(&view).expect("strict decode"), Some(base.clone()));
    view.panel_json = Some(serde_json::json!({ "activePanelTab": "s-play-catalogue", "spawnedApps": [], "programs": [] }).to_string());
    assert_eq!(ShellState::panel_state_from_view(&view).unwrap_err(), "host-panel.invalid-json");
    let mut duplicate = base.clone();
    duplicate.spawned_apps.push(base.spawned_apps[0].clone());
    assert_eq!(ShellState::panel_json(&duplicate).unwrap_err(), "host-panel.duplicate-spawned-identity");
    let mut missing_focus = base;
    missing_focus.active_spawned_id = Some("spawned-missing".into());
    assert_eq!(ShellState::panel_json(&missing_focus).unwrap_err(), "host-panel.invalid-active-spawned");
}

#[test]
fn host_panel_action_is_claimed_before_guest_and_preserves_the_session_roster_and_home_projection() {
    let mut shell = host_test_shell();
    let before = ShellState::panel_state_from_view(&shell.session.as_ref().unwrap().view_state).unwrap().unwrap();
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "s-play-inspector" }) })).expect("configured leaf is host-owned");
    let after = ShellState::panel_state_from_view(&shell.session.as_ref().unwrap().view_state).unwrap().unwrap();
    assert_eq!(after.active_panel_tab, "s-play-inspector");
    assert_eq!(after.spawned_apps, before.spawned_apps);
    assert_eq!(after.active_spawned_id, before.active_spawned_id);
    assert!(shell.right_panel_open);
    assert_eq!(shell.active_right_kind, RightPanelKind::Details);
    assert_eq!(shell.active_right_tab.as_deref(), Some("s-play-inspector"));

    let accepted_json = shell.session.as_ref().unwrap().view_state.panel_json.clone();
    let container_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "studio-settings" }) })).unwrap_err();
    assert_eq!(container_error, "host-panel.tab-is-not-configured-leaf");
    assert_eq!(shell.session.as_ref().unwrap().view_state.panel_json, accepted_json);
    let stale_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "missing" }) })).unwrap_err();
    assert_eq!(stale_error, "host-panel.tab-is-not-configured-leaf");
    let missing_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.studio".into(), action: "setActivePanelTab".into(), args: None })).unwrap_err();
    assert_eq!(missing_error, "host-panel.invalid-tab-id");

    let guest_error = semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "foreign.controller".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "s-play-inspector" }) })).expect_err("unclaimed controller reaches the missing guest fixture");
    assert!(!guest_error.starts_with("host-panel."), "unclaimed route must expose the actual missing guest failure: {guest_error}");

    let (home, _) = host_test_apps();
    let home_panel = SpacePanelState { active_panel_tab: "s-play-catalogue".into(), spawned_apps: after.spawned_apps.clone(), active_spawned_id: after.active_spawned_id.clone() };
    let mut home_view = ViewModel::default();
    home_view.active_mode_id = Some(home.default_mode_id.clone());
    home_view.active_window_kind_id = Some(home.window_kinds.first().id.clone());
    home_view.panel_json = Some(ShellState::panel_json(&home_panel).unwrap());
    shell.directory_home = Some(DirectoryHomeProjection::new("space".into(), 88, home.clone(), home_view.clone()).expect("directory home projection"));
    shell.session = Some(ActiveSession { plugin_id: "space".into(), instance_id: 88, app: home, view_state: home_view });
    semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "space.home".into(), action: "setActivePanelTab".into(), args: crate::action_args_json!({ "tabId": "home-library" }) })).expect("active host session owns its configured leaf");
    let restored = shell.directory_home.as_ref().unwrap().active_session();
    let restored_panel = ShellState::panel_state_from_view(&restored.view_state).unwrap().unwrap();
    assert_eq!(restored_panel.active_panel_tab, "home-library");
    assert_eq!(restored_panel.spawned_apps, after.spawned_apps);
    assert_eq!(restored_panel.active_spawned_id, after.active_spawned_id);
    assert!(shell.left_panel_open);
    assert_eq!(shell.active_left_kind, LeftPanelKind::Display);
    assert_eq!(shell.active_left_tab.as_deref(), Some("home-library"));
    eprintln!("[DEBUG] native host panel action stayed in session ownership, rejected invalid claimed routes before the missing guest, and restored DirectoryHomeProjection state");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn host_panel_view_context_replaces_stale_session_identity_for_every_native_call() {
    let mut shell = host_test_shell();
    let session = shell.session.as_mut().expect("session");
    session.view_state.session_identity = Some(ViewSessionIdentity { user_id: "stale".into(), display_name: "Stale".into() });
    let session = shell.session.as_ref().expect("session").clone();
    assert_eq!(shell.live_view_state(&session).session_identity, None, "an unauthenticated host must remove a persisted stale identity");
    shell.identity = Some(Identity { user_id: "user-a".into(), email: "a@example.test".into(), display_name: "Ada".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 1 });
    assert_eq!(shell.live_view_state(&session).session_identity, Some(ViewSessionIdentity { user_id: "user-a".into(), display_name: "Ada".into() }));
    shell.identity = Some(Identity { user_id: "user-b".into(), email: "b@example.test".into(), display_name: "Berta".into(), hub_base_url: "https://hub.example".into(), issued_at_ms: 2 });
    assert_eq!(shell.live_view_state(&session).session_identity, Some(ViewSessionIdentity { user_id: "user-b".into(), display_name: "Berta".into() }));
}

#[test]
fn panel_anchor_from_group_matches_panel_group_anchor_corners() {
    assert_eq!(PanelAnchor::from_group(PanelGroup::Workbench), PanelAnchor::TopLeft);
    assert_eq!(PanelAnchor::from_group(PanelGroup::Details), PanelAnchor::TopRight);
    assert_eq!(PanelAnchor::from_group(PanelGroup::Display), PanelAnchor::BottomLeft);
    assert_eq!(PanelAnchor::from_group(PanelGroup::Settings), PanelAnchor::BottomRight);
}

#[test]
fn panel_anchor_as_str_matches_react_panel_anchor_ids() {
    let expected = [
        (PanelAnchor::TopLeft, "top-left"),
        (PanelAnchor::TopMiddle, "top-middle"),
        (PanelAnchor::TopRight, "top-right"),
        (PanelAnchor::RightMiddle, "right-middle"),
        (PanelAnchor::BottomRight, "bottom-right"),
        (PanelAnchor::BottomMiddle, "bottom-middle"),
        (PanelAnchor::BottomLeft, "bottom-left"),
        (PanelAnchor::LeftMiddle, "left-middle"),
    ];
    for (anchor, id) in expected {
        assert_eq!(anchor.as_str(), id);
    }
    assert_eq!(PanelAnchor::ALL.len(), 8);
}

/// ↔ Keeps native panel initialization aligned with the React shell's shared 300px default.
#[test]
fn panel_default_width_is_uniform_and_wider_than_the_former_document_panel() {
    assert_eq!(DEFAULT_PANEL_WIDTH_PX, 300.0);
    assert!(DEFAULT_PANEL_WIDTH_PX > 280.0);
}

#[test]
fn panel_anchor_snapshot_top_left_visible_only_when_workbench_active_and_open() {
    let mut state = fresh_state();
    state.left_panel_open = true;
    state.active_left_kind = LeftPanelKind::Workbench;
    state.left_panel_width = 300.0;
    let top_left = state.panel_anchor_snapshot(PanelAnchor::TopLeft);
    assert!(top_left.visible);
    assert_eq!(top_left.size, 300.0);
    assert_eq!(top_left.active_tab.as_deref(), Some("workbench"));
    let bottom_left = state.panel_anchor_snapshot(PanelAnchor::BottomLeft);
    assert!(!bottom_left.visible, "display anchor must stay hidden while workbench occupies the left column");
}

#[test]
fn panel_anchor_snapshot_switches_corner_with_active_kind_not_visibility_alone() {
    let mut state = fresh_state();
    state.left_panel_open = true;
    state.active_left_kind = LeftPanelKind::Display;
    assert!(!state.panel_anchor_snapshot(PanelAnchor::TopLeft).visible);
    assert!(state.panel_anchor_snapshot(PanelAnchor::BottomLeft).visible);
    state.right_panel_open = true;
    state.active_right_kind = RightPanelKind::Settings;
    assert!(!state.panel_anchor_snapshot(PanelAnchor::TopRight).visible);
    assert!(state.panel_anchor_snapshot(PanelAnchor::BottomRight).visible);
}

#[test]
fn panel_anchor_snapshot_middle_anchors_are_always_empty() {
    let mut state = fresh_state();
    state.left_panel_open = true;
    state.right_panel_open = true;
    assert_eq!(state.panel_anchor_snapshot(PanelAnchor::TopMiddle), PanelAnchorSnapshot::default());
    assert_eq!(state.panel_anchor_snapshot(PanelAnchor::BottomMiddle), PanelAnchorSnapshot::default());
    assert_eq!(state.panel_anchor_snapshot(PanelAnchor::LeftMiddle), PanelAnchorSnapshot::default());
    assert_eq!(state.panel_anchor_snapshot(PanelAnchor::RightMiddle), PanelAnchorSnapshot::default());
}

#[test]
fn panel_layout_snapshot_round_trips_through_apply_panel_layout() {
    let mut source = fresh_state();
    source.left_panel_open = true;
    source.right_panel_open = false;
    source.active_left_kind = LeftPanelKind::Display;
    source.active_right_kind = RightPanelKind::Settings;
    source.left_panel_width = 411.0;
    source.right_panel_width = 233.0;
    let snapshot = source.panel_layout_snapshot();
    assert_eq!(snapshot.active_left_kind.as_deref(), Some("display"));
    assert_eq!(snapshot.active_right_kind.as_deref(), Some("settings"));

    let mut target = fresh_state();
    target.apply_panel_layout(&snapshot);
    assert_eq!(target.left_panel_open, source.left_panel_open);
    assert_eq!(target.right_panel_open, source.right_panel_open);
    assert_eq!(target.active_left_kind, source.active_left_kind);
    assert_eq!(target.active_right_kind, source.active_right_kind);
    assert_eq!(target.left_panel_width, source.left_panel_width);
    assert_eq!(target.right_panel_width, source.right_panel_width);
}

#[test]
fn apply_panel_layout_leaves_widths_untouched_when_absent_from_snapshot() {
    let mut state = fresh_state();
    state.left_panel_width = 555.0;
    state.right_panel_width = 666.0;
    let sparse = PanelLayoutPersisted { left_panel_open: true, right_panel_open: true, active_left_kind: None, active_right_kind: None, left_panel_width: None, right_panel_width: None };
    state.apply_panel_layout(&sparse);
    assert_eq!(state.left_panel_width, 555.0, "absent width in a persisted snapshot must not clobber the current width");
    assert_eq!(state.right_panel_width, 666.0);
    assert_eq!(state.active_left_kind, LeftPanelKind::Workbench, "absent active kind falls back to the default");
}

/// 🗄️ Now that panel layout storage routes through the same `prefs_get`/`prefs_set` primitives as
/// every other uiPref (see the dedup note on `load_panel_layout_from_store`), a round trip through
/// `save_panel_layout_to_store`/`load_panel_layout_from_store` exercises the exact same `PREFS_STORE`
/// thread-local singleton `file_prefs_store_round_trips_through_disk` (🧪️UiPrefsThemesI18nTests)
/// already proves is disk-durable on native — this only needs to prove the panel-layout JSON shape
/// itself round-trips through that singleton correctly.
#[test]
fn panel_layout_round_trips_through_prefs_store() {
    let layout = PanelLayoutPersisted { left_panel_open: true, right_panel_open: false, active_left_kind: Some("display".to_string()), active_right_kind: Some("details".to_string()), left_panel_width: Some(321.0), right_panel_width: Some(210.0) };
    save_panel_layout_to_store(&layout);
    let loaded = load_panel_layout_from_store().expect("round-tripped layout must parse back");
    assert_eq!(loaded, layout);
}

/// 🗄️ `persist_panel_layout_if_changed`'s dirty-check: a second call with no field changes since the
/// last persist must not touch storage again — mirrors `persist_ui_prefs_if_changed_is_idempotent_
/// when_nothing_changed` (🧪️UiPrefsThemesI18nTests) one region over, same shape for the same reason
/// (this is the render-loop hook that replaces patching every `ui.panelToggle.*` call site — see
/// `persist_panel_layout_if_changed`'s doc comment).
#[test]
fn persist_panel_layout_if_changed_is_idempotent_when_nothing_changed() {
    let mut state = fresh_state();
    state.left_panel_open = true;
    state.active_left_kind = LeftPanelKind::Display;
    state.persist_panel_layout_if_changed();
    let after_first = load_panel_layout_from_store().expect("first call must persist");
    assert_eq!(after_first.active_left_kind.as_deref(), Some("display"));

    // A second call with identical state must be a no-op — flip storage underneath it directly so a
    // wrongly-unconditional write would be observable.
    save_panel_layout_to_store(&PanelLayoutPersisted::default());
    state.persist_panel_layout_if_changed();
    let after_second = load_panel_layout_from_store().expect("storage still has a value");
    assert_eq!(after_second, PanelLayoutPersisted::default(), "unchanged state must not re-persist and clobber the manual write above");
}

/// 🎨️ `build_settings_theme_ui`'s reachability contract: a select node listing the built-in themes
/// plus any saved custom ones, a reset button always present, and a delete button gated strictly on
/// the active theme id being a `"custom."`-prefixed one (mirrors React's `host.themeId.startsWith(
/// "custom.")` gate on the same button, `ui/js/react/index.tsx:9489`).
#[test]
fn build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme() {
    let mut state = fresh_state();
    state.chrome_build.preferences.theme_id = "semio".to_string();
    let UiNode::Stack(builtin_panel) = state.build_settings_theme_ui() else {
        panic!("expected a stack root");
    };
    let has_select = builtin_panel.children.iter().any(|node| matches!(node, UiNode::Select(_)));
    assert!(has_select, "must render the theme picker select");
    let button_count = builtin_panel.children.iter().filter(|node| matches!(node, UiNode::Button(_))).count();
    assert_eq!(button_count, 1, "only Reset, no Delete, while the built-in \"semio\" theme is active");

    state.chrome_build.preferences.theme_id = "custom.wp-audit-test".to_string();
    let UiNode::Stack(custom_panel) = state.build_settings_theme_ui() else {
        panic!("expected a stack root");
    };
    let button_count = custom_panel.children.iter().filter(|node| matches!(node, UiNode::Button(_))).count();
    assert_eq!(button_count, 2, "Reset and Delete once a custom theme is active");
}
