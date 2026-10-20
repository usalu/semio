//! 🧭️ Event model and focus: the pointer, accessibility and IME events, the hit test with the tree item lookup, the capture state and the focus ring. See [`super`].

use super::*;

// #region events
//#region 🔖️UiEvent
/// 🖱️ Mouse button identity for `UiEvent::{PointerDown,PointerUp}`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
    Middle,
}

/// ⌨️ Modifier keys held during a keyboard OR pointer event. A minimal fresh type rather than reusing
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
///
/// 🖱️ The POINTER variants carry `modifiers` for the same reason the key variants do: React reads
/// `event.shiftKey`/`ctrlKey`/`metaKey`/`altKey` off the DOM `PointerEvent` itself
/// (`⚙️VirtualFileSystem/🟦️.tsx:520-521`'s `additiveKey`/`rangeKey`, `marqueeModeFromModifiers` in
/// `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:900`), so every merge-mode-from-modifiers decision downstream is a
/// property of the press, not of some separately tracked keyboard state.
///
/// 🩸️ They used to carry none, and the whole modifier half of selection was structurally
/// unreachable through this router: `UiCommand::Scene` handed the surface handlers an event with no
/// modifier field at all, so `vfs_selection_for_click(…, false, false)` was the only call that could
/// ever be written and shift-extend/ctrl-toggle could not be expressed on ANY list surface
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️audit-w14-scenes-residual.md` C1).
#[derive(Clone, Debug, PartialEq)]
pub enum UiEvent {
    PointerCancel,
    PointerDown {
        x: f32,
        y: f32,
        button: PointerButton,
        modifiers: EventModifiers,
    },
    PointerUp {
        x: f32,
        y: f32,
        button: PointerButton,
        modifiers: EventModifiers,
    },
    PointerMove {
        x: f32,
        y: f32,
        modifiers: EventModifiers,
    },
    Scroll {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
        modifiers: EventModifiers,
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

#[derive(Clone, Debug, PartialEq)]
pub enum AccessibilityUiEvent {
    Focus,
    Blur,
    Activate,
    Value(String),
}

/// 🈶️ One IME composition step — see `UiEvent::Ime`.
#[derive(Clone, Debug, PartialEq)]
pub enum ImeEvent {
    Start,
    /// 🔺️ `cursor` is the IME's own preedit-relative cursor, informational only (not routed into
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
pub fn hit_test(tree: &UiTree, root: NodeId, x: f32, y: f32) -> Option<NodeId> {
    hit_test_node(tree, root, 0.0, 0.0, x, y, false, None)
}

/// 🪟️ An OPEN floating overlay is tested at the placement it was painted at, never at its in-flow
/// position — one origin rule shared with the retained paint/hit walk (`UiTree::overlay_origins`).
pub(super) fn hit_test_node(tree: &UiTree, id: NodeId, origin_x: f32, origin_y: f32, x: f32, y: f32, reversed: bool, tree_metrics: Option<&TreeRowMetrics>) -> Option<NodeId> {
    let node = tree.node(id)?;
    let layout = tree.accepted_layout(id)?;
    let (origin_x, origin_y) = tree.overlay_walk_origin(id).unwrap_or((origin_x, origin_y));
    let abs_x = origin_x + layout.x;
    let abs_y = origin_y + layout.y;
    let inside = Rect::new(abs_x, abs_y, layout.width, layout.height).contains(x, y);
    if node.flags.contains(NodeFlags::CLIPS_CHILDREN) && !inside {
        return None;
    }
    let mut overlays: Vec<NodeId> = Vec::new();
    let mut normal: Vec<NodeId> = Vec::new();
    if tree.disclosure_open(id).unwrap_or(true) {
        for child in tree.children(id) {
            match tree.node(child) {
                Some(child_node) if child_node.flags.contains(NodeFlags::OVERLAY) => overlays.push(child),
                _ => normal.push(child),
            }
        }
    }
    let (child_x, child_y) = tree.child_walk_origin(id, (origin_x, origin_y))?;
    for child in overlays.into_iter().rev().chain(normal.into_iter().rev()) {
        if let Some(hit) = hit_test_node(tree, child, child_x, child_y, x, y, reversed, tree_metrics) {
            return Some(hit);
        }
    }
    let inside_self = if tree.disclosure_is_interactive(id) { disclosure_header_band(tree, id, Rect::new(abs_x, abs_y, layout.width, layout.height), reversed, tree_metrics).contains(x, y) } else { inside };
    let is_plain_container = is_plain_stack_container(tree, id, node);
    if inside_self && !node.flags.contains(NodeFlags::HIT_TRANSPARENT) && !is_plain_container {
        Some(id)
    } else {
        None
    }
}

fn disclosure_header_band(tree: &UiTree, id: NodeId, rect: Rect, reversed: bool, tree_metrics: Option<&TreeRowMetrics>) -> Rect {
    let tree_section = tree.node(id).and_then(|node| Some((tree.node(node.parent?)?, &node.key))).and_then(|(parent, key)| match (&parent.spec.0, key) {
        (UiNode::Tree(owner), NodeKey::Explicit(key)) => owner.sections.iter().find(|section| &section.id == key),
        _ => None,
    });
    let scoped_metrics = tree_metrics.map(|metrics| semio_framework_ui_wgpu_mounted_layout::retained_tree_row_metrics(tree, id, metrics));
    match (tree_section, scoped_metrics.as_ref()) {
        (Some(section), Some(metrics)) => tree_section_header_band(rect, tree_section_header_height(section, metrics), reversed),
        (None, Some(metrics)) if tree.authored_tree_item(id).is_some() => tree_section_header_band(rect, metrics.row_height, reversed),
        _ if matches!(tree.node(id).map(|node| &node.spec.0), Some(UiNode::Section(_))) => Rect::new(rect.x, rect.y, rect.w, semio_framework_ui_wgpu_mounted_layout::retained_section_title_height(tree, id).min(rect.h)),
        _ => Rect::new(rect.x, rect.y, rect.w, scoped_metrics.map_or(0.0, |metrics| metrics.row_height).min(rect.h)),
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
/// never an ad hoc per-item action. A tree or table row carrying Row-placed actions is a target too:
/// its action slots resolve in `EventRouter::pointer_row_action`.
fn is_plain_stack_container(tree: &UiTree, id: NodeId, node: &Node) -> bool {
    let UiNode::Stack(stack) = &node.spec.0 else { return false };
    stack.activate.is_none()
        && stack.drop_action.is_none()
        && !node.flags.contains(NodeFlags::DRAG_SOURCE)
        && !tree.disclosure_is_interactive(id)
        && !tree.authored_tree_item(id).and_then(|item| item.actions.as_deref()).is_some_and(|actions| actions.iter().any(|action| action.placement() == semio_framework_ui_declaration::ui::UiTreeActionPlacement::Row))
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
pub(super) fn find_tree_item_spec(tree: &UiTree, row: NodeId) -> Option<&UiTreeItemNode> {
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

/// 📐️ Uses the same scroll and overlay placement as the retained paint walk.
pub fn node_abs_rect(tree: &UiTree, id: NodeId) -> Option<Rect> {
    tree.absolute_rect(id)
}

/// 🎯️ Tests a placed subtree in window coordinates, including open overlays outside ancestor clips.
pub fn hit_test_subtree(tree: &UiTree, subtree_root: NodeId, x: f32, y: f32) -> Option<NodeId> {
    let rect = tree.absolute_rect(subtree_root)?;
    let layout = tree.accepted_layout(subtree_root)?;
    hit_test_node(tree, subtree_root, rect.x - layout.x, rect.y - layout.y, x, y, false, None)
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
pub(super) struct CaptureState {
    pub target: Option<(u64, NodeId, CaptureKind)>,
}

impl CaptureState {
    pub(super) fn release(&mut self, pointer_id: u64) -> Option<(NodeId, CaptureKind)> {
        if !self.target.is_some_and(|(owner, _, _)| owner == pointer_id) {
            return None;
        }
        self.target.take().map(|(_, node, kind)| (node, kind))
    }

    pub(super) fn release_any(&mut self) -> Option<(NodeId, CaptureKind)> {
        self.target.take().map(|(_, node, kind)| (node, kind))
    }
}
//#endregion 🔖️Capture

//#region 🔖️Focus
/// 🎯️ Which `UiNode` variants participate in Tab-order focus cycling.
fn is_focusable(node: &UiNode) -> bool {
    matches!(node, UiNode::Input(_) | UiNode::Button(_) | UiNode::Select(_) | UiNode::Toggle(_) | UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) | UiNode::IconSelect(_)) && node.presence().state != UiState::Disabled
}

pub(super) fn node_is_focusable(tree: &UiTree, id: NodeId) -> bool {
    tree.node(id).is_some_and(|node| (is_focusable(&node.spec.0) || tree.disclosure_is_interactive(id)) && node.spec.0.presence().state != UiState::Disabled)
}

fn collect_focusable(tree: &UiTree, id: NodeId, out: &mut Vec<NodeId>) {
    if let Some(_node) = tree.node(id) {
        if node_is_focusable(tree, id) {
            out.push(id);
        }
    }
    if !tree.disclosure_open(id).unwrap_or(true) {
        return;
    }
    for child in tree.children(id) {
        collect_focusable(tree, child, out);
    }
}

/// 🔦️ Currently-focused node plus a lazily-rebuilt document-order Tab cycle over focusable nodes.
pub struct FocusState {
    pub focused: Option<NodeId>,
    pub(super) tab_order: Vec<NodeId>,
}

impl FocusState {
    pub fn new() -> Self {
        Self { focused: None, tab_order: Vec::new() }
    }

    /// 🎯️ Sets/clears focus, flipping `NodeFlags::FOCUSED` on the old and new targets and marking
    /// both `DIRTY_PAINT` (a focus ring likely needs repainting) via `UiTree::mark_dirty`. A no-operation
    /// (no flag churn) when `node` already matches the current focus. Also owns `EditState`'s
    /// lifecycle: blurring a node clears its `WidgetState::edit` (the buffer relinquishes control,
    /// so the node's declarative `value` governs again on the next `apply_tree`); focusing a
    /// editable node for the first time seeds `edit` from that declarative `value` with the caret
    /// at the end — see `tree::WidgetState`'s own doc comment for why reconcile never clobbers this.
    ///
    /// 🎬️ Returns the BLURRED node — its id alongside its commit action — when that node commits on
    /// blur (`commits_on_blur` — React's `commitOnBlur`), so the caller can address the intent at the
    /// node that actually fired it rather than at whatever now holds focus. Losing focus is the only
    /// moment such a node's typed value is ever dispatched, and this is the one place blur happens.
    pub fn set_focus(&mut self, tree: &mut UiTree, node: Option<NodeId>, focus_visible: bool) -> Option<(NodeId, FiredAction)> {
        if self.focused == node {
            return None;
        }
        let mut committed = None;
        if let Some(previous) = self.focused {
            if let Some(previous_node) = tree.node_mut(previous) {
                previous_node.flags.set(NodeFlags::FOCUSED, false);
                previous_node.flags.set(NodeFlags::FOCUS_VISIBLE, false);
                previous_node.state.caret_visible = false;
                previous_node.state.slider_readout_click_at = None;
                let refusal = if matches!(previous_node.spec.0, UiNode::Slider(_)) { previous_node.state.number_refusal.clone() } else { edit_refusal(previous_node) };
                previous_node.state.number_refusal = refusal.clone();
                let buffer = if refusal.is_some() { None } else { previous_node.state.edit.take() };
                let press = matches!(previous_node.spec.0, UiNode::Input(_) | UiNode::NumberStepper(_)).then(|| previous_node.state.scrub_gesture.take()).flatten();
                let offered = previous_node.state.scrub_offered.take();
                if let Some(edit) = buffer.as_ref().filter(|_| commits_on_blur(&previous_node.spec.0)) {
                    committed = edit_commit_action(previous_node, &edit.text).map(|fired| (previous, fired));
                } else if let Some(gesture) = press {
                    let value = buffer.and_then(|edit| edit_commit_action(previous_node, &edit.text)).and_then(|mut fired| fired.input.take()).or(offered);
                    committed = match value {
                        Some(value) => press_release(&previous_node.spec.0, gesture, value),
                        None => press_cancel(&previous_node.spec.0, gesture, "blur"),
                    }
                    .map(|fired| (previous, fired));
                }
            }
            tree.mark_dirty(previous, NodeFlags::DIRTY_PAINT);
        }
        if let Some(next) = node {
            if let Some(next_node) = tree.node_mut(next) {
                next_node.flags.set(NodeFlags::FOCUSED, true);
                next_node.flags.set(NodeFlags::FOCUS_VISIBLE, focus_visible);
                if next_node.state.edit.is_none() {
                    if let Some(value) = editable_value(&next_node.spec.0) {
                        let caret = value.len();
                        next_node.state.edit = Some(EditState { text: value.to_string(), caret, anchor: caret, composition: None, scroll_x: 0.0 });
                    }
                }
                next_node.state.caret_visible = next_node.state.edit.is_some();
            }
            tree.mark_dirty(next, NodeFlags::DIRTY_PAINT);
        }
        self.focused = node;
        committed
    }

    pub fn clear_focus(&mut self, tree: &mut UiTree) -> Option<(NodeId, FiredAction)> {
        self.set_focus(tree, None, false)
    }

    fn rebuild_tab_order(&mut self, tree: &UiTree, root: NodeId) {
        self.tab_order.clear();
        collect_focusable(tree, root, &mut self.tab_order);
    }

    pub fn focus_next(&mut self, tree: &mut UiTree, root: NodeId) -> Option<(NodeId, FiredAction)> {
        self.rebuild_tab_order(tree, root);
        if self.tab_order.is_empty() {
            return self.set_focus(tree, None, true);
        }
        let next_index = match self.focused.and_then(|id| self.tab_order.iter().position(|&candidate| candidate == id)) {
            Some(index) => (index + 1) % self.tab_order.len(),
            None => 0,
        };
        self.set_focus(tree, Some(self.tab_order[next_index]), true)
    }

    pub fn focus_prev(&mut self, tree: &mut UiTree, root: NodeId) -> Option<(NodeId, FiredAction)> {
        self.rebuild_tab_order(tree, root);
        if self.tab_order.is_empty() {
            return self.set_focus(tree, None, true);
        }
        let previous_index = match self.focused.and_then(|id| self.tab_order.iter().position(|&candidate| candidate == id)) {
            Some(index) => (index + self.tab_order.len() - 1) % self.tab_order.len(),
            None => self.tab_order.len() - 1,
        };
        self.set_focus(tree, Some(self.tab_order[previous_index]), true)
    }
}
//#endregion 🔖️Focus
// #endregion events
