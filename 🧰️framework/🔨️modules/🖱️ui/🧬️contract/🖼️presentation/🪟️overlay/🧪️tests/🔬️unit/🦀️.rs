use super::*;
use crate::{Anchor, FlowBlock, UiFlow};

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn anchor() -> OverlayRect {
    OverlayRect::new(100.0, 200.0, 60.0, 20.0)
}

//#region 🔖️AnchoredPlacementTests

#[test]
fn an_anchored_overlay_sits_on_its_requested_side_with_the_declared_offset() {
    let resolved = resolve_anchored_placement(anchor(), (80.0, 40.0), (1000.0, 1000.0), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(resolved.side, OverlaySide::Bottom);
    assert_eq!(resolved.y, 224.0);
    assert_eq!(resolved.x, 90.0);
    let tooltip = resolve_anchored_placement(anchor(), (80.0, 40.0), (1000.0, 1000.0), AnchoredPlacement::TOOLTIP, FlowInline::Ltr);
    assert_eq!(tooltip.side, OverlaySide::Top);
    assert_eq!(tooltip.y, 152.0);
}

#[test]
fn an_anchored_overlay_flips_to_the_opposite_side_when_the_main_axis_overflows() {
    let below = resolve_anchored_placement(OverlayRect::new(0.0, 180.0, 40.0, 20.0), (40.0, 100.0), (400.0, 240.0), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(below.side, OverlaySide::Top);
    let boxed_in = resolve_anchored_placement(OverlayRect::new(0.0, 60.0, 40.0, 20.0), (40.0, 100.0), (400.0, 130.0), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(boxed_in.side, OverlaySide::Bottom, "a flip that also overflows is not taken — React keeps the requested side");
}

#[test]
fn a_menu_aligns_to_the_anchors_inline_start_and_mirrors_under_rtl() {
    let ltr = resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), AnchoredPlacement::MENU, FlowInline::Ltr);
    assert_eq!(ltr.x, 100.0, "ltr Start is the anchor's LEFT edge");
    let rtl = resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), AnchoredPlacement::MENU, FlowInline::Rtl);
    assert_eq!(rtl.x, 130.0, "rtl Start is the anchor's RIGHT edge minus the content width");
}

#[test]
fn an_end_aligned_overlay_mirrors_the_other_way_and_center_never_mirrors() {
    let end = AnchoredPlacement { align: OverlayAlign::End, ..AnchoredPlacement::MENU };
    assert_eq!(resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), end, FlowInline::Ltr).x, 130.0);
    assert_eq!(resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), end, FlowInline::Rtl).x, 100.0);
    let center = AnchoredPlacement { align: OverlayAlign::Center, ..AnchoredPlacement::MENU };
    assert_eq!(resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), center, FlowInline::Ltr).x, resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), center, FlowInline::Rtl).x);
}

#[test]
fn a_vertical_cross_axis_carries_no_flow_direction() {
    let side_placed = AnchoredPlacement { side: OverlaySide::Right, align: OverlayAlign::Start, ..AnchoredPlacement::POPOVER };
    let ltr = resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), side_placed, FlowInline::Ltr);
    let rtl = resolve_anchored_placement(anchor(), (30.0, 40.0), (1000.0, 1000.0), side_placed, FlowInline::Rtl);
    assert_eq!(ltr, rtl, "React's alignedTop has no rtl branch, so a Left/Right side is direction-invariant");
    assert_eq!(ltr.side, OverlaySide::Right);
}

