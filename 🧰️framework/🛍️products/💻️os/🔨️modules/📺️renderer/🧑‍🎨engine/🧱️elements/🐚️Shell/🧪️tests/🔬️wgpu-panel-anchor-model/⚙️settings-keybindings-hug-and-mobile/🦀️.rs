/// ⚙️ Settings, keybindings, hug and mobile: the Settings branch and shell-owned leaves, remappable keybindings, content hug and the mobile flattening.

//#region ⚙️SettingsBranchAndShellOwnedLeaves
/// ⚙️ The bottom-right Settings BRANCH: one toggle carrying the app's own Settings-group tabs first and
/// then the five framework leaves, exactly React's
/// `integrateAppSettingsPanelTabsIntoFrameworkBranch(frameworkSettingsTab, settingsBottomRightTabs)`
/// child order — plus the Marketplace leaf and the framework-owned History leaf beside it, never
/// nested inside it.
#[test]
fn the_settings_branch_nests_the_five_framework_leaves_after_the_apps_own() {
    let fixture = default_dock_fixture();
    let shell = fixture_dock_shell();
    let dock = SessionCtx::of(&mut shell).default_dock();
    let roots: Vec<&str> = dock.tabs(PanelAnchor::BottomRight).iter().map(|tab| tab.id.as_str()).collect();
    assert_eq!(roots, vec![FRAMEWORK_SETTINGS_PANEL_ID, FRAMEWORK_MARKETPLACE_TAB_ID, FRAMEWORK_TASK_MANAGER_PANEL_ID, FRAMEWORK_PANEL_TAB_HISTORY_ID], "⚙️ bottom-right carries Settings, Marketplace, Task Manager, then History");
    let branch = dock.tabs(PanelAnchor::BottomRight).first().expect("the Settings branch");
    assert!(branch.is_branch(), "⚙️ Settings is a branch, not a leaf");
    let children: Vec<&str> = branch.children.iter().map(|child| child.id.as_str()).collect();
    assert_eq!(children.first(), Some(&"fixture.settings"), "⚙️ the app's own Settings tabs come first (document-first child order)");
    let declared: Vec<&str> = fixture["settingsBranch"]["children"].as_array().expect("fixture settings children").iter().map(|id| id.as_str().expect("child id")).collect();
    assert_eq!(&children[1..], declared.as_slice(), "⚙️ then React's five framework leaves, in React's order");
    for (index, child) in branch.children.iter().enumerate() {
        assert_eq!(child.order, index as i32, "⚙️ children are re-ordered past the app's, React's own `+ appSettingsTabs.length`");
    }
}

/// 🧾️ Every SHELL-OWNED leaf the dock declares has a body, and every body the shell publishes is a
/// leaf the dock declares — the assembler's own closure property. A leaf that is dockable but paints
/// nothing (which is exactly what the Settings branch's children were before this packet) fails here.
#[test]
fn every_shell_owned_leaf_projects_into_retained_records() {
    let shell = fixture_dock_shell();
    let dock = SessionCtx::of(&mut shell).default_dock();
    for tab_id in shell.docs.shell_owned_panel_leaves(&shell.chrome, &shell.plugins, &shell.prefs) {
        if tab_id == semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::FRAMEWORK_HUB_PANEL_ID {
            assert!(dock.locate(&tab_id).is_none(), "🧾️ Hub is the route-owned overlay React opens from its footer/command, never a dock substitute");
        } else {
            assert!(dock.locate(&tab_id).is_some(), "🧾️ {tab_id} publishes a body but no anchor declares it");
        }
        let node = match tab_id.as_str() {
            FRAMEWORK_SETTINGS_GENERAL_TAB_ID => shell.docs.build_settings_general_ui(&shell.chrome, &shell.prefs),
            FRAMEWORK_SETTINGS_THEME_TAB_ID => shell.input.build_settings_theme_ui(&shell.chrome, &shell.prefs),
            FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID => shell.input.build_settings_keybindings_ui(&shell.chrome, &shell.prefs),
            FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID => shell.prefs.build_settings_default_apps_ui(&shell.plugins),
            FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID => shell.prefs.build_settings_conflicts_ui(),
            FRAMEWORK_MARKETPLACE_TAB_ID => SessionCtx::of(&mut shell).build_marketplace_ui(),
            FRAMEWORK_SYNC_PANEL_TAB_ID => shell.sync.build_sync_attach_ui(&shell.prefs),
            FRAMEWORK_TASK_MANAGER_PANEL_ID => shell.opening.build_task_manager_ui(&shell.prefs),
            FRAMEWORK_CHAT_PANEL_ID => shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs),
            other => SessionCtx::of(&mut shell).build_command_category_ui(other.trim_start_matches(FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX)),
        };
        let records = panel_ui_records(&tab_id, &node).unwrap_or_else(|error| panic!("🧾️ {tab_id} must project: {error}"));
        assert!(!records.is_empty(), "🧾️ {tab_id} projected an empty document");
        assert_eq!(records[0].id, ui_contract::UiNodeId(1), "🧾️ the root record is always id 1 (the published assembly identity's root)");
        for (index, record) in records.iter().enumerate() {
            assert_eq!(record.id, ui_contract::UiNodeId(index as u64 + 1), "🧾️ records are parent-first and densely numbered");
        }
    }
}

