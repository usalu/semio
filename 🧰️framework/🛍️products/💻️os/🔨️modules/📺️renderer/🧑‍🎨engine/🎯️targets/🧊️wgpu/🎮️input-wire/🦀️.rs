//! 🎮️ The browser input WIRE vocabulary and its stateless projection onto `DispatchEvent`.
//!
//! Split out of `../🌐️browser-worker/🦀️.rs` because none of it is a browser capability: these are
//! plain data declarations plus a total function, and only the `#[wasm_bindgen]` host that decodes a
//! `postMessage` payload into them is `wasm32`-only. Mounted unconditionally so the law over
//! `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` — the same oracle the TypeScript twin in
//! `🚚️browser-frame-transport/🟦️.ts` answers from the other side — runs in an ordinary native
//! `cargo test` instead of only inside a browser.
//!
//! The serde spelling IS the contract: `#[serde(tag = "kind", rename_all = "kebab-case",
//! rename_all_fields = "camelCase")]` is what makes `{"kind":"pointer-down","pointerId":1,…}` — the
//! exact bytes `browserFrameEventFromDom` produces — decode here. Coordinates arrive already in
//! physical pixels; this side never scales.

use serde::Deserialize;
use ui_render::{DispatchEvent, EventModifiers, ImeEvent, PointerButton, PointerId, PointerInfo, PointerKind};

#[derive(Deserialize)]
pub(crate) struct BrowserBatch {
    pub(crate) replaceable: Vec<BrowserWireEvent>,
    pub(crate) lossless: Vec<BrowserWireEvent>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub(crate) enum BrowserWireEvent {
    PointerMove {
        pointer_id: u64,
        pointer_kind: BrowserPointerKind,
        x: f32,
        y: f32,
        pressure: Option<f32>,
        tilt_x: Option<f32>,
        tilt_y: Option<f32>,
    },
    PointerDown {
        pointer_id: u64,
        pointer_kind: BrowserPointerKind,
        x: f32,
        y: f32,
        pressure: Option<f32>,
        tilt_x: Option<f32>,
        tilt_y: Option<f32>,
        button: BrowserPointerButton,
    },
    PointerUp {
        pointer_id: u64,
        pointer_kind: BrowserPointerKind,
        x: f32,
        y: f32,
        pressure: Option<f32>,
        tilt_x: Option<f32>,
        tilt_y: Option<f32>,
        button: BrowserPointerButton,
    },
    Wheel {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
    },
    Resize {
        width: u32,
        height: u32,
        dpr: f32,
    },
    KeyDown {
        key: String,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
    },
    KeyUp {
        key: String,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
    },
    ImeStart,
    ImeCancel,
    TextChunk {
        stream_id: u64,
        target: TextTarget,
        text: String,
        total_bytes: usize,
        #[serde(rename = "final")]
        final_: bool,
        cursor: Option<usize>,
    },
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum TextTarget {
    Text,
    Paste,
    ImeUpdate,
    ImeCommit,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum BrowserPointerKind {
    Mouse,
    Touch,
    Pen,
    Eraser,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum BrowserPointerButton {
    Primary,
    Secondary,
    Middle,
}

pub(crate) fn pointer(id: u64, kind: BrowserPointerKind, pressure: Option<f32>, tilt_x: Option<f32>, tilt_y: Option<f32>) -> PointerInfo {
    PointerInfo { id: PointerId(id), kind: kind.into(), pressure, tilt: tilt_x.zip(tilt_y) }
}

impl From<BrowserPointerKind> for PointerKind {
    fn from(value: BrowserPointerKind) -> Self {
        match value {
            BrowserPointerKind::Mouse => Self::Mouse,
            BrowserPointerKind::Touch => Self::Touch,
            BrowserPointerKind::Pen => Self::Pen,
            BrowserPointerKind::Eraser => Self::Eraser,
        }
    }
}

impl From<BrowserPointerButton> for PointerButton {
    fn from(value: BrowserPointerButton) -> Self {
        match value {
            BrowserPointerButton::Primary => Self::Primary,
            BrowserPointerButton::Secondary => Self::Secondary,
            BrowserPointerButton::Middle => Self::Middle,
        }
    }
}

/// 🎯️ Projects one wire event onto the `DispatchEvent` `Ui::dispatch_event` receives, for every wire
/// event whose meaning is complete on its own.
///
/// `None` is not a refusal — it names the two wire events that are NOT a dispatch: a `resize` is the
/// surface's metrics (`WindowDelegate::handle_metrics`, not the event queue), and a `text-chunk` is one
/// page of a segmented text stream whose meaning depends on the Worker's own stream table. Both stay
/// with the host that owns that state.
pub(crate) fn stateless_dispatch(event: &BrowserWireEvent) -> Option<DispatchEvent> {
    Some(match event {
        BrowserWireEvent::PointerMove { pointer_id, pointer_kind, x, y, pressure, tilt_x, tilt_y } => {
            DispatchEvent::PointerMove { pointer: pointer(*pointer_id, *pointer_kind, *pressure, *tilt_x, *tilt_y), x: *x, y: *y }
        }
        BrowserWireEvent::PointerDown { pointer_id, pointer_kind, x, y, pressure, tilt_x, tilt_y, button } => {
            DispatchEvent::PointerDown { pointer: pointer(*pointer_id, *pointer_kind, *pressure, *tilt_x, *tilt_y), x: *x, y: *y, button: (*button).into() }
        }
        BrowserWireEvent::PointerUp { pointer_id, pointer_kind, x, y, pressure, tilt_x, tilt_y, button } => {
            DispatchEvent::PointerUp { pointer: pointer(*pointer_id, *pointer_kind, *pressure, *tilt_x, *tilt_y), x: *x, y: *y, button: (*button).into() }
        }
        BrowserWireEvent::Wheel { x, y, delta_x, delta_y } => DispatchEvent::Scroll { x: *x, y: *y, delta_x: *delta_x, delta_y: *delta_y },
        BrowserWireEvent::KeyDown { key, shift, ctrl, alt, meta } => DispatchEvent::KeyDown { key: key.clone(), modifiers: EventModifiers { shift: *shift, ctrl: *ctrl, alt: *alt, meta: *meta } },
        BrowserWireEvent::KeyUp { key, shift, ctrl, alt, meta } => DispatchEvent::KeyUp { key: key.clone(), modifiers: EventModifiers { shift: *shift, ctrl: *ctrl, alt: *alt, meta: *meta } },
        BrowserWireEvent::ImeStart => DispatchEvent::Ime(ImeEvent::Start),
        BrowserWireEvent::ImeCancel => DispatchEvent::Ime(ImeEvent::Cancel),
        BrowserWireEvent::Resize { .. } | BrowserWireEvent::TextChunk { .. } => return None,
    })
}

#[cfg(test)]
#[path = "../../../🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs"]
mod browser_input_wire_tests;
