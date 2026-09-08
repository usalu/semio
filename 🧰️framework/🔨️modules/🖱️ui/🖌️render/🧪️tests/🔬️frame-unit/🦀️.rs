
use super::*;
use ui_contract::{ActionBinding, ActionId, EdgeSpace, LayoutSpec, LeafLayout, ScrollAxes, ScrollLayout, Sizing, SurfaceId, Trigger, UiNodeId, UiRevision as UiRev, UiText};

#[test]
fn frame_generation_next_is_strictly_increasing() {
    let zero = FrameGeneration::ZERO;
    let one = zero.next();
    let two = one.next();
    assert!(zero < one);
    assert!(one < two);
    assert!(zero < two);
}

#[test]
fn a_fresh_frame_engine_has_no_presented_snapshot() {
    let engine = FrameEngine::new();
    assert!(engine.presented().is_none());
}

#[test]
fn animation_frame_interval_is_positive_and_sub_frame_sized() {
    assert!(ANIMATION_FRAME_INTERVAL_SECONDS > 0.0);
    assert!(ANIMATION_FRAME_INTERVAL_SECONDS < 1.0);
}

//#region 🧪️TestElements

/// 🍃️ Registers itself as a leaf dispatch node using whatever absolute `bounds` its parent handed
/// it — this module's test elements never consult `LayoutCx::resolved` below the root (see
/// `Wrap`/`Pair`), so their hit-test geometry is deterministic test data, not emergent flex math.
struct TestLeaf {
    flags: crate::DispatchFlags,
    listeners: crate::ListenerSet,
}

impl TestLeaf {
    fn new() -> Self {
        Self { flags: crate::DispatchFlags::NONE, listeners: crate::ListenerSet::default() }
    }
}

impl Element for TestLeaf {
    type LayoutState = ();
    type PrepaintState = ();

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn request_layout(&mut self, _id: ElementId, cx: &mut LayoutCx<'_>) -> (crate::layout::LayoutNodeId, ()) {
        (cx.leaf(&LayoutSpec::Leaf(LeafLayout::default()), None), ())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn prepaint(&mut self, id: ElementId, bounds: Bounds, _layout: &mut (), cx: &mut PrepaintCx<'_>) {
        let hitbox = crate::Hitbox { element: id, bounds, clips_children: self.flags.contains(crate::DispatchFlags::CLIPS_CHILDREN), hit_transparent: self.flags.contains(crate::DispatchFlags::HIT_TRANSPARENT) };
        cx.register(id, self.flags, self.listeners.clone(), Some(hitbox));
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn paint(&mut self, _id: ElementId, _bounds: Bounds, _layout: &mut (), _prepaint: &mut (), _cx: &mut PaintCx<'_>) {}
}

/// 📦️ Every container `LayoutSpec` this test module hands taffy — `Sizing::Fill` so the outermost
/// call's `layout_cx.resolved(root_node)` is a deterministic 100%-of-viewport rect (a `const fn`
/// percent constructor, not this file's own guess at taffy's implicit auto-root sizing). Nothing
/// below the root ever reads a resolved rect back from taffy at all — see this region's docstring.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn test_container_spec() -> LayoutSpec {
    LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::None, padding: EdgeSpace::default(), sizing: Sizing::Fill })
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn offset_bounds(base: Bounds, offset: (f32, f32, f32, f32)) -> Bounds {
    Bounds::new(base.x + offset.0, base.y + offset.1, offset.2, offset.3)
}

/// 🌳️ Single-child test container: registers itself, then hands `child` an absolute rect computed
/// from its own incoming `bounds` plus a fixed `offset` — the take→recurse→put_back shape a real
/// container uses (`element.rs`'s own docstring), minus `FrameArena` erasure, which a
/// compile-time-known single child type never needs. `cx.with_children` is what makes every
/// registration `child` performs parent under this node's own — the mechanism this ticket adds.
struct Wrap<C: Element> {
    flags: crate::DispatchFlags,
    listeners: crate::ListenerSet,
    offset: (f32, f32, f32, f32),
    child: C,
}

impl<C: Element> Element for Wrap<C> {
    type LayoutState = C::LayoutState;
    type PrepaintState = C::PrepaintState;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn request_layout(&mut self, id: ElementId, cx: &mut LayoutCx<'_>) -> (crate::layout::LayoutNodeId, Self::LayoutState) {
        let child_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("child".into()));
        let (child_node, child_state) = self.child.request_layout(child_id, cx);
        let node = cx.container(&test_container_spec(), &[child_node]);
        (node, child_state)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn prepaint(&mut self, id: ElementId, bounds: Bounds, layout: &mut Self::LayoutState, cx: &mut PrepaintCx<'_>) -> Self::PrepaintState {
        let hitbox = crate::Hitbox { element: id, bounds, clips_children: self.flags.contains(crate::DispatchFlags::CLIPS_CHILDREN), hit_transparent: self.flags.contains(crate::DispatchFlags::HIT_TRANSPARENT) };
        let node = cx.register(id, self.flags, self.listeners.clone(), Some(hitbox));
        let child_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("child".into()));
        let child_bounds = offset_bounds(bounds, self.offset);
        let child = &mut self.child;
        let mut result = None;
        cx.with_children(node, |cx| {
            result = Some(child.prepaint(child_id, child_bounds, layout, cx));
        });
        result.expect("with_children always invokes its body exactly once")
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn paint(&mut self, id: ElementId, bounds: Bounds, layout: &mut Self::LayoutState, prepaint: &mut Self::PrepaintState, cx: &mut PaintCx<'_>) {
        let child_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("child".into()));
        let child_bounds = offset_bounds(bounds, self.offset);
        self.child.paint(child_id, child_bounds, layout, prepaint, cx);
    }
}