/// 🧾️ The assembler refuses a node it cannot project instead of silently dropping it, so a builder
/// that grows a `Tree`/`Image`/`ComponentScene` fails at publication rather than painting a hole.
#[test]
fn the_panel_assembler_refuses_an_unprojectable_node() {
    let node = UiNode::Image(semio_framework_ui_declaration::ui::UiImageNode { id: "x".into(), src: "y".into(), alt: None, presence: UiPresence::default(), menu: None });
    let error = panel_ui_records("framework.settings.general", &node).expect_err("an image is not projectable by this assembler");
    assert!(error.contains("image"), "the refusal names the variant: {error}");
}
//#endregion ⚙️SettingsBranchAndShellOwnedLeaves

//#region ⌨️RemappableKeybindings
/// ⌨️ An override REPLACES a row's chord: the new chord resolves to that row's verb and is reserved,
/// and the default it replaced stops being the shell's — React's `composeControlKeybindings` (the
/// override wins unconditionally) plus `reservedShellChordsV1` over the same resolved table.
#[test]
fn a_keybinding_override_remaps_a_shell_chord_and_frees_its_default() {
    let mut overrides = HashMap::new();
    overrides.insert("ui.search.toggle".to_string(), "ctrl+j".to_string());
    let table = shell_shortcut_table(&overrides);
    let accelerator = PointerModifiers { ctrl: true, ..PointerModifiers::default() };
    let remapped = semio_framework_ui_wgpu_input::KeyAction::Char("j".into());
    let default = semio_framework_ui_wgpu_input::KeyAction::Char("p".into());
    assert_eq!(shell_shortcut_for_in(&table, &remapped, &accelerator), Some(ShellShortcut::ToggleSearch), "⌨️ the override's chord opens the palette");
    assert!(is_reserved_shell_chord_in(&table, &remapped, &accelerator), "⌨️ and is reserved at its new spelling");
    assert_eq!(shell_shortcut_for_in(&table, &default, &accelerator), None, "⌨️ `mod+p` is no longer the shell's");
    assert!(!is_reserved_shell_chord_in(&table, &default, &accelerator), "⌨️ so an app may claim it");
    assert_eq!(shell_shortcut_for(&default, &accelerator), Some(ShellShortcut::ToggleSearch), "⌨️ the DEFAULT table is untouched");
}

/// ⌨️ An override carrying no parsable chord is dropped rather than blanking the row — React's
/// `parseUiKeybindingOverrides` gate.
#[test]
fn a_blank_keybinding_override_keeps_the_declared_default() {
    let mut overrides = HashMap::new();
    overrides.insert("ui.find.toggle".to_string(), "  ,  ".to_string());
    let table = shell_shortcut_table(&overrides);
    let row = table.iter().find(|(id, _)| id == "ui.find.toggle").expect("the find row");
    assert_eq!(row.1, "mod+f", "⌨️ a chordless override never retires the default");
}

