/// ⌨️ Retained pane focus and keyboard: concrete window pane actions, keyboard focus routing and successors, pane owner activation and the select and typeahead key mapper.

#[test]
fn retained_pane_actions_address_the_concrete_window_before_guest_admission() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture = pane_owner_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let owner = case["window"].as_str().unwrap();
        let surface = case["surface"].as_str().unwrap();
        let mut shell = host_test_shell();
        let controller_id = shell.docs.session.as_ref().unwrap().app.controller_id.clone();
        shell.plugins.plugins.clear();
        let result =
            semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(ActionDescriptor { controller_id, action: semio_framework::SET_ACTIVE_UTILITY_ACTION_ID.into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "windowId": surface, "utilityId": "owner-brush" }) }));
        assert_eq!(result.unwrap_err(), "action program missing", "fixture stops at guest admission");
        assert_eq!(shell.windows.active_utility_for_window(owner), Some("owner-brush"), "{} canonical host utility scope", case["kind"]);
        assert_eq!(shell.windows.active_utility_by_window.len(), 1, "{} one concrete owner", case["kind"]);
        if surface != owner {
            assert!(!shell.windows.active_utility_by_window.contains_key(surface), "{} pane identity never becomes an application window", case["kind"]);
        }
    }
}

fn pane_owner_select_records() -> Vec<ui_contract::UiNodeRecord> {
    pane_owner_select_records_with("pane-owner-select", "system")
}

fn pane_owner_select_records_with(key: &str, value: &str) -> Vec<ui_contract::UiNodeRecord> {
    serde_json::from_value(serde_json::json!([{
        "id": 1, "key": key,
        "component": { "type": "select", "value": value, "items": [{ "value": "system", "label": "System" }, { "value": "dark", "label": "Dark" }] },
        "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
        "style": {}, "activity": "idle", "accessibility": { "label": "Pane selection" },
        "bindings": [{ "trigger": "change", "action": { "scope": "framework", "name": "setAppearance", "version": 1 } }]
    }]))
    .unwrap()
}

fn pane_owner_input_records_with(key: &str) -> Vec<ui_contract::UiNodeRecord> {
    serde_json::from_value(serde_json::json!([{
        "id": 1, "key": key,
        "component": { "type": "input", "kind": "text", "value": "", "commit": "enter" },
        "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
        "style": {}, "activity": "idle", "accessibility": { "label": "Replacement input" },
        "bindings": [{ "trigger": "commit", "action": { "scope": "framework", "name": "setReplacementInput", "version": 1 } }]
    }]))
    .unwrap()
}

fn retained_focus_panel_shell(surface: &str) -> ShellState {
    let mut shell = host_test_shell();
    shell.dock.dock_tabs = ShellDock::default();
    shell.dock.panel_anchors = std::array::from_fn(|_| PanelAnchorState::default());
    shell.dock.dock_tabs.tabs_mut(PanelAnchor::BottomRight).push(DockTabNode::leaf(surface, "Preferences", "settings", 0));
    *shell.dock.anchor_state_mut(PanelAnchor::BottomRight) = PanelAnchorState { visible: true, size: 300.0, path: vec![surface.into()] };
    shell
}

#[test]
fn retained_panel_focus_routes_keyboard_without_activating_an_application_window() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture = pane_owner_fixture();
    let surface = fixture["panel"]["surface"].as_str().unwrap();
    let previous = fixture["previousWindow"].as_str().unwrap();
    let mut shell = retained_focus_panel_shell(surface);
    shell.windows.active_window_id = Some(previous.into());
    let mut document = shell.windows.publish_surface_records(surface, pane_owner_select_records()).unwrap();
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(surface, "pane-owner-select").expect("the published pane exposes its exact accessible address");
    let focused = semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap();
    let active = shell.windows.active_window_id.clone();
    let open_focus = SessionCtx::of(&mut shell).retained_keyboard_focus().map(|(surface, _)| surface);
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).visible = false;
    let hidden_focus = SessionCtx::of(&mut shell).retained_keyboard_focus();
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    let hidden_actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    shell.dock.anchor_state_mut(PanelAnchor::BottomRight).visible = true;
    assert!(SessionCtx::of(&mut shell).retained_keyboard_focus().is_some(), "the same visible retained owner recovers without a synthetic focus event");
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents();
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::publish_accessibility_visible_documents();
    let closed_focus = SessionCtx::of(&mut shell).retained_keyboard_focus();
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    let closed_actions = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    while !document.close_step() {}
    assert!(focused);
    assert_eq!(active.as_deref(), Some(previous));
    assert_eq!(open_focus.as_deref(), Some(surface));
    assert!(hidden_focus.is_none(), "a hidden panel cannot receive keyboard input");
    assert!(hidden_actions.is_empty(), "a hidden panel dispatches no retained keyboard action");
    assert!(closed_focus.is_none(), "a closed surface cannot receive keyboard input");
    assert!(closed_actions.is_empty(), "a closed surface dispatches no retained keyboard action");
}

