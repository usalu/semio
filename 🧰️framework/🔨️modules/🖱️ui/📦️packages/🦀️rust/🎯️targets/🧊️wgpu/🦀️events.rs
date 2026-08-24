// #region events
//! 🎯️ Retained-mode input routing (`UiEvent` in, `UiCommand` out): reverse-paint-order hit testing
//! with clip pruning and overlay priority, pointer capture, Tab-order focus, and parent-chain
//! bubbling. Conceptually replaces the old immediate-mode `input` region's per-frame
//! `hit_targets`/`DragState` bookkeeping, but that region stays fully in place — `widgets`/`chrome`
//! and, transitively, `framework/renderer/wgpu` and `infinite_world` still consume it directly, and
//! the cutover to this module is later-phase renderer-thinning work (see the plan). `events` is
//! purely additive: it depends on `tree`/`component`/`geometry` only, never on `input`.

use std::collections::HashMap;

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{SurfaceKind, UiNode, UiTreeItemNode, UiTreeSectionNode};
use crate::wgpu::geometry::Rect;
use crate::wgpu::tree::{EditState, Node, NodeFlags, NodeKey, UiTree};

//#region 🔖️UiEvent
/// 🖱️ Mouse button identity for `UiEvent::{PointerDown,PointerUp}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
    Middle,
}

/// ⌨️ Modifier keys held during a keyboard event. A minimal fresh type rather than reusing
/// `input::PointerModifiers`, so this module stays decoupled from the region it conceptually
/// replaces (see module doc comment).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EventModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// 📥️ Input events the host feeds into `EventRouter::dispatch`.
#[derive(Clone, Debug, PartialEq)]
pub enum UiEvent {
    PointerDown {
        x: f32,
        y: f32,
        button: PointerButton,
    },
    PointerUp {
        x: f32,
        y: f32,
        button: PointerButton,
    },
    PointerMove {
        x: f32,
        y: f32,
    },
    Scroll {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
    },
    KeyDown {
        key: String,
        modifiers: EventModifiers,
    },
    KeyUp {
        key: String,
        modifiers: EventModifiers,
    },
    TextInput {
        text: String,
    },
    /// 📋️ Host-delivered clipboard text in response to a `UiCommand::ClipboardPasteRequested` (the
    /// actual OS clipboard read is a `host`-region/renderer-integration concern; this is just the
    /// inbound half of the round trip). Routed identically to `TextInput`: inserted at the focused
    /// `EditState`'s caret, replacing any selection.
    Paste {
        text: String,
    },
    /// 🈶️ IME composition lifecycle for the focused editable node. Shapes `EditState::composition`
    /// is ready to receive; actually wiring winit's `Ime` events (native) or a hidden DOM input
    /// (web) to *produce* these is later `host`-region work, out of scope here.
    Ime(ImeEvent),
}

/// 🈶️ One IME composition step — see `UiEvent::Ime`.
#[derive(Clone, Debug, PartialEq)]
pub enum ImeEvent {
    Start,
    /// `cursor` is the IME's own preedit-relative cursor, informational only (not routed into
    /// `EditState::caret` — the composition is still uncommitted).
    Update {
        text: String,
        cursor: usize,
    },
    /// 🈶️ Finalizes the composition: clears `EditState::composition` and inserts `text` at the caret
    /// exactly like `TextInput`.
    Commit {
        text: String,
    },
    Cancel,
}
//#endregion 🔖️UiEvent

//#region 🔖️HitTest
/// 🎯️ Reverse-paint-order hit test from `root`: `paint::paint_stack` walks first_child→last_child
/// (parent background first, then children in that order, each drawn over the last), so the
/// topmost node at any point is the *last*-painted one — this walk visits children last-first to
/// match. Overlay-flagged (`NodeFlags::OVERLAY`) children are tested before normal siblings at
/// every level, so a popup always wins over base content underneath it. `CLIPS_CHILDREN` prunes
/// early: a point outside that node's own bounds skips testing its children entirely, even if a
/// child's own (unclipped) rect would nominally contain the point. `HIT_TRANSPARENT` nodes are
/// skipped for the match itself (their children are still tested — pass-through). Returns the
/// deepest/topmost matching node.
pub(crate) fn hit_test(tree: &UiTree, root: NodeId, x: f32, y: f32) -> Option<NodeId> {
    hit_test_node(tree, root, 0.0, 0.0, x, y)
}

fn hit_test_node(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, x: f32, y: f32) -> Option<NodeId> {
    let node = tree.node(id)?;
    let layout = tree.accepted_layout(id)?;
    let abs_x = origin_x + layout.x;
    let abs_y = origin_y + layout.y;
    let inside = Rect::new(abs_x, abs_y, layout.width, layout.height).contains(x, y);
    if node.flags.contains(NodeFlags::CLIPS_CHILDREN) && !inside {
        return None;
    }
    let mut overlays: Vec<NodeId> = Vec::new();
    let mut normal: Vec<NodeId> = Vec::new();
    for child in tree.children(id) {
        match tree.node(child) {
            Some(child_node) if child_node.flags.contains(NodeFlags::OVERLAY) => overlays.push(child),
            _ => normal.push(child),
        }
    }
    for child in overlays.into_iter().rev().chain(normal.into_iter().rev()) {
        if let Some(hit) = hit_test_node(tree, child, abs_x, abs_y, x, y) {
            return Some(hit);
        }
    }
    // A bare `Stack` is a layout-only container with no interaction semantics of its own — it
    // must never be the hit result itself, only a pass-through to its children (same intent as
    // `HIT_TRANSPARENT`, just implicit for this variant instead of flag-driven) — *unless* W2
    // wiring (`is_plain_stack_container`) finds it actually carries `activate`/`drop_action`, or is
    // a registered drag source — any of those make it a real interaction target.
    let is_plain_container = is_plain_stack_container(node);
    if inside && !node.flags.contains(NodeFlags::HIT_TRANSPARENT) && !is_plain_container {
        Some(id)
    } else {
        None
    }
}

/// 🎯️🌳️ W2 wiring: a `Stack` (`node.spec.0`) stops being a plain pass-through container the moment
/// it carries `activate`/`drop_action` of its own, or is a registered `NodeFlags::DRAG_SOURCE`
/// (`paint::sync_interactive_state` keeps that flag synced with `Tree` rows' `draggable` field, and
/// `dispatch`'s `PointerDown` handling can only ever register a drag payload on a node that's
/// actually reachable as a hit-test target in the first place — see `find_tree_item_spec`'s own
/// caller in `dispatch`). ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: a `Tree`
/// row's per-item `hover_action`/`unhover_action` exception is deleted — hover on a tree row is now
/// dispatched through the row's `UiTreeNode.interaction_domain` binding (`interactionHover`),
/// never an ad hoc per-item action.
fn is_plain_stack_container(node: &Node) -> bool {
    let UiNode::Stack(stack) = &node.spec.0 else { return false };
    stack.activate.is_none() && stack.drop_action.is_none() && !node.flags.contains(NodeFlags::DRAG_SOURCE)
}

//#region 🔖️TreeItemLookup
/// 🌳️ Re-derives a `Tree` row's *original* `UiTreeItemNode` spec — `draggable`/`drag_data`, fields
/// `UiStackNode` (the row's synthesized retained shape, see
/// `reconcile::children_of`'s `Tree` arm) has no room for at all — by walking up from `row` to the
/// nearest ancestor `UiNode::Tree` and searching its still-fully-intact spec (`reconcile` never
/// drops fields, only clones them into `WidgetSpec` — see that module's own doc comment) for the
/// item whose `id` matches this row's own stable key (`NodeKey::Explicit(item.id)`, exactly what
/// `reconcile::tree_item_row` keys the row with). `None` for anything that isn't a keyed descendant
/// of a `Tree` (ordinary `Stack`s, a `Tree`'s section rows, which are keyed by `section.id` instead).
fn find_tree_item_spec(tree: &UiTree, row: NodeId) -> Option<&UiTreeItemNode> {
    let NodeKey::Explicit(row_id) = &tree.node(row)?.key else { return None };
    let mut ancestor = tree.node(row)?.parent;
    while let Some(candidate) = ancestor {
        let candidate_node = tree.node(candidate)?;
        if let UiNode::Tree(tree_node) = &candidate_node.spec.0 {
            return find_item_in_sections(&tree_node.sections, row_id);
        }
        ancestor = candidate_node.parent;
    }
    None
}

fn find_item_in_sections<'a>(sections: &'a [UiTreeSectionNode], id: &str) -> Option<&'a UiTreeItemNode> {
    sections.iter().find_map(|section| find_item_in_items(&section.items, id))
}

fn find_item_in_items<'a>(items: &'a [UiTreeItemNode], id: &str) -> Option<&'a UiTreeItemNode> {
    for item in items {
        if item.id == id {
            return Some(item);
        }
        if let Some(nested) = &item.items {
            if let Some(found) = find_item_in_items(nested, id) {
                return Some(found);
            }
        }
    }
    None
}
//#endregion 🔖️TreeItemLookup

/// 📐️ A node's absolute (window-space) origin: `LayoutBucket`'s own doc comment fixes `x`/`y` as
/// **parent-relative**, so this walks the parent chain to `root` (whose own origin is `(0.0, 0.0)`)
/// summing offsets. Used by the overlay placement/dismissal machinery, which needs a node's real
/// on-screen bounds rather than its parent-relative layout rect.
fn node_abs_origin(tree: &UiTree, id: NodeId) -> (f32, f32) {
    match tree.node(id) {
        Some(node) => {
            let layout = tree.accepted_layout(id).unwrap_or_default();
            let (parent_x, parent_y) = match node.parent {
                Some(parent) => node_abs_origin(tree, parent),
                None => (0.0, 0.0),
            };
            (parent_x + layout.x, parent_y + layout.y)
        }
        None => (0.0, 0.0),
    }
}

/// 📐️ `node_abs_origin` plus the node's own size, as a `Rect` — `None` if `id` isn't in `tree`.
pub(crate) fn node_abs_rect(tree: &UiTree, id: NodeId) -> Option<Rect> {
    let node = tree.node(id)?;
    let layout = tree.accepted_layout(id)?;
    let (x, y) = node_abs_origin(tree, id);
    Some(Rect::new(x, y, layout.width, layout.height))
}

/// 🎯️ `hit_test`, but `subtree_root` need not be the window's true tree root (an overlay root
/// virtually never is — it's some descendant node). `hit_test` walks from its `root` argument
/// treating that node's own `layout.x`/`layout.y` as relative to origin `(0.0, 0.0)`, which is only
/// correct window-absolute-coordinate behavior when `root` itself has no parent; this instead
/// resolves `subtree_root`'s *parent's* absolute origin (`(0.0, 0.0)` if it has none) and translates
/// `(x, y)` into that frame first, so overlay dismissal/hover-out checks against a non-root overlay
/// subtree stay correct regardless of how deep it's nested.
pub(crate) fn hit_test_subtree(tree: &UiTree, subtree_root: NodeId, x: f32, y: f32) -> Option<NodeId> {
    let (parent_x, parent_y) = match tree.node(subtree_root).and_then(|node| node.parent) {
        Some(parent) => node_abs_origin(tree, parent),
        None => (0.0, 0.0),
    };
    hit_test(tree, subtree_root, x - parent_x, y - parent_y)
}
//#endregion 🔖️HitTest

