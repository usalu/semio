//! @emoji 🧱️ The `Element` trait, stable `ElementId` identity, and the per-frame arena that erases
//! concrete `Element` types without `dyn Element` (ruling U3).
//!
//! **Erasure mechanism (U3):** `dyn Element` is banned because `Element` carries two associated
//! types (`LayoutState`, `PrepaintState`) that differ per implementor, so a vtable would need to be
//! built by hand regardless of whether the entry point is `dyn Element` or something else. Rather
//! than reach for raw-pointer casts, [`AnyElement`] pairs a **fn-pointer vtable** (three plain `fn`
//! items, monomorphized once per concrete `E: Element` and stored by value — fn pointers are `Copy`
//! and need no lifetime gymnastics) with **safe** `Box<dyn Any>` storage for the erased element and
//! its per-phase state. `dyn Any` is explicitly permitted by U3 (it is not a first-party trait); the
//! vtable's generic glue functions (`request_layout_erased::<E>` etc.) are the only place a
//! `downcast_mut::<E>()` happens, and it always succeeds because the vtable and the boxed value are
//! constructed together in [`AnyElement::new`] and never mixed across instances. No `unsafe` anywhere
//! in this file.
//!
//! **`FrameArena`** is the "per-frame bump arena": a single `Vec<Option<AnyElement>>` that every
//! [`AnyElement`] in one frame's tree lives in, allocated by index (push-only growth, O(1)) and
//! dropped wholesale when [`crate::frame::FrameEngine::build_frame`] clears it at the end of the
//! transaction. A container element holds its children as [`FrameArenaIndex`] values (cheap, `Copy`)
//! rather than owning `AnyElement`s directly, which is what makes the **take → recurse → put back**
//! pattern below sound: a container's own phase method holds `&mut *Cx` (which itself contains
//! `&mut FrameArena`), so it cannot simultaneously hold a live `&mut AnyElement` borrowed out of that
//! same arena while also handing `cx` to the child — `take` ends the arena borrow the moment it
//! returns an owned value, freeing `cx` for the recursive call:
//!
//! ```text
//! let mut child = cx.shared.arena.take(child_index);
//! let node = child.request_layout(child_id, cx);   // cx is fully free here
//! cx.shared.arena.put_back(child_index, child);
//! ```
//!
//! **`ElementId` stability:** `ElementId::new(parent, key)` folds the parent's own id together with
//! a [`ReconciliationKey`] through a hand-implemented FxHash (the same small, well-known
//! rotate-xor-multiply hash `rustc-hash` uses) — no crate dependency added, mirroring `🦀️arena.rs`'s
//! own hand-rolled generational arena in the wgpu-old target this packet ports from. Two calls with
//! equal `(parent, key)` always produce the same id, which is exactly what lets retained per-id state
//! (hover, scroll offset, animation clocks) survive an element tree rebuilt from scratch every frame.
//!
//! **The dispatch tree is built during prepaint, not derived from geometry afterward** (ticket
//! `dispatch-tree-seam`, wave W3). [`PrepaintCx::register`] is how an [`Element`] hands
//! [`crate::DispatchTree`] what a hit-testable region actually needs beyond a rect: its
//! [`crate::DispatchFlags`] (`OVERLAY`/`CLIPS_CHILDREN`/`HIT_TRANSPARENT`/`LAYOUT_CONTAINER`/…) and its
//! [`crate::ListenerSet`]. The parent link a container's own registered node gives its children is
//! threaded through [`PrepaintCx::with_children`], since prepaint's take→recurse→put_back call shape
//! (this file's own docstring above) has no other way to pass "my node id" down into a child's own
//! `prepaint` call — the walk itself already visits parent-before-children in order, which is exactly
//! what makes that link free to capture here and unrecoverable from a flat post-hoc hitbox list, the
//! reason the previous `DispatchTree: From<Vec<Hitbox>>` adapter could never carry real semantics.

use std::any::Any;
use std::collections::{HashMap, HashSet};

//#region 🔖️Identity

/// 🔑️ Stable child identity for keyed reconciliation, mirroring the wgpu-old target's
/// `tree::NodeKey`: an explicit key when the node declares one, else a `(discriminant, ordinal)`
/// positional fallback among same-kind siblings. A `Positional` element's identity does not survive a
/// reorder — only `Explicit` keys do — which is inherent to a positional fallback, not a bug here.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ReconciliationKey {
    Explicit(String),
    Positional(u32, u32),
}

