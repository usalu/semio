use super::*;
use crate::wgpu::component::ui::{UiButtonNode, UiComponentSceneNode, UiInputNode, UiPresence, UiSelectItem, UiSelectNode, UiSeparatorNode, UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::tree::{Node, NodeKey, WidgetSpec};
use crate::wgpu::IconName;
use crate::wgpu::Label;

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
        on_change: action(),
        on_submit: None,
        on_abort: None,
        on_repeat_last: None,
        presence: UiPresence::default(),
        menu: None,
    })
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔽️retained-select-overlay-raster/🔣️.json")).unwrap();
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
    let theme = crate::wgpu::theme::Theme::default();
    let popup = crate::wgpu::select::select_popup_geometry(Rect::new(8.0, 62.0, 120.0, 22.4), 20, &theme, 100.0, 0.0, 0.0);
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
    let rows: Vec<_> = (0..20).map(|_| oracle.new_leaf(taffy::Style { size: taffy::geometry::Size { width: taffy::style::Dimension::length(120.0), height: taffy::style::Dimension::length(law["rowHeight"].as_f64().unwrap() as f32) }, flex_shrink: 0.0, ..Default::default() }).unwrap()).collect();
    let stack = oracle.new_with_children(taffy::Style { flex_direction: taffy::style::FlexDirection::Column, ..Default::default() }, &rows).unwrap();
    oracle.compute_layout(stack, taffy::geometry::Size { width: taffy::style::AvailableSpace::MaxContent, height: taffy::style::AvailableSpace::MaxContent }).unwrap();
    let maximum = oracle.layout(stack).unwrap().size.height + law["viewportPadding"].as_f64().unwrap() as f32 * 2.0 - crate::wgpu::select::select_popup_viewport_rect(popup).h;
    assert!((maximum - law["expectedMaximum"].as_f64().unwrap() as f32).abs() < 0.001);
    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 20.0, y: 33.0, delta_x: 0.0, delta_y: 10_000.0, modifiers: Default::default() });
    assert!((tree.node(select).unwrap().state.scroll_offset.1 - maximum).abs() < 0.001);
    router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 20.0, y: 33.0, delta_x: 0.0, delta_y: -10_000.0, modifiers: Default::default() });
    assert_eq!(tree.node(select).unwrap().state.scroll_offset.1, 0.0);
    println!("[DEBUG] escaped Select wheel offset={} outer={}", tree.node(select).unwrap().state.scroll_offset.1, tree.node(root).unwrap().state.scroll_offset.1);
}

#[test]
fn keyboard_movement_reveals_the_highlighted_select_row_without_committing_it() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔽️select-popup-geometry/🔣️.json")).expect("Select popup geometry fixture");
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, fixture["cases"][1]["viewportHeight"].as_f64().expect("viewport height") as f32));
    let mut authored = select_ui("reveal", "row-1");
    if let UiNode::Select(select) = &mut authored {
        select.items = (0..fixture["optionCount"].as_u64().expect("option count") as usize)
            .map(|index| UiSelectItem { value: format!("row-{index}"), label: Label::data(format!("Row {index}")) })
            .collect();
    }
    let trigger = Rect::new(20.0, 120.0, fixture["triggerWidth"].as_f64().expect("trigger width") as f32, 22.4);
    let select = leaf(&mut tree, Some(root), 1, authored, (trigger.x, trigger.y, trigger.w, trigger.h));
    let theme = crate::wgpu::theme::Theme::default();
    let popup = crate::wgpu::select::select_popup_geometry(trigger, fixture["optionCount"].as_u64().expect("option count") as usize, &theme, tree.node(root).unwrap().layout.height, 0.0, 0.0);
    let mut router = EventRouter::new("select-reveal");
    router.focus.set_focus(&mut tree, Some(select), true);
    router.toggle_select_popup(&mut tree, select);
    tree.node_mut(select).unwrap().state.select_popup = Some(popup);

    router.dispatch(&mut tree, root, &key("End"));

    let state = &tree.node(select).expect("mounted Select").state;
    assert_eq!(state.highlighted, Some(6));
    assert!(state.scroll_offset.1 > 0.0, "moving the active descendant to an offscreen row requests nearest-edge reveal");
    let revealed = crate::wgpu::select::select_popup_geometry(trigger, 7, &theme, tree.node(root).unwrap().layout.height, state.scroll_offset.1, 0.0);
    let row = crate::wgpu::select::select_popup_row_hit_rect(trigger, 6, revealed, &theme);
    assert!(row.h > 0.0, "the highlighted row enters the accepted viewport");
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

    // pointer moved far outside `a`'s bounds and into `b`'s — capture must still target `a`.
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
    button.presence.state = crate::wgpu::component::ui::UiState::Disabled;
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

