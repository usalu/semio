//! 🧲️ Overlays, drag and commands: the overlay stack and placement, the tooltip and caret clock steps, the drag session, the text helpers, `UiCommand` and the `EventRouter` state. See [`super`].

use super::*;

// #region events
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
pub struct OverlayStack {
    pub(super) open: Vec<OpenOverlay>,
}

impl OverlayStack {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn open(&mut self, overlay: OpenOverlay) {
        self.open.push(overlay);
    }

    pub(super) fn topmost(&self) -> Option<&OpenOverlay> {
        self.open.last()
    }

    /// 🔝️ Bottom-to-top: the exact order a paint pass must draw them in.
    pub(super) fn as_slice(&self) -> &[OpenOverlay] {
        &self.open
    }

    pub(super) fn close_root(&mut self, root: NodeId) -> Option<OpenOverlay> {
        let position = self.open.iter().position(|overlay| overlay.root == root)?;
        Some(self.open.remove(position))
    }

    pub(super) fn close_topmost(&mut self) -> Option<OpenOverlay> {
        self.open.pop()
    }

    /// 🪤️ The root of the topmost `focus_trap` overlay, if any — `Escape`/outside-press only ever
    /// close the *topmost* overlay, but a focus trap set by a lower (still-open) trapping overlay
    /// stays in effect once a higher non-trapping overlay (e.g. a `Tooltip`) is on top of it, so this
    /// searches from the top down rather than just checking `topmost()`.
    pub(super) fn topmost_focus_trap_root(&self) -> Option<NodeId> {
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

/// 💥️ [`resolve_overlay_placement`] plus the side a collision flip settled on — what a paint pass
/// reads when it also wants React's `transformOrigin` equivalent. `flow` is React's own `rtl`
/// argument, read from the window's [`ui_contract::UiFlow`] (`Ui::window_flow`).
pub fn resolve_overlay_placement_side(tree: &UiTree, anchor: OverlayAnchor, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement, flow: FlowInline) -> ResolvedOverlayPlacement {
    let anchor_rect = match anchor {
        OverlayAnchor::Node(id) => node_abs_rect(tree, id).unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0)),
        OverlayAnchor::Point { x, y } => Rect::new(x, y, 0.0, 0.0),
    };
    ui_contract::resolve_overlay_placement(overlay_rect(anchor_rect), content_size, viewport, placement, flow)
}

/// 🧬️ This target's [`Rect`] as the contract's renderer-neutral [`OverlayRect`] — the ONE conversion
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
/// 💬️ A disabled row action's reason hint (audit W1E-1, `💬️row-semantics` `revealReason`): shown on hover, keyboard focus and a
/// press of the action, hidden on leave, blur and Escape; `hovered` marks a hint the pointer revealed, which only leaving hides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowReasonHint {
    pub row: NodeId,
    pub index: usize,
    pub hovered: bool,
}

/// 🧠️ The reason of `row`'s `index`-th action while that action is disabled and names one — the contract's `RowAction.reason`,
/// the text every renderer announces as its description and shows as its hint.
pub fn disabled_row_action_reason(tree: &UiTree, row: NodeId, index: usize) -> Option<&str> {
    let action = tree.authored_tree_item(row)?.actions.as_deref()?.get(index)?;
    action.reason.as_ref().map(|reason| reason.as_str()).filter(|reason| action.disabled && !reason.is_empty())
}

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
    /// 💭️ `node` has been hovered for [`TOOLTIP_DWELL_SECONDS`]: reconcile its tooltip content in
    /// and `open_overlay` it as an [`OverlayKind::Tooltip`] anchored on `node`.
    Reveal(NodeId),
    /// 🚪️ The armed hover-out delay elapsed — the caller closed the tooltip as part of this step.
    Dismissed,
}