const FXHASH_SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

/// 🌀️ One FxHash mixing step: rotate-xor-multiply, the same primitive `rustc-hash` uses for every
/// machine word it ingests.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn fxhash_word(hash: u64, word: u64) -> u64 {
    (hash.rotate_left(5) ^ word).wrapping_mul(FXHASH_SEED)
}

/// 🌀️ Folds `bytes` into `hash` eight bytes at a time (the tail chunk is zero-padded), matching
/// `rustc-hash`'s own byte-slice handling closely enough for this crate's purpose: a fast, stable,
/// dependency-free identity hash — not a cryptographic or DoS-resistant one.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn fxhash_bytes(hash: u64, bytes: &[u8]) -> u64 {
    let mut hash = hash;
    for chunk in bytes.chunks(8) {
        let mut word_bytes = [0u8; 8];
        word_bytes[..chunk.len()].copy_from_slice(chunk);
        hash = fxhash_word(hash, u64::from_le_bytes(word_bytes));
    }
    hash
}

/// 🪪️ A per-frame-rebuilt element's stable identity: `fxhash(parent ElementId, reconciliation key)`.
/// Deliberately a **separate identity domain** from [`ui_contract::UiNodeId`] (a protocol identity for
/// one row of a [`ui_contract::UiSnapshot`]'s flat node table) and from any entity id the headless
/// runtime mints — the three never alias each other, even when, as is common, one `UiNodeRecord`
/// happens to lower into exactly one `Element`. Stability across a from-scratch rebuild (never a
/// database key, never an index) is what lets hover state, scroll offsets, edit buffers and animation
/// clocks in [`RetainedStore`] survive frame to frame.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ElementId(u64);

impl ElementId {
    /// 🌱️ Combines an optional parent id with `key`. `parent: None` seeds the hash the same way a
    /// child would seed it from a parent id of `FXHASH_SEED` — there is no reserved all-zero root
    /// sentinel, so a root element's id is exactly as well-distributed as any other.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(parent: Option<ElementId>, key: &ReconciliationKey) -> Self {
        let seed = parent.map_or(FXHASH_SEED, |id| id.0);
        let hash = match key {
            ReconciliationKey::Explicit(text) => fxhash_bytes(seed, text.as_bytes()),
            ReconciliationKey::Positional(discriminant, ordinal) => fxhash_word(fxhash_word(seed, u64::from(*discriminant)), u64::from(*ordinal)),
        };
        Self(hash)
    }
}

//#endregion 🔖️Identity

//#region 🔖️Element

/// 📐️ A resolved, absolute (window-space) rect in logical pixels — the same type
/// [`crate::scene::LayoutRect`] already is, reused here rather than duplicated, since `prepaint`/
/// `paint` bounds and scene-primitive rects are exactly the same kind of value.
pub type Bounds = crate::scene::LayoutRect;

/// 🎯️ One hit-testable region's geometry, passed to [`PrepaintCx::register`] alongside the flags and
/// listeners [`crate::DispatchTree`] actually needs to interpret it (parent link, overlay bit,
/// bindings) — `Hitbox` itself stays geometry-only and is what [`crate::DispatchTree`] indexes by
/// [`crate::HitboxId`] for spatial lookups (`crate::DispatchTree::hitboxes`/`bounds`), never the
/// source of tree structure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hitbox {
    pub element: ElementId,
    pub bounds: Bounds,
    pub clips_children: bool,
    pub hit_transparent: bool,
}

/// 🗄️ Per-`ElementId` retained state — scroll offsets, edit buffers, animation clocks — that must
/// survive an element tree rebuilt from scratch every frame. `get_or_insert_with` marks `id` touched;
/// [`Self::end_frame`] releases every entry whose id was not touched since the last [`Self::begin_frame`],
/// which is what keeps this from growing unbounded as elements disappear.
#[derive(Default)]
pub struct RetainedStore {
    entries: HashMap<ElementId, Box<dyn Any>>,
    touched: HashSet<ElementId>,
}

