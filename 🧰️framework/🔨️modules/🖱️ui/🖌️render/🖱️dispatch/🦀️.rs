//! @emoji 🖱️ Hit testing, capture/target/bubble propagation, focus, pointer capture, drag/drop and
//! `DispatchOutcome` — the frame-local dispatch tree that replaces `wgpu-old`'s retained-mode
//! `events.rs`/`EventRouter`. Semantics are ported verbatim from that file; only the structure changes:
//! a frame-local [`DispatchTree`] of generic [`DispatchNode`]s with typed [`ListenerSet`]s stands in
//! for the retained `UiTree` of product-specific `UiNode` variants, and the persistent [`Dispatcher`]
//! stands in for `EventRouter`.
//!
//! **Read-only per frame, mutable across frames.** [`DispatchTree`] lives inside a `Rc<FrameSnapshot>`
//! (ruling U1's "dispatch always runs against the presented generation") and is never mutated by this
//! module — every old `EventRouter` behaviour that flipped a `NodeFlags` bit on the retained tree
//! (`ACTIVE`/`FOCUSED`/`HOVERED`/`OVERLAY`) instead becomes a read accessor on the persistent
//! [`Dispatcher`], keyed by [`ElementId`] (the one identity stable across a frame rebuild — see
//! `element.rs`'s own docstring). A widget `Element`'s own `prepaint`/`paint` consults those accessors
//! next frame to decide its own visual state; this module never reaches back into the tree it was
//! handed.
//!
//! **Absolute bounds simplify the port.** `events.rs`'s `layout.x`/`layout.y` were parent-relative,
//! which is why that file needed `node_abs_origin`'s parent-chain walk and `hit_test_subtree`'s
//! re-basing math. [`crate::element::Hitbox::bounds`] is already window-absolute (the `bounds` param
//! `Element::prepaint` receives), so neither is needed here — `hit_test_subtree` degenerates to
//! `hit_test` with a different root.
//!
//! **Generic nodes, typed listeners.** The old `is_plain_stack_container` check pattern-matched
//! `UiNode::Stack`; this module has no product enum to match on, so the equivalent opt-in is the
//! [`DispatchFlags::LAYOUT_CONTAINER`] flag — an element that is *purely* layout (the old `Stack`) sets
//! it and becomes a hit-test pass-through unless it carries a binding of its own or is a drag source,
//! exactly `is_plain_stack_container`'s rule. Every other element (the old `Text`/`Button`/`Input`/…)
//! never sets it and stays a real hit-test target regardless of whether it has listeners — this is load
//! -bearing: ported test `hit_test_finds_the_topmost_of_two_non_overlapping_siblings` hits two bare
//! `Text`-equivalent leaves with zero bindings and still expects them to match.
//!
//! `ListenerSet` carries the contract's own [`ui_contract::ActionBinding`]s (already a closed,
//! product-agnostic `Trigger` enum — `Activate`/`Change`/`Commit`/`Delta`/`Drop`/`Submit`/`Abort`/
//! `RepeatLast`/`HoverPreview`) plus the protocol addressing (`surface`/`node`/`node_key`/`revision`)
//! needed to build a real [`ui_contract::UiIntent`] when one fires — the "typed listeners" the ticket
//! asks for, replacing the old `HitKind` enum without growing a new variant per product feature.
//!
//! 🚫️async: every `fn` below is plain sync per ruling U1 — a frame transaction (which input dispatch is
//! part of) is run-to-completion, never suspended. See ticket 26/08/20 📌️important.md.

use std::collections::HashMap;

use crate::element::{Bounds, ElementId, Hitbox};
use crate::schedule::InvalidationReason;
use ui_contract::{ActionBinding, ActionId, SurfaceId, Trigger, UiIntent, UiNodeId, UiRevision, UiText, UiValue};

//#region 🔖️HitTest

/// 🧮️ A [`DispatchNode`]'s index within one [`DispatchTree`] — valid only for the tree it came from;
/// never persisted across a frame rebuild (use [`ElementId`] for that — see this file's docstring).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FrameNodeId(u32);

/// 🧮️ A [`Hitbox`]'s index within one [`DispatchTree::hitboxes`] — same per-frame-only validity as
/// [`FrameNodeId`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HitboxId(u32);

/// 🧮️ A [`DispatchNode`]'s slot in the focus ring — present exactly when the node carries
/// [`DispatchFlags::FOCUSABLE`]. A thin wrapper over [`FrameNodeId`] rather than a separate allocation,
/// since a focus target always *is* a dispatch node.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FocusId(FrameNodeId);

/// 🚩️ Hand-rolled bitflag `u32`, matching `schedule.rs::InvalidationReason`'s own convention (this
/// crate's `Cargo.toml` is registrar-only and already carries no bitflags-style dependency).
///
/// The old `tree::NodeFlags` additionally carried `ACTIVE`/`FOCUSED`/`HOVERED` — dynamic interaction
/// *state*, which this port deliberately does not put here: [`DispatchTree`] is immutable per frame
/// (shared via `Rc<FrameSnapshot>`), so those three live on [`Dispatcher`] instead, keyed by
/// [`ElementId`], and are read accessors rather than flags flipped in place. What remains here is every
/// flag that is a **declared capability** of the node itself, set once when the element is built.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DispatchFlags(u32);

impl DispatchFlags {
    pub const NONE: Self = Self(0);
    pub const CLIPS_CHILDREN: Self = Self(1 << 0);
    pub const HIT_TRANSPARENT: Self = Self(1 << 1);
    pub const OVERLAY: Self = Self(1 << 2);
    pub const DRAG_SOURCE: Self = Self(1 << 3);
    pub const DROP_TARGET: Self = Self(1 << 4);
    pub const SCROLLABLE: Self = Self(1 << 5);
    pub const FOCUSABLE: Self = Self(1 << 6);
    /// 🌳️ Generic replacement for matching `UiNode::Stack` — see this file's module docstring.
    pub const LAYOUT_CONTAINER: Self = Self(1 << 7);
    /// 🔽️ Generic replacement for `toggle_select_popup`'s `Select`-specific wiring: `PointerUp`
    /// activating a node with this flag toggles an overlay whose root *and* anchor are that same node.
    pub const OVERLAY_TRIGGER: Self = Self(1 << 8);
    /// ✍️ Generic replacement for matching `UiNode::Input`: focusing this node seeds an [`EditState`].
    pub const EDITABLE: Self = Self(1 << 9);

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl std::ops::BitOr for DispatchFlags {
    type Output = Self;
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// 📐️ Half-open rect containment — `Bounds`/`LayoutRect` (`scene.rs`) carries no such method itself.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn rect_contains(bounds: Bounds, x: f32, y: f32) -> bool {
    x >= bounds.x && x < bounds.x + bounds.w && y >= bounds.y && y < bounds.y + bounds.h
}

/// 📐️ A node's absolute bounds, if it has a hitbox — `None` for a purely logical node with no
/// geometry of its own.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn node_bounds(tree: &DispatchTree, id: FrameNodeId) -> Option<Bounds> {
    tree.node(id)?.hitbox.map(|hitbox| tree.bounds(hitbox))
}

/// 🌳️ Ported from `events.rs::is_plain_stack_container` — see this file's module docstring for why
/// the check is now flag-driven instead of a variant match. Every exception is preserved verbatim: a
/// [`DispatchFlags::LAYOUT_CONTAINER`] node stops being a pass-through the moment it carries any
/// binding of its own, or is a registered [`DispatchFlags::DRAG_SOURCE`].
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_plain_pass_through(node: &DispatchNode) -> bool {
    node.flags.contains(DispatchFlags::LAYOUT_CONTAINER) && node.listeners.bindings.is_empty() && !node.flags.contains(DispatchFlags::DRAG_SOURCE)
}

