//! 🖥️ LAW: the four panel bodies React has and this renderer had not — the two Display leaves
//! (`framework.display.windows` / `framework.display.layout`, whose dock tabs existed with no router
//! arm at all), the Conflicts leaf's REAL projection, the Tool leaf's measures and the Marketplace
//! leaf's install/uninstall lane — publish retained bodies carrying React's own control ids.
//!
//! Oracles, read off React directly: `buildDisplayWindowsTree`/`buildDisplayLayoutTree`/
//! `buildConflictsTree`/`buildMarketplaceTree` (`📌️ChromePanels/🟦️.tsx`), `buildToolTree` +
//! `windowMeasuresToTreeItems` (`🛠️ShellHelpers/🟦️.tsx`) and `createWorldProjectionTemplates`
//! (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`).

use super::*;

/// 🧪️ The panel-anchor fixture's own shell (one `space` bridge, a bound `studio` session), which is
/// what makes these bodies exercise a REAL session rather than the no-session fallbacks.
fn display_shell() -> ShellState {
    super::panel_anchor_model_tests::host_test_shell()
}

/// 🧾️ Every record key one shell-owned leaf's published body carries, in publication order — each
/// tail is React's own control id (`panel_ui_records` qualifies the key with its surface).
fn published_keys(shell: &ShellState, tab_id: &str, node: &UiNode) -> Vec<String> {
    panel_ui_records(tab_id, node).expect("the leaf body projects").into_iter().map(|record| record.key.as_str().to_string()).collect()
}

/// 🖥️ **The P0 pin.** Both Display leaves are shell-owned AND routed: before this packet the two dock
/// tabs existed while `publish_shell_panel_document` had no arm for either id, so selecting one
/// published nothing at all.
#[test]
fn both_display_leaves_are_shell_owned_and_publish_a_body() {
    let mut shell = display_shell();
    let leaves = shell.shell_owned_panel_leaves();
    for tab_id in [FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, FRAMEWORK_DISPLAY_LAYOUT_TAB_ID] {
        assert!(leaves.contains(&tab_id.to_string()), "🖥️ '{tab_id}' must be a shell-owned leaf");
        let published = shell.publish_shell_panel_document(tab_id).expect("the Display leaf publishes");
        assert!(published.is_some(), "🖥️ '{tab_id}' must publish a retained body, not `Ok(None)`");
    }
}

/// 🖥️ **The Windows census.** One section per window kind carrying React's own
/// `framework.display.windows.<kind>` id, its plain `.kind` leaf, and — only for a `world-3d`
/// surface — the whole `createWorldProjectionTemplates` taxonomy under React's nested id grammar.
#[test]
fn the_display_windows_leaf_publishes_reacts_kind_and_projection_ids() {
    let mut shell = display_shell();
    let keys = published_keys(&shell, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &shell.build_display_windows_ui());
    assert!(keys.iter().any(|key| key.ends_with("framework.display.windows.main")), "🖥️ a section per window kind, got {keys:?}");
    assert!(keys.iter().any(|key| key.ends_with("framework.display.windows.main.kind")), "🖥️ the plain kind leaf carries React's own id");
    assert!(!keys.iter().any(|key| key.contains(".projection.")), "🖥️ a canvas-2d kind offers no projection rows, exactly as React's `surfaceKind === \"world-3d\"` gate says");

    let session = shell.session.as_mut().expect("the fixture binds a session");
    session.app.window_kinds.first_mut().surface_kind = ui_wgpu::wgpu::SurfaceKind::World3d;
    let keys = published_keys(&shell, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &shell.build_display_windows_ui());
    for react_id in [
        "framework.display.windows.main.projection.parallel",
        "framework.display.windows.main.projection.parallel.orthographic",
        "framework.display.windows.main.projection.parallel.axonometric",
        "framework.display.windows.main.projection.parallel.axonometric.axonometric-isometric",
        "framework.display.windows.main.projection.perspective",
    ] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🔀️ '{react_id}' is React's own nested template id, got {keys:?}");
    }
    let kind_row = keys.iter().position(|key| key.ends_with("framework.display.windows.main.kind")).expect("the kind leaf");
    let parallel = keys.iter().position(|key| key.ends_with("framework.display.windows.main.projection.parallel")).expect("the parallel branch");
    assert!(kind_row < parallel, "🔃️ React RENDERS the plain kind leaf first, then Parallel, then Perspective");
}