//#region 🔖️EditStateTests
#[test]
fn focusing_an_input_seeds_edit_state_from_its_value_and_blur_clears_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let input = leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(tree.node(input).unwrap().state.edit, Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 }));

    // clicking empty space blurs.
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(tree.node(input).unwrap().state.edit, None, "blur must relinquish the buffer so the declarative value governs again");
}

#[test]
fn arrow_keys_move_the_caret_and_backspace_deletes_the_previous_char() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
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
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
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
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
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
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
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

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0, modifiers: Default::default() });
    assert!(tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
    assert!(tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED), "an ancestor Stack row must observe hover too, for paint's reveal-on-hover");
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 500.0, y: 500.0, modifiers: Default::default() });
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
    let section = UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

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
    let section = UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    // `paint::sync_tree_row_layout` is what would normally flip this on (mirroring `item.draggable`)
    // — these tests build the retained tree by hand (no `paint_tree` call), so it's set directly.
    tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    let mut router = EventRouter::new("main");
    router.set_tree_drag_policy(&mut tree, UiDriverDrag::Surface, TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default()));

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
    let section = UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    let mut router = EventRouter::new("main");
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
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
    let section = UiTreeSectionNode { window: None, id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
    let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
    tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
    let mut router = EventRouter::new("main");
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
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

//#region 🔖️AnchoredPlacementTests
// 📍️ W1n: `resolve_anchored_placement` is a field-for-field port of React's `resolvePopoverPlacement`
// (`🧱️elements/🗨️Popover/🟦️.tsx:215-258`). These pin the four behaviours that positioner actually has:
// side placement, cross-axis alignment, main-axis collision flip, and the viewport clamp.

fn viewport() -> (f32, f32) {
    (400.0, 300.0)
}

#[test]
fn an_anchored_overlay_sits_on_its_requested_side_with_the_declared_offset() {
    let anchor = Rect::new(100.0, 100.0, 40.0, 20.0);
    let below = resolve_anchored_placement(anchor, (60.0, 30.0), viewport(), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(below.side, OverlaySide::Bottom);
    assert_eq!(below.y, 120.0 + 4.0, "sideOffset: 4");
    assert_eq!(below.x, 100.0 + (40.0 - 60.0) / 2.0, "align: center");

    let above = resolve_anchored_placement(anchor, (60.0, 30.0), viewport(), AnchoredPlacement::TOOLTIP, FlowInline::Ltr);
    assert_eq!(above.side, OverlaySide::Top);
    assert_eq!(above.y, 100.0 - 30.0 - 8.0, "the tooltip's own sideOffset: 8");
}

#[test]
fn an_anchored_overlay_flips_to_the_opposite_side_when_the_main_axis_overflows() {
    let low = Rect::new(100.0, 250.0, 40.0, 20.0);
    let flipped = resolve_anchored_placement(low, (60.0, 60.0), viewport(), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(flipped.side, OverlaySide::Top, "no room below inside the collision padding, so it flips above");
    assert_eq!(flipped.y, 250.0 - 60.0 - 4.0);

    let high = Rect::new(100.0, 4.0, 40.0, 20.0);
    let unflipped = resolve_anchored_placement(high, (60.0, 30.0), viewport(), AnchoredPlacement::TOOLTIP, FlowInline::Ltr);
    assert_eq!(unflipped.side, OverlaySide::Bottom, "a tooltip with no room above flips below");
}

#[test]
fn an_anchored_overlay_never_flips_into_a_side_that_also_overflows() {
    let squeezed = Rect::new(100.0, 140.0, 40.0, 20.0);
    let resolved = resolve_anchored_placement(squeezed, (60.0, 280.0), viewport(), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(resolved.side, OverlaySide::Bottom, "React keeps the requested side when the flip would overflow too");
}

#[test]
fn an_anchored_overlay_is_clamped_inside_the_collision_padding() {
    let edge = Rect::new(390.0, 10.0, 8.0, 8.0);
    let resolved = resolve_anchored_placement(edge, (100.0, 40.0), viewport(), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(resolved.x, 400.0 - 8.0 - 100.0, "clamped to viewport width minus collisionPadding minus content width");
    assert!(resolved.y >= 8.0);
}

#[test]
fn a_centered_overlay_ignores_its_anchor() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 400.0, 300.0));
    let (x, y) = resolve_overlay_placement(&tree, OverlayAnchor::Node(root), (100.0, 80.0), viewport(), OverlayKind::Dialog.default_placement(), FlowInline::Ltr);
    assert_eq!((x, y), (150.0, 110.0));
    assert!(OverlayKind::Dialog.has_backdrop(), "a modal draws React's scrim; a popover does not");
    assert!(!OverlayKind::Popover.has_backdrop());
}
//#endregion 🔖️AnchoredPlacementTests

//#region 🔖️TooltipDwellTests
// 💡️ W1n: React arms a 400 ms `setTimeout` on pointer-enter (`🧱️elements/💡️ChromeControlHint/🟦️.tsx:21,51`).
// `advance_clock` is this target's equivalent, and it also debounces the hover-out delay that
// `DismissPolicy` had only ever *documented*.

#[test]
fn a_hover_reveals_a_tooltip_only_after_the_react_dwell_elapses() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let control = leaf(&mut tree, Some(root), 1, button_ui("save"), (10.0, 10.0, 60.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 20.0, y: 15.0, modifiers: Default::default() });
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_DWELL_SECONDS - 0.01).0, TooltipStep::Idle, "nothing reveals before the dwell");
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_DWELL_SECONDS).0, TooltipStep::Reveal(control));
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_DWELL_SECONDS + 1.0).0, TooltipStep::Idle, "one reveal per hover, not one per frame");
}

