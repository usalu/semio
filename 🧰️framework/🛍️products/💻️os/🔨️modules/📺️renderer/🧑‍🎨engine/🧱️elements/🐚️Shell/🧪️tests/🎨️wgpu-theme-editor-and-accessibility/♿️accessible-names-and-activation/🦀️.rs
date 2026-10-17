/// ♿️ Accessible names and activation: chrome control names and shortcuts, pressed buttons on actual hit ids, focusable field labels and the accessibility and pointer activation routes.

/// ⚖️ LAW (§C4): the shell CHROME publishes accessible names — the production path that gives a GPU
/// canvas's navbar/footer/panel chips ARIA elements through `🚀️browser-boot/🟦️.ts`'s mirror. Names
/// are the painted text; a control with a chord also publishes its `aria-keyshortcuts`.
#[test]
fn chrome_controls_publish_accessible_names_and_shortcuts() {
    let shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    with_chrome_control_names(|names| {
        names.clear();
    });
    note_chrome_control_name("playground.navbar.roles.editor", Some("Editor"));
    note_chrome_control_name("", Some("an id-less chip is not a control"));
    let hits = vec![
        HitTarget { rect: Rect::new(1.0, 2.0, 3.0, 4.0), event: None, control_id: Some("playground.navbar.roles.editor".to_string()), kind: HitKind::NavbarItem, drag_axis: None, drag_data: None },
        HitTarget { rect: Rect::new(0.0, 0.0, 1.0, 1.0), event: None, control_id: Some("framework.panelTab.settings.general".to_string()), kind: HitKind::PanelTab, drag_axis: None, drag_data: None },
    ];
    let published = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&hits);
    let nodes: Vec<_> = hits.iter().map(|hit| published.iter().find(|node| Some(node.key.as_str()) == hit.control_id.as_deref()).expect("registered chrome target is announced")).collect();
    assert_eq!(nodes[0].label.as_deref(), Some("Editor"), "♿️ the announced name is the text the chip painted");
    assert_eq!(nodes[0].role, "button", "♿️ a navbar item announces as a button, as React's element does");
    assert_eq!(nodes[0].shortcut.as_deref(), Some(format_keybinding_shortcut("mod+alt+e").as_str()), "⌨️ `aria-keyshortcuts` comes from this session's own remappable table");
    assert_eq!(nodes[1].role, "button", "♿️ PanelTabBar publishes pressed buttons");
    assert_eq!(nodes[1].pressed, Some(false));
    assert!(nodes[1].selected.is_none());
    assert!(nodes[1].label.is_some(), "♿️ a control that painted no label still gets a name rather than none");
    assert!(nodes.iter().all(|node| node.actionable && node.focusable), "♿️ a chrome target is always actionable");
}

/// 🔘️ Fullscreen and utility chrome publish React Toggle's button/pressed semantics on the exact
/// hit ids their retained painters register; plain utility actions keep labels without pressed state.
#[test]
fn fullscreen_and_utility_painters_publish_reacts_pressed_buttons_on_their_actual_hit_ids() {
    let fixture = accessibility_fixture();
    let semantics = &fixture["chromePressedSemantics"];
    let action = ActionDescriptor { controller_id: "fixture".into(), action: "activate".into(), args: None };
    let utilities = vec![
        semio_framework_ui_layout::utilities::utility_button("save", "circle".into(), semantics["utility"]["button"]["label"].as_str().expect("button label"), action.clone()),
        semio_framework_ui_layout::utilities::utility_toggle("grid", "circle".into(), semantics["utility"]["toggle"]["label"].as_str().expect("toggle label"), semantics["utility"]["toggle"]["pressed"].as_bool().expect("toggle pressed"), action),
        semio_framework_ui_layout::utilities::utility_collection("views", "circle".into(), semantics["utility"]["collection"]["label"].as_str().expect("collection label"), vec![]),
    ];
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let mut chrome = ShellChromeBuildState::default();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::light();
    with_chrome_control_names(|names| names.clear());

    let fullscreen = &semantics["fullscreen"];
    let fullscreen_item = ChromeGroupItem {
        control_id: fullscreen["id"].as_str().expect("fullscreen id"),
        icon_id: Some("minimize-2"),
        label: Some(fullscreen["label"].as_str().expect("fullscreen label")),
        active: fullscreen["pressed"].as_bool().expect("fullscreen pressed"),
        disabled: false,
        kind: HitKind::Toggle,
    };
    let fullscreen_rect = Rect::new(0.0, 0.0, retained_chrome_group_item_width(&mut atlas, &theme, &fullscreen_item).expect("fullscreen width"), theme.control_height);
    let mut fullscreen_phase = 0;
    let mut fullscreen_glyph = RetainedGlyphCursor::default();
    loop {
        match render_retained_chrome_group_item_step(&mut fullscreen_phase, &mut fullscreen_glyph, &mut draw, &mut atlas, &icons, &mut input, &theme, fullscreen_rect, &fullscreen_item, true) {
            RetainedChromeGroupStep::Pending => {}
            RetainedChromeGroupStep::Complete => break,
            RetainedChromeGroupStep::Fault => panic!("fullscreen paint"),
        }
    }

    let mut x = fullscreen_rect.w + theme.gap_standard;
    let mut cursor = ShellChromeChildCursor::default();
    for utility in &utilities {
        loop {
            match render_utility_node_step(&mut cursor, &mut chrome, &mut draw, &mut atlas, &icons, &mut input, &theme, x, 0.0, theme.control_height, utility, &HashMap::new()).expect("utility paint") {
                Some((next_x, _)) => {
                    x = next_x;
                    break;
                }
                None => {}
            }
        }
    }

    let nodes = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.staged_hits());
    for row in [fullscreen, &semantics["utility"]["button"], &semantics["utility"]["toggle"], &semantics["utility"]["collection"]] {
        let id = row["id"].as_str().expect("control id");
        let node = nodes.iter().find(|node| node.key == id).unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!(node.role, "button", "{id}");
        assert_eq!(node.label.as_deref(), row["label"].as_str(), "{id}");
        assert_eq!(node.pressed, row.get("pressed").and_then(Value::as_bool), "{id}");
        assert!(node.checked.is_none(), "{id} uses aria-pressed rather than aria-checked");
    }
}