#[test]
fn an_anchored_overlay_is_clamped_inside_the_collision_padding() {
    let resolved = resolve_anchored_placement(OverlayRect::new(-40.0, 10.0, 20.0, 20.0), (60.0, 30.0), (200.0, 200.0), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(resolved.x, 8.0);
    let far = resolve_anchored_placement(OverlayRect::new(400.0, 10.0, 20.0, 20.0), (60.0, 30.0), (200.0, 200.0), AnchoredPlacement::POPOVER, FlowInline::Ltr);
    assert_eq!(far.x, 132.0);
}

#[test]
fn a_placement_with_collisions_off_is_left_exactly_where_it_was_asked_for() {
    let loose = AnchoredPlacement { avoid_collisions: false, ..AnchoredPlacement::POPOVER };
    let resolved = resolve_anchored_placement(OverlayRect::new(-40.0, 190.0, 20.0, 20.0), (60.0, 30.0), (200.0, 200.0), loose, FlowInline::Ltr);
    assert_eq!((resolved.side, resolved.x, resolved.y), (OverlaySide::Bottom, -60.0, 214.0));
}

#[test]
fn a_centered_placement_ignores_the_anchor_and_the_flow() {
    let centered = resolve_overlay_placement(anchor(), (100.0, 50.0), (400.0, 300.0), OverlayPlacement::Centered, FlowInline::Rtl);
    assert_eq!((centered.x, centered.y), (150.0, 125.0));
    assert_eq!(centered, resolve_centered_placement((100.0, 50.0), (400.0, 300.0)));
}

#[test]
fn every_side_answers_reacts_own_transform_origin_and_flip_target() {
    assert_eq!(OverlaySide::Top.transform_origin(), "center bottom");
    assert_eq!(OverlaySide::Bottom.transform_origin(), "center top");
    assert_eq!(OverlaySide::Left.transform_origin(), "right center");
    assert_eq!(OverlaySide::Right.transform_origin(), "left center");
    assert_eq!(OverlaySide::Top.opposite(), OverlaySide::Bottom);
    assert_eq!(OverlaySide::Left.opposite(), OverlaySide::Right);
}

//#endregion 🔖️AnchoredPlacementTests

//#region 🔖️SelectPlacementTests

#[test]
fn a_select_popup_resolves_its_own_inline_edge_and_mirrors_both_ends() {
    let trigger = OverlayRect::new(100.0, 0.0, 60.0, 20.0);
    assert_eq!(resolve_select_inline_left(trigger, 30.0, OverlayAlign::Start, FlowInline::Ltr), 100.0);
    assert_eq!(resolve_select_inline_left(trigger, 30.0, OverlayAlign::End, FlowInline::Ltr), 130.0);
    assert_eq!(resolve_select_inline_left(trigger, 30.0, OverlayAlign::Start, FlowInline::Rtl), 130.0);
    assert_eq!(resolve_select_inline_left(trigger, 30.0, OverlayAlign::End, FlowInline::Rtl), 100.0);
    assert_eq!(resolve_select_inline_left(trigger, 30.0, OverlayAlign::Center, FlowInline::Ltr), 115.0);
    assert_eq!(resolve_select_inline_left(trigger, 30.0, OverlayAlign::Center, FlowInline::Rtl), 115.0);
}

//#endregion 🔖️SelectPlacementTests

//#region 🧭️FlowTests

#[test]
fn the_default_flow_is_reacts_own_ltr_down() {
    assert_eq!(UiFlow::default(), UiFlow::DEFAULT);
    assert_eq!(UiFlow::DEFAULT, UiFlow { inline: FlowInline::Ltr, block: FlowBlock::Down });
    assert!(!UiFlow::DEFAULT.inline.is_rtl());
    assert!(!UiFlow::DEFAULT.block.is_reversed());
}

#[test]
fn a_flow_override_inherits_the_axis_it_does_not_name() {
    let parent = UiFlow { inline: FlowInline::Rtl, block: FlowBlock::Up };
    assert_eq!(parent.merged(None, None), parent);
    assert_eq!(parent.merged(Some(FlowInline::Ltr), None), UiFlow { inline: FlowInline::Ltr, block: FlowBlock::Up });
    assert_eq!(parent.merged(None, Some(FlowBlock::Down)), UiFlow { inline: FlowInline::Rtl, block: FlowBlock::Down });
}

#[test]
fn an_anchor_mirrors_exactly_the_axes_reacts_flow_from_anchor_mirrors() {
    assert_eq!(UiFlow::for_anchor(Anchor::TopStart), UiFlow { inline: FlowInline::Ltr, block: FlowBlock::Down });
    assert_eq!(UiFlow::for_anchor(Anchor::TopEnd), UiFlow { inline: FlowInline::Rtl, block: FlowBlock::Down });
    assert_eq!(UiFlow::for_anchor(Anchor::BottomStart), UiFlow { inline: FlowInline::Ltr, block: FlowBlock::Up });
    assert_eq!(UiFlow::for_anchor(Anchor::BottomEnd), UiFlow { inline: FlowInline::Rtl, block: FlowBlock::Up });
    assert_eq!(UiFlow::for_anchor(Anchor::Top), UiFlow::DEFAULT, "a middle column never mirrors inline");
    assert_eq!(UiFlow::for_anchor(Anchor::Start), UiFlow::DEFAULT, "a middle row never mirrors block");
    assert_eq!(UiFlow::for_anchor(Anchor::End).inline, FlowInline::Rtl);
    assert_eq!(UiFlow::for_anchor(Anchor::Bottom).block, FlowBlock::Up);
    assert_eq!(UiFlow::for_anchor(Anchor::Center), UiFlow::DEFAULT);
}

#[test]
fn an_inline_delta_carries_the_mirrored_sign() {
    assert_eq!(FlowInline::Ltr.inline_sign(), 1.0);
    assert_eq!(FlowInline::Rtl.inline_sign(), -1.0);
}

#[test]
fn the_flow_axes_round_trip_on_the_wire_as_reacts_own_lowercase_tags() {
    assert_eq!(serde_json::to_string(&FlowInline::Rtl).expect("serialize"), "\"rtl\"");
    assert_eq!(serde_json::to_string(&FlowBlock::Up).expect("serialize"), "\"up\"");
    assert_eq!(serde_json::to_string(&UiFlow::DEFAULT).expect("serialize"), "{\"inline\":\"ltr\",\"block\":\"down\"}");
    let back: UiFlow = serde_json::from_str("{\"inline\":\"rtl\",\"block\":\"up\"}").expect("deserialize");
    assert_eq!(back, UiFlow { inline: FlowInline::Rtl, block: FlowBlock::Up });
}

//#endregion 🧭️FlowTests