//#region 🔖️Capture
/// ↕️ Which axis a `CaptureKind::ScrollThumb` drag maps pointer delta onto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollAxis {
    Horizontal,
    Vertical,
}

/// 🫳️ What kind of interaction currently holds pointer capture. A coarser-grained, retained-mode
/// replacement for the old `input::DragState`/`TreeDragState` pair. `Drag` is a generic
/// `DragSession` (see 🔖️DragDrop below) promoted from `Press` once pointer movement past a small
/// threshold is observed on a node with a registered `DragPayload`. `ScrollThumb` is a scrollbar
/// thumb (painted by `paint`, registered via `EventRouter::register_scroll_thumb`) dragging its
/// owning `NodeFlags::SCROLLABLE` node's `WidgetState::scroll_offset` along one axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureKind {
    Press,
    Drag,
    ScrollThumb(ScrollAxis),
}

/// 🔒️ Once a node captures, subsequent pointer-move/up events route directly to it regardless of
/// what's actually under the pointer, until released on `PointerUp` (or explicit `release`).
#[derive(Clone, Copy, Debug, Default)]
struct CaptureState {
    target: Option<(NodeId, CaptureKind)>,
}

impl CaptureState {
    fn release(&mut self) -> Option<(NodeId, CaptureKind)> {
        self.target.take()
    }
}
//#endregion 🔖️Capture

//#region 🔖️Focus
/// 🎯️ Which `UiNode` variants participate in Tab-order focus cycling.
fn is_focusable(node: &UiNode) -> bool {
    matches!(node, UiNode::Input(_) | UiNode::Button(_) | UiNode::Select(_) | UiNode::Toggle(_) | UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) | UiNode::IconSelect(_))
}

fn collect_focusable(tree: &UiTree, id: NodeId, out: &mut Vec<NodeId>) {
    if let Some(node) = tree.node(id) {
        if is_focusable(&node.spec.0) {
            out.push(id);
        }
    }
    for child in tree.children(id) {
        collect_focusable(tree, child, out);
    }
}

/// 🔦️ Currently-focused node plus a lazily-rebuilt document-order Tab cycle over focusable nodes.
struct FocusState {
    focused: Option<NodeId>,
    tab_order: Vec<NodeId>,
}

impl FocusState {
    fn new() -> Self {
        Self { focused: None, tab_order: Vec::new() }
    }

    /// 🎯️ Sets/clears focus, flipping `NodeFlags::FOCUSED` on the old and new targets and marking
    /// both `DIRTY_PAINT` (a focus ring likely needs repainting) via `UiTree::mark_dirty`. A no-operation
    /// (no flag churn) when `node` already matches the current focus. Also owns `EditState`'s
    /// lifecycle: blurring a node clears its `WidgetState::edit` (the buffer relinquishes control,
    /// so the node's declarative `value` governs again on the next `apply_tree`); focusing a
    /// `UiNode::Input` for the first time seeds `edit` from that declarative `value` with the caret
    /// at the end — see `tree::WidgetState`'s own doc comment for why reconcile never clobbers this.
    fn set_focus(&mut self, tree: &mut UiTree, node: Option<NodeId>) {
        if self.focused == node {
            return;
        }
        if let Some(previous) = self.focused {
            if let Some(previous_node) = tree.node_mut(previous) {
                previous_node.flags.set(NodeFlags::FOCUSED, false);
                previous_node.state.edit = None;
            }
            tree.mark_dirty(previous, NodeFlags::DIRTY_PAINT);
        }
        if let Some(next) = node {
            if let Some(next_node) = tree.node_mut(next) {
                next_node.flags.set(NodeFlags::FOCUSED, true);
                if next_node.state.edit.is_none() {
                    if let UiNode::Input(input) = &next_node.spec.0 {
                        let caret = input.value.len();
                        next_node.state.edit = Some(EditState { text: input.value.clone(), caret, anchor: caret, composition: None, scroll_x: 0.0 });
                    }
                }
            }
            tree.mark_dirty(next, NodeFlags::DIRTY_PAINT);
        }
        self.focused = node;
    }

    fn clear_focus(&mut self, tree: &mut UiTree) {
        self.set_focus(tree, None);
    }

    fn rebuild_tab_order(&mut self, tree: &UiTree, root: NodeId) {
        self.tab_order.clear();
        collect_focusable(tree, root, &mut self.tab_order);
    }

    fn focus_next(&mut self, tree: &mut UiTree, root: NodeId) {
        self.rebuild_tab_order(tree, root);
        if self.tab_order.is_empty() {
            self.set_focus(tree, None);
            return;
        }
        let next_index = match self.focused.and_then(|id| self.tab_order.iter().position(|&candidate| candidate == id)) {
            Some(index) => (index + 1) % self.tab_order.len(),
            None => 0,
        };
        self.set_focus(tree, Some(self.tab_order[next_index]));
    }

    fn focus_prev(&mut self, tree: &mut UiTree, root: NodeId) {
        self.rebuild_tab_order(tree, root);
        if self.tab_order.is_empty() {
            self.set_focus(tree, None);
            return;
        }
        let previous_index = match self.focused.and_then(|id| self.tab_order.iter().position(|&candidate| candidate == id)) {
            Some(index) => (index + self.tab_order.len() - 1) % self.tab_order.len(),
            None => self.tab_order.len() - 1,
        };
        self.set_focus(tree, Some(self.tab_order[previous_index]));
    }
}
//#endregion 🔖️Focus

//#region 🔖️Bubble
/// 🫧️ Walks from `from` up through `parent` links (including `from` itself), calling `handler(id)`
/// for each ancestor until it returns `true` ("handled, stop bubbling") or the root is reached.
pub(crate) fn bubble<F: FnMut(NodeId) -> bool>(tree: &UiTree, from: NodeId, mut handler: F) {
    let mut cursor = Some(from);
    while let Some(id) = cursor {
        if handler(id) {
            return;
        }
        cursor = tree.node(id).and_then(|node| node.parent);
    }
}

/// 🌳️ Whether `id` is `ancestor` itself or a descendant of it, walking the parent chain.
fn is_descendant(tree: &UiTree, id: NodeId, ancestor: NodeId) -> bool {
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
//#endregion 🔖️Bubble

//#region 🔖️Overlay
// 🪟️ One first-class overlay mechanism serving Select popups, context menus, tooltips, dialogs, and
// a command palette — not five bespoke implementations. `NodeFlags::OVERLAY` already gives a
// flagged child hit-test priority over its normal siblings (see 🔖️HitTest above); `EventRouter`
// layers open/close/anchor/placement/dismissal/focus-trap bookkeeping on top of that one existing
// primitive. Building the popup CONTENTS (a `Select`'s item list, a context menu's entries, …) is
// explicitly not this module's job — a caller (future `reconcile`/`paint`/`host` wiring) reconciles
// that subtree in and hands this module its root `NodeId` plus a `kind`/`anchor`; from there this
// module owns the subtree's lifecycle.

/// 🏷️ Which of the five overlay use-cases is open — drives the default placement rule and
/// dismissal policy (`OverlayKind::default_placement`/`dismiss_policy`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayKind {
    SelectPopup,
    ContextMenu,
    Tooltip,
    Dialog,
    CommandPalette,
}

/// ⚓️ What an overlay is positioned relative to: an existing node (a `Select`'s trigger, a hovered
/// row) or a raw point (where a context menu was right-clicked, where the pointer was when a
/// tooltip's hover-delay fired).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayAnchor {
    Node(NodeId),
    Point { x: f32, y: f32 },
}

/// 📐️ How an overlay's resolved position is computed from its anchor — see
/// `resolve_overlay_placement`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayPlacement {
    /// 👇️ `SelectPopup`/`ContextMenu`: directly below the anchor, flipped above it if that would
    /// overflow the viewport's bottom edge.
    BelowAnchorWithFlip,
    /// 🖱️ `Tooltip`: offset from the anchor point.
    AtPointer { offset_x: f32, offset_y: f32 },
    /// 🎯️ `Dialog`/`CommandPalette`: viewport-centered.
    Centered,
}

/// 🚪️ How an open overlay can be dismissed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DismissPolicy {
    /// 👆️ A `PointerDown` landing outside the overlay's subtree closes it and swallows the press
    /// (doesn't fall through to whatever's underneath) — standard popup click-outside semantics.
    pub outside_press_swallow: bool,
    /// ⎋️ `Escape` closes this overlay if it's the topmost open one.
    pub escape_closes: bool,
    /// ⏱️ Tooltip-specific: close this many seconds after the pointer leaves both the anchor and the
    /// overlay's own bounds. **Not actually debounced yet** — this crate has no animation-clock
    /// scaffolding anywhere (`engine::Ui::needs_frame`'s own doc comment makes the same admission for
    /// animations generally), so `maybe_dismiss_tooltip_on_hover_out` closes immediately on hover-out
    /// today; this field records the *intended* delay for whenever a clock exists to debounce it.
    pub hover_out_delay_seconds: Option<f32>,
}

impl OverlayKind {
    pub fn default_placement(self) -> OverlayPlacement {
        match self {
            OverlayKind::SelectPopup | OverlayKind::ContextMenu => OverlayPlacement::BelowAnchorWithFlip,
            OverlayKind::Tooltip => OverlayPlacement::AtPointer { offset_x: 12.0, offset_y: 16.0 },
            OverlayKind::Dialog | OverlayKind::CommandPalette => OverlayPlacement::Centered,
        }
    }

    pub fn dismiss_policy(self) -> DismissPolicy {
        match self {
            OverlayKind::Tooltip => DismissPolicy { outside_press_swallow: false, escape_closes: true, hover_out_delay_seconds: Some(0.4) },
            _ => DismissPolicy { outside_press_swallow: true, escape_closes: true, hover_out_delay_seconds: None },
        }
    }
}

/// 🪟️ One currently-open overlay's lifecycle state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenOverlay {
    /// 🌳️ The overlay's content subtree root — `EventRouter::open_overlay` flags this
    /// `NodeFlags::OVERLAY` for hit-test priority and clears it again on close.
    pub root: NodeId,
    pub kind: OverlayKind,
    pub anchor: OverlayAnchor,
    pub placement: OverlayPlacement,
    pub dismiss: DismissPolicy,
    /// 🔒️ `Dialog`/`CommandPalette`: while `true`, Tab-order cycling is bounded to this overlay's
    /// subtree (see `EventRouter::dispatch`'s `Tab` handling).
    pub focus_trap: bool,
}

/// 🥞️ Open overlays in z-order (last = topmost = painted last = hit-tested first, matching
/// `NodeFlags::OVERLAY`'s own priority rule). Only one `EventRouter` field, but a `Vec` rather than a
/// single slot because a context menu can itself spawn a submenu, or a Select popup can open above a
/// Dialog — nesting is a real case this mechanism must support, not just a single global popup.
#[derive(Default)]
pub(crate) struct OverlayStack {
    open: Vec<OpenOverlay>,
}

