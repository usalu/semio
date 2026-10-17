/// 🗄️ Dock override round trip: dock skeleton application, tab moves, the shell config round trip, persistence idempotence and the theme panel gating.

//#region 🗄️DockOverrideRoundTrip
/// 🗄️ `apply_dock_skeleton` REARRANGES the default dock, never reconstructs it: a tab moved to another
/// anchor lands there, and a default tab the skeleton never mentions is appended at its default home.
#[test]
fn apply_dock_skeleton_moves_mentioned_tabs_and_appends_unmentioned_defaults() {
    let shell = fixture_dock_shell();
    let default_dock = SessionCtx::of(&mut shell).default_dock();
    let mut skeleton = dock_skeleton_of(&default_dock);
    skeleton.anchors.insert("top-left".to_string(), Vec::new());
    skeleton.anchors.insert("left-middle".to_string(), vec![DockTabSkeleton { id: "fixture.workbench".into(), children: None, trees: Some(vec![dock_leaf_tree_unit_id("fixture.workbench")]) }]);
    let rearranged = apply_dock_skeleton(&default_dock, Some(&skeleton));
    assert_eq!(rearranged.locate("fixture.workbench").map(|(anchor, _)| anchor), Some(PanelAnchor::LeftMiddle));
    assert!(rearranged.tabs(PanelAnchor::TopLeft).is_empty());
    assert_eq!(rearranged.tabs(PanelAnchor::TopRight).iter().map(|tab| tab.id.as_str()).collect::<Vec<_>>(), default_dock.tabs(PanelAnchor::TopRight).iter().map(|tab| tab.id.as_str()).collect::<Vec<_>>(), "an untouched anchor keeps its default row");
}

/// 🗄️ A skeleton that mentions nothing is the identity, and an id the default no longer declares is
/// dropped rather than resurrected.
#[test]
fn apply_dock_skeleton_is_identity_for_none_and_drops_unknown_ids() {
    let shell = fixture_dock_shell();
    let default_dock = SessionCtx::of(&mut shell).default_dock();
    assert_eq!(apply_dock_skeleton(&default_dock, None), default_dock);
    let mut stale = DockSkeleton::default();
    stale.anchors.insert("top-middle".to_string(), vec![DockTabSkeleton { id: "tab.that.no.longer.exists".into(), children: None, trees: None }]);
    let applied = apply_dock_skeleton(&default_dock, Some(&stale));
    assert!(applied.tabs(PanelAnchor::TopMiddle).is_empty());
    assert_eq!(applied.tabs(PanelAnchor::TopLeft).len(), default_dock.tabs(PanelAnchor::TopLeft).len(), "nothing mentioned it, so the default row is re-seeded");
}

/// 🎯️ A tab dragged to another anchor is a pure `move_tab_in_dock` transform, and dropping a subtree
/// into itself is refused.
#[test]
fn move_tab_in_dock_reanchors_a_tab_and_refuses_self_drops() {
    let shell = fixture_dock_shell();
    let mut dock = SessionCtx::of(&mut shell).default_dock();
    let target = DockTabMoveTarget { anchor: PanelAnchor::RightMiddle, parent_path: Vec::new(), index: 0 };
    assert!(move_tab_in_dock(&mut dock, "fixture.workbench", &target));
    assert_eq!(dock.locate("fixture.workbench").map(|(anchor, _)| anchor), Some(PanelAnchor::RightMiddle));
    assert!(dock.tabs(PanelAnchor::TopLeft).is_empty());
    let into_itself = DockTabMoveTarget { anchor: PanelAnchor::BottomMiddle, parent_path: vec![FRAMEWORK_CATEGORY_COMMAND_ID.to_string()], index: 0 };
    let before = dock.clone();
    assert!(!move_tab_in_dock(&mut dock, FRAMEWORK_CATEGORY_COMMAND_ID, &into_itself));
    assert_eq!(dock, before, "a refused move leaves the dock byte-identical");
}

