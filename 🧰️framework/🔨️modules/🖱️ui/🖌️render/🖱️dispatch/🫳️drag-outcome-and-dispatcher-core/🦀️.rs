//! 🫳️ Drag, outcome and dispatcher core: the focusable collection, the drag payload, ghost and session, the cursor and ime requests, the `DispatchOutcome`, the `Dispatcher` state and its accessors and registration. See [`super`].

use super::*;

//#region 🔖️Focus
/// 🎯️ Ported from `events.rs::collect_focusable`, generalized from matching `is_focusable(&UiNode)`'s
/// closed variant list to the generic [`DispatchFlags::FOCUSABLE`] flag — see this file's module
/// docstring for why that generalization is sound (an element declares its own focusability once,
/// dispatch never needs to know its product type).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn collect_focusable(tree: &DispatchTree, id: FrameNodeId, out: &mut Vec<ElementId>) {
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
pub(super) const DRAG_PROMOTE_THRESHOLD_SQ: f32 = 16.0;

/// 🎯️ A [`Dispatcher::set_drop_accept`] predicate — `dyn Fn` is U3-permitted (not a first-party
/// trait).
type DropAcceptPredicate = Box<dyn Fn(&DragPayload) -> bool>;

/// 🧬️ Converts a [`DragPayload`] into the neutral [`UiValue`] a `Trigger::Drop` binding's `input`
/// carries — the generic replacement for `events.rs`'s dedicated `UiCommand::DropCommitted` variant
/// (see this file's report for why: `Trigger::Drop` already exists in the contract's closed set, so a
/// drop commit is just an ordinary fired binding like any other, no special command needed).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn drag_payload_to_value(payload: &DragPayload) -> Option<UiValue> {
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
    pub(super) capture: HashMap<PointerId, CaptureEntry>,
    pub(super) press_origin: HashMap<PointerId, (f32, f32)>,
    pub(super) focus: Option<ElementId>,
    pub(super) hovered: Option<ElementId>,
    pub(super) hover_chain: Vec<ElementId>,
    pub(super) overlays: OverlayStack,
    pub(super) drag: Option<DragSession>,
    pub(super) drag_payloads: HashMap<ElementId, DragPayload>,
    pub(super) drop_accept: HashMap<ElementId, DropAcceptPredicate>,
    pub(super) scroll_thumbs: HashMap<ElementId, (ElementId, ScrollAxis)>,
    pub(super) thumb_start: HashMap<PointerId, (f32, f32, f32, f32)>,
    pub(super) scroll_offsets: HashMap<ElementId, (f32, f32)>,
    pub(super) edit_states: HashMap<ElementId, EditState>,
    pub(super) seq: u64,
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

}
//#endregion 🔖️Outcome
