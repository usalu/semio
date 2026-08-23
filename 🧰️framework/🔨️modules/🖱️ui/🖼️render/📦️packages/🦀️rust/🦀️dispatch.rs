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
use ui_contract::{ActionBinding, ActionId, SurfaceId, Trigger, UiIntent, UiNodeId, UiRevision, UiValue};

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
#[derive(Clone, Debug, Default)]
pub struct ListenerSet {
    pub surface: SurfaceId,
    pub node: UiNodeId,
    pub node_key: String,
    pub revision: UiRevision,
    pub value: Option<UiValue>,
    pub bindings: Vec<ActionBinding>,
}

impl ListenerSet {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn binding_for(&self, trigger: Trigger) -> Option<&ActionBinding> {
        self.bindings.iter().find(|binding| binding.trigger == trigger)
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
        self.children.get(id.0 as usize).map(Vec::as_slice).unwrap_or(&[])
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
fn drag_payload_to_value(payload: &DragPayload) -> UiValue {
    UiValue::Map(payload.iter().map(|(key, value)| (key.clone(), UiValue::Text(value.clone()))).collect())
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
        let mut dispatcher = Self::default();
        dispatcher.hover_chain = Vec::with_capacity(64);
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
            if !self.edit_states.contains_key(&next) {
                let value = tree.element_node(next).and_then(|id| tree.node(id)).and_then(|node| node.listeners.value.as_ref());
                let text = match value {
                    Some(UiValue::Text(text)) if text.len() <= 16 * 1024 => text.clone(),
                    _ => String::new(),
                };
                let caret = text.len();
                self.edit_states.insert(next, EditState { text, caret, anchor: caret, composition: None });
            }
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
                return (true, Some(self.build_clipboard_intent("clipboardCopy", text)));
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
                return (true, Some(self.build_clipboard_intent("clipboardCut", text)));
            }
            "v" | "V" if modifiers.ctrl || modifiers.meta => {
                return (true, Some(self.build_clipboard_intent("clipboardPasteRequested", String::new())));
            }
            _ => return (false, None),
        }
        (true, None)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn build_clipboard_intent(&mut self, name: &str, text: String) -> UiIntent {
        self.seq += 1;
        UiIntent {
            surface: SurfaceId::default(),
            revision: UiRevision::default(),
            node: UiNodeId::default(),
            node_key: String::new(),
            trigger: Trigger::Commit,
            action: ActionId::v1("dispatch", name),
            args: None,
            input: (!text.is_empty()).then_some(UiValue::Text(text)),
            seq: self.seq,
        }
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
        self.seq += 1;
        Some(UiIntent {
            surface: node.listeners.surface.clone(),
            revision: node.listeners.revision,
            node: node.listeners.node,
            node_key: node.listeners.node_key.clone(),
            trigger,
            action: binding.action.clone(),
            args: binding.args.clone(),
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
                                        if let Some(intent) = self.fire(tree, target_id, Trigger::Drop, Some(drag_payload_to_value(&drag.payload))) {
                                            outcome.intents.push(intent);
                                            outcome.invalidation.insert(InvalidationReason::STRUCTURE);
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
mod tests {
    use super::*;
    use crate::element::ReconciliationKey;

    fn leaf(tree: &mut DispatchTree, parent: Option<FrameNodeId>, key: &str, flags: DispatchFlags, listeners: ListenerSet, rect: (f32, f32, f32, f32)) -> FrameNodeId {
        let parent_element = parent.and_then(|id| tree.node(id)).map(|node| node.element);
        let element = ElementId::new(parent_element, &ReconciliationKey::Explicit(key.to_string()));
        let bounds = Bounds::new(rect.0, rect.1, rect.2, rect.3);
        let hitbox = Hitbox { element, bounds, clips_children: flags.contains(DispatchFlags::CLIPS_CHILDREN), hit_transparent: flags.contains(DispatchFlags::HIT_TRANSPARENT) };
        tree.insert(parent, element, flags, listeners, Some(hitbox))
    }

    fn act(name: &str) -> ActionId {
        ActionId::v1("test", name)
    }

    fn bind(trigger: Trigger, name: &str) -> ActionBinding {
        ActionBinding { trigger, action: act(name), args: None, capability: None }
    }

    fn listen(bindings: Vec<ActionBinding>) -> ListenerSet {
        ListenerSet { surface: SurfaceId::from("s"), node: UiNodeId(1), node_key: "k".into(), revision: UiRevision(0), value: None, bindings }
    }

    fn listen_editable(value: &str) -> ListenerSet {
        ListenerSet { surface: SurfaceId::from("s"), node: UiNodeId(1), node_key: "k".into(), revision: UiRevision(0), value: Some(UiValue::Text(value.into())), bindings: Vec::new() }
    }

    fn ptr(id: u64) -> PointerInfo {
        PointerInfo { id: PointerId(id), kind: PointerKind::Mouse, pressure: None, tilt: None }
    }

    fn down(id: u64, x: f32, y: f32) -> DispatchEvent {
        DispatchEvent::PointerDown { pointer: ptr(id), x, y, button: PointerButton::Primary }
    }

    fn up(id: u64, x: f32, y: f32) -> DispatchEvent {
        DispatchEvent::PointerUp { pointer: ptr(id), x, y, button: PointerButton::Primary }
    }

    fn mv(id: u64, x: f32, y: f32) -> DispatchEvent {
        DispatchEvent::PointerMove { pointer: ptr(id), x, y }
    }

    #[test]
    fn hit_test_finds_the_topmost_of_two_non_overlapping_siblings() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let left = leaf(&mut tree, Some(root), "left", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        let right = leaf(&mut tree, Some(root), "right", DispatchFlags::NONE, listen(vec![]), (100.0, 0.0, 100.0, 100.0));

        assert_eq!(hit_test(&tree, root, 50.0, 50.0), Some(left));
        assert_eq!(hit_test(&tree, root, 150.0, 50.0), Some(right));
    }

    #[test]
    fn hit_test_respects_clips_children_pruning() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let clipper = leaf(&mut tree, Some(root), "clipper", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::CLIPS_CHILDREN, listen(vec![]), (0.0, 0.0, 50.0, 50.0));
        let overflowing_child = leaf(&mut tree, Some(clipper), "overflow", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 500.0, 500.0));

        assert_eq!(hit_test(&tree, root, 400.0, 400.0), None, "point outside the clipper must not match the overflowing child");
        assert_eq!(hit_test(&tree, root, 10.0, 10.0), Some(overflowing_child), "inside the clip bounds the child still matches");
    }

    #[test]
    fn hit_test_skips_hit_transparent_node_but_still_matches_its_children() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let overlay_glass = leaf(&mut tree, Some(root), "glass", DispatchFlags::HIT_TRANSPARENT, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let child = leaf(&mut tree, Some(overlay_glass), "child", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 50.0, 50.0));

        assert_eq!(hit_test(&tree, root, 30.0, 30.0), Some(child));
        assert_eq!(hit_test(&tree, root, 150.0, 150.0), None, "hit-transparent node itself must never match outside its children");
    }

    #[test]
    fn capture_routes_move_and_up_to_the_captured_node_regardless_of_pointer_position() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let a = leaf(&mut tree, Some(root), "a", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        leaf(&mut tree, Some(root), "b", DispatchFlags::NONE, listen(vec![]), (100.0, 0.0, 100.0, 100.0));
        let mut dispatcher = Dispatcher::new();
        let element_a = tree.node(a).unwrap().element;

        dispatcher.dispatch(&tree, &down(1, 50.0, 50.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), Some((element_a, CaptureKind::Press)));

        dispatcher.dispatch(&tree, &mv(1, 150.0, 50.0));
        assert!(dispatcher.is_hovered(element_a), "capture must keep resolving to `a` even once the pointer is over `b`");

        dispatcher.dispatch(&tree, &up(1, 150.0, 50.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), None, "capture releases on PointerUp");
    }

    #[test]
    fn distinct_pointer_identity_storm_saturates_at_sixteen_slots() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        let mut dispatcher = Dispatcher::new();
        for pointer in 0..16 {
            assert!(dispatcher.dispatch(&tree, &down(pointer, 10.0, 10.0)).handled);
            assert!(dispatcher.capture_of(PointerId(pointer)).is_some());
        }
        assert!(!dispatcher.dispatch(&tree, &down(16, 10.0, 10.0)).handled);
        assert!(dispatcher.capture_of(PointerId(16)).is_none());
        assert_eq!(tree.root(), Some(root));
    }

    #[test]
    fn focus_next_and_prev_cycle_only_through_focusable_nodes() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 300.0, 100.0));
        leaf(&mut tree, Some(root), "text", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 50.0, 20.0));
        let button_a = leaf(&mut tree, Some(root), "a", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "a")]), (50.0, 0.0, 50.0, 20.0));
        leaf(&mut tree, Some(root), "sep", DispatchFlags::NONE, listen(vec![]), (100.0, 0.0, 50.0, 20.0));
        let button_b = leaf(&mut tree, Some(root), "b", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "b")]), (150.0, 0.0, 50.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        let element_a = tree.node(button_a).unwrap().element;
        let element_b = tree.node(button_b).unwrap().element;
        let tab = |shift: bool| DispatchEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers { shift, ..Default::default() } };

        dispatcher.dispatch(&tree, &tab(false));
        assert_eq!(dispatcher.focused(), Some(element_a));
        dispatcher.dispatch(&tree, &tab(false));
        assert_eq!(dispatcher.focused(), Some(element_b));
        dispatcher.dispatch(&tree, &tab(false));
        assert_eq!(dispatcher.focused(), Some(element_a), "cycles back to the first focusable node");

        dispatcher.dispatch(&tree, &tab(true));
        assert_eq!(dispatcher.focused(), Some(element_b), "wraps to the last focusable node going backwards");
    }

    #[test]
    fn clicking_a_button_emits_its_action_descriptor_as_a_ui_intent() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        leaf(&mut tree, Some(root), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 100.0, 40.0));
        let mut dispatcher = Dispatcher::new();

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

        assert!(outcome.intents.iter().any(|intent| intent.action == act("go") && intent.trigger == Trigger::Activate));
    }

    #[test]
    fn releasing_off_the_captured_button_does_not_fire_its_action() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        leaf(&mut tree, Some(root), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 40.0, 40.0));
        let mut dispatcher = Dispatcher::new();

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        let outcome = dispatcher.dispatch(&tree, &up(1, 90.0, 90.0));

        assert!(outcome.intents.is_empty(), "release outside the pressed button must not fire its action");
    }

    #[test]
    fn bubble_stops_when_a_handler_returns_true() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        let mid = leaf(&mut tree, Some(root), "mid", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        let leaf_node = leaf(&mut tree, Some(mid), "leaf", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));

        let mut visited = Vec::new();
        bubble(&tree, leaf_node, |id| {
            visited.push(id);
            id == mid
        });

        assert_eq!(visited, vec![leaf_node, mid], "bubbling must stop at `mid` and never reach `root`");
    }

    //#region 🔖️OverlayTests
    #[test]
    fn overlay_open_and_close_flips_the_open_overlays_list() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let popup = leaf(&mut tree, Some(root), "popup", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
        let mut dispatcher = Dispatcher::new();
        let popup_element = tree.node(popup).unwrap().element;

        dispatcher.open_overlay(popup_element, OverlayKind::SelectPopup, OverlayAnchor::Element(popup_element));
        assert_eq!(dispatcher.open_overlays().len(), 1);
        assert_eq!(dispatcher.open_overlays()[0].kind, OverlayKind::SelectPopup);

        assert!(dispatcher.close_overlay(&tree, popup_element));
        assert!(dispatcher.open_overlays().is_empty());
    }

    #[test]
    fn pointer_down_outside_a_dismissable_overlay_closes_it_and_swallows_the_press() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "underneath", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let popup = leaf(&mut tree, Some(root), "popup", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::OVERLAY, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
        leaf(&mut tree, Some(popup), "item", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
        let mut dispatcher = Dispatcher::new();
        let popup_element = tree.node(popup).unwrap().element;
        dispatcher.open_overlay(popup_element, OverlayKind::SelectPopup, OverlayAnchor::Element(popup_element));

        let outcome = dispatcher.dispatch(&tree, &down(1, 150.0, 150.0));

        assert!(dispatcher.open_overlays().is_empty(), "outside press must close the overlay");
        assert_eq!(dispatcher.capture_of(PointerId(1)), None, "the outside press must be swallowed, not routed to whatever's underneath");
        assert!(outcome.handled);
    }

    #[test]
    fn pointer_down_inside_a_dismissable_overlay_does_not_close_it() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let popup = leaf(&mut tree, Some(root), "popup", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::OVERLAY, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
        leaf(&mut tree, Some(popup), "item", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 50.0, 50.0));
        let mut dispatcher = Dispatcher::new();
        let popup_element = tree.node(popup).unwrap().element;
        dispatcher.open_overlay(popup_element, OverlayKind::SelectPopup, OverlayAnchor::Element(popup_element));

        dispatcher.dispatch(&tree, &down(1, 20.0, 20.0));

        assert!(dispatcher.open_overlays().len() == 1, "a press inside the overlay must not dismiss it");
    }

    #[test]
    fn escape_closes_only_the_topmost_of_two_open_overlays() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let menu = leaf(&mut tree, Some(root), "menu", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 50.0, 50.0));
        let submenu = leaf(&mut tree, Some(root), "submenu", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (60.0, 0.0, 50.0, 50.0));
        let mut dispatcher = Dispatcher::new();
        let menu_element = tree.node(menu).unwrap().element;
        let submenu_element = tree.node(submenu).unwrap().element;
        dispatcher.open_overlay(menu_element, OverlayKind::ContextMenu, OverlayAnchor::Element(menu_element));
        dispatcher.open_overlay(submenu_element, OverlayKind::ContextMenu, OverlayAnchor::Element(menu_element));

        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });

        assert_eq!(dispatcher.open_overlays().len(), 1);
        assert_eq!(dispatcher.open_overlays()[0].root, menu_element, "only the topmost overlay closes on Escape");
    }

    #[test]
    fn tab_focus_is_trapped_inside_an_open_dialog_overlay() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 300.0, 300.0));
        leaf(&mut tree, Some(root), "a", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "a")]), (0.0, 0.0, 50.0, 20.0));
        leaf(&mut tree, Some(root), "b", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "b")]), (50.0, 0.0, 50.0, 20.0));
        let dialog = leaf(&mut tree, Some(root), "dialog", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (100.0, 100.0, 100.0, 100.0));
        let button_c = leaf(&mut tree, Some(dialog), "c", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "c")]), (100.0, 100.0, 50.0, 20.0));
        let button_d = leaf(&mut tree, Some(dialog), "d", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "d")]), (150.0, 100.0, 50.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        let dialog_element = tree.node(dialog).unwrap().element;
        let element_c = tree.node(button_c).unwrap().element;
        let element_d = tree.node(button_d).unwrap().element;
        dispatcher.open_overlay(dialog_element, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });

        let tab = || DispatchEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() };
        dispatcher.dispatch(&tree, &tab());
        assert_eq!(dispatcher.focused(), Some(element_c));
        dispatcher.dispatch(&tree, &tab());
        assert_eq!(dispatcher.focused(), Some(element_d));
        dispatcher.dispatch(&tree, &tab());
        assert_eq!(dispatcher.focused(), Some(element_c), "focus-trapped Tab cycling must never reach a/b outside the dialog");
    }

    #[test]
    fn closing_an_overlay_clears_focus_that_was_inside_it() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let dialog = leaf(&mut tree, Some(root), "dialog", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        let button = leaf(&mut tree, Some(dialog), "ok", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "ok")]), (0.0, 0.0, 50.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        let dialog_element = tree.node(dialog).unwrap().element;
        let button_element = tree.node(button).unwrap().element;
        dispatcher.open_overlay(dialog_element, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });
        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() });
        assert_eq!(dispatcher.focused(), Some(button_element));

        assert!(dispatcher.close_overlay(&tree, dialog_element));

        assert_eq!(dispatcher.focused(), None, "focus dangling into a closed overlay's subtree must be cleared");
    }
    //#endregion 🔖️OverlayTests

    //#region 🔖️DragDropTests
    #[test]
    fn drag_session_promotes_after_threshold_and_commits_on_an_accepting_drop_target() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let source = leaf(&mut tree, Some(root), "source", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));
        let target = leaf(&mut tree, Some(root), "target", DispatchFlags::DROP_TARGET, listen(vec![bind(Trigger::Drop, "drop")]), (100.0, 100.0, 50.0, 50.0));
        leaf(&mut tree, Some(target), "drop-here", DispatchFlags::NONE, listen(vec![]), (100.0, 100.0, 50.0, 50.0));
        let mut dispatcher = Dispatcher::new();
        let source_element = tree.node(source).unwrap().element;
        let target_element = tree.node(target).unwrap().element;
        let mut payload = DragPayload::new();
        payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\"}".into());
        dispatcher.set_drag_payload(source_element, payload.clone());

        dispatcher.dispatch(&tree, &down(1, 5.0, 5.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Press)), "a plain press must not immediately start a drag");

        dispatcher.dispatch(&tree, &mv(1, 6.0, 6.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Press)));

        dispatcher.dispatch(&tree, &mv(1, 120.0, 120.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Drag)));
        assert_eq!(dispatcher.drag_session().and_then(|drag| drag.drop_target), Some(target_element));

        let outcome = dispatcher.dispatch(&tree, &up(1, 120.0, 120.0));
        assert!(outcome.intents.iter().any(|intent| intent.trigger == Trigger::Drop && intent.action == act("drop") && intent.input == Some(drag_payload_to_value(&payload))));
        assert_eq!(dispatcher.capture_of(PointerId(1)), None);
        assert!(dispatcher.drag_session().is_none());
    }

    #[test]
    fn drag_session_cancels_when_released_over_no_accepting_drop_target() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let source = leaf(&mut tree, Some(root), "source", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        let source_element = tree.node(source).unwrap().element;
        dispatcher.set_drag_payload(source_element, DragPayload::new());

        dispatcher.dispatch(&tree, &down(1, 5.0, 5.0));
        dispatcher.dispatch(&tree, &mv(1, 190.0, 190.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), Some((source_element, CaptureKind::Drag)));

        let outcome = dispatcher.dispatch(&tree, &up(1, 190.0, 190.0));
        assert!(outcome.intents.iter().all(|intent| intent.trigger != Trigger::Drop), "no accepting target means no Drop intent fires");
        assert!(dispatcher.drag_session().is_none());
    }

    #[test]
    fn a_drop_targets_accept_predicate_can_reject_the_active_payload() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let source = leaf(&mut tree, Some(root), "source", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 20.0, 20.0));
        let target = leaf(&mut tree, Some(root), "target", DispatchFlags::DROP_TARGET, listen(vec![bind(Trigger::Drop, "drop")]), (100.0, 100.0, 50.0, 50.0));
        leaf(&mut tree, Some(target), "drop-here", DispatchFlags::NONE, listen(vec![]), (100.0, 100.0, 50.0, 50.0));
        let mut dispatcher = Dispatcher::new();
        let source_element = tree.node(source).unwrap().element;
        let target_element = tree.node(target).unwrap().element;
        dispatcher.set_drag_payload(source_element, DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "x".to_string())]));
        dispatcher.set_drop_accept(target_element, |payload| payload.contains_key("application/x-semio-catalogue-item"));

        dispatcher.dispatch(&tree, &down(1, 5.0, 5.0));
        dispatcher.dispatch(&tree, &mv(1, 120.0, 120.0));

        assert_eq!(dispatcher.drag_session().and_then(|drag| drag.drop_target), None, "the predicate must reject this payload's mime key");
    }
    //#endregion 🔖️DragDropTests

    //#region 🔖️ScrollTests
    #[test]
    fn scroll_routes_to_the_nearest_scrollable_ancestor_and_clamps_at_zero() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::SCROLLABLE, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "content", DispatchFlags::NONE, listen(vec![]), (10.0, 10.0, 20.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        let root_element = tree.node(root).unwrap().element;

        dispatcher.dispatch(&tree, &DispatchEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: 30.0 });
        assert_eq!(dispatcher.scroll_offset(root_element), (0.0, 30.0));

        dispatcher.dispatch(&tree, &DispatchEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: -100.0 });
        assert_eq!(dispatcher.scroll_offset(root_element), (0.0, 0.0), "scroll offset must clamp at zero, not go negative");
    }

    #[test]
    fn scroll_thumb_capture_drags_the_scrollable_offset_along_its_registered_axis() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER | DispatchFlags::SCROLLABLE, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let thumb = leaf(&mut tree, Some(root), "thumb", DispatchFlags::NONE, listen(vec![]), (190.0, 0.0, 10.0, 40.0));
        let mut dispatcher = Dispatcher::new();
        let root_element = tree.node(root).unwrap().element;
        let thumb_element = tree.node(thumb).unwrap().element;
        dispatcher.register_scroll_thumb(thumb_element, root_element, ScrollAxis::Vertical);

        dispatcher.dispatch(&tree, &down(1, 195.0, 5.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), Some((root_element, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));

        dispatcher.dispatch(&tree, &mv(1, 195.0, 25.0));
        assert_eq!(dispatcher.scroll_offset(root_element), (0.0, 20.0));

        dispatcher.dispatch(&tree, &up(1, 195.0, 25.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), None);
    }
    //#endregion 🔖️ScrollTests

    //#region 🔖️EditStateTests
    #[test]
    fn focusing_an_input_seeds_edit_state_from_its_value_and_blur_clears_it() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let input = leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("hello"), (0.0, 0.0, 100.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        let input_element = tree.node(input).unwrap().element;

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        assert_eq!(dispatcher.edit_state(input_element), Some(&EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None }));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

        dispatcher.dispatch(&tree, &down(1, 190.0, 190.0));
        assert_eq!(dispatcher.edit_state(input_element), None, "blur must relinquish the buffer so the declarative value governs again");
    }

    #[test]
    fn arrow_keys_move_the_caret_and_backspace_deletes_the_previous_char() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("abc"), (0.0, 0.0, 100.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        let input_element = dispatcher.focused().unwrap();

        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers::default() });
        assert_eq!(dispatcher.edit_state(input_element).unwrap().caret, 2);

        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
        let edit = dispatcher.edit_state(input_element).unwrap();
        assert_eq!((edit.anchor, edit.caret), (2, 1), "shift+arrow extends the selection instead of collapsing it");

        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Backspace".into(), modifiers: EventModifiers::default() });
        let edit = dispatcher.edit_state(input_element).unwrap();
        assert_eq!(edit.text, "ac", "backspace over a selection deletes the selected range");
        assert_eq!((edit.anchor, edit.caret), (1, 1));
    }

    #[test]
    fn character_insertion_replaces_the_selection() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("abc"), (0.0, 0.0, 100.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        let input_element = dispatcher.focused().unwrap();

        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
        dispatcher.dispatch(&tree, &DispatchEvent::TextInput { text: "xyz".into() });

        let edit = dispatcher.edit_state(input_element).unwrap();
        assert_eq!(edit.text, "xyz");
        assert_eq!((edit.anchor, edit.caret), (3, 3));
    }

    #[test]
    fn copy_over_a_selection_emits_a_clipboard_intent_without_mutating_the_buffer() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable("hello"), (0.0, 0.0, 100.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        let input_element = dispatcher.focused().unwrap();

        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
        dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
        let outcome = dispatcher.dispatch(&tree, &DispatchEvent::KeyDown { key: "c".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });

        assert!(outcome.intents.iter().any(|intent| intent.action == ActionId::v1("dispatch", "clipboardCopy") && intent.input == Some(UiValue::Text("hello".into()))));
        assert_eq!(dispatcher.edit_state(input_element).unwrap().text, "hello", "copy must not mutate the buffer");
    }

    #[test]
    fn ime_commit_inserts_the_composed_text_and_clears_composition() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "name", DispatchFlags::FOCUSABLE | DispatchFlags::EDITABLE, listen_editable(""), (0.0, 0.0, 100.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        let input_element = dispatcher.focused().unwrap();

        dispatcher.dispatch(&tree, &DispatchEvent::Ime(ImeEvent::Start));
        dispatcher.dispatch(&tree, &DispatchEvent::Ime(ImeEvent::Update { text: "ねこ".into(), cursor: 2 }));
        assert_eq!(dispatcher.edit_state(input_element).unwrap().composition.as_deref(), Some("ねこ"));

        dispatcher.dispatch(&tree, &DispatchEvent::Ime(ImeEvent::Commit { text: "ねこ".into() }));
        let edit = dispatcher.edit_state(input_element).unwrap();
        assert_eq!(edit.text, "ねこ");
        assert_eq!(edit.composition, None);
    }
    //#endregion 🔖️EditStateTests

    //#region 🔖️HoverRevealTests
    #[test]
    fn hovering_a_leaf_marks_its_whole_ancestor_chain_hovered_and_clearing_hover_clears_it_all() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        let row = leaf(&mut tree, Some(root), "row", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        let label = leaf(&mut tree, Some(row), "label", DispatchFlags::NONE, listen(vec![]), (0.0, 0.0, 50.0, 20.0));
        let mut dispatcher = Dispatcher::new();
        let root_element = tree.node(root).unwrap().element;
        let row_element = tree.node(row).unwrap().element;
        let label_element = tree.node(label).unwrap().element;

        dispatcher.dispatch(&tree, &mv(1, 10.0, 10.0));
        assert!(dispatcher.is_hovered(label_element));
        assert!(dispatcher.is_hovered(row_element), "an ancestor layout container must observe hover too, for a host's reveal-on-hover styling");
        assert!(dispatcher.is_hovered(root_element));

        dispatcher.dispatch(&tree, &mv(1, 500.0, 500.0));
        assert!(!dispatcher.is_hovered(label_element));
        assert!(!dispatcher.is_hovered(row_element));
        assert!(!dispatcher.is_hovered(root_element));
    }
    //#endregion 🔖️HoverRevealTests

    //#region 🔖️W2InteractivityTests
    // 🔽️ Generic replacement for `events.rs`'s `Select`-open/close and `Stack.activate`/`drop_action`/
    // `Tree`-row-`draggable` wiring — see this file's module docstring for why the mechanism is now
    // flag-driven (`OVERLAY_TRIGGER`/`LAYOUT_CONTAINER`/`DRAG_SOURCE`) instead of matching product enum
    // variants, and `find_tree_item_spec`'s own doc-comment-documented product knowledge is gone
    // entirely: `Dispatcher::set_drag_payload` is the whole registration surface now, callable by any
    // element regardless of its product shape.
    #[test]
    fn clicking_an_overlay_trigger_opens_its_popup_and_clicking_again_closes_it() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), "select", DispatchFlags::FOCUSABLE | DispatchFlags::OVERLAY_TRIGGER, listen(vec![bind(Trigger::Activate, "toggle")]), (0.0, 0.0, 100.0, 30.0));
        let mut dispatcher = Dispatcher::new();
        let select_element = tree.node(select).unwrap().element;

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        assert_eq!(dispatcher.open_overlays().len(), 1, "clicking a closed overlay trigger should open its popup");
        assert_eq!(dispatcher.open_overlays()[0].root, select_element);

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        assert!(dispatcher.open_overlays().is_empty(), "clicking an open trigger again should close its popup");
    }

    #[test]
    fn a_press_outside_an_open_overlay_trigger_closes_it_and_swallows_the_press() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), "select", DispatchFlags::FOCUSABLE | DispatchFlags::OVERLAY_TRIGGER | DispatchFlags::OVERLAY, listen(vec![bind(Trigger::Activate, "toggle")]), (0.0, 0.0, 100.0, 30.0));
        let mut dispatcher = Dispatcher::new();
        let _ = tree.node(select).unwrap().element;
        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        assert_eq!(dispatcher.open_overlays().len(), 1);

        let outcome = dispatcher.dispatch(&tree, &down(1, 190.0, 190.0));

        assert!(dispatcher.open_overlays().is_empty(), "a press well outside the trigger and its popup should close it");
        assert!(outcome.handled);
    }

    #[test]
    fn picking_a_descendant_row_fires_its_action_and_closes_the_popup() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), "select", DispatchFlags::FOCUSABLE | DispatchFlags::OVERLAY_TRIGGER | DispatchFlags::OVERLAY, listen(vec![bind(Trigger::Activate, "toggle")]), (0.0, 0.0, 100.0, 30.0));
        leaf(&mut tree, Some(select), "row-b", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "pick-b")]), (0.0, 32.0, 100.0, 24.0));
        let mut dispatcher = Dispatcher::new();

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));
        assert_eq!(dispatcher.open_overlays().len(), 1);

        dispatcher.dispatch(&tree, &down(1, 10.0, 40.0));
        let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 40.0));

        assert!(outcome.intents.iter().any(|intent| intent.action == act("pick-b")), "picking a row should fire its own action");
        assert!(dispatcher.open_overlays().is_empty(), "picking an item should close the popup");
        let _ = select;
    }

    #[test]
    fn clicking_an_activatable_layout_container_fires_its_activate_action() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "card", DispatchFlags::LAYOUT_CONTAINER, listen(vec![bind(Trigger::Activate, "open-card")]), (0.0, 0.0, 100.0, 40.0));
        let mut dispatcher = Dispatcher::new();

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

        assert!(outcome.intents.iter().any(|intent| intent.action == act("open-card")), "a LAYOUT_CONTAINER node with its own binding must still fire it");
    }

    #[test]
    fn a_bare_layout_container_without_bindings_stays_a_hit_test_pass_through() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "plain", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 40.0));

        assert_eq!(hit_test(&tree, root, 10.0, 10.0), None, "a bare layout container (no bindings/drag source) must remain a hit-test pass-through");
    }

    #[test]
    fn pressing_a_drag_source_then_moving_past_threshold_promotes_it_to_a_drag_session() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        let row = leaf(&mut tree, Some(root), "row", DispatchFlags::DRAG_SOURCE, listen(vec![]), (0.0, 0.0, 200.0, 24.0));
        let mut dispatcher = Dispatcher::new();
        let row_element = tree.node(row).unwrap().element;
        let payload = DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "{}".to_string())]);
        dispatcher.set_drag_payload(row_element, payload.clone());

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        assert_eq!(dispatcher.capture_of(PointerId(1)), Some((row_element, CaptureKind::Press)), "the row must be a real hit-test target once DRAG_SOURCE-flagged");

        dispatcher.dispatch(&tree, &mv(1, 30.0, 10.0));
        let drag = dispatcher.drag_session().expect("moving past the promote threshold should start a DragSession for a draggable row");
        assert_eq!(drag.source, row_element);
        assert_eq!(drag.payload, payload);
    }
    //#endregion 🔖️W2InteractivityTests

    //#region 🔖️W4SceneCommandTests
    // 🎬️ `events.rs`'s `UiCommand::Scene` routed a raw event into a `ComponentScene` leaf by matching
    // that one `UiNode` variant — the exact per-product special-casing this port eliminates (see this
    // file's report). The replacement property to test is generic multi-binding dispatch: any node can
    // declare several typed bindings, and only the one matching the interaction that actually occurred
    // fires, with zero dispatch.rs code paths caring what the node "is".
    #[test]
    fn a_node_with_multiple_typed_bindings_only_fires_the_one_matching_the_interaction() {
        let mut tree = DispatchTree::new(UiRevision(0));
        let root = leaf(&mut tree, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), "control", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "activate-name"), bind(Trigger::Delta, "delta-name")]), (0.0, 0.0, 200.0, 20.0));
        let mut dispatcher = Dispatcher::new();

        dispatcher.dispatch(&tree, &down(1, 10.0, 10.0));
        let outcome = dispatcher.dispatch(&tree, &up(1, 10.0, 10.0));

        assert!(outcome.intents.iter().any(|intent| intent.trigger == Trigger::Activate && intent.action == act("activate-name")));
        assert!(outcome.intents.iter().all(|intent| intent.trigger != Trigger::Delta), "only the Activate binding should fire from a pointer release, regardless of what other triggers the same node also declares");
    }
    //#endregion 🔖️W4SceneCommandTests

    //#region 🔖️StaleRevisionTests
    #[test]
    fn stale_captured_activate_is_rejected_rather_than_misapplied() {
        let mut tree_at_capture = DispatchTree::new(UiRevision(5));
        let root = leaf(&mut tree_at_capture, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        leaf(&mut tree_at_capture, Some(root), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 100.0, 40.0));
        let mut dispatcher = Dispatcher::new();
        dispatcher.dispatch(&tree_at_capture, &down(1, 10.0, 10.0));
        assert!(dispatcher.capture_of(PointerId(1)).is_some());

        let mut tree_later = DispatchTree::new(UiRevision(8));
        let root_later = leaf(&mut tree_later, None, "root", DispatchFlags::LAYOUT_CONTAINER, listen(vec![]), (0.0, 0.0, 100.0, 100.0));
        leaf(&mut tree_later, Some(root_later), "go", DispatchFlags::FOCUSABLE, listen(vec![bind(Trigger::Activate, "go")]), (0.0, 0.0, 100.0, 40.0));

        let outcome = dispatcher.dispatch(&tree_later, &up(1, 10.0, 10.0));

        assert!(outcome.intents.is_empty(), "a capture that outlived 3 revisions of unrelated churn must not fire its stale Activate");
        assert_eq!(dispatcher.capture_of(PointerId(1)), None, "capture is still released even though the intent is rejected");
    }
    //#endregion 🔖️StaleRevisionTests
}