#[test]
fn same_surface_keyed_successor_retains_keyboard_focus_without_a_synthetic_focus_event() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "focus-lifecycle-keyed-successor";
    let mut shell = retained_focus_panel_shell(surface);
    let mut initial = shell.windows.publish_surface_records(surface, pane_owner_select_records_with("stable-select", "system")).expect("initial keyed document publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &initial, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(surface, "stable-select").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    let before = SessionCtx::of(&mut shell).retained_keyboard_focus().expect("the initial keyed node owns keyboard focus");

    let mut successor = shell.windows.publish_surface_records(surface, pane_owner_select_records_with("stable-select", "dark")).expect("same-surface successor publishes");
    input = paint_tree_pointer_document(&mut shell, surface, &successor, Rect::new(0.0, 0.0, 320.0, 120.0));
    let after = SessionCtx::of(&mut shell).retained_keyboard_focus().expect("the keyed successor retains keyboard focus");
    assert_eq!(after, before, "key reconciliation preserves the exact focused node identity");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "the retained Select opens before commit");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert_eq!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);
    while !initial.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn successor_that_removes_the_focused_key_cannot_receive_retained_keyboard_input() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "focus-lifecycle-removed-key";
    let mut shell = retained_focus_panel_shell(surface);
    let mut initial = shell.windows.publish_surface_records(surface, pane_owner_select_records_with("retired-select", "system")).expect("initial removable document publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &initial, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(surface, "retired-select").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(SessionCtx::of(&mut shell).retained_keyboard_focus().is_some());

    let mut successor = shell.windows.publish_surface_records(surface, pane_owner_select_records_with("replacement-select", "dark")).expect("replacement document publishes");
    input = paint_tree_pointer_document(&mut shell, surface, &successor, Rect::new(0.0, 0.0, 320.0, 120.0));
    assert!(shell.chrome.chrome_build.content_has_focus(surface), "the shell receives no invented blur command");
    assert!(SessionCtx::of(&mut shell).retained_keyboard_focus().is_none(), "the removed focused node generation cannot route into its replacement");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "the replacement receives no keyboard action without its own focus event");
    while !initial.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn same_key_changed_control_kind_cannot_inherit_retained_keyboard_focus() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "focus-lifecycle-changed-kind";
    let mut shell = retained_focus_panel_shell(surface);
    let mut initial = shell.windows.publish_surface_records(surface, pane_owner_select_records_with("polymorphic-control", "system")).expect("initial Select document publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &initial, Rect::new(0.0, 0.0, 320.0, 120.0));
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(surface, "polymorphic-control").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(SessionCtx::of(&mut shell).retained_keyboard_focus().is_some());

    let mut successor = shell.windows.publish_surface_records(surface, pane_owner_input_records_with("polymorphic-control")).expect("same-key Input successor publishes");
    input = paint_tree_pointer_document(&mut shell, surface, &successor, Rect::new(0.0, 0.0, 320.0, 120.0));
    assert!(shell.chrome.chrome_build.content_has_focus(surface), "document reconciliation emits no synthetic blur");
    assert!(SessionCtx::of(&mut shell).retained_keyboard_focus().is_none(), "a same-key replacement control kind cannot inherit its predecessor's focus");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Enter, &PointerModifiers::default(), &mut input)).unwrap();
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "the replacement Input receives no Enter commit without its own focus event");
    while !initial.close_step() {}
    while !successor.close_step() {}
}

