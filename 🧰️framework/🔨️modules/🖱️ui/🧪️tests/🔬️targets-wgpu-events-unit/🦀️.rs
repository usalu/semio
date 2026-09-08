
use super::*;
use crate::wgpu::IconName;
use crate::wgpu::Label;
use crate::wgpu::component::ui::{UiButtonNode, UiComponentSceneNode, UiInputNode, UiPresence, UiSelectItem, UiSelectNode, UiSeparatorNode, UiStackNode, UiTextNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::tree::{Node, NodeKey, WidgetSpec};

fn action() -> ActionDescriptor {
    ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
}

fn select_ui(id: &str, value: &str) -> UiNode {
    UiNode::Select(UiSelectNode {
        id: id.into(),
        value: value.into(),
        items: vec![UiSelectItem { value: "a".into(), label: Label::data("A") }, UiSelectItem { value: "b".into(), label: Label::data("B") }],
        placeholder: None,
        on_change: action(),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn tree_ui(sections: Vec<UiTreeSectionNode>) -> UiNode {
    UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
}

/// 🌳️ Manually inserts a `Tree` row `Stack` (mirroring `reconcile::tree_item_row`'s synthesized
/// shape/key exactly) as a retained child of `tree_id` — these tests build the retained tree by
/// hand (like every other test in this module, via `leaf`), so `reconcile` never actually runs;
/// this stand-in keeps the row's key (`NodeKey::Explicit(item.id)`) and geometry consistent with
/// what `paint::sync_tree_row_layout` would have written.
fn insert_tree_row(tree: &mut UiTree, tree_id: NodeId, item_id: &str, rect: (f32, f32, f32, f32)) -> NodeId {
    let spec = UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(item_id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
    let id = tree.insert_child(Some(tree_id), Node::new(NodeKey::Explicit(item_id.into()), WidgetSpec(spec)));
    let bucket = tree.node_mut(id).unwrap();
    bucket.layout.x = rect.0;
    bucket.layout.y = rect.1;
    bucket.layout.width = rect.2;
    bucket.layout.height = rect.3;
    id
}

fn input_ui(id: &str, value: &str) -> UiNode {
    UiNode::Input(UiInputNode { id: id.into(), input_kind: "text".into(), value: value.into(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, on_change: action(), presence: UiPresence::default(), menu: None })
}

fn stack_ui() -> UiNode {
    UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
}

fn text_ui(value: &str) -> UiNode {
    UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
}

fn separator_ui() -> UiNode {
    UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })
}

fn button_ui(id: &str) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(id), action: action(), style: None, presence: UiPresence::default(), menu: None })
}

fn leaf(tree: &mut UiTree, parent: Option<NodeId>, ordinal: u32, node: UiNode, rect: (f32, f32, f32, f32)) -> NodeId {
    let id = tree.insert_child(parent, Node::new(NodeKey::Positional(ordinal, ordinal), WidgetSpec(node)));
    let bucket = tree.node_mut(id).unwrap();
    bucket.layout.x = rect.0;
    bucket.layout.y = rect.1;
    bucket.layout.width = rect.2;
    bucket.layout.height = rect.3;
    id
}

fn set_flag(tree: &mut UiTree, id: NodeId, flag: NodeFlags) {
    tree.node_mut(id).unwrap().flags.set(flag, true);
}

#[test]
fn hit_test_finds_the_topmost_of_two_non_overlapping_siblings() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let left = leaf(&mut tree, Some(root), 1, text_ui("left"), (0.0, 0.0, 100.0, 100.0));
    let right = leaf(&mut tree, Some(root), 2, text_ui("right"), (100.0, 0.0, 100.0, 100.0));

    assert_eq!(hit_test(&tree, root, 50.0, 50.0), Some(left));
    assert_eq!(hit_test(&tree, root, 150.0, 50.0), Some(right));
}

#[test]
fn hit_test_respects_clips_children_pruning() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let clipper = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 50.0, 50.0));
    set_flag(&mut tree, clipper, NodeFlags::CLIPS_CHILDREN);
    // child's own rect extends far outside the clipper's 50x50 bounds.
    let overflowing_child = leaf(&mut tree, Some(clipper), 1, text_ui("overflow"), (0.0, 0.0, 500.0, 500.0));

    assert_eq!(hit_test(&tree, root, 400.0, 400.0), None, "point outside the clipper must not match the overflowing child");
    assert_eq!(hit_test(&tree, root, 10.0, 10.0), Some(overflowing_child), "inside the clip bounds the child still matches");
}

