// 🪟️ Overlay, drag-drop and scroll laws: the overlay open, dismiss and placement, the drag promotion and drop acceptance and the scroll routing and thumbs.

//#region 🔖️OverlayTests
#[test]
fn overlay_open_flags_the_node_and_close_clears_it_and_emits_overlay_closed() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let popup = leaf(&mut tree, Some(root), 1, stack_ui(), (10.0, 10.0, 50.0, 50.0));
    let mut router = EventRouter::new("main");

    router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));
    assert!(tree.node(popup).unwrap().flags.contains(NodeFlags::OVERLAY));
    assert_eq!(router.topmost_overlay().map(|overlay| overlay.kind), Some(OverlayKind::SelectPopup));

    let commands = router.close_topmost_overlay(&mut tree);
    assert!(!tree.node(popup).unwrap().flags.contains(NodeFlags::OVERLAY));
    assert!(router.topmost_overlay().is_none());
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { root, kind, .. } if *root == popup && *kind == OverlayKind::SelectPopup)));
}

#[test]
fn pointer_down_outside_a_dismissable_overlay_closes_it_and_swallows_the_press() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, button_ui("underneath"), (0.0, 0.0, 200.0, 200.0));
    let popup = leaf(&mut tree, Some(root), 2, stack_ui(), (10.0, 10.0, 50.0, 50.0));
    leaf(&mut tree, Some(popup), 1, text_ui("item"), (0.0, 0.0, 50.0, 50.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { .. })), "outside press must close the overlay");
    assert!(router.topmost_overlay().is_none());
    assert_eq!(router.capture(), None, "the outside press must be swallowed, not routed to whatever's underneath");
}

#[test]
fn pointer_down_inside_a_dismissable_overlay_does_not_close_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let popup = leaf(&mut tree, Some(root), 1, stack_ui(), (10.0, 10.0, 50.0, 50.0));
    leaf(&mut tree, Some(popup), 1, text_ui("item"), (0.0, 0.0, 50.0, 50.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::OverlayClosed { .. })), "a press inside the overlay must not dismiss it");
    assert!(router.topmost_overlay().is_some());
}

#[test]
fn escape_closes_only_the_topmost_of_two_open_overlays() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let menu = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 50.0, 50.0));
    let submenu = leaf(&mut tree, Some(root), 2, stack_ui(), (60.0, 0.0, 50.0, 50.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, menu, OverlayKind::ContextMenu, OverlayAnchor::Node(root));
    router.open_overlay(&mut tree, submenu, OverlayKind::ContextMenu, OverlayAnchor::Node(menu));

    let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { root, .. } if *root == submenu)));
    assert_eq!(router.topmost_overlay().map(|overlay| overlay.root), Some(menu), "only the topmost overlay closes on Escape");
}

#[test]
fn tab_focus_is_trapped_inside_an_open_dialog_overlay() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
    leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
    leaf(&mut tree, Some(root), 2, button_ui("b"), (50.0, 0.0, 50.0, 20.0));
    let dialog = leaf(&mut tree, Some(root), 3, stack_ui(), (100.0, 100.0, 100.0, 100.0));
    let button_c = leaf(&mut tree, Some(dialog), 1, button_ui("c"), (0.0, 0.0, 50.0, 20.0));
    let button_d = leaf(&mut tree, Some(dialog), 2, button_ui("d"), (50.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, dialog, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });

    let tab = || UiEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() };
    router.dispatch(&mut tree, root, &tab());
    assert_eq!(router.focus.focused, Some(button_c));
    router.dispatch(&mut tree, root, &tab());
    assert_eq!(router.focus.focused, Some(button_d));
    router.dispatch(&mut tree, root, &tab());
    assert_eq!(router.focus.focused, Some(button_c), "focus-trapped Tab cycling must never reach button_a/button_b outside the dialog");
}
//#endregion 🔖️OverlayTests

//#region 🔖️DragDropTests
#[test]
fn drag_session_promotes_after_threshold_and_commits_on_an_accepting_drop_target() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
    set_flag(&mut tree, source, NodeFlags::DRAG_SOURCE);
    let target = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 100.0, 50.0, 50.0));
    set_flag(&mut tree, target, NodeFlags::DROP_TARGET);
    leaf(&mut tree, Some(target), 1, text_ui("drop-here"), (0.0, 0.0, 50.0, 50.0));
    let mut router = EventRouter::new("main");
    let mut payload = DragPayload::new();
    payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\"}".into());
    router.set_drag_payload(source, payload.clone());

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((source, CaptureKind::Press)), "a plain press must not immediately start a drag");

    // Small move under the promotion threshold: still just a Press.
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 6.0, y: 6.0, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((source, CaptureKind::Press)));

    // Move past the threshold and over the drop target: promotes to Drag and finds the target.
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));
    assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), Some(target));

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 120.0, y: 120.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::DropCommitted { source: s, target: t, payload: p, .. } if *s == source && *t == target && *p == payload)));
    assert_eq!(router.capture(), None);
    assert!(router.drag_session().is_none());
}

#[test]
fn drag_session_cancels_when_released_over_no_accepting_drop_target() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0_f32, 200.0));
    let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
    let mut router = EventRouter::new("main");
    router.set_drag_payload(source, DragPayload::new());

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 190.0, y: 190.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::DropCancelled { source: s, .. } if *s == source)));
}

#[test]
fn a_drop_targets_accept_predicate_can_reject_the_active_payload() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
    let target = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 100.0, 50.0, 50.0));
    set_flag(&mut tree, target, NodeFlags::DROP_TARGET);
    leaf(&mut tree, Some(target), 1, text_ui("drop-here"), (0.0, 0.0, 50.0, 50.0));
    let mut router = EventRouter::new("main");
    router.set_drag_payload(source, DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "x".to_string())]));
    router.set_drop_accept(target, |payload| payload.contains_key("application/x-semio-catalogue-item"));

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0, modifiers: Default::default() });

    assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), None, "the predicate must reject this payload's mime key");
}
//#endregion 🔖️DragDropTests

//#region 🔖️ScrollTests
#[test]
fn scroll_routes_to_the_nearest_scrollable_ancestor_and_clamps_at_zero() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
    leaf(&mut tree, Some(root), 1, text_ui("content"), (10.0, 10.0, 180.0, 390.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: 30.0, modifiers: Default::default() });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 30.0));

    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: -100.0, modifiers: Default::default() });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 0.0), "scroll offset must clamp at zero, not go negative");

    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: 1_000.0, modifiers: Default::default() });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 200.0), "scroll offset must clamp at the retained content extent");
}

#[test]
fn scroll_thumb_capture_drags_the_scrollable_offset_along_its_registered_axis() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
    // A bare `Stack` is a `hit_test`-transparent pass-through container (see 🔖️HitTest's
    // `is_plain_container`) — the thumb needs to be a real leaf to be hit-testable itself.
    let thumb = leaf(&mut tree, Some(root), 1, separator_ui(), (190.0, 0.0, 10.0, 40.0));
    let mut router = EventRouter::new("main");
    router.register_scroll_thumb(thumb, root, ScrollAxis::Vertical);

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 195.0, y: 5.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((root, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 195.0, y: 25.0, modifiers: Default::default() });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 20.0));

    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 195.0, y: 25.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), None);
}
//#endregion 🔖️ScrollTests
