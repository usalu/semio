/// 🧭️ Settings strip and keybindings: the constrained tab strip, mobile panel buttons, inline chords, the flagged keybinding rows and accepted dock names.

#[test]
fn a_constrained_settings_strip_retains_all_semantic_tabs_and_reveals_an_accessibility_selected_tail() {
    let fixture = accessibility_fixture();
    let contract = &fixture["settingsTabStrip"];
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let anchor = PanelAnchor::BottomRight;
    let tabs = contract["tabs"].as_array().expect("Settings tabs").iter().enumerate().map(|(order, tab)| DockTabNode::leaf(tab["id"].as_str().unwrap(), tab["label"].as_str().unwrap(), "settings", order as i32)).collect::<Vec<_>>();
    shell.dock.dock_tabs.tabs_mut(anchor).push(DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, "Settings", "settings", 0, tabs));
    shell.dock.anchor_state_mut(anchor).path = vec![FRAMEWORK_SETTINGS_PANEL_ID.into(), contract["activeId"].as_str().unwrap().into()];
    shell.dock.anchor_state_mut(anchor).visible = true;
    let theme = Theme::default();
    let panel = Rect::new(0.0, 0.0, contract["availableWidth"].as_f64().unwrap() as f32, 240.0);
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    ChromeViewCtx::of(&mut shell).paint_anchor_tab_bar(anchor, &mut draw, &mut atlas, &icons, &mut input, &theme, panel);
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);

    let expected = contract["tabs"].as_array().unwrap().iter().map(|tab| tab["id"].as_str().unwrap()).collect::<Vec<_>>();
    let pointer_ids = input.hits().iter().filter(|hit| hit.kind == HitKind::PanelTab).filter_map(|hit| hit.control_id.as_deref()).collect::<Vec<_>>();
    assert!(pointer_ids.len() < expected.len(), "the constrained pointer row exposes only nonempty clipped chips");
    let semantic = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits());
    for id in &expected {
        let node = semantic.iter().find(|node| node.key == *id).expect("every declared Settings control survives pointer clipping");
        assert_eq!(node.role, contract["role"].as_str().unwrap());
        let wire = serde_json::to_value(node).unwrap();
        assert_eq!(wire["pressed"].as_bool(), Some(*id == contract["activeId"].as_str().unwrap()));
        assert!(node.selected.is_none());
        assert!(node.checked.is_none());
    }

    let tail_id = contract["tailIds"][1].as_str().unwrap();
    let tail = semantic.iter().find(|node| node.key == tail_id).expect("semantic tail tab");
    let target =
        ui_render::AccessibilityTarget { window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: shell.chrome.presented_chrome_accessibility_generation, node_id: tail.node_id, node_key: tail.key.clone() };
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("tail tab activation"));
    assert_eq!(shell.dock.anchor_state(anchor).active_tab(), Some(tail_id));

    while input.retire_hit_step() {}
    let mut draw = DrawList::default();
    ChromeViewCtx::of(&mut shell).paint_anchor_tab_bar(anchor, &mut draw, &mut atlas, &icons, &mut input, &theme, panel);
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    assert!(input.hits().iter().any(|hit| hit.kind == HitKind::PanelTab && hit.control_id.as_deref() == Some(tail_id)), "the next accepted pointer row reveals the accessibility-selected tail");
}

#[test]
fn mobile_panel_buttons_publish_their_painted_names_and_pressed_state_once() {
    let fixture = accessibility_fixture();
    let contract = &fixture["settingsTabStrip"];
    let mut shell = settings_toggle_shell();
    shell.chrome.screen_w = 390.0;
    shell.dock.mobile_panel_visible = true;
    shell.dock.mobile_panel_path = vec![contract["activeId"].as_str().unwrap().into()];
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).visible = true;
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).path = vec![FRAMEWORK_SETTINGS_PANEL_ID.into(), FRAMEWORK_SETTINGS_GENERAL_TAB_ID.into()];
    let tabs = contract["tabs"].as_array().unwrap().iter().take(2).enumerate().map(|(order, row)| DockTabNode::leaf(row["id"].as_str().unwrap(), row["label"].as_str().unwrap(), "settings", order as i32)).collect::<Vec<_>>();
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::default();
    ChromeViewCtx::of(&mut shell).paint_mobile_tab_bar(&mut draw, &mut atlas, &IconAtlas::default(), &mut input, &Theme::default(), Rect::new(0.0, 0.0, 390.0, 40.0), &tabs);
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let nodes = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits());
    for tab in tabs {
        let key = format!("shell.panel.tab.mobile.{}", tab.id);
        let matching = nodes.iter().filter(|node| node.key == key).collect::<Vec<_>>();
        assert_eq!(matching.len(), 1);
        let node = matching[0];
        assert_eq!(node.label.as_deref(), Some(tab.label.as_str()));
        assert_eq!(node.role, contract["role"].as_str().unwrap());
        assert_eq!(node.pressed, Some(tab.id == contract["activeId"].as_str().unwrap()));
        assert!(node.checked.is_none() && node.selected.is_none());
        assert!(!nodes.iter().any(|node| node.key == tab.id), "desktop catalogue entries are not mounted in the mobile panel");
    }
}