impl OverlayStack {
    fn new() -> Self {
        Self::default()
    }

    fn open(&mut self, overlay: OpenOverlay) {
        self.open.push(overlay);
    }

    fn topmost(&self) -> Option<&OpenOverlay> {
        self.open.last()
    }

    fn close_root(&mut self, root: NodeId) -> Option<OpenOverlay> {
        let position = self.open.iter().position(|overlay| overlay.root == root)?;
        Some(self.open.remove(position))
    }

    fn close_topmost(&mut self) -> Option<OpenOverlay> {
        self.open.pop()
    }

    /// 🔒️ The root of the topmost `focus_trap` overlay, if any — `Escape`/outside-press only ever
    /// close the *topmost* overlay, but a focus trap set by a lower (still-open) trapping overlay
    /// stays in effect once a higher non-trapping overlay (e.g. a `Tooltip`) is on top of it, so this
    /// searches from the top down rather than just checking `topmost()`.
    fn topmost_focus_trap_root(&self) -> Option<NodeId> {
        self.open.iter().rev().find(|overlay| overlay.focus_trap).map(|overlay| overlay.root)
    }
}

/// 📐️ Resolves an overlay's top-left origin from its anchor, `kind`'s `placement` rule, the
/// overlay's own measured `content_size` (post-layout — paint/flex, not this module, own measuring
/// it), and the window's `viewport` size. Pure geometry: callers (a future `paint`/`flex` wiring)
/// still own actually writing the result into the overlay root's layout — `events` only decides
/// *where*, per the module doc comment's "the content subtree itself is whatever the caller
/// reconciled in" scoping.
pub fn resolve_overlay_placement(tree: &UiTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement) -> (f32, f32) {
    let anchor_rect = match anchor {
        OverlayAnchor::Node(id) => node_abs_rect(tree, id).unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0)),
        OverlayAnchor::Point { x, y } => Rect::new(x, y, 0.0, 0.0),
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
//#endregion 🔖️Overlay

//#region 🔖️DragDrop
// 🫳️ Generic drag-and-drop session lifecycle: start-drag (promoted from a `Press` capture once
// pointer movement clears `DRAG_PROMOTE_THRESHOLD_SQ`), update-position/evaluate-drop-target
// (`EventRouter::update_drag`, called from `PointerMove`), commit-or-cancel (`PointerUp`). Building
// the specific CONSUMERS (tree reorder, dock retiling, …) is out of scope — this is wire-format
// parity plumbing for whatever consumes `UiCommand::DropCommitted`.

/// 🏷️ Drag payload: MIME-style keys, JSON-encoded string values — exactly the shape
/// `framework/renderer/react/ui-interpreter.tsx`'s `handleDrop` reads off `DataTransfer` (`data:
/// Record<string, string>`, matched by `application/x-semio-*` key prefix) and the shape
/// `UiTreeItemNode::drag_data` already carries. Reusing this shape (rather than a bespoke Rust enum)
/// means a later workstream wiring this into the same program action contracts needs zero translation.
pub type DragPayload = HashMap<String, String>;

/// 👻️ Minimal drag-ghost shape — the actual visual is `paint`'s job (another region/agent); this is
/// just enough for a caller to render *something* under the pointer.
#[derive(Clone, Debug, PartialEq)]
pub struct DragGhost {
    pub label: String,
    pub offset_x: f32,
    pub offset_y: f32,
}

/// 🫳️ One active drag, from promotion out of a `Press` capture (`EventRouter::maybe_promote_to_drag`)
/// through to `PointerUp`'s commit/cancel.
#[derive(Clone, Debug, PartialEq)]
pub struct DragSession {
    pub source: NodeId,
    pub payload: DragPayload,
    pub ghost: Option<DragGhost>,
    pub pointer_x: f32,
    pub pointer_y: f32,
    /// 🎯️ The nearest `NodeFlags::DROP_TARGET` ancestor of whatever's under the pointer right now
    /// that also passes its registered accept predicate (`EventRouter::set_drop_accept`), if any —
    /// recomputed every `PointerMove` by `EventRouter::update_drag`.
    pub drop_target: Option<NodeId>,
}

/// 📏️ Squared pixel distance a `Press` capture on a `DragPayload`-registered node must travel before
/// `EventRouter::maybe_promote_to_drag` promotes it to a real `DragSession` — a small dead-zone so an
/// ordinary click on a draggable node doesn't spuriously start (and then immediately cancel) a drag.
const DRAG_PROMOTE_THRESHOLD_SQ: f32 = 16.0;
//#endregion 🔖️DragDrop

//#region 🔖️Scroll
// 🖱️ Wheel events route to the nearest scrollable ancestor of the node under the pointer, walking
// the bubble chain for `NodeFlags::SCROLLABLE` exactly like `nearest_accepting_drop_target` walks it
// for `NodeFlags::DROP_TARGET`. Thumb-drag capture (`CaptureKind::ScrollThumb`) is a separate path:
// `paint` paints the actual thumb node wherever it likes in the tree and registers it once via
// `EventRouter::register_scroll_thumb`, decoupling thumb geometry from scrollable-content geometry.

/// 🖱️ Walks `from`'s bubble chain (inclusive) for the nearest `NodeFlags::SCROLLABLE` node.
fn nearest_scrollable_ancestor(tree: &UiTree, from: NodeId) -> Option<NodeId> {
    let mut found = None;
    bubble(tree, from, |id| {
        if tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::SCROLLABLE)) {
            found = Some(id);
            true
        } else {
            false
        }
    });
    found
}
//#endregion 🔖️Scroll

//#region 🔖️EditRouting
// ✍️ Key routing for a focused editable node's `tree::EditState`. Byte-offset caret/anchor
// throughout (see `EditState`'s own doc comment for why); `prev_char_boundary`/`next_char_boundary`
// step one `char` at a time without re-deriving a full `char_indices` pass per keystroke.

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

/// ↔ Selection bounds as `(start, end)` regardless of which of `anchor`/`caret` is smaller —
/// `EditState`'s own doc comment documents the selection as `anchor..caret` in either order.
fn selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
    (anchor.min(caret), anchor.max(caret))
}

/// ✍️ Replaces the current selection (or inserts at the caret if there isn't one) with `text`,
/// collapsing caret and anchor to just past the inserted text. Shared by `TextInput`, `Paste`, and
/// `Ime::Commit` routing — insertion semantics are identical for all three.
fn insert_at_caret(edit: &mut EditState, text: &str) {
    let (start, end) = selection_bounds(edit.anchor, edit.caret);
    edit.text.replace_range(start..end, text);
    let caret = start + text.len();
    edit.caret = caret;
    edit.anchor = caret;
}
//#endregion 🔖️EditRouting

//#region 🔖️UiCommand
/// 📤️ What the engine emits for the host to act on, drained once per tick.
#[derive(Clone, Debug, PartialEq)]
pub enum UiCommand {
    /// 🧩️ A widget's declarative `ActionDescriptor` fired (currently: a `Button` clicked while it
    /// still had the node the pointer went down on under the pointer at release).
    App { window_id: String, action: ActionDescriptor },
    /// 🔦️ Focus moved (or cleared) as a result of routing an event.
    FocusChanged { window_id: String, node: Option<NodeId> },
    /// 🪟️ An overlay closed — either explicitly (`EventRouter::close_overlay`) or via dismissal
    /// (outside-press swallow, `Escape`, tooltip hover-out).
    OverlayClosed { window_id: String, root: NodeId, kind: OverlayKind },
    /// 🫳️ A `DragSession` released over an accepting drop target.
    DropCommitted { window_id: String, source: NodeId, target: NodeId, payload: DragPayload },
    /// 🫳️ A `DragSession` released with no accepting drop target under the pointer.
    DropCancelled { window_id: String, source: NodeId },
    /// 📋️ `Ctrl`/`Cmd`+`C` over a text selection — host copies `text` to the OS clipboard.
    ClipboardCopy { window_id: String, text: String },
    /// 📋️ `Ctrl`/`Cmd`+`X` over a text selection — `text` is already removed from the `EditState`
    /// buffer; host copies it to the OS clipboard.
    ClipboardCut { window_id: String, text: String },
    /// 📋️ `Ctrl`/`Cmd`+`V`: host must read the OS clipboard and feed the result back as
    /// `UiEvent::Paste` (the OS clipboard read itself is a `host`-region concern, not `events`').
    ClipboardPasteRequested { window_id: String },
    /// 🎬️ A real `PointerDown`/`PointerUp`/`PointerMove`/`Scroll` `event` that hit-tested to a
    /// `ComponentScene` leaf — the host looks up `node`'s live `UiComponentSceneNode` (same
    /// `window_id`+`node` the retained tree's `scene_slots` region reads) and routes `event` into
    /// that `kind`'s own per-`SurfaceKind` input handler, instead of sampling an aggregate
    /// `InputState` once per render frame the way `framework/renderer/wgpu`'s `RenderEntry` region
    /// used to. `surface_id`/`kind`/`rect` are carried directly (resolved once here, at dispatch
    /// time, from the same ancestor-offset accumulation `scene_slots::collect_scene_slots`/
    /// `hit_test_node` use) so a host doesn't need its own tree walk just to decide whether this
    /// surface already gets real OS-event-driven input through its own bespoke host (`world-3d`/
    /// `node-graph`/`tiled-map`/`board-2d`) before paying for the `node` lookup.
    Scene { window_id: String, node: NodeId, surface_id: String, kind: SurfaceKind, rect: Rect, event: UiEvent },
}

/// 🧭️ Owns capture + focus + overlay + drag + scroll-thumb state for one window's retained tree and
/// turns `UiEvent`s into `NodeFlags`/`WidgetState` updates plus a minimal, correct (not speculative)
/// set of `UiCommand`s. Per-widget-variant semantics beyond generic routing (e.g. actually committing
/// an edited `Input`'s value via its `on_change` `ActionDescriptor`) are a documented gap for a later
/// milestone, same as this struct's own precedent (`Button` was the only concretely-wired variant
/// before M5).
/// 🎯️ A `set_drop_accept` predicate — see `EventRouter::drop_accept`.
type DropAcceptPredicate = Box<dyn Fn(&DragPayload) -> bool + Send + Sync>;

pub(crate) struct EventRouter {
    window_id: String,
    capture: CaptureState,
    focus: FocusState,
    hovered: Option<NodeId>,
    /// 🫧️ Every node currently in the hover bubble chain (leaf-to-root from `hovered`), so an
    /// ancestor container (e.g. a `Stack`-based tree-item row, which `hit_test` never itself returns
    /// as the match — see 🔖️HitTest's `is_plain_container`) still observes `NodeFlags::HOVERED` for
    /// `paint`'s hover-reveal (React's `placement` / driver.chrome reveal) to key off of.
    hover_chain: Vec<NodeId>,
    /// 👇️ Pointer position at the start of the current `Press` capture, for `maybe_promote_to_drag`'s
    /// movement-threshold check.
    press_origin: Option<(f32, f32)>,
    overlays: OverlayStack,
    drag: Option<DragSession>,
    /// 🫳️ Per-node `DragPayload` a `Press` capture on that node may promote into, set via
    /// `set_drag_payload`.
    drag_payloads: HashMap<NodeId, DragPayload>,
    /// 🎯️ Per-node accept predicate refining plain `NodeFlags::DROP_TARGET` membership, set via
    /// `set_drop_accept`. Absent from this map but flagged `DROP_TARGET` still accepts everything.
    drop_accept: HashMap<NodeId, DropAcceptPredicate>,
    /// 🖱️ Scrollbar-thumb node id → (its owning `NodeFlags::SCROLLABLE` node, drag axis), set via
    /// `register_scroll_thumb`.
    scroll_thumbs: HashMap<NodeId, (NodeId, ScrollAxis)>,
    /// 🖱️ `(pointer_x, pointer_y, scroll_offset_x, scroll_offset_y)` captured at the start of a
    /// `ScrollThumb` drag, for `update_scroll_thumb`'s delta-computation baseline.
    thumb_start: Option<(f32, f32, f32, f32)>,
}

