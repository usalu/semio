
use super::*;
use crate::element::ReconciliationKey;

fn leaf(tree: &mut DispatchTree, parent: Option<FrameNodeId>, key: &str, flags: DispatchFlags, listeners: ListenerSet, rect: (f32, f32, f32, f32)) -> FrameNodeId {
    let parent_element = parent.and_then(|id| tree.node(id)).map(|node| node.element);
    let element = ElementId::new(parent_element, &ReconciliationKey::Explicit(key.to_string()));
    let bounds = Bounds::new(rect.0, rect.1, rect.2, rect.3);
    let hitbox = Hitbox { element, bounds, clips_children: flags.contains(DispatchFlags::CLIPS_CHILDREN), hit_transparent: flags.contains(DispatchFlags::HIT_TRANSPARENT) };
    tree.insert(parent, element, flags, listeners, Some(hitbox))
}

fn act(name: &str) -> ActionId {
    ActionId::try_v1("test", name).expect("bounded test action")
}

fn bind(trigger: Trigger, name: &str) -> ActionBinding {
    ActionBinding { trigger, action: act(name), args: None, capability: None }
}

fn listen(bindings: Vec<ActionBinding>) -> ListenerSet {
    ListenerSet { surface: SurfaceId(UiText::try_from_str("s").expect("bounded fixture surface")), node: UiNodeId(1), node_key: UiText::try_from_str("k").expect("bounded fixture key"), revision: UiRevision(0), value: None, bindings }
}

fn listen_editable(value: &str) -> ListenerSet {
    ListenerSet {
        surface: SurfaceId(UiText::try_from_str("s").expect("bounded fixture surface")),
        node: UiNodeId(1),
        node_key: UiText::try_from_str("k").expect("bounded fixture key"),
        revision: UiRevision(0),
        value: Some(UiValue::Text(UiText::try_from_str(value).expect("bounded fixture text"))),
        bindings: Vec::new(),
    }
}

fn ptr(id: u64) -> PointerInfo {
    PointerInfo { id: PointerId(id), kind: PointerKind::Mouse, pressure: None, tilt: None }
}

fn down(id: u64, x: f32, y: f32) -> DispatchEvent {
    DispatchEvent::PointerDown { pointer: ptr(id), x, y, button: PointerButton::Primary }
}

fn up(id: u64, x: f32, y: f32) -> DispatchEvent {
    DispatchEvent::PointerUp { pointer: ptr(id), x, y, button: PointerButton::Primary }
}

fn mv(id: u64, x: f32, y: f32) -> DispatchEvent {
    DispatchEvent::PointerMove { pointer: ptr(id), x, y }
}

#[test]
fn hit_test_finds_the_topmost_of_two_non_overlapping_siblings() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let left = leaf(&mut tree, Some(root), "left", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    let right = leaf(&mut tree, Some(root), "right", DispatchFlags::NONE, listen(vec![]), (100.0, 0.0, 100.0, 100.0));

    assert_eq!(hit_test(&tree, root, 50.0, 50.0), Some(left));
    assert_eq!(hit_test(&tree, root, 150.0, 50.0), Some(right));
}

#[test]
fn hit_test_respects_clips_children_pruning() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let clipper = leaf(&mut tree, Some(root), "clipper", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::CLIPS_CHILDREN, listen(vec![]), (0.0, 0.0, 50.0, 50.0));
    let overflowing_child = leaf(&mut tree, Some(clipper), "overflow", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 500.0, 500.0));

    assert_eq!(hit_test(&tree, root, 400.0, 400.0), None, "point outside the clipper must not match the overflowing child");
    assert_eq!(hit_test(&tree, root, 10.0, 10.0), Some(overflowing_child), "inside the clip bounds the child still matches");
}

#[test]
fn hit_test_skips_hit_transparent_node_but_still_matches_its_children() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let overlay_glass = leaf(&mut tree, Some(root), "glass", DispatchFlags::HIT_TRANSPARENT, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let child = leaf(&mut tree, Some(overlay_glass), "child", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 50.0, 50.0));

    assert_eq!(hit_test(&tree, root, 30.0, 30.0), Some(child));
    assert_eq!(hit_test(&tree, root, 150.0, 150.0), None, "hit-transparent node itself must never match outside its children");
}