/// 🗄️ The override ROUND TRIP through the exact `semio.os.config` document React's `DockLayoutStore`
/// owns. It runs over an in-memory document rather than the process-global store on purpose: every
/// `prefs_set` is a read-modify-write of that ONE document, so a parallel test writing an unrelated
/// `preferences` key clobbers whatever dock layer was written a microsecond earlier. The layer rules —
/// per-app precedence, `None` removal, the version gate — are what this pins, and they are the whole
/// of what a renderer switch depends on.
///
/// 🗄️ A per-app layer wins over the shared `os` one; removing it falls back to `os`; removing both
/// leaves nothing — `DockLayoutStore::getSnapshot`/`save`/`saveOs` exactly.
///
/// 🧭️ And moving the tab back to its computed default drops the override entirely, so nothing persists.
#[test]
fn dock_override_round_trips_through_the_os_shell_config_document() {
    let mut shell = fixture_dock_shell();
    let target = DockTabMoveTarget { anchor: PanelAnchor::TopMiddle, parent_path: Vec::new(), index: 0 };
    assert!(SessionCtx::of(&mut shell).move_dock_tab("fixture.details", &target));
    let moved = shell.dock.dock_override.clone().expect("a rearranged dock stops matching its default");

    let mut config = serde_json::json!({ "version": 1, "preferences": {} });
    write_os_shell_config_layer_in(&mut config, "dockLayouts", Some("fixture-app"), Some(&moved));
    assert_eq!(config["dockLayouts"]["apps"]["fixture-app"]["version"], serde_json::json!(3));
    assert_eq!(config["dockLayouts"]["apps"]["fixture-app"]["anchors"]["top-middle"][0]["id"], serde_json::json!("fixture.details"));

    let loaded: DockSkeleton = read_os_shell_config_layer(&config, "dockLayouts", Some("fixture-app")).expect("the per-app layer reads back");
    assert!(dock_skeletons_equal(Some(&loaded), Some(&moved)));
    let reloaded = apply_dock_skeleton(&SessionCtx::of(&mut shell).default_dock(), Some(&loaded));
    assert_eq!(reloaded.locate("fixture.details").map(|(anchor, _)| anchor), Some(PanelAnchor::TopMiddle));
    assert_eq!(dock_skeleton_of(&reloaded), dock_skeleton_of(&shell.dock.dock_tabs));

    let default_skeleton = dock_skeleton_of(&SessionCtx::of(&mut shell).default_dock());
    write_os_shell_config_layer_in(&mut config, "dockLayouts", None, Some(&default_skeleton));
    let still_app: DockSkeleton = read_os_shell_config_layer(&config, "dockLayouts", Some("fixture-app")).expect("per-app layer still wins");
    assert!(dock_skeletons_equal(Some(&still_app), Some(&moved)));
    write_os_shell_config_layer_in::<DockSkeleton>(&mut config, "dockLayouts", Some("fixture-app"), None);
    let os_layer: DockSkeleton = read_os_shell_config_layer(&config, "dockLayouts", Some("fixture-app")).expect("falls back to the os layer");
    assert!(dock_skeletons_equal(Some(&os_layer), Some(&default_skeleton)));
    write_os_shell_config_layer_in::<DockSkeleton>(&mut config, "dockLayouts", None, None);
    assert!(read_os_shell_config_layer::<DockSkeleton>(&config, "dockLayouts", Some("fixture-app")).is_none());

    let back = DockTabMoveTarget { anchor: PanelAnchor::TopRight, parent_path: Vec::new(), index: 0 };
    assert!(SessionCtx::of(&mut shell).move_dock_tab("fixture.details", &back));
    assert!(shell.dock.dock_override.is_none(), "a dock back at its computed default persists no override");
}