impl EventRouter {
    pub(crate) fn new(window_id: impl Into<String>) -> Self {
        Self {
            window_id: window_id.into(),
            capture: CaptureState::default(),
            focus: FocusState::new(),
            hovered: None,
            hover_chain: Vec::new(),
            press_origin: None,
            overlays: OverlayStack::new(),
            drag: None,
            drag_payloads: HashMap::new(),
            drop_accept: HashMap::new(),
            scroll_thumbs: HashMap::new(),
            thumb_start: None,
        }
    }

    fn resolve_target(&self, tree: &UiTree, root: NodeId, x: f32, y: f32) -> Option<NodeId> {
        match self.capture.target {
            Some((id, _)) => Some(id),
            None => hit_test(tree, root, x, y),
        }
    }

    /// 👆️ Flips `NodeFlags::HOVERED` off every node in the old hover bubble chain that isn't in the
    /// new one, and on for every new node that wasn't in the old one — see `hover_chain`'s own doc
    /// comment for why the whole chain (not just the leaf) carries the flag. ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: the per-row `hover_action`/
    /// `unhover_action` dispatch this used to fire is deleted — a `Tree`'s hover now flows through its
    /// `UiTreeNode.interaction_domain` binding (`interactionHover`) instead of an ad hoc per-item action.
    fn update_hover(&mut self, tree: &mut UiTree, target: Option<NodeId>) -> Vec<UiCommand> {
        let commands = Vec::new();
        if self.hovered == target {
            return commands;
        }
        let mut new_chain = Vec::new();
        if let Some(leaf) = target {
            bubble(tree, leaf, |id| {
                new_chain.push(id);
                false
            });
        }
        for &previous in &self.hover_chain {
            if !new_chain.contains(&previous) {
                if let Some(node) = tree.node_mut(previous) {
                    node.flags.set(NodeFlags::HOVERED, false);
                }
                tree.mark_dirty(previous, NodeFlags::DIRTY_PAINT);
            }
        }
        for &next in &new_chain {
            if !self.hover_chain.contains(&next) {
                if let Some(node) = tree.node_mut(next) {
                    node.flags.set(NodeFlags::HOVERED, true);
                }
                tree.mark_dirty(next, NodeFlags::DIRTY_PAINT);
            }
        }
        self.hover_chain = new_chain;
        self.hovered = target;
        commands
    }

    //#region 🔖️OverlayApi
    /// 🪟️ Opens an overlay: flags `root` `NodeFlags::OVERLAY` (hit-test priority — see 🔖️HitTest) and
    /// pushes it onto the z-ordered stack with `kind`'s default placement/dismissal policy.
    /// `Dialog`/`CommandPalette` become focus-trap scopes automatically.
    pub(crate) fn open_overlay(&mut self, tree: &mut UiTree, root: NodeId, kind: OverlayKind, anchor: OverlayAnchor) {
        if let Some(node) = tree.node_mut(root) {
            node.flags.set(NodeFlags::OVERLAY, true);
        }
        tree.mark_dirty(root, NodeFlags::DIRTY_PAINT);
        let focus_trap = matches!(kind, OverlayKind::Dialog | OverlayKind::CommandPalette);
        self.overlays.open(OpenOverlay { root, kind, anchor, placement: kind.default_placement(), dismiss: kind.dismiss_policy(), focus_trap });
    }

    pub(crate) fn close_overlay(&mut self, tree: &mut UiTree, root: NodeId) -> Vec<UiCommand> {
        match self.overlays.close_root(root) {
            Some(overlay) => self.finish_close(tree, overlay),
            None => Vec::new(),
        }
    }

    pub(crate) fn close_topmost_overlay(&mut self, tree: &mut UiTree) -> Vec<UiCommand> {
        match self.overlays.close_topmost() {
            Some(overlay) => self.finish_close(tree, overlay),
            None => Vec::new(),
        }
    }

