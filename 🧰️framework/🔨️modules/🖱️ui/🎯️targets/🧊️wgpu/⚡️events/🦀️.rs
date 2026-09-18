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
use crate::wgpu::component::ui::{SurfaceKind, UiNode, UiNumberStepperNode, UiSliderNode, UiState, UiTreeItemNode, UiTreeSectionNode};
use crate::wgpu::geometry::Rect;
use crate::wgpu::layout::{number_stepper_segments, ring_t_at, slider_value_at};
use crate::wgpu::select;
use crate::wgpu::tree::{EditState, Node, NodeFlags, NodeKey, UiTree};
use crate::wgpu::{intent_is_stale, UiIntentAddress, UiIntentCommand, UiIntentSequencer};
use dsl::DslValue;
use ui_contract::{FlowInline, Trigger, UiFlow};

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
    // 🪟️ An OPEN floating overlay is tested at the placement it was painted at, never at its in-flow
    // position — one origin rule shared with the retained paint/hit walk (`UiTree::overlay_origins`).
    let (origin_x, origin_y) = tree.overlay_walk_origin(id).unwrap_or((origin_x, origin_y));
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
    tree.node(id)?;
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
    matches!(node, UiNode::Input(_) | UiNode::Button(_) | UiNode::Select(_) | UiNode::Toggle(_) | UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) | UiNode::IconSelect(_)) && node.presence().state != UiState::Disabled
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
    /// editable node for the first time seeds `edit` from that declarative `value` with the caret
    /// at the end — see `tree::WidgetState`'s own doc comment for why reconcile never clobbers this.
    ///
    /// 🎬️ Returns the BLURRED node — its id alongside its commit action — when that node commits on
    /// blur (`commits_on_blur` — React's `commitOnBlur`), so the caller can address the intent at the
    /// node that actually fired it rather than at whatever now holds focus. Losing focus is the only
    /// moment such a node's typed value is ever dispatched, and this is the one place blur happens.
    fn set_focus(&mut self, tree: &mut UiTree, node: Option<NodeId>, focus_visible: bool) -> Option<(NodeId, FiredAction)> {
        if self.focused == node {
            return None;
        }
        let mut committed = None;
        if let Some(previous) = self.focused {
            if let Some(previous_node) = tree.node_mut(previous) {
                previous_node.flags.set(NodeFlags::FOCUSED, false);
                previous_node.flags.set(NodeFlags::FOCUS_VISIBLE, false);
                let buffer = previous_node.state.edit.take();
                if let Some(edit) = buffer {
                    if commits_on_blur(&previous_node.spec.0) {
                        committed = edit_commit_action(previous_node, &edit.text).map(|fired| (previous, fired));
                    }
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
            }
            tree.mark_dirty(next, NodeFlags::DIRTY_PAINT);
        }
        self.focused = node;
        committed
    }

    fn clear_focus(&mut self, tree: &mut UiTree) -> Option<(NodeId, FiredAction)> {
        self.set_focus(tree, None, false)
    }

    fn rebuild_tab_order(&mut self, tree: &UiTree, root: NodeId) {
        self.tab_order.clear();
        collect_focusable(tree, root, &mut self.tab_order);
    }

    fn focus_next(&mut self, tree: &mut UiTree, root: NodeId) -> Option<(NodeId, FiredAction)> {
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

    fn focus_prev(&mut self, tree: &mut UiTree, root: NodeId) -> Option<(NodeId, FiredAction)> {
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

//#region 🔖️Commit
// 🎬️ THE ONE commit authority for a retained document's form controls. Every `UiNode` variant that
// carries a value — `Input`, `Toggle`, `Slider`, `NumberStepper`, `Ring`, `IconSelect` — turns its
// own gesture into a `UiIntentCommand` addressed at the node that fired it (`EventRouter::
// build_intent`), and hands it to the host as `UiCommand::App`. The host admits it by `seq` and then
// collapses it to an address through `UiIntentCommand::descriptor`, which is where its authored args
// and its trigger payload finally merge — `publish_retained_action` stamps `windowId` and reserves it
// from there, exactly as it already did.
//
// 🌉️ The spec-carried `ActionDescriptor` never leaves this module on its own any more: it is the
// CARRIER for `controller_id`/name/authored args, and the intent is the DISPATCH. React draws the
// same line — `UiInterpreterContext.onAction` is reserved for its unowned scene hosts while every
// semantic control goes through `onIntent` (`🗣️Interpreter/🟦️.tsx`).
//
// 🩸️ Before this region the retained router documented "committing the edited value via `on_change`
// is not implemented", and the shell's own immediate-mode `commit_focused_input`/`stepper_metas`
// system (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) only ever saw ids minted by the CHROME's immediate-mode
// widget walk — never a retained document's. Editing a generation's parameters on wgpu was therefore
// fully cosmetic: a live caret, a live knob, and nothing reaching the guest (ticket
// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-wgpu-parity-2026-09-13.md` gaps #1/#6).
//
// Parity reference — React's own retained interpreter, NOT its older declarative-control path:
// `🗣️Interpreter/🟦️.tsx`'s `dispatchTrigger` → `UiDocumentStore::emitIntent` → `🛠️ShellHelpers/🟦️.tsx`'s
// `uiIntentPayload`/`uiInputField`. Two rules come from there and are reproduced verbatim below:
// a scalar payload is NAMED by its trigger (`delta` for `Trigger::Delta`, `value` for every other),
// and it is MERGED OVER the node's authored args rather than replacing them.

/// 🎬️ One gesture, before it is addressed: which `Trigger` fired, the spec-carried descriptor whose
/// `controller_id`/`action`/AUTHORED args it inherits, and the trigger's own payload — kept apart
/// from the args exactly as `ui_contract::UiIntent` keeps `args` and `input` apart.
/// [`UiIntentCommand::payload`] is the single place they merge, on the way to the host.
#[derive(Clone, Debug, PartialEq)]
struct FiredAction {
    trigger: Trigger,
    action: ActionDescriptor,
    input: Option<DslValue>,
}

/// 🎬️ One gesture against `action`'s binding. `None` when the node declares no binding for this
/// trigger — `reconcile::record_action_or_inert` marks that by an EMPTY action name, which is exactly
/// the condition React's `emitIntent` answers `undefined` for.
fn fired_action(action: &ActionDescriptor, trigger: Trigger, input: DslValue) -> Option<FiredAction> {
    if action.action.is_empty() {
        return None;
    }
    Some(FiredAction { trigger, action: action.clone(), input: Some(input) })
}

/// 🎬️ A gesture that carries no payload of its own (`Activate`, a focus commit's own re-dispatch).
fn bare_action(action: &ActionDescriptor, trigger: Trigger) -> Option<FiredAction> {
    if action.action.is_empty() {
        return None;
    }
    Some(FiredAction { trigger, action: action.clone(), input: None })
}

/// 🆔️ The versioned [`ui_contract::ActionId`] a bare descriptor stands for, for the nodes no document
/// published (chrome widgets, testkit trees) and for a binding the record no longer carries. The
/// descriptor's `controller_id` IS the id's scope — the same identity React's
/// `uiIntentToActionDescriptor` maps in the other direction.
fn descriptor_action_id(action: &ActionDescriptor) -> Option<ui_contract::ActionId> {
    ui_contract::ActionId::try_v1(&action.controller_id, &action.action)
}

/// ✍️ Which `UiNode` variants own a live `EditState` text buffer. `Input` is the obvious one;
/// `IconSelect`'s value IS its icon string, which React edits through the `IconSelector`'s own
/// textarea (`🎴️IconSelector/🟦️.tsx`'s `onEditorChange`), so the retained target edits it the same
/// way rather than inventing a second gesture for it.
fn editable_value(node: &UiNode) -> Option<&str> {
    match node {
        UiNode::Input(input) => Some(input.value.as_str()),
        UiNode::IconSelect(select) => Some(select.value.as_str()),
        _ => None,
    }
}

/// ⏎️ Whether an editable node commits only on Enter/blur (`Trigger::Commit`, `commit == "blur"`)
/// rather than on every keystroke (`Trigger::Change`) — React's `InputView`'s own `commitOnBlur`.
fn commits_on_blur(node: &UiNode) -> bool {
    matches!(node, UiNode::Input(input) if input.commit.as_deref() == Some("blur"))
}

/// ✍️ The action an editable node's CURRENT buffer commits: `{value: <text>}`, numeric for a
/// `Component::Input` of kind `number` (React's `InputView`: `kind === "number" ? Number(raw) : raw`),
/// and constrained by that input's own `min`/`max`/`step` before it leaves — the canvas twin of the
/// `<input type="number" min max step>` attributes React hands the browser to enforce
/// (`🗣️Interpreter/🟦️.tsx`'s `InputView`). The trigger is the node's own commit moment: `Commit` for a
/// `commit: "blur"` input, `Change` otherwise, exactly as `reconcile::input_commit_action` resolved
/// the binding it is firing.
fn edit_commit_action(node: &Node, text: &str) -> Option<FiredAction> {
    match &node.spec.0 {
        UiNode::Input(input) => {
            let value = if input.input_kind == "number" {
                DslValue::float(constrain_number_input(text.parse::<f64>().unwrap_or(f64::NAN), input.min, input.max, input.step))
            } else {
                DslValue::String(text.to_string())
            };
            let trigger = if commits_on_blur(&node.spec.0) { Trigger::Commit } else { Trigger::Change };
            fired_action(&input.on_change, trigger, value)
        }
        UiNode::IconSelect(select) => fired_action(&select.on_change, Trigger::Change, DslValue::String(text.to_string())),
        _ => None,
    }
}

/// 🔢️ Clamps to `[min, max]` and snaps to the nearest `step` off `min` — the three `InputProps`
/// constraints the wire has always carried and this target used to drop on the floor, letting a
/// number field commit `999` into a `max: 10` parameter that React's own DOM input refuses. A `NaN`
/// (an unparseable buffer) passes through untouched so the guest still sees the refusal rather than a
/// silently invented number.
pub fn constrain_number_input(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
    if value.is_nan() {
        return value;
    }
    let stepped = match step.filter(|step| *step > 0.0) {
        Some(step) => {
            let origin = min.unwrap_or(0.0);
            origin + ((value - origin) / step).round() * step
        }
        None => value,
    };
    stepped.max(min.unwrap_or(f64::NEG_INFINITY)).min(max.unwrap_or(f64::INFINITY))
}

/// 👆️ The action a press/drag at `(x, y)` over `bounds` commits for a pointer-valued control —
/// `Toggle` flips its own `presence.selected`, `Slider`/`Ring` read the gesture position off the
/// same geometry `paint` drew them at (`layout::{slider_value_at, ring_t_at}`), and a
/// `NumberStepper`'s outer thirds step its value (relative through `on_delta` when that binding
/// exists, absolute through `on_absolute` otherwise — React's `NumberStepperView` makes exactly that
/// choice). `None` for the stepper's own value segment, for an unbound trigger, and for every
/// variant whose press means something else (`Button`/`Select`/`Stack`, handled by the caller).
fn pointer_commit_action(node: &Node, bounds: Rect, x: f32, y: f32) -> Option<FiredAction> {
    match &node.spec.0 {
        UiNode::Toggle(toggle) => fired_action(&toggle.on_change, Trigger::Change, DslValue::Bool(!toggle.presence.selected)),
        UiNode::Slider(slider) => fired_action(&slider.on_change, Trigger::Change, DslValue::float(slider_value_at(bounds, x, slider.min, slider.max, slider.step))),
        UiNode::Ring(ring) => fired_action(&ring.on_change, Trigger::Change, DslValue::float(ring_t_at(bounds, x, y))),
        UiNode::NumberStepper(stepper) => {
            let [decrement, _value, increment] = number_stepper_segments(bounds);
            let sign = if decrement.contains(x, y) {
                -1.0
            } else if increment.contains(x, y) {
                1.0
            } else {
                return None;
            };
            // ➕️➖️ Only a node that DECLARES a `Delta` binding takes the relative path — React's
            // `NumberStepperView` supplies `onDelta` only when `record.bindings` contains one
            // (`🗣️Interpreter/🟦️.tsx`). Supplying it unconditionally sent every +/− click down a
            // trigger most programs never bind and swallowed the gesture (ticket
            // 26/09/02/PUZZLE-3D-END-TO-END wave B12). The node's own stamped bindings are the
            // authority when it has them; an unaddressed node falls back to the spec's own marker
            // (an empty action name), which `fired_action` already answers `None` for. The keyboard
            // half of the same gesture (`focused_value_key_activation`) shares this rule verbatim.
            number_stepper_fired(node, stepper, sign)
        }
        _ => None,
    }
}

/// 🎚️ The value one arrow/`Home`/`End`/`Page` key commits on a focused `Slider`, clamped to the
/// track — ported from React's own `Slider` (`🧱️elements/🎚️Slider/🟦️.tsx:383-396`):
/// `ArrowRight`/`ArrowUp`/`PageUp` step up, `ArrowLeft`/`ArrowDown`/`PageDown` step down,
/// `Home`/`End` jump to the ends, and `PageUp`/`PageDown` or any `Shift` chord move ten steps at
/// once. `None` for every other key, so it never swallows one.
fn slider_key_value(slider: &UiSliderNode, key: &str, shift: bool) -> Option<f64> {
    let delta = match key {
        "ArrowRight" | "ArrowUp" | "PageUp" => 1.0,
        "ArrowLeft" | "ArrowDown" | "PageDown" => -1.0,
        "Home" | "End" => 0.0,
        _ => return None,
    };
    let multiplier = if matches!(key, "PageUp" | "PageDown") || shift { SLIDER_PAGE_STEPS } else { 1.0 };
    let step = if slider.step > 0.0 { slider.step } else { 1.0 };
    let value = match key {
        "Home" => slider.min,
        "End" => slider.max,
        _ => slider.value + delta * step * multiplier,
    };
    Some(value.clamp(slider.min, slider.max))
}

/// 🎚️ How many steps a `PageUp`/`PageDown` (or a `Shift` chord) moves a slider — React's own
/// `multiplier` (`🧱️elements/🎚️Slider/🟦️.tsx:394`).
const SLIDER_PAGE_STEPS: f64 = 10.0;

/// ➕️➖️ One `NumberStepper` increment/decrement of `sign`, taking the relative `Delta` path only for
/// a node that DECLARES that binding — see the long note at this function's pointer-side caller.
fn number_stepper_fired(node: &Node, stepper: &UiNumberStepperNode, sign: f64) -> Option<FiredAction> {
    let binds_delta = node.intent.as_ref().map_or_else(|| !stepper.on_delta.action.is_empty(), |intent| intent.binds(Trigger::Delta));
    if binds_delta {
        fired_action(&stepper.on_delta, Trigger::Delta, DslValue::float(sign * stepper.step))
    } else {
        fired_action(&stepper.on_absolute, Trigger::Change, DslValue::float(stepper.value + sign * stepper.step))
    }
}

/// 🎚️ Whether a press on this variant keeps committing while the pointer drags — React delegates
/// `Slider`/`Ring` to controls that report every intermediate value, not only the release.
fn commits_while_dragging(node: &UiNode) -> bool {
    matches!(node, UiNode::Slider(_) | UiNode::Ring(_))
}
/// 📐️ One node's absolute painted rect: its own accepted layout plus every ancestor's origin — the
/// same accumulation `scene_slots::collect_scene_slots`/`hit_test_node`/`paint::paint_node` each
/// walk independently, resolved once here at dispatch time rather than by a whole-tree pass.
fn absolute_rect(tree: &UiTree, id: NodeId) -> Option<Rect> {
    tree.absolute_rect(id)
}
//#endregion 🔖️Commit

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

/// ⚓️ What an overlay is positioned relative to: an existing node (a `Select`'s trigger, a hovered
/// row) or a raw point (where a context menu was right-clicked, where the pointer was when a
/// tooltip's hover-delay fired).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayAnchor {
    Node(NodeId),
    Point { x: f32, y: f32 },
}

/// 🪟️ The anchored-overlay placement vocabulary and its positioner are the CONTRACT's, not this
/// target's: `ui_contract::🪟️overlay` owns the one port of React's `resolvePopoverPlacement` so this
/// module and `🖌️render/🖱️dispatch` cannot drift apart again (ticket 26/09/17 packet W2k).
pub use ui_contract::{
    resolve_centered_placement, resolve_select_inline_left, AnchoredPlacement, DismissPolicy, OverlayAlign, OverlayKind, OverlayPlacement, OverlayRect, OverlaySide,
    ResolvedOverlayPlacement, TOOLTIP_DWELL_SECONDS, TOOLTIP_HOVER_OUT_SECONDS,
};

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

    /// 🥞️ Bottom-to-top: the exact order a paint pass must draw them in.
    fn as_slice(&self) -> &[OpenOverlay] {
        &self.open
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
pub fn resolve_overlay_placement(tree: &UiTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement, flow: FlowInline) -> (f32, f32) {
    let resolved = resolve_overlay_placement_side(tree, anchor, content_size, viewport, placement, flow);
    (resolved.x, resolved.y)
}

/// 📐️ [`resolve_overlay_placement`] plus the side a collision flip settled on — what a paint pass
/// reads when it also wants React's `transformOrigin` equivalent. `flow` is React's own `rtl`
/// argument, read from the window's [`ui_contract::UiFlow`] (`Ui::window_flow`).
pub fn resolve_overlay_placement_side(tree: &UiTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement, flow: FlowInline) -> ResolvedOverlayPlacement {
    let anchor_rect = match anchor {
        OverlayAnchor::Node(id) => node_abs_rect(tree, id).unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0)),
        OverlayAnchor::Point { x, y } => Rect::new(x, y, 0.0, 0.0),
    };
    ui_contract::resolve_overlay_placement(overlay_rect(anchor_rect), content_size, viewport, placement, flow)
}

/// 📐️ This target's [`Rect`] as the contract's renderer-neutral [`OverlayRect`] — the ONE conversion
/// site, so `ui_contract`'s positioner stays free of any target's geometry type.
pub fn overlay_rect(rect: Rect) -> OverlayRect {
    OverlayRect::new(rect.x, rect.y, rect.w, rect.h)
}

/// 📍️ React's `resolvePopoverPlacement` over this target's [`Rect`] — a forwarding shim, because the
/// math itself lives exactly once, in `ui_contract::🪟️overlay`.
pub fn resolve_anchored_placement(anchor: Rect, content_size: (f32, f32), viewport: (f32, f32), placement: AnchoredPlacement, flow: FlowInline) -> ResolvedOverlayPlacement {
    ui_contract::resolve_anchored_placement(overlay_rect(anchor), content_size, viewport, placement, flow)
}


//#endregion 🔖️Overlay

//#region 🔖️Tooltip
// 💡️ React gets hover reveal from the DOM: `ChromeControlHint` arms a `setTimeout` on
// `onPointerEnter`/`onFocusCapture` and opens a portalled `role="tooltip"` after
// `CHROME_CONTROL_TOOLTIP_DELAY_MS`. An immediate-mode canvas has no such affordance, so the dwell
// timer lives here: `update_hover` stamps when the hover chain's leaf last changed, and
// `advance_clock` — fed one monotonic seconds value per frame by `engine::Ui::advance_clock` — is
// what actually fires. The same clock debounces `DismissPolicy::hover_out_delay_seconds`, which was
// previously documented as "not actually debounced yet".

/// 💡️ What a clock step asks the host to do about hover reveal this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TooltipStep {
    /// 😴️ Nothing changed — no dwell elapsed, nothing to dismiss.
    Idle,
    /// 💡️ `node` has been hovered for [`TOOLTIP_DWELL_SECONDS`]: reconcile its tooltip content in
    /// and `open_overlay` it as an [`OverlayKind::Tooltip`] anchored on `node`.
    Reveal(NodeId),
    /// 🚪️ The armed hover-out delay elapsed — the caller closed the tooltip as part of this step.
    Dismissed,
}
//#endregion 🔖️Tooltip

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
    /// 🧩️ A semantic control fired — carried as a [`UiIntentCommand`], the renderer-side twin of
    /// `ui_contract::UiIntent`, never the bare `ActionDescriptor` this variant used to hold. The
    /// intent adds the addressing (`surface`/`revision`/`node`/`node_key`), the versioned `ActionId`,
    /// a payload kept apart from the authored args, and the per-surface `seq` — everything the host's
    /// admission gate (`BoundedActionQueue::admit_intent`) needs to order, de-duplicate and refuse a
    /// stale gesture. `intent.descriptor()` is the one place it collapses back to an address the
    /// plugin bridge dispatches.
    App { window_id: String, intent: UiIntentCommand },
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
    /// ⏱️ Monotonic seconds, advanced once per frame by `advance_clock`. Every hover-reveal deadline
    /// is an absolute value on this clock, never a countdown, so a dropped frame cannot lose time.
    clock_seconds: f32,
    /// 💡️ When the current hover leaf was entered, and whether its dwell already fired — so a
    /// tooltip opens exactly once per hover, not every frame after the deadline.
    hover_since: Option<(NodeId, f32)>,
    hover_revealed: Option<NodeId>,
    /// 🚪️ Deadline armed by `maybe_dismiss_tooltip_on_hover_out` once the pointer leaves an open
    /// tooltip's anchor and bounds — see `DismissPolicy::hover_out_delay_seconds`.
    tooltip_dismiss_at: Option<f32>,
    /// 🔤️ The open `Select`'s live typeahead query and the `clock_seconds` of its last keystroke.
    /// React keeps the same buffer behind a 700 ms timer it restarts per key
    /// (`🧱️elements/🔽️Select/🟦️.tsx:661-666`); this expires it against the same monotonic clock
    /// every other reveal deadline here uses, so a dropped frame cannot lose time.
    select_typeahead: Option<(String, f32)>,
    /// 🔢️ The per-surface monotonic `seq` every fired intent carries — this window's own twin of
    /// `UiDocumentStore`'s `seq` counter (`📃️UiDocumentStore/🟦️.tsx`'s `buildIntent`). Minted here,
    /// enforced at the queue (`BoundedActionQueue::admit_intent`).
    intents: UiIntentSequencer,
    /// 🧭️ This window's logical flow — the twin of React's `useFlow()`
    /// (`🔨️modules/🧭️flow-direction-context/🟦️.tsx:48`). `Rtl` mirrors the inline axis of every
    /// anchored overlay this router places and of every inline arrow key it routes; `Up` is read by
    /// paint, not here. Set from the window's dock anchor through `Ui::set_window_flow`, exactly as
    /// React derives it from `flowFromAnchor`.
    flow: UiFlow,
    /// ⌨️ Whether the LAST input this router saw was a key rather than a pointer — React's
    /// `:focus-visible` in one bit. A focus ring paints only while this is set, so a clicked control
    /// gets focus without the keyboard ring (`🖌️paint`'s focus-ring sites read `focus_visible`).
    focus_visible: bool,
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
            clock_seconds: 0.0,
            hover_since: None,
            hover_revealed: None,
            tooltip_dismiss_at: None,
            select_typeahead: None,
            intents: UiIntentSequencer::default(),
            flow: UiFlow::DEFAULT,
            focus_visible: false,
        }
    }

    /// 🧭️ This window's logical flow.
    pub(crate) fn flow(&self) -> UiFlow {
        self.flow
    }

    /// 🧭️ This window's inline direction — the `rtl` argument every mirrored formula takes.
    pub(crate) fn flow_inline(&self) -> FlowInline {
        self.flow.inline
    }

    /// 🧭️ Replaces this window's flow. Answers whether it changed, so the caller can invalidate the
    /// layout generation the way `Ui::set_viewport` does for metrics.
    pub(crate) fn set_flow(&mut self, flow: UiFlow) -> bool {
        if self.flow == flow {
            return false;
        }
        self.flow = flow;
        true
    }

    /// ⌨️ Whether a focus ring should paint — React's `focus-visible`. True only after a KEY moved or
    /// activated focus; any pointer press clears it.
    pub(crate) fn focus_visible(&self) -> bool {
        self.focus_visible
    }

    //#region 🎬️IntentApi
    /// 🎬️ Turns one fired gesture into an addressed [`UiIntentCommand`] — the wgpu twin of React's
    /// `dispatchTrigger` → `emitIntent` → `UiDocumentStore::buildIntent` chain.
    ///
    /// Three things happen here and nowhere else: the node's stamped address supplies
    /// `surface`/`revision`/`node`/`node_key`; a gesture whose recorded revision is already more than
    /// one behind the document it is being resolved against is DROPPED (`intent_is_stale` — React's
    /// own `is_stale`, so a click on geometry the user never saw cannot misapply); and the surface's
    /// `seq` is minted. A node the document never published (the chrome's immediate-mode widgets, the
    /// testkit's `apply_tree`) has no address and no revision to be stale against — it still fires,
    /// with a default address, which is exactly the unowned-element channel React keeps `onAction` for.
    fn build_intent(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction) -> Option<UiIntentCommand> {
        let bindings = tree.node(node).and_then(|node| node.intent.clone());
        let current_revision = tree.document().map_or(0, |document| document.revision().0);
        let (address, action) = match bindings {
            Some(bindings) => {
                if intent_is_stale(bindings.address.revision, current_revision) {
                    return None;
                }
                let action = match bindings.action_for(fired.trigger) {
                    Some(action) => action.clone(),
                    None => descriptor_action_id(&fired.action)?,
                };
                (bindings.address, action)
            }
            None => (UiIntentAddress::default(), descriptor_action_id(&fired.action)?),
        };
        let seq = self.intents.next(&address.surface);
        Some(UiIntentCommand { address, trigger: fired.trigger, action, args: fired.action.args, input: fired.input, seq })
    }

    /// 🎬️ [`Self::build_intent`] wrapped as the command the host drains. `None` when the gesture was
    /// dropped as stale or could not be addressed.
    fn app_command(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction) -> Option<UiCommand> {
        Some(UiCommand::App { window_id: self.window_id.clone(), intent: self.build_intent(tree, node, fired)? })
    }

    /// 🎬️ [`Self::app_command`] for a caller that already owns the command list.
    fn push_app_command(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction, out: &mut Vec<UiCommand>) {
        if let Some(command) = self.app_command(tree, node, fired) {
            out.push(command);
        }
    }
    //#endregion 🎬️IntentApi

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
        self.hover_since = target.map(|leaf| (leaf, self.clock_seconds));
        self.hover_revealed = None;
        commands
    }

    //#region 🔖️TooltipClock
    /// ⏱️ Feeds one frame's monotonic seconds in and answers what hover reveal owes this frame —
    /// the immediate-mode stand-in for React's `setTimeout`/`clearTimeout` pair in
    /// `ChromeControlHint` (`🧱️elements/💡️ChromeControlHint/🟦️.tsx:44-57`). A non-monotonic value is
    /// ignored rather than rewinding every armed deadline.
    pub(crate) fn advance_clock(&mut self, tree: &mut UiTree, seconds: f32) -> (TooltipStep, Vec<UiCommand>) {
        if !seconds.is_finite() || seconds < self.clock_seconds {
            return (TooltipStep::Idle, Vec::new());
        }
        self.clock_seconds = seconds;
        if let Some(deadline) = self.tooltip_dismiss_at {
            if seconds >= deadline {
                self.tooltip_dismiss_at = None;
                if self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::Tooltip) {
                    return (TooltipStep::Dismissed, self.close_topmost_overlay(tree));
                }
            }
            return (TooltipStep::Idle, Vec::new());
        }
        if self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::Tooltip) {
            return (TooltipStep::Idle, Vec::new());
        }
        let Some((leaf, entered)) = self.hover_since else { return (TooltipStep::Idle, Vec::new()) };
        if self.hover_revealed == Some(leaf) || seconds - entered < TOOLTIP_DWELL_SECONDS {
            return (TooltipStep::Idle, Vec::new());
        }
        self.hover_revealed = Some(leaf);
        (TooltipStep::Reveal(leaf), Vec::new())
    }

    //#endregion 🔖️TooltipClock

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

    /// 🥞️ Every open overlay bottom-to-top — what `engine::Ui::open_overlays` publishes so a paint
    /// pass can place and draw them above the document.
    pub(crate) fn open_overlays(&self) -> &[OpenOverlay] {
        self.overlays.as_slice()
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
                node.state.highlighted = None;
            }
        }
        if overlay.kind == OverlayKind::Tooltip {
            self.tooltip_dismiss_at = None;
        }
        tree.mark_dirty(overlay.root, NodeFlags::DIRTY_PAINT);
        let mut out = vec![UiCommand::OverlayClosed { window_id: self.window_id.clone(), root: overlay.root, kind: overlay.kind }];
        if let Some(focused) = self.focus.focused {
            if is_descendant(tree, focused, overlay.root) {
                if let Some((blurred, fired)) = self.focus.clear_focus(tree) {
                    self.push_app_command(tree, blurred, fired, &mut out);
                }
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

    /// 🖱️ `Tooltip`-only: arms (or disarms) the hover-out countdown as the pointer leaves or re-enters
    /// the anchor and the overlay's own bounds. `advance_clock` is what actually closes it once
    /// `DismissPolicy::hover_out_delay_seconds` has elapsed, matching React's `onPointerLeave` →
    /// `clearTimeout`/close pair rather than the immediate close this used to do.
    fn maybe_dismiss_tooltip_on_hover_out(&mut self, tree: &mut UiTree, x: f32, y: f32) -> Vec<UiCommand> {
        let Some(top) = self.overlays.topmost() else { return Vec::new() };
        if top.kind != OverlayKind::Tooltip {
            return Vec::new();
        }
        let overlay_root = top.root;
        let anchor = top.anchor;
        let delay = top.dismiss.hover_out_delay_seconds;
        let inside_overlay = node_abs_rect(tree, overlay_root).is_some_and(|rect| rect.contains(x, y));
        let inside_anchor = match anchor {
            OverlayAnchor::Node(id) => node_abs_rect(tree, id).is_some_and(|rect| rect.contains(x, y)),
            OverlayAnchor::Point { .. } => false,
        };
        if inside_overlay || inside_anchor {
            self.tooltip_dismiss_at = None;
            return Vec::new();
        }
        match delay {
            Some(delay) => {
                self.tooltip_dismiss_at.get_or_insert(self.clock_seconds + delay);
                Vec::new()
            }
            None => self.close_topmost_overlay(tree),
        }
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
    /// 🎬️ The focused editable node's buffer as a dispatchable action, for a node that commits on
    /// every keystroke (`Trigger::Change` — React's `InputView` with no `commit: "blur"`, and its
    /// `IconSelector` editor, both of which fire per change). `None` for a blur-committing node,
    /// whose one dispatch moment is `FocusState::set_focus`.
    fn changed_buffer_action(&self, tree: &UiTree) -> Option<(NodeId, FiredAction)> {
        let id = self.focus.focused?;
        let node = tree.node(id)?;
        if commits_on_blur(&node.spec.0) {
            return None;
        }
        edit_commit_action(node, &node.state.edit.as_ref()?.text).map(|fired| (id, fired))
    }

    /// 🎬️ Wraps `changed_buffer_action` for a caller that already owns the command list.
    fn push_buffer_change(&mut self, tree: &UiTree, out: &mut Vec<UiCommand>) {
        if let Some((id, fired)) = self.changed_buffer_action(tree) {
            self.push_app_command(tree, id, fired, out);
        }
    }

    fn route_text_insert(&mut self, tree: &mut UiTree, text: &str) -> Vec<UiCommand> {
        let mut out = Vec::new();
        let Some(id) = self.focus.focused else { return out };
        let Some(node) = tree.node_mut(id) else { return out };
        let Some(edit) = node.state.edit.as_mut() else { return out };
        insert_at_caret(edit, text);
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        self.push_buffer_change(tree, &mut out);
        out
    }

    fn route_ime(&mut self, tree: &mut UiTree, event: &ImeEvent) -> Vec<UiCommand> {
        let mut out = Vec::new();
        let Some(id) = self.focus.focused else { return out };
        let Some(node) = tree.node_mut(id) else { return out };
        let Some(edit) = node.state.edit.as_mut() else { return out };
        let mutated = match event {
            ImeEvent::Start => {
                edit.composition = Some(String::new());
                false
            }
            ImeEvent::Update { text, .. } => {
                edit.composition = Some(text.clone());
                false
            }
            ImeEvent::Commit { text } => {
                edit.composition = None;
                insert_at_caret(edit, text);
                true
            }
            ImeEvent::Cancel => {
                edit.composition = None;
                false
            }
        };
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
        if mutated {
            self.push_buffer_change(tree, &mut out);
        }
        out
    }

    /// ⌨️ Caret motion (with `Shift` extending the selection), `Home`/`End`, `Backspace`/`Delete`,
    /// and clipboard shortcuts for the focused node's `EditState`. A no-operation if nothing is focused or
    /// the focused node has no `EditState` (isn't a `UiNode::Input`, or hasn't been focused since
    /// `FocusState::set_focus` seeded one).
    fn route_edit_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers) -> Vec<UiCommand> {
        let mut out = Vec::new();
        let Some(id) = self.focus.focused else { return out };
        // ⏎️ Enter is the other half of a blur-committing node's contract (`Trigger::Commit`, which
        // `🖼️semantic-ui/🦀️.rs`'s own docstring names "Enter / blur") — the gesture generation3d's
        // inline rename editor is built on. A change-committing node already dispatched every
        // keystroke, so Enter adds nothing there and must not double-fire.
        if matches!(key, "Enter" | "NumpadEnter") {
            let fired = tree.node(id).filter(|node| commits_on_blur(&node.spec.0)).and_then(|node| node.state.edit.as_ref().and_then(|edit| edit_commit_action(node, &edit.text)));
            if let Some(fired) = fired {
                self.push_app_command(tree, id, fired, &mut out);
            }
            return out;
        }
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
        self.push_buffer_change(tree, &mut out);
        out
    }
    //#endregion 🔖️EditApi

    //#region 🔖️CursorApi
    #[allow(dead_code, reason = "cursor-state accessor, not yet called; likely wired by a later events-integration milestone")]
    pub(crate) fn hovered(&self) -> Option<NodeId> {
        self.hovered
    }

    /// 🖱️ What this window's retained content currently holds pointer capture on — read by
    /// `engine::Ui::window_with_pointer_capture` so a host keeps feeding a live drag its moves.
    pub(crate) fn capture(&self) -> Option<(NodeId, CaptureKind)> {
        self.capture.target
    }

    /// 🎯️ Read-only: whether this window's retained content currently holds keyboard focus — see
    /// `engine::Ui::window_has_focus` (its only caller), added for the `w2-input-wiring` host-side
    /// focus arbitration (content vs. chrome routing, `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`).
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
                    // 🎚️ A captured `Slider`/`Ring` reports EVERY intermediate value, not only the
                    // release — Radix's own behaviour, which React's `SliderView`/`RingView` delegate to.
                    Some((pressed, CaptureKind::Press)) => {
                        if tree.node(pressed).is_some_and(|node| commits_while_dragging(&node.spec.0)) {
                            commands.extend(self.pointer_commit(tree, pressed, *x, *y));
                        }
                    }
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
                self.focus_visible = false;
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
                            if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(id), false) {
                                self.push_app_command(tree, blurred, fired, &mut commands);
                            }
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
                        if tree.node(id).is_some_and(|node| commits_while_dragging(&node.spec.0) && node.spec.0.presence().state != UiState::Disabled) {
                            commands.extend(self.pointer_commit(tree, id, *x, *y));
                        }
                    }
                } else {
                    if let Some((blurred, fired)) = self.focus.clear_focus(tree) {
                        self.push_app_command(tree, blurred, fired, &mut commands);
                    }
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
                            if hit_test(tree, root, *x, *y) == Some(active_id) && tree.node(active_id).is_some_and(|node| node.spec.0.presence().state != UiState::Disabled) {
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
                                        UiNode::Button(button) => bare_action(&button.action, Trigger::Activate).map(|fired| (fired, node.parent)),
                                        UiNode::Stack(stack) => stack.activate.as_ref().and_then(|action| bare_action(action, Trigger::Activate)).map(|fired| (fired, None)),
                                        _ => None,
                                    });
                                    if let Some((fired, parent)) = fired {
                                        self.push_app_command(tree, active_id, fired, &mut commands);
                                        if let Some(parent) = parent {
                                            let picked_from_open_select = self.overlays.topmost().is_some_and(|overlay| overlay.kind == OverlayKind::SelectPopup && overlay.root == parent);
                                            if picked_from_open_select {
                                                commands.extend(self.close_topmost_overlay(tree));
                                            }
                                        }
                                    } else {
                                        // 🎬️ Every other value-carrying kind — `Toggle`, `Slider`,
                                        // `NumberStepper`, `Ring` — commits its own gesture here, through
                                        // the same one authority (see 🔖️Commit).
                                        commands.extend(self.pointer_commit(tree, active_id, *x, *y));
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
                self.focus_visible = true;
                // 🔽️ A focused `Select` answers navigation/typeahead/commit keys before the generic
                // control routing below, exactly as React's `SelectTrigger`/`SelectContent` handlers
                // claim them ahead of the browser's own default (`🧱️elements/🔽️Select/🟦️.tsx`).
                // `Escape` stays with the overlay stack and `Tab` still moves focus after the popup
                // this closes, so neither of those two is ever reported as consumed.
                let select_consumed = key != "Escape" && self.route_select_key(tree, key, *modifiers, &mut commands);
                if key == "Escape" {
                    commands.extend(self.close_topmost_overlay(tree));
                } else if select_consumed {
                } else if key == "Tab" {
                    let scope = self.overlays.topmost_focus_trap_root().unwrap_or(root);
                    let committed = if modifiers.shift { self.focus.focus_prev(tree, scope) } else { self.focus.focus_next(tree, scope) };
                    if let Some((blurred, fired)) = committed {
                        self.push_app_command(tree, blurred, fired, &mut commands);
                    }
                    commands.push(UiCommand::FocusChanged { window_id: self.window_id.clone(), node: self.focus.focused });
                } else if let Some((activated, fired)) = self.focused_button_activation(tree, key) {
                    self.push_app_command(tree, activated, fired, &mut commands);
                } else if let Some((changed, fired)) = self.focused_value_key_activation(tree, self.mirrored_inline_key(key), *modifiers) {
                    self.push_app_command(tree, changed, fired, &mut commands);
                } else {
                    commands.extend(self.route_edit_key(tree, key, *modifiers));
                }
            }
            UiEvent::KeyUp { .. } => {}
            UiEvent::TextInput { text } => commands.extend(self.route_text_insert(tree, text)),
            UiEvent::Paste { text } => commands.extend(self.route_text_insert(tree, text)),
            UiEvent::Ime(ime_event) => commands.extend(self.route_ime(tree, ime_event)),
            UiEvent::Scroll { x, y, delta_x, delta_y } => {
                if let Some(id) = hit_test(tree, root, *x, *y) {
                    if let Some(cmd) = self.scene_command(tree, id, event) {
                        commands.push(cmd);
                    }
                }
                self.route_scroll(tree, root, *x, *y, *delta_x * self.flow.inline.inline_sign(), *delta_y);
            }
        }
        commands
    }

    /// 🧭️ One key name as this window's INLINE direction means it. React inverts the horizontal pair
    /// per control (`🎚️Slider/🟦️.tsx:383-384`, `📑️Tabs/🟦️.tsx:164-165`, `🎛️ToggleGroup/🟦️.tsx:118-119`
    /// all read `dir === "rtl"`); doing it once here means every value control inherits the rule
    /// instead of each re-deriving it. Vertical and page keys are direction-invariant, and a caret
    /// move inside text is NOT routed through here — `route_edit_key` walks byte boundaries, which
    /// are already logical.
    fn mirrored_inline_key<'k>(&self, key: &'k str) -> &'k str {
        if !self.flow.inline.is_rtl() {
            return key;
        }
        match key {
            "ArrowLeft" => "ArrowRight",
            "ArrowRight" => "ArrowLeft",
            other => other,
        }
    }

    /// 🎬️ If `id` is a `ComponentScene` leaf, resolves its `SurfaceKind`/absolute rect (the same
    /// ancestor-offset accumulation `scene_slots::collect_scene_slots`/`hit_test_node`/
    /// `paint::paint_node` each do independently — not reusing `collect_scene_slots` itself, since
    /// that walks the WHOLE tree per call and this runs once per real input event) and builds the
    /// `UiCommand::Scene` the host should route into that surface's per-`SurfaceKind` input handler.
    fn scene_command(&self, tree: &UiTree, id: NodeId, event: &UiEvent) -> Option<UiCommand> {
        let node = tree.node(id)?;
        let UiNode::ComponentScene(scene) = &node.spec.0 else { return None };
        let rect = absolute_rect(tree, id)?;
        Some(UiCommand::Scene { window_id: self.window_id.clone(), node: id, surface_id: scene.surface_id.clone(), kind: scene.component_kind, rect, event: event.clone() })
    }

    /// ⏎️ The action `Enter` or `Space` activates on the focused enabled `Button` — the keyboard half of a click, as a
    /// native `<button>` answers it ([WAI-ARIA button pattern](https://www.w3.org/WAI/ARIA/apg/patterns/button/)).
    fn focused_button_activation(&self, tree: &UiTree, key: &str) -> Option<(NodeId, FiredAction)> {
        if !matches!(key, "Enter" | "NumpadEnter" | " ") {
            return None;
        }
        let id = self.focus.focused?;
        match &tree.node(id)?.spec.0 {
            UiNode::Button(button) if button.presence.state != UiState::Disabled => bare_action(&button.action, Trigger::Activate).map(|fired| (id, fired)),
            _ => None,
        }
    }

    //#region 🔖️WidgetKeyboard
    /// 🔽️ Routes one key at the focused `Select`, returning whether it was consumed. Opens the popup
    /// (highlighting the selected/last/first-matching row), moves the highlight, commits it, closes
    /// on `Tab`, or extends the typeahead query — the union of React's `SelectTrigger` and
    /// `SelectContent` `onKeyDown` handlers, whose decision table lives with the element
    /// (`select::select_key`). `Tab` closes but reports `false`, so the same keystroke still moves
    /// focus the way it does in the DOM.
    fn route_select_key(&mut self, tree: &mut UiTree, key: &str, modifiers: EventModifiers, out: &mut Vec<UiCommand>) -> bool {
        let Some(id) = self.focus.focused else { return false };
        let Some(node) = tree.node(id) else { return false };
        let UiNode::Select(select) = &node.spec.0 else { return false };
        if select.presence.state == UiState::Disabled {
            return false;
        }
        let open = node.state.open;
        let highlighted = node.state.highlighted;
        let labels: Vec<String> = select.items.iter().map(|item| item.label.as_str().to_string()).collect();
        let values: Vec<String> = select.items.iter().map(|item| item.value.clone()).collect();
        let selected = values.iter().position(|value| value == &select.value);
        let on_change = select.on_change.clone();
        match select::select_key(open, key, modifiers.alt, modifiers.ctrl, modifiers.meta) {
            select::SelectKey::Ignored => false,
            select::SelectKey::Open(intent) => {
                let index = select::select_open_index(intent, labels.iter().map(String::as_str), selected);
                self.select_typeahead = match intent {
                    select::SelectOpenIntent::Typeahead(char) => Some((char.to_string(), self.clock_seconds)),
                    _ => None,
                };
                out.extend(self.toggle_select_popup(tree, id));
                self.set_select_highlight(tree, id, index);
                true
            }
            select::SelectKey::Move(movement) => {
                let index = select::select_moved_index(highlighted, labels.len(), movement);
                self.select_typeahead = None;
                self.set_select_highlight(tree, id, index);
                true
            }
            select::SelectKey::Commit => {
                let Some(value) = highlighted.and_then(|index| values.get(index)) else { return true };
                if let Some(fired) = fired_action(&on_change, Trigger::Change, DslValue::String(value.clone())) {
                    self.push_app_command(tree, id, fired, out);
                }
                self.select_typeahead = None;
                out.extend(self.close_overlay(tree, id));
                true
            }
            select::SelectKey::Close => {
                self.select_typeahead = None;
                out.extend(self.close_overlay(tree, id));
                // ↹️ `finish_close` drops focus that sat inside the closed overlay's subtree, and a
                // `Select` IS its own popup root — so put focus back on the trigger before the `Tab`
                // handler runs, or the tab move would restart from the top of the document instead
                // of stepping off this control, which is what React's trigger-retained focus does.
                if self.focus.focused.is_none() {
                    if let Some((blurred, fired)) = self.focus.set_focus(tree, Some(id), true) {
                        self.push_app_command(tree, blurred, fired, out);
                    }
                }
                false
            }
            select::SelectKey::Typeahead(char) => {
                let query = self.extend_select_typeahead(char);
                let index = select::select_typeahead_index(labels.iter().map(String::as_str), &query, highlighted);
                if index.is_some() {
                    self.set_select_highlight(tree, id, index);
                }
                true
            }
        }
    }

    /// 🔤️ Appends `char` to the live typeahead query, restarting it when the previous keystroke is
    /// older than `select::SELECT_TYPEAHEAD_RESET_SECONDS`.
    fn extend_select_typeahead(&mut self, char: char) -> String {
        let fresh = self.select_typeahead.as_ref().is_some_and(|(_, at)| self.clock_seconds - at < select::SELECT_TYPEAHEAD_RESET_SECONDS);
        let mut query = if fresh { self.select_typeahead.take().map(|(query, _)| query).unwrap_or_default() } else { String::new() };
        query.push(char);
        self.select_typeahead = Some((query.clone(), self.clock_seconds));
        query
    }

    /// ⌨️ Writes the popup's highlighted row and repaints it — `paint`'s only input for the
    /// keyboard-active row (`tree::WidgetState::highlighted`).
    fn set_select_highlight(&mut self, tree: &mut UiTree, id: NodeId, index: Option<usize>) {
        if let Some(node) = tree.node_mut(id) {
            node.state.highlighted = index;
        }
        tree.mark_dirty(id, NodeFlags::DIRTY_PAINT);
    }

    /// ⌨️ The value-committing keyboard gestures the other focusable controls answer, which a plain
    /// `<button>`'s Enter/Space (see `focused_button_activation`) does not cover: a `Toggle` flips
    /// (React renders it as a button, so Enter/Space click it), a `Slider` steps
    /// (`slider_key_value`), and a `NumberStepper`'s `ArrowUp`/`ArrowDown` are its +/− segments —
    /// the same binding-gated choice `pointer_commit_action` makes for a click on them.
    fn focused_value_key_activation(&self, tree: &UiTree, key: &str, modifiers: EventModifiers) -> Option<(NodeId, FiredAction)> {
        let id = self.focus.focused?;
        let node = tree.node(id)?;
        if node.spec.0.presence().state == UiState::Disabled {
            return None;
        }
        let fired = match &node.spec.0 {
            UiNode::Toggle(toggle) if matches!(key, "Enter" | "NumpadEnter" | " ") => fired_action(&toggle.on_change, Trigger::Change, DslValue::Bool(!toggle.presence.selected)),
            UiNode::Slider(slider) => fired_action(&slider.on_change, Trigger::Change, DslValue::float(slider_key_value(slider, key, modifiers.shift)?)),
            UiNode::NumberStepper(stepper) => {
                let sign = match key {
                    "ArrowUp" => 1.0,
                    "ArrowDown" => -1.0,
                    _ => return None,
                };
                number_stepper_fired(node, stepper, sign)
            }
            _ => None,
        }?;
        Some((id, fired))
    }
    //#endregion 🔖️WidgetKeyboard

    /// 👆️ The `UiCommand::App` a press/drag at `(x, y)` over `id` commits, for the pointer-valued
    /// control kinds — see `pointer_commit_action`. The rect it measures the gesture against is the
    /// node's own absolute painted rect, the same one `paint` drew the knob/segments at.
    fn pointer_commit(&mut self, tree: &UiTree, id: NodeId, x: f32, y: f32) -> Option<UiCommand> {
        let node = tree.node(id)?;
        let fired = pointer_commit_action(node, absolute_rect(tree, id)?, x, y)?;
        self.app_command(tree, id, fired)
    }
}
//#endregion 🔖️UiCommand

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎛️retained-control-commit/🦀️.rs"]
mod control_commit_tests;
// #endregion events
