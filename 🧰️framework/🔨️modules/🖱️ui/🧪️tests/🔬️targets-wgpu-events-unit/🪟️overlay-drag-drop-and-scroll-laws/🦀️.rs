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

    assert_eq!(router.focus.focused, Some(button_c), "opening a dialog moves focus to its first focusable, React's onOpenAutoFocus default");
    let tab = || UiEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() };
    router.dispatch(&mut tree, root, &tab());
    assert_eq!(router.focus.focused, Some(button_d));
    router.dispatch(&mut tree, root, &tab());
    assert_eq!(router.focus.focused, Some(button_c), "focus-trapped Tab cycling must never reach button_a/button_b outside the dialog");
}

/// 🔙️ React's `restoreFocusRef` / `triggerRef.focus()`: a closed Dialog, Popover or ContextMenu hands focus back instead of dropping it.
#[test]
fn closing_a_dialog_returns_focus_to_what_held_it_and_a_popover_to_its_trigger() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
    let opener = leaf(&mut tree, Some(root), 1, button_ui("open"), (0.0, 0.0, 50.0, 20.0));
    let trigger = leaf(&mut tree, Some(root), 2, button_ui("trigger"), (60.0, 0.0, 50.0, 20.0));
    let dialog = leaf(&mut tree, Some(root), 3, stack_ui(), (100.0, 100.0, 100.0, 100.0));
    let inside = leaf(&mut tree, Some(dialog), 1, button_ui("ok"), (0.0, 0.0, 50.0, 20.0));
    let popover = leaf(&mut tree, Some(root), 4, stack_ui(), (100.0, 10.0, 80.0, 40.0));
    let popover_inside = leaf(&mut tree, Some(popover), 1, button_ui("item"), (0.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(opener), true);
    router.open_overlay(&mut tree, dialog, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });
    router.focus.set_focus(&mut tree, Some(inside), true);

    let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert_eq!(router.focus.focused, Some(opener), "Escape on a dialog returns focus to the element that held it");
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::FocusChanged { node: Some(node), .. } if *node == opener)));

    router.focus.set_focus(&mut tree, None, false);
    router.open_overlay(&mut tree, popover, OverlayKind::Popover, OverlayAnchor::Node(trigger));
    router.focus.set_focus(&mut tree, Some(popover_inside), true);
    router.close_overlay(&mut tree, popover);
    assert_eq!(router.focus.focused, Some(trigger), "a popover returns focus to its trigger even when nothing was focused before it opened");
}

#[test]
fn a_prevented_focus_return_leaves_focus_dropped_exactly_as_before() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
    let trigger = leaf(&mut tree, Some(root), 1, button_ui("trigger"), (0.0, 0.0, 50.0, 20.0));
    let popover = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 10.0, 80.0, 40.0));
    let inside = leaf(&mut tree, Some(popover), 1, button_ui("item"), (0.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, popover, OverlayKind::Popover, OverlayAnchor::Node(trigger));
    assert!(router.set_overlay_veto(popover, OverlayVeto { focus_return: true, ..OverlayVeto::default() }));
    router.focus.set_focus(&mut tree, Some(inside), true);
    router.close_overlay(&mut tree, popover);
    assert_eq!(router.focus.focused, None);
}

/// 🛑️ `onEscapeKeyDown` / `onPointerDownOutside` `preventDefault()`: the overlay stays open, and a prevented outside press is not swallowed.
#[test]
fn a_prevented_escape_or_outside_press_leaves_the_overlay_open_and_the_press_unswallowed() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let underneath = leaf(&mut tree, Some(root), 1, button_ui("underneath"), (0.0, 0.0, 200.0, 200.0));
    let popover = leaf(&mut tree, Some(root), 2, stack_ui(), (10.0, 10.0, 50.0, 50.0));
    leaf(&mut tree, Some(popover), 1, text_ui("item"), (0.0, 0.0, 50.0, 50.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, popover, OverlayKind::Popover, OverlayAnchor::Node(underneath));
    assert!(router.set_overlay_veto(popover, OverlayVeto { escape: true, outside_press: true, focus_return: false }));

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert!(router.topmost_overlay().is_some(), "a prevented Escape does not close");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(router.topmost_overlay().is_some(), "a prevented outside press does not close");
    assert_eq!(router.capture().map(|(node, _)| node), Some(underneath), "and the press reaches what is underneath");

    router.set_overlay_veto(popover, OverlayVeto::default());
    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert!(router.topmost_overlay().is_none(), "an unprevented Escape closes the overlay");
}