impl RetainedStore {
    /// 🌱️ Clears the touched-set ahead of a new frame's walk. Existing entries are left in place —
    /// they are proven live only by being touched again before [`Self::end_frame`] runs.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn begin_frame(&mut self) {
        self.touched.clear();
    }

    /// 🔍️ Returns `id`'s retained value, creating it from `init` on first sight, and marks `id`
    /// touched this frame. Panics if `id` was already used this frame at a different `T` — a retained
    /// slot's type is part of an element's identity contract and must not change frame to frame.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn get_or_insert_with<T: 'static>(&mut self, id: ElementId, init: impl FnOnce() -> T) -> &mut T {
        self.touched.insert(id);
        let boxed = self.entries.entry(id).or_insert_with(|| Box::new(init()) as Box<dyn Any>);
        boxed.downcast_mut::<T>().expect("RetainedStore: retained state type changed for an ElementId between frames")
    }

    /// 🧹️ Drops every entry whose id was not touched since the last [`Self::begin_frame`] — the
    /// element that owned it no longer exists in this frame's tree.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn end_frame(&mut self) {
        let touched = &self.touched;
        self.entries.retain(|id, _| touched.contains(id));
    }
}

/// 🤝️ Fields every phase's context struct carries, factored out once instead of repeated three times.
/// `arena` lets any `Element` (typically a container) allocate/recurse into children via the
/// take-recurse-put-back pattern documented on this file's own docstring.
pub struct SharedFrameCx<'a> {
    pub arena: &'a mut FrameArena,
    pub resources: &'a mut crate::resource::ResourceRegistry,
    pub retained: &'a mut RetainedStore,
    pub time_seconds: f32,
}

/// 🎨️ Context for [`Element::prepaint`]: builds [`crate::DispatchTree`] and resolves text, ahead of
/// paint. `parent` is this walk's current dispatch-tree parent — private, since [`Self::register`]/
/// [`Self::with_children`] are the only sanctioned way to read or advance it; a phase method never sets
/// it by hand the way it would a public field.
pub struct PrepaintCx<'a> {
    pub shared: SharedFrameCx<'a>,
    pub dispatch: &'a mut crate::DispatchTree,
    pub text: &'a mut crate::TextSystem,
    parent: Option<crate::FrameNodeId>,
}

impl<'a> PrepaintCx<'a> {
    /// 🌱️ `parent: None` — the first [`Self::register`] call this frame becomes the tree's root (see
    /// [`crate::DispatchTree::insert`]).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(shared: SharedFrameCx<'a>, dispatch: &'a mut crate::DispatchTree, text: &'a mut crate::TextSystem) -> Self {
        Self { shared, dispatch, text, parent: None }
    }

    /// 🪪️ Registers `element` as a [`crate::DispatchTree`] node parented under whichever node the
    /// nearest enclosing [`Self::with_children`] call (if any) established — this is the "typed
    /// listeners" hand-off the ticket asks for: an [`Element`] supplies its own bounds/flags/listeners
    /// directly, instead of a later pass reconstructing them from geometry. Returns the node's
    /// [`crate::FrameNodeId`] so a container can pass it to [`Self::with_children`] to parent its own
    /// children under it.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn register(&mut self, element: ElementId, flags: crate::DispatchFlags, listeners: crate::ListenerSet, hitbox: Option<Hitbox>) -> crate::FrameNodeId {
        self.dispatch.insert(self.parent, element, flags, listeners, hitbox)
    }

    /// 🌳️ Runs `body` with `node` as the current dispatch-tree parent, restoring the previous parent
    /// afterward — the free parent link this file's docstring describes: a container `Element` calls
    /// this around the take→recurse→put_back calls it makes into its children's own `prepaint`, so
    /// every [`Self::register`] a descendant performs during `body` is parented under `node` without
    /// that descendant ever being told its own parent id explicitly.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn with_children(&mut self, node: crate::FrameNodeId, body: impl FnOnce(&mut Self)) {
        let previous = self.parent.replace(node);
        body(self);
        self.parent = previous;
    }
}

/// 🖌️ Context for [`Element::paint`]: the only phase allowed to append to the scene.
pub struct PaintCx<'a> {
    pub shared: SharedFrameCx<'a>,
    pub scene: &'a mut crate::scene::SceneBuilder,
}