/// ⌨️ The conflict marker is React's exactly: last writer of a first chord owns it, so the EARLIER row
/// of a colliding pair is the flagged one and the later is clean.
#[test]
fn keybinding_rows_flag_the_earlier_owner_of_a_shared_chord() {
    let mut overrides = HashMap::new();
    overrides.insert("ui.find.toggle".to_string(), "mod+p".to_string());
    let rows = shell_keybinding_rows(&overrides);
    let conflicted: Vec<&str> = rows.iter().filter(|row| row.conflicted).map(|row| row.control_id.as_str()).collect();
    assert_eq!(conflicted.len(), 1, "⌨️ exactly one of a colliding pair is marked, never both: {conflicted:?}");
    assert!(rows.iter().any(|row| row.control_id == "ui.find.toggle" && row.overridden), "⌨️ the remapped row reports itself overridden");
    assert!(rows.windows(2).all(|pair| pair[0].control_id <= pair[1].control_id), "⌨️ rows are control-id sorted, React's own `localeCompare` order");
}

/// ⌨️ An armed capture turns the next key event into that row's override and consumes it — the chord
/// spelling is React's `chordFromKeyboardEvent` order (`ctrl, meta, alt, shift`, then the key).
#[test]
fn an_armed_capture_writes_the_next_chord_as_an_override() {
    let mut shell = fresh_state();
    shell.input.keybinding_capture_control_id = Some("ui.nav.up".into());
    let modifiers = PointerModifiers { ctrl: true, alt: true, shift: true, ..PointerModifiers::default() };
    assert!(shell.input.capture_keybinding_chord(&semio_framework_ui_wgpu_input::KeyAction::ArrowUp, &modifiers, &mut shell.chrome), "⌨️ the capture consumes the event");
    assert_eq!(shell.chrome.chrome_build.preferences.keybinding_overrides.get("ui.nav.up").map(String::as_str), Some("ctrl+alt+shift+arrowup"));
    assert_eq!(shell.input.keybinding_capture_control_id, None, "⌨️ one gesture, one write, then disarmed");
    shell.input.keybinding_capture_control_id = Some("ui.nav.back".into());
    assert!(shell.input.capture_keybinding_chord(&semio_framework_ui_wgpu_input::KeyAction::Escape, &modifiers, &mut shell.chrome), "⌨️ Escape cancels and still consumes");
    assert!(!shell.chrome.chrome_build.preferences.keybinding_overrides.contains_key("ui.nav.back"), "⌨️ a cancelled capture writes nothing");
}

/// ⌨️ Every shortcut row the Keybindings leaf paints is addressable and every chord it prints is the
/// one in force — the rows W1c's shortcut list and this leaf must agree on.
#[test]
fn the_keybindings_leaf_paints_one_row_per_shortcut() {
    let mut shell = fresh_state();
    shell.chrome.chrome_build.preferences.keybinding_overrides.insert("ui.search.toggle".into(), "ctrl+j".into());
    let UiNode::Stack(panel) = shell.input.build_settings_keybindings_ui(&shell.chrome, &shell.prefs) else { panic!("expected a stack root") };
    let UiNode::Section(section) = panel.children.first().expect("the keybindings section") else { panic!("expected a section") };
    assert_eq!(section.children.len(), SHELL_SHORTCUT_ROWS.len(), "⌨️ one row per declared shortcut");
    let ids: Vec<String> = section.children.iter().filter_map(|node| if let UiNode::Stack(row) = node { row.id.clone() } else { None }).collect();
    assert!(ids.contains(&"framework.settings.keybindings.ui.search.toggle".to_string()));
    let overridden_row = section.children.iter().find_map(|node| if let UiNode::Stack(row) = node { (row.id.as_deref() == Some("framework.settings.keybindings.ui.search.toggle")).then_some(row) } else { None }).expect("the remapped row");
    assert_eq!(overridden_row.children.iter().filter(|node| matches!(node, UiNode::Button(_))).count(), 2, "⌨️ a remapped row offers capture AND reset");
}
//#endregion ⌨️RemappableKeybindings