#[test]
fn capture_routes_move_and_up_to_the_captured_node_regardless_of_pointer_position() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let a = leaf(&mut tree, Some(root), "a", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    leaf(&mut tree, Some(root), "b", DispatchFlags::NONE, listen(vec![]), (100.0, 0.0, 100.0, 100.0));
    let mut dispatcher = Dispatcher::new();
    let element_a = tree.node(a).unwrap().element;

    dispatcher.dispatch(&tree, &down(1, 50.0, 50.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), Some((element_a, CaptureKind::Press)));

    dispatcher.dispatch(&tree, &mv(1, 150.0, 50.0));
    assert!(dispatcher.is_hovered(element_a), "capture must keep resolving to `a` even once the pointer is over `b`");

    dispatcher.dispatch(&tree, &up(1, 150.0, 50.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), None, "capture releases on PointerUp");
}

#[test]
fn distinct_pointer_identity_storm_saturates_at_sixteen_slots() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    let mut dispatcher = Dispatcher::new();
    for pointer in 0..16 {
        assert!(dispatcher.dispatch(&tree, &down(pointer, 10.0, 10.0)).handled);
        assert!(dispatcher.capture_of(PointerId(pointer)).is_some());
    }
    assert!(!dispatcher.dispatch(&tree, &down(16, 10.0, 10.0)).handled);
    assert!(dispatcher.capture_of(PointerId(16)).is_none());
    assert_eq!(tree.root(), Some(root));
}

#[test]
fn focus_next_and_prev_cycle_only_through_focusable_nodes() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 300.0, 100.0));
    leaf(&mut tree, Some(root), "text", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 50.0, 20.0));
    let button_a = leaf(&mut tree, Some(root), "a", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "a")]), (50.0, 0.0, 50.0, 20.0));
    leaf(&mut tree, Some(root), "sep", DispatchFlags::NONE, listen(vec![]), (100.0, 0.0, 50.0, 20.0));
    let button_b = leaf(&mut tree, Some(root), "b", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "b")]), (150.0, 0.0, 50.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    let element_a = tree.node(button_a).unwrap().element;
    let element_b = tree.node(button_b).unwrap().element;
    let tab = |shift: bool| DispatchEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers { shift, ..Default::default() } };

    dispatcher.dispatch(&tree, &tab(false));
    assert_eq!(dispatcher.focused(), Some(element_a));
    dispatcher.dispatch(&tree, &tab(false));
    assert_eq!(dispatcher.focused(), Some(element_b));
    dispatcher.dispatch(&tree, &tab(false));
    assert_eq!(dispatcher.focused(), Some(element_a), "cycles back to the first focusable node");

    dispatcher.dispatch(&tree, &tab(true));
    assert_eq!(dispatcher.focused(), Some(element_b), "wraps to the last focusable node going backwards");
}

#[test]
fn clicking_a_button_emits_its_action_descriptor_as_a_ui_intent() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    leaf(&mut tree, Some(root), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 100.0, 40.0));
    let mut dispatcher = Dispatcher::new();

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

    assert!(outcome.intents.iter().any(|intent| intent.action == act("go") && intent.trigger == Trigger::Activate));
}

#[test]
fn releasing_off_the_captured_button_does_not_fire_its_action() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    leaf(&mut tree, Some(root), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 40.0, 40.0));
    let mut dispatcher = Dispatcher::new();

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    let outcome = dispatcher.dispatch(&tree, &up(1, 90.0, 90.0));

    assert!(outcome.intents.is_empty(), "release outside the pressed button must not fire its action");
}

#[test]
fn bubble_stops_when_a_handler_returns_true() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    let mid = leaf(&mut tree, Some(root), "mid", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    let leaf_node = leaf(&mut tree, Some(mid), "leaf", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));

    let mut visited = Vec::new();
    bubble(&tree, leaf_node, |id| {
        visited.push(id);
        id == mid
    });

    assert_eq!(visited, vec![leaf_node, mid], "bubbling must stop at `mid` and never reach `root`");
}

//#region 🔖️OverlayTests
#[test]
fn overlay_open_and_close_flips_the_open_overlays_list() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let popup = leaf(&mut tree, Some(root), "popup", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
    let mut dispatcher = Dispatcher::new();
    let popup_element = tree.node(popup).unwrap().element;

    dispatcher.open_overlay(popup_element, OverlayKind::SelectPopup, OverlayAnchor::Element(popup_element));
    assert_eq!(dispatcher.open_overlays().len(), 1);
    assert_eq!(dispatcher.open_overlays()[0].kind, OverlayKind::SelectPopup);

    assert!(dispatcher.close_overlay(&tree, popup_element));
    assert!(dispatcher.open_overlays().is_empty());
}