/// 🖥️ **The Layout census.** React's save section (name box + Save, disabled on a blank name), the
/// list section, `groupPath` folding, and a user layout's own `framework.display.delete.<id>` action.
#[test]
fn the_display_layout_leaf_publishes_the_save_box_and_both_layout_rosters() {
    let mut shell = display_shell();
    shell.layout_save_label.clear();
    shell.user_layouts = vec![ui_wgpu::wgpu::NamedLayout {
        id: "user-7".into(),
        label: "Mine".into(),
        icon_id: None,
        layout: shell.dock.to_window_layout(),
        origin: "user".into(),
        group_path: None,
    }];
    if let Some(session) = shell.session.as_mut() {
        session.app.named_layouts = vec![ui_wgpu::wgpu::NamedLayout {
            id: "review".into(),
            label: "Review".into(),
            icon_id: None,
            layout: ui_wgpu::wgpu::create_stack_layout(&["main".to_string()], None),
            origin: "builtin".into(),
            group_path: Some(vec!["Work".into()]),
        }];
    }
    let node = shell.build_display_layout_ui();
    let keys = published_keys(&shell, FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, &node);
    for react_id in [
        "framework.display.layout.save",
        "framework.display.layout.save.label",
        "framework.display.saveLabel",
        "framework.display.layout.save.action",
        "framework.display.save",
        "framework.display.layout.list",
        "framework.display.layout.group.Work",
        "framework.display.layout.review",
        "framework.display.layout.group.saved",
        "framework.display.layout.user-7",
        "framework.display.delete.user-7",
    ] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🖥️ '{react_id}' is React's own id, got {keys:?}");
    }
    let records = panel_ui_records(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, &node).expect("the layout body projects");
    let save = records.iter().find(|record| record.key.as_str().ends_with("framework.display.save")).expect("the Save button");
    assert!(save.disabled, "🖥️ React disables Save on a blank name (`!host.layoutSaveLabel.trim()`)");
    shell.layout_save_label = "Mine".into();
    let records = panel_ui_records(FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, &shell.build_display_layout_ui()).expect("the layout body projects");
    let save = records.iter().find(|record| record.key.as_str().ends_with("framework.display.save")).expect("the Save button");
    assert!(!save.disabled, "🖥️ a typed name enables Save");
}

/// ⚖️ **The Conflicts projection.** With no open conflict the leaf still paints React's own
/// unavailable row; with one it paints the real row plus React's `.accept`/`.discard` pair — the gap
/// the leaf carried since W2c ("never paints a conflict either").
#[test]
fn the_conflicts_leaf_paints_the_live_roster_and_reacts_resolution_pair() {
    let mut shell = display_shell();
    let keys = published_keys(&shell, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &shell.build_settings_conflicts_ui());
    assert!(keys.iter().any(|key| key.ends_with("framework.settings.conflicts.empty.row")), "⚖️ an empty roster is React's own unavailable row");

    shell.open_conflicts = vec![ShellConflictRow { id: "conflict-1".into(), quarantined: true, code: "mergeQuarantined".into(), message: "two actors edited the same joint".into() }];
    shell.selected_conflict_id = Some("conflict-1".into());
    let node = shell.build_settings_conflicts_ui();
    let keys = published_keys(&shell, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &node);
    for react_id in ["framework.settings.conflicts.table", "framework.settings.conflicts.conflict-1", "framework.settings.conflicts.conflict-1.accept", "framework.settings.conflicts.conflict-1.discard"] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "⚖️ '{react_id}' is React's own id, got {keys:?}");
    }
    assert!(!keys.iter().any(|key| key.ends_with("framework.settings.conflicts.empty.row")), "⚖️ an open conflict replaces the unavailable row");
    let records = panel_ui_records(FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, &node).expect("the conflicts body projects");
    let row = records.iter().find(|record| record.key.as_str().ends_with("framework.settings.conflicts.conflict-1")).expect("the conflict row");
    let ui_contract::Component::TreeItem(props) = &row.component else { panic!("⚖️ a conflict is a tree row") };
    let label = props.label.0.as_str();
    assert!(label.contains("mergeQuarantined") && label.contains("two actors edited the same joint"), "⚖️ the row prints the worst message's code and text, got '{label}'");
}