/// 🌳️ Two-child test container — `Wrap`'s sibling for the tests that need overlapping/adjacent
/// regions under one parent (an overlay over content, a container with more than one target).
struct Pair<A: Element, B: Element> {
    flags: crate::DispatchFlags,
    listeners: crate::ListenerSet,
    a: A,
    a_offset: (f32, f32, f32, f32),
    b: B,
    b_offset: (f32, f32, f32, f32),
}

impl<A: Element, B: Element> Element for Pair<A, B> {
    type LayoutState = (A::LayoutState, B::LayoutState);
    type PrepaintState = (A::PrepaintState, B::PrepaintState);

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn request_layout(&mut self, id: ElementId, cx: &mut LayoutCx<'_>) -> (crate::layout::LayoutNodeId, Self::LayoutState) {
        let a_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("a".into()));
        let b_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("b".into()));
        let (a_node, a_state) = self.a.request_layout(a_id, cx);
        let (b_node, b_state) = self.b.request_layout(b_id, cx);
        let node = cx.container(&test_container_spec(), &[a_node, b_node]);
        (node, (a_state, b_state))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn prepaint(&mut self, id: ElementId, bounds: Bounds, layout: &mut Self::LayoutState, cx: &mut PrepaintCx<'_>) -> Self::PrepaintState {
        let hitbox = crate::Hitbox { element: id, bounds, clips_children: self.flags.contains(crate::DispatchFlags::CLIPS_CHILDREN), hit_transparent: self.flags.contains(crate::DispatchFlags::HIT_TRANSPARENT) };
        let node = cx.register(id, self.flags, self.listeners.clone(), Some(hitbox));
        let a_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("a".into()));
        let b_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("b".into()));
        let a_bounds = offset_bounds(bounds, self.a_offset);
        let b_bounds = offset_bounds(bounds, self.b_offset);
        let (a, b) = (&mut self.a, &mut self.b);
        let (a_layout, b_layout) = (&mut layout.0, &mut layout.1);
        let mut result: Option<Self::PrepaintState> = None;
        cx.with_children(node, |cx| {
            let a_prepaint = a.prepaint(a_id, a_bounds, a_layout, cx);
            let b_prepaint = b.prepaint(b_id, b_bounds, b_layout, cx);
            result = Some((a_prepaint, b_prepaint));
        });
        result.expect("with_children always invokes its body exactly once")
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn paint(&mut self, id: ElementId, bounds: Bounds, layout: &mut Self::LayoutState, prepaint: &mut Self::PrepaintState, cx: &mut PaintCx<'_>) {
        let a_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("a".into()));
        let b_id = ElementId::new(Some(id), &ReconciliationKey::Explicit("b".into()));
        let a_bounds = offset_bounds(bounds, self.a_offset);
        let b_bounds = offset_bounds(bounds, self.b_offset);
        self.a.paint(a_id, a_bounds, &mut layout.0, &mut prepaint.0, cx);
        self.b.paint(b_id, b_bounds, &mut layout.1, &mut prepaint.1, cx);
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn act(name: &str) -> ActionId {
    ActionId::try_v1("test", name).expect("bounded test action")
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn bind(trigger: Trigger, name: &str) -> ActionBinding {
    ActionBinding { trigger, action: act(name), args: None, capability: None }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn listen(bindings: Vec<ActionBinding>) -> crate::ListenerSet {
    crate::ListenerSet { surface: SurfaceId(UiText::try_from_str("s").expect("bounded fixture surface")), node: UiNodeId(1), node_key: UiText::try_from_str("k").expect("bounded fixture key"), revision: UiRev(0), value: None, bindings }
}

/// 🏁️ Runs `root` through a real `build_frame` call against a fresh `FrameEngine` and returns the
/// presented snapshot — the harness every test below shares.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn build<E: Element>(root: E, viewport: [f32; 2]) -> Rc<FrameSnapshot> {
    let mut engine = FrameEngine::new();
    let mut resources = crate::resource::ResourceRegistry::default();
    let mut text = crate::TextSystem::default();
    let mut scheduler = FrameScheduler::default();
    let inputs = FrameInputs { resources: &mut resources, text: &mut text, scheduler: &mut scheduler, viewport, dpr: 1.0, time_seconds: 0.0 };
    engine.build_frame(root, inputs).expect("build_frame must succeed for these test elements");
    engine.presented().expect("a successful build_frame always leaves a presented snapshot")
}

//#endregion 🧪️TestElements

#[test]
fn build_frame_populates_parent_links_overlay_flags_and_listeners_on_the_real_tree() {
    let root_element = Wrap {
        flags: crate::DispatchFlags::LAYOUT_CONTAINER,
        listeners: crate::ListenerSet::default(),
        offset: (0.0, 0.0, 40.0, 40.0),
        child: TestLeaf { flags: crate::DispatchFlags::OVERLAY, listeners: listen(vec![bind(Trigger::Activate, "open")]) },
    };
    let snapshot = build(root_element, [200.0, 200.0]);
    let tree = &snapshot.dispatch;

    let root_id = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));
    let child_id = ElementId::new(Some(root_id), &ReconciliationKey::Explicit("child".into()));
    let root_node = tree.element_node(root_id).expect("root must be registered by the real prepaint walk");
    let child_node = tree.element_node(child_id).expect("child must be registered by the real prepaint walk");

    assert_eq!(tree.node(root_node).unwrap().parent, None, "root has no parent");
    assert_eq!(tree.node(child_node).unwrap().parent, Some(root_node), "child's parent link must point at the real root node — this is exactly what a geometry-only reconstruction could never recover");
    assert!(tree.node(child_node).unwrap().flags.contains(crate::DispatchFlags::OVERLAY), "the child's own declared OVERLAY flag must survive registration");
    assert!(!tree.node(root_node).unwrap().flags.contains(crate::DispatchFlags::OVERLAY));
    assert_eq!(
        tree.node(child_node).unwrap().listeners.binding_for(Trigger::Activate).map(|binding| binding.action.clone()),
        Some(act("open")),
        "the child's real ActionBinding must survive registration into a typed ListenerSet, not an empty default one"
    );
}