#[test]
fn pointer_down_outside_a_dismissable_overlay_closes_it_and_swallows_the_press() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "underneath", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let popup = leaf(&mut tree, Some(root), "popup", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::OVERLAY, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
    leaf(&mut tree, Some(popup), "item", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
    let mut dispatcher = Dispatcher::new();
    let popup_element = tree.node(popup).unwrap().element;
    dispatcher.open_overlay(popup_element, OverlayKind::SelectPopup, OverlayAnchor::Element(popup_element));

    let outcome = dispatcher.dispatch(&tree, &down(1, 150.0, 150.0));

    assert!(dispatcher.open_overlays().is_empty(), "outside press must close the overlay");
    assert_eq!(dispatcher.capture_of(PointerId(1)), None, "the outside press must be swallowed, not routed to whatever's underneath");
    assert!(outcome.handled);
}

#[test]
fn pointer_down_inside_a_dismissable_overlay_does_not_close_it() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let popup = leaf(&mut tree, Some(root), "popup", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::OVERLAY, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
    leaf(&mut tree, Some(popup), "item", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
    let mut dispatcher = Dispatcher::new();
    let popup_element = tree.node(popup).unwrap().element;
    dispatcher.open_overlay(popup_element, OverlayKind::SelectPopup, OverlayAnchor::Element(popup_element));

    dispatcher.dispatch(&tree, &down(1, 20.0, 20.0));

    assert!(dispatcher.open_overlays().len() == 1, "a press inside the overlay must not dismiss it");
}

#[test]
fn escape_closes_only_the_topmost_of_two_open_overlays() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let menu = leaf(&mut tree, Some(root), "menu", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 50.0, 50.0));
    let submenu = leaf(&mut tree, Some(root), "submenu", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (60.0, 0.0, 50.0, 50.0));
    let mut dispatcher = Dispatcher::new();
    let menu_element = tree.node(menu).unwrap().element;
    let submenu_element = tree.node(submenu).unwrap().element;
    dispatcher.open_overlay(menu_element, OverlayKind::ContextMenu, OverlayAnchor::Element(menu_element));
    dispatcher.open_overlay(submenu_element, OverlayKind::ContextMenu, OverlayAnchor::Element(menu_element));

    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });

    assert_eq!(dispatcher.open_overlays().len(), 1);
    assert_eq!(dispatcher.open_overlays()[0].root, menu_element, "only the topmost overlay closes on Escape");
}

#[test]
fn tab_focus_is_trapped_inside_an_open_dialog_overlay() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 300.0, 300.0));
    leaf(&mut tree, Some(root), "a", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "a")]), (0.0, 0.0, 50.0, 20.0));
    leaf(&mut tree, Some(root), "b", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "b")]), (50.0, 0.0, 50.0, 20.0));
    let dialog = leaf(&mut tree, Some(root), "dialog", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (100.0, 100.0, 100.0, 100.0));
    let button_c = leaf(&mut tree, Some(dialog), "c", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "c")]), (100.0, 100.0, 50.0, 20.0));
    let button_d = leaf(&mut tree, Some(dialog), "d", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "d")]), (150.0, 100.0, 50.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    let dialog_element = tree.node(dialog).unwrap().element;
    let element_c = tree.node(button_c).unwrap().element;
    let element_d = tree.node(button_d).unwrap().element;
    dispatcher.open_overlay(dialog_element, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });

    let tab = || DispatchEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() };
    dispatcher.dispatch(&tree, &tab());
    assert_eq!(dispatcher.focused(), Some(element_c));
    dispatcher.dispatch(&tree, &tab());
    assert_eq!(dispatcher.focused(), Some(element_d));
    dispatcher.dispatch(&tree, &tab());
    assert_eq!(dispatcher.focused(), Some(element_c), "focus-trapped Tab cycling must never reach a/b outside the dialog");
}