#[test]
fn moving_to_another_control_restarts_the_dwell() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let first = leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
    let second = leaf(&mut tree, Some(root), 2, button_ui("b"), (60.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0, modifiers: Default::default() });
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_DWELL_SECONDS).0, TooltipStep::Reveal(first));
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 70.0, y: 10.0, modifiers: Default::default() });
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_DWELL_SECONDS + 0.1).0, TooltipStep::Idle, "the second control's own dwell restarts from the move");
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_DWELL_SECONDS * 2.0).0, TooltipStep::Reveal(second));
}

#[test]
fn an_open_tooltip_dismisses_only_after_the_hover_out_delay() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let control = leaf(&mut tree, Some(root), 1, button_ui("save"), (0.0, 0.0, 50.0, 20.0));
    let tip = leaf(&mut tree, Some(root), 2, text_ui("Save"), (0.0, 30.0, 50.0, 16.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, tip, OverlayKind::Tooltip, OverlayAnchor::Node(control));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 150.0, y: 150.0, modifiers: Default::default() });
    assert!(router.topmost_overlay().is_some(), "hover-out arms the countdown instead of closing immediately");
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_HOVER_OUT_SECONDS - 0.05).0, TooltipStep::Idle);
    let (step, commands) = router.advance_clock(&mut tree, TOOLTIP_HOVER_OUT_SECONDS);
    assert_eq!(step, TooltipStep::Dismissed);
    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { kind: OverlayKind::Tooltip, .. })));
    assert!(router.topmost_overlay().is_none());
}