/// 🎯️ Ported from `events.rs::hit_test`/`hit_test_node`: overlay-flagged children are tested before
/// normal siblings at every level (reverse-paint-order within each group), `CLIPS_CHILDREN` prunes the
/// whole subtree when the point falls outside the node's own bounds, `HIT_TRANSPARENT` and a plain
/// layout-container pass-through are skipped for the match itself but still recursed into. Returns the
/// deepest/topmost matching node.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn hit_test(tree: &DispatchTree, root: FrameNodeId, x: f32, y: f32) -> Option<FrameNodeId> {
    hit_test_node(tree, root, x, y)
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn hit_test_node(tree: &DispatchTree, id: FrameNodeId, x: f32, y: f32) -> Option<FrameNodeId> {
    let node = tree.node(id)?;
    let bounds = node.hitbox.map(|hitbox| tree.bounds(hitbox));
    let inside = bounds.is_some_and(|bounds| rect_contains(bounds, x, y));
    if node.flags.contains(DispatchFlags::CLIPS_CHILDREN) && !inside {
        return None;
    }
    for &child in tree.children(id).iter().rev().filter(|child| tree.node(**child).is_some_and(|node| node.flags.contains(DispatchFlags::OVERLAY))) {
        if let Some(hit) = hit_test_node(tree, child, x, y) {
            return Some(hit);
        }
    }
    for &child in tree.children(id).iter().rev().filter(|child| !tree.node(**child).is_some_and(|node| node.flags.contains(DispatchFlags::OVERLAY))) {
        if let Some(hit) = hit_test_node(tree, child, x, y) {
            return Some(hit);
        }
    }
    if inside && !node.flags.contains(DispatchFlags::HIT_TRANSPARENT) && !is_plain_pass_through(node) {
        Some(id)
    } else {
        None
    }
}

/// 🎯️ Ported from `events.rs::hit_test_subtree`, simplified: since [`Hitbox::bounds`] is already
/// window-absolute (unlike `wgpu-old`'s parent-relative `layout.x`/`layout.y`), no re-basing into the
/// subtree's own coordinate frame is needed — this is exactly [`hit_test`] with a non-root starting
/// point.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn hit_test_subtree(tree: &DispatchTree, subtree_root: FrameNodeId, x: f32, y: f32) -> Option<FrameNodeId> {
    hit_test(tree, subtree_root, x, y)
}

//#endregion 🔖️HitTest

//#region 🔖️DispatchTree

/// 🧩️ One node's typed interaction contract — the "typed listeners" that replace the old `HitKind`
/// enum. `bindings` are the node's own [`ui_contract::ActionBinding`]s (already the contract's closed,
/// product-agnostic `Trigger` set); `surface`/`node`/`node_key`/`revision` are the protocol addressing
/// needed to stamp a real [`UiIntent`] when one fires, captured at the revision this node's snapshot
/// was built from — see [`is_stale`] for why that revision matters. `value` is the node's current
/// declarative value if it is [`DispatchFlags::EDITABLE`] (`None` otherwise), used only to seed an
/// [`EditState`] on focus gain — dispatch never interprets it beyond that.
#[derive(Debug, Default)]
pub struct ListenerSet {
    pub surface: SurfaceId,
    pub node: UiNodeId,
    pub node_key: UiText,
    pub revision: UiRevision,
    pub value: Option<UiValue>,
    pub bindings: Vec<ActionBinding>,
}

impl ListenerSet {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            surface: self.surface.clone(),
            node: self.node,
            node_key: self.node_key.clone(),
            revision: self.revision,
            value: match self.value.as_ref() {
                Some(value) => Some(value.credited_clone()?),
                None => None,
            },
            bindings: self.bindings.iter().map(ActionBinding::credited_clone).collect::<Option<Vec<_>>>()?,
        })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn binding_for(&self, trigger: Trigger) -> Option<&ActionBinding> {
        self.bindings.iter().find(|binding| binding.trigger == trigger)
    }
}

#[cfg(test)]
impl Clone for ListenerSet {
    fn clone(&self) -> Self {
        self.credited_clone().expect("bounded test listener set has clone credits")
    }
}

/// 🧱️ One frame-local dispatch node — see this file's module docstring for why dynamic interaction
/// state (`ACTIVE`/`FOCUSED`/`HOVERED`) is deliberately absent: [`DispatchTree`] is immutable once
/// built, so that state lives on [`Dispatcher`] instead, keyed by [`ElementId`].
pub struct DispatchNode {
    pub parent: Option<FrameNodeId>,
    pub hitbox: Option<HitboxId>,
    pub focus: Option<FocusId>,
    pub element: ElementId,
    pub flags: DispatchFlags,
    pub listeners: ListenerSet,
}

/// 🌳️ The frame-local interpreted dispatch tree — built node-by-node during prepaint (see
/// `element.rs`'s [`crate::PrepaintCx::register`]/[`crate::PrepaintCx::with_children`]) and held
/// read-only inside the presented `Rc<FrameSnapshot>` for the lifetime of that generation. `index` is
/// the reverse [`ElementId`] →
/// [`FrameNodeId`] lookup [`Dispatcher`] uses to resolve its own persistent, `ElementId`-keyed state
/// (capture/focus/overlay roots/drag source) against *this* frame's tree.
#[derive(Default)]
pub struct DispatchTree {
    nodes: Vec<DispatchNode>,
    children: Vec<Vec<FrameNodeId>>,
    hitboxes: Vec<Hitbox>,
    root: Option<FrameNodeId>,
    revision: UiRevision,
    index: HashMap<ElementId, FrameNodeId>,
}

impl DispatchTree {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(revision: UiRevision) -> Self {
        Self { nodes: Vec::new(), children: Vec::new(), hitboxes: Vec::new(), root: None, revision, index: HashMap::new() }
    }

