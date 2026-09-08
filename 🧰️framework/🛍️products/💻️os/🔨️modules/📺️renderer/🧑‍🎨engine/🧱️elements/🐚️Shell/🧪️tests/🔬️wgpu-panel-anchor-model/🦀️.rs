
use super::*;

fn fresh_state() -> ShellState {
    // 🧪️ `ShellState::new` calls `load_persisted_panel_layout`, which — on native — reads whatever
    // happens to be at `~/.semio/panel-layout.json` on the machine running the test. Every assertion
    // below explicitly sets the fields it exercises afterward, so the outcome never depends on that.
    ShellState::new(Vec::new(), String::new())
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