#[test]
fn retained_pane_focus_follows_the_last_focus_event_and_ignores_an_old_surface_blur() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture = pane_owner_fixture();
    let cases = fixture["cases"].as_array().unwrap();
    let mut shell = host_test_shell();
    let mut documents = Vec::new();
    for row in cases {
        let surface = row["surface"].as_str().unwrap();
        let document = shell.windows.publish_surface_records(surface, pane_owner_select_records()).unwrap();
        documents.push(document);
    }
    let paint_rows: Vec<_> = cases.iter().zip(documents.iter()).map(|(row, document)| (row["surface"].as_str().unwrap(), document, Rect::new(0.0, 0.0, 320.0, 120.0))).collect();
    let mut input = paint_tree_pointer_documents(&mut shell, &paint_rows);
    let mut observations = Vec::new();
    let mut previous: Option<String> = None;
    for kind in fixture["focusSequence"].as_array().unwrap() {
        let row = cases.iter().find(|row| &row["kind"] == kind).unwrap();
        let surface = row["surface"].as_str().unwrap();
        let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(surface, "pane-owner-select").expect("the published pane exposes its exact accessible address");
        let accepted = semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap();
        observations.push((accepted, shell.windows.active_window_id.clone(), SessionCtx::of(&mut shell).retained_keyboard_focus().map(|(surface, _)| surface), previous.as_ref().is_some_and(|old| shell.chrome.chrome_build.content_has_focus(old))));
        previous = Some(surface.into());
    }
    let blurred = cases.iter().find(|row| row["kind"] == fixture["lateBlur"]).unwrap()["surface"].as_str().unwrap();
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(blurred, "pane-owner-select").expect("the published pane exposes its exact accessible address");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Blur, &mut input)).unwrap();
    let final_focus = SessionCtx::of(&mut shell).retained_keyboard_focus().map(|(surface, _)| surface);
    for mut document in documents {
        while !document.close_step() {}
    }
    for (kind, (accepted, owner, focus, old_has_focus)) in fixture["focusSequence"].as_array().unwrap().iter().zip(observations) {
        let row = cases.iter().find(|row| &row["kind"] == kind).unwrap();
        assert!(accepted);
        assert_eq!(owner.as_deref(), row["window"].as_str(), "{kind} concrete owner");
        assert_eq!(focus.as_deref(), row["surface"].as_str(), "{kind} latest focused surface");
        assert!(!old_has_focus, "{kind} clears its previously focused sibling");
    }
    assert_eq!(final_focus, previous, "an old surface blur cannot clear the newest focused surface");
}

fn pane_owner_fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🪪️window-surface-owner/🔣️.json")).expect("window surface ownership fixture")
}

#[test]
fn retained_pane_pointer_activation_preserves_the_concrete_window_owner() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture = pane_owner_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        let owner = case["window"].as_str().unwrap();
        let surface = case["surface"].as_str().unwrap();
        let previous = fixture["previousWindow"].as_str().unwrap();
        shell.dock.dock.root = DockNode::Row(vec![(DockNode::Stack { windows: vec![DockStackTab::new(previous)], active: previous.into() }, 1.0), (DockNode::Stack { windows: vec![DockStackTab::new(owner)], active: owner.into() }, 1.0)]);
        shell.dock.dock.sync_active_window(previous);
        shell.windows.active_window_id = Some(previous.into());
        let mut input = InputState::<ActionDescriptor>::default();
        semio_framework_async::block_on(PointerCtx::of(&mut shell).route_retained_pointer_press(surface, Rect::new(0.0, 0.0, 320.0, 120.0), 15.0, 15.0, true, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).unwrap();
        assert_eq!(shell.windows.active_window_id.as_deref(), Some(owner), "{} shell owner", case["kind"]);
        assert_eq!(shell.dock.dock.active_window_id.as_deref(), Some(owner), "{} dock owner", case["kind"]);
        assert_eq!(shell.dock.dock.active_stack, Some(vec![1]), "{} active stack", case["kind"]);
    }
}