/// 🛠️ **The Tool measures pin.** A tool leaf's body is its own published measures under the measures'
/// own ids — React's `windowMeasuresToTreeItems(toolMeasuresByToolId[id])`, which this renderer could
/// not reach at all before the program bridge grew its `tool_measures_section` reader.
#[test]
fn a_tool_leaf_publishes_its_own_measures_under_their_own_ids() {
    let mut shell = display_shell();
    let tab_id = format!("{FRAMEWORK_TOOL_PANEL_TAB_PREFIX}fill");
    let keys = published_keys(&shell, &tab_id, &shell.build_tool_panel_ui("fill"));
    assert!(keys.iter().any(|key| key.ends_with("tool.fill.options")), "🛠️ the headerless options section is React's own `tool.<id>.options`");
    assert_eq!(keys.iter().filter(|key| key.contains("/tool.fill.")).count(), 1, "🛠️ with no measures in hand React renders the section and nothing else");

    shell.tool_measures.insert(
        "fill".into(),
        vec![WindowMeasure::Select {
            id: "fill.material".into(),
            label: Some("Material".into()),
            value: "oak".into(),
            items: vec![ui_wgpu::wgpu::MeasureSelectItem { id: "oak".into(), value: "oak".into(), label: "Oak".into() }],
            on_change: ActionDescriptor { controller_id: "test".into(), action: "setFillMaterial".into(), args: None },
        }],
    );
    let node = shell.build_tool_panel_ui("fill");
    let keys = published_keys(&shell, &tab_id, &node);
    assert_eq!(keys.iter().filter(|key| key.ends_with("fill.material")).count(), 2, "🛠️ the measure publishes its row AND its control, both under the measure's own id, got {keys:?}");
    let records = panel_ui_records(&tab_id, &node).expect("the tool body projects");
    let control = records.iter().find(|record| matches!(record.component, ui_contract::Component::Select(_))).expect("the measure's own editor");
    assert!(control.bindings.iter().any(|binding| format!("{:?}", binding.action).contains("setFillMaterial")), "🛠️ the control dispatches the measure's OWN action, not a shell verb");
}

/// 🛍️ **The Marketplace action lane.** A resident plugin offers React's reload/uninstall pair, a
/// plugin only the activation catalogue knows offers Install, and the session's own program may never
/// be uninstalled (React's `canUninstall`).
#[test]
fn the_marketplace_leaf_offers_reacts_install_reload_and_uninstall_verbs() {
    let shell = display_shell();
    let node = shell.build_marketplace_ui();
    let keys = published_keys(&shell, FRAMEWORK_MARKETPLACE_TAB_ID, &node);
    for react_id in ["framework.marketplace.source.local", "framework.marketplace.plugin.space", "framework.marketplace.plugin.space.reload", "framework.marketplace.plugin.space.uninstall"] {
        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🛍️ '{react_id}' is React's own id, got {keys:?}");
    }
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &node).expect("the marketplace body projects");
    let uninstall = records.iter().find(|record| record.key.as_str().ends_with("framework.marketplace.plugin.space.uninstall")).expect("the uninstall row");
    assert!(uninstall.disabled, "🛍️ the session's OWN program is never uninstallable — React's `canUninstall`");

    let installable = crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS.iter().map(|(_, plugin_id)| *plugin_id).find(|plugin_id| *plugin_id != "space").expect("the catalogue claims a plugin this shell does not hold");
    let install_id = format!("framework.marketplace.plugin.{installable}.install");
    assert!(keys.iter().any(|key| key.ends_with(install_id.as_str())), "🛍️ a non-resident plugin offers React's Install verb, got {keys:?}");

    let mut shell = shell;
    assert!(!shell.uninstall_plugin("space"), "🛍️ uninstalling the running program is refused");
    assert_eq!(shell.plugins.len(), 1, "🛍️ and the roster is untouched");
}