#[test]
fn returning_to_the_anchor_disarms_the_hover_out_countdown() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let control = leaf(&mut tree, Some(root), 1, button_ui("save"), (0.0, 0.0, 50.0, 20.0));
    let tip = leaf(&mut tree, Some(root), 2, text_ui("Save"), (0.0, 30.0, 50.0, 16.0));
    let mut router = EventRouter::new("main");
    router.open_overlay(&mut tree, tip, OverlayKind::Tooltip, OverlayAnchor::Node(control));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 150.0, y: 150.0, modifiers: Default::default() });
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0, modifiers: Default::default() });
    assert_eq!(router.advance_clock(&mut tree, TOOLTIP_HOVER_OUT_SECONDS * 4.0).0, TooltipStep::Idle);
    assert!(router.topmost_overlay().is_some(), "the pointer came back, so the tooltip stays open");
}
//#endregion 🔖️TooltipDwellTests

//#region ⌨️WidgetKeyboardTests
// ⌨️ Keyboard parity for the controls a plain `<button>`'s Enter/Space does not cover — ticket
// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o. The per-key decision tables themselves are unit
// tested with the element (`🧱️elements/🔽️Select/🧪️tests/🔬️wgpu-select-keyboard`); these assert the
// ROUTING: that a focused control claims the key, mutates the retained state, and emits (or
// withholds) a command.

fn slider_ui(id: &str, value: f64) -> UiNode {
    UiNode::Slider(UiSliderNode { id: id.into(), value, min: 0.0, max: 10.0, step: 1.0, unit: None, on_change: action(), presence: UiPresence::default(), menu: None })
}

fn toggle_ui(id: &str, on: bool) -> UiNode {
    let mut presence = UiPresence::default();
    presence.selected = on;
    UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: id.into(), icon_id: IconName::CircleDot, text: None, on_change: action(), presence, menu: None })
}

fn key(key: &str) -> UiEvent {
    UiEvent::KeyDown { key: key.into(), modifiers: EventModifiers::default() }
}

/// ⬇️ `ArrowDown` on a closed, focused `Select` opens the popup highlighting the SELECTED row, the
/// way React's `SelectTrigger` opens with intent `"selected"`.
#[test]
fn arrow_down_opens_a_focused_select_on_its_selected_row() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "b"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("ArrowDown"));

    assert!(tree.node(select).unwrap().state.open, "ArrowDown opens the popup");
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "the highlight starts on the selected row");
}

/// ⬆️ `ArrowUp` opens on the LAST row; arrowing then wraps, and `Escape` clears both bits.
#[test]
fn arrow_up_opens_on_the_last_row_and_arrowing_wraps() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("ArrowUp"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "ArrowUp opens on the last row");

    router.dispatch(&mut tree, root, &key("ArrowDown"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(0), "the highlight wraps past the last row");

    router.dispatch(&mut tree, root, &key("Escape"));
    assert!(!tree.node(select).unwrap().state.open);
    assert_eq!(tree.node(select).unwrap().state.highlighted, None, "a dismissed popup keeps no stale highlight");
}

/// ⏎️ `Enter` over the open popup commits the highlighted row's value and closes it.
#[test]
fn enter_commits_the_highlighted_row_and_closes_the_popup() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);
    router.dispatch(&mut tree, root, &key("ArrowDown"));
    router.dispatch(&mut tree, root, &key("ArrowDown"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1));

    let commands = router.dispatch(&mut tree, root, &key("Enter"));

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { .. })), "committing a row dispatches the select's own change action");
    assert!(!tree.node(select).unwrap().state.open, "committing closes the popup");
}

/// 🔤️ A printable key opens the popup on the first matching row, and a second key extends the same
/// query rather than restarting it.
#[test]
fn typing_jumps_to_the_matching_row_and_extends_the_query() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("b"));
    assert!(tree.node(select).unwrap().state.open, "typing opens a closed select");
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "the query lands on the row whose label starts with it");

    router.dispatch(&mut tree, root, &key("a"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "\"ba\" matches nothing, so the highlight holds instead of jumping to \"A\"");
}