#[test]
fn closing_an_overlay_clears_focus_that_was_inside_it() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let dialog = leaf(&mut tree, Some(root), "dialog", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    let button = leaf(&mut tree, Some(dialog), "ok", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "ok")]), (0.0, 0.0, 50.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    let dialog_element = tree.node(dialog).unwrap().element;
    let button_element = tree.node(button).unwrap().element;
    dispatcher.open_overlay(dialog_element, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });
    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() });
    assert_eq!(dispatcher.focused(), Some(button_element));

    assert!(dispatcher.close_overlay(&tree, dialog_element));

    assert_eq!(dispatcher.focused(), None, "focus dangling into a closed overlay's subtree must be cleared");
}
//#endregion 🔖️OverlayTests

//#region 🔖️DragDropTests
#[test]
fn drag_session_promotes_after_threshold_and_commits_on_an_accepting_drop_target() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let source = leaf(&mut tree, Some(root), "source", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));
    let target = leaf(&mut tree, Some(root), "target", DispatchFlags::DROP_TARGET, listen(vec![bind(Trigger::Drop, "drop")]), (100.0, 100.0, 50.0, 50.0));
    leaf(&mut tree, Some(target), "drop-here", DispatchFlags::NONE, listen(vec![]), (100.0, 100.0, 50.0, 50.0));
    let mut dispatcher = Dispatcher::new();
    let source_element = tree.node(source).unwrap().element;
    let target_element = tree.node(target).unwrap().element;
    let mut payload = DragPayload::new();
    payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\"}".into());
    dispatcher.set_drag_payload(source_element, payload.clone());

    dispatcher.dispatch(&tree, &down(1, 5.0, 5.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Press)), "a plain press must not immediately start a drag");

    dispatcher.dispatch(&tree, &mv(1, 6.0, 6.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Press)));

    dispatcher.dispatch(&tree, &mv(1, 120.0, 120.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Drag)));
    assert_eq!(dispatcher.drag_session().and_then(|drag| drag.drop_target), Some(target_element));

    let outcome = dispatcher.dispatch(&tree, &up(1, 120.0, 120.0));
    assert!(outcome.intents.iter().any(|intent| intent.trigger == Trigger::Drop && intent.action == act("drop") && intent.input == drag_payload_to_value(&payload)));
    assert_eq!(dispatcher.capture_of(PointerId(1)), None);
    assert!(dispatcher.drag_session().is_none());
}

#[test]
fn drag_session_cancels_when_released_over_no_accepting_drop_target() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let source = leaf(&mut tree, Some(root), "source", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    let source_element = tree.node(source).unwrap().element;
    dispatcher.set_drag_payload(source_element, DragPayload::new());

    dispatcher.dispatch(&tree, &down(1, 5.0, 5.0));
    dispatcher.dispatch(&tree, &mv(1, 190.0, 190.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Drag)));

    let outcome = dispatcher.dispatch(&tree, &up(1, 190.0, 190.0));
    assert!(outcome.intents.iter().all(|intent| intent.trigger != Trigger::Drop), "no accepting target means no Drop intent fires");
    assert!(dispatcher.drag_session().is_none());
}

#[test]
fn a_drop_targets_accept_predicate_can_reject_the_active_payload() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let source = leaf(&mut tree, Some(root), "source", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));
    let target = leaf(&mut tree, Some(root), "target", DispatchFlags::DROP_TARGET, listen(vec![bind(Trigger::Drop, "drop")]), (100.0, 100.0, 50.0, 50.0));
    leaf(&mut tree, Some(target), "drop-here", DispatchFlags::NONE, listen(vec![]), (100.0, 100.0, 50.0, 50.0));
    let mut dispatcher = Dispatcher::new();
    let source_element = tree.node(source).unwrap().element;
    let target_element = tree.node(target).unwrap().element;
    dispatcher.set_drag_payload(source_element, DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "x".to_string())]));
    dispatcher.set_drop_accept(target_element, |payload| payload.contains_key("application/x-semio-catalogue-item"));

    dispatcher.dispatch(&tree, &down(1, 5.0, 5.0));
    dispatcher.dispatch(&tree, &mv(1, 120.0, 120.0));

    assert_eq!(dispatcher.drag_session().and_then(|drag| drag.drop_target), None, "the predicate must reject this payload's mime key");
}
//#endregion 🔖️DragDropTests