/// ⚖️ LAW (§B14): a bound chrome control carries its chord INLINE, like React's `ControlHotkeyBadge`
/// beside any bound control's own text — and appending it twice is not a thing that can happen.
#[test]
fn a_bound_chrome_control_carries_its_chord_inline_exactly_once() {
    let shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let badged = shell.chrome.chrome_control_label("playground.navbar.roles.editor", "Editor");
    assert!(badged.starts_with("Editor "), "⌨️ the badge follows the label, as React's `ms-auto` span does");
    assert_eq!(shell.chrome.chrome_control_label("playground.navbar.roles.editor", &badged), badged, "⌨️ idempotent: a builder that already badged its own label is left alone");
    assert_eq!(shell.chrome.chrome_control_label("playground.navbar.fixture", "Concrete Forest"), "Concrete Forest", "⌨️ an unbound control carries no badge, as `useControlHotkey` answers `undefined`");
}

/// ⚖️ LAW (§E2/§E3): full-screen belongs to the platform-scoped OS command registry, while the
/// framework-universal history aliases stay in the shell tail.
#[test]
fn the_flagged_keybinding_rows_match_reacts_registry() {
    let shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let fullscreen = shell.docs.build_os_commands(&shell.chrome).into_iter().find(|command| command.id == "os.toggleFullscreen").expect("fullscreen OS command");
    assert!(fullscreen.keybindings.iter().any(|binding| binding.chord == "control+meta+f" && binding.platform == Some(semio_framework::manifest::Platform::MacOs)));
    assert_eq!(fullscreen.keybindings.iter().filter(|binding| binding.chord == "f11").count(), 2);
    assert!(!SHELL_SHORTCUT_ROWS.iter().any(|(id, _)| *id == "os.toggleFullscreen"), "the control-badge fallback is not an executable shell shortcut");
    for (chord, expected) in [("mod+z", ShellEditVerb::Undo), ("mod+shift+z", ShellEditVerb::Redo), ("mod+y", ShellEditVerb::Redo)] {
        let mut modifiers = PointerModifiers::default();
        let mut key = "";
        for token in chord.split('+') {
            match token {
                "mod" | "meta" => modifiers.meta = true,
                "ctrl" => modifiers.ctrl = true,
                "shift" => modifiers.shift = true,
                "alt" => modifiers.alt = true,
                other => key = other,
            }
        }
        let action = semio_framework_ui_wgpu_input::KeyAction::Char(key.to_string());
        assert_eq!(shell_edit_verb_for(&action, &modifiers), Some(expected), "⌨️ {chord} is React's framework-universal history tail");
    }
}

#[test]
fn accepted_dock_names_match_painted_instance_titles_and_localized_react_actions() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🪟️dock-accessible-names/🔣️.json")).expect("dock names fixture");
    for locale in fixture["locales"].as_array().expect("locales") {
        for layout in ["split", "maximized", "stacked"] {
            let mut shell = split_pane_shell();
            let maximized = layout == "maximized";
            let stacked = layout == "stacked";
            let stacked_active = fixture["stackedActiveId"].as_str().unwrap();
            shell.prefs.locale_id = locale["id"].as_str().unwrap().into();
            shell.chrome.screen_w = 1280.0;
            shell.chrome.screen_h = 720.0;
            if stacked {
                shell.dock.dock.root = semio_framework_os_renderer_wgpu_dock::DockNode::Stack {
                    windows: shell.dock.dock.window_instances().into_iter().map(|(id, kind)| semio_framework_os_renderer_wgpu_dock::DockStackTab::instance(id, kind, semio_framework_ui_layout::layout::WindowStackCorner::TopLeft)).collect(),
                    active: stacked_active.into(),
                };
            }
            if maximized {
                shell.dock.dock.toggle_maximize(&vec![0]);
            }
            with_chrome_control_names(|names| names.clear());
            let mut input = InputState::<ActionDescriptor>::default();
            let mut draw = DrawList::default();
            let mut atlas = FontAtlas::builtin();
            let icons = IconAtlas::default();
            let theme = Theme::light();
            let bounds = Rect::new(0.0, 0.0, 1280.0, 720.0);
            let mut overlay = None;
            let mut world = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
            let mut cursor = ShellChromeChildCursor { phase: 1, rect: Some(bounds), ..Default::default() };
            caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, bounds, &mut world, cx));
            caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, bounds, &mut world, cx));
            for index in 0..shell.dock.dock_window_plan.len() {
                cursor.item = index;
                cursor.phase = 3;
                caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, bounds, &mut world, cx));
            }
            cursor.phase = 5;
            caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, bounds, &mut world, cx));
            assert!(shell.chrome.presented_chrome_accessibility.is_empty());
            PanelsViewCtx::of(&mut shell).publish_retained_input_for_test(&mut input, &theme);
            let published = shell.chrome.presented_chrome_accessibility.clone();
            for window in fixture["windows"].as_array().unwrap().iter().take(if maximized { 1 } else { 2 }) {
                let id = window["id"].as_str().unwrap();
                let title = window["title"].as_str().unwrap();
                let body = published.iter().find(|node| node.key == id);
                if !stacked || id == stacked_active {
                    let body = body.expect("published body");
                    assert_eq!(body.label.as_deref(), Some(title));
                    assert_eq!(body.role, fixture["semantics"]["panelRole"].as_str().unwrap());
                    assert_eq!(body.actionable, fixture["semantics"]["panelActionable"].as_bool().unwrap());
                } else {
                    assert!(body.is_none(), "inactive tabs share the mounted stack panel");
                }
                let select = published.iter().find(|node| node.key.starts_with("dock.tab.") && node.key.ends_with(id)).expect("published tab");
                assert_eq!(select.label.as_deref(), Some(title));
                assert_eq!(select.role, fixture["semantics"]["tabRole"].as_str().unwrap());
                assert_eq!(select.selected, Some(!stacked || id == stacked_active));
                assert_eq!(select.controls.as_deref(), Some(if stacked { stacked_active } else { id }));
                for action in ["focus", "close", "drag"] {
                    let key = format!("{}.{}", select.key, action);
                    let expected = locale[if action == "focus" && maximized { "unfocus" } else { action }].as_str().unwrap().replace("{{target}}", title);
                    let control = published.iter().find(|node| node.key == key).expect("published dock action");
                    assert_eq!(control.label.as_deref(), Some(expected.as_str()), "{}:{key}", shell.prefs.locale_id);
                }
            }
            shell.prefs.locale_id = "candidate-only".into();
            with_chrome_control_names(|names| names.clear());
            assert_eq!(shell.chrome.presented_chrome_accessibility, published, "unaccepted candidate cannot relabel the displayed frame");
            caller.retire_world(&mut world);
        }
    }
    caller.finish();
}