    /// ➕️ Appends one node. A `hitbox.clips_children`/`hitbox.hit_transparent` bit, if the hitbox is
    /// present, is folded into `flags` automatically so a caller never has to state the same fact
    /// twice. `parent: None` makes this node the tree's root (last such call wins — real callers build
    /// exactly one root).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn insert(&mut self, parent: Option<FrameNodeId>, element: ElementId, mut flags: DispatchFlags, listeners: ListenerSet, hitbox: Option<Hitbox>) -> FrameNodeId {
        let id = FrameNodeId(self.nodes.len() as u32);
        let hitbox_id = hitbox.map(|hitbox| {
            if hitbox.clips_children {
                flags.insert(DispatchFlags::CLIPS_CHILDREN);
            }
            if hitbox.hit_transparent {
                flags.insert(DispatchFlags::HIT_TRANSPARENT);
            }
            let hitbox_id = HitboxId(self.hitboxes.len() as u32);
            self.hitboxes.push(hitbox);
            hitbox_id
        });
        let focus = flags.contains(DispatchFlags::FOCUSABLE).then_some(FocusId(id));
        self.nodes.push(DispatchNode { parent, hitbox: hitbox_id, focus, element, flags, listeners });
        self.children.push(Vec::new());
        match parent {
            Some(parent) => self.children[parent.0 as usize].push(id),
            None => self.root = Some(id),
        }
        self.index.insert(element, id);
        id
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn node(&self, id: FrameNodeId) -> Option<&DispatchNode> {
        self.nodes.get(id.0 as usize)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn children(&self, id: FrameNodeId) -> &[FrameNodeId] {
        self.children.get(id.0 as usize).map_or(&[], Vec::as_slice)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn element_node(&self, element: ElementId) -> Option<FrameNodeId> {
        self.index.get(&element).copied()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn bounds(&self, hitbox: HitboxId) -> Bounds {
        self.hitboxes[hitbox.0 as usize].bounds
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn root(&self) -> Option<FrameNodeId> {
        self.root
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn revision(&self) -> UiRevision {
        self.revision
    }

    /// 📇️ Every hitbox registered this frame, flat — [`Hitbox`] earning its place as a spatial-index
    /// source (a future quadtree/BVH over pointer-heavy scenes) rather than as tree structure, which
    /// now lives in [`DispatchNode::parent`]/[`Self::children`] instead. Index order matches
    /// [`HitboxId`] (`self.bounds`'s own indexing), never insertion-into-`nodes` order for a node
    /// without a hitbox.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn hitboxes(&self) -> &[Hitbox] {
        &self.hitboxes
    }
}

//#endregion 🔖️DispatchTree

//#region 🔖️Propagation

//#region 📥️DispatchEvent

/// 🆔️ One physical pointer's identity — multi-pointer capable per U3's platform-events rule (never a
/// winit type): a host normalizes touch/pen/mouse input into these before calling [`Dispatcher::dispatch`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PointerId(pub u64);

/// 🖊️ Which physical device produced a pointer event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerKind {
    Mouse,
    Touch,
    Pen,
    Eraser,
}

/// 🖊️ Per-pointer identity plus whatever the device reports beyond x/y — `pressure`/`tilt` are `None`
/// for a mouse or a touch point that doesn't report them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerInfo {
    pub id: PointerId,
    pub kind: PointerKind,
    pub pressure: Option<f32>,
    pub tilt: Option<(f32, f32)>,
}

/// 🖱️ Mouse button identity — ported from `events.rs::PointerButton` verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
    Middle,
}

/// ⌨️ Modifier keys — ported from `events.rs::EventModifiers` verbatim.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EventModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// 🈶️ IME composition lifecycle — ported from `events.rs::ImeEvent` verbatim.
#[derive(Clone, Debug, PartialEq)]
pub enum ImeEvent {
    Start,
    Update { text: String, cursor: usize },
    Commit { text: String },
    Cancel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextEditTarget {
    Text,
    Paste,
}

/// 📥️ Input events a host feeds into [`Dispatcher::dispatch`] — the same shape as
/// `events.rs::UiEvent`, generalized to carry a full [`PointerInfo`] instead of bare x/y (U3: multi-
/// pointer capable, never a winit type).
#[derive(Clone, Debug, PartialEq)]
pub enum DispatchEvent {
    PointerDown { pointer: PointerInfo, x: f32, y: f32, button: PointerButton },
    PointerUp { pointer: PointerInfo, x: f32, y: f32, button: PointerButton },
    PointerMove { pointer: PointerInfo, x: f32, y: f32 },
    Scroll { x: f32, y: f32, delta_x: f32, delta_y: f32 },
    KeyDown { key: String, modifiers: EventModifiers },
    KeyUp { key: String, modifiers: EventModifiers },
    TextInput { text: String },
    Paste { text: String },
    TextEditStart { stream: u64, target: TextEditTarget, declared_bytes: usize },
    TextEditChunk { stream: u64, text: String },
    TextEditCommit { stream: u64 },
    TextEditAbort { stream: u64 },
    Ime(ImeEvent),
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn event_pointer(event: &DispatchEvent) -> Option<PointerId> {
    match event {
        DispatchEvent::PointerDown { pointer, .. } | DispatchEvent::PointerUp { pointer, .. } | DispatchEvent::PointerMove { pointer, .. } => Some(pointer.id),
        _ => None,
    }
}

//#endregion 📥️DispatchEvent

//#region 🫧️Bubble

/// 🫧️ Ported from `events.rs::bubble` verbatim: walks from `from` up through `parent` links
/// (including `from` itself), calling `handler(id)` for each ancestor until it returns `true`
/// ("handled, stop bubbling — this is the cancellation ticket U1/U2 ask for") or the root is reached.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn bubble<F: FnMut(FrameNodeId) -> bool>(tree: &DispatchTree, from: FrameNodeId, mut handler: F) {
    let mut cursor = Some(from);
    while let Some(id) = cursor {
        if handler(id) {
            return;
        }
        cursor = tree.node(id).and_then(|node| node.parent);
    }
}

/// 🌳️ Ported from `events.rs::is_descendant` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_descendant(tree: &DispatchTree, id: FrameNodeId, ancestor: FrameNodeId) -> bool {
    let mut found = false;
    bubble(tree, id, |current| {
        if current == ancestor {
            found = true;
            true
        } else {
            false
        }
    });
    found
}

//#endregion 🫧️Bubble

//#region 🔒️Capture

/// ↕️ Ported from `events.rs::ScrollAxis` verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAxis {
    Horizontal,
    Vertical,
}

/// 🫳️ Ported from `events.rs::CaptureKind` verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureKind {
    Press,
    Drag,
    ScrollThumb(ScrollAxis),
}

/// 🔒️ One pointer's capture: the target it was taken against, the kind, and the tree revision it was
/// taken at — the last is what [`is_stale`] compares against the *current* tree's revision when the
/// capture is finally released, so a capture that outlived several revisions of unrelated churn cannot
/// misapply a stale action (see this file's module docstring and the `stale_capture_is_rejected...` test).
#[derive(Clone, Copy, Debug, PartialEq)]
struct CaptureEntry {
    element: ElementId,
    kind: CaptureKind,
    revision: UiRevision,
}

/// 🔢️ Ported from master.md's own rule ("Stale intents (revision < current − 1) are dropped"): a
/// capture/listener revision more than one behind the tree it is now being resolved against is stale.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_stale(recorded: UiRevision, current: UiRevision) -> bool {
    current.0 > recorded.0.saturating_add(1)
}

//#endregion 🔒️Capture

//#region 🪟️Overlay

/// 🏷️ Ported from `events.rs::OverlayKind` verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayKind {
    SelectPopup,
    ContextMenu,
    Tooltip,
    Dialog,
    CommandPalette,
}

/// ⚓️ Ported from `events.rs::OverlayAnchor`, `NodeId` swapped for the stable [`ElementId`] (an
/// overlay anchor must survive across the frame boundary that opens it, unlike a [`FrameNodeId`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayAnchor {
    Element(ElementId),
    Point { x: f32, y: f32 },
}

/// 📐️ Ported from `events.rs::OverlayPlacement` verbatim.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayPlacement {
    BelowAnchorWithFlip,
    AtPointer { offset_x: f32, offset_y: f32 },
    Centered,
}

/// 🚪️ Ported from `events.rs::DismissPolicy` verbatim.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DismissPolicy {
    pub outside_press_swallow: bool,
    pub escape_closes: bool,
    pub hover_out_delay_seconds: Option<f32>,
}