//#region 🔖️ScrollTests
#[test]
fn scroll_routes_to_the_nearest_scrollable_ancestor_and_clamps_at_zero() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::SCROLLABLE, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "content", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 20.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    let root_element = tree.node(root).unwrap().element;

    dispatcher.dispatch(&tree, &DispatchEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: 30.0 });
    assert_eq!(dispatcher.scroll_offset(root_element), (0.0, 30.0));

    dispatcher.dispatch(&tree, &DispatchEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: -100.0 });
    assert_eq!(dispatcher.scroll_offset(root_element), (0.0, 0.0), "scroll offset must clamp at zero, not go negative");
}

#[test]
fn scroll_thumb_capture_drags_the_scrollable_offset_along_its_registered_axis() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::SCROLLABLE, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let thumb = leaf(&mut tree, Some(root), "thumb", DispatchFlags::NONE, listen(vec![]), (190.0, 0.0, 10.0, 40.0));
    let mut dispatcher = Dispatcher::new();
    let root_element = tree.node(root).unwrap().element;
    let thumb_element = tree.node(thumb).unwrap().element;
    dispatcher.register_scroll_thumb(thumb_element, root_element, ScrollAxis::Vertical);

    dispatcher.dispatch(&tree, &down(1, 195.0, 5.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), Some((root_element, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));

    dispatcher.dispatch(&tree, &mv(1, 195.0, 25.0));
    assert_eq!(dispatcher.scroll_offset(root_element), (0.0, 20.0));

    dispatcher.dispatch(&tree, &up(1, 195.0, 25.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), None);
}
//#endregion 🔖️ScrollTests

//#region 🔖️EditStateTests
#[test]
fn focusing_an_input_seeds_edit_state_from_its_value_and_blur_clears_it() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let input = leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("hello"), (0.0, 0.0, 100.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    let input_element = tree.node(input).unwrap().element;

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    assert_eq!(dispatcher.edit_state(input_element), Some(&EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None }));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

    dispatcher.dispatch(&tree, &down(1, 190.0, 190.0));
    assert_eq!(dispatcher.edit_state(input_element), None, "blur must relinquish the buffer so the declarative value governs again");
}

#[test]
fn arrow_keys_move_the_caret_and_backspace_deletes_the_previous_char() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("abc"), (0.0, 0.0, 100.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    let input_element = dispatcher.focused().unwrap();

    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers::default() });
    assert_eq!(dispatcher.edit_state(input_element).unwrap().caret, 2);

    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    let edit = dispatcher.edit_state(input_element).unwrap();
    assert_eq!((edit.anchor, edit.caret), (2, 1), "shift+arrow extends the selection instead of collapsing it");

    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Backspace".into(), modifiers: EventModifiers::default() });
    let edit = dispatcher.edit_state(input_element).unwrap();
    assert_eq!(edit.text, "ac", "backspace over a selection deletes the selected range");
    assert_eq!((edit.anchor, edit.caret), (1, 1));
}

#[test]
fn character_insertion_replaces_the_selection() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("abc"), (0.0, 0.0, 100.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    let input_element = dispatcher.focused().unwrap();

    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    dispatcher.dispatch(&tree, &DispatchEvent::TextInput { text: "xyz".into() });

    let edit = dispatcher.edit_state(input_element).unwrap();
    assert_eq!(edit.text, "xyz");
    assert_eq!((edit.anchor, edit.caret), (3, 3));
}

#[test]
fn copy_over_a_selection_emits_a_clipboard_intent_without_mutating_the_buffer() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("hello"), (0.0, 0.0, 100.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    let input_element = dispatcher.focused().unwrap();

    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
    dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    let outcome = dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "c".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });

    assert!(outcome.intents.iter().any(|intent| { intent.action == ActionId::try_v1("dispatch", "clipboardCopy").expect("bounded test action") && intent.input == Some(UiValue::Text(UiText::try_from_str("hello").expect("bounded fixture text"))) }));
    assert_eq!(dispatcher.edit_state(input_element).unwrap().text, "hello", "copy must not mutate the buffer");
}

#[test]
fn ime_commit_inserts_the_composed_text_and_clears_composition() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable(""), (0.0, 0.0, 100.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    let input_element = dispatcher.focused().unwrap();

    dispatcher.dispatch(&tree, &DispatchEvent::Ime(ImeEvent::Start));
    dispatcher.dispatch(&tree, &DispatchEvent::Ime(ImeEvent::Update { text: "ねこ".into(), cursor: 2 }));
    assert_eq!(dispatcher.edit_state(input_element).unwrap().composition.as_deref(), Some("ねこ"));

    dispatcher.dispatch(&tree, &DispatchEvent::Ime(ImeEvent::Commit { text: "ねこ".into() }));
    let edit = dispatcher.edit_state(input_element).unwrap();
    assert_eq!(edit.text, "ねこ");
    assert_eq!(edit.composition, None);
}
//#endregion 🔖️EditStateTests

