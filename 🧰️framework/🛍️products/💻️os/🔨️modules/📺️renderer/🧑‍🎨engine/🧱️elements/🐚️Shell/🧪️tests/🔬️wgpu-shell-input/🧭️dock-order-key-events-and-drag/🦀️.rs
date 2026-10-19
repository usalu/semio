/// 🧭️ Dock order, key events and drag: the silhouette cap gap, dock window order, key action events, the content focus tracker, dock drags and body presses.

//#region SilhouetteContentTests

#[test]
fn silhouette_hit_intersections_leave_the_cap_gap_empty() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let silhouette = WindowSilhouette::from_measured_top(Rect::new(0.0, 0.0, 300.0, 200.0), 80.0, 60.0, 32.0);
    let hits: Vec<Rect> = silhouette.content_clip_rects().iter().filter_map(|clip| intersect_content_rect(*clip, silhouette.bounds)).collect();
    assert_eq!(hits.len(), 3);
    assert!(!hits.iter().any(|rect| rect.contains(150.0, 16.0)));
    assert!(hits.iter().any(|rect| rect.contains(150.0, 100.0)));
}

//#endregion SilhouetteContentTests

/// 🧪️ `dock_window_order` is a `Self`-less associated fn, so it's callable without constructing a
/// full `ShellState` fixture (impractically large: 90+ fields, several without `Default`).
fn window_ids(node: &semio_framework_os_renderer_wgpu_dock::DockNode) -> Vec<String> {
    let mut out = Vec::new();
    dock_window_order(node, &mut Vec::new(), &mut out);
    out.into_iter().map(|(_, window_id)| window_id).collect()
}

#[test]
fn dock_window_order_flattens_a_single_stack_in_tab_order() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let node = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() };
    assert_eq!(window_ids(&node), vec!["a", "b", "c"]);
}

#[test]
fn dock_window_order_walks_row_and_column_children_depth_first() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let left = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("left")], active: "left".into() };
    let right_top = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("top")], active: "top".into() };
    let right_bottom = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("bottom")], active: "bottom".into() };
    let right = semio_framework_os_renderer_wgpu_dock::DockNode::Column(vec![(right_top, 0.5), (right_bottom, 0.5)]);
    let root = semio_framework_os_renderer_wgpu_dock::DockNode::Row(vec![(left, 0.5), (right, 0.5)]);
    assert_eq!(window_ids(&root), vec!["left", "top", "bottom"]);
}

#[test]
fn dock_window_order_pairs_each_window_with_its_own_stack_path() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let a = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("a")], active: "a".into() };
    let b = semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("b")], active: "b".into() };
    let root = semio_framework_os_renderer_wgpu_dock::DockNode::Row(vec![(a, 0.5), (b, 0.5)]);
    let mut out = Vec::new();
    dock_window_order(&root, &mut Vec::new(), &mut out);
    assert_eq!(out, vec![(vec![0], "a".to_string()), (vec![1], "b".to_string())]);
}

// 🎯️🕹️ w2-input-wiring: key mapping is pure and focus tracking is explicit owned build state,
// so both remain testable without a full `ShellState` fixture.

#[test]
fn ui_event_from_key_action_maps_plain_char_to_text_input() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let modifiers = PointerModifiers::default();
    let event = ui_event_from_key_action(&semio_framework_ui_wgpu_input::KeyAction::Char("a".into()), &modifiers, false);
    assert_eq!(event, Some(semio_framework_ui_wgpu_events::UiEvent::TextInput { text: "a".into() }));
}

#[test]
fn ui_event_from_key_action_routes_ctrl_char_as_key_down_for_clipboard_chords() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let modifiers = PointerModifiers { ctrl: true, ..Default::default() };
    let event = ui_event_from_key_action(&semio_framework_ui_wgpu_input::KeyAction::Char("c".into()), &modifiers, false);
    assert_eq!(event, Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: "c".into(), modifiers: semio_framework_ui_wgpu_events::EventModifiers { shift: false, ctrl: true, alt: false, meta: false } }));
}

