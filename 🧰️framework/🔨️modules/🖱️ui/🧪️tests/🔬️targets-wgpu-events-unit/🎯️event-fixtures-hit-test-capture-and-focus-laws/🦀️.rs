/// 🎯️ Event fixtures, hit test, capture and focus laws: the select, tree, input, stepper and stack fixtures, the hit test pruning and scroll offsets, the select popup commit and wheel, the pointer capture and the focus cycling.

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
    UiNode::Tree(UiTreeNode { presentation: Default::default(), sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
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
    UiNode::Input(UiInputNode {
        id: id.into(),
        input_kind: "text".into(),
        value: value.into(),
        placeholder: None,
        accessibility_label: None,
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        precision: None,
        snaps: Vec::new(),
        on_change: action(),
        on_submit: None,
        on_abort: None,
        on_repeat_last: None,
        presence: UiPresence::default(),
        menu: None,
        ..Default::default()
    })
}

fn number_stepper_ui(value: f64, min: f64, max: f64, step: f64, uniform: bool, delta_binding: bool) -> UiNode {
    let absolute = ActionDescriptor { controller_id: "ctrl".into(), action: "absolute".into(), args: None };
    let delta = ActionDescriptor { controller_id: "ctrl".into(), action: if delta_binding { "delta" } else { "" }.into(), args: None };
    UiNode::NumberStepper(UiNumberStepperNode { id: "stepper".into(), value, step, uniform, min: Some(min), max: Some(max), precision: None, on_absolute: absolute, on_delta: delta, presence: UiPresence::default(), menu: None, ..Default::default() })
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

/// 🎚️ A dispatched payload without its press keys (`gesture`, `commit`): the value contract these laws pin; the press
/// shape itself is pinned by `🧪️scrub-press`.
fn without_scrub_press(payload: DslValue) -> DslValue {
    match payload {
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().filter(|(key, _)| key != "gesture" && key != "commit").collect()),
        other => other,
    }
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
    let overflowing_child = leaf(&mut tree, Some(clipper), 1, text_ui("overflow"), (0.0, 0.0, 500.0, 500.0));

    assert_eq!(hit_test(&tree, root, 400.0, 400.0), None, "point outside the clipper must not match the overflowing child");
    assert_eq!(hit_test(&tree, root, 10.0, 10.0), Some(overflowing_child), "inside the clip bounds the child still matches");
}

#[test]
fn hit_test_and_absolute_rect_follow_nested_scroll_offsets() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (10.0, 20.0, 200.0, 60.0));
    set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
    set_flag(&mut tree, root, NodeFlags::CLIPS_CHILDREN);
    tree.node_mut(root).unwrap().state.scroll_offset = (0.0, 48.0);
    let group = leaf(&mut tree, Some(root), 1, stack_ui(), (3.0, 4.0, 190.0, 144.0));
    let hidden = leaf(&mut tree, Some(group), 1, button_ui("hidden"), (0.0, 0.0, 100.0, 24.0));
    let visible = leaf(&mut tree, Some(group), 2, button_ui("visible"), (0.0, 48.0, 100.0, 24.0));
    assert_eq!(tree.absolute_rect(visible), Some(Rect::new(13.0, 24.0, 100.0, 24.0)));
    assert_eq!(hit_test(&tree, root, 20.0, 30.0), Some(visible));
    assert_ne!(hit_test(&tree, root, 20.0, -20.0), Some(hidden));
}