//#region 🔖️HoverRevealTests
#[test]
fn hovering_a_leaf_marks_its_whole_ancestor_chain_hovered_and_clearing_hover_clears_it_all() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    let row = leaf(&mut tree, Some(root), "row", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    let label = leaf(&mut tree, Some(row), "label", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 50.0, 20.0));
    let mut dispatcher = Dispatcher::new();
    let root_element = tree.node(root).unwrap().element;
    let row_element = tree.node(row).unwrap().element;
    let label_element = tree.node(label).unwrap().element;

    dispatcher.dispatch(&tree, &mv(1, 10.0, 10.0));
    assert!(dispatcher.is_hovered(label_element));
    assert!(dispatcher.is_hovered(row_element), "an ancestor layout container must observe hover too, for a host's reveal-on-hover styling");
    assert!(dispatcher.is_hovered(root_element));

    dispatcher.dispatch(&tree, &mv(1, 500.0, 500.0));
    assert!(!dispatcher.is_hovered(label_element));
    assert!(!dispatcher.is_hovered(row_element));
    assert!(!dispatcher.is_hovered(root_element));
}
//#endregion 🔖️HoverRevealTests

//#region 🔖️W2InteractivityTests
// 🔽️ Generic replacement for `events.rs`'s `Select`-open/close and `Stack.activate`/`drop_action`/
// `Tree`-row-`draggable` wiring — see this file's module docstring for why the mechanism is now
// flag-driven (`OVERLAY_TRIGGER`/`LAYOUT_CONTAINER`/`DRAG_SOURCE`) instead of matching product enum
// variants, and `find_tree_item_spec`'s own doc-comment-documented product knowledge is gone
// entirely: `Dispatcher::set_drag_payload` is the whole registration surface now, callable by any
// element regardless of its product shape.
#[test]
fn clicking_an_overlay_trigger_opens_its_popup_and_clicking_again_closes_it() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), "select", DispatchFlags::FOCUSABLE | DispatchFlags::OVERLAY_TRIGGER, listen(vec![bind(Trigger::Activate, "toggle")]), (0.0, 0.0, 100.0, 30.0));
    let mut dispatcher = Dispatcher::new();
    let select_element = tree.node(select).unwrap().element;

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    assert_eq!(dispatcher.open_overlays().len(), 1, "clicking a closed overlay trigger should open its popup");
    assert_eq!(dispatcher.open_overlays()[0].root, select_element);

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    assert!(dispatcher.open_overlays().is_empty(), "clicking an open trigger again should close its popup");
}

#[test]
fn a_press_outside_an_open_overlay_trigger_closes_it_and_swallows_the_press() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), "select", DispatchFlags::FOCUSABLE | DispatchFlags::OVERLAY_TRIGGER | DispatchFlags::OVERLAY, listen(vec![bind(Trigger::Activate, "toggle")]), (0.0, 0.0, 100.0, 30.0));
    let mut dispatcher = Dispatcher::new();
    let _ = tree.node(select).unwrap().element;
    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    assert_eq!(dispatcher.open_overlays().len(), 1);

    let outcome = dispatcher.dispatch(&tree, &down(1, 190.0, 190.0));

    assert!(dispatcher.open_overlays().is_empty(), "a press well outside the trigger and its popup should close it");
    assert!(outcome.handled);
}

#[test]
fn picking_a_descendant_row_fires_its_action_and_closes_the_popup() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), "select", DispatchFlags::FOCUSABLE | DispatchFlags::OVERLAY_TRIGGER | DispatchFlags::OVERLAY, listen(vec![bind(Trigger::Activate, "toggle")]), (0.0, 0.0, 100.0, 30.0));
    leaf(&mut tree, Some(select), "row-b", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "pick-b")]), (0.0, 32.0, 100.0, 24.0));
    let mut dispatcher = Dispatcher::new();

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
    assert_eq!(dispatcher.open_overlays().len(), 1);

    dispatcher.dispatch(&tree, &down(1, 10.0, 40.0));
    let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 40.0));

    assert!(outcome.intents.iter().any(|intent| intent.action == act("pick-b")), "picking a row should fire its own action");
    assert!(dispatcher.open_overlays().is_empty(), "picking an item should close the popup");
    let _ = select;
}