#[test]
fn ui_event_from_key_action_maps_editing_and_tab_keys_to_matching_key_down_strings() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let modifiers = PointerModifiers::default();
    let cases = [
        (semio_framework_ui_wgpu_input::KeyAction::Backspace, "Backspace"),
        (semio_framework_ui_wgpu_input::KeyAction::Delete, "Delete"),
        (semio_framework_ui_wgpu_input::KeyAction::Enter, "Enter"),
        (semio_framework_ui_wgpu_input::KeyAction::Escape, "Escape"),
        (semio_framework_ui_wgpu_input::KeyAction::ArrowLeft, "ArrowLeft"),
        (semio_framework_ui_wgpu_input::KeyAction::ArrowRight, "ArrowRight"),
        (semio_framework_ui_wgpu_input::KeyAction::ArrowUp, "ArrowUp"),
        (semio_framework_ui_wgpu_input::KeyAction::ArrowDown, "ArrowDown"),
        (semio_framework_ui_wgpu_input::KeyAction::Home, "Home"),
        (semio_framework_ui_wgpu_input::KeyAction::End, "End"),
        (semio_framework_ui_wgpu_input::KeyAction::PageUp, "PageUp"),
        (semio_framework_ui_wgpu_input::KeyAction::PageDown, "PageDown"),
        (semio_framework_ui_wgpu_input::KeyAction::Tab, "Tab"),
    ];
    for (action, key) in cases {
        let event = ui_event_from_key_action(&action, &modifiers, false);
        assert_eq!(event, Some(semio_framework_ui_wgpu_events::UiEvent::KeyDown { key: key.into(), modifiers: semio_framework_ui_wgpu_events::EventModifiers::default() }), "KeyAction {action:?} should map to KeyDown{{{key}}}");
    }
}

#[test]
fn ui_event_from_key_action_has_no_mapping_for_space() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let event = ui_event_from_key_action(&semio_framework_ui_wgpu_input::KeyAction::Space(true), &PointerModifiers::default(), false);
    assert_eq!(event, None);
}

#[test]
fn content_focus_tracker_defaults_unfocused_and_clears_a_typed_focus_record() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-a";
    assert!(!chrome.content_has_focus(window_id));
    let mut arena: semio_framework_ui_arena::Arena<()> = semio_framework_ui_arena::Arena::new();
    let node_id = arena.insert(());
    chrome.content_focus.insert(window_id.into(), Some(RetainedContentFocus { node: node_id, key: semio_framework_ui_retained_tree::tree::NodeKey::Explicit("typed-focus".into()), kind: RetainedNodeFocusKind::Input }));
    chrome.focused_retained_surface = Some(window_id.into());
    assert!(chrome.content_has_focus(window_id));
    chrome.note_content_focus_commands(&[semio_framework_ui_wgpu_events::UiCommand::FocusChanged { window_id: window_id.to_string(), node: None }]);
    assert!(!chrome.content_has_focus(window_id));
}

#[test]
fn content_focus_tracker_keeps_a_typed_record_scoped_to_its_window() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-b";
    let other_window_id = "w2-input-wiring-test-window-c";
    let mut arena: semio_framework_ui_arena::Arena<()> = semio_framework_ui_arena::Arena::new();
    let node_id = arena.insert(());
    chrome.content_focus.insert(other_window_id.into(), Some(RetainedContentFocus { node: node_id, key: semio_framework_ui_retained_tree::tree::NodeKey::Explicit("other-focus".into()), kind: RetainedNodeFocusKind::Input }));
    assert!(!chrome.content_has_focus(window_id));
    assert!(chrome.content_has_focus(other_window_id));
}

#[test]
fn content_focus_tracker_ignores_non_focus_commands() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-d";
    chrome.note_content_focus_commands(&[semio_framework_ui_wgpu_events::UiCommand::ClipboardPasteRequested { window_id: window_id.to_string() }]);
    assert!(!chrome.content_has_focus(window_id));
}

/// 🕒️ `finish_dock_drag`'s successful-drop branch persists the new layout and clears the drag —
/// unchanged behavior this ticket's `noteShellCommand` dispatch is appended after, not instead of.
/// No host app is configured (`ShellState::new`'s bare fixture, same "impractically large" 90+-field
/// constraint as `dock_window_order`'s own fixture note above), so `host_controller_id()` is `None`
/// and the new dispatch is a documented no-op here — this pins the "must still complete without a
/// host to log against" half of that behavior; `note_shell_command_action`'s own shape (the other
/// half) is covered directly in `command_registry_tests`.
///
/// 🎬️ A live drag never edits the committed tree, so the drop zone is a path into
/// `DockState::render_view`'s derivation — lifting `a` out of the lone axis child hoists the stack
/// to the ROOT, exactly as React's `collapseLayout` does.
#[test]
fn finish_dock_drag_persists_layout_and_clears_drag_state_on_successful_drop() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.dock.dock.root = semio_framework_os_renderer_wgpu_dock::DockNode::Row(vec![(semio_framework_os_renderer_wgpu_dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() }, 1.0)]);
    let payload = DockDragPayload { kind: DockDragKind::Tab, window_id: "a".into(), window_kind_id: "a".into(), template_id: None, source_path: vec![0], tab_index: 0, ghost_label: "a".into() };
    let zone = DockDropZone::Tab { stack_path: vec![], corner: WindowStackCorner::TopLeft, index: 2 };
    shell.dock.dock_drag = Some(DockDragState { payload, x: 10.0, y: 10.0, drop_zone: Some(zone) });
    assert!(shell.dock.layout_override.is_none(), "sanity: nothing persisted yet");
    let input = InputState::<ActionDescriptor>::default();
    let result = semio_framework_async::block_on(PointerCtx::of(&mut shell).finish_dock_drag(10.0, 10.0, &input));
    assert!(result.is_ok(), "finish_dock_drag must not error even without a host app to log a shell.windowMove against");
    assert!(shell.dock.layout_override.is_some(), "a successful drop persists the new dock layout");
    assert!(shell.dock.dock_drag.is_none(), "the transient drag state is always taken");
}