#[test]
fn hit_test_skips_hit_transparent_node_but_still_matches_its_children() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let overlay_glass = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    set_flag(&mut tree, overlay_glass, NodeFlags::HIT_TRANSPARENT);
    let child = leaf(&mut tree, Some(overlay_glass), 1, text_ui("under-glass"), (10.0, 10.0, 50.0, 50.0));

    assert_eq!(hit_test(&tree, root, 30.0, 30.0), Some(child));
    assert_eq!(hit_test(&tree, root, 150.0, 150.0), None, "hit-transparent node itself must never match outside its children");
}

#[test]
fn capture_routes_move_and_up_to_the_captured_node_regardless_of_pointer_position() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let a = leaf(&mut tree, Some(root), 1, separator_ui(), (0.0, 0.0, 100.0, 100.0));
    let _b = leaf(&mut tree, Some(root), 2, separator_ui(), (100.0, 0.0, 100.0, 100.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 50.0, y: 50.0, button: PointerButton::Primary });
    assert_eq!(router.capture.target.map(|(id, _)| id), Some(a));

    // pointer moved far outside `a`'s bounds and into `b`'s — capture must still target `a`.
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 150.0, y: 50.0 });
    assert_eq!(router.hovered, Some(a));

    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 150.0, y: 50.0, button: PointerButton::Primary });
    assert_eq!(router.capture.target, None, "capture releases on PointerUp");
}

#[test]
fn focus_next_and_prev_cycle_only_through_focusable_nodes() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 100.0));
    leaf(&mut tree, Some(root), 1, text_ui("not focusable"), (0.0, 0.0, 50.0, 20.0));
    let button_a = leaf(&mut tree, Some(root), 2, button_ui("a"), (50.0, 0.0, 50.0, 20.0));
    leaf(&mut tree, Some(root), 3, separator_ui(), (100.0, 0.0, 50.0, 20.0));
    let button_b = leaf(&mut tree, Some(root), 4, button_ui("b"), (150.0, 0.0, 50.0, 20.0));
    let mut focus = FocusState::new();

    focus.focus_next(&mut tree, root);
    assert_eq!(focus.focused, Some(button_a));
    focus.focus_next(&mut tree, root);
    assert_eq!(focus.focused, Some(button_b));
    focus.focus_next(&mut tree, root);
    assert_eq!(focus.focused, Some(button_a), "cycles back to the first focusable node");

    focus.focus_prev(&mut tree, root);
    assert_eq!(focus.focused, Some(button_b), "wraps to the last focusable node going backwards");
}

#[test]
fn set_focus_flips_the_focused_flag_in_both_directions() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let a = leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
    let b = leaf(&mut tree, Some(root), 2, button_ui("b"), (0.0, 20.0, 50.0, 20.0));
    let mut focus = FocusState::new();

    focus.set_focus(&mut tree, Some(a));
    assert!(tree.node(a).unwrap().flags.contains(NodeFlags::FOCUSED));
    assert!(!tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED));

    focus.set_focus(&mut tree, Some(b));
    assert!(!tree.node(a).unwrap().flags.contains(NodeFlags::FOCUSED), "moving focus away must clear the old node's flag");
    assert!(tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED));

    focus.clear_focus(&mut tree);
    assert!(!tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED), "clearing focus must clear the flag, not just the router's own field");
}

#[test]
fn clicking_a_button_emits_its_action_descriptor_as_a_ui_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let button = leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 100.0, 40.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

    let expected = action();
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { window_id, action } if window_id == "main" && *action == expected)));
    let _ = button;
}

#[test]
fn releasing_off_the_captured_button_does_not_fire_its_action() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 40.0, 40.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 90.0, y: 90.0, button: PointerButton::Primary });

    assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "release outside the pressed button must not fire its action");
}