#[test]
fn a_click_three_levels_deep_bubbles_to_the_root_through_the_built_tree() {
    let root_element = Wrap {
        flags: crate::DispatchFlags::LAYOUT_CONTAINER,
        listeners: crate::ListenerSet::default(),
        offset: (0.0, 0.0, 100.0, 100.0),
        child: Wrap { flags: crate::DispatchFlags::LAYOUT_CONTAINER, listeners: crate::ListenerSet::default(), offset: (10.0, 10.0, 50.0, 50.0), child: TestLeaf::new() },
    };
    let snapshot = build(root_element, [200.0, 200.0]);
    let tree = &snapshot.dispatch;

    let root_id = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));
    let mid_id = ElementId::new(Some(root_id), &ReconciliationKey::Explicit("child".into()));
    let leaf_id = ElementId::new(Some(mid_id), &ReconciliationKey::Explicit("child".into()));
    let root_node = tree.element_node(root_id).expect("root registered");
    let mid_node = tree.element_node(mid_id).expect("mid registered");
    let leaf_node = tree.element_node(leaf_id).expect("leaf registered");

    let leaf_bounds = crate::node_bounds(tree, leaf_node).expect("the leaf must carry a hitbox");
    let (x, y) = (leaf_bounds.x + 1.0, leaf_bounds.y + 1.0);
    assert_eq!(crate::hit_test(tree, root_node, x, y), Some(leaf_node), "the click must resolve to the deepest nested element through the real, prepaint-built tree");

    let mut dispatcher = crate::Dispatcher::new();
    let pointer = crate::PointerInfo { id: crate::PointerId(1), kind: crate::PointerKind::Mouse, pressure: None, tilt: None };
    dispatcher.dispatch(tree, &crate::DispatchEvent::PointerDown { pointer, x, y, button: crate::PointerButton::Primary });

    assert_eq!(dispatcher.capture_of(crate::PointerId(1)).map(|(element, _)| element), Some(tree.node(leaf_node).unwrap().element), "the press must capture the actual nested leaf, resolved through the built tree");
    assert!(dispatcher.is_hovered(tree.node(mid_node).unwrap().element), "bubbling must reach the mid ancestor through the built parent link — this is what PrepaintCx::with_children threads");
    assert!(dispatcher.is_hovered(tree.node(root_node).unwrap().element), "bubbling must reach the root through the built parent link, three levels up from where the click landed");
}

