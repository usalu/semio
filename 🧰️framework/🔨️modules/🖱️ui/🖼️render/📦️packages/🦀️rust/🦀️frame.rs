//! @emoji 🖼️ `FrameSnapshot`, `FrameEngine` and the presented/building swap.
//!
//! `build_frame` runs compose → layout → prepaint → paint → `Scene::finish` → commit as **one
//! synchronous run-to-completion transaction** (ruling U1). "Atomic swap" does not need `Arc`/`Mutex`
//! here — this whole crate is single-threaded and nothing suspends mid-transaction (U1's entire
//! point), so the only thing that must be true is transactional all-or-nothing visibility: if any step
//! fails, `self.presented` is left completely untouched and the caller still holds the previous,
//! fully-formed [`FrameSnapshot`]. An `Rc` is enough to make that snapshot cheaply shareable with
//! whoever dispatches input against it without cloning the [`crate::scene::RenderPacket`] itself.
//! **The invariant this exists to guarantee: input is always dispatched against the presented
//! generation** — a click lands on the element the user actually saw, never on a half-built successor,
//! because there is no such thing as a "half-built successor" visible from outside `build_frame`.
//!
//! `FocusSnapshot`/`ImeSnapshot`/`AccessibilitySnapshot` are defined here, not ported from anywhere —
//! nothing in this repo already names them.
//!
//! **The dispatch tree is built, not derived (ticket `dispatch-tree-seam`, wave W3).**
//! [`crate::DispatchTree`] is constructed fresh each `build_frame` call and handed to the prepaint walk
//! via [`crate::PrepaintCx::register`]/[`crate::PrepaintCx::with_children`] — every registration an
//! [`Element`] makes during its own `prepaint` call appends one node, carrying real parent/flags/
//! listeners, not a geometry-only [`crate::element::Hitbox`] reconstructed after the fact. The former
//! `crate::DispatchTree: From<Vec<Hitbox>>` hand-off (this packet's own earlier report flagged it as
//! known-lossy — no parent link, no overlay bit, no listeners) is deleted, not left behind:
//! [`crate::element::Hitbox`] stays geometry-only and now lives inside [`crate::DispatchTree`] itself
//! (spatial-index material, see `dispatch.rs::DispatchTree::hitboxes`), never the tree's structural
//! source.

use crate::element::{Bounds, Element, ElementId, FrameArena, PaintCx, PrepaintCx, ReconciliationKey, RetainedStore, SharedFrameCx};
use crate::layout::LayoutCx;
use crate::scene::{FinishParams, RenderPacket, Scene, SceneBuilder, SceneError};
use crate::schedule::{Deadline, FrameScheduler, InvalidationReason};
use std::rc::Rc;
use ui_contract::UiRevision;

//#region 🔖️Frame

//#region 🔢️FrameGeneration

/// 🔢️ A monotonically increasing frame counter — never reused, never decreasing. Two snapshots from
/// the same [`FrameEngine`] are strictly ordered by this alone.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct FrameGeneration(u64);

impl FrameGeneration {
    pub const ZERO: Self = Self(0);

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

//#endregion 🔢️FrameGeneration

//#region 🗒️FrameSnapshot fields

/// 🎯️ Which element (if any) holds keyboard focus this frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FocusSnapshot {
    pub focused: Option<ElementId>,
}

/// 🈶️ IME composition state for the focused element, if it is mid-composition.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImeSnapshot {
    pub composition: Option<String>,
    pub cursor_bounds: Option<Bounds>,
}

/// ♿️ One element's resolved accessibility node — [`ui_contract::AccessibilitySpec`] (already the
/// per-node contract type) paired with this frame's absolute bounds for it.
///
/// **Deliberately not `Default`.** `ElementId` itself is deliberately not `Default` (see
/// `element.rs`): every legitimate id is `fxhash(parent, key)`, so a manufactured "default" id would
/// be indistinguishable from — and could collide with — a real element's id, which is exactly the
/// mysterious-collision hazard the packet-`render-frame` follow-up asked to avoid. Nothing needs an
/// out-of-thin-air `AccessibilityNode`; `AccessibilitySnapshot::default()` below only needs `Vec`'s own
/// empty default, which requires nothing of its element type.
#[derive(Clone, Debug, PartialEq)]
pub struct AccessibilityNode {
    pub element: ElementId,
    pub bounds: Bounds,
    pub spec: ui_contract::AccessibilitySpec,
}