#[test]
fn field_and_engagement_labels_reach_focusable_child_controls() {
    let action = ActionDescriptor { controller_id: "fixture".into(), action: "setIterations".into(), args: None };
    let field = UiNode::Field(UiFieldNode {
        id: "settings.iterations.field".into(),
        label: Label::data("Iterations"),
        description: Some("Whole-number iterations".into()),
        required: None,
        error: None,
        child: Box::new(UiNode::NumberStepper(UiNumberStepperNode {
            id: "settings.iterations".into(),
            value: 3.0,
            step: 1.0,
            uniform: false,
            min: None,
            max: None,
            precision: None,
            on_absolute: action.clone(),
            on_delta: action,
            presence: UiPresence::default(),
            menu: None,
            ..Default::default()
        })),
        presence: UiPresence::default(),
        menu: None,
    });
    let records = panel_ui_records("settings", &field).expect("field projection");
    let child = records.iter().find(|record| record.key.as_str() == "settings/settings.iterations").expect("field child");
    let projected = ui_contract::accessibility_projection_node(child, 1);
    assert_eq!(projected.role, "spinbutton");
    assert_eq!(projected.label.as_deref(), Some("Iterations"));
    assert_eq!(projected.description.as_deref(), Some("Whole-number iterations"));

    let rows =
        engagement_control_rows(&semio_framework_ui_layout::layout::WindowEngagementControl::Stepper { id: None, label: Some("Count".into()), value: 3.0, min: None, max: None, step: Some(1.0), unit: None, disabled: None, on_change: None, on_commit: None }, false);
    assert_eq!(rows.len(), 1, "the semantic field owns the visible label and control");
    let records = panel_ui_records("engagement", &rows[0]).expect("engagement projection");
    let child = records.iter().find(|record| record.key.as_str() == "engagement/engagement-control.stepper").expect("engagement stepper");
    assert_eq!(ui_contract::accessibility_projection_node(child, 1).label.as_deref(), Some("Count"));
}