#[test]
fn clicking_an_activatable_layout_container_fires_its_activate_action() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "card", DispatchFlags::LAYOUT_CONTAINER, listen(vec![bind(Trigger::Activate, "open-card")]), (0.0, 0.0, 100.0, 40.0));
    let mut dispatcher = Dispatcher::new();

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

    assert!(outcome.intents.iter().any(|intent| intent.action == act("open-card")), "a LAYOUT_CONTAINER node with its own binding must still fire it");
}

#[test]
fn a_bare_layout_container_without_bindings_stays_a_hit_test_pass_through() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "plain", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 40.0));

    assert_eq!(hit_test(&tree, root, 10.0, 10.0), None, "a bare layout container (no bindings/drag source) must remain a hit-test pass-through");
}

#[test]
fn pressing_a_drag_source_then_moving_past_threshold_promotes_it_to_a_drag_session() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    let row = leaf(&mut tree, Some(root), "row", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 200.0, 24.0));
    let mut dispatcher = Dispatcher::new();
    let row_element = tree.node(row).unwrap().element;
    let payload = DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "{}".to_string())]);
    dispatcher.set_drag_payload(row_element, payload.clone());

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    assert_eq!(dispatcher.capture_of(PointerId(1)), Some((row_element, CaptureKind::Press)), "the row must be a real hit-test target once DRAG_SOURCE-flagged");

    dispatcher.dispatch(&tree, &mv(1, 30.0, 10.0));
    let drag = dispatcher.drag_session().expect("moving past the promote threshold should start a DragSession for a draggable row");
    assert_eq!(drag.source, row_element);
    assert_eq!(drag.payload, payload);
}
//#endregion 🔖️W2InteractivityTests

//#region 🔖️W4SceneCommandTests
// 🎬️ `events.rs`'s `UiCommand::Scene` routed a raw event into a `ComponentScene` leaf by matching
// that one `UiNode` variant — the exact per-product special-casing this port eliminates (see this
// file's report). The replacement property to test is generic multi-binding dispatch: any node can
// declare several typed bindings, and only the one matching the interaction that actually occurred
// fires, with zero dispatch.rs code paths caring what the node "is".
#[test]
fn a_node_with_multiple_typed_bindings_only_fires_the_one_matching_the_interaction() {
    let mut tree = DispatchTree::new(UiRevision(0));
    let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), "control", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "activate-name"), bind(Trigger::Delta, "delta-name")]), (0.0, 0.0, 200.0, 20.0));
    let mut dispatcher = Dispatcher::new();

    dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
    let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

    assert!(outcome.intents.iter().any(|intent| intent.trigger == Trigger::Activate && intent.action == act("activate-name")));
    assert!(outcome.intents.iter().all(|intent| intent.trigger != Trigger::Delta), "only the Activate binding should fire from a pointer release, regardless of what other triggers the same node also declares");
}
//#endregion 🔖️W4SceneCommandTests

//#region 🔖️StaleRevisionTests
#[test]
fn stale_captured_activate_is_rejected_rather_than_misapplied() {
    let mut tree_at_capture = DispatchTree::new(UiRevision(5));
    let root = leaf(&mut tree_at_capture, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    leaf(&mut tree_at_capture, Some(root), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 100.0, 40.0));
    let mut dispatcher = Dispatcher::new();
    dispatcher.dispatch(&tree_at_capture, &down(1, 10.0, 10.0));
    assert!(dispatcher.capture_of(PointerId(1)).is_some());

    let mut tree_later = DispatchTree::new(UiRevision(8));
    let root_later = leaf(&mut tree_later, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
    leaf(&mut tree_later, Some(root_later), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 100.0, 40.0));

    let outcome = dispatcher.dispatch(&tree_later, &up(1, 10.0, 10.0));

    assert!(outcome.intents.is_empty(), "a capture that outlived 3 revisions of unrelated churn must not fire its stale Activate");
    assert_eq!(dispatcher.capture_of(PointerId(1)), None, "capture is still released even though the intent is rejected");
}
//#endregion 🔖️StaleRevisionTests