/// ♿️ Every accessibility node in this frame, flat — mirrors [`crate::element::Hitbox`]'s flat
/// list-plus-interpretation-elsewhere shape rather than a tree, since nothing here needs tree walk
/// order beyond what `element` + `bounds` already encode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccessibilitySnapshot {
    pub nodes: Vec<AccessibilityNode>,
}

//#endregion 🗒️FrameSnapshot fields

//#region 📸️FrameSnapshot

/// 📸️ Everything downstream of one `build_frame` call: the paintable packet, the dispatch tree built
/// during prepaint (its own `hitboxes()` accessor carries the flat spatial-index list — see
/// `dispatch.rs`), and every other piece of frame-coherent state input dispatch or an accessibility
/// tree walk needs — bundled together so all of it swaps in at once.
pub struct FrameSnapshot {
    pub generation: FrameGeneration,
    pub packet: RenderPacket,
    pub dispatch: crate::DispatchTree,
    pub focus: FocusSnapshot,
    pub ime: ImeSnapshot,
    pub access: AccessibilitySnapshot,
    pub next_deadline: Option<Deadline>,
}

//#endregion 📸️FrameSnapshot

//#region 📥️FrameInputs

/// 📥️ Everything [`FrameEngine::build_frame`] needs but does not itself own — resources/text/the
/// scheduler are long-lived across frames and owned by whatever host constructs a window, not by
/// [`FrameEngine`] (compare [`crate::resource::ResourceRegistry`]'s own docstring: "one registry is
/// long-lived across frames").
pub struct FrameInputs<'a> {
    pub resources: &'a mut crate::resource::ResourceRegistry,
    pub text: &'a mut crate::TextSystem,
    pub scheduler: &'a mut FrameScheduler,
    pub viewport: [f32; 2],
    pub dpr: f32,
    pub time_seconds: f64,
}

//#endregion 📥️FrameInputs

//#region ⚙️FrameEngine

/// 🎞️ A packet reporting [`RenderPacket::has_animated_primitives`] gets one more deadline registered
/// at roughly the next frame interval, so an in-flight loading/waiting/introducing border animation
/// keeps waking the window (master.md: "a packet reporting `has_animated_primitives` registers an
/// ANIMATION deadline").
const ANIMATION_FRAME_INTERVAL_SECONDS: f64 = 1.0 / 60.0;

/// ⚙️ Drives one window's frame lifecycle: the frame arena and retained-state store (both cleared/
/// swept once per `build_frame` — see `element.rs`), the generation counter, and the presented
/// snapshot. `presented` is `None` until the first successful `build_frame` call.
#[derive(Default)]
pub struct FrameEngine {
    presented: Option<Rc<FrameSnapshot>>,
    arena: FrameArena,
    retained: RetainedStore,
    generation: FrameGeneration,
}

