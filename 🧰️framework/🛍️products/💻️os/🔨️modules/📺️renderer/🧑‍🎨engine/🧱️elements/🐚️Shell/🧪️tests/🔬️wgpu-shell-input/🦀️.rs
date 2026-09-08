
use super::*;
#[test]
fn standalone_multi_app_variants_resolve_their_declared_app() {
    assert_eq!(resolve_playground_app_id("puzzle2d"), Some("puzzle2d-play"));
    assert_eq!(resolve_playground_app_id("puzzle3d"), Some("puzzle3d-play"));
    assert_eq!(resolve_playground_app_id("3d"), Some("puzzle3d-play"));
    assert_eq!(resolve_playground_app_id("puzzle5d"), Some("puzzle5d-play"));
}

//#region SilhouetteContentTests

#[test]
fn silhouette_hit_intersections_leave_the_cap_gap_empty() {
    let silhouette = WindowSilhouette::from_measured_top(Rect::new(0.0, 0.0, 300.0, 200.0), 80.0, 60.0, 32.0);
    let hits: Vec<Rect> = silhouette.content_clip_rects().iter().filter_map(|clip| ShellState::intersect_content_rect(*clip, silhouette.bounds)).collect();
    assert_eq!(hits.len(), 3);
    assert!(!hits.iter().any(|rect| rect.contains(150.0, 16.0)));
    assert!(hits.iter().any(|rect| rect.contains(150.0, 100.0)));
}

//#endregion SilhouetteContentTests

/// 🧪️ `dock_window_order` is a `Self`-less associated fn, so it's callable without constructing a
/// full `ShellState` fixture (impractically large: 90+ fields, several without `Default`).
fn window_ids(node: &crate::dock::DockNode) -> Vec<String> {
    let mut out = Vec::new();
    ShellState::dock_window_order(node, &mut Vec::new(), &mut out);
    out.into_iter().map(|(_, window_id)| window_id).collect()
}

#[test]
fn dock_window_order_flattens_a_single_stack_in_tab_order() {
    let node = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() };
    assert_eq!(window_ids(&node), vec!["a", "b", "c"]);
}

#[test]
fn dock_window_order_walks_row_and_column_children_depth_first() {
    let left = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("left")], active: "left".into() };
    let right_top = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("top")], active: "top".into() };
    let right_bottom = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("bottom")], active: "bottom".into() };
    let right = crate::dock::DockNode::Column(vec![(right_top, 0.5), (right_bottom, 0.5)]);
    let root = crate::dock::DockNode::Row(vec![(left, 0.5), (right, 0.5)]);
    assert_eq!(window_ids(&root), vec!["left", "top", "bottom"]);
}

#[test]
fn dock_window_order_pairs_each_window_with_its_own_stack_path() {
    let a = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a")], active: "a".into() };
    let b = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("b")], active: "b".into() };
    let root = crate::dock::DockNode::Row(vec![(a, 0.5), (b, 0.5)]);
    let mut out = Vec::new();
    ShellState::dock_window_order(&root, &mut Vec::new(), &mut out);
    assert_eq!(out, vec![(vec![0], "a".to_string()), (vec![1], "b".to_string())]);
}

// 🎯️🕹️ w2-input-wiring: key mapping is pure and focus tracking is explicit owned build state,
// so both remain testable without a full `ShellState` fixture.

#[test]
fn ui_event_from_key_action_maps_plain_char_to_text_input() {
    let modifiers = PointerModifiers::default();
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("a".into()), &modifiers);
    assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::TextInput { text: "a".into() }));
}

#[test]
fn ui_event_from_key_action_routes_ctrl_char_as_key_down_for_clipboard_chords() {
    let modifiers = PointerModifiers { ctrl: true, ..Default::default() };
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Char("c".into()), &modifiers);
    assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: "c".into(), modifiers: ui_wgpu::wgpu::EventModifiers { shift: false, ctrl: true, alt: false, meta: false } }));
}

#[test]
fn ui_event_from_key_action_maps_editing_and_tab_keys_to_matching_key_down_strings() {
    let modifiers = PointerModifiers::default();
    let cases = [
        (ui_wgpu::wgpu::KeyAction::Backspace, "Backspace"),
        (ui_wgpu::wgpu::KeyAction::Delete, "Delete"),
        (ui_wgpu::wgpu::KeyAction::Enter, "Enter"),
        (ui_wgpu::wgpu::KeyAction::Escape, "Escape"),
        (ui_wgpu::wgpu::KeyAction::ArrowLeft, "ArrowLeft"),
        (ui_wgpu::wgpu::KeyAction::ArrowRight, "ArrowRight"),
        (ui_wgpu::wgpu::KeyAction::ArrowUp, "ArrowUp"),
        (ui_wgpu::wgpu::KeyAction::ArrowDown, "ArrowDown"),
        (ui_wgpu::wgpu::KeyAction::Tab, "Tab"),
    ];
    for (action, key) in cases {
        let event = ui_event_from_key_action(&action, &modifiers);
        assert_eq!(event, Some(ui_wgpu::wgpu::UiEvent::KeyDown { key: key.into(), modifiers: ui_wgpu::wgpu::EventModifiers::default() }), "KeyAction {action:?} should map to KeyDown{{{key}}}");
    }
}