/// 🎯️ Actual normalized tab releases preserve the committed source when destination geometry is stale.
#[test]
fn normalized_tab_drop_uses_shifted_destination_and_refuses_invalid_path_atomically() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    for invalid in [false, true] {
        let stack = |id: &str| DockNode::Stack { windows: vec![DockStackTab::new(id)], active: id.into() };
        let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
        shell.dock.dock.root = DockNode::Row(vec![(stack("a"), 1.0), (stack("b"), 1.0), (stack("c"), 1.0)]);
        let before = shell.dock.dock.root.clone();
        let rect = Rect::new(20.0, 20.0, 20.0, 24.0);
        let mut input = InputState::<ActionDescriptor>::default();
        input.register_hit(HitTarget { rect, event: None, control_id: Some("dock.tab.1.b.drag".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
        input.publish_hits();
        let mut interaction = pointer_interaction(shell, input);
        let pointer = mouse_pointer(99);
        let modifiers = ui_render::EventModifiers::default();
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut ui_render::DispatchEvent::PointerDown { pointer, x: 30.0, y: 30.0, button: ui_render::PointerButton::Primary, modifiers }));
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut ui_render::DispatchEvent::PointerMove { pointer, x: 40.0, y: 30.0, modifiers }));
        assert!(interaction.shell.dock.dock_drag.is_some());
        assert_eq!(interaction.shell.dock.dock.root, before);
        interaction.shell.input.presented_input_geometry.dock_drop_tab_bars = vec![(vec![if invalid { 4 } else { 1 }], WindowStackCorner::TopLeft, Rect::new(100.0, 20.0, 100.0, 24.0), vec![100.0])];
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut ui_render::DispatchEvent::PointerMove { pointer, x: 110.0, y: 30.0, modifiers }));
        semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut ui_render::DispatchEvent::PointerUp { pointer, x: 110.0, y: 30.0, button: ui_render::PointerButton::Primary, modifiers }));
        assert!(interaction.shell.dock.dock_drag.is_none());
        if invalid {
            assert_eq!(interaction.shell.dock.dock.root, before);
            assert!(interaction.shell.dock.layout_override.is_none());
        } else {
            assert_eq!(interaction.shell.dock.dock.stack_windows_at_path(&vec![1]), Some(vec!["b".into(), "c".into()]));
            assert_eq!(interaction.shell.windows.active_window_id.as_deref(), Some("b"));
            assert_eq!(interaction.shell.dock.layout_override, Some(interaction.shell.dock.dock.to_window_layout()));
        }
    }
}