/// 🧱️ Three strictly-ordered, strictly-synchronous phases (ruling U1): `request_layout` may not
/// paint; `prepaint` resolves bounds/hitboxes/text but never touches the scene; `paint` only appends
/// to [`PaintCx::scene`]. The split into three associated-type-carrying methods is the type-level
/// enforcement the ticket asks for: `prepaint` cannot be called without a `Self::LayoutState` value,
/// which in ordinary code only exists once `request_layout` produced one, and `paint` likewise needs
/// both `Self::LayoutState` and `Self::PrepaintState`. [`AnyElement`] additionally enforces the same
/// ordering at runtime (`expect`-panics with a named phase violation) for the erased path, since
/// erasure necessarily boxes the state and loses the compile-time guarantee.
pub trait Element {
    type LayoutState: 'static;
    type PrepaintState: 'static;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn request_layout(&mut self, id: ElementId, cx: &mut crate::layout::LayoutCx<'_>) -> (crate::layout::LayoutNodeId, Self::LayoutState);

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn prepaint(&mut self, id: ElementId, bounds: Bounds, layout: &mut Self::LayoutState, cx: &mut PrepaintCx<'_>) -> Self::PrepaintState;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn paint(&mut self, id: ElementId, bounds: Bounds, layout: &mut Self::LayoutState, prepaint: &mut Self::PrepaintState, cx: &mut PaintCx<'_>);
}

//#endregion 🔖️Element

//#region 🔖️Arena

/// 🧮️ An index into a [`FrameArena`] — cheap, `Copy`, and what a container `Element` stores for each
/// child instead of owning an [`AnyElement`] directly (see this file's docstring for why).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FrameArenaIndex(usize);