#[test]
fn bubble_stops_when_a_handler_returns_true() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let mid = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let leaf_node = leaf(&mut tree, Some(mid), 1, text_ui("leaf"), (0.0, 0.0, 20.0, 20.0));

    let mut visited = Vec::new();
    bubble(&tree, leaf_node, |id| {
        visited.push(id);
        id == mid
    });

    assert_eq!(visited, vec![leaf_node, mid], "bubbling must stop at `mid` and never reach `root`");
}

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

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary });

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

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Primary });

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

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
    assert_eq!(router.capture(), Some((source, CaptureKind::Press)), "a plain press must not immediately start a drag");

    // Small move under the promotion threshold: still just a Press.
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 6.0, y: 6.0 });
    assert_eq!(router.capture(), Some((source, CaptureKind::Press)));

    // Move past the threshold and over the drop target: promotes to Drag and finds the target.
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0 });
    assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));
    assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), Some(target));

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 120.0, y: 120.0, button: PointerButton::Primary });
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

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0 });
    assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 190.0, y: 190.0, button: PointerButton::Primary });
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

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0 });

    assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), None, "the predicate must reject this payload's mime key");
}
//#endregion 🔖️DragDropTests

//#region 🔖️ScrollTests
#[test]
fn scroll_routes_to_the_nearest_scrollable_ancestor_and_clamps_at_zero() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
    leaf(&mut tree, Some(root), 1, text_ui("content"), (10.0, 10.0, 20.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: 30.0 });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 30.0));

    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: -100.0 });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 0.0), "scroll offset must clamp at zero, not go negative");
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

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 195.0, y: 5.0, button: PointerButton::Primary });
    assert_eq!(router.capture(), Some((root, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 195.0, y: 25.0 });
    assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 20.0));

    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 195.0, y: 25.0, button: PointerButton::Primary });
    assert_eq!(router.capture(), None);
}
//#endregion 🔖️ScrollTests

//#region 🔖️EditStateTests
#[test]
fn focusing_an_input_seeds_edit_state_from_its_value_and_blur_clears_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let input = leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    assert_eq!(tree.node(input).unwrap().state.edit, Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 }));

    // clicking empty space blurs.
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary });
    assert_eq!(tree.node(input).unwrap().state.edit, None, "blur must relinquish the buffer so the declarative value governs again");
}

#[test]
fn arrow_keys_move_the_caret_and_backspace_deletes_the_previous_char() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers::default() });
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().caret, 2);

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!((edit.anchor, edit.caret), (2, 1), "shift+arrow extends the selection instead of collapsing it");

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Backspace".into(), modifiers: EventModifiers::default() });
    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!(edit.text, "ac", "backspace over a selection deletes the selected range");
    assert_eq!((edit.anchor, edit.caret), (1, 1));
}

#[test]
fn character_insertion_replaces_the_selection() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "xyz".into() });

    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!(edit.text, "xyz");
    assert_eq!((edit.anchor, edit.caret), (3, 3));
}

#[test]
fn copy_over_a_selection_emits_a_clipboard_command_without_mutating_the_buffer() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "c".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::ClipboardCopy { text, .. } if text == "hello")));
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().text, "hello", "copy must not mutate the buffer");
}

#[test]
fn ime_commit_inserts_the_composed_text_and_clears_composition() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", ""), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Start));
    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Update { text: "ねこ".into(), cursor: 2 }));
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().composition.as_deref(), Some("ねこ"));

    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Commit { text: "ねこ".into() }));
    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!(edit.text, "ねこ");
    assert_eq!(edit.composition, None);
}
//#endregion 🔖️EditStateTests

//#region 🔖️HoverRevealTests
#[test]
fn hovering_a_leaf_marks_its_whole_ancestor_chain_hovered_and_clearing_hover_clears_it_all() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let row = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let label = leaf(&mut tree, Some(row), 1, text_ui("item"), (0.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0 });
    assert!(tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
    assert!(tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED), "an ancestor Stack row must observe hover too, for paint's reveal-on-hover");
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 500.0, y: 500.0 });
    assert!(!tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
    assert!(!tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED));
    assert!(!tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));
}
//#endregion 🔖️HoverRevealTests

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

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
    assert!(tree.node(select).unwrap().state.open, "clicking a closed select should open its popup");
    assert!(tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY), "an open select's popup subtree should win hit-test priority over its siblings");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
    assert!(!tree.node(select).unwrap().state.open, "clicking an open select's trigger again should close its popup");
    assert!(!tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY));
}