/// ↹️ `Tab` closes the popup AND still moves focus — React's `SelectContent` closes without
/// restoring focus so the browser's own tab move lands.
#[test]
fn tab_closes_an_open_select_and_still_moves_focus() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let next = leaf(&mut tree, Some(root), 2, button_ui("after"), (0.0, 40.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);
    router.dispatch(&mut tree, root, &key("ArrowDown"));

    router.dispatch(&mut tree, root, &key("Tab"));

    assert!(!tree.node(select).unwrap().state.open, "Tab dismisses the popup");
    assert_eq!(router.focus.focused, Some(next), "and the focus move still happens");
}

/// 🚫️ A disabled `Select` ignores every key, exactly as a `disabled` trigger does in the DOM.
#[test]
fn a_disabled_select_ignores_keyboard_input() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let mut disabled = select_ui("sel", "a");
    if let UiNode::Select(node) = &mut disabled {
        node.presence.state = UiState::Disabled;
    }
    let select = leaf(&mut tree, Some(root), 1, disabled, (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("ArrowDown"));

    assert!(!tree.node(select).unwrap().state.open);
}

/// 🎚️ A focused `Slider` steps on the arrow keys, jumps to its ends on `Home`/`End`, moves ten
/// steps on `PageUp`/`PageDown`, and clamps — React's own `Slider` key handler.
#[test]
fn a_focused_slider_steps_jumps_and_clamps_on_the_arrow_keys() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let slider = leaf(&mut tree, Some(root), 1, slider_ui("s", 5.0), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(slider), true);

    for pressed in ["ArrowRight", "ArrowUp", "ArrowLeft", "ArrowDown", "Home", "End", "PageUp", "PageDown"] {
        let commands = router.dispatch(&mut tree, root, &key(pressed));
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { .. })), "{pressed} must commit a slider value");
    }
    let ignored = router.dispatch(&mut tree, root, &key("F5"));
    assert!(ignored.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "an unrelated key must not move the slider");
}

/// 🎚️ The values themselves: one step per arrow, ten per page key, clamped to the track's ends.
#[test]
fn slider_key_values_match_reacts_step_multiplier_and_clamp() {
    let UiNode::Slider(slider) = slider_ui("s", 5.0) else { panic!("slider") };
    assert_eq!(slider_key_value(&slider, "ArrowRight", false), Some(6.0));
    assert_eq!(slider_key_value(&slider, "ArrowLeft", false), Some(4.0));
    assert_eq!(slider_key_value(&slider, "ArrowUp", false), Some(6.0));
    assert_eq!(slider_key_value(&slider, "ArrowDown", false), Some(4.0));
    assert_eq!(slider_key_value(&slider, "Home", false), Some(0.0));
    assert_eq!(slider_key_value(&slider, "End", false), Some(10.0));
    assert_eq!(slider_key_value(&slider, "PageUp", false), Some(10.0), "ten steps from 5 clamps at the max");
    assert_eq!(slider_key_value(&slider, "PageDown", false), Some(0.0));
    assert_eq!(slider_key_value(&slider, "ArrowRight", true), Some(10.0), "a Shift chord moves ten steps too");
    assert_eq!(slider_key_value(&slider, "Enter", false), None, "a key the slider does not own is never swallowed");
}

/// 🔀️ A focused `Toggle` flips on `Enter`/`Space` — React renders it as a `<button>`, so both keys
/// click it; wgpu used to answer neither.
#[test]
fn a_focused_toggle_flips_on_enter_and_space() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let toggle = leaf(&mut tree, Some(root), 1, toggle_ui("t", false), (0.0, 0.0, 40.0, 20.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(toggle), true);

    for pressed in ["Enter", " "] {
        let commands = router.dispatch(&mut tree, root, &key(pressed));
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { .. })), "{pressed} must fire the toggle's change action");
    }
}
//#endregion ⌨️WidgetKeyboardTests