#[test]
fn scrolled_select_popup_commits_a_row_outside_its_scroll_viewport() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let viewport = leaf(&mut tree, Some(root), 1, stack_ui(), (10.0, 20.0, 150.0, 30.0));
    set_flag(&mut tree, viewport, NodeFlags::SCROLLABLE);
    set_flag(&mut tree, viewport, NodeFlags::CLIPS_CHILDREN);
    tree.node_mut(viewport).unwrap().state.scroll_offset = (0.0, 48.0);
    let select = leaf(&mut tree, Some(viewport), 1, select_ui("sel", "a"), (0.0, 48.0, 100.0, 30.0));
    let _row = leaf(&mut tree, Some(select), 1, button_ui("b"), (0.0, 32.0, 100.0, 24.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 30.0, button: PointerButton::Primary, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 20.0, y: 30.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(tree.node(select).unwrap().state.open, "the painted scrolled trigger opens");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 60.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 20.0, y: 60.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(commands.iter().any(|command| matches!(command, UiCommand::App { intent, .. } if intent.descriptor().action == "go")), "the escaped row remains actionable");
    assert!(!tree.node(select).unwrap().state.open);
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
fn wheel_over_an_escaped_select_popup_scrolls_its_accepted_viewport_only() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).unwrap();
    let law = &fixture["select"]["wheel"];
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 160.0, 100.0));
    set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
    let viewport = leaf(&mut tree, Some(root), 1, stack_ui(), (8.0, 62.0, 120.0, 22.4));
    set_flag(&mut tree, viewport, NodeFlags::CLIPS_CHILDREN);
    let mut authored = select_ui("long.item.owner", "0");
    if let UiNode::Select(select) = &mut authored {
        select.items = (0..20).map(|index| UiSelectItem { value: index.to_string(), label: Label::data(index.to_string()) }).collect();
    }
    let select = leaf(&mut tree, Some(viewport), 1, authored, (0.0, 0.0, 120.0, 22.4));
    leaf(&mut tree, Some(root), 2, text_ui("outer extent"), (0.0, 200.0, 120.0, 100.0));
    let mut router = EventRouter::new("wheel-select");
    router.toggle_select_popup(&mut tree, select);
    let theme = semio_framework_ui_theme::Theme::default();
    let popup = semio_framework_ui_select_behavior::select_popup_geometry(Rect::new(8.0, 62.0, 120.0, 22.4), 20, &theme, 100.0, 0.0, 0.0);
    tree.node_mut(select).unwrap().state.select_popup = Some(popup);
    let event = UiEvent::Scroll { x: law["point"]["x"].as_f64().unwrap() as f32, y: law["point"]["y"].as_f64().unwrap() as f32, delta_x: 0.0, delta_y: law["delta"].as_f64().unwrap() as f32, modifiers: Default::default() };
    let commands = router.dispatch(&mut tree, root, &event);
    assert_eq!(commands.len(), law["commandCount"].as_u64().unwrap() as usize);
    assert_eq!(tree.node(root).unwrap().state.scroll_offset.1, law["unrelatedScroll"].as_f64().unwrap() as f32);
    assert_eq!(tree.node(select).unwrap().state.scroll_offset.1, law["expectedScroll"].as_f64().unwrap() as f32);
    assert!(tree.node(select).unwrap().state.open);
    for delta in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 20.0, y: 33.0, delta_x: 0.0, delta_y: delta, modifiers: Default::default() });
        assert_eq!(tree.node(select).unwrap().state.scroll_offset.1, 34.0);
    }
    let mut oracle = taffy::TaffyTree::<()>::new();
    oracle.disable_rounding();
    let rows: Vec<_> = (0..20)
        .map(|_| {
            oracle
                .new_leaf(taffy::Style { size: taffy::geometry::Size { width: taffy::style::Dimension::length(120.0), height: taffy::style::Dimension::length(law["rowHeight"].as_f64().unwrap() as f32) }, flex_shrink: 0.0, ..Default::default() })
                .unwrap()
        })
        .collect();
    let stack = oracle.new_with_children(taffy::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() }, &rows).unwrap();
    oracle.compute_layout(stack, taffy::geometry::Size { width: taffy::style::AvailableSpace::MaxContent, height: taffy::style::AvailableSpace::MaxContent }).unwrap();
    let maximum = oracle.layout(stack).unwrap().size.height + law["viewportPadding"].as_f64().unwrap() as f32 * 2.0 - semio_framework_ui_select_behavior::select_popup_viewport_rect(popup).h;
    assert!((maximum - law["expectedMaximum"].as_f64().unwrap() as f32).abs() < 0.001);
    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 20.0, y: 33.0, delta_x: 0.0, delta_y: 10_000.0, modifiers: Default::default() });
    assert!((tree.node(select).unwrap().state.scroll_offset.1 - maximum).abs() < 0.001);
    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 20.0, y: 33.0, delta_x: 0.0, delta_y: -10_000.0, modifiers: Default::default() });
    assert_eq!(tree.node(select).unwrap().state.scroll_offset.1, 0.0);
}

#[test]
fn keyboard_movement_reveals_the_highlighted_select_row_without_committing_it() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🔽️select-popup-geometry/🔣️.json")).expect("Select popup geometry fixture");
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, fixture["cases"][1]["viewportHeight"].as_f64().expect("viewport height") as f32));
    let mut authored = select_ui("reveal", "row-1");
    if let UiNode::Select(select) = &mut authored {
        select.items = (0..fixture["optionCount"].as_u64().expect("option count") as usize).map(|index| UiSelectItem { value: format!("row-{index}"), label: Label::data(format!("Row {index}")) }).collect();
    }
    let trigger = Rect::new(20.0, 120.0, fixture["triggerWidth"].as_f64().expect("trigger width") as f32, 22.4);
    let select = leaf(&mut tree, Some(root), 1, authored, (trigger.x, trigger.y, trigger.w, trigger.h));
    let theme = semio_framework_ui_theme::Theme::default();
    let popup = semio_framework_ui_select_behavior::select_popup_geometry(trigger, fixture["optionCount"].as_u64().expect("option count") as usize, &theme, tree.node(root).unwrap().layout.height, 0.0, 0.0);
    let mut router = EventRouter::new("select-reveal");
    router.focus.set_focus(&mut tree, Some(select), true);
    router.toggle_select_popup(&mut tree, select);
    tree.node_mut(select).unwrap().state.select_popup = Some(popup);

    router.dispatch(&mut tree, root, &key("End"));

    let state = &tree.node(select).expect("mounted Select").state;
    assert_eq!(state.highlighted, Some(6));
    assert!(state.scroll_offset.1 > 0.0, "moving the active descendant to an offscreen row requests nearest-edge reveal");
    let revealed = semio_framework_ui_select_behavior::select_popup_geometry(trigger, 7, &theme, tree.node(root).unwrap().layout.height, state.scroll_offset.1, 0.0);
    let viewport = semio_framework_ui_select_behavior::select_popup_viewport_rect(revealed);
    let row = semio_framework_ui_select_behavior::select_popup_row_rect(trigger, 6, revealed, &theme);
    assert!(row.y >= viewport.y - 0.001 && row.y + row.h <= viewport.y + viewport.h + 0.001, "the highlighted row is fully visible inside the accepted viewport");
    assert!(matches!(&tree.node(select).unwrap().spec.0, UiNode::Select(node) if node.value == "row-1"), "revealing a highlight does not commit it");
}