impl OverlayKind {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn default_placement(self) -> OverlayPlacement {
        match self {
            OverlayKind::SelectPopup | OverlayKind::ContextMenu => OverlayPlacement::BelowAnchorWithFlip,
            OverlayKind::Tooltip => OverlayPlacement::AtPointer { offset_x: 12.0, offset_y: 16.0 },
            OverlayKind::Dialog | OverlayKind::CommandPalette => OverlayPlacement::Centered,
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn dismiss_policy(self) -> DismissPolicy {
        match self {
            OverlayKind::Tooltip => DismissPolicy { outside_press_swallow: false, escape_closes: true, hover_out_delay_seconds: Some(0.4) },
            _ => DismissPolicy { outside_press_swallow: true, escape_closes: true, hover_out_delay_seconds: None },
        }
    }
}

/// 🪟️ Ported from `events.rs::OpenOverlay`, `root`/`anchor` swapped to [`ElementId`] for the same
/// cross-frame-survival reason as [`OverlayAnchor`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenOverlay {
    pub root: ElementId,
    pub kind: OverlayKind,
    pub anchor: OverlayAnchor,
    pub placement: OverlayPlacement,
    pub dismiss: DismissPolicy,
    pub focus_trap: bool,
}

/// 🥞️ Ported from `events.rs::OverlayStack` verbatim (z-order: last = topmost = hit-tested first).
#[derive(Default, Clone, Debug)]
struct OverlayStack {
    open: Vec<OpenOverlay>,
}

impl OverlayStack {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn open(&mut self, overlay: OpenOverlay) {
        if self.open.len() < 16 {
            self.open.push(overlay);
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn topmost(&self) -> Option<&OpenOverlay> {
        self.open.last()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn close_root(&mut self, root: ElementId) -> Option<OpenOverlay> {
        let position = self.open.iter().position(|overlay| overlay.root == root)?;
        Some(self.open.remove(position))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn close_topmost(&mut self) -> Option<OpenOverlay> {
        self.open.pop()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn topmost_focus_trap_root(&self) -> Option<ElementId> {
        self.open.iter().rev().find(|overlay| overlay.focus_trap).map(|overlay| overlay.root)
    }
}

/// 📐️ Ported from `events.rs::resolve_overlay_placement` verbatim (field-for-field math), with
/// `OverlayAnchor::Node` resolved through [`DispatchTree::element_node`] instead of a direct `NodeId`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn resolve_overlay_placement(tree: &DispatchTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement) -> (f32, f32) {
    let anchor_rect = match anchor {
        OverlayAnchor::Element(element) => tree.element_node(element).and_then(|id| node_bounds(tree, id)).unwrap_or(Bounds::new(0.0, 0.0, 0.0, 0.0)),
        OverlayAnchor::Point { x, y } => Bounds::new(x, y, 0.0, 0.0),
    };
    let (content_w, content_h) = content_size;
    let (viewport_w, viewport_h) = viewport;
    match placement {
        OverlayPlacement::BelowAnchorWithFlip => {
            let below_y = anchor_rect.y + anchor_rect.h;
            let fits_below = below_y + content_h <= viewport_h;
            let y = if fits_below { below_y } else { (anchor_rect.y - content_h).max(0.0) };
            let x = anchor_rect.x.clamp(0.0, (viewport_w - content_w).max(0.0));
            (x, y)
        }
        OverlayPlacement::AtPointer { offset_x, offset_y } => {
            let x = (anchor_rect.x + offset_x).clamp(0.0, (viewport_w - content_w).max(0.0));
            let y = (anchor_rect.y + offset_y).clamp(0.0, (viewport_h - content_h).max(0.0));
            (x, y)
        }
        OverlayPlacement::Centered => (((viewport_w - content_w) / 2.0).max(0.0), ((viewport_h - content_h) / 2.0).max(0.0)),
    }
}

//#endregion 🪟️Overlay

//#region 🖱️Scroll

/// 🖱️ Ported from `events.rs::nearest_scrollable_ancestor` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn nearest_scrollable_ancestor(tree: &DispatchTree, from: FrameNodeId) -> Option<FrameNodeId> {
    let mut found = None;
    bubble(tree, from, |id| {
        if tree.node(id).is_some_and(|node| node.flags.contains(DispatchFlags::SCROLLABLE)) {
            found = Some(id);
            true
        } else {
            false
        }
    });
    found
}

//#endregion 🖱️Scroll

//#region ✍️EditRouting

/// ✍️ Ported from `events.rs`'s `tree::EditState` shape, minus `scroll_x` — caret-into-view horizontal
/// scroll is a paint-time concern belonging to whichever `Element` renders the caret (it consults
/// [`Dispatcher::edit_state`] and its own retained scroll position), never dispatch's own job now that
/// there is no paint step in this module.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EditState {
    pub text: String,
    pub caret: usize,
    pub anchor: usize,
    pub composition: Option<String>,
}

/// ✍️ Ported from `events.rs::prev_char_boundary` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn prev_char_boundary(text: &str, index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    let mut candidate = index - 1;
    while candidate > 0 && !text.is_char_boundary(candidate) {
        candidate -= 1;
    }
    candidate
}

/// ✍️ Ported from `events.rs::next_char_boundary` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn next_char_boundary(text: &str, index: usize) -> usize {
    if index >= text.len() {
        return text.len();
    }
    let mut candidate = index + 1;
    while candidate < text.len() && !text.is_char_boundary(candidate) {
        candidate += 1;
    }
    candidate
}

/// ↔️ Ported from `events.rs::selection_bounds` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
    (anchor.min(caret), anchor.max(caret))
}

/// ✍️ Ported from `events.rs::insert_at_caret` verbatim.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn insert_at_caret(edit: &mut EditState, text: &str) {
    let (start, end) = selection_bounds(edit.anchor, edit.caret);
    edit.text.replace_range(start..end, text);
    let caret = start + text.len();
    edit.caret = caret;
    edit.anchor = caret;
}

//#endregion ✍️EditRouting

//#endregion 🔖️Propagation

//#region 🔖️Focus

/// 🎯️ Ported from `events.rs::collect_focusable`, generalized from matching `is_focusable(&UiNode)`'s
/// closed variant list to the generic [`DispatchFlags::FOCUSABLE`] flag — see this file's module
/// docstring for why that generalization is sound (an element declares its own focusability once,
/// dispatch never needs to know its product type).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn collect_focusable(tree: &DispatchTree, id: FrameNodeId, out: &mut Vec<ElementId>) {
    if out.len() >= 64 {
        return;
    }
    if let Some(node) = tree.node(id) {
        if node.flags.contains(DispatchFlags::FOCUSABLE) {
            out.push(node.element);
        }
        for &child in tree.children(id) {
            collect_focusable(tree, child, out);
        }
    }
}

//#endregion 🔖️Focus

//#region 🔖️Drag

/// 🏷️ Ported from `events.rs::DragPayload` verbatim.
pub type DragPayload = HashMap<String, String>;

/// 👻️ Ported from `events.rs::DragGhost` verbatim. Per master.md: the ghost is painted into the
/// *next* frame's overlay stream, never the current one — [`Dispatcher::drag_session`] is how a host's
/// paint pass picks it up on its next call.
#[derive(Clone, Debug, PartialEq)]
pub struct DragGhost {
    pub label: String,
    pub offset_x: f32,
    pub offset_y: f32,
}

/// 🫳️ Ported from `events.rs::DragSession`, `source`/`drop_target` swapped to [`ElementId`] — a drag
/// spans many frames, so it must be addressed by the identity that survives a rebuild.
#[derive(Clone, Debug, PartialEq)]
pub struct DragSession {
    pub source: ElementId,
    pub payload: DragPayload,
    pub ghost: Option<DragGhost>,
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub drop_target: Option<ElementId>,
}

/// 📏️ Ported from `events.rs::DRAG_PROMOTE_THRESHOLD_SQ` verbatim.
const DRAG_PROMOTE_THRESHOLD_SQ: f32 = 16.0;

/// 🎯️ A [`Dispatcher::set_drop_accept`] predicate — `dyn Fn` is U3-permitted (not a first-party
/// trait).
type DropAcceptPredicate = Box<dyn Fn(&DragPayload) -> bool>;

/// 🧬️ Converts a [`DragPayload`] into the neutral [`UiValue`] a `Trigger::Drop` binding's `input`
/// carries — the generic replacement for `events.rs`'s dedicated `UiCommand::DropCommitted` variant
/// (see this file's report for why: `Trigger::Drop` already exists in the contract's closed set, so a
/// drop commit is just an ordinary fired binding like any other, no special command needed).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn drag_payload_to_value(payload: &DragPayload) -> Option<UiValue> {
    if payload.is_empty() {
        return Some(UiValue::Map(ui_contract::UiMap::default()));
    }
    let mut entries = payload.iter().collect::<Vec<_>>();
    entries.sort_unstable_by(|left, right| left.0.cmp(right.0));
    let mut builder = ui_contract::UiMapBuilder::try_new()?;
    for (key, value) in entries {
        let value = UiValue::Text(UiText::try_from_str(value)?);
        builder.push(key.clone(), value).ok()?;
    }
    Some(UiValue::Map(builder.finish()))
}

//#endregion 🔖️Drag

//#region 🔖️Outcome

/// 🖱️ What cursor a host should show for the pointer that produced this [`DispatchOutcome`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorRequest {
    #[default]
    Default,
    Pointer,
    Text,
    Grab,
    Grabbing,
}