#[test]
fn accessibility_activation_updates_settings_pressed_button_projection() {
    let fixture = accessibility_fixture();
    let selection = &fixture["nestedPanelSelection"];
    let mut shell = shell_with_nested_app_settings();
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).path.clear();
    let mut input = InputState::default();
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 1.0, 1.0), event: None, control_id: Some(FRAMEWORK_SETTINGS_PANEL_ID.into()), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let initial = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits()).into_iter().next().expect("settings projection");
    assert_eq!(initial.pressed, Some(false));
    assert_eq!(initial.role, "button");
    assert!(initial.checked.is_none());
    let target = ui_render::AccessibilityTarget { window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: shell.chrome.presented_chrome_accessibility_generation, node_id: initial.node_id, node_key: initial.key };
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("settings activation"));
    assert_eq!(PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits())[0].pressed, Some(true), "the Settings branch button reflects its now-visible anchor");
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).visible = false;
    assert_eq!(PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits())[0].pressed, fixture["settingsTabStrip"]["pressedFollowsSelectionWhenFolded"].as_bool(), "folding removes the active fill while retaining React's pressed selection");

    assert_eq!(shell.dock.reveal_dock_tab(FRAMEWORK_SETTINGS_THEME_TAB_ID), Some(PanelAnchor::BottomRight));
    let mut input = InputState::default();
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 1.0, 1.0), event: None, control_id: Some(FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into()), kind: HitKind::PanelTab, drag_axis: None, drag_data: None });
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let general = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits()).into_iter().next().expect("general projection");
    assert_eq!(general.pressed, Some(false));
    assert!(general.selected.is_none());
    let target = ui_render::AccessibilityTarget { window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: shell.chrome.presented_chrome_accessibility_generation, node_id: general.node_id, node_key: general.key };
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("general activation"));
    assert_eq!(PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits())[0].pressed, Some(true), "the activated General leaf publishes aria-pressed=true");
    let expected: Vec<&str> = selection["expectedPath"].as_array().expect("expected path").iter().map(|id| id.as_str().expect("path id")).collect();
    assert_eq!(shell.dock.anchor_state(PanelAnchor::BottomRight).path.iter().map(String::as_str).collect::<Vec<_>>(), expected);
    assert_ne!(shell.dock.anchor_state(PanelAnchor::BottomRight).active_tab(), selection["appFirstLeafId"].as_str());
    let records = panel_ui_records(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, &shell.docs.build_settings_general_ui(&shell.chrome, &shell.prefs)).expect("General retained projection");
    let required = selection["requiredControlId"].as_str().expect("required control id");
    assert!(records.iter().any(|record| record.key.as_str().ends_with(required)), "the selected General document publishes Appearance");
    drop(records);
    while !ui_contract::close_ui_value_page_one() {}
}

#[test]
fn pointer_activation_resolves_every_nested_leaf_from_root_to_target_independent_of_panel_rect() {
    let fixture = accessibility_fixture();
    let selection = &fixture["nestedPanelSelection"];
    let expected: Vec<&str> = selection["expectedPath"].as_array().expect("expected path").iter().map(|id| id.as_str().expect("path id")).collect();
    for rect in selection["pointerRects"].as_array().expect("pointer rectangles") {
        let mut shell = shell_with_nested_app_settings();
        let x = rect["x"].as_f64().expect("x") as f32;
        let y = rect["y"].as_f64().expect("y") as f32;
        let width = rect["width"].as_f64().expect("width") as f32;
        let height = rect["height"].as_f64().expect("height") as f32;
        let mut input = InputState::default();
        input.register_hit(HitTarget { rect: Rect::new(x, y, width, height), event: None, control_id: Some(selection["targetLeafId"].as_str().expect("target leaf").into()), kind: HitKind::PanelTab, drag_axis: None, drag_data: None });
        input.publish_hits();
        semio_framework_async::block_on(ChromeViewCtx::of(&mut shell).handle_pointer_button(x + width * 0.5, y + height * 0.5, true, 0, &mut input, &Theme::default())).expect("nested panel pointer activation");
        assert_eq!(shell.dock.anchor_state(PanelAnchor::BottomRight).path.iter().map(String::as_str).collect::<Vec<_>>(), expected, "{} panel rectangle", rect["id"].as_str().expect("rectangle id"));
        assert_eq!(shell.dock.anchor_state(PanelAnchor::BottomRight).active_tab(), selection["targetLeafId"].as_str());
        assert_ne!(shell.dock.anchor_state(PanelAnchor::BottomRight).active_tab(), selection["appFirstLeafId"].as_str());
    }
}

#[test]
fn chrome_accessibility_dispatch_validates_current_identity_and_activates_once() {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let mut input = InputState::default();
    input.register_hit(HitTarget { rect: Rect::new(1.0, 2.0, 3.0, 4.0), event: None, control_id: Some("ui.search.toggle".to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let node = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits()).into_iter().next().expect("search projection");
    let target =
        ui_render::AccessibilityTarget { window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: shell.chrome.presented_chrome_accessibility_generation, node_id: node.node_id, node_key: node.key.clone() };

    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).expect("focus dispatch"));
    assert_eq!(PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits())[0].focused, true);
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("activation dispatch"));
    assert!(shell.commands.search_open, "the real chrome handler toggles search once");

    let stale = ui_render::AccessibilityTarget { window_generation: target.window_generation + 1, ..target.clone() };
    assert!(!semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&stale, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("stale dispatch"));
    assert!(shell.commands.search_open, "a stale generation cannot toggle the live control");
    let reused = ui_render::AccessibilityTarget { node_key: "ui.find.toggle".to_string(), ..target };
    assert!(!semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&reused, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("reused identity dispatch"));
    assert!(shell.commands.search_open, "a mismatched node id and key cannot activate another control");
}