type ErasedLayoutFn = fn(&mut dyn Any, ElementId, &mut crate::layout::LayoutCx<'_>) -> (crate::layout::LayoutNodeId, Box<dyn Any>);
type ErasedPrepaintFn = fn(&mut dyn Any, ElementId, Bounds, &mut dyn Any, &mut PrepaintCx<'_>) -> Box<dyn Any>;
type ErasedPaintFn = fn(&mut dyn Any, ElementId, Bounds, &mut dyn Any, &mut dyn Any, &mut PaintCx<'_>);

/// 🧬️ Three monomorphized `fn` items per concrete `E: Element`, stored by value (fn pointers are
/// `Copy`) — the "fn-pointer vtable" ruling U3 calls for in place of `dyn Element`.
#[derive(Clone, Copy)]
struct ElementVTable {
    request_layout: ErasedLayoutFn,
    prepaint: ErasedPrepaintFn,
    paint: ErasedPaintFn,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn request_layout_erased<E: Element + 'static>(element: &mut dyn Any, id: ElementId, cx: &mut crate::layout::LayoutCx<'_>) -> (crate::layout::LayoutNodeId, Box<dyn Any>) {
    let element = element.downcast_mut::<E>().expect("AnyElement: vtable/element type mismatch — see AnyElement::new");
    let (node, state) = element.request_layout(id, cx);
    (node, Box::new(state))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn prepaint_erased<E: Element + 'static>(element: &mut dyn Any, id: ElementId, bounds: Bounds, layout: &mut dyn Any, cx: &mut PrepaintCx<'_>) -> Box<dyn Any> {
    let element = element.downcast_mut::<E>().expect("AnyElement: vtable/element type mismatch — see AnyElement::new");
    let layout = layout.downcast_mut::<E::LayoutState>().expect("AnyElement: layout state type mismatch — see AnyElement::new");
    Box::new(element.prepaint(id, bounds, layout, cx))
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn paint_erased<E: Element + 'static>(element: &mut dyn Any, id: ElementId, bounds: Bounds, layout: &mut dyn Any, prepaint: &mut dyn Any, cx: &mut PaintCx<'_>) {
    let element = element.downcast_mut::<E>().expect("AnyElement: vtable/element type mismatch — see AnyElement::new");
    let layout = layout.downcast_mut::<E::LayoutState>().expect("AnyElement: layout state type mismatch — see AnyElement::new");
    let prepaint = prepaint.downcast_mut::<E::PrepaintState>().expect("AnyElement: prepaint state type mismatch — see AnyElement::new");
    element.paint(id, bounds, layout, prepaint, cx);
}

/// 🧱️ One type-erased [`Element`], its erasure vtable, and its per-phase state once produced. Never
/// constructed as `dyn Element` (ruling U3) — see this file's docstring for the fn-pointer + `dyn Any`
/// mechanism. Phase methods `expect`-panic (never silently no-op) when called out of order, since an
/// erased element cannot lean on the compile-time ordering [`Element`]'s own associated types give
/// unerased call sites.
pub struct AnyElement {
    element: Box<dyn Any>,
    layout_state: Option<Box<dyn Any>>,
    prepaint_state: Option<Box<dyn Any>>,
    vtable: ElementVTable,
}

impl AnyElement {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new<E: Element + 'static>(element: E) -> Self {
        Self {
            element: Box::new(element),
            layout_state: None,
            prepaint_state: None,
            vtable: ElementVTable { request_layout: request_layout_erased::<E>, prepaint: prepaint_erased::<E>, paint: paint_erased::<E> },
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn request_layout(&mut self, id: ElementId, cx: &mut crate::layout::LayoutCx<'_>) -> crate::layout::LayoutNodeId {
        let (node, state) = (self.vtable.request_layout)(self.element.as_mut(), id, cx);
        self.layout_state = Some(state);
        self.prepaint_state = None;
        node
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn prepaint(&mut self, id: ElementId, bounds: Bounds, cx: &mut PrepaintCx<'_>) {
        let layout_state = self.layout_state.as_mut().expect("AnyElement::prepaint called before request_layout — phase order violated");
        let prepaint_state = (self.vtable.prepaint)(self.element.as_mut(), id, bounds, layout_state.as_mut(), cx);
        self.prepaint_state = Some(prepaint_state);
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn paint(&mut self, id: ElementId, bounds: Bounds, cx: &mut PaintCx<'_>) {
        let layout_state = self.layout_state.as_mut().expect("AnyElement::paint called before request_layout — phase order violated");
        let prepaint_state = self.prepaint_state.as_mut().expect("AnyElement::paint called before prepaint — phase order violated");
        (self.vtable.paint)(self.element.as_mut(), id, bounds, layout_state.as_mut(), prepaint_state.as_mut(), cx);
    }
}

/// 🕳️ The per-frame arena: every [`AnyElement`] in one frame's tree lives here, indexed by
/// [`FrameArenaIndex`]. `take`/`put_back` implement the borrow-splitting pattern this file's docstring
/// documents — a slot is briefly `None` while its element is out being recursed into.
#[derive(Default)]
pub struct FrameArena {
    slots: Vec<Option<AnyElement>>,
}

impl FrameArena {
    /// ➕️ Pushes a freshly erased `element` and returns its index. Bump-allocation: append-only,
    /// O(1), no reuse of freed slots within a frame — the whole arena is cleared wholesale by
    /// [`Self::clear`] rather than individual slots being recycled mid-frame.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn alloc<E: Element + 'static>(&mut self, element: E) -> FrameArenaIndex {
        let index = FrameArenaIndex(self.slots.len());
        self.slots.push(Some(AnyElement::new(element)));
        index
    }

    /// 📤️ Takes `index`'s element out, leaving the slot empty. Panics if `index` is already taken
    /// (a bug in the caller's own take/put_back nesting, never a legitimate state).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn take(&mut self, index: FrameArenaIndex) -> AnyElement {
        self.slots[index.0].take().expect("FrameArena::take: slot already taken — unbalanced take/put_back")
    }

    /// 📥️ Restores `element` to `index` after a `take`. Panics on a double put-back for the same
    /// reason `take` panics on a double take.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn put_back(&mut self, index: FrameArenaIndex, element: AnyElement) {
        let slot = &mut self.slots[index.0];
        assert!(slot.is_none(), "FrameArena::put_back: slot already occupied — unbalanced take/put_back");
        *slot = Some(element);
    }

    /// 🧨️ Drops every element wholesale — called once [`crate::frame::FrameEngine::build_frame`]'s
    /// transaction has extracted everything the presented [`crate::frame::FrameSnapshot`] needs, so
    /// nothing outside this file ever observes an `AnyElement` surviving past its frame.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn clear(&mut self) {
        self.slots.clear();
    }
}

//#endregion 🔖️Arena

//#region Tests

#[cfg(test)]
mod tests {
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
}

//#endregion Tests