/// 🈶️ What a host should do with the platform IME this call — `Enable` when focus just landed on an
/// [`DispatchFlags::EDITABLE`] node (positioned at the caret's bounds), `Disable` when focus just left
/// one. Composition text itself still flows in via [`DispatchEvent::Ime`] and out via whatever the
/// focused element paints from [`Dispatcher::edit_state`] next frame — this directive is only the
/// platform on/off + position signal, decoupled from that text per `frame.rs::ImeSnapshot`'s own split.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImeDirective {
    Enable { cursor_bounds: Bounds },
    Disable,
}

/// 📤️ What one [`Dispatcher::dispatch`] call produced — replaces async handlers entirely (ruling U1):
/// a listener runs synchronously, mutates only [`Dispatcher`]'s own persistent state, and this is what
/// it *returns*. `capture` reports the pointer's capture state as of the end of this call (for a host
/// that wants to mirror it into an OS-level pointer-capture call); `intents` are the fired
/// [`UiIntent`]s a runtime should apply.
pub struct DispatchOutcome {
    pub handled: bool,
    pub intents: Vec<UiIntent>,
    pub cursor: CursorRequest,
    pub invalidation: InvalidationReason,
    pub capture: Option<(HitboxId, CaptureKind)>,
    pub ime: Option<ImeDirective>,
}

impl Default for DispatchOutcome {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn default() -> Self {
        Self { handled: false, intents: Vec::new(), cursor: CursorRequest::default(), invalidation: InvalidationReason::NONE, capture: None, ime: None }
    }
}

/// 🧭️ Persistent, cross-frame owner of capture/focus/hover/overlay/drag/scroll/edit state — the
/// [`ElementId`]-keyed replacement for `events.rs::EventRouter`. A host constructs one per window and
/// keeps it alongside its `FrameEngine`; every real input event calls [`Self::dispatch`] against the
/// window's currently *presented* [`DispatchTree`] (never a mid-build one — ruling U1's own point).
#[derive(Default)]
pub struct Dispatcher {
    capture: HashMap<PointerId, CaptureEntry>,
    press_origin: HashMap<PointerId, (f32, f32)>,
    focus: Option<ElementId>,
    hovered: Option<ElementId>,
    hover_chain: Vec<ElementId>,
    overlays: OverlayStack,
    drag: Option<DragSession>,
    drag_payloads: HashMap<ElementId, DragPayload>,
    drop_accept: HashMap<ElementId, DropAcceptPredicate>,
    scroll_thumbs: HashMap<ElementId, (ElementId, ScrollAxis)>,
    thumb_start: HashMap<PointerId, (f32, f32, f32, f32)>,
    scroll_offsets: HashMap<ElementId, (f32, f32)>,
    edit_states: HashMap<ElementId, EditState>,
    seq: u64,
}