/// 🔒️ React's modal isolation (`inert`/`aria-hidden` on siblings): while a dialog is open nothing outside it, or above it, is hit, hovered, scrolled or focused by the pointer.
#[test]
fn a_modal_dialog_makes_everything_outside_it_inert_to_the_pointer() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
    let behind = leaf(&mut tree, Some(root), 1, button_ui("behind"), (0.0, 0.0, 50.0, 20.0));
    let dialog = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 100.0, 100.0, 100.0));
    let inside = leaf(&mut tree, Some(dialog), 1, button_ui("ok"), (0.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");
    assert_eq!(router.hit_test(&tree, root, 10.0, 10.0), Some(behind), "nothing is inert before the dialog opens");
    router.open_overlay(&mut tree, dialog, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });
    assert_eq!(router.hit_test(&tree, root, 10.0, 10.0), None, "the page behind a modal is not hit");
    assert_eq!(router.hit_test(&tree, root, 110.0, 110.0), Some(inside), "the dialog itself still is");
    router.close_overlay(&mut tree, dialog);
    assert_eq!(router.hit_test(&tree, root, 10.0, 10.0), Some(behind), "closing lifts the isolation");
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

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 6.0, y: 6.0, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((source, CaptureKind::Press)));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));
    assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), Some(target));

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 120.0, y: 120.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::DropCommitted { source: s, target: t, payload: p, .. } if *s == source && *t == target && *p == payload)));
    assert_eq!(router.capture(), None);
    assert!(router.drag_session().is_none());
}

#[test]
fn a_synthetic_pointer_drag_of_one_row_over_another_marks_the_hover_target_and_commits_the_drop_on_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let rows: Vec<NodeId> = (0..3u32).map(|n| leaf(&mut tree, Some(root), n + 1, stack_ui(), (0.0, n as f32 * 30.0, 200.0, 30.0))).collect();
    rows.iter().for_each(|row| set_flag(&mut tree, *row, NodeFlags::DROP_TARGET));
    set_flag(&mut tree, rows[2], NodeFlags::DRAG_SOURCE);
    let mut router = EventRouter::new("main");
    let mut payload = DragPayload::new();
    payload.insert("application/x-semio-tree-row".into(), "{\"id\":\"b3\"}".into());
    router.set_drag_payload(rows[2], payload.clone());

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 75.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((rows[2], CaptureKind::Press)));
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 50.0, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((rows[2], CaptureKind::Drag)), "moving past the threshold promotes the press to a drag");
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 40.0, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0, modifiers: Default::default() });
    assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), Some(rows[0]), "the row under the pointer is the hover marker target");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let drops: Vec<_> = commands.iter().filter_map(|cmd| match cmd { UiCommand::DropCommitted { source, target, payload, .. } => Some((*source, *target, payload.clone())), _ => None }).collect();
    assert_eq!(drops, vec![(rows[2], rows[0], payload)], "release commits exactly one drop on the hovered row, carrying the dragged row payload");
    assert!(router.drag_session().is_none() && router.capture().is_none());
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

/// 📜️ The scrollbar is derived from the scroll host's own extent, no thumb node registered: a press on the thumb captures it and a drag scrolls in proportion to the thumb's travel, a press beside it pages one viewport.
#[test]
fn a_scrollbar_thumb_drag_scrolls_in_proportion_and_a_track_press_pages() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
    leaf(&mut tree, Some(root), 1, text_ui("content"), (10.0, 10.0, 180.0, 390.0));
    let mut router = EventRouter::new("main");
    let bar = scrollbar_for(&tree, root, true, false).expect("400 of content in a 200 viewport overflows");
    let (grab_x, grab_y) = (bar.thumb.x + bar.thumb.w * 0.5, bar.thumb.y + bar.thumb.h * 0.5);

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: grab_x, y: grab_y, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((root, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: grab_x, y: grab_y + 50.0, modifiers: Default::default() });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 100.0), "half the thumb travel is half the content travel");

    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: grab_x, y: grab_y + 50.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), None);

    let bar = scrollbar_for(&tree, root, true, false).unwrap();
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: bar.track.x + 1.0, y: bar.track.y + 1.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 0.0), "a press above the thumb pages up");
    assert_eq!(router.capture(), None, "paging is not a capture");
}

#[test]
fn a_horizontal_scrollbar_thumb_drag_scrolls_the_x_offset() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
    leaf(&mut tree, Some(root), 1, text_ui("wide"), (0.0, 0.0, 390.0, 20.0));
    let mut router = EventRouter::new("main");
    let bar = scrollbar_for(&tree, root, false, false).expect("390 of content in a 200 viewport overflows horizontally");
    let (grab_x, grab_y) = (bar.thumb.x + bar.thumb.w * 0.5, bar.thumb.y + bar.thumb.h * 0.5);
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: grab_x, y: grab_y, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((root, CaptureKind::ScrollThumb(ScrollAxis::Horizontal))));
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: grab_x + 1_000.0, y: grab_y, modifiers: Default::default() });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (190.0, 0.0), "dragging past the end clamps at the content extent");
}
//#endregion 🔖️ScrollTests
