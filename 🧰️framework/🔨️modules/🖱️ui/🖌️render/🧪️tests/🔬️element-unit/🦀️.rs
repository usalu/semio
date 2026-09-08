
use super::*;

#[test]
fn equal_parent_and_key_produce_the_same_element_id() {
    let parent = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));
    let a = ElementId::new(Some(parent), &ReconciliationKey::Explicit("child".into()));
    let b = ElementId::new(Some(parent), &ReconciliationKey::Explicit("child".into()));
    assert_eq!(a, b);
}

#[test]
fn explicit_key_survives_a_sibling_reorder() {
    let parent = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));
    let before_first = ElementId::new(Some(parent), &ReconciliationKey::Explicit("alpha".into()));
    let before_second = ElementId::new(Some(parent), &ReconciliationKey::Explicit("beta".into()));
    // Reordered: "beta" now comes first, "alpha" second — an explicit key's id does not depend on
    // position, unlike ReconciliationKey::Positional's ordinal.
    let after_first = ElementId::new(Some(parent), &ReconciliationKey::Explicit("beta".into()));
    let after_second = ElementId::new(Some(parent), &ReconciliationKey::Explicit("alpha".into()));
    assert_eq!(before_second, after_first);
    assert_eq!(before_first, after_second);
}

#[test]
fn different_keys_under_the_same_parent_produce_different_ids() {
    let parent = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));
    let a = ElementId::new(Some(parent), &ReconciliationKey::Explicit("a".into()));
    let b = ElementId::new(Some(parent), &ReconciliationKey::Explicit("b".into()));
    assert_ne!(a, b);
}

#[test]
fn same_key_under_different_parents_produces_different_ids() {
    let parent_a = ElementId::new(None, &ReconciliationKey::Explicit("a".into()));
    let parent_b = ElementId::new(None, &ReconciliationKey::Explicit("b".into()));
    let under_a = ElementId::new(Some(parent_a), &ReconciliationKey::Explicit("child".into()));
    let under_b = ElementId::new(Some(parent_b), &ReconciliationKey::Explicit("child".into()));
    assert_ne!(under_a, under_b);
}

#[test]
fn retained_state_survives_a_rebuild_and_is_released_when_the_id_disappears() {
    let mut store = RetainedStore::default();
    let surviving = ElementId::new(None, &ReconciliationKey::Explicit("surviving".into()));
    let disappearing = ElementId::new(None, &ReconciliationKey::Explicit("disappearing".into()));

    store.begin_frame();
    *store.get_or_insert_with(surviving, || 1i32) = 42;
    store.get_or_insert_with(disappearing, || 7i32);
    store.end_frame();
    assert_eq!(*store.get_or_insert_with(surviving, || 0i32), 42);

    // Next rebuild: only `surviving` is touched — `disappearing`'s element left the tree.
    store.begin_frame();
    assert_eq!(*store.get_or_insert_with(surviving, || 0i32), 42, "retained value must survive the rebuild untouched");
    store.end_frame();

    // A third frame with nothing touched proves `disappearing` was actually dropped, not merely
    // shadowed: re-inserting it now must run `init` again and see the fresh value, not 7.
    store.begin_frame();
    assert_eq!(*store.get_or_insert_with(disappearing, || 99i32), 99, "an untouched id's retained state must be released, not merely stale");
    store.end_frame();
}

#[test]
fn frame_arena_take_and_put_back_round_trip() {
    struct Probe;
    impl Element for Probe {
        type LayoutState = ();
        type PrepaintState = ();
        fn request_layout(&mut self, _id: ElementId, _cx: &mut crate::layout::LayoutCx<'_>) -> (crate::layout::LayoutNodeId, Self::LayoutState) {
            unimplemented!("not exercised by this test")
        }
        fn prepaint(&mut self, _id: ElementId, _bounds: Bounds, _layout: &mut Self::LayoutState, _cx: &mut PrepaintCx<'_>) -> Self::PrepaintState {}
        fn paint(&mut self, _id: ElementId, _bounds: Bounds, _layout: &mut Self::LayoutState, _prepaint: &mut Self::PrepaintState, _cx: &mut PaintCx<'_>) {}
    }

    let mut arena = FrameArena::default();
    let index = arena.alloc(Probe);
    let element = arena.take(index);
    arena.put_back(index, element);
    assert_eq!(arena.slots.len(), 1);
}

#[test]
#[should_panic(expected = "phase order violated")]
fn any_element_paint_before_request_layout_panics() {
    struct Probe;
    impl Element for Probe {
        type LayoutState = ();
        type PrepaintState = ();
        fn request_layout(&mut self, _id: ElementId, _cx: &mut crate::layout::LayoutCx<'_>) -> (crate::layout::LayoutNodeId, Self::LayoutState) {
            unimplemented!("not exercised by this test")
        }
        fn prepaint(&mut self, _id: ElementId, _bounds: Bounds, _layout: &mut Self::LayoutState, _cx: &mut PrepaintCx<'_>) -> Self::PrepaintState {}
        fn paint(&mut self, _id: ElementId, _bounds: Bounds, _layout: &mut Self::LayoutState, _prepaint: &mut Self::PrepaintState, _cx: &mut PaintCx<'_>) {}
    }

    // Uses `paint` (needs only `PaintCx`, which needs only the already-landed
    // `crate::scene::SceneBuilder`) rather than `prepaint` (which would need a
    // `crate::TextSystem` value that does not exist until packet `render-text` lands) so this
    // test can run against this packet alone.
    let mut any = AnyElement::new(Probe);
    let mut resources = crate::resource::ResourceRegistry::default();
    let mut retained = RetainedStore::default();
    let mut arena = FrameArena::default();
    let mut scene = crate::scene::SceneBuilder::default();
    let mut cx = PaintCx { shared: SharedFrameCx { arena: &mut arena, resources: &mut resources, retained: &mut retained, time_seconds: 0.0 }, scene: &mut scene };
    any.paint(ElementId::new(None, &ReconciliationKey::Explicit("x".into())), Bounds::default(), &mut cx);
}