impl FrameEngine {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self::default()
    }

    /// 📸️ A cheap clone of the currently presented snapshot — safe to hold across input dispatch even
    /// while a later `build_frame` call runs, since a failed build never touches `self.presented` (see
    /// this file's docstring) and a successful one only replaces it once fully formed.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn presented(&self) -> Option<Rc<FrameSnapshot>> {
        self.presented.clone()
    }

    /// 🏁️ compose → layout → prepaint → paint → `Scene::finish` → commit, one synchronous
    /// run-to-completion transaction over `root`. On `Err`, `self.presented`/`self.generation` are
    /// unchanged — the caller still has the last good frame. The dispatch tree is built live, node by
    /// node, as the prepaint walk registers each element (`crate::PrepaintCx::register`/
    /// `with_children` — see this file's module docstring), never derived from geometry afterward.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub fn build_frame<E: Element>(&mut self, mut root: E, inputs: FrameInputs<'_>) -> Result<FrameGeneration, SceneError> {
        let FrameInputs { resources, text, scheduler, viewport, dpr, time_seconds } = inputs;

        self.retained.begin_frame();
        self.arena.clear();
        let root_id = ElementId::new(None, &ReconciliationKey::Explicit("root".into()));

        let mut layout_cx = LayoutCx::new(SharedFrameCx { arena: &mut self.arena, resources: &mut *resources, retained: &mut self.retained, time_seconds: time_seconds as f32 });
        let (root_node, mut layout_state) = root.request_layout(root_id, &mut layout_cx);
        layout_cx.compute(root_node, viewport[0], viewport[1]);
        let bounds = layout_cx.resolved(root_node);

        // 🌱️ Freshly rebuilt every frame, matching the element tree it is built alongside — see this
        // file's module docstring. `UiRevision(0)` is a placeholder until a packet threads a real
        // `UiSnapshot`-sourced revision through `FrameInputs` (see this packet's report).
        let mut dispatch = crate::DispatchTree::new(UiRevision(0));
        let mut prepaint_cx = PrepaintCx::new(SharedFrameCx { arena: &mut self.arena, resources: &mut *resources, retained: &mut self.retained, time_seconds: time_seconds as f32 }, &mut dispatch, text);
        let mut prepaint_state = root.prepaint(root_id, bounds, &mut layout_state, &mut prepaint_cx);

        let mut scene_builder = SceneBuilder::default();
        let mut paint_cx = PaintCx { shared: SharedFrameCx { arena: &mut self.arena, resources: &mut *resources, retained: &mut self.retained, time_seconds: time_seconds as f32 }, scene: &mut scene_builder };
        root.paint(root_id, bounds, &mut layout_state, &mut prepaint_state, &mut paint_cx);

        let resource_ops = resources.drain_ops();
        let packet = Scene::finish(scene_builder, FinishParams { viewport, dpr, time_seconds_origin: time_seconds, resource_ops })?;

        if packet.has_animated_primitives {
            scheduler.request_deadline(time_seconds + ANIMATION_FRAME_INTERVAL_SECONDS, InvalidationReason::ANIMATION);
        }

        self.retained.end_frame();
        self.arena.clear();

        self.generation = self.generation.next();
        let snapshot = FrameSnapshot { generation: self.generation, packet, dispatch, focus: FocusSnapshot::default(), ime: ImeSnapshot::default(), access: AccessibilitySnapshot::default(), next_deadline: scheduler.next_deadline() };
        self.presented = Some(Rc::new(snapshot));
        Ok(self.generation)
    }
}

//#endregion ⚙️FrameEngine

//#endregion 🔖️Frame

/// 🧪️ `build_frame` is now fully exercisable in-file: `render-text`'s `crate::TextSystem` and this
/// ticket's own rebuilt `crate::DispatchTree` have both landed, and neither needs a forward-reference
/// stand-in any more. `TestLeaf`/`Wrap`/`Pair` below are minimal `Element`s that go through the real
/// `request_layout`→`prepaint`→`paint` phases and register themselves via `PrepaintCx::register`/
/// `with_children` exactly as a production element would — no `crate::DispatchTree` is ever hand-built
/// in this module, unlike `dispatch.rs`'s own test suite (deliberately: that suite is the semantic
/// specification against hand-assembled trees, this one proves the real construction path produces the
/// same shape). Geometry below the root is authored directly by each test element's own fixed `offset`
/// rather than left to taffy's flex arithmetic, and every assertion below reads bounds back from the
/// built tree itself (`crate::node_bounds`) rather than predicting them — this suite does not depend on
/// this packet's own unverified guesses about taffy's exact numeric layout (see the packet report).
#[cfg(test)]
mod tests {
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
}