//#region 📐️ContentHug
/// 📐️ A measured panel takes exactly its content's height from its own bonded edge, and never more
/// than React's `calc(100% - spacing*2)` clamp — the divergence `📓️w1h`'s gap 1 recorded.
#[test]
fn a_panel_hugs_its_measured_content_up_to_the_react_clamp() {
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let inset = theme.panel_inset;
    let band = body.h - inset * 2.0;
    let top = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, Some(180.0), 1, 0, body, &theme);
    assert_eq!(top.h, 180.0, "📐️ a top anchor takes its content's height");
    assert_eq!(top.y, body.y + inset, "📐️ and grows DOWN from its own inset");
    let bottom = anchor_panel_rect(PanelAnchor::BottomLeft, DEFAULT_PANEL_WIDTH_PX, Some(180.0), 1, 0, body, &theme);
    assert_eq!(bottom.h, 180.0);
    assert_eq!(bottom.y + bottom.h, body.y + inset + band, "📐️ a bottom anchor grows UP from the body's bottom inset");
    let middle = anchor_panel_rect(PanelAnchor::LeftMiddle, DEFAULT_PANEL_WIDTH_PX, Some(180.0), 1, 0, body, &theme);
    assert!((middle.y + middle.h * 0.5 - (body.y + inset + band * 0.5)).abs() < 0.01, "📐️ a middle anchor centres on its band");
    let oversized = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, Some(5000.0), 1, 0, body, &theme);
    assert_eq!(oversized.h, band, "📐️ the clamp is React's own maxHeight, never the content");
    let unmeasured = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, None, 1, 0, body, &theme);
    assert_eq!(unmeasured.h, band, "📐️ an unmeasured surface keeps its full band rather than collapsing");
}

/// 📐️ Two hugging anchors in one column still cannot overlap: each is clamped to its own share.
#[test]
fn two_hugging_anchors_in_one_column_never_overlap() {
    let theme = Theme::default();
    let body = Rect::new(0.0, 40.0, 1200.0, 700.0);
    let top = anchor_panel_rect(PanelAnchor::TopLeft, DEFAULT_PANEL_WIDTH_PX, Some(5000.0), 2, 0, body, &theme);
    let bottom = anchor_panel_rect(PanelAnchor::BottomLeft, DEFAULT_PANEL_WIDTH_PX, Some(5000.0), 2, 1, body, &theme);
    assert!(top.y + top.h <= bottom.y + 0.01, "📐️ {top:?} must not reach into {bottom:?}");
}
//#endregion 📐️ContentHug

//#region 📱️MobileFlattening
/// 📱️ Below React's `UI_MOBILE_MEDIA_QUERY` breakpoint the eight anchors flatten into ONE panel whose
/// tab list is every anchor's root tabs in `ANCHORS` order — React's `mobilePanelTabs`, which flattens
/// `defaultDock` and appends the synthetic app leaf only when the navbar has clusters with no room.
#[test]
fn the_mobile_panel_flattens_every_anchor_in_anchor_order() {
    let fixture = default_dock_fixture();
    let mut shell = fixture_dock_shell();
    shell.chrome.screen_w = 1280.0;
    assert!(!shell.chrome.mobile_panel_active(), "📱️ a desktop viewport keeps the eight anchors");
    shell.chrome.screen_w = semio_framework_os_renderer_wgpu_dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX;
    assert!(shell.chrome.mobile_panel_active(), "📱️ the breakpoint itself is mobile (React's `max-width: 767px` is inclusive)");
    let ids: Vec<String> = SessionCtx::of(&mut shell).mobile_panel_tabs().into_iter().map(|tab| tab.id).collect();
    let expected: Vec<String> = fixture["mobilePanelTabs"]["ids"].as_array().expect("fixture mobile ids").iter().map(|id| id.as_str().expect("id").to_string()).collect();
    assert_eq!(ids, expected, "📱️ the flattened order is the fixture's");
}

/// 📱️ The mobile panel's remembered path reconciles onto the flattened list and drills a branch to its
/// first leaf, so selecting the Settings branch on mobile lands on a real body.
#[test]
fn the_mobile_panel_path_drills_a_branch_to_its_first_leaf() {
    let shell = fixture_dock_shell();
    let tabs = SessionCtx::of(&mut shell).mobile_panel_tabs();
    let path = mobile_panel_reconcile_path(&tabs, &[FRAMEWORK_SETTINGS_PANEL_ID.to_string()]);
    assert_eq!(path.first().map(String::as_str), Some(FRAMEWORK_SETTINGS_PANEL_ID));
    assert_eq!(path.last().map(String::as_str), Some("fixture.settings"), "📱️ drilled to the branch's first leaf");
    let vanished = mobile_panel_reconcile_path(&tabs, &["gone".to_string()]);
    assert_eq!(vanished.first().map(String::as_str), tabs.first().map(|tab| tab.id.as_str()), "📱️ a vanished path falls back to the first tab");
}
//#endregion 📱️MobileFlattening
