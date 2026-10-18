// 📍️ Anchored placement and tooltip dwell laws: the anchored overlay placement and the tooltip dwell, reveal and dismissal.

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
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_DWELL_SECONDS) - 0.01).tooltip, TooltipStep::Idle, "nothing reveals before the dwell");
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_DWELL_SECONDS)).tooltip, TooltipStep::Reveal(control));
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_DWELL_SECONDS) + 1.0).tooltip, TooltipStep::Idle, "one reveal per hover, not one per frame");
}

#[test]
fn moving_to_another_control_restarts_the_dwell() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let first = leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
    let second = leaf(&mut tree, Some(root), 2, button_ui("b"), (60.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0, modifiers: Default::default() });
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_DWELL_SECONDS)).tooltip, TooltipStep::Reveal(first));
    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 70.0, y: 10.0, modifiers: Default::default() });
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_DWELL_SECONDS) + 0.1).tooltip, TooltipStep::Idle, "the second control's own dwell restarts from the move");
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_DWELL_SECONDS) * 2.0).tooltip, TooltipStep::Reveal(second));
}

#[test]
fn suspending_a_hidden_surface_clears_every_hover_tooltip_clock() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let control = leaf(&mut tree, Some(root), 1, button_ui("save"), (10.0, 10.0, 60.0, 20.0));
    let mut router = EventRouter::new("main");
    router.hover_since = Some((control, 10.0));
    router.hover_revealed = Some(control);
    router.tooltip_dismiss_at = Some(11.0);

    assert!(router.suspend_clock(&mut tree));
    assert!(router.hover_since.is_none());
    assert!(router.hover_revealed.is_none());
    assert!(router.tooltip_dismiss_at.is_none());
    assert!(router.next_clock_deadline().is_none());
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
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_HOVER_OUT_SECONDS) - 0.05).tooltip, TooltipStep::Idle);
    let step = router.advance_clock(&mut tree, f64::from(TOOLTIP_HOVER_OUT_SECONDS));
    assert_eq!(step.tooltip, TooltipStep::Dismissed);
    assert!(step.commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { kind: OverlayKind::Tooltip, .. })));
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
    assert_eq!(router.advance_clock(&mut tree, f64::from(TOOLTIP_HOVER_OUT_SECONDS) * 4.0).tooltip, TooltipStep::Idle);
    assert!(router.topmost_overlay().is_some(), "the pointer came back, so the tooltip stays open");
}
//#endregion 🔖️TooltipDwellTests