#[test]
fn ui_event_from_key_action_has_no_mapping_for_space() {
    let event = ui_event_from_key_action(&ui_wgpu::wgpu::KeyAction::Space(true), &PointerModifiers::default());
    assert_eq!(event, None);
}

#[test]
fn content_focus_tracker_defaults_unfocused_and_tracks_focus_changed_commands() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-a";
    assert!(!chrome.content_has_focus(window_id));
    let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
    let node_id = arena.insert(());
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.to_string(), node: Some(node_id) }]);
    assert!(chrome.content_has_focus(window_id));
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: window_id.to_string(), node: None }]);
    assert!(!chrome.content_has_focus(window_id));
}

#[test]
fn content_focus_tracker_ignores_commands_for_other_windows() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-b";
    let other_window_id = "w2-input-wiring-test-window-c";
    let mut arena: ui_wgpu::wgpu::Arena<()> = ui_wgpu::wgpu::Arena::new();
    let node_id = arena.insert(());
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::FocusChanged { window_id: other_window_id.to_string(), node: Some(node_id) }]);
    assert!(!chrome.content_has_focus(window_id));
    assert!(chrome.content_has_focus(other_window_id));
}

#[test]
fn content_focus_tracker_ignores_non_focus_commands() {
    let mut chrome = ShellChromeBuildState::default();
    let window_id = "w2-input-wiring-test-window-d";
    chrome.note_content_focus_commands(&[ui_wgpu::wgpu::UiCommand::ClipboardPasteRequested { window_id: window_id.to_string() }]);
    assert!(!chrome.content_has_focus(window_id));
}

/// 🕒️ `finish_dock_drag`'s successful-drop branch persists the new layout and clears the drag —
/// unchanged behavior this ticket's `noteShellCommand` dispatch is appended after, not instead of.
/// No host app is configured (`ShellState::new`'s bare fixture, same "impractically large" 90+-field
/// constraint as `dock_window_order`'s own fixture note above), so `host_controller_id()` is `None`
/// and the new dispatch is a documented no-op here — this pins the "must still complete without a
/// host to log against" half of that behavior; `note_shell_command_action`'s own shape (the other
/// half) is covered directly in `command_registry_tests`.
#[test]
fn finish_dock_drag_persists_layout_and_clears_drag_state_on_successful_drop() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock.root = crate::dock::DockNode::Row(vec![(crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("a"), DockStackTab::new("b"), DockStackTab::new("c")], active: "a".into() }, 1.0)]);
    assert!(shell.dock.remove_window("a"));
    let payload = DockDragPayload { kind: DockDragKind::Tab, window_id: "a".into(), window_kind_id: "a".into(), source_path: vec![0], tab_index: 0, ghost_label: "a".into() };
    let zone = DockDropZone::Tab { stack_path: vec![0], corner: WindowStackCorner::TopLeft, index: 2 };
    shell.dock_drag = Some(DockDragState { payload, x: 10.0, y: 10.0, drop_zone: Some(zone) });
    assert!(shell.layout_override.is_none(), "sanity: nothing persisted yet");
    let input = InputState::<ActionDescriptor>::default();
    let result = semio_framework_async::block_on(shell.finish_dock_drag(10.0, 10.0, &input));
    assert!(result.is_ok(), "finish_dock_drag must not error even without a host app to log a shell.windowMove against");
    assert!(shell.layout_override.is_some(), "a successful drop persists the new dock layout");
    assert!(shell.dock_drag.is_none(), "the transient drag state is always taken");
}

#[test]
fn context_menu_point_resolves_the_exact_concrete_window_instance() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock_drop_bodies = vec![(Vec::new(), Rect::new(0.0, 0.0, 100.0, 100.0), "canvas".into()), (vec![1], Rect::new(100.0, 0.0, 100.0, 100.0), "canvas-copy".into())];
    assert_eq!(shell.context_window_instance_id(25.0, 25.0), Some("canvas"));
    assert_eq!(shell.context_window_instance_id(125.0, 25.0), Some("canvas-copy"));
    assert_eq!(shell.context_window_instance_id(250.0, 25.0), None);
    println!("[DEBUG] native context menu resolved its exact concrete window and preserved panel scope outside dock bodies");
}