/// 🗄️ The per-anchor chrome round trip: visibility, size and active path survive a reload, and an
/// anchor absent from the persisted document reloads folded.
#[test]
fn dock_ui_state_round_trips_visibility_size_and_path() {
    let mut shell = fixture_dock_shell();
    shell.dock.toggle_anchor_tab(PanelAnchor::BottomRight, "fixture.settings");
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).size = 412.0;
    let snapshot = shell.dock.dock_ui_snapshot();
    assert_eq!(snapshot.version, 3);
    assert_eq!(snapshot.anchors.get("bottom-right").and_then(|entry| entry.size), Some(412.0));
    assert!(snapshot.anchors.get("top-left").is_none(), "a default anchor stores nothing");

    let mut config = serde_json::json!({ "version": 1, "preferences": {} });
    write_os_shell_config_layer_in(&mut config, "dockUi", Some("fixture-app"), Some(&snapshot));
    let loaded: DockUiState = read_os_shell_config_layer(&config, "dockUi", Some("fixture-app")).expect("dockUi layer reads back");
    assert_eq!(loaded, snapshot);

    let mut target = fixture_dock_shell();
    target.dock.apply_dock_ui(&loaded);
    assert!(target.dock.anchor_open(PanelAnchor::BottomRight));
    assert_eq!(target.dock.anchor_state(PanelAnchor::BottomRight).size, 412.0);
    assert_eq!(target.dock.anchor_state(PanelAnchor::BottomRight).active_tab(), Some("fixture.settings"));
    assert!(!target.dock.anchor_open(PanelAnchor::TopLeft));
}

/// 🗄️ `persist_dock_ui_if_changed`'s dirty-check: the first call after a real change persists, and a
/// second call with nothing changed must not write again — the render-loop hook that replaces patching
/// every `ui.panelToggle.*` call site. Asserted against the owned present-state snapshot rather than the
/// shared store, for the same read-modify-write reason the override test above documents.
///
/// Nothing changed since: a wrongly-unconditional write would be observable as a new snapshot object
/// replacing this deliberately-wrong marker.
#[test]
fn persist_dock_ui_if_changed_is_idempotent_when_nothing_changed() {
    let mut shell = fixture_dock_shell();
    shell.chrome.chrome_present.last_persisted_dock_ui = Some(shell.dock.dock_ui_snapshot());
    shell.dock.toggle_anchor(PanelAnchor::TopLeft);
    shell.dock.persist_dock_ui_if_changed(&mut shell.chrome, &shell.docs);
    let persisted = shell.chrome.chrome_present.last_persisted_dock_ui.clone().expect("first call must persist");
    assert!(persisted.anchors.get("top-left").and_then(|entry| entry.visible).unwrap_or(false));

    shell.chrome.chrome_present.last_persisted_dock_ui = Some(persisted.clone());
    shell.dock.persist_dock_ui_if_changed(&mut shell.chrome, &shell.docs);
    assert_eq!(shell.chrome.chrome_present.last_persisted_dock_ui.as_ref(), Some(&persisted), "unchanged state must not re-persist");
    assert_eq!(shell.chrome.chrome_present.last_persisted_dock_skeleton, Some(shell.dock.dock_override.clone()));
}
//#endregion 🗄️DockOverrideRoundTrip

/// 🎨️ `build_settings_theme_ui`'s reachability contract: a select node listing the built-in themes
/// plus any saved custom ones, a reset button always present, and a delete button gated strictly on
/// the active theme id being a `"custom."`-prefixed one (mirrors React's `host.themeId.startsWith(
/// "custom.")` gate on the same button, `ui/js/react/index.tsx:9489`).
#[test]
fn build_settings_theme_ui_lists_builtins_and_gates_delete_on_custom_theme() {
    let mut state = fresh_state();
    state.chrome.chrome_build.preferences.theme_id = "semio".to_string();
    let builtin = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &state.input.build_settings_theme_ui(&state.chrome, &state.prefs)).expect("built-in theme editor projection");
    assert!(builtin.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.select")), "the canonical tree publishes the theme picker");
    assert!(builtin.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")), "the canonical tree publishes Reset");
    assert!(!builtin.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.delete")), "a built-in theme publishes no Delete control");
    drop(builtin);
    while !ui_contract::close_ui_value_page_one() {}

    state.chrome.chrome_build.preferences.theme_id = "custom.wp-audit-test".to_string();
    let custom = panel_ui_records(FRAMEWORK_SETTINGS_THEME_TAB_ID, &state.input.build_settings_theme_ui(&state.chrome, &state.prefs)).expect("custom theme editor projection");
    assert!(custom.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.reset")), "a custom theme retains Reset");
    assert!(custom.iter().any(|record| record.key.as_str().ends_with("/framework.settings.theme.delete")), "a custom theme publishes Delete");
    drop(custom);
    while !ui_contract::close_ui_value_page_one() {}
}
