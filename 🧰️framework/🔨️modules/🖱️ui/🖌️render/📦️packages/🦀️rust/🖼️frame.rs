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

use crate::element::{Bounds, ElementId};
#[cfg(test)]
use crate::element::{Element, FrameArena, PaintCx, PrepaintCx, ReconciliationKey, RetainedStore, SharedFrameCx};
#[cfg(test)]
use crate::layout::LayoutCx;
use crate::scene::RenderPacket;
#[cfg(test)]
use crate::scene::{FinishParams, Scene, SceneBuilder, SceneError};
use crate::schedule::{Deadline, FrameScheduler};
#[cfg(test)]
use crate::schedule::InvalidationReason;
use std::rc::Rc;
#[cfg(test)]
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
    #[cfg(test)]
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
#[cfg(test)]
const ANIMATION_FRAME_INTERVAL_SECONDS: f64 = 1.0 / 60.0;

/// ⚙️ Drives one window's frame lifecycle: the frame arena and retained-state store (both cleared/
/// swept once per `build_frame` — see `element.rs`), the generation counter, and the presented
/// snapshot. `presented` is `None` until the first successful `build_frame` call.
#[derive(Default)]
pub struct FrameEngine {
    presented: Option<Rc<FrameSnapshot>>,
    #[cfg(test)]
    arena: FrameArena,
    #[cfg(test)]
    retained: RetainedStore,
    #[cfg(test)]
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
#[path = "../../🧪️tests/🔬️frame-unit/🦀️.rs"]
mod tests;
