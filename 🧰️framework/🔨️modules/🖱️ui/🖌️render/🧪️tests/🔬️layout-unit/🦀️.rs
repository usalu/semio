
use super::*;
use ui_contract::LeafLayout;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn shared<'a>(arena: &'a mut crate::element::FrameArena, resources: &'a mut crate::resource::ResourceRegistry, retained: &'a mut crate::element::RetainedStore) -> crate::element::SharedFrameCx<'a> {
    crate::element::SharedFrameCx { arena, resources, retained, time_seconds: 0.0 }
}

#[test]
fn a_fixed_size_leaf_resolves_to_exactly_that_size() {
    let mut arena = crate::element::FrameArena::default();
    let mut resources = crate::resource::ResourceRegistry::default();
    let mut retained = crate::element::RetainedStore::default();
    let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

    let spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Fixed(SpaceToken::Lg), height: Sizing::Fixed(SpaceToken::Md) });
    let leaf = cx.leaf(&spec, None);
    cx.compute(leaf, 200.0, 200.0);

    let rect = cx.resolved(leaf);
    assert_eq!(rect.w, space_token_px(SpaceToken::Lg));
    assert_eq!(rect.h, space_token_px(SpaceToken::Md));
}

#[test]
fn a_vertical_stack_places_its_second_child_below_the_first() {
    let mut arena = crate::element::FrameArena::default();
    let mut resources = crate::resource::ResourceRegistry::default();
    let mut retained = crate::element::RetainedStore::default();
    let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

    let leaf_spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Fixed(SpaceToken::Lg), height: Sizing::Fixed(SpaceToken::Lg) });
    let first = cx.leaf(&leaf_spec, None);
    let second = cx.leaf(&leaf_spec, None);
    let stack_spec = LayoutSpec::Stack(StackLayout { axis: Axis::Vertical, gap: SpaceToken::None, padding: EdgeSpace::default(), align: Align::Start, justify: Justify::Start, grow: false, wrap: false });
    let root = cx.container(&stack_spec, &[first, second]);
    cx.compute(root, 400.0, 400.0);

    let first_rect = cx.resolved(first);
    let second_rect = cx.resolved(second);
    assert_eq!(first_rect.y, 0.0);
    assert!(second_rect.y >= first_rect.y + first_rect.h);
}

#[test]
fn a_pending_measurement_yields_the_placeholder_size_and_is_recorded() {
    let mut arena = crate::element::FrameArena::default();
    let mut resources = crate::resource::ResourceRegistry::default();
    let mut retained = crate::element::RetainedStore::default();
    let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

    let spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Hug, height: Sizing::Hug });
    let measure: MeasureFn = Box::new(|_w, _h| Measurement::Pending { placeholder_width: 42.0, placeholder_height: 17.0 });
    let leaf = cx.leaf(&spec, Some(measure));
    assert!(!cx.had_pending_measurement());
    cx.compute(leaf, 200.0, 200.0);

    let rect = cx.resolved(leaf);
    assert_eq!(rect.w, 42.0);
    assert_eq!(rect.h, 17.0);
    assert!(cx.had_pending_measurement());
}

#[test]
fn a_ready_measurement_does_not_mark_pending() {
    let mut arena = crate::element::FrameArena::default();
    let mut resources = crate::resource::ResourceRegistry::default();
    let mut retained = crate::element::RetainedStore::default();
    let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

    let spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Hug, height: Sizing::Hug });
    let measure: MeasureFn = Box::new(|_w, _h| Measurement::Ready { width: 10.0, height: 10.0 });
    let leaf = cx.leaf(&spec, Some(measure));
    cx.compute(leaf, 200.0, 200.0);

    assert!(!cx.had_pending_measurement());
}

#[test]
fn snap_logical_rounds_taffy_remainder_noise() {
    assert_eq!(snap_logical(33.333336), 33.0);
    assert_eq!(snap_logical(33.6), 34.0);
}