    #[allow(dead_code, reason = "overlay-stack accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn topmost_overlay(&self) -> Option<&OpenOverlay> {
        self.overlays.topmost()
    }

    /// 🔽️ W2 wiring: the consumer-side effect of a `Select` click — flips `tree::WidgetState::open`
    /// via `open_overlay`/`close_overlay` (root *and* anchor are the `Select` node itself: its own
    /// synthesized item rows, see `reconcile::children_of`'s `Select` arm, are already its retained
    /// children, and marking the `Select` node `NodeFlags::OVERLAY` gives the whole popup hit-test
    /// priority over its own later-painted siblings). All dismissal paths (outside-press, `Escape`,
    /// an explicit `close_overlay`, or picking an item — see `dispatch`'s `PointerUp` handling) funnel
    /// through `finish_close`, which clears `open` back to `false` uniformly.
    pub(crate) fn toggle_select_popup(&mut self, tree: &mut UiTree, select_id: NodeId) -> Vec<UiCommand> {
        let already_open = tree.node(select_id).is_some_and(|node| node.state.open);
        if already_open {
            self.close_overlay(tree, select_id)
        } else {
            self.open_overlay(tree, select_id, OverlayKind::SelectPopup, OverlayAnchor::Node(select_id));
            if let Some(node) = tree.node_mut(select_id) {
                node.state.open = true;
            }
            Vec::new()
        }
    }

    /// 🧹️ Clears `NodeFlags::OVERLAY`, and clears focus too if it was inside the closed overlay's
    /// subtree (dangling focus into a now-hidden subtree would otherwise route key events nowhere
    /// useful). `SelectPopup`'s `tree::WidgetState::open` is the popup's own show/hide bit
    /// (`paint::paint_select` reads it) — cleared here too, so every dismissal path (see
    /// `toggle_select_popup`'s doc comment) stays in sync with the overlay lifecycle uniformly.
    fn finish_close(&mut self, tree: &mut UiTree, overlay: OpenOverlay) -> Vec<UiCommand> {
        if let Some(node) = tree.node_mut(overlay.root) {
            node.flags.set(NodeFlags::OVERLAY, false);
            if overlay.kind == OverlayKind::SelectPopup {
                node.state.open = false;
            }
        }
        tree.mark_dirty(overlay.root, NodeFlags::DIRTY_PAINT);
        let mut out = vec![UiCommand::OverlayClosed { window_id: self.window_id.clone(), root: overlay.root, kind: overlay.kind }];
        if let Some(focused) = self.focus.focused {
            if is_descendant(tree, focused, overlay.root) {
                self.focus.clear_focus(tree);
                out.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
            }
        }
        out
    }

    /// 👆️ If the topmost overlay dismisses on outside-press and `(x, y)` lands outside its subtree,
    /// closes it and returns the resulting commands — the caller must swallow the press (not route it
    /// any further) when this returns `Some`.
    fn dismiss_topmost_if_outside_press(&mut self, tree: &mut UiTree, x: f32, y: f32) -> Option<Vec<UiCommand>> {
        let top = self.overlays.topmost()?;
        if !top.dismiss.outside_press_swallow {
            return None;
        }
        let overlay_root = top.root;
        if hit_test_subtree(tree, overlay_root, x, y).is_some() {
            return None;
        }
        Some(self.close_topmost_overlay(tree))
    }

    /// 🖱️ `Tooltip`-only: closes the topmost overlay once the pointer leaves both its anchor and its
    /// own bounds. See `DismissPolicy::hover_out_delay_seconds` for why this is immediate, not
    /// debounced.
    fn maybe_dismiss_tooltip_on_hover_out(&mut self, tree: &mut UiTree, x: f32, y: f32) -> Vec<UiCommand> {
        let Some(top) = self.overlays.topmost() else { return Vec::new() };
        if top.kind != OverlayKind::Tooltip {
            return Vec::new();
        }
        let overlay_root = top.root;
        let anchor = top.anchor;
        let inside_overlay = node_abs_rect(tree, overlay_root).is_some_and(|rect| rect.contains(x, y));
        let inside_anchor = match anchor {
            OverlayAnchor::Node(id) => node_abs_rect(tree, id).is_some_and(|rect| rect.contains(x, y)),
            OverlayAnchor::Point { .. } => false,
        };
        if inside_overlay || inside_anchor {
            return Vec::new();
        }
        self.close_topmost_overlay(tree)
    }
    //#endregion 🔖️OverlayApi

    //#region 🔖️DragDropApi
    pub(crate) fn set_drag_payload(&mut self, node: NodeId, payload: DragPayload) {
        self.drag_payloads.insert(node, payload);
    }

    #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn clear_drag_payload(&mut self, node: NodeId) {
        self.drag_payloads.remove(&node);
    }

    #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn set_drop_accept(&mut self, node: NodeId, predicate: impl Fn(&DragPayload) -> bool + Send + Sync + 'static) {
        self.drop_accept.insert(node, Box::new(predicate));
    }

    #[allow(dead_code, reason = "drag-drop registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn drag_session(&self) -> Option<&DragSession> {
        self.drag.as_ref()
    }

    /// 🫳️ Promotes a `Press` capture on a `drag_payloads`-registered node to `CaptureKind::Drag` once
    /// the pointer has moved past `DRAG_PROMOTE_THRESHOLD_SQ` from `press_origin`.
    fn maybe_promote_to_drag(&mut self, x: f32, y: f32) {
        let Some((id, CaptureKind::Press)) = self.capture.target else { return };
        let Some(payload) = self.drag_payloads.get(&id).cloned() else { return };
        let Some((origin_x, origin_y)) = self.press_origin else { return };
        if (x - origin_x).powi(2) + (y - origin_y).powi(2) < DRAG_PROMOTE_THRESHOLD_SQ {
            return;
        }
        self.capture.target = Some((id, CaptureKind::Drag));
        self.drag = Some(DragSession { source: id, payload, ghost: None, pointer_x: x, pointer_y: y, drop_target: None });
    }

    /// 🫳️ Live-updates the active `DragSession`'s pointer position and re-evaluates the drop target
    /// under it.
    fn update_drag(&mut self, tree: &UiTree, root: NodeId, x: f32, y: f32) {
        if let Some(drag) = self.drag.as_mut() {
            drag.pointer_x = x;
            drag.pointer_y = y;
        }
        let target = hit_test(tree, root, x, y).and_then(|hit| self.nearest_accepting_drop_target(tree, hit));
        if let Some(drag) = self.drag.as_mut() {
            drag.drop_target = target;
        }
    }

    /// 🎯️ Walks `from`'s bubble chain for the nearest `NodeFlags::DROP_TARGET` node whose
    /// `drop_accept` predicate (if any) accepts the active `DragSession`'s payload.
    fn nearest_accepting_drop_target(&self, tree: &UiTree, from: NodeId) -> Option<NodeId> {
        let mut found = None;
        bubble(tree, from, |id| {
            if !tree.node(id).is_some_and(|node| node.flags.contains(NodeFlags::DROP_TARGET)) {
                return false;
            }
            let accepts = match self.drop_accept.get(&id) {
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
    //#endregion 🔖️DragDropApi

    //#region 🔖️ScrollApi
    #[allow(dead_code, reason = "scroll-thumb registry accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn register_scroll_thumb(&mut self, thumb: NodeId, scrollable: NodeId, axis: ScrollAxis) {
        self.scroll_thumbs.insert(thumb, (scrollable, axis));
    }

    fn route_scroll(&mut self, tree: &mut UiTree, root: NodeId, x: f32, y: f32, delta_x: f32, delta_y: f32) {
        let Some(hit) = hit_test(tree, root, x, y) else { return };
        let Some(scrollable) = nearest_scrollable_ancestor(tree, hit) else { return };
        if let Some(node) = tree.node_mut(scrollable) {
            let (offset_x, offset_y) = node.state.scroll_offset;
            node.state.scroll_offset = ((offset_x + delta_x).max(0.0), (offset_y + delta_y).max(0.0));
        }
        tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
    }

    fn update_scroll_thumb(&mut self, tree: &mut UiTree, scrollable: NodeId, axis: ScrollAxis, x: f32, y: f32) {
        let Some((origin_x, origin_y, start_x, start_y)) = self.thumb_start else { return };
        let (delta_x, delta_y) = (x - origin_x, y - origin_y);
        let Some(node) = tree.node_mut(scrollable) else { return };
        node.state.scroll_offset = match axis {
            ScrollAxis::Horizontal => ((start_x + delta_x).max(0.0), start_y),
            ScrollAxis::Vertical => (start_x, (start_y + delta_y).max(0.0)),
        };
        tree.mark_dirty(scrollable, NodeFlags::DIRTY_PAINT);
    }
    //#endregion 🔖️ScrollApi

    //#region 🔖️EditApi
    fn route_text_insert(&mut self, tree: &mut UiTree, text: &str) {
        let Some(id) = self.focus.focused else { return };
        let Some(node) = tree.node_mut(id) else { return };
        let Some(edit) = node.state.edit.as_mut() else { return };
        insert_at_caret(edit, text);
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    fn route_ime(&mut self, tree: &mut UiTree, event: &ImeEvent) {
        let Some(id) = self.focus.focused else { return };
        let Some(node) = tree.node_mut(id) else { return };
        let Some(edit) = node.state.edit.as_mut() else { return };
        match event {
            ImeEvent::Start => edit.composition = Some(String::new()),
            ImeEvent::Update { text, .. } => edit.composition = Some(text.clone()),
            ImeEvent::Commit { text } => {
                edit.composition = None;
                insert_at_caret(edit, text);
            }
            ImeEvent::Cancel => edit.composition = None,
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// ⌨️ Caret motion (with `Shift` extending the selection), `Home`/`End`, `Backspace`/`Delete`,
    /// and clipboard shortcuts for the focused node's `EditState`. A no-operation if nothing is focused or
    /// the focused node has no `EditState` (isn't a `UiNode::Input`, or hasn't been focused since
    /// `FocusState::set_focus` seeded one).
    fn route_edit_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers) -> Vec<UiCommand> {
        let mut out = Vec::new();
        let Some(id) = self.focus.focused else { return out };
        let Some(node) = tree.node_mut(id) else { return out };
        let Some(edit) = node.state.edit.as_mut() else { return out };
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
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    out.push(UiCommand::ClipboardCopy { window_id: self.window_id.clone(), text: edit.text[start..end].to_string() });
                }
                return out;
            }
            "x" | "X" if modifiers.ctrl || modifiers.meta => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    out.push(UiCommand::ClipboardCut { window_id: self.window_id.clone(), text: edit.text[start..end].to_string() });
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                } else {
                    return out;
                }
            }
            "v" | "V" if modifiers.ctrl || modifiers.meta => {
                out.push(UiCommand::ClipboardPasteRequested { window_id: self.window_id.clone() });
                return out;
            }
            _ => return out,
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        out
    }
    //#endregion 🔖️EditApi

    //#region 🔖️CursorApi
    #[allow(dead_code, reason = "cursor-state accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn hovered(&self) -> Option<NodeId> {
        self.hovered
    }

    #[allow(dead_code, reason = "cursor-state accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn capture(&self) -> Option<(NodeId, CaptureKind)> {
        self.capture.target
    }

    /// 🎯️ Read-only: whether this window's retained content currently holds keyboard focus — see
    /// `engine::Ui::window_has_focus` (its only caller), added for the `w2-input-wiring` host-side
    /// focus arbitration (content vs. chrome routing, `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`).
    pub(crate) fn is_focused(&self) -> bool {
        self.focus.focused.is_some()
    }
    //#endregion 🔖️CursorApi

    /// 🧹️ Drops registry entries (`drag_payloads`/`drop_accept`/`scroll_thumbs`) keyed by a `NodeId`
    /// `reconcile` has since removed from `tree` — generation-tagged `NodeId`s (see `arena`'s own doc
    /// comment) make stale entries harmless to *use* (they simply never match a live node again), but
    /// this keeps the maps from growing unboundedly across a long session's worth of churn.
    fn prune_dead_registrations(&mut self, tree: &UiTree) {
        self.drag_payloads.retain(|id, _| tree.contains(*id));
        self.drop_accept.retain(|id, _| tree.contains(*id));
        self.scroll_thumbs.retain(|thumb, (scrollable, _)| tree.contains(*thumb) && tree.contains(*scrollable));
    }

    /// 🚦️ Resolves the event's target (capture target if captured, else `hit_test`), updates
    /// interaction flags, and returns any `UiCommand`s the event produced.
    pub(crate) fn dispatch(&mut self, tree: &mut UiTree, root: NodeId, event: &UiEvent) -> Vec<UiCommand> {
        self.prune_dead_registrations(tree);
        let mut commands = Vec::new();
        match event {
            UiEvent::PointerMove { x, y } => {
                self.maybe_promote_to_drag(*x, *y);
                match self.capture.target {
                    Some((_, CaptureKind::Drag)) => self.update_drag(tree, root, *x, *y),
                    Some((scrollable, CaptureKind::ScrollThumb(axis))) => self.update_scroll_thumb(tree, scrollable, axis, *x, *y),
                    _ => {}
                }
                let target = self.resolve_target(tree, root, *x, *y);
                if let Some(id) = target {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                }
                commands.extend(self.update_hover(tree, target));
                commands.extend(self.maybe_dismiss_tooltip_on_hover_out(tree, *x, *y));
            }
            UiEvent::PointerDown { x, y, .. } => {
                if let Some(dismissed) = self.dismiss_topmost_if_outside_press(tree, *x, *y) {
                    return dismissed;
                }
                self.press_origin = Some((*x, *y));
                let target = hit_test(tree, root, *x, *y);
                commands.extend(self.update_hover(tree, target));
                if let Some(id) = target {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                    if let Some(&(scrollable, axis)) = self.scroll_thumbs.get(&id) {
                        let offset = tree.node(scrollable).map(|node| node.state.scroll_offset).unwrap_or_default();
                        self.capture.target = Some((scrollable, CaptureKind::ScrollThumb(axis)));
                        self.thumb_start = Some((*x, *y, offset.0, offset.1));
                    } else {
                        if let Some(node) = tree.node_mut(id) {
                            node.flags.set(NodeFlags::ACTIVE, true);
                        }
                        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
                        self.capture.target = Some((id, CaptureKind::Press));
                        let focusable = tree.node(id).is_some_and(|node| is_focusable(&node.spec.0));
                        if focusable {
                            self.focus.set_focus(tree, Some(id));
                            commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: Some(id) });
                        }
                        // 🫳️ W2 wiring: a `Tree` row's `draggable`/`drag_data` (re-derived by key —
                        // see `find_tree_item_spec`) registers this press as a promotable drag
                        // source, exactly like a widget spec would call `set_drag_payload` itself if
                        // `UiStackNode` had room for the field (it doesn't — see that fn's own doc).
                        if let Some(item) = find_tree_item_spec(tree, id) {
                            if item.draggable.unwrap_or(false) {
                                self.set_drag_payload(id, item.drag_data.clone().unwrap_or_default());
                            }
                        }
                    }
                } else {
                    self.focus.clear_focus(tree);
                    commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: None });
                }
            }
            UiEvent::PointerUp { x, y, .. } => {
                if let Some((active_id, kind)) = self.capture.release() {
                    match kind {
                        CaptureKind::Press => {
                            if let Some(node) = tree.node_mut(active_id) {
                                node.flags.set(NodeFlags::ACTIVE, false);
                            }
                            tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
                            if hit_test(tree, root, *x, *y) == Some(active_id) {
                                // 🔽️🎴️ W2 wiring: `Select` toggles its popup (`toggle_select_popup`);
                                // a `Button` (this covers `Select`'s own synthesized item rows too —
                                // see `reconcile::children_of`'s `Select` arm — since they're plain
                                // `UiNode::Button`s) fires its action, additionally closing an open
                                // `SelectPopup` if this button *is* one of that popup's rows (picking
                                // an item closes the popup, per `toggle_select_popup`'s doc comment);
                                // a `Stack` with `activate` set fires that action (see
                                // `paint::paint_stack_frame`'s matching visual for the same field).
                                let is_select = tree.node(active_id).is_some_and(|node| matches!(node.spec.0, UiNode::Select(_)));
                                if is_select {
                                    commands.extend(self.toggle_select_popup(tree, active_id));
                                } else {
                                    let fired = tree.node(active_id).and_then(|node| match &node.spec.0 {
                                        UiNode::Button(button) => Some((button.action.clone(), node.parent)),
                                        UiNode::Stack(stack) => stack.activate.clone().map(|action| (action, None)),
                                        _ => None,
                                    });
                                    if let Some((action, parent)) = fired {
                                        commands.push(UiCommand::App { window_id: self.window_id.clone(), action });
                                        if let Some(parent) = parent {
                                            let picked_from_open_select = self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::SelectPopup && overlay.root == parent);
                                            if picked_from_open_select {
                                                commands.extend(self.close_topmost_overlay(tree));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        CaptureKind::Drag => {
                            if let Some(node) = tree.node_mut(active_id) {
                                node.flags.set(NodeFlags::ACTIVE, false);
                            }
                            tree.mark_dirty(active_id, NodeFlags::DIRTY_PAINT);
                            if let Some(drag) = self.drag.take() {
                                commands.push(match drag.drop_target {
                                    Some(target) => UiCommand::DropCommitted { window_id: self.window_id.clone(), source: drag.source, target, payload: drag.payload },
                                    None => UiCommand::DropCancelled { window_id: self.window_id.clone(), source: drag.source },
                                });
                            }
                        }
                        CaptureKind::ScrollThumb(_) => {
                            self.thumb_start = None;
                        }
                    }
                }
                let target = self.resolve_target(tree, root, *x, *y);
                if let Some(id) = target {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                }
                commands.extend(self.update_hover(tree, target));
            }
            UiEvent::KeyDown { key, modifiers } => {
                if key == "Escape" {
                    commands.extend(self.close_topmost_overlay(tree));
                } else if key == "Tab" {
                    let scope = self.overlays.topmost_focus_trap_root().unwrap_or(root);
                    if modifiers.shift {
                        self.focus.focus_prev(tree, scope);
                    } else {
                        self.focus.focus_next(tree, scope);
                    }
                    commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: self.focus.focused });
                } else {
                    commands.extend(self.route_edit_key(tree, key, *modifiers));
                }
            }
            UiEvent::KeyUp { .. } => {}
            UiEvent::TextInput { text } => self.route_text_insert(tree, text),
            UiEvent::Paste { text } => self.route_text_insert(tree, text),
            UiEvent::Ime(ime_event) => self.route_ime(tree, ime_event),
            UiEvent::Scroll { x, y, delta_x, delta_y } => {
                if let Some(id) = hit_test(tree, root, *x, *y) {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                }
                self.route_scroll(tree, root, *x, *y, *delta_x, *delta_y);
            }
        }
        commands
    }

    /// 🎬️ If `id` is a `ComponentScene` leaf, resolves its `SurfaceKind`/absolute rect (the same
    /// ancestor-offset accumulation `scene_slots::collect_scene_slots`/`hit_test_node`/
    /// `paint::paint_node` each do independently — not reusing `collect_scene_slots` itself, since
    /// that walks the WHOLE tree per call and this runs once per real input event) and builds the
    /// `UiCommand::Scene` the host should route into that surface's per-`SurfaceKind` input handler.
    fn scene_command(&self, tree: &UiTree, id: NodeId, event: &UiEvent) -> Option<UiCommand> {
        let node = tree.node(id)?;
        let layout = tree.accepted_layout(id)?;
        let UiNode::ComponentScene(scene) = &node.spec.0 else { return None };
        let mut x = layout.x;
        let mut y = layout.y;
        let mut current = node.parent;
        while let Some(parent_id) = current {
            let parent = tree.node(parent_id)?;
            let parent_layout = tree.accepted_layout(parent_id)?;
            x += parent_layout.x;
            y += parent_layout.y;
            current = parent.parent;
        }
        Some(UiCommand::Scene { window_id: self.window_id.clone(), node: id, surface_id: scene.surface_id.clone(), kind: scene.component_kind, rect: Rect::new(x, y, layout.width, layout.height), event: event.clone() })
    }
}
//#endregion 🔖️UiCommand

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wgpu::component::ui::{UiButtonNode, UiComponentSceneNode, UiInputNode, UiPresence, UiSelectItem, UiSelectNode, UiSeparatorNode, UiStackNode, UiTextNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
    use crate::wgpu::tree::{Node, NodeKey, WidgetSpec};
    use crate::wgpu::IconName;
    use crate::wgpu::Label;

    fn action() -> ActionDescriptor {
        ActionDescriptor { controller_id: "ctrl".into(), action: "go".into(), args: None }
    }

    fn select_ui(id: &str, value: &str) -> UiNode {
        UiNode::Select(UiSelectNode {
            id: id.into(),
            value: value.into(),
            items: vec![UiSelectItem { value: "a".into(), label: Label::data("A") }, UiSelectItem { value: "b".into(), label: Label::data("B") }],
            placeholder: None,
            on_change: action(),
            presence: UiPresence::default(),
            menu: None,
        })
    }

    fn tree_ui(sections: Vec<UiTreeSectionNode>) -> UiNode {
        UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), drop_action: None, menu: None, interaction_domain: None })
    }

    /// 🌳️ Manually inserts a `Tree` row `Stack` (mirroring `reconcile::tree_item_row`'s synthesized
    /// shape/key exactly) as a retained child of `tree_id` — these tests build the retained tree by
    /// hand (like every other test in this module, via `leaf`), so `reconcile` never actually runs;
    /// this stand-in keeps the row's key (`NodeKey::Explicit(item.id)`) and geometry consistent with
    /// what `paint::sync_tree_row_layout` would have written.
    fn insert_tree_row(tree: &mut UiTree, tree_id: NodeId, item_id: &str, rect: (f32, f32, f32, f32)) -> NodeId {
        let spec =
            UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some(item_id.into()), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None });
        let id = tree.insert_child(Some(tree_id), Node::new(NodeKey::Explicit(item_id.into()), WidgetSpec(spec)));
        let bucket = tree.node_mut(id).unwrap();
        bucket.layout.x = rect.0;
        bucket.layout.y = rect.1;
        bucket.layout.width = rect.2;
        bucket.layout.height = rect.3;
        id
    }

    fn input_ui(id: &str, value: &str) -> UiNode {
        UiNode::Input(UiInputNode { id: id.into(), input_kind: "text".into(), value: value.into(), placeholder: None, commit: None, min: None, max: None, step: None, accept: None, on_change: action(), presence: UiPresence::default(), menu: None })
    }

    fn stack_ui() -> UiNode {
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
    }

    fn text_ui(value: &str) -> UiNode {
        UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
    }

    fn separator_ui() -> UiNode {
        UiNode::Separator(UiSeparatorNode { presence: UiPresence::default(), menu: None })
    }

    fn button_ui(id: &str) -> UiNode {
        UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: IconName::CircleDot, label: Label::data(id), action: action(), style: None, presence: UiPresence::default(), menu: None })
    }

    fn leaf(tree: &mut UiTree, parent: Option<NodeId>, ordinal: u32, node: UiNode, rect: (f32, f32, f32, f32)) -> NodeId {
        let id = tree.insert_child(parent, Node::new(NodeKey::Positional(ordinal, ordinal), WidgetSpec(node)));
        let bucket = tree.node_mut(id).unwrap();
        bucket.layout.x = rect.0;
        bucket.layout.y = rect.1;
        bucket.layout.width = rect.2;
        bucket.layout.height = rect.3;
        id
    }

    fn set_flag(tree: &mut UiTree, id: NodeId, flag: NodeFlags) {
        tree.node_mut(id).unwrap().flags.set(flag, true);
    }

    #[test]
    fn hit_test_finds_the_topmost_of_two_non_overlapping_siblings() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let left = leaf(&mut tree, Some(root), 1, text_ui("left"), (0.0, 0.0, 100.0, 100.0));
        let right = leaf(&mut tree, Some(root), 2, text_ui("right"), (100.0, 0.0, 100.0, 100.0));

        assert_eq!(hit_test(&tree, root, 50.0, 50.0), Some(left));
        assert_eq!(hit_test(&tree, root, 150.0, 50.0), Some(right));
    }

    #[test]
    fn hit_test_respects_clips_children_pruning() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let clipper = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 50.0, 50.0));
        set_flag(&mut tree, clipper, NodeFlags::CLIPS_CHILDREN);
        // child's own rect extends far outside the clipper's 50x50 bounds.
        let overflowing_child = leaf(&mut tree, Some(clipper), 1, text_ui("overflow"), (0.0, 0.0, 500.0, 500.0));

        assert_eq!(hit_test(&tree, root, 400.0, 400.0), None, "point outside the clipper must not match the overflowing child");
        assert_eq!(hit_test(&tree, root, 10.0, 10.0), Some(overflowing_child), "inside the clip bounds the child still matches");
    }

    #[test]
    fn hit_test_skips_hit_transparent_node_but_still_matches_its_children() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let overlay_glass = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        set_flag(&mut tree, overlay_glass, NodeFlags::HIT_TRANSPARENT);
        let child = leaf(&mut tree, Some(overlay_glass), 1, text_ui("under-glass"), (10.0, 10.0, 50.0, 50.0));

        assert_eq!(hit_test(&tree, root, 30.0, 30.0), Some(child));
        assert_eq!(hit_test(&tree, root, 150.0, 150.0), None, "hit-transparent node itself must never match outside its children");
    }

    #[test]
    fn capture_routes_move_and_up_to_the_captured_node_regardless_of_pointer_position() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let a = leaf(&mut tree, Some(root), 1, separator_ui(), (0.0, 0.0, 100.0, 100.0));
        let _b = leaf(&mut tree, Some(root), 2, separator_ui(), (100.0, 0.0, 100.0, 100.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 50.0, y: 50.0, button: PointerButton::Primary });
        assert_eq!(router.capture.target.map(|(id, _)| id), Some(a));

        // pointer moved far outside `a`'s bounds and into `b`'s — capture must still target `a`.
        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 150.0, y: 50.0 });
        assert_eq!(router.hovered, Some(a));

        router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 150.0, y: 50.0, button: PointerButton::Primary });
        assert_eq!(router.capture.target, None, "capture releases on PointerUp");
    }

    #[test]
    fn focus_next_and_prev_cycle_only_through_focusable_nodes() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 100.0));
        leaf(&mut tree, Some(root), 1, text_ui("not focusable"), (0.0, 0.0, 50.0, 20.0));
        let button_a = leaf(&mut tree, Some(root), 2, button_ui("a"), (50.0, 0.0, 50.0, 20.0));
        leaf(&mut tree, Some(root), 3, separator_ui(), (100.0, 0.0, 50.0, 20.0));
        let button_b = leaf(&mut tree, Some(root), 4, button_ui("b"), (150.0, 0.0, 50.0, 20.0));
        let mut focus = FocusState::new();

        focus.focus_next(&mut tree, root);
        assert_eq!(focus.focused, Some(button_a));
        focus.focus_next(&mut tree, root);
        assert_eq!(focus.focused, Some(button_b));
        focus.focus_next(&mut tree, root);
        assert_eq!(focus.focused, Some(button_a), "cycles back to the first focusable node");

        focus.focus_prev(&mut tree, root);
        assert_eq!(focus.focused, Some(button_b), "wraps to the last focusable node going backwards");
    }

    #[test]
    fn set_focus_flips_the_focused_flag_in_both_directions() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
        let a = leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
        let b = leaf(&mut tree, Some(root), 2, button_ui("b"), (0.0, 20.0, 50.0, 20.0));
        let mut focus = FocusState::new();

        focus.set_focus(&mut tree, Some(a));
        assert!(tree.node(a).unwrap().flags.contains(NodeFlags::FOCUSED));
        assert!(!tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED));

        focus.set_focus(&mut tree, Some(b));
        assert!(!tree.node(a).unwrap().flags.contains(NodeFlags::FOCUSED), "moving focus away must clear the old node's flag");
        assert!(tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED));

        focus.clear_focus(&mut tree);
        assert!(!tree.node(b).unwrap().flags.contains(NodeFlags::FOCUSED), "clearing focus must clear the flag, not just the router's own field");
    }

    #[test]
    fn clicking_a_button_emits_its_action_descriptor_as_a_ui_command() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
        let button = leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 100.0, 40.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

        let expected = action();
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { window_id, action } if window_id == "main" && *action == expected)));
        let _ = button;
    }

    #[test]
    fn releasing_off_the_captured_button_does_not_fire_its_action() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
        leaf(&mut tree, Some(root), 1, button_ui("go"), (0.0, 0.0, 40.0, 40.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 90.0, y: 90.0, button: PointerButton::Primary });

        assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "release outside the pressed button must not fire its action");
    }

    #[test]
    fn bubble_stops_when_a_handler_returns_true() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
        let mid = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 100.0));
        let leaf_node = leaf(&mut tree, Some(mid), 1, text_ui("leaf"), (0.0, 0.0, 20.0, 20.0));

        let mut visited = Vec::new();
        bubble(&tree, leaf_node, |id| {
            visited.push(id);
            id == mid
        });

        assert_eq!(visited, vec![leaf_node, mid], "bubbling must stop at `mid` and never reach `root`");
    }

    //#region 🔖️OverlayTests
    #[test]
    fn overlay_open_flags_the_node_and_close_clears_it_and_emits_overlay_closed() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let popup = leaf(&mut tree, Some(root), 1, stack_ui(), (10.0, 10.0, 50.0, 50.0));
        let mut router = EventRouter::new("main");

        router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));
        assert!(tree.node(popup).unwrap().flags.contains(NodeFlags::OVERLAY));
        assert_eq!(router.topmost_overlay().map(|overlay| overlay.kind), Some(OverlayKind::SelectPopup));

        let commands = router.close_topmost_overlay(&mut tree);
        assert!(!tree.node(popup).unwrap().flags.contains(NodeFlags::OVERLAY));
        assert!(router.topmost_overlay().is_none());
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { root, kind, .. } if *root == popup && *kind == OverlayKind::SelectPopup)));
    }

    #[test]
    fn pointer_down_outside_a_dismissable_overlay_closes_it_and_swallows_the_press() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, button_ui("underneath"), (0.0, 0.0, 200.0, 200.0));
        let popup = leaf(&mut tree, Some(root), 2, stack_ui(), (10.0, 10.0, 50.0, 50.0));
        leaf(&mut tree, Some(popup), 1, text_ui("item"), (0.0, 0.0, 50.0, 50.0));
        let mut router = EventRouter::new("main");
        router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary });

        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { .. })), "outside press must close the overlay");
        assert!(router.topmost_overlay().is_none());
        assert_eq!(router.capture(), None, "the outside press must be swallowed, not routed to whatever's underneath");
    }

    #[test]
    fn pointer_down_inside_a_dismissable_overlay_does_not_close_it() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let popup = leaf(&mut tree, Some(root), 1, stack_ui(), (10.0, 10.0, 50.0, 50.0));
        leaf(&mut tree, Some(popup), 1, text_ui("item"), (0.0, 0.0, 50.0, 50.0));
        let mut router = EventRouter::new("main");
        router.open_overlay(&mut tree, popup, OverlayKind::SelectPopup, OverlayAnchor::Node(root));

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Primary });

        assert!(commands.iter().all(|cmd| !matches!(cmd, UiCommand::OverlayClosed { .. })), "a press inside the overlay must not dismiss it");
        assert!(router.topmost_overlay().is_some());
    }

    #[test]
    fn escape_closes_only_the_topmost_of_two_open_overlays() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let menu = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 50.0, 50.0));
        let submenu = leaf(&mut tree, Some(root), 2, stack_ui(), (60.0, 0.0, 50.0, 50.0));
        let mut router = EventRouter::new("main");
        router.open_overlay(&mut tree, menu, OverlayKind::ContextMenu, OverlayAnchor::Node(root));
        router.open_overlay(&mut tree, submenu, OverlayKind::ContextMenu, OverlayAnchor::Node(menu));

        let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Escape".into(), modifiers: EventModifiers::default() });

        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { root, .. } if *root == submenu)));
        assert_eq!(router.topmost_overlay().map(|overlay| overlay.root), Some(menu), "only the topmost overlay closes on Escape");
    }

    #[test]
    fn tab_focus_is_trapped_inside_an_open_dialog_overlay() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
        leaf(&mut tree, Some(root), 1, button_ui("a"), (0.0, 0.0, 50.0, 20.0));
        leaf(&mut tree, Some(root), 2, button_ui("b"), (50.0, 0.0, 50.0, 20.0));
        let dialog = leaf(&mut tree, Some(root), 3, stack_ui(), (100.0, 100.0, 100.0, 100.0));
        let button_c = leaf(&mut tree, Some(dialog), 1, button_ui("c"), (0.0, 0.0, 50.0, 20.0));
        let button_d = leaf(&mut tree, Some(dialog), 2, button_ui("d"), (50.0, 0.0, 50.0, 20.0));
        let mut router = EventRouter::new("main");
        router.open_overlay(&mut tree, dialog, OverlayKind::Dialog, OverlayAnchor::Point { x: 0.0, y: 0.0 });

        let tab = || UiEvent::KeyDown { key: "Tab".into(), modifiers: EventModifiers::default() };
        router.dispatch(&mut tree, root, &tab());
        assert_eq!(router.focus.focused, Some(button_c));
        router.dispatch(&mut tree, root, &tab());
        assert_eq!(router.focus.focused, Some(button_d));
        router.dispatch(&mut tree, root, &tab());
        assert_eq!(router.focus.focused, Some(button_c), "focus-trapped Tab cycling must never reach button_a/button_b outside the dialog");
    }
    //#endregion 🔖️OverlayTests

    //#region 🔖️DragDropTests
    #[test]
    fn drag_session_promotes_after_threshold_and_commits_on_an_accepting_drop_target() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
        set_flag(&mut tree, source, NodeFlags::DRAG_SOURCE);
        let target = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 100.0, 50.0, 50.0));
        set_flag(&mut tree, target, NodeFlags::DROP_TARGET);
        leaf(&mut tree, Some(target), 1, text_ui("drop-here"), (0.0, 0.0, 50.0, 50.0));
        let mut router = EventRouter::new("main");
        let mut payload = DragPayload::new();
        payload.insert("application/x-semio-catalogue-item".into(), "{\"id\":\"abc\"}".into());
        router.set_drag_payload(source, payload.clone());

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
        assert_eq!(router.capture(), Some((source, CaptureKind::Press)), "a plain press must not immediately start a drag");

        // Small move under the promotion threshold: still just a Press.
        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 6.0, y: 6.0 });
        assert_eq!(router.capture(), Some((source, CaptureKind::Press)));

        // Move past the threshold and over the drop target: promotes to Drag and finds the target.
        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0 });
        assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));
        assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), Some(target));

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 120.0, y: 120.0, button: PointerButton::Primary });
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::DropCommitted { source: s, target: t, payload: p, .. } if *s == source && *t == target && *p == payload)));
        assert_eq!(router.capture(), None);
        assert!(router.drag_session().is_none());
    }

    #[test]
    fn drag_session_cancels_when_released_over_no_accepting_drop_target() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0_f32, 200.0));
        let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
        let mut router = EventRouter::new("main");
        router.set_drag_payload(source, DragPayload::new());

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0 });
        assert_eq!(router.capture(), Some((source, CaptureKind::Drag)));

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 190.0, y: 190.0, button: PointerButton::Primary });
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::DropCancelled { source: s, .. } if *s == source)));
    }

    #[test]
    fn a_drop_targets_accept_predicate_can_reject_the_active_payload() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let source = leaf(&mut tree, Some(root), 1, text_ui("drag-me"), (0.0, 0.0, 20.0, 20.0));
        let target = leaf(&mut tree, Some(root), 2, stack_ui(), (100.0, 100.0, 50.0, 50.0));
        set_flag(&mut tree, target, NodeFlags::DROP_TARGET);
        leaf(&mut tree, Some(target), 1, text_ui("drop-here"), (0.0, 0.0, 50.0, 50.0));
        let mut router = EventRouter::new("main");
        router.set_drag_payload(source, DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "x".to_string())]));
        router.set_drop_accept(target, |payload| payload.contains_key("application/x-semio-catalogue-item"));

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 5.0, y: 5.0, button: PointerButton::Primary });
        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 120.0, y: 120.0 });

        assert_eq!(router.drag_session().and_then(|drag| drag.drop_target), None, "the predicate must reject this payload's mime key");
    }
    //#endregion 🔖️DragDropTests

    //#region 🔖️ScrollTests
    #[test]
    fn scroll_routes_to_the_nearest_scrollable_ancestor_and_clamps_at_zero() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
        leaf(&mut tree, Some(root), 1, text_ui("content"), (10.0, 10.0, 20.0, 20.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: 30.0 });
        assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 30.0));

        router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 15.0, y: 15.0, delta_x: 0.0, delta_y: -100.0 });
        assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 0.0), "scroll offset must clamp at zero, not go negative");
    }

    #[test]
    fn scroll_thumb_capture_drags_the_scrollable_offset_along_its_registered_axis() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        set_flag(&mut tree, root, NodeFlags::SCROLLABLE);
        // A bare `Stack` is a `hit_test`-transparent pass-through container (see 🔖️HitTest's
        // `is_plain_container`) — the thumb needs to be a real leaf to be hit-testable itself.
        let thumb = leaf(&mut tree, Some(root), 1, separator_ui(), (190.0, 0.0, 10.0, 40.0));
        let mut router = EventRouter::new("main");
        router.register_scroll_thumb(thumb, root, ScrollAxis::Vertical);

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 195.0, y: 5.0, button: PointerButton::Primary });
        assert_eq!(router.capture(), Some((root, CaptureKind::ScrollThumb(ScrollAxis::Vertical))));

        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 195.0, y: 25.0 });
        assert_eq!(tree.node(root).unwrap().state.scroll_offset, (0.0, 20.0));

        router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 195.0, y: 25.0, button: PointerButton::Primary });
        assert_eq!(router.capture(), None);
    }
    //#endregion 🔖️ScrollTests

    //#region 🔖️EditStateTests
    #[test]
    fn focusing_an_input_seeds_edit_state_from_its_value_and_blur_clears_it() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let input = leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        assert_eq!(tree.node(input).unwrap().state.edit, Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 }));

        // clicking empty space blurs.
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary });
        assert_eq!(tree.node(input).unwrap().state.edit, None, "blur must relinquish the buffer so the declarative value governs again");
    }

    #[test]
    fn arrow_keys_move_the_caret_and_backspace_deletes_the_previous_char() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let input = router.focus.focused.unwrap();

        router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers::default() });
        assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().caret, 2);

        router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
        let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
        assert_eq!((edit.anchor, edit.caret), (2, 1), "shift+arrow extends the selection instead of collapsing it");

        router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Backspace".into(), modifiers: EventModifiers::default() });
        let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
        assert_eq!(edit.text, "ac", "backspace over a selection deletes the selected range");
        assert_eq!((edit.anchor, edit.caret), (1, 1));
    }

    #[test]
    fn character_insertion_replaces_the_selection() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let input = router.focus.focused.unwrap();

        router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
        router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
        router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "xyz".into() });

        let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
        assert_eq!(edit.text, "xyz");
        assert_eq!((edit.anchor, edit.caret), (3, 3));
    }

    #[test]
    fn copy_over_a_selection_emits_a_clipboard_command_without_mutating_the_buffer() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let input = router.focus.focused.unwrap();

        router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
        router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
        let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "c".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });

        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::ClipboardCopy { text, .. } if text == "hello")));
        assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().text, "hello", "copy must not mutate the buffer");
    }

    #[test]
    fn ime_commit_inserts_the_composed_text_and_clears_composition() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, input_ui("name", ""), (0.0, 0.0, 100.0, 20.0));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let input = router.focus.focused.unwrap();

        router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Start));
        router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Update { text: "ねこ".into(), cursor: 2 }));
        assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().composition.as_deref(), Some("ねこ"));

        router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Commit { text: "ねこ".into() }));
        let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
        assert_eq!(edit.text, "ねこ");
        assert_eq!(edit.composition, None);
    }
    //#endregion 🔖️EditStateTests

    //#region 🔖️HoverRevealTests
    #[test]
    fn hovering_a_leaf_marks_its_whole_ancestor_chain_hovered_and_clearing_hover_clears_it_all() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
        let row = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 100.0));
        let label = leaf(&mut tree, Some(row), 1, text_ui("item"), (0.0, 0.0, 50.0, 20.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0 });
        assert!(tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
        assert!(tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED), "an ancestor Stack row must observe hover too, for paint's reveal-on-hover");
        assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));

        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 500.0, y: 500.0 });
        assert!(!tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
        assert!(!tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED));
        assert!(!tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));
    }
    //#endregion 🔖️HoverRevealTests

    //#region 🔖️W2InteractivityTests
    // 🔽️🎴️🌳️ Tests for the wiring closed out per `.🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`'s W2
    // pass: `Select` popup open/close (`toggle_select_popup`/`finish_close`), `Stack`
    // `activate`/`drop_action` (`is_plain_stack_container`'s hit-test exception), and `Tree` row
    // `draggable` (`find_tree_item_spec`).

    #[test]
    fn clicking_a_select_opens_its_popup_and_clicking_again_closes_it() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
        assert!(tree.node(select).unwrap().state.open, "clicking a closed select should open its popup");
        assert!(tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY), "an open select's popup subtree should win hit-test priority over its siblings");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
        assert!(!tree.node(select).unwrap().state.open, "clicking an open select's trigger again should close its popup");
        assert!(!tree.node(select).unwrap().flags.contains(NodeFlags::OVERLAY));
    }

    #[test]
    fn a_press_outside_an_open_selects_popup_closes_it_and_swallows_the_press() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
        assert!(tree.node(select).unwrap().state.open);

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary });

        assert!(!tree.node(select).unwrap().state.open, "a press well outside the select and its popup should close it");
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::OverlayClosed { kind: OverlayKind::SelectPopup, .. })));
    }

    #[test]
    fn picking_a_selects_item_row_fires_its_action_and_closes_the_popup() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
        let row_b = leaf(&mut tree, Some(select), 1, button_ui("b"), (0.0, 32.0, 100.0, 24.0));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });
        assert!(tree.node(select).unwrap().state.open);

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 40.0, button: PointerButton::Primary });
        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 40.0, button: PointerButton::Primary });

        let expected = action();
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == expected)), "picking a row should fire its (merged) action");
        assert!(!tree.node(select).unwrap().state.open, "picking an item should close the popup, per toggle_select_popup's dismissal-paths doc comment");
        let _ = row_b;
    }

    fn activatable_stack_ui(action: ActionDescriptor) -> UiNode {
        UiNode::Stack(UiStackNode { direction: "vertical".into(), gap: None, padding: None, id: Some("card".into()), presence: UiPresence::default(), activate: Some(action), drop_action: None, drop_overlay: None, children: Vec::new(), menu: None })
    }

    #[test]
    fn clicking_an_activatable_stack_fires_its_activate_action() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let _card = leaf(&mut tree, Some(root), 1, activatable_stack_ui(action()), (0.0, 0.0, 100.0, 40.0));
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x: 10.0, y: 10.0, button: PointerButton::Primary });

        let expected = action();
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { action, .. } if *action == expected)), "clicking an activatable Stack should fire its `activate` action");
    }

    #[test]
    fn a_bare_stack_without_activate_or_drop_action_stays_a_hit_test_pass_through() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let _plain = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 40.0));

        assert_eq!(hit_test(&tree, root, 10.0, 10.0), None, "a bare Stack (no activate/drop_action/drag_source) must remain a hit-test pass-through");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a: `UiTreeItemNode.hoverAction`/
    /// `unhoverAction` are deleted, so a tree row's hover no longer dispatches an ad hoc per-item
    /// action here — the framework now owns hover per `UiTreeNode.interactionDomain`
    /// (`interactionHover`), wired at a layer above this retained-mode event router.
    #[test]
    fn hovering_a_tree_row_no_longer_fires_a_per_item_action() {
        let item = UiTreeItemNode::base("row1", Label::data("Row One"));
        let section = UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
        insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
        let mut router = EventRouter::new("main");

        let entered = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0 });
        assert!(entered.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "hovering a plain tree row must never fire a per-item action anymore");

        let left = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 190.0, y: 190.0 });
        assert!(left.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "leaving a plain tree row must never fire a per-item action anymore");
    }

    #[test]
    fn pressing_a_draggable_tree_row_then_moving_past_threshold_promotes_it_to_a_drag_session() {
        let mut item = UiTreeItemNode::base("row1", Label::data("Row One"));
        item.draggable = Some(true);
        let payload = DragPayload::from([("application/x-semio-tree-section-reorder".to_string(), "{}".to_string())]);
        item.drag_data = Some(payload.clone());
        let section = UiTreeSectionNode { id: "s1".into(), label: None, default_open: Some(true), presence: UiPresence::default(), items: vec![item] };

        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let tree_id = leaf(&mut tree, Some(root), 1, tree_ui(vec![section]), (0.0, 0.0, 200.0, 200.0));
        let row_id = insert_tree_row(&mut tree, tree_id, "row1", (0.0, 0.0, 200.0, 24.0));
        // `paint::sync_tree_row_layout` is what would normally flip this on (mirroring `item.draggable`)
        // — these tests build the retained tree by hand (no `paint_tree` call), so it's set directly.
        tree.node_mut(row_id).unwrap().flags.set(NodeFlags::DRAG_SOURCE, true);
        let mut router = EventRouter::new("main");

        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });
        assert_eq!(router.capture(), Some((row_id, CaptureKind::Press)), "the row must be a real hit-test target once DRAG_SOURCE-flagged");

        router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 30.0, y: 10.0 });
        let drag = router.drag_session().expect("moving past the promote threshold should start a DragSession for a draggable row");
        assert_eq!(drag.source, row_id);
        assert_eq!(drag.payload, payload);
    }
    //#endregion 🔖️W2InteractivityTests

    //#region 🔖️W4SceneCommandTests
    /// 🎬️ A minimal `ComponentScene` leaf — every optional per-`SurfaceKind` payload left `None`,
    /// mirroring `scene_slots::tests::scene`'s own fixture (this module can't reuse that one directly:
    /// it's private to the `scene_slots` submodule).
    fn component_scene_ui(surface_id: &str, kind: SurfaceKind) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: "ctrl".into(),
            component_kind: kind,
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        })
    }

    #[test]
    fn pointer_down_on_a_component_scene_leaf_emits_a_scene_command() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let scene_id = leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Canvas2d), (10.0, 10.0, 100.0, 80.0));
        let mut router = EventRouter::new("main");

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary });

        let scene_cmd = commands.iter().find_map(|cmd| match cmd {
            UiCommand::Scene { window_id, node, surface_id, kind, rect, event } => Some((window_id, node, surface_id, kind, rect, event)),
            _ => None,
        });
        let (window_id, node, surface_id, kind, rect, event) = scene_cmd.expect("pointer-down over a ComponentScene leaf should emit UiCommand::Scene");
        assert_eq!(window_id, "main");
        assert_eq!(*node, scene_id);
        assert_eq!(surface_id, "s1");
        assert_eq!(*kind, SurfaceKind::Canvas2d);
        assert_eq!(*rect, Rect::new(10.0, 10.0, 100.0, 80.0), "rect should be the leaf's own absolute layout rect");
        assert_eq!(*event, UiEvent::PointerDown { x: 20.0, y: 20.0, button: PointerButton::Secondary }, "the real event should be carried through verbatim, including its button");
    }

    #[test]
    fn pointer_down_outside_any_component_scene_leaf_emits_no_scene_command() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Canvas2d), (10.0, 10.0, 100.0, 80.0));
        let mut router = EventRouter::new("main");

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 150.0, y: 150.0, button: PointerButton::Primary });

        assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a press outside the scene's own rect should not emit UiCommand::Scene");
    }

    #[test]
    fn pointer_down_on_a_plain_button_emits_no_scene_command() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, button_ui("b1"), (0.0, 0.0, 50.0, 20.0));
        let mut router = EventRouter::new("main");

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary });

        assert!(!commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { .. })), "a plain widget leaf should never emit UiCommand::Scene");
    }

    #[test]
    fn pointer_move_over_a_component_scene_leaf_emits_a_scene_command() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::InkCanvas), (0.0, 0.0, 200.0, 200.0));
        let mut router = EventRouter::new("main");

        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 50.0, y: 50.0 });

        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { kind: SurfaceKind::InkCanvas, .. })), "moving over a ComponentScene leaf should emit UiCommand::Scene too, not just PointerDown/Up");
    }

    #[test]
    fn scroll_over_a_component_scene_leaf_emits_a_scene_command_and_still_routes_container_scroll() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        leaf(&mut tree, Some(root), 1, component_scene_ui("s1", SurfaceKind::Table), (0.0, 0.0, 200.0, 200.0));
        let mut router = EventRouter::new("main");

        let commands = router.dispatch(&mut tree, root, &UiEvent::Scroll { x: 50.0, y: 50.0, delta_x: 0.0, delta_y: 12.0 });

        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::Scene { kind: SurfaceKind::Table, event: UiEvent::Scroll { .. }, .. })), "wheel input over a ComponentScene leaf should emit UiCommand::Scene carrying the Scroll event");
    }

    #[test]
    fn a_component_scene_nested_under_a_container_resolves_its_absolute_rect() {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 300.0, 300.0));
        let container = leaf(&mut tree, Some(root), 1, stack_ui(), (20.0, 30.0, 250.0, 250.0));
        let scene_id = leaf(&mut tree, Some(container), 2, component_scene_ui("s1", SurfaceKind::Paint2d), (5.0, 5.0, 100.0, 100.0));
        let mut router = EventRouter::new("main");

        // Absolute position is (20+5, 30+5) = (25, 35); a point inside that rect must hit-test to the scene.
        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 40.0, y: 50.0, button: PointerButton::Primary });

        let rect = commands.iter().find_map(|cmd| match cmd {
            UiCommand::Scene { node, rect, .. } if *node == scene_id => Some(*rect),
            _ => None,
        });
        assert_eq!(rect, Some(Rect::new(25.0, 35.0, 100.0, 100.0)), "a nested scene's rect should accumulate every ancestor's own layout offset");
    }
    //#endregion 🔖️W4SceneCommandTests
}
// #endregion events
