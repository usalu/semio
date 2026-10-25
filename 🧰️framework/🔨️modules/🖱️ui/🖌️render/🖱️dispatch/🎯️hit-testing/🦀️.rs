//! 🧪️ Hit testing: the frame node, hitbox and focus ids, the dispatch flags and the hit test of a point against the hitbox tree. See [`super`].

use super::*;

//#region 🔖️HitTest
/// 🧮️ A [`DispatchNode`]'s index within one [`DispatchTree`] — valid only for the tree it came from;
/// never persisted across a frame rebuild (use [`ElementId`] for that — see this file's docstring).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FrameNodeId(pub(super) u32);

/// 🗂️ A [`Hitbox`]'s index within one [`DispatchTree::hitboxes`] — same per-frame-only validity as
/// [`FrameNodeId`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HitboxId(pub(super) u32);

/// ⭕️ A [`DispatchNode`]'s slot in the focus ring — present exactly when the node carries
/// [`DispatchFlags::FOCUSABLE`]. A thin wrapper over [`FrameNodeId`] rather than a separate allocation,
/// since a focus target always *is* a dispatch node.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FocusId(pub(super) FrameNodeId);

/// 🚩️ Hand-rolled bitflag `u32`, matching `schedule.rs::InvalidationReason`'s own convention (this
/// crate's `Cargo.toml` is registrar-only and already carries no bitflags-style dependency).
///
/// The old `tree::NodeFlags` additionally carried `ACTIVE`/`FOCUSED`/`HOVERED` — dynamic interaction
/// *state*, which this port deliberately does not put here: [`DispatchTree`] is immutable per frame
/// (shared via `Rc<FrameSnapshot>`), so those three live on [`Dispatcher`] instead, keyed by
/// [`ElementId`], and are read accessors rather than flags flipped in place. What remains here is every
/// flag that is a **declared capability** of the node itself, set once when the element is built.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DispatchFlags(pub(super) u32);

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

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl std::ops::BitOr for DispatchFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// 📐️ Half-open rect containment — `Bounds`/`LayoutRect` (`scene.rs`) carries no such method itself.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn rect_contains(bounds: Bounds, x: f32, y: f32) -> bool {
    x >= bounds.x && x < bounds.x + bounds.w && y >= bounds.y && y < bounds.y + bounds.h
}

/// 📏️ A node's absolute bounds, if it has a hitbox — `None` for a purely logical node with no
/// geometry of its own.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn node_bounds(tree: &DispatchTree, id: FrameNodeId) -> Option<Bounds> {
    tree.node(id)?.hitbox.map(|hitbox| tree.bounds(hitbox))
}

/// 🥞️ Ported from `events.rs::is_plain_stack_container` — see this file's module docstring for why
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

/// 🔬️ Ported from `events.rs::hit_test_subtree`, simplified: since [`Hitbox::bounds`] is already
/// window-absolute (unlike `wgpu-old`'s parent-relative `layout.x`/`layout.y`), no re-basing into the
/// subtree's own coordinate frame is needed — this is exactly [`hit_test`] with a non-root starting
/// point.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn hit_test_subtree(tree: &DispatchTree, subtree_root: FrameNodeId, x: f32, y: f32) -> Option<FrameNodeId> {
    hit_test(tree, subtree_root, x, y)
}
//#endregion 🔖️HitTest
