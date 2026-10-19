// 🖱️ Interactivity and scene command laws: the press, release, click and scrub interactivity laws and the scene command and hit routing.

//#region 🔖️W2InteractivityTests
// 🔽️🎴️🌳️ Tests for the wiring closed out per `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2
// pass: `Select` popup open/close (`toggle_select_popup`/`finish_close`), `Stack`
// `activate`/`drop_action` (`is_plain_stack_container`'s hit-test exception), and `Tree` row
// `draggable` (`find_tree_item_spec`).

#[test]
fn clicking_a_select_opens_its_popup_and_clicking_again_closes_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(tree.node(select).unwrap().state.open, "clicking a closed select should open its popup");
    assert!(tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY), "an open select's popup subtree should win hit-test priority over its siblings");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(!tree.node(select).unwrap().state.open, "clicking an open select's trigger again should close its popup");
    assert!(!tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY));
}

#[test]
fn a_press_outside_an_open_selects_popup_closes_it_and_swallows_the_press() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(tree.node(select).unwrap().state.open);

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(!tree.node(select).unwrap().state.open, "a press well outside the select and its popup should close it");
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { kind: OverlayKind::SelectPopup, .. })));
}

#[test]
fn picking_a_selects_item_row_fires_its_action_and_closes_the_popup() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let row_b = leaf(&mut tree, Some(select), 1, button_ui("b"), (0.0, 32.0, 100.0, 24.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(tree.node(select).unwrap().state.open);

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 40.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 40.0, button: PointerButton::Primary, modifiers: Default::default() });

    let expected = action();
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { intent, .. } if intent.descriptor().action == expected.action)), "picking a row should fire its (merged) action");
    assert!(!tree.node(select).unwrap().state.open, "picking an item should close the popup, per toggle_select_popup's dismissal-paths doc comment");
    let _ = row_b;
}

fn activatable_stack_ui(action: ActionDescriptor) -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("card".into()), presence: UiPresence::default(), activate: Some(action), drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
}

#[test]
fn clicking_an_activatable_stack_fires_its_activate_action() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let _card = leaf(&mut tree, Some(root), 1, activatable_stack_ui(action()), (0.0, 0.0, 100.0, 40.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let expected = action();
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { intent, .. } if intent.descriptor() == expected)), "clicking an activatable Stack should fire its `activate` action");
}

