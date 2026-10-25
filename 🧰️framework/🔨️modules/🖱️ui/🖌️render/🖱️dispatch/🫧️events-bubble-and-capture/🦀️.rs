//! 🫧️ Events, bubble and capture: the pointer, ime and accessibility events, the `DispatchEvent`, the bubble walk and the pointer capture entries. See [`super`].

use super::*;

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

/// ℹ️ Per-pointer identity plus whatever the device reports beyond x/y — `pressure`/`tilt` are `None`
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
    PasteImageDataUrl,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessibilityTarget {
    pub window_id: String,
    pub window_generation: u64,
    pub node_id: u64,
    pub node_key: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccessibilityEvent {
    Focus,
    Blur,
    Activate,
    Value(String),
}

/// 📥️ Input events a host feeds into [`Dispatcher::dispatch`] — the same shape as
/// `events.rs::UiEvent`, generalized to carry a full [`PointerInfo`] instead of bare x/y (U3: multi-
/// pointer capable, never a winit type).
#[derive(Clone, Debug, PartialEq)]
pub enum DispatchEvent {
    PointerCancel { pointer: PointerInfo },
    PointerDown { pointer: PointerInfo, x: f32, y: f32, button: PointerButton, modifiers: EventModifiers },
    PointerUp { pointer: PointerInfo, x: f32, y: f32, button: PointerButton, modifiers: EventModifiers },
    PointerMove { pointer: PointerInfo, x: f32, y: f32, modifiers: EventModifiers },
    Scroll { x: f32, y: f32, delta_x: f32, delta_y: f32, modifiers: EventModifiers },
    KeyDown { key: String, modifiers: EventModifiers },
    KeyUp { key: String, modifiers: EventModifiers },
    TextInput { text: String },
    Paste { text: String },
    TextEditStart { stream: u64, target: TextEditTarget, declared_bytes: usize },
    TextEditChunk { stream: u64, text: String },
    TextEditCommit { stream: u64 },
    TextEditAbort { stream: u64 },
    Ime(ImeEvent),
    Accessibility { target: AccessibilityTarget, event: AccessibilityEvent },
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn event_pointer(event: &DispatchEvent) -> Option<PointerId> {
    match event {
        DispatchEvent::PointerDown { pointer, .. } | DispatchEvent::PointerUp { pointer, .. } | DispatchEvent::PointerMove { pointer, .. } | DispatchEvent::PointerCancel { pointer } => Some(pointer.id),
        _ => None,
    }
}
//#endregion 📥️DispatchEvent

//#region 🫧️Bubble
/// 🗨️ Ported from `events.rs::bubble` verbatim: walks from `from` up through `parent` links
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
pub(super) fn is_descendant(tree: &DispatchTree, id: FrameNodeId, ancestor: FrameNodeId) -> bool {
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
pub(super) struct CaptureEntry {
    pub(super) element: ElementId,
    pub(super) kind: CaptureKind,
    pub(super) revision: UiRevision,
}

/// 🔢️ Ported from master.md's own rule ("Stale intents (revision < current − 1) are dropped"): a
/// capture/listener revision more than one behind the tree it is now being resolved against is stale.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(super) fn is_stale(recorded: UiRevision, current: UiRevision) -> bool {
    current.0 > recorded.0.saturating_add(1)
}
//#endregion 🔒️Capture
//#endregion 🔖️Propagation