//#region 🧭️FlowDirectionTests
// 🧭️ W2k: this target had ZERO flow-direction support — `events`' own positioner docstring said so.
// React takes `rtl` from `useFlow().inline` (`🧱️elements/🗨️Popover/🟦️.tsx:311, :326`) and inverts the
// horizontal arrow pair per control (`🎚️Slider/🟦️.tsx:383-384`, `📑️Tabs/🟦️.tsx:164-165`).

#[semio_framework_async_macros::async_test]
async fn a_menu_mirrors_its_inline_edge_once_the_window_carries_an_rtl_flow() {
    let anchor = Rect::new(100.0, 40.0, 60.0, 20.0);
    let ltr = resolve_anchored_placement(anchor, (30.0, 40.0), viewport(), AnchoredPlacement::MENU, FlowInline::Ltr);
    let rtl = resolve_anchored_placement(anchor, (30.0, 40.0), viewport(), AnchoredPlacement::MENU, FlowInline::Rtl);
    assert_eq!(ltr.x, 100.0);
    assert_eq!(rtl.x, 130.0);
    assert_eq!(ltr.y, rtl.y, "only the inline axis mirrors");
    assert_eq!(ltr.side, rtl.side, "React never mirrors `side`, only `align`");
}

#[semio_framework_async_macros::async_test]
async fn a_routers_flow_defaults_to_ltr_down_and_only_changes_once() {
    let mut router = EventRouter::new("w");
    assert_eq!(router.flow(), UiFlow::DEFAULT);
    assert_eq!(router.flow_inline(), FlowInline::Ltr);
    let mirrored = UiFlow { inline: FlowInline::Rtl, block: ui_contract::FlowBlock::Up };
    assert!(router.set_flow(mirrored), "a real change reports true so the caller can invalidate layout");
    assert!(!router.set_flow(mirrored), "re-setting the same flow is a no-op");
    assert_eq!(router.flow_inline(), FlowInline::Rtl);
}

#[test]
fn an_up_flow_tree_section_routes_its_bottom_header_through_the_event_router() {
    let section = UiTreeSectionNode { window: None, id: "drivers".into(), label: Some(Label::data("Drivers")), default_open: Some(true), presence: UiPresence::default(), items: Vec::new() };
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, tree_ui(vec![section]), (0.0, 0.0, 200.0, 100.0));
    let disclosure = insert_tree_row(&mut tree, root, "drivers", (0.0, 0.0, 200.0, 100.0));
    let mut router = EventRouter::new("w");
    router.set_flow(UiFlow { inline: FlowInline::Ltr, block: ui_contract::FlowBlock::Up });

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 100.0, y: 88.0, button: PointerButton::Primary, modifiers: EventModifiers::default() });
    assert_eq!(router.capture.target.map(|(_, id, _)| id), Some(disclosure), "the painted bottom header owns the press");
    router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 100.0, y: 88.0, button: PointerButton::Primary, modifiers: EventModifiers::default() });
    assert_eq!(tree.disclosure_open(disclosure), Some(false), "release over the same bottom band closes the disclosure");
}

#[semio_framework_async_macros::async_test]
async fn an_rtl_window_swaps_the_horizontal_arrow_pair_and_leaves_every_other_key_alone() {
    let mut router = EventRouter::new("w");
    assert_eq!(router.mirrored_inline_key("ArrowLeft"), "ArrowLeft");
    router.set_flow(UiFlow { inline: FlowInline::Rtl, block: ui_contract::FlowBlock::Down });
    assert_eq!(router.mirrored_inline_key("ArrowLeft"), "ArrowRight");
    assert_eq!(router.mirrored_inline_key("ArrowRight"), "ArrowLeft");
    for invariant in ["ArrowUp", "ArrowDown", "Home", "End", "PageUp", "PageDown", "Enter", " "] {
        assert_eq!(router.mirrored_inline_key(invariant), invariant, "{invariant} carries no flow direction");
    }
}