#[test]
fn a_bare_stack_without_activate_or_drop_action_stays_a_hit_test_pass_through() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let _plain = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 40.0));

    assert_eq!(hit_test(&tree, root, 10.0, 10.0), None, "a bare Stack (no activate/drop_action/drag_source) must remain a hit-test pass-through");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeItemNode.hoverAction`/
/// `unhoverAction` are deleted, so a tree row's hover no longer dispatches an ad hoc per-item
/// action here — the framework now owns hover per `UiTreeNode.interactionDomain`
/// (`interactionHover`), wired at a layer above this retained-mode event router.
#[test]
fn hovering_a_tree_row_no_longer_fires_a_per_item_action() {
    let item = UiTreeItemNode::base("row1", Label::data("Row One"));
    let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    let mut router = EventRouter::new("main");

    let entered = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0, modifiers: Default::default() });
    assert!(entered.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "hovering a plain tree row must never fire a per-item action anymore");

    let left = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0, modifiers: Default::default() });
    assert!(left.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "leaving a plain tree row must never fire a per-item action anymore");
}

#[test]
fn pressing_a_draggable_tree_row_then_moving_past_threshold_promotes_it_to_a_drag_session() {
    let mut item = UiTreeItemNode::base("row1", Label::data("Row One"));
    item.draggable = Some(true);
    let payload = DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "{}".to_string())]);
    item.drag_data = Some(payload.clone());
    let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    // `paint::sync_tree_row_layout` is what would normally flip this on (mirroring `item.draggable`)
    // — these tests build the retained tree by hand (no `paint_tree` call), so it's set directly.
    tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    let mut router = EventRouter::new("main");
    router.set_tree_drag_policy(&mut tree, UiDriverDrag::Surface, TreeRowMetrics::from_theme(&semio_framework_ui_theme::Theme::default()));

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture(), Some((row_id, CaptureKind::Press)), "the row must be a real hit-test target once DRAG_SOURCE-flagged");

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 30.0, y: 10.0, modifiers: Default::default() });
    let drag = router.drag_session().expect("moving past the promote threshold should start a DragSession for a draggable row");
    assert_eq!(drag.source, row_id);
    assert_eq!(drag.payload, payload);
}

#[test]
fn handle_driver_arms_only_the_canonical_trailing_tree_handle() {
    let mut item = UiTreeItemNode::base("row1", Label::data("Row One"));
    item.draggable = Some(true);
    item.drag_data = Some(DragPayload::from([("application/x-semio-catalogue-item".to_string(), "{}".to_string())]));
    let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    let mut router = EventRouter::new("main");
    let metrics = TreeRowMetrics::from_theme(&semio_framework_ui_theme::Theme::default());
    router.set_tree_drag_policy(&mut tree, UiDriverDrag::Handle, metrics);

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 30.0, y: 10.0, modifiers: Default::default() });
    assert!(router.drag_session().is_none(), "the label band stays selection-only under the handle driver");
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 30.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let handle = tree_drag_handle_rect(200.0, &metrics);
    let x = handle.x + handle.w * 0.5;
    let y = handle.y + handle.h * 0.5;
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: x - 20.0, y, modifiers: Default::default() });
    assert_eq!(router.drag_session().map(|drag| drag.source), Some(row_id));
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: x - 20.0, y, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(commands.iter().any(|command| matches!(command, UiCommand::DropCancelled { source, .. } if *source == row_id)));
}

#[test]
fn changing_tree_drag_driver_cancels_an_in_flight_surface_drag() {
    let mut item = UiTreeItemNode::base("row1", Label::data("Row One"));
    item.draggable = Some(true);
    let section = UiTreeSectionNode { header_toolbar: None, window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    let mut router = EventRouter::new("main");
    let metrics = TreeRowMetrics::from_theme(&semio_framework_ui_theme::Theme::default());
    router.set_tree_drag_policy(&mut tree, UiDriverDrag::Surface, metrics);
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 30.0, y: 10.0, modifiers: Default::default() });
    assert!(router.drag_session().is_some());

    let commands = router.set_tree_drag_policy(&mut tree, UiDriverDrag::Handle, metrics);
    assert!(router.drag_session().is_none());
    assert_eq!(router.capture(), None);
    assert!(commands.iter().any(|command| matches!(command, UiCommand::DropCancelled { source, .. } if *source == row_id)));
}
//#endregion 🔖️W2InteractivityTests

//#region 🔖️W4SceneCommandTests
/// 🎬️ A minimal `ComponentScene` leaf — every optional per-`SurfaceKind` payload left `None`,
/// mirroring `scene_slots::tests::scene`'s own fixture (this module can't reuse that one directly:
/// it's private to the `scene_slots` submodule).
fn component_scene_ui(surface_id: &str, kind: SurfaceKind) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
        host_id: surface_id.into(),
        surface_id: surface_id.into(),
        controller_id: "ctrl".into(),
        component_kind: kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    })
}

#[test]
fn pointer_down_on_a_component_scene_leaf_emits_a_scene_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let scene_id = leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Canvas2d), (10.0, 10.0, 100.0, 80.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary, modifiers: Default::default() });

    let scene_cmd = commands.iter().find_map(|cmd| match cmd {
        UiCommand::Scene { window_id, node, surface_id, kind, rect, event } => Some((window_id, node, surface_id, kind, rect, event)),
        _ => None,
    });
    let (window_id, node, surface_id, kind, rect, event) = scene_cmd.expect("pointer-down over a ComponentScene leaf should emit UiCommand::Scene");
    assert_eq!(window_id, "main");
    assert_eq!(*node, scene_id);
    assert_eq!(surface_id, "s1");
    assert_eq!(*kind, SurfaceKind::Canvas2d);
    assert_eq!(*rect, Rect::new(10.0, 10.0, 100.0, 80.0), "rect should be the leaf's own absolute layout rect");
    assert_eq!(*event, UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary, modifiers: Default::default() }, "the real event should be carried through verbatim, including its button");
}

#[test]
fn pointer_down_outside_any_component_scene_leaf_emits_no_scene_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Canvas2d), (10.0, 10.0, 100.0, 80.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a press outside the scene's own rect should not emit UiCommand::Scene");
}

#[test]
fn pointer_down_on_a_plain_button_emits_no_scene_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, button_ui("b1"), (0.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a plain widget leaf should never emit UiCommand::Scene");
}

#[test]
fn pointer_move_over_a_component_scene_leaf_emits_a_scene_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::InkCanvas), (0.0, 0.0, 200.0, 200.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 50.0, y: 50.0, modifiers: Default::default() });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { kind: SurfaceKind::InkCanvas, .. })), "moving over a ComponentScene leaf should emit UiCommand::Scene too, not just PointerDown/Up");
}

#[test]
fn scroll_over_a_component_scene_leaf_emits_a_scene_command_and_still_routes_container_scroll() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Table), (0.0, 0.0, 200.0, 200.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 50.0, y: 50.0, delta_x: 0.0, delta_y: 12.0, modifiers: Default::default() });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { kind: SurfaceKind::Table, event: UiEvent::Scroll { .. }, .. })), "wheel input over a ComponentScene leaf should emit UiCommand::Scene carrying the Scroll event");
}

#[test]
fn a_component_scene_nested_under_a_container_resolves_its_absolute_rect() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
    let container = leaf(&mut tree, Some(root), 1, stack_ui(), (20.0, 30.0, 250.0, 250.0));
    let scene_id = leaf(&mut tree, Some(container), 2, component_scene_ui("s1", SurfaceKind::Paint2d), (5.0, 5.0, 100.0, 100.0));
    let mut router = EventRouter::new("main");

    // Absolute position is (20+5, 30+5) = (25, 35); a point inside that rect must hit-test to the scene.
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 40.0, y: 50.0, button: PointerButton::Primary, modifiers: Default::default() });

    let rect = commands.iter().find_map(|cmd| match cmd {
        UiCommand::Scene { node, rect, .. } if *node == scene_id => Some(*rect),
        _ => None,
    });
    assert_eq!(rect, Some(Rect::new(25.0, 35.0, 100.0, 100.0)), "a nested scene's rect should accumulate every ancestor's own layout offset");
}
//#endregion 🔖️W4SceneCommandTests
