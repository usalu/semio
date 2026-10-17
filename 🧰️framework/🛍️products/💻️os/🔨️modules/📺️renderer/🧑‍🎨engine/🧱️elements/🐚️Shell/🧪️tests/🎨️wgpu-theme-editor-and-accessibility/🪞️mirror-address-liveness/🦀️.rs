/// 🪞️ Mirror address liveness: the settings toggle shell and the accessibility address staying live, stale or delayed across frames.

fn settings_toggle_shell() -> ShellState {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.dock.dock_tabs.tabs_mut(PanelAnchor::BottomRight).push(DockTabNode::branch(FRAMEWORK_SETTINGS_PANEL_ID, "Settings", "settings", 0, vec![DockTabNode::leaf(FRAMEWORK_SETTINGS_GENERAL_TAB_ID, "General", "settings", 0)]));
    shell
}

fn generation_rule_hit(rect_key: &str) -> HitTarget<ActionDescriptor> {
    let rule = &accessibility_fixture()["presentedChrome"]["generationRule"];
    let rect = rule[rect_key].as_array().expect("generation rule rect").iter().map(|value| value.as_f64().expect("rect component") as f32).collect::<Vec<_>>();
    HitTarget { rect: Rect::new(rect[0], rect[1], rect[2], rect[3]), event: None, control_id: Some(rule["controlId"].as_str().expect("generation rule control").to_string()), kind: HitKind::Toggle, drag_axis: None, drag_data: None }
}

fn settings_toggle_address(shell: &ShellState, input: &InputState<ActionDescriptor>) -> ui_render::AccessibilityTarget {
    let control = accessibility_fixture()["presentedChrome"]["generationRule"]["controlId"].as_str().expect("generation rule control").to_string();
    let node = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == control).expect("Settings mirror node");
    ui_render::AccessibilityTarget { window_id: semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID.to_string(), window_generation: shell.chrome.presented_chrome_accessibility_generation, node_id: node.node_id, node_key: node.key }
}

#[test]
fn a_mirror_address_stays_live_across_frames_that_present_the_same_chrome() {
    let mut shell = settings_toggle_shell();
    let mut input = InputState::default();
    input.register_hit(generation_rule_hit("firstRect"));
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let (initial_epoch, delayed) = (shell.input.presented_input_epoch, settings_toggle_address(&shell, &input));

    while input.retire_hit_step() {}
    input.register_hit(generation_rule_hit("sameChromeRect"));
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    assert!(shell.input.presented_input_epoch > initial_epoch, "every presented frame owns its own input epoch");
    assert_eq!(shell.chrome.presented_chrome_accessibility_generation, delayed.window_generation, "an unchanged chrome keeps the generation the mirror addresses");
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&delayed, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("idle-frame Settings activation"));
    assert!(shell.dock.anchor_open(PanelAnchor::BottomRight), "a mirror one idle frame behind still activates the live control exactly once");
}

#[test]
fn a_chrome_focus_update_keeps_the_following_activation_address_live() {
    let mut shell = settings_toggle_shell();
    let mut input = InputState::default();
    input.register_hit(generation_rule_hit("firstRect"));
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let target = settings_toggle_address(&shell, &input);
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    while input.retire_hit_step() {}
    input.register_hit(generation_rule_hit("sameChromeRect"));
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    assert_eq!(shell.chrome.presented_chrome_accessibility_generation == target.window_generation, accessibility_fixture()["presentedChrome"]["generationRule"]["focusOnlyKeepsGeneration"].as_bool().unwrap());
    assert!(shell.chrome.presented_chrome_accessibility.iter().find(|node| node.key == target.node_key).unwrap().focused);
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).unwrap());
    assert!(shell.dock.anchor_open(PanelAnchor::BottomRight));
}

#[test]
fn a_delayed_chrome_mirror_address_cannot_activate_after_the_chrome_changes() {
    let mut shell = settings_toggle_shell();
    let mut input = InputState::default();
    input.register_hit(generation_rule_hit("firstRect"));
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    let delayed = settings_toggle_address(&shell, &input);

    while input.retire_hit_step() {}
    input.register_hit(generation_rule_hit("changedChromeRect"));
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    assert!(shell.chrome.presented_chrome_accessibility_generation > delayed.window_generation, "a changed chrome projection owns a new generation");
    assert!(!semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&delayed, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("delayed Settings activation"));
    assert!(!shell.dock.anchor_open(PanelAnchor::BottomRight), "a delayed address cannot mutate the changed chrome");

    let current = settings_toggle_address(&shell, &input);
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&current, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("current Settings activation"));
    assert!(shell.dock.anchor_open(PanelAnchor::BottomRight), "the current mirror address activates exactly once");
}
