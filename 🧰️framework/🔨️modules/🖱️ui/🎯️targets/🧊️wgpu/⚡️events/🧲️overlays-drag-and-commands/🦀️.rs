//! 🧲️ Overlays, drag and commands: the overlay stack and placement, the tooltip and caret clock steps, the drag session, the text helpers, `UiCommand` and the `EventRouter` state. See [`super`].

use super::*;

// #region events
//#region 🔖️Overlay
// 🪟️ One overlay mechanism for Select popups, context menus, tooltips, dialogs and the command palette; the caller reconciles the content subtree and hands over its root, this module owns the lifecycle.
/// ⚓️ What an overlay is positioned relative to: an existing node (a `Select`'s trigger, a hovered
/// row) or a raw point (where a context menu was right-clicked, where the pointer was when a
/// tooltip's hover-delay fired).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayAnchor {
    Node(NodeId),
    Point { x: f32, y: f32 },
}

/// 🛑️ The dismissals an owner prevented, React's `event.preventDefault()` inside `onEscapeKeyDown`, `onPointerDownOutside` and `onCloseAutoFocus`: a prevented dismissal leaves the overlay open (or focus where it is) and, for an outside press, does not swallow the press.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OverlayVeto {
    pub escape: bool,
    pub outside_press: bool,
    pub focus_return: bool,
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
    /// 🔙️ The node holding focus when the overlay opened — where focus returns on close when the anchor is not a focusable trigger (React's `restoreFocusRef`).
    pub return_focus: Option<NodeId>,
    pub veto: OverlayVeto,
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

// 💡️ React's `ChromeControlHint` dwell timer lives here: `advance_clock`, fed one monotonic seconds value per frame, fires the reveal and the `hover_out_delay_seconds` close.
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
// 🫳️ Drag session lifecycle: promoted from a `Press` capture past `DRAG_PROMOTE_THRESHOLD_SQ`, updated on `PointerMove`, committed or cancelled on `PointerUp`.
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
// 🖱️ Wheel events route to the nearest `NodeFlags::SCROLLABLE` ancestor; its scrollbar is derived from its own extent ([`scrollbar_for`]).
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

/// 📜️ The scrollbar of one axis `scrollable` shows: its viewport is its own placed rect, its content the furthest extent of its children, exactly the extent wheel scrolling clamps to; `None` when everything fits.
pub fn scrollbar_for(tree: &UiTree, scrollable: NodeId, vertical: bool, rtl: bool) -> Option<semio_ui_wgpu_layout::layout::Scrollbar> {
    let viewport = node_abs_rect(tree, scrollable)?;
    let content = tree.children(scrollable).filter_map(|child| tree.accepted_layout(child)).fold(0.0_f32, |extent, child| extent.max(if vertical { child.y + child.height } else { child.x + child.width }));
    let (x, y) = tree.node(scrollable)?.state.scroll_offset;
    semio_ui_wgpu_layout::layout::scrollbar(viewport, content, if vertical { y } else { x }, vertical, rtl)
}
//#endregion 🔖️Scroll

//#region 🔖️EditRouting
// ✍️ Key routing for a focused editable node's `tree::EditState` (byte-offset caret and anchor).
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

/// 🔠️ The class a scalar belongs to for word motion: blank, word or punctuation.
fn word_class(ch: char) -> u8 {
    if ch.is_whitespace() {
        0
    } else if ch.is_alphanumeric() || ch == '_' {
        1
    } else {
        2
    }
}

/// ⬅️ The start of the word at or before `index`: blanks are skipped backwards, then one run of a class.
pub(super) fn word_start_before(text: &str, index: usize) -> usize {
    let head = &text[..index];
    let trimmed = head.trim_end_matches(|ch| word_class(ch) == 0);
    match trimmed.chars().next_back().map(word_class) {
        Some(class) => trimmed.trim_end_matches(|ch| word_class(ch) == class).len(),
        None => 0,
    }
}

/// ➡️ The end of the word at or after `index`: blanks are skipped forwards, then one run of a class.
pub(super) fn word_end_after(text: &str, index: usize) -> usize {
    let tail = &text[index..];
    let trimmed = tail.trim_start_matches(|ch| word_class(ch) == 0);
    let skipped = tail.len() - trimmed.len();
    match trimmed.chars().next().map(word_class) {
        Some(class) => index + skipped + trimmed.len() - trimmed.trim_start_matches(|ch| word_class(ch) == class).len(),
        None => text.len(),
    }
}