#[test]
fn a_discarded_chrome_walk_cannot_exhaust_successor_accessible_names() {
    let mut caller=semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let theme = Theme::light();
    let mut input = InputState::<ActionDescriptor>::default();
    with_chrome_control_names(|names| names.clear());
    note_chrome_control_name("accepted", Some("Accepted label"));
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 20.0, 20.0), event: None, control_id: Some("accepted".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    PanelsViewCtx::of(&mut shell).publish_retained_input_for_test(&mut input, &theme);
    let accepted = shell.chrome.presented_chrome_accessibility.clone();
    let accepted_generation = shell.chrome.presented_chrome_accessibility_generation;
    for index in 0..SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY {
        note_chrome_control_name(&format!("discarded.{index}"), Some("Discarded label"));
    }
    while input.retire_hit_step() {}
    input.register_hit(HitTarget { rect: Rect::new(30.0, 0.0, 20.0, 20.0), event: None, control_id: Some("discarded.0".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    let discarded = ChromeViewCtx::of(&mut shell).seal_presented_input_candidate(&theme).expect("sealed chrome candidate");
    assert_eq!(shell.chrome.presented_chrome_accessibility, accepted);
    assert!(shell.input.discard_presented_input_candidate(discarded));
    assert!(!PanelsViewCtx::of(&mut shell).acknowledge_presented_input(&mut input, discarded), "late acknowledgement cannot publish discarded chrome");
    assert_eq!(shell.chrome.presented_chrome_accessibility, accepted);
    assert_eq!(shell.chrome.presented_chrome_accessibility_generation, accepted_generation);
    assert_eq!(input.hit_at(10.0, 10.0).and_then(|hit| hit.control_id.as_deref()), Some("accepted"));
    assert!(input.hit_at(40.0, 10.0).is_none());
    let mut cursor = ShellChromeFrameCursor::default();
    cursor.phase = ShellChromeFramePhase::FrameSetup;
    let mut draw = DrawList::default();
    let mut overlay = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut world = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_chrome_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, &mut world, cx)).expect("original fixture caller turn");
    assert!(with_chrome_control_names(|names| names.is_empty()));
    assert_eq!(shell.chrome.presented_chrome_accessibility, accepted);
    note_chrome_control_name("successor", Some("Successor label"));
    while input.retire_hit_step() {}
    input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, 20.0, 20.0), event: None, control_id: Some("successor".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    PanelsViewCtx::of(&mut shell).publish_retained_input_for_test(&mut input, &theme);
    let current = &shell.chrome.presented_chrome_accessibility;
    assert!(current.iter().any(|node| node.key == "successor" && node.label.as_deref() == Some("Successor label")));
    assert!(!current.iter().any(|node| node.key.starts_with("discarded.") || node.key == "accepted"));
    let successor = current.clone();
    let successor_generation = shell.chrome.presented_chrome_accessibility_generation;
    assert!(!PanelsViewCtx::of(&mut shell).acknowledge_presented_input(&mut input, discarded));
    assert_eq!(shell.chrome.presented_chrome_accessibility, successor);
    assert_eq!(shell.chrome.presented_chrome_accessibility_generation, successor_generation);
    caller.retire_world(&mut world);
    caller.finish();
}