#[test]
fn a_press_outside_an_open_selects_popup_closes_it_and_swallows_the_press() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
    assert!(tree.node(select).unwrap().state.open);

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary });

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
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
    assert!(tree.node(select).unwrap().state.open);

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 40.0, button: PointerButton::Primary });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 40.0, button: PointerButton::Primary });

    let expected = action();
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == expected)), "picking a row should fire its (merged) action");
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

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

    let expected = action();
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == expected)), "clicking an activatable Stack should fire its `activate` action");
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
    let section = UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    let mut router = EventRouter::new("main");

    let entered = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0 });
    assert!(entered.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "hovering a plain tree row must never fire a per-item action anymore");

    let left = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0 });
    assert!(left.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "leaving a plain tree row must never fire a per-item action anymore");
}

#[test]
fn pressing_a_draggable_tree_row_then_moving_past_threshold_promotes_it_to_a_drag_session() {
    let mut item = UiTreeItemNode::base("row1", Label::data("Row One"));
    item.draggable = Some(true);
    let payload = DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "{}".to_string())]);
    item.drag_data = Some(payload.clone());
    let section = UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    // `paint::sync_tree_row_layout` is what would normally flip this on (mirroring `item.draggable`)
    // — these tests build the retained tree by hand (no `paint_tree` call), so it's set directly.
    tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
    assert_eq!(router.capture(), Some((row_id, CaptureKind::Press)), "the row must be a real hit-test target once DRAG_SOURCE-flagged");

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 30.0, y: 10.0 });
    let drag = router.drag_session().expect("moving past the promote threshold should start a DragSession for a draggable row");
    assert_eq!(drag.source, row_id);
    assert_eq!(drag.payload, payload);
}
//#endregion 🔖️W2InteractivityTests

//#region 🔖️W4SceneCommandTests
/// 🎬️ A minimal `ComponentScene` leaf — every optional per-`SurfaceKind` payload left `None`,
/// mirroring `scene_slots::tests::scene`'s own fixture (this module can't reuse that one directly:
/// it's private to the `scene_slots` submodule).
fn component_scene_ui(surface_id: &str, kind: SurfaceKind) -> UiNode {
    UiNode::ComponentScene(UiComponentSceneNode {
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

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary });

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
    assert_eq!(*event, UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary }, "the real event should be carried through verbatim, including its button");
}

#[test]
fn pointer_down_outside_any_component_scene_leaf_emits_no_scene_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Canvas2d), (10.0, 10.0, 100.0, 80.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary });

    assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a press outside the scene's own rect should not emit UiCommand::Scene");
}

#[test]
fn pointer_down_on_a_plain_button_emits_no_scene_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, button_ui("b1"), (0.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });

    assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a plain widget leaf should never emit UiCommand::Scene");
}

#[test]
fn pointer_move_over_a_component_scene_leaf_emits_a_scene_command() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::InkCanvas), (0.0, 0.0, 200.0, 200.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 50.0, y: 50.0 });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { kind: SurfaceKind::InkCanvas, .. })), "moving over a ComponentScene leaf should emit UiCommand::Scene too, not just PointerDown/Up");
}

#[test]
fn scroll_over_a_component_scene_leaf_emits_a_scene_command_and_still_routes_container_scroll() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Table), (0.0, 0.0, 200.0, 200.0));
    let mut router = EventRouter::new("main");

    let commands = router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 50.0, y: 50.0, delta_x: 0.0, delta_y: 12.0 });

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
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 40.0, y: 50.0, button: PointerButton::Primary });

    let rect = commands.iter().find_map(|cmd| match cmd {
        UiCommand::Scene { node, rect, .. } if *node == scene_id => Some(*rect),
        _ => None,
    });
    assert_eq!(rect, Some(Rect::new(25.0, 35.0, 100.0, 100.0)), "a nested scene's rect should accumulate every ancestor's own layout offset");
}
//#endregion 🔖️W4SceneCommandTests