#[test]
fn capture_routes_move_and_up_to_the_captured_node_regardless_of_pointer_position() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let a = leaf(&mut tree, Some(root), 1, separator_ui(), (0.0, 0.0, 100.0, 100.0));
    let _b = leaf(&mut tree, Some(root), 2, separator_ui(), (100.0, 0.0, 100.0, 100.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 50.0, y: 50.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture.target.map(|(_, id, _)| id), Some(a));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 150.0, y: 50.0, modifiers: Default::default() });
    assert_eq!(router.hovered, Some(a));

    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 150.0, y: 50.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture.target, None, "capture releases on PointerUp");
}

#[test]
fn foreign_pointer_terminal_cannot_release_retained_capture() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let pressed = leaf(&mut tree, Some(root), 1, separator_ui(), (0.0, 0.0, 100.0, 100.0));
    let mut router = EventRouter::new("main");

    router.dispatch_pointer(&mut tree, root, 77, &UiEvent::PointerDown { x: 50.0, y: 50.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(router.capture.target, Some((77, pressed, CaptureKind::Press)));
    assert!(router.dispatch_pointer(&mut tree, root, 78, &UiEvent::PointerCancel).is_empty());
    assert_eq!(router.capture.target, Some((77, pressed, CaptureKind::Press)));
    assert!(router.dispatch_pointer(&mut tree, root, 78, &UiEvent::PointerUp { x: 50.0, y: 50.0, button: PointerButton::Primary, modifiers: Default::default() }).is_empty());
    assert_eq!(router.capture.target, Some((77, pressed, CaptureKind::Press)));
    router.dispatch_pointer(&mut tree, root, 77, &UiEvent::PointerCancel);
    assert_eq!(router.capture.target, None);
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

    focus.set_focus(&mut tree, Some(a), true);
    assert!(tree.node(a).unwrap().flags.contains(NodeFlags::FOCUSED));
    assert!(!tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED));

    focus.set_focus(&mut tree, Some(b), true);
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

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let expected = action();
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { window_id, intent } if window_id == "main" && intent.descriptor() == expected)));
    let _ = button;
}

#[test]
fn releasing_off_the_captured_button_does_not_fire_its_action() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 40.0, 40.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 90.0, y: 90.0, button: PointerButton::Primary, modifiers: Default::default() });

    assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "release outside the pressed button must not fire its action");
}

fn disabled_button_ui(id: &str) -> UiNode {
    let UiNode::Button(mut button) = button_ui(id) else { unreachable!("button_ui builds a button") };
    button.presence.state = semio_framework_ui_declaration::ui::UiState::Disabled;
    UiNode::Button(button)
}

#[test]
fn a_disabled_button_neither_takes_focus_nor_fires_on_click() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 40.0));
    let enabled = leaf(&mut tree, Some(root), 1, button_ui("enabled"), (0.0, 0.0, 50.0, 40.0));
    leaf(&mut tree, Some(root), 2, disabled_button_ui("disabled"), (50.0, 0.0, 50.0, 40.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 60.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 60.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. } | UiCommand::FocusChanged { node: Some(_), .. })), "a disabled button is inert to the pointer: {commands:?}");
    router.focus.focus_next(&mut tree, root);
    router.focus.focus_next(&mut tree, root);
    assert_eq!(router.focus.focused, Some(enabled), "Tab skips the disabled button");
}

#[test]
fn enter_and_space_activate_the_focused_button_like_a_click() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 40.0));
    leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 50.0, 40.0));
    let mut router = EventRouter::new("main");
    let expected = action();
    let fired = |commands: &[UiCommand]| commands.iter().filter(|cmd| matches!(cmd, UiCommand::App { intent, .. } if intent.descriptor() == expected)).count();

    assert_eq!(fired(&router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Enter".into(), modifiers: EventModifiers::default() })), 0, "Enter with nothing focused activates nothing");
    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() });
    assert_eq!(fired(&router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Enter".into(), modifiers: EventModifiers::default() })), 1, "Enter activates the focused button");
    assert_eq!(fired(&router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: " ".into(), modifiers: EventModifiers::default() })), 1, "Space activates the focused button");
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