pub const CARET_BLINK_SECONDS: f64 = 0.5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaretSource {
    Retained,
    Scene,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaretStep {
    pub node: NodeId,
    pub source: CaretSource,
    pub visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct CaretClock {
    pub(super) node: NodeId,
    pub(super) source: CaretSource,
    pub(super) visible: bool,
    pub(super) next_at: f64,
}

pub struct RouterClockStep {
    pub tooltip: TooltipStep,
    pub caret: Option<CaretStep>,
    pub commands: Vec<UiCommand>,
    pub changed: bool,
    pub next_deadline: Option<f64>,
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
pub(super) const DRAG_PROMOTE_THRESHOLD_SQ: f32 = 16.0;
//#endregion 🔖️DragDrop

//#region 🔖️Scroll
// 🖱️ Wheel events route to the nearest scrollable ancestor of the node under the pointer, walking
// the bubble chain for `NodeFlags::SCROLLABLE` exactly like `nearest_accepting_drop_target` walks it
// for `NodeFlags::DROP_TARGET`. Thumb-drag capture (`CaptureKind::ScrollThumb`) is a separate path:
// `paint` paints the actual thumb node wherever it likes in the tree and registers it once via
// `EventRouter::register_scroll_thumb`, decoupling thumb geometry from scrollable-content geometry.

/// ⛓️ Walks `from`'s bubble chain (inclusive) for the nearest `NodeFlags::SCROLLABLE` node.
pub(super) fn nearest_scrollable_ancestor(tree: &UiTree, from: NodeId) -> Option<NodeId> {
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

pub(super) fn prev_char_boundary(text: &str, index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    let mut candidate = index - 1;
    while candidate > 0 && !text.is_char_boundary(candidate) {
        candidate -= 1;
    }
    candidate
}

pub(super) fn next_char_boundary(text: &str, index: usize) -> usize {
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
pub(super) fn selection_bounds(anchor: usize, caret: usize) -> (usize, usize) {
    (anchor.min(caret), anchor.max(caret))
}

/// ✍️ Replaces the current selection (or inserts at the caret if there isn't one) with `text`,
/// collapsing caret and anchor to just past the inserted text. Shared by `TextInput`, `Paste`, and
/// `Ime::Commit` routing — insertion semantics are identical for all three.
pub(super) fn insert_at_caret(edit: &mut EditState, text: &str) {
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
    /// 📪️ An overlay closed — either explicitly (`EventRouter::close_overlay`) or via dismissal
    /// (outside-press swallow, `Escape`, tooltip hover-out).
    OverlayClosed { window_id: String, root: NodeId, kind: OverlayKind },
    /// 🏹️ A `DragSession` released over an accepting drop target.
    DropCommitted { window_id: String, source: NodeId, target: NodeId, payload: DragPayload },
    /// 🗑️ A `DragSession` released with no accepting drop target under the pointer.
    DropCancelled { window_id: String, source: NodeId },
    /// 📋️ `Ctrl`/`Cmd`+`C` over a text selection — host copies `text` to the OS clipboard.
    ClipboardCopy { window_id: String, text: String },
    /// ✂️ `Ctrl`/`Cmd`+`X` over a text selection — `text` is already removed from the `EditState`
    /// buffer; host copies it to the OS clipboard.
    ClipboardCut { window_id: String, text: String },
    /// 📥️ `Ctrl`/`Cmd`+`V`: host must read the OS clipboard and feed the result back as
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

pub(super) struct RetiringDragPayload {
    entries: std::collections::hash_map::IntoIter<String, String>,
    entry: Option<(String, String)>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct StepperRepeat {
    pub(super) node: NodeId,
    pub(super) sign: f64,
    pub(super) next_at: f64,
}

pub(super) const STEPPER_REPEAT_DELAY_SECONDS: f64 = 0.5;

pub(super) const STEPPER_REPEAT_INTERVAL_SECONDS: f64 = 0.1;

pub(super) const STEPPER_REPEAT_CLOCK_EPSILON_SECONDS: f64 = 0.000_001;

pub(super) const SLIDER_DOUBLE_CLICK_SECONDS: f64 = 0.5;

impl RetiringDragPayload {
    pub(super) fn new(payload: DragPayload) -> Self {
        Self { entries: payload.into_iter(), entry: None }
    }

    pub(super) fn close_step(&mut self) -> bool {
        if let Some((key, value)) = self.entry.as_mut() {
            if value.pop().is_some() || key.pop().is_some() {
                return false;
            }
            if value.capacity() > 0 {
                *value = String::new();
                return false;
            }
            if key.capacity() > 0 {
                *key = String::new();
                return false;
            }
            self.entry = None;
            return false;
        }
        self.entry = self.entries.next();
        self.entry.is_none()
    }
}

pub struct EventRouter {
    pub(super) window_id: String,
    pub capture: CaptureState,
    pub focus: FocusState,
    pub hovered: Option<NodeId>,
    /// 🫧️ Every node currently in the hover bubble chain (leaf-to-root from `hovered`), so an
    /// ancestor container (e.g. a `Stack`-based tree-item row, which `hit_test` never itself returns
    /// as the match — see 🔖️HitTest's `is_plain_container`) still observes `NodeFlags::HOVERED` for
    /// `paint`'s hover-reveal (React's `placement` / driver.chrome reveal) to key off of.
    pub(super) hover_chain: Vec<NodeId>,
    /// 👇️ Pointer position at the start of the current `Press` capture, for `maybe_promote_to_drag`'s
    /// movement-threshold check.
    pub(super) press_origin: Option<(f32, f32)>,
    pub(super) overlays: OverlayStack,
    pub(super) drag: Option<DragSession>,
    pub(super) tree_drag_driver: UiDriverDrag,
    pub(super) tree_drag_metrics: TreeRowMetrics,
    pub(super) control_border: f32,
    pub(super) control_gap: f32,
    pub(super) tree_drag_handle_press: Option<NodeId>,
    /// 🎒️ Per-node `DragPayload` a `Press` capture on that node may promote into, set via
    /// `set_drag_payload`.
    pub(super) drag_payloads: BTreeMap<NodeId, DragPayload>,
    /// ✅️ Per-node accept predicate refining plain `NodeFlags::DROP_TARGET` membership, set via
    /// `set_drop_accept`. Absent from this map but flagged `DROP_TARGET` still accepts everything.
    pub(super) drop_accept: BTreeMap<NodeId, DropAcceptPredicate>,
    /// 🏁️ Scrollbar-thumb node id → (its owning `NodeFlags::SCROLLABLE` node, drag axis), set via
    /// `register_scroll_thumb`.
    pub(super) scroll_thumbs: BTreeMap<NodeId, (NodeId, ScrollAxis)>,
    pub(super) retiring_payload: Option<RetiringDragPayload>,
    /// 🖱️ `(pointer_x, pointer_y, scroll_offset_x, scroll_offset_y)` captured at the start of a
    /// `ScrollThumb` drag, for `update_scroll_thumb`'s delta-computation baseline.
    pub(super) thumb_start: Option<(f32, f32, f32, f32)>,
    /// ⏱️ Monotonic seconds, advanced once per frame by `advance_clock`. Every hover-reveal deadline
    /// is an absolute value on this clock, never a countdown, so a dropped frame cannot lose time.
    pub(super) clock_seconds: f64,
    /// 🍃️ When the current hover leaf was entered, and whether its dwell already fired — so a
    /// tooltip opens exactly once per hover, not every frame after the deadline.
    pub hover_since: Option<(NodeId, f64)>,
    pub hover_revealed: Option<NodeId>,
    /// 🕵️ The disabled row action whose reason hint is shown ([`RowReasonHint`], audit W1E-1).
    pub(super) row_reason: Option<RowReasonHint>,
    /// ⌛️ Deadline armed by `maybe_dismiss_tooltip_on_hover_out` once the pointer leaves an open
    /// tooltip's anchor and bounds — see `DismissPolicy::hover_out_delay_seconds`.
    pub tooltip_dismiss_at: Option<f64>,
    /// 🔤️ The open `Select`'s live typeahead query and the `clock_seconds` of its last keystroke.
    /// React keeps the same buffer behind a 700 ms timer it restarts per key
    /// (`🧱️elements/🔽️Select/🟦️.tsx:661-666`); this expires it against the same monotonic clock
    /// every other reveal deadline here uses, so a dropped frame cannot lose time.
    pub(super) select_typeahead: Option<(String, f64)>,
    pub(super) stepper_repeat: Option<StepperRepeat>,
    pub(super) caret: Option<CaretClock>,
    /// 🔢️ The per-surface monotonic `seq` every fired intent carries — this window's own twin of
    /// `UiDocumentStore`'s `seq` counter (`📃️UiDocumentStore/🟦️.tsx`'s `buildIntent`). Minted here,
    /// enforced at the queue (`BoundedActionQueue::admit_intent`).
    pub(super) intents: UiIntentSequencer,
    /// 🌊️ This window's logical flow — the twin of React's `useFlow()`
    /// (`🔨️modules/🧭️flow-direction-context/🟦️.tsx:48`). `Rtl` mirrors the inline axis of every
    /// anchored overlay this router places and of every inline arrow key it routes; `Up` is read by
    /// paint, not here. Set from the window's dock anchor through `Ui::set_window_flow`, exactly as
    /// React derives it from `flowFromAnchor`.
    pub(super) flow: UiFlow,
    /// ⌨️ Whether the LAST input this router saw was a key rather than a pointer — React's
    /// `:focus-visible` in one bit. A focus ring paints only while this is set, so a clicked control
    /// gets focus without the keyboard ring (`🖌️paint`'s focus-ring sites read `focus_visible`).
    pub(super) focus_visible: bool,
}
//#endregion 🔖️UiCommand
// #endregion events