#[semio_framework_async_macros::async_test]
async fn an_rtl_slider_moves_the_other_way_for_the_same_arrow_key() {
    let slider = UiSliderNode { id: "s".into(), value: 5.0, min: 0.0, max: 10.0, step: 1.0, unit: None, on_change: action(), presence: UiPresence::default(), menu: None };
    let mut router = EventRouter::new("w");
    let ltr_right = slider_key_value(&slider, router.mirrored_inline_key("ArrowRight"), false);
    router.set_flow(UiFlow { inline: FlowInline::Rtl, block: ui_contract::FlowBlock::Down });
    let rtl_right = slider_key_value(&slider, router.mirrored_inline_key("ArrowRight"), false);
    assert_eq!(ltr_right, Some(6.0));
    assert_eq!(rtl_right, Some(4.0), "React's `positiveHorizontal` is ArrowLeft under rtl (🎚️Slider/🟦️.tsx:383)");
}
//#endregion 🧭️FlowDirectionTests

//#region ⌨️FocusVisibleTests
// ⌨️ W2k: `focus-visible:border-accent` is a `:focus-visible` selector, so a CLICKED control must not
// paint the accent ring. `EventRouter` carries the modality latch and stamps `NodeFlags::FOCUS_VISIBLE`.

#[semio_framework_async_macros::async_test]
async fn a_pointer_press_focuses_without_the_keyboard_ring_and_a_tab_move_restores_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let pressed = leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
    leaf(&mut tree, Some(root), 2, button_ui("b"), (0.0, 20.0, 50.0, 20.0));
    let mut router = EventRouter::new("w");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert!(!router.focus_visible(), "a press clears the modality latch");
    let flags = tree.node(pressed).expect("focused node is live").flags;
    assert!(flags.contains(NodeFlags::FOCUSED), "the press still focuses the control");
    assert!(!flags.contains(NodeFlags::FOCUS_VISIBLE), "a pointer-focused control paints no focus ring");

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() });
    assert!(router.focus_visible(), "a key sets the modality latch");
    let tabbed = router.focus.focused.expect("Tab moved focus");
    let tabbed_flags = tree.node(tabbed).expect("tabbed node is live").flags;
    assert!(tabbed_flags.contains(NodeFlags::FOCUSED) && tabbed_flags.contains(NodeFlags::FOCUS_VISIBLE), "a keyboard-focused control paints the ring");
    assert!(!tree.node(pressed).expect("blurred node is live").flags.contains(NodeFlags::FOCUS_VISIBLE), "blurring clears the bit");
}
//#endregion ⌨️FocusVisibleTests

//#region ✍️SearchLineMomentTests

/// ✍️ One window-search line — React's `SearchInput` shape: `onChange` per keystroke plus the
/// `onSubmit`/`onAbort`/`onRepeatLast` trio, each a DISTINCT verb so a dispatched intent names which
/// moment fired it.
fn search_line_ui(id: &str, value: &str, repeat_last: bool) -> UiNode {
    let verb = |name: &str| ActionDescriptor { controller_id: "ctrl".into(), action: name.into(), args: None };
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
        on_change: verb("engagementInput"),
        on_submit: Some(verb("engagementSubmit")),
        on_abort: Some(verb("engagementAbort")),
        on_repeat_last: repeat_last.then(|| verb("engagementRepeatLast")),
        presence: UiPresence::default(),
        menu: None,
    })
}

fn fired_verbs(commands: &[UiCommand]) -> Vec<(String, String)> {
    commands.iter().filter_map(|cmd| if let UiCommand::App { intent, .. } = cmd { Some((format!("{:?}", intent.trigger), intent.action_name())) } else { None }).collect()
}