#[test]
fn an_overlay_registered_last_is_hit_before_the_content_beneath_it_through_the_real_pipeline() {
    let root_element = Pair {
        flags: crate::DispatchFlags::LAYOUT_CONTAINER,
        listeners: crate::ListenerSet::default(),
        a: TestLeaf::new(),
        a_offset: (0.0, 0.0, 50.0, 50.0),
        b: TestLeaf { flags: crate::DispatchFlags::OVERLAY, listeners: crate::ListenerSet::default() },
        b_offset: (0.0, 0.0, 50.0, 50.0),
    };
    let snapshot = build(root_element, [200.0, 200.0]);
    let tree = &snapshot.dispatch;

    let root_id = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));
    let overlay_id = ElementId::new(Some(root_id), &ReconciliationKey::Explicit("b".into()));
    let root_node = tree.element_node(root_id).expect("root registered");
    let overlay_node = tree.element_node(overlay_id).expect("overlay registered");

    let overlay_bounds = crate::node_bounds(tree, overlay_node).expect("the overlay must carry a hitbox");
    let hit = crate::hit_test(tree, root_node, overlay_bounds.x + 1.0, overlay_bounds.y + 1.0);
    assert_eq!(hit, Some(overlay_node), "an OVERLAY-flagged sibling registered last must win the hit test over fully overlapping content beneath it, through the real built tree");
}

#[test]
fn a_layout_container_with_no_bindings_passes_the_hit_through_to_what_is_under_it() {
    let root_element = Wrap { flags: crate::DispatchFlags::LAYOUT_CONTAINER, listeners: crate::ListenerSet::default(), offset: (0.0, 0.0, 20.0, 20.0), child: TestLeaf::new() };
    let snapshot = build(root_element, [200.0, 200.0]);
    let tree = &snapshot.dispatch;

    let root_id = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));
    let child_id = ElementId::new(Some(root_id), &ReconciliationKey::Explicit("child".into()));
    let root_node = tree.element_node(root_id).expect("root registered");
    let child_node = tree.element_node(child_id).expect("child registered");

    let root_bounds = crate::node_bounds(tree, root_node).expect("root must carry a hitbox");
    let child_bounds = crate::node_bounds(tree, child_node).expect("child must carry a hitbox");
    let (px, py) = (root_bounds.x + root_bounds.w - 1.0, root_bounds.y + root_bounds.h - 1.0);
    assert!(px >= child_bounds.x + child_bounds.w || py >= child_bounds.y + child_bounds.h, "test setup sanity: the query point must fall inside the root but outside the child");

    assert_eq!(crate::hit_test(tree, root_node, px, py), None, "a LAYOUT_CONTAINER with no bindings must never itself match — it is a pure pass-through, even reached through the real built tree");
    assert_eq!(crate::hit_test(tree, root_node, child_bounds.x + 1.0, child_bounds.y + 1.0), Some(child_node), "clicking directly on the child must still find it");
}