impl Dispatcher {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        let mut dispatcher = Self { hover_chain: Vec::with_capacity(64), ..Self::default() };
        dispatcher.overlays.open = Vec::with_capacity(16);
        dispatcher
    }

    //#region 🔎️Accessors
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn focused(&self) -> Option<ElementId> {
        self.focus
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_hovered(&self, element: ElementId) -> bool {
        self.hover_chain.contains(&element)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn is_captured(&self, element: ElementId) -> bool {
        self.capture.values().any(|entry| entry.element == element)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn capture_of(&self, pointer: PointerId) -> Option<(ElementId, CaptureKind)> {
        self.capture.get(&pointer).map(|entry| (entry.element, entry.kind))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn drag_session(&self) -> Option<&DragSession> {
        self.drag.as_ref()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn edit_state(&self, element: ElementId) -> Option<&EditState> {
        self.edit_states.get(&element)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn scroll_offset(&self, element: ElementId) -> (f32, f32) {
        self.scroll_offsets.get(&element).copied().unwrap_or((0.0, 0.0))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn open_overlays(&self) -> &[OpenOverlay] {
        &self.overlays.open
    }
    //#endregion 🔎️Accessors

    //#region 🧩️Registration
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn set_drag_payload(&mut self, element: ElementId, payload: DragPayload) -> bool {
        if payload.len() > 16 || payload.iter().any(|(key, value)| key.len() > 256 || value.len() > 256) || (!self.drag_payloads.contains_key(&element) && self.drag_payloads.len() >= 256) {
            return false;
        }
        self.drag_payloads.insert(element, payload);
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn set_drop_accept(&mut self, element: ElementId, predicate: impl Fn(&DragPayload) -> bool + 'static) -> bool {
        if !self.drop_accept.contains_key(&element) && self.drop_accept.len() >= 256 {
            return false;
        }
        self.drop_accept.insert(element, Box::new(predicate));
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn register_scroll_thumb(&mut self, thumb: ElementId, scrollable: ElementId, axis: ScrollAxis) -> bool {
        if !self.scroll_thumbs.contains_key(&thumb) && self.scroll_thumbs.len() >= 256 {
            return false;
        }
        self.scroll_thumbs.insert(thumb, (scrollable, axis));
        true
    }
    //#endregion 🧩️Registration

    //#region 🔦️FocusApi
    /// 🎯️ Single choke point for every focus change (pointer-down onto a focusable node, Tab/Shift+Tab,
    /// an overlay closing out from under the focused node) — ported from `events.rs::FocusState::set_focus`'s
    /// role as the one place `EditState` lifecycle is owned: blurring drops the old element's edit
    /// buffer, focusing an [`DispatchFlags::EDITABLE`] element for the first time since its last blur
    /// seeds one from [`ListenerSet::value`] with the caret at the end.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_focus_transition(&mut self, tree: &DispatchTree, target: Option<ElementId>) -> bool {
        if self.focus == target {
            return false;
        }
        if let Some(previous) = self.focus {
            self.edit_states.remove(&previous);
        }
        if let Some(next) = target {
            self.edit_states.entry(next).or_insert_with(|| {
                let value = tree.element_node(next).and_then(|id| tree.node(id)).and_then(|node| node.listeners.value.as_ref());
                let text = match value {
                    Some(UiValue::Text(text)) => text.to_string(),
                    _ => String::new(),
                };
                let caret = text.len();
                EditState { text, caret, anchor: caret, composition: None }
            });
        }
        self.focus = target;
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn clear_focus(&mut self, tree: &DispatchTree) -> bool {
        self.apply_focus_transition(tree, None)
    }

    /// 🎯️ Ported from `events.rs::FocusState::focus_next` verbatim, `NodeId` swapped for `ElementId`
    /// and the tab order rebuilt from [`collect_focusable`] scoped to `scope` (the topmost focus-trap
    /// overlay's root, or the tree root — see `Self::dispatch`'s `Tab` handling).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn focus_next(&mut self, tree: &DispatchTree, scope: FrameNodeId) -> bool {
        let mut order = Vec::with_capacity(64);
        collect_focusable(tree, scope, &mut order);
        if order.is_empty() {
            return self.apply_focus_transition(tree, None);
        }
        let next = match self.focus.and_then(|current| order.iter().position(|&candidate| candidate == current)) {
            Some(index) => (index + 1) % order.len(),
            None => 0,
        };
        self.apply_focus_transition(tree, Some(order[next]))
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn focus_prev(&mut self, tree: &DispatchTree, scope: FrameNodeId) -> bool {
        let mut order = Vec::with_capacity(64);
        collect_focusable(tree, scope, &mut order);
        if order.is_empty() {
            return self.apply_focus_transition(tree, None);
        }
        let previous = match self.focus.and_then(|current| order.iter().position(|&candidate| candidate == current)) {
            Some(index) => (index + order.len() - 1) % order.len(),
            None => order.len() - 1,
        };
        self.apply_focus_transition(tree, Some(order[previous]))
    }
    //#endregion 🔦️FocusApi

    //#region 🪟️OverlayApi
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn open_overlay(&mut self, root: ElementId, kind: OverlayKind, anchor: OverlayAnchor) {
        let focus_trap = matches!(kind, OverlayKind::Dialog | OverlayKind::CommandPalette);
        self.overlays.open(OpenOverlay { root, kind, anchor, placement: kind.default_placement(), dismiss: kind.dismiss_policy(), focus_trap });
    }

    /// 🧹️ Ported from `events.rs::EventRouter::finish_close`: clears focus too if it was inside the
    /// closed overlay's subtree, so a dangling focus into a now-hidden subtree can't route key events
    /// nowhere useful.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn finish_close(&mut self, tree: &DispatchTree, overlay: OpenOverlay) {
        if let Some(focused) = self.focus {
            let inside = tree.element_node(focused).is_some_and(|focused_node| tree.element_node(overlay.root).is_some_and(|root_node| is_descendant(tree, focused_node, root_node)));
            if inside {
                self.clear_focus(tree);
            }
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn close_overlay(&mut self, tree: &DispatchTree, root: ElementId) -> bool {
        match self.overlays.close_root(root) {
            Some(overlay) => {
                self.finish_close(tree, overlay);
                true
            }
            None => false,
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn close_topmost_overlay(&mut self, tree: &DispatchTree) -> bool {
        match self.overlays.close_topmost() {
            Some(overlay) => {
                self.finish_close(tree, overlay);
                true
            }
            None => false,
        }
    }

    /// 🔽️ Generic replacement for `events.rs::toggle_select_popup`: activating a
    /// [`DispatchFlags::OVERLAY_TRIGGER`] node toggles an overlay whose root *and* anchor are that same
    /// node (a `Select`'s own synthesized item rows are already its children, so flagging the trigger
    /// itself `OVERLAY` gives the whole popup hit-test priority over its later-painted siblings).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn toggle_overlay_trigger(&mut self, tree: &DispatchTree, element: ElementId) {
        if let Some(overlay) = self.overlays.close_root(element) {
            self.finish_close(tree, overlay);
        } else {
            self.open_overlay(element, OverlayKind::SelectPopup, OverlayAnchor::Element(element));
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn dismiss_topmost_if_outside_press(&mut self, tree: &DispatchTree, x: f32, y: f32) -> bool {
        let Some(top) = self.overlays.topmost().copied() else { return false };
        if !top.dismiss.outside_press_swallow {
            return false;
        }
        let Some(root_node) = tree.element_node(top.root) else { return false };
        if hit_test_subtree(tree, root_node, x, y).is_some() {
            return false;
        }
        self.overlays.close_root(top.root);
        self.finish_close(tree, top);
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn maybe_dismiss_tooltip_on_hover_out(&mut self, tree: &DispatchTree, x: f32, y: f32) -> bool {
        let Some(top) = self.overlays.topmost().copied() else { return false };
        if top.kind != OverlayKind::Tooltip {
            return false;
        }
        let inside_overlay = tree.element_node(top.root).and_then(|id| node_bounds(tree, id)).is_some_and(|bounds| rect_contains(bounds, x, y));
        let inside_anchor = match top.anchor {
            OverlayAnchor::Element(element) => tree.element_node(element).and_then(|id| node_bounds(tree, id)).is_some_and(|bounds| rect_contains(bounds, x, y)),
            OverlayAnchor::Point { .. } => false,
        };
        if inside_overlay || inside_anchor {
            return false;
        }
        self.overlays.close_root(top.root);
        self.finish_close(tree, top);
        true
    }
    //#endregion 🪟️OverlayApi

    //#region 🫳️DragApi
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn maybe_promote_to_drag(&mut self, pointer: PointerId, x: f32, y: f32) {
        let Some(entry) = self.capture.get(&pointer).copied() else { return };
        if entry.kind != CaptureKind::Press {
            return;
        }
        let Some(payload) = self.drag_payloads.get(&entry.element).cloned() else { return };
        let Some((origin_x, origin_y)) = self.press_origin.get(&pointer).copied() else { return };
        if (x - origin_x).powi(2) + (y - origin_y).powi(2) < DRAG_PROMOTE_THRESHOLD_SQ {
            return;
        }
        self.capture.insert(pointer, CaptureEntry { element: entry.element, kind: CaptureKind::Drag, revision: entry.revision });
        self.drag = Some(DragSession { source: entry.element, payload, ghost: None, pointer_x: x, pointer_y: y, drop_target: None });
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn update_drag(&mut self, tree: &DispatchTree, root: FrameNodeId, x: f32, y: f32) {
        if let Some(drag) = self.drag.as_mut() {
            drag.pointer_x = x;
            drag.pointer_y = y;
        }
        let target = hit_test(tree, root, x, y).and_then(|hit| self.nearest_accepting_drop_target(tree, hit));
        let element = target.and_then(|id| tree.node(id)).map(|node| node.element);
        if let Some(drag) = self.drag.as_mut() {
            drag.drop_target = element;
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn nearest_accepting_drop_target(&self, tree: &DispatchTree, from: FrameNodeId) -> Option<FrameNodeId> {
        let mut found = None;
        bubble(tree, from, |id| {
            let Some(node) = tree.node(id) else { return false };
            if !node.flags.contains(DispatchFlags::DROP_TARGET) {
                return false;
            }
            let accepts = match self.drop_accept.get(&node.element) {
                Some(predicate) => self.drag.as_ref().is_some_and(|drag| predicate(&drag.payload)),
                None => true,
            };
            if accepts {
                found = Some(id);
                true
            } else {
                false
            }
        });
        found
    }

    /// 👻️ Attaches a ghost to the active drag session — a host/widget calls this once a session is
    /// promoted; see [`DragGhost`]'s own doc for why painting it is next-frame work, not this call's.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn set_drag_ghost(&mut self, ghost: DragGhost) {
        if let Some(drag) = self.drag.as_mut() {
            drag.ghost = Some(ghost);
        }
    }
    //#endregion 🫳️DragApi

    //#region 🖱️ScrollApi
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn route_scroll(&mut self, tree: &DispatchTree, root: FrameNodeId, x: f32, y: f32, delta_x: f32, delta_y: f32) -> bool {
        let Some(hit) = hit_test(tree, root, x, y) else { return false };
        let Some(scrollable) = nearest_scrollable_ancestor(tree, hit) else { return false };
        let Some(element) = tree.node(scrollable).map(|node| node.element) else { return false };
        if !self.scroll_offsets.contains_key(&element) && self.scroll_offsets.len() >= 256 {
            return false;
        }
        let (offset_x, offset_y) = self.scroll_offset(element);
        self.scroll_offsets.insert(element, ((offset_x + delta_x).max(0.0), (offset_y + delta_y).max(0.0)));
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn update_scroll_thumb(&mut self, scrollable: ElementId, axis: ScrollAxis, pointer: PointerId, x: f32, y: f32) {
        let Some((origin_x, origin_y, start_x, start_y)) = self.thumb_start.get(&pointer).copied() else { return };
        let (delta_x, delta_y) = (x - origin_x, y - origin_y);
        let offset = match axis {
            ScrollAxis::Horizontal => ((start_x + delta_x).max(0.0), start_y),
            ScrollAxis::Vertical => (start_x, (start_y + delta_y).max(0.0)),
        };
        self.scroll_offsets.insert(scrollable, offset);
    }
    //#endregion 🖱️ScrollApi

    //#region ✍️EditApi
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn route_text_insert(&mut self, text: &str) -> bool {
        let Some(id) = self.focus else { return false };
        let Some(edit) = self.edit_states.get_mut(&id) else { return false };
        insert_at_caret(edit, text);
        true
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn route_ime(&mut self, event: &ImeEvent) -> bool {
        let Some(id) = self.focus else { return false };
        let Some(edit) = self.edit_states.get_mut(&id) else { return false };
        match event {
            ImeEvent::Start => edit.composition = Some(String::new()),
            ImeEvent::Update { text, .. } => edit.composition = Some(text.clone()),
            ImeEvent::Commit { text } => {
                edit.composition = None;
                insert_at_caret(edit, text);
            }
            ImeEvent::Cancel => edit.composition = None,
        }
        true
    }

    /// ⌨️ Ported from `events.rs::route_edit_key` verbatim for caret motion/`Home`/`End`/`Backspace`/
    /// `Delete`; the clipboard shortcuts synthesize a `UiIntent` against a well-known dispatch-owned
    /// [`ActionId`] (`scope: "dispatch"`) rather than a bespoke `UiCommand` variant, since
    /// `DispatchOutcome` has no side-channel beyond `intents` — see this file's report for the
    /// trade-off. A no-operation if nothing is focused or the focused element has no [`EditState`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn route_edit_key(&mut self, key: &str, modifiers: EventModifiers) -> (bool, Option<UiIntent>) {
        let Some(id) = self.focus else { return (false, None) };
        let Some(edit) = self.edit_states.get_mut(&id) else { return (false, None) };
        let has_selection = edit.anchor != edit.caret;
        match key {
            "ArrowLeft" => {
                edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).0 } else { prev_char_boundary(&edit.text, edit.caret) };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
                }
            }
            "ArrowRight" => {
                edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).1 } else { next_char_boundary(&edit.text, edit.caret) };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
                }
            }
            "Home" => {
                edit.caret = 0;
                if !modifiers.shift {
                    edit.anchor = 0;
                }
            }
            "End" => {
                edit.caret = edit.text.len();
                if !modifiers.shift {
                    edit.anchor = edit.text.len();
                }
            }
            "Backspace" => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                } else if edit.caret > 0 {
                    let start = prev_char_boundary(&edit.text, edit.caret);
                    edit.text.replace_range(start..edit.caret, "");
                    edit.caret = start;
                    edit.anchor = start;
                }
            }
            "Delete" => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                } else if edit.caret < edit.text.len() {
                    let end = next_char_boundary(&edit.text, edit.caret);
                    edit.text.replace_range(edit.caret..end, "");
                }
            }
            "c" | "C" if modifiers.ctrl || modifiers.meta => {
                if !has_selection {
                    return (false, None);
                }
                let (start, end) = selection_bounds(edit.anchor, edit.caret);
                let text = edit.text[start..end].to_string();
                return (true, self.build_clipboard_intent("clipboardCopy", text));
            }
            "x" | "X" if modifiers.ctrl || modifiers.meta => {
                if !has_selection {
                    return (false, None);
                }
                let (start, end) = selection_bounds(edit.anchor, edit.caret);
                let text = edit.text[start..end].to_string();
                edit.text.replace_range(start..end, "");
                edit.caret = start;
                edit.anchor = start;
                return (true, self.build_clipboard_intent("clipboardCut", text));
            }
            "v" | "V" if modifiers.ctrl || modifiers.meta => {
                return (true, self.build_clipboard_intent("clipboardPasteRequested", String::new()));
            }
            _ => return (false, None),
        }
        (true, None)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn build_clipboard_intent(&mut self, name: &str, text: String) -> Option<UiIntent> {
        let action = ActionId::try_v1("dispatch", name)?;
        let input = (!text.is_empty()).then(|| UiText::try_from_string(text).ok()).flatten().map(UiValue::Text);
        self.seq += 1;
        Some(UiIntent {
            surface: SurfaceId::default(),
            revision: UiRevision::default(),
            node: UiNodeId::default(),
            node_key: UiText::default(),
            trigger: Trigger::Commit,
            action,
            args: None,
            input,
            seq: self.seq,
        })
    }
    //#endregion ✍️EditApi

    //#region 🎬️FireApi
    /// 🎬️ Fires `id`'s binding for `trigger`, if any — building the [`UiIntent`] from the node's own
    /// [`ListenerSet`] addressing. `None` if there's no such binding, or if the listener's own captured
    /// revision is already stale against the tree it's being resolved in (see [`is_stale`]).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn fire(&mut self, tree: &DispatchTree, id: FrameNodeId, trigger: Trigger, input: Option<UiValue>) -> Option<UiIntent> {
        let node = tree.node(id)?;
        let binding = node.listeners.binding_for(trigger)?;
        if is_stale(node.listeners.revision, tree.revision()) {
            return None;
        }
        let args = match binding.args.as_ref() {
            Some(args) => Some(args.credited_clone()?),
            None => None,
        };
        self.seq += 1;
        Some(UiIntent {
            surface: node.listeners.surface.clone(),
            revision: node.listeners.revision,
            node: node.listeners.node,
            node_key: node.listeners.node_key.clone(),
            trigger,
            action: binding.action.clone(),
            args,
            input,
            seq: self.seq,
        })
    }

    /// 🎬️ Fires a binding on behalf of an already-captured node, additionally rejecting it if the
    /// *capture's own* recorded revision (from when it started, possibly frames ago) is now stale
    /// against the current tree — the case [`fire`] alone cannot catch, since `id`'s own
    /// [`ListenerSet::revision`] is always in sync with the tree that produced it, never with how long
    /// ago the capture began. This is the ticket's "a stale interaction is rejected rather than
    /// misapplied" — see the `stale_captured_activate_is_rejected...` test.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn fire_captured(&mut self, tree: &DispatchTree, id: FrameNodeId, entry: CaptureEntry, trigger: Trigger, input: Option<UiValue>) -> Option<UiIntent> {
        if is_stale(entry.revision, tree.revision()) {
            return None;
        }
        self.fire(tree, id, trigger, input)
    }
    //#endregion 🎬️FireApi

    //#region 🖱️CursorApi
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn cursor_for(&self, tree: &DispatchTree, target: Option<FrameNodeId>) -> CursorRequest {
        if self.drag.is_some() {
            return CursorRequest::Grabbing;
        }
        let Some(node) = target.and_then(|id| tree.node(id)) else { return CursorRequest::Default };
        if node.flags.contains(DispatchFlags::DRAG_SOURCE) {
            return CursorRequest::Grab;
        }
        if node.flags.contains(DispatchFlags::EDITABLE) {
            return CursorRequest::Text;
        }
        if !node.listeners.bindings.is_empty() || node.flags.contains(DispatchFlags::FOCUSABLE) {
            return CursorRequest::Pointer;
        }
        CursorRequest::Default
    }
    //#endregion 🖱️CursorApi

    /// 🚦️ Ported from `events.rs::EventRouter::dispatch`: resolves the event's target (capture target
    /// if captured, else [`hit_test`]), updates interaction state, and returns the resulting
    /// [`DispatchOutcome`]. Always called against the *presented* [`DispatchTree`] — see this file's
    /// module docstring.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn dispatch(&mut self, tree: &DispatchTree, event: &DispatchEvent) -> DispatchOutcome {
        let Some(root) = tree.root() else { return DispatchOutcome::default() };
        let mut outcome = DispatchOutcome::default();

        match event {
            DispatchEvent::PointerMove { pointer, x, y } => {
                self.maybe_promote_to_drag(pointer.id, *x, *y);
                match self.capture.get(&pointer.id).copied() {
                    Some(entry) if entry.kind == CaptureKind::Drag => self.update_drag(tree, root, *x, *y),
                    Some(entry) => {
                        if let CaptureKind::ScrollThumb(axis) = entry.kind {
                            self.update_scroll_thumb(entry.element, axis, pointer.id, *x, *y);
                            outcome.invalidation.insert(InvalidationReason::LAYOUT);
                        }
                    }
                    None => {}
                }
                let target = self.resolve_target(tree, root, pointer.id, *x, *y);
                if self.update_hover(tree, target) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                if self.maybe_dismiss_tooltip_on_hover_out(tree, *x, *y) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                outcome.handled = target.is_some();
                outcome.cursor = self.cursor_for(tree, target);
            }
            DispatchEvent::PointerDown { pointer, x, y, .. } => {
                if !self.press_origin.contains_key(&pointer.id) && self.press_origin.len() >= 16 {
                    return outcome;
                }
                if self.dismiss_topmost_if_outside_press(tree, *x, *y) {
                    outcome.handled = true;
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                    return outcome;
                }
                self.press_origin.insert(pointer.id, (*x, *y));
                let target = hit_test(tree, root, *x, *y);
                if self.update_hover(tree, target) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                match target.and_then(|id| tree.node(id).map(|node| (id, node))) {
                    Some((id, node)) => {
                        outcome.handled = true;
                        if let Some(&(scrollable, axis)) = self.scroll_thumbs.get(&node.element) {
                            self.capture.insert(pointer.id, CaptureEntry { element: scrollable, kind: CaptureKind::ScrollThumb(axis), revision: tree.revision() });
                            let (offset_x, offset_y) = self.scroll_offset(scrollable);
                            self.thumb_start.insert(pointer.id, (*x, *y, offset_x, offset_y));
                        } else {
                            self.capture.insert(pointer.id, CaptureEntry { element: node.element, kind: CaptureKind::Press, revision: tree.revision() });
                            if node.flags.contains(DispatchFlags::FOCUSABLE) {
                                self.apply_focus_transition(tree, Some(node.element));
                                outcome.invalidation.insert(InvalidationReason::PAINT);
                                if node.flags.contains(DispatchFlags::EDITABLE) {
                                    let cursor_bounds = node_bounds(tree, id).unwrap_or(Bounds::new(0.0, 0.0, 0.0, 0.0));
                                    outcome.ime = Some(ImeDirective::Enable { cursor_bounds });
                                }
                            }
                        }
                    }
                    None => {
                        if self.clear_focus(tree) {
                            outcome.invalidation.insert(InvalidationReason::PAINT);
                            outcome.ime = Some(ImeDirective::Disable);
                        }
                    }
                }
                outcome.cursor = self.cursor_for(tree, target);
            }
            DispatchEvent::PointerUp { pointer, x, y, .. } => {
                self.press_origin.remove(&pointer.id);
                if let Some(entry) = self.capture.remove(&pointer.id) {
                    outcome.handled = true;
                    match entry.kind {
                        CaptureKind::Press => {
                            if hit_test(tree, root, *x, *y) == tree.element_node(entry.element) {
                                if let Some(id) = tree.element_node(entry.element) {
                                    let node = tree.node(id).expect("resolved from tree.element_node");
                                    if node.flags.contains(DispatchFlags::OVERLAY_TRIGGER) {
                                        self.toggle_overlay_trigger(tree, entry.element);
                                        outcome.invalidation.insert(InvalidationReason::PAINT);
                                    } else if let Some(intent) = self.fire_captured(tree, id, entry, Trigger::Activate, None) {
                                        let parent_overlay = self.overlays.topmost().copied().filter(|overlay| overlay.kind == OverlayKind::SelectPopup && is_descendant(tree, id, tree.element_node(overlay.root).unwrap_or(id)));
                                        outcome.intents.push(intent);
                                        outcome.invalidation.insert(InvalidationReason::STRUCTURE);
                                        if let Some(overlay) = parent_overlay {
                                            self.overlays.close_root(overlay.root);
                                            self.finish_close(tree, overlay);
                                        }
                                    }
                                }
                            }
                        }
                        CaptureKind::Drag => {
                            if let Some(drag) = self.drag.take() {
                                outcome.invalidation.insert(InvalidationReason::PAINT);
                                if let Some(target) = drag.drop_target {
                                    if let Some(target_id) = tree.element_node(target) {
                                        if let Some(value) = drag_payload_to_value(&drag.payload) {
                                            if let Some(intent) = self.fire(tree, target_id, Trigger::Drop, Some(value)) {
                                                outcome.intents.push(intent);
                                                outcome.invalidation.insert(InvalidationReason::STRUCTURE);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        CaptureKind::ScrollThumb(_) => {
                            self.thumb_start.remove(&pointer.id);
                        }
                    }
                }
                let target = self.resolve_target(tree, root, pointer.id, *x, *y);
                if self.update_hover(tree, target) {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
                outcome.cursor = self.cursor_for(tree, target);
            }
            DispatchEvent::KeyDown { key, modifiers } => {
                if key == "Escape" {
                    if self.close_topmost_overlay(tree) {
                        outcome.handled = true;
                        outcome.invalidation.insert(InvalidationReason::PAINT);
                    }
                } else if key == "Tab" {
                    let scope_element = self.overlays.topmost_focus_trap_root();
                    let scope = scope_element.and_then(|element| tree.element_node(element)).unwrap_or(root);
                    let changed = if modifiers.shift { self.focus_prev(tree, scope) } else { self.focus_next(tree, scope) };
                    outcome.handled = true;
                    if changed {
                        outcome.invalidation.insert(InvalidationReason::PAINT);
                    }
                } else {
                    let (changed, intent) = self.route_edit_key(key, *modifiers);
                    outcome.handled = changed;
                    if changed {
                        outcome.invalidation.insert(InvalidationReason::PAINT);
                    }
                    if let Some(intent) = intent {
                        outcome.intents.push(intent);
                    }
                }
            }
            DispatchEvent::KeyUp { .. } => {}
            DispatchEvent::TextInput { text } => {
                outcome.handled = self.route_text_insert(text);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
            }
            DispatchEvent::Paste { text } => {
                outcome.handled = self.route_text_insert(text);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
            }
            DispatchEvent::TextEditStart { .. } | DispatchEvent::TextEditChunk { .. } | DispatchEvent::TextEditCommit { .. } | DispatchEvent::TextEditAbort { .. } => {}
            DispatchEvent::Ime(ime_event) => {
                outcome.handled = self.route_ime(ime_event);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::PAINT);
                }
            }
            DispatchEvent::Scroll { x, y, delta_x, delta_y } => {
                outcome.handled = self.route_scroll(tree, root, *x, *y, *delta_x, *delta_y);
                if outcome.handled {
                    outcome.invalidation.insert(InvalidationReason::LAYOUT);
                }
            }
        }

        if let Some(pointer) = event_pointer(event) {
            outcome.capture = self.capture.get(&pointer).and_then(|entry| {
                let hitbox = tree.element_node(entry.element).and_then(|id| tree.node(id)).and_then(|node| node.hitbox);
                hitbox.map(|hitbox| (hitbox, entry.kind))
            });
        }
        outcome
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resolve_target(&self, tree: &DispatchTree, root: FrameNodeId, pointer: PointerId, x: f32, y: f32) -> Option<FrameNodeId> {
        match self.capture.get(&pointer) {
            Some(entry) => tree.element_node(entry.element),
            None => hit_test(tree, root, x, y),
        }
    }

    /// 👆️ Ported from `events.rs::EventRouter::update_hover`: flips the hover bubble chain — every
    /// node from the hit target up to the root observes [`Dispatcher::is_hovered`], not just the leaf,
    /// so an ancestor layout container still reads as hovered for a host's reveal-on-hover styling.
    /// Returns whether anything changed.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn update_hover(&mut self, tree: &DispatchTree, target: Option<FrameNodeId>) -> bool {
        let target_element = target.and_then(|id| tree.node(id)).map(|node| node.element);
        if self.hovered == target_element {
            return false;
        }
        let mut new_chain = Vec::with_capacity(64);
        if let Some(leaf) = target {
            bubble(tree, leaf, |id| {
                if new_chain.len() >= 64 {
                    return true;
                }
                if let Some(node) = tree.node(id) {
                    new_chain.push(node.element);
                }
                false
            });
        }
        self.hover_chain = new_chain;
        self.hovered = target_element;
        true
    }
}

//#endregion 🔖️Outcome

/// 🧪️ Ports `events.rs`'s seven test regions onto [`DispatchTree`]/[`Dispatcher`] — they are this
/// port's specification (see the ticket). Built directly against hand-assembled [`DispatchTree`]s via
/// [`DispatchTree::insert`] — the same primitive [`crate::PrepaintCx::register`] itself calls — exactly
/// as `events.rs`'s own tests built a `UiTree` by hand via a `leaf()` helper. `frame.rs`'s own test
/// module covers the *other* half: that a real `build_frame` call, going through prepaint registration
/// rather than a hand-built tree, produces the same kind of structure these tests assume.
#[cfg(test)]
#[path = "../🧪️tests/🔬️dispatch-unit/🦀️.rs"]
mod tests;