/// ✍️ **LAW: one retained line fires FOUR moments, not one.** React's window search binds
/// `onChange` on every keystroke, `onSubmit` on Enter (with the TRIMMED line), `onAbort` on Escape and
/// `onRepeatLast` on Space over an empty idle line — all on the SAME field
/// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `Search`, `onKeyDown`; `applySearchSpaceAction` at `:10512`).
///
/// 🩸️ This target produced only `Trigger::Change`/`Trigger::Commit` from an editable node, so a
/// command line could be EITHER a typing feed or a verb runner and never both: the os shell's search
/// pane resolved `on_submit` into the single `on_change` slot, committed it on blur, and the program
/// never saw a keystroke (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w13b-actions-search-pane-bodies.md` §6 gap 4).
#[test]
fn a_search_line_fires_change_while_typing_and_submit_on_enter() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, search_line_ui("puzzle3d-engagement", "", false), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let typed = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "box ".into() });
    assert_eq!(fired_verbs(&typed), vec![("Change".to_string(), "engagementInput".to_string())], "✍️ every keystroke feeds the program's autocomplete through `onChange`");

    let entered = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Enter".into(), modifiers: EventModifiers::default() });
    assert_eq!(fired_verbs(&entered), vec![("Submit".to_string(), "engagementSubmit".to_string())], "⏎️ Enter confirms through `onSubmit`, and never re-fires the change");
    let submitted = entered.iter().find_map(|cmd| if let UiCommand::App { intent, .. } = cmd { intent.payload() } else { None }).expect("⏎️ the submit carries the line");
    assert_eq!(submitted, DslValue::Object(vec![("value".into(), DslValue::String("box".into()))]), "⏎️ React submits `draft.trim()`");
}

/// ⎋️🔁️ **LAW: Escape aborts and Space repeats — and only where React routes them.** Escape
/// dispatches `onAbort` when no overlay is open (React's `Search` closes its possibles popover first);
/// Space over an EMPTY line dispatches `onRepeatLast`, falls back to the submit the program did bind,
/// and types an ordinary space once the line carries text.
#[test]
fn a_search_line_aborts_on_escape_and_repeats_last_on_space_over_an_empty_line() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let line = leaf(&mut tree, Some(root), 1, search_line_ui("puzzle3d-engagement", "", true), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let spaced = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: " ".into() });
    assert_eq!(fired_verbs(&spaced), vec![("RepeatLast".to_string(), "engagementRepeatLast".to_string())], "🔁️ Space over an empty idle line restarts the last engagement");
    assert_eq!(tree.node(line).unwrap().state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_default(), "", "🔁️ and it never types the space it consumed");

    router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "box".into() });
    let spaced = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: " ".into() });
    assert!(fired_verbs(&spaced).iter().all(|(_, verb)| verb != "engagementRepeatLast"), "🔁️ a line that carries text types its space: {:?}", fired_verbs(&spaced));
    assert_eq!(tree.node(line).unwrap().state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_default(), "box ", "🔁️ and the space lands in the buffer");

    let escaped = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert_eq!(fired_verbs(&escaped), vec![("Abort".to_string(), "engagementAbort".to_string())], "⎋️ Escape cancels the session through `onAbort`");

    // ⎋️ An OPEN overlay swallows the same key, exactly as React's popover does before the abort.
    let popup = leaf(&mut tree, Some(root), 2, stack_ui(), (10.0, 10.0, 50.0, 50.0));
    router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));
    let escaped = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert!(fired_verbs(&escaped).is_empty(), "⎋️ the first Escape only closes the popover: {:?}", fired_verbs(&escaped));
}

/// ✍️ **LAW: a line that binds none of the three keeps its old behaviour.** Every other retained
/// input in the fleet binds `on_change` alone, so Escape must stay an overlay key and Space must stay
/// a character — the producers above are opt-in per node, never a new global key grammar.
#[test]
fn an_ordinary_input_keeps_escape_and_space_to_itself() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let input = leaf(&mut tree, Some(root), 1, input_ui("name", ""), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });

    let spaced = router.dispatch(&mut tree, root, &UiEvent::TextInput { text: " ".into() });
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().map(|edit| edit.text.clone()).unwrap_or_default(), " ", "✍️ an ordinary field types its space");
    assert!(fired_verbs(&spaced).iter().all(|(trigger, _)| trigger == "Change"), "✍️ and reports it as an ordinary change: {:?}", fired_verbs(&spaced));

    let escaped = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });
    assert!(fired_verbs(&escaped).is_empty(), "✍️ and dispatches nothing on Escape: {:?}", fired_verbs(&escaped));
}
//#endregion ✍️SearchLineMomentTests