#[test]
fn retained_pane_accessibility_and_keyboard_focus_keep_surface_and_window_distinct() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let fixture = pane_owner_fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let owner = case["window"].as_str().unwrap();
        let surface = case["surface"].as_str().unwrap();
        let records = pane_owner_select_records();
        let mut shell = host_test_shell();
        let mut document = shell.windows.publish_surface_records(surface, records).expect("pane document publishes");
        let mut input = paint_tree_pointer_document(&mut shell, surface, &document, Rect::new(0.0, 0.0, 320.0, 120.0));
        shell.windows.active_window_id = Some(fixture["previousWindow"].as_str().unwrap().into());
        let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(surface, "pane-owner-select").expect("the published pane exposes its exact accessible address");
        let focused = semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap();
        let active = shell.windows.active_window_id.clone();
        let keyboard_surface = SessionCtx::of(&mut shell).retained_keyboard_focus().map(|(surface, _)| surface);
        let select_owns_keyboard = SessionCtx::of(&mut shell).retained_select_owns_keyboard();
        while !document.close_step() {}
        assert!(focused, "{} accessibility focus dispatch", case["kind"]);
        assert_eq!(active.as_deref(), Some(owner), "{} active window", case["kind"]);
        assert_eq!(keyboard_surface.as_deref(), Some(surface), "{} keyboard surface", case["kind"]);
        assert!(select_owns_keyboard, "{} Select keyboard ownership", case["kind"]);
    }
}

#[test]
fn retained_key_mapper_separates_select_typeahead_and_space_from_input_text() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let modifiers = PointerModifiers::default();
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("d".into()), &modifiers, true), Some(ui_wgpu::wgpu::UiEvent::KeyDown { key, .. }) if key == "d"));
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(true), &modifiers, true), Some(ui_wgpu::wgpu::UiEvent::KeyDown { key, .. }) if key == " "));
    assert!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(false), &modifiers, true).is_none());
    assert!(matches!(ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("d".into()), &modifiers, false), Some(ui_wgpu::wgpu::UiEvent::TextInput { text }) if text == "d"));
}

#[test]
fn accessibility_focus_arms_shell_select_space_and_typeahead_routing() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let surface = "control-focus-select";
    let records: Vec<ui_contract::UiNodeRecord> = serde_json::from_value(serde_json::json!([
        {
            "id": 1,
            "key": "framework.settings.appearance",
            "component": { "type": "select", "value": "system", "items": [{ "value": "system", "label": "System" }, { "value": "dark", "label": "Dark" }] },
            "layout": { "kind": "leaf", "width": "fill", "height": "hug" },
            "style": {}, "activity": "idle", "accessibility": { "label": "Appearance" },
            "bindings": [{ "trigger": "change", "action": { "scope": "framework", "name": "setAppearance", "version": 1 } }]
        }
    ]))
    .unwrap();
    let mut shell = host_test_shell();
    let mut document = shell.windows.publish_surface_records(surface, records).expect("Select surface publishes");
    let mut input = paint_tree_pointer_document(&mut shell, surface, &document, Rect::new(0.0, 0.0, 320.0, 120.0));
    shell.windows.active_window_id = Some(surface.to_string());
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(surface, "framework.settings.appearance").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(shell.chrome.chrome_build.content_has_focus(surface));
    assert!(SessionCtx::of(&mut shell).retained_select_owns_keyboard());

    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "opening with Space commits nothing");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default(), &mut input)).unwrap();
    let space = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(space.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);

    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Char("d".into()), &PointerModifiers::default(), &mut input)).unwrap();
    assert!(semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input).is_empty(), "typeahead opens and highlights before commit");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Enter, &PointerModifiers::default(), &mut input)).unwrap();
    let typed = semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input);
    assert_eq!(typed.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), ["setAppearance"]);
    assert_eq!(typed[0].args.as_ref().and_then(|args| args.get("value")).and_then(DslValue::as_str), Some("dark"));
    while !document.close_step() {}
}