/// ⏩ The start of the next word after `index`: one run of a class, then the blanks that follow it.
pub(super) fn word_start_after(text: &str, index: usize) -> usize {
    let tail = &text[index..];
    let run = match tail.chars().next().map(word_class) {
        Some(0) | None => 0,
        Some(class) => tail.len() - tail.trim_start_matches(|ch| word_class(ch) == class).len(),
    };
    let rest = &tail[run..];
    index + run + rest.len() - rest.trim_start_matches(|ch| word_class(ch) == 0).len()
}

/// 🔤️ The word, blank run or punctuation run around `index` (a click's caret position).
pub(super) fn word_range_at(text: &str, index: usize) -> (usize, usize) {
    let class = text[index..].chars().next().map(word_class).or_else(|| text[..index].chars().next_back().map(word_class));
    let Some(class) = class else { return (index, index) };
    let start = text[..index].trim_end_matches(|ch| word_class(ch) == class).len();
    let tail = &text[index..];
    (start, index + tail.len() - tail.trim_start_matches(|ch| word_class(ch) == class).len())
}

/// 🏁️ The start of the logical line holding `index`.
pub(super) fn line_start(text: &str, index: usize) -> usize {
    text[..index].rfind('\n').map_or(0, |at| at + 1)
}

/// 🛑️ The end of the logical line holding `index`.
pub(super) fn line_end(text: &str, index: usize) -> usize {
    text[index..].find('\n').map_or(text.len(), |at| index + at)
}

/// ↕️ The caret one logical line up or down, keeping its column in scalars, `None` past the first or last line.
pub(super) fn vertical_target(text: &str, index: usize, down: bool) -> Option<usize> {
    let start = line_start(text, index);
    let column = text[start..index].chars().count();
    let (target_start, target_end) = if down {
        let end = line_end(text, index);
        if end >= text.len() {
            return None;
        }
        (end + 1, line_end(text, end + 1))
    } else {
        if start == 0 {
            return None;
        }
        (line_start(text, start - 1), start - 1)
    };
    Some(text[target_start..target_end].char_indices().nth(column).map_or(target_end, |(offset, _)| target_start + offset))
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
    DropCommitted { window_id: String, source: NodeId, target: NodeId, y: f32, payload: DragPayload },
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
    Scene { window_id: String, node: NodeId, surface_id: String, kind: SurfaceKind, rect: Rect, event: UiEvent, click_count: u32 },
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

/// ⏱️ The interval within which a press continues the click chain of the one before: the OS double-click time every browser reads for DOM `MouseEvent.detail` (winit has no query for it, so its default).
pub(super) const DOUBLE_CLICK_SECONDS: f64 = 0.5;

/// 📏️ How far, per axis in logical pixels, a press may land from the one before and still continue its click chain: the OS double-click slop.
pub(super) const DOUBLE_CLICK_SLOP_PX: f32 = 4.0;

/// 🖱️ The latest primary or secondary press of a click chain: `count` is DOM `MouseEvent.detail` (1 single, 2 double, 3 triple, saturating), the number a `click` handler reads.
#[derive(Clone, Copy, Debug)]
pub(super) struct ClickChain {
    pub(super) seconds: f64,
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) button: PointerButton,
    pub(super) count: u32,
}

/// ✋️ A held press that extends its text selection: `unit` is the click count that began it (1 by character, 2 by word, 3 by line) and `origin` the span that click selected.
#[derive(Clone, Copy, Debug)]
pub(super) struct TextDrag {
    pub(super) node: NodeId,
    pub(super) unit: u8,
    pub(super) origin: (usize, usize),
}

/// ⏳️ A text press whose field had no laid-out text yet (no frame ran since it was first hovered or focused): the caret waits for the layout and then goes where the pointer was.
#[derive(Clone, Copy, Debug)]
pub(super) struct PendingTextPress {
    pub(super) node: NodeId,
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) extend: bool,
    pub(super) unit: u8,
}

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
    pub(super) hover_pointer: Option<(f32, f32)>,
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
    /// 🖱️ The click chain of the latest press, for every double and triple click rule.
    pub(super) click_chain: Option<ClickChain>,
    /// ✋️ The text field whose selection a held primary press is dragging.
    pub(super) text_drag: Option<TextDrag>,
    /// ⏳️ The text press waiting for its field's first layout.
    pub(super) text_pending: Option<PendingTextPress>,
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