/// 🛰️ A browser-normalized press and move preserve Chrome capture and promote the exact dock-tab
/// grip after the five-pixel threshold, before any release mutates the committed dock.
#[test]
fn normalized_dock_tab_pointer_sequence_promotes_the_drag_before_release() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let stack = |window_id: &str| DockNode::Stack { windows: vec![DockStackTab::instance(window_id, "main", WindowStackCorner::TopLeft)], active: window_id.to_string() };
    let mut shell = host_test_shell();
    shell.dock.dock.root = DockNode::Row(vec![(stack("top"), 0.5), (stack("perspective"), 0.5)]);
    shell.dock.dock.mobile = false;
    let rect = Rect::new(80.0, 35.0, 18.0, 23.0);
    let mut input = InputState::<ActionDescriptor>::default();
    input.register_hit(HitTarget { rect, event: None, control_id: Some("dock.tab.0.top.drag".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
    input.publish_hits();
    let mut interaction = pointer_interaction(shell, input);
    let pointer = ui_render::PointerInfo { id: ui_render::PointerId(1), kind: ui_render::PointerKind::Mouse, pressure: Some(0.5), tilt: None };
    let start = (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(
        &mut interaction,
        &mut ui_render::DispatchEvent::PointerDown { pointer, x: start.0, y: start.1, button: ui_render::PointerButton::Primary, modifiers: ui_render::EventModifiers::default() },
    ));
    assert!(interaction.shell.dock.pending_dock_drag.as_ref().is_some_and(|(payload, origin)| payload.kind == DockDragKind::Tab && payload.window_id == "top" && *origin == start));
    assert_eq!(interaction.pointer_capture.holder(pointer.id), Some(PointerHitOwner::Chrome));

    semio_framework_async::block_on(semio_framework_os_renderer_wgpu_frame_actions::dispatch_normalized_event(&mut interaction, &mut ui_render::DispatchEvent::PointerMove { pointer, x: start.0 + 6.0, y: start.1, modifiers: ui_render::EventModifiers::default() }));
    assert!(interaction.shell.dock.pending_dock_drag.is_none());
    assert!(interaction.shell.dock.dock_drag.as_ref().is_some_and(|drag| drag.payload.kind == DockDragKind::Tab && drag.payload.window_id == "top"));
    assert_eq!(interaction.shell.dock.dock.collect_window_ids(), vec!["top".to_string(), "perspective".to_string()], "promotion derives a ghost without mutating the committed dock");

    interaction.handle_pointer_cancel(pointer);
    assert!(interaction.shell.dock.pending_dock_drag.is_none() && interaction.shell.dock.dock_drag.is_none());
}

#[test]
fn context_menu_point_resolves_the_exact_concrete_window_instance() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.dock.dock_drop_bodies = vec![(Vec::new(), Rect::new(0.0, 0.0, 100.0, 100.0), "canvas".into()), (vec![1], Rect::new(100.0, 0.0, 100.0, 100.0), "canvas-copy".into())];
    let mut input = InputState::<ActionDescriptor>::default();
    assert_eq!(shell.input.context_window_instance_id(25.0, 25.0), None, "candidate geometry does not route input before presentation");
    PanelsViewCtx::of(&mut shell).publish_retained_hit_registry(&mut input);
    assert_eq!(shell.input.context_window_instance_id(25.0, 25.0), Some("canvas"));
    assert_eq!(shell.input.context_window_instance_id(125.0, 25.0), Some("canvas-copy"));
    assert_eq!(shell.input.context_window_instance_id(250.0, 25.0), None);
}

/// ⚖️ LAW: pressing a window's retained BODY activates that window, so the keyboard follows the
/// window the user actually clicked into rather than whichever one the session opened with.
///
/// 🩸️ `handle_keyboard_async` routes real keys into retained content on exactly one predicate —
/// `content_has_focus(active_window_id)` — and a retained body press was the one way into a window
/// that never set `active_window_id`. Measured on 6118: pressing the Generations window's inline
/// rename editor logged `content focus window=generation3d-generations node=Some(..)` and then every
/// keystroke logged `key routing window=procedural-main contentFocus=false`, so the editor opened,
/// took focus, and could not be typed into (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-generation-publication-2026-09-13.md`).
#[test]
fn a_retained_body_press_activates_its_own_window_so_the_keyboard_follows_it() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let opened_with = "procedural-main";
    let pressed = "generation3d-generations";
    shell.windows.active_window_id = Some(opened_with.into());
    let mut document = shell.windows.publish_surface_records(pressed, pane_owner_select_records_with("body-focus", "system")).expect("pressed window document publishes");
    let body = Rect::new(3.0, 54.0, 315.0, 814.0);
    let mut input = paint_tree_pointer_document(&mut shell, pressed, &document, body);
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(pressed, "body-focus").expect("the published pane exposes its exact accessible address");
    assert!(semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Focus, &mut input)).unwrap());
    assert!(!shell.chrome.chrome_build.content_has_focus(opened_with), "sanity: the window the keyboard used to follow never had content focus");

    semio_framework_async::block_on(PointerCtx::of(&mut shell).route_retained_pointer_press(pressed, body, 255.0, 90.0, true, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).expect("a retained body press routes");

    assert_eq!(shell.windows.active_window_id.as_deref(), Some(pressed), "the pressed window is the active one");
    assert!(shell.chrome.chrome_build.content_has_focus(shell.windows.active_window_id.as_deref().expect("an active window")), "…so the keyboard's own predicate now answers for the window whose content holds focus");
    while !document.close_step() {}
}

/// ⚖️ LAW: the RELEASE half of the same click changes nothing about activation — a click activates on
/// the press, the way every window manager and every browser does, and the action fires on release
/// (`route_retained_pointer_press`'s own `else if !down` arm).
#[test]
fn only_the_press_half_of_a_body_click_moves_the_active_window() {
    let _camera_owner = semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease(semio_framework_os_renderer_wgpu_scene_state::camera_fixture_grant());
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.windows.active_window_id = Some("procedural-main".into());
    let mut input = InputState::<ActionDescriptor>::default();
    let body = Rect::new(3.0, 54.0, 315.0, 814.0);
    semio_framework_async::block_on(PointerCtx::of(&mut shell).route_retained_pointer_press("generation3d-generations", body, 255.0, 90.0, false, 0, HitKind::Input, None, ui_render::PointerId(1), &mut input)).expect("a retained body release routes");
    assert_eq!(shell.windows.active_window_id.as_deref(), Some("procedural-main"), "a release alone never activates — the press already did, or nothing did");
}
