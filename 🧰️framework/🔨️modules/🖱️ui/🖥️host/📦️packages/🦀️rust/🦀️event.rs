//! @emoji 🎛️ Platform event normalization — the layer that stops `winit` and `web_sys` types from
//! leaking upward. A caller in `🦀️window.rs` (same crate) feeds this module the raw pieces of a
//! platform event; this module hands back `ui_render`'s own multi-pointer, physical/logical-key
//! vocabulary (`ui_render::dispatch::DispatchEvent` and friends). Nothing above `ui_host` ever names
//! a `winit` or `web_sys` type — a product/runtime crate consuming [`crate::window::WindowDelegate`]
//! only ever sees the types re-exported from here.
//!
//! Every function below takes primitives or `winit`/web values *by value or reference* and returns a
//! plain-data normalized type — deliberately not a whole `winit::event::KeyEvent`/`WindowEvent`,
//! several of whose fields are `pub(crate)` in `winit` 0.30 and cannot be constructed outside it. That
//! split is also what keeps every mapping in this file unit-testable without a real window: construct
//! a `winit::keyboard::PhysicalKey`, a `winit::event::Touch`, a `ModifiersState` bitflag directly, and
//! call the pure function.
//!
//! Ported mappings (`modifiers_from_winit`, mouse-button and wheel-delta handling) come from
//! `wgpu-old`'s `🎯️targets/🧊️wgpu/🦀️host.rs::modifiers_from_winit`/`dispatch_window_event` — real
//! mappings carried over verbatim, not reinvented — generalized to multi-pointer and to the browser's
//! own delta-mode/pointerType vocabulary alongside winit's.
//!
//! 🚫️async: every fn below is plain sync per ruling U1 — normalization never suspends. See ticket
//! 26/08/20 📌️important.md.

#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;

use ui_render::{DispatchEvent, EventModifiers, ImeEvent, PointerButton, PointerId, PointerInfo, PointerKind};

//#region 🔖️Host

//#region 🆔️PointerRegistry

/// 🆔️ Assigns each distinct native `winit::event::DeviceId` a small stable slot, so a mouse pointer
/// and every simultaneous touch/pen contact resolve to a distinct, session-stable [`PointerId`] —
/// U3's "multi-pointer from the start" rule. Browser pointers need no such registry: the Pointer
/// Events spec's own `pointerId` is already globally unique per active contact (see
/// [`pointer_id_from_web`]), so this type exists only on native targets.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
pub struct PointerRegistry {
    slots: HashMap<winit::event::DeviceId, u32>,
    next_slot: u32,
}

#[cfg(not(target_arch = "wasm32"))]
impl PointerRegistry {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self::default()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn slot(&mut self, device: winit::event::DeviceId) -> u32 {
        if let Some(&slot) = self.slots.get(&device) {
            return slot;
        }
        let slot = self.next_slot;
        self.next_slot += 1;
        self.slots.insert(device, slot);
        slot
    }

    /// 🖱️ A native platform exposes exactly one logical mouse pointer per device — no finger id to
    /// fold in, so the device's own slot is the whole identity.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn pointer_id_for_mouse(&mut self, device: winit::event::DeviceId) -> PointerId {
        PointerId((self.slot(device) as u64) << 32)
    }

    /// 👆️ A device slot in the high bits, the OS-assigned finger id in the low bits — two fingers on
    /// the *same* device (including two `DeviceId::dummy()` values in a test, which compare equal)
    /// still resolve to distinct ids because their finger ids differ.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn pointer_id_for_touch(&mut self, touch: &winit::event::Touch) -> PointerId {
        PointerId(((self.slot(touch.device_id) as u64) << 32) ^ touch.id)
    }
}

/// 🌐️ The browser's own `PointerEvent.pointerId` is already a globally unique per-contact identity
/// (mouse, each finger, each pen/eraser contact) — no registry needed, unlike native's `DeviceId` +
/// finger-id pair.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn pointer_id_from_web(pointer_id: i32) -> PointerId {
    PointerId(pointer_id as u64)
}

/// 🌐️ Maps the DOM `PointerEvent.pointerType` string onto [`PointerKind`]. The Pointer Events spec
/// has no `"eraser"` type of its own (an eraser contact is reported as `"pen"` with a side-button
/// flag) — a caller with access to that flag should override the result rather than this fn guessing.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn pointer_kind_from_web_type(pointer_type: &str) -> PointerKind {
    match pointer_type {
        "pen" => PointerKind::Pen,
        "touch" => PointerKind::Touch,
        _ => PointerKind::Mouse,
    }
}

//#endregion 🆔️PointerRegistry

//#region 🖲️Pointer construction

/// 🖊️ Native mouse pointer — no pressure/tilt on any platform winit reports mouse input from.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn pointer_info_for_mouse(registry: &mut PointerRegistry, device: winit::event::DeviceId) -> PointerInfo {
    PointerInfo { id: registry.pointer_id_for_mouse(device), kind: PointerKind::Mouse, pressure: None, tilt: None }
}

/// 👆️ Native touch pointer — `force` is normalized to `0.0..=1.0` by winit's own
/// [`winit::event::Force::normalized`] before it reaches us.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn pointer_info_for_touch(registry: &mut PointerRegistry, touch: &winit::event::Touch) -> PointerInfo {
    PointerInfo { id: registry.pointer_id_for_touch(touch), kind: PointerKind::Touch, pressure: touch.force.map(|f| f.normalized() as f32), tilt: None }
}

/// 🌐️ Browser pointer — `pressure`/tilt-x/tilt-y come straight off `PointerEvent`'s own fields; a
/// caller passes `0.0` pressure through as `None` since the DOM reports `0` for devices that don't
/// support it at all (same convention winit uses for a mouse).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn pointer_info_from_web(pointer_id: i32, pointer_type: &str, pressure: f32, tilt_x: f32, tilt_y: f32) -> PointerInfo {
    let kind = pointer_kind_from_web_type(pointer_type);
    let pressure = if kind == PointerKind::Mouse && pressure == 0.0 { None } else { Some(pressure) };
    let tilt = if tilt_x == 0.0 && tilt_y == 0.0 { None } else { Some((tilt_x, tilt_y)) };
    PointerInfo { id: pointer_id_from_web(pointer_id), kind, pressure, tilt }
}

//#endregion 🖲️Pointer construction

//#region 🖱️Buttons and modifiers

/// 🖱️ Ported from `host.rs`'s old `mouse_button_to_i16` — [`PointerButton`] only distinguishes
/// primary/secondary/middle (`events.rs`'s own closed set), so `Back`/`Forward`/`Other` map to
/// `None`: a caller drops those rather than inventing a fourth variant `ui_render` doesn't have.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn pointer_button_from_winit(button: winit::event::MouseButton) -> Option<PointerButton> {
    match button {
        winit::event::MouseButton::Left => Some(PointerButton::Primary),
        winit::event::MouseButton::Right => Some(PointerButton::Secondary),
        winit::event::MouseButton::Middle => Some(PointerButton::Middle),
        _ => None,
    }
}

/// 🌐️ The DOM's `MouseEvent.button` values (0/1/2 = primary/middle/secondary — note the DOM orders
/// middle before secondary, unlike winit's enum).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn pointer_button_from_web(button: i16) -> Option<PointerButton> {
    match button {
        0 => Some(PointerButton::Primary),
        1 => Some(PointerButton::Middle),
        2 => Some(PointerButton::Secondary),
        _ => None,
    }
}

/// ⌨️ Ported from `host.rs::modifiers_from_winit` verbatim, made sync (ruling U1 — the old fn was
/// `async` for no reason, since nothing in it ever suspended).
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn modifiers_from_winit(state: winit::keyboard::ModifiersState) -> EventModifiers {
    EventModifiers { shift: state.shift_key(), ctrl: state.control_key(), alt: state.alt_key(), meta: state.super_key() }
}

/// 🌐️ The DOM reports each modifier as its own boolean on every keyboard/mouse/pointer event — no
/// combined bitflag to unpack, so this is a plain field-for-field copy.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn modifiers_from_web(shift: bool, ctrl: bool, alt: bool, meta: bool) -> EventModifiers {
    EventModifiers { shift, ctrl, alt, meta }
}

//#endregion 🖱️Buttons and modifiers

//#region 🎡️Wheel

/// 🎡️ A line's worth of pixels for `LineDelta`/`DOM_DELTA_LINE` normalization — matches `host.rs`'s
/// old `* 40.0` constant verbatim so wheel feel doesn't shift under the port.
pub const WHEEL_LINE_HEIGHT_PX: f32 = 40.0;

/// 🌐️ [`WheelEvent.deltaMode`](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)
/// values the browser disagrees with itself about depending on device and OS wheel settings.
pub const DOM_DELTA_PIXEL: u32 = 0;
pub const DOM_DELTA_LINE: u32 = 1;
pub const DOM_DELTA_PAGE: u32 = 2;

/// 🎡️ Ported from `host.rs::dispatch_window_event`'s `MouseWheel` arm, generalized to both axes
/// (the old port only normalized `delta_y`).
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn normalize_wheel_delta_native(delta: winit::event::MouseScrollDelta) -> (f32, f32) {
    match delta {
        winit::event::MouseScrollDelta::LineDelta(x, y) => (x * WHEEL_LINE_HEIGHT_PX, y * WHEEL_LINE_HEIGHT_PX),
        winit::event::MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
    }
}

/// 🌐️ Normalizes a `WheelEvent`'s delta to pixels across all three `deltaMode`s browsers disagree
/// about: `DOM_DELTA_PIXEL` passes through, `DOM_DELTA_LINE` scales by [`WHEEL_LINE_HEIGHT_PX`], and
/// `DOM_DELTA_PAGE` scales by the caller's own `viewport` size (a "page" of scroll is one viewport).
/// An unrecognized mode degrades to pixel passthrough rather than panicking.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn normalize_wheel_delta_web(delta_x: f64, delta_y: f64, delta_mode: u32, viewport: (f32, f32)) -> (f32, f32) {
    match delta_mode {
        DOM_DELTA_LINE => (delta_x as f32 * WHEEL_LINE_HEIGHT_PX, delta_y as f32 * WHEEL_LINE_HEIGHT_PX),
        DOM_DELTA_PAGE => (delta_x as f32 * viewport.0, delta_y as f32 * viewport.1),
        _ => (delta_x as f32, delta_y as f32),
    }
}

//#endregion 🎡️Wheel

//#region ⌨️Keys

/// ⌨️ A hand-rolled, deliberately partial mirror of `winit::keyboard::KeyCode` — every key a product
/// keybinding realistically needs (letters, digits, the common editing/navigation keys, both-hand
/// modifiers, F1–F12) plus `Unidentified` for the ~180 remaining niche `KeyCode` variants (numpad,
/// media keys, IME composition keys, browser keys). Extending this is additive and never a breaking
/// rename, so starting partial is the honest choice over guessing at completeness up front.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhysicalKeyCode {
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Space,
    Enter,
    Backspace,
    Delete,
    Escape,
    Tab,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Home,
    End,
    PageUp,
    PageDown,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,
    /// 🔢️ `Function(11)` for F11, etc. — one variant instead of twelve near-identical ones.
    Function(u8),
    Unidentified,
}

/// ⌨️ Physical-key mapping — independent of active keyboard layout (ruling: "physical vs logical
/// distinction preserved"). `winit::keyboard::PhysicalKey::Unidentified` (a key winit couldn't
/// translate to any `KeyCode` at all) degrades to [`PhysicalKeyCode::Unidentified`] the same as any
/// `KeyCode` variant not yet listed above.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn physical_key_from_winit(key: winit::keyboard::PhysicalKey) -> PhysicalKeyCode {
    use winit::keyboard::{KeyCode as K, PhysicalKey};
    let PhysicalKey::Code(code) = key else {
        return PhysicalKeyCode::Unidentified;
    };
    match code {
        K::KeyA => PhysicalKeyCode::KeyA,
        K::KeyB => PhysicalKeyCode::KeyB,
        K::KeyC => PhysicalKeyCode::KeyC,
        K::KeyD => PhysicalKeyCode::KeyD,
        K::KeyE => PhysicalKeyCode::KeyE,
        K::KeyF => PhysicalKeyCode::KeyF,
        K::KeyG => PhysicalKeyCode::KeyG,
        K::KeyH => PhysicalKeyCode::KeyH,
        K::KeyI => PhysicalKeyCode::KeyI,
        K::KeyJ => PhysicalKeyCode::KeyJ,
        K::KeyK => PhysicalKeyCode::KeyK,
        K::KeyL => PhysicalKeyCode::KeyL,
        K::KeyM => PhysicalKeyCode::KeyM,
        K::KeyN => PhysicalKeyCode::KeyN,
        K::KeyO => PhysicalKeyCode::KeyO,
        K::KeyP => PhysicalKeyCode::KeyP,
        K::KeyQ => PhysicalKeyCode::KeyQ,
        K::KeyR => PhysicalKeyCode::KeyR,
        K::KeyS => PhysicalKeyCode::KeyS,
        K::KeyT => PhysicalKeyCode::KeyT,
        K::KeyU => PhysicalKeyCode::KeyU,
        K::KeyV => PhysicalKeyCode::KeyV,
        K::KeyW => PhysicalKeyCode::KeyW,
        K::KeyX => PhysicalKeyCode::KeyX,
        K::KeyY => PhysicalKeyCode::KeyY,
        K::KeyZ => PhysicalKeyCode::KeyZ,
        K::Digit0 => PhysicalKeyCode::Digit0,
        K::Digit1 => PhysicalKeyCode::Digit1,
        K::Digit2 => PhysicalKeyCode::Digit2,
        K::Digit3 => PhysicalKeyCode::Digit3,
        K::Digit4 => PhysicalKeyCode::Digit4,
        K::Digit5 => PhysicalKeyCode::Digit5,
        K::Digit6 => PhysicalKeyCode::Digit6,
        K::Digit7 => PhysicalKeyCode::Digit7,
        K::Digit8 => PhysicalKeyCode::Digit8,
        K::Digit9 => PhysicalKeyCode::Digit9,
        K::Space => PhysicalKeyCode::Space,
        K::Enter | K::NumpadEnter => PhysicalKeyCode::Enter,
        K::Backspace => PhysicalKeyCode::Backspace,
        K::Delete => PhysicalKeyCode::Delete,
        K::Escape => PhysicalKeyCode::Escape,
        K::Tab => PhysicalKeyCode::Tab,
        K::ArrowLeft => PhysicalKeyCode::ArrowLeft,
        K::ArrowRight => PhysicalKeyCode::ArrowRight,
        K::ArrowUp => PhysicalKeyCode::ArrowUp,
        K::ArrowDown => PhysicalKeyCode::ArrowDown,
        K::Home => PhysicalKeyCode::Home,
        K::End => PhysicalKeyCode::End,
        K::PageUp => PhysicalKeyCode::PageUp,
        K::PageDown => PhysicalKeyCode::PageDown,
        K::ShiftLeft => PhysicalKeyCode::ShiftLeft,
        K::ShiftRight => PhysicalKeyCode::ShiftRight,
        K::ControlLeft => PhysicalKeyCode::ControlLeft,
        K::ControlRight => PhysicalKeyCode::ControlRight,
        K::AltLeft => PhysicalKeyCode::AltLeft,
        K::AltRight => PhysicalKeyCode::AltRight,
        K::SuperLeft => PhysicalKeyCode::SuperLeft,
        K::SuperRight => PhysicalKeyCode::SuperRight,
        K::F1 => PhysicalKeyCode::Function(1),
        K::F2 => PhysicalKeyCode::Function(2),
        K::F3 => PhysicalKeyCode::Function(3),
        K::F4 => PhysicalKeyCode::Function(4),
        K::F5 => PhysicalKeyCode::Function(5),
        K::F6 => PhysicalKeyCode::Function(6),
        K::F7 => PhysicalKeyCode::Function(7),
        K::F8 => PhysicalKeyCode::Function(8),
        K::F9 => PhysicalKeyCode::Function(9),
        K::F10 => PhysicalKeyCode::Function(10),
        K::F11 => PhysicalKeyCode::Function(11),
        K::F12 => PhysicalKeyCode::Function(12),
        _ => PhysicalKeyCode::Unidentified,
    }
}

/// 🌐️ The DOM `KeyboardEvent.code` string is already a stable physical identifier (`"KeyW"`,
/// `"ArrowLeft"`, `"F11"`, …) — this parses that same vocabulary winit's `KeyCode` variant names
/// mirror, so native and web share one [`PhysicalKeyCode`] result for the same physical key.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn physical_key_from_web_code(code: &str) -> PhysicalKeyCode {
    match code {
        "KeyA" => PhysicalKeyCode::KeyA,
        "KeyB" => PhysicalKeyCode::KeyB,
        "KeyC" => PhysicalKeyCode::KeyC,
        "KeyD" => PhysicalKeyCode::KeyD,
        "KeyE" => PhysicalKeyCode::KeyE,
        "KeyF" => PhysicalKeyCode::KeyF,
        "KeyG" => PhysicalKeyCode::KeyG,
        "KeyH" => PhysicalKeyCode::KeyH,
        "KeyI" => PhysicalKeyCode::KeyI,
        "KeyJ" => PhysicalKeyCode::KeyJ,
        "KeyK" => PhysicalKeyCode::KeyK,
        "KeyL" => PhysicalKeyCode::KeyL,
        "KeyM" => PhysicalKeyCode::KeyM,
        "KeyN" => PhysicalKeyCode::KeyN,
        "KeyO" => PhysicalKeyCode::KeyO,
        "KeyP" => PhysicalKeyCode::KeyP,
        "KeyQ" => PhysicalKeyCode::KeyQ,
        "KeyR" => PhysicalKeyCode::KeyR,
        "KeyS" => PhysicalKeyCode::KeyS,
        "KeyT" => PhysicalKeyCode::KeyT,
        "KeyU" => PhysicalKeyCode::KeyU,
        "KeyV" => PhysicalKeyCode::KeyV,
        "KeyW" => PhysicalKeyCode::KeyW,
        "KeyX" => PhysicalKeyCode::KeyX,
        "KeyY" => PhysicalKeyCode::KeyY,
        "KeyZ" => PhysicalKeyCode::KeyZ,
        "Digit0" => PhysicalKeyCode::Digit0,
        "Digit1" => PhysicalKeyCode::Digit1,
        "Digit2" => PhysicalKeyCode::Digit2,
        "Digit3" => PhysicalKeyCode::Digit3,
        "Digit4" => PhysicalKeyCode::Digit4,
        "Digit5" => PhysicalKeyCode::Digit5,
        "Digit6" => PhysicalKeyCode::Digit6,
        "Digit7" => PhysicalKeyCode::Digit7,
        "Digit8" => PhysicalKeyCode::Digit8,
        "Digit9" => PhysicalKeyCode::Digit9,
        "Space" => PhysicalKeyCode::Space,
        "Enter" | "NumpadEnter" => PhysicalKeyCode::Enter,
        "Backspace" => PhysicalKeyCode::Backspace,
        "Delete" => PhysicalKeyCode::Delete,
        "Escape" => PhysicalKeyCode::Escape,
        "Tab" => PhysicalKeyCode::Tab,
        "ArrowLeft" => PhysicalKeyCode::ArrowLeft,
        "ArrowRight" => PhysicalKeyCode::ArrowRight,
        "ArrowUp" => PhysicalKeyCode::ArrowUp,
        "ArrowDown" => PhysicalKeyCode::ArrowDown,
        "Home" => PhysicalKeyCode::Home,
        "End" => PhysicalKeyCode::End,
        "PageUp" => PhysicalKeyCode::PageUp,
        "PageDown" => PhysicalKeyCode::PageDown,
        "ShiftLeft" => PhysicalKeyCode::ShiftLeft,
        "ShiftRight" => PhysicalKeyCode::ShiftRight,
        "ControlLeft" => PhysicalKeyCode::ControlLeft,
        "ControlRight" => PhysicalKeyCode::ControlRight,
        "AltLeft" => PhysicalKeyCode::AltLeft,
        "AltRight" => PhysicalKeyCode::AltRight,
        "MetaLeft" => PhysicalKeyCode::SuperLeft,
        "MetaRight" => PhysicalKeyCode::SuperRight,
        "F1" => PhysicalKeyCode::Function(1),
        "F2" => PhysicalKeyCode::Function(2),
        "F3" => PhysicalKeyCode::Function(3),
        "F4" => PhysicalKeyCode::Function(4),
        "F5" => PhysicalKeyCode::Function(5),
        "F6" => PhysicalKeyCode::Function(6),
        "F7" => PhysicalKeyCode::Function(7),
        "F8" => PhysicalKeyCode::Function(8),
        "F9" => PhysicalKeyCode::Function(9),
        "F10" => PhysicalKeyCode::Function(10),
        "F11" => PhysicalKeyCode::Function(11),
        "F12" => PhysicalKeyCode::Function(12),
        _ => PhysicalKeyCode::Unidentified,
    }
}

/// 🔤️ Logical-key mapping (layout- and modifier-affected, except Ctrl) into the plain `String`
/// [`DispatchEvent::KeyDown`]/`KeyUp` carry — normalized to match the W3C `KeyboardEvent.key` string
/// vocabulary both winit's `NamedKey` docs and the DOM itself already follow, so native and web
/// produce identical strings for identical logical keys.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn logical_key_to_dispatch_string(key: &winit::keyboard::Key) -> String {
    use winit::keyboard::Key;
    match key {
        Key::Character(s) => s.to_string(),
        Key::Named(named) => named_key_label(*named).to_string(),
        Key::Dead(Some(c)) => c.to_string(),
        _ => "Unidentified".to_string(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn named_key_label(named: winit::keyboard::NamedKey) -> &'static str {
    use winit::keyboard::NamedKey as N;
    match named {
        N::Enter => "Enter",
        N::Tab => "Tab",
        N::Space => " ",
        N::ArrowDown => "ArrowDown",
        N::ArrowLeft => "ArrowLeft",
        N::ArrowRight => "ArrowRight",
        N::ArrowUp => "ArrowUp",
        N::End => "End",
        N::Home => "Home",
        N::PageDown => "PageDown",
        N::PageUp => "PageUp",
        N::Backspace => "Backspace",
        N::Delete => "Delete",
        N::Escape => "Escape",
        N::Shift => "Shift",
        N::Control => "Control",
        N::Alt => "Alt",
        N::Meta | N::Super => "Meta",
        N::CapsLock => "CapsLock",
        N::ContextMenu => "ContextMenu",
        N::F1 => "F1",
        N::F2 => "F2",
        N::F3 => "F3",
        N::F4 => "F4",
        N::F5 => "F5",
        N::F6 => "F6",
        N::F7 => "F7",
        N::F8 => "F8",
        N::F9 => "F9",
        N::F10 => "F10",
        N::F11 => "F11",
        N::F12 => "F12",
        _ => "Unidentified",
    }
}

//#endregion ⌨️Keys

//#region 🈶️IME

/// 🈶️ Ported to `ui_render::ImeEvent`'s four-variant lifecycle. `Ime::Disabled` has no exact
/// counterpart — `ImeEvent` has no "disabled" state of its own, only `Start`/`Update`/`Commit`/
/// `Cancel` — so it maps to `Cancel`: the safest reading, since any in-flight composition must be
/// treated as abandoned the moment the platform IME turns itself off.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn ime_event_from_winit(event: winit::event::Ime) -> ImeEvent {
    match event {
        winit::event::Ime::Enabled => ImeEvent::Start,
        winit::event::Ime::Preedit(text, cursor) => {
            let cursor = cursor.map(|(start, _end)| start).unwrap_or(text.len());
            ImeEvent::Update { text, cursor }
        }
        winit::event::Ime::Commit(text) => ImeEvent::Commit { text },
        winit::event::Ime::Disabled => ImeEvent::Cancel,
    }
}

//#endregion 🈶️IME

//#region 📥️DispatchEvent assembly

/// 📥️ The small final step every normalized piece above feeds into — kept separate from the
/// per-platform extraction so both native and web share this one assembly point.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn key_dispatch_event(logical: String, modifiers: EventModifiers, pressed: bool) -> DispatchEvent {
    if pressed {
        DispatchEvent::KeyDown { key: logical, modifiers }
    } else {
        DispatchEvent::KeyUp { key: logical, modifiers }
    }
}

//#endregion 📥️DispatchEvent assembly

//#endregion 🔖️Host

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    //#region 🎡️Wheel tests

    #[test]
    fn native_line_delta_scales_by_line_height() {
        let (dx, dy) = normalize_wheel_delta_native(winit::event::MouseScrollDelta::LineDelta(1.0, -2.0));
        assert_eq!(dx, WHEEL_LINE_HEIGHT_PX);
        assert_eq!(dy, -2.0 * WHEEL_LINE_HEIGHT_PX);
    }

    #[test]
    fn native_pixel_delta_passes_through() {
        let (dx, dy) = normalize_wheel_delta_native(winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition::new(3.5, -7.25)));
        assert_eq!(dx, 3.5);
        assert_eq!(dy, -7.25);
    }

    #[test]
    fn web_pixel_mode_passes_through() {
        assert_eq!(normalize_wheel_delta_web(10.0, 20.0, DOM_DELTA_PIXEL, (800.0, 600.0)), (10.0, 20.0));
    }

    #[test]
    fn web_line_mode_scales_by_line_height() {
        assert_eq!(normalize_wheel_delta_web(1.0, 2.0, DOM_DELTA_LINE, (800.0, 600.0)), (WHEEL_LINE_HEIGHT_PX, 2.0 * WHEEL_LINE_HEIGHT_PX));
    }

    #[test]
    fn web_page_mode_scales_by_viewport() {
        assert_eq!(normalize_wheel_delta_web(1.0, 1.0, DOM_DELTA_PAGE, (800.0, 600.0)), (800.0, 600.0));
    }

    //#endregion 🎡️Wheel tests

    //#region ⌨️Modifier tests

    #[test]
    fn native_modifiers_map_every_flag() {
        let state = winit::keyboard::ModifiersState::SHIFT | winit::keyboard::ModifiersState::ALT;
        let modifiers = modifiers_from_winit(state);
        assert!(modifiers.shift);
        assert!(!modifiers.ctrl);
        assert!(modifiers.alt);
        assert!(!modifiers.meta);
    }

    #[test]
    fn web_modifiers_are_a_plain_copy() {
        let modifiers = modifiers_from_web(false, true, false, true);
        assert!(!modifiers.shift);
        assert!(modifiers.ctrl);
        assert!(!modifiers.alt);
        assert!(modifiers.meta);
    }

    //#endregion ⌨️Modifier tests

    //#region ⌨️Key tests

    /// 🇫🇷️ On an AZERTY layout the physical `KeyQ` position produces the logical character "a" — the
    /// whole point of the physical/logical split (see `KeyCode::KeyW`'s own docstring example).
    #[test]
    fn physical_vs_logical_key_mapping_stays_distinct_across_layouts() {
        let physical = physical_key_from_winit(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyQ));
        let logical = logical_key_to_dispatch_string(&winit::keyboard::Key::Character("a".into()));
        assert_eq!(physical, PhysicalKeyCode::KeyQ);
        assert_eq!(logical, "a");
    }

    #[test]
    fn named_keys_map_to_their_dom_key_string() {
        assert_eq!(logical_key_to_dispatch_string(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::Enter)), "Enter");
        assert_eq!(logical_key_to_dispatch_string(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::Space)), " ");
    }

    #[test]
    fn unidentified_physical_key_degrades_cleanly() {
        assert_eq!(physical_key_from_winit(winit::keyboard::PhysicalKey::Unidentified(winit::keyboard::NativeKeyCode::Unidentified)), PhysicalKeyCode::Unidentified);
    }

    #[test]
    fn native_and_web_agree_on_the_same_physical_key() {
        assert_eq!(physical_key_from_winit(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft)), physical_key_from_web_code("ArrowLeft"));
    }

    //#endregion ⌨️Key tests

    //#region 🈶️IME tests

    #[test]
    fn ime_preedit_with_no_cursor_range_falls_back_to_text_end() {
        let event = ime_event_from_winit(winit::event::Ime::Preedit("ab".into(), None));
        assert_eq!(event, ImeEvent::Update { text: "ab".into(), cursor: 2 });
    }

    #[test]
    fn ime_disabled_maps_to_cancel() {
        assert_eq!(ime_event_from_winit(winit::event::Ime::Disabled), ImeEvent::Cancel);
    }

    //#endregion 🈶️IME tests

    //#region 🆔️Multi-pointer tests

    #[test]
    fn two_simultaneous_touches_on_the_same_device_stay_distinct() {
        let mut registry = PointerRegistry::new();
        let touch_a = winit::event::Touch { device_id: winit::event::DeviceId::dummy(), phase: winit::event::TouchPhase::Started, location: winit::dpi::PhysicalPosition::new(10.0, 10.0), force: None, id: 1 };
        let touch_b = winit::event::Touch { device_id: winit::event::DeviceId::dummy(), phase: winit::event::TouchPhase::Started, location: winit::dpi::PhysicalPosition::new(20.0, 20.0), force: None, id: 2 };
        let a = pointer_info_for_touch(&mut registry, &touch_a);
        let b = pointer_info_for_touch(&mut registry, &touch_b);
        assert_ne!(a.id, b.id, "two simultaneous pointers must normalize to distinct ids");
        assert_eq!(a.kind, PointerKind::Touch);
    }

    #[test]
    fn a_mouse_and_a_touch_on_the_same_device_stay_distinct() {
        let mut registry = PointerRegistry::new();
        let mouse = pointer_info_for_mouse(&mut registry, winit::event::DeviceId::dummy());
        let touch = winit::event::Touch { device_id: winit::event::DeviceId::dummy(), phase: winit::event::TouchPhase::Started, location: winit::dpi::PhysicalPosition::new(0.0, 0.0), force: None, id: 0 };
        let touch_info = pointer_info_for_touch(&mut registry, &touch);
        assert_ne!(mouse.id, touch_info.id);
        assert_eq!(mouse.kind, PointerKind::Mouse);
    }

    #[test]
    fn web_pointer_ids_are_used_verbatim_and_stay_distinct() {
        let a = pointer_info_from_web(1, "touch", 0.5, 0.0, 0.0);
        let b = pointer_info_from_web(2, "touch", 0.5, 0.0, 0.0);
        assert_ne!(a.id, b.id);
        assert_eq!(a.id, pointer_id_from_web(1));
    }

    //#endregion 🆔️Multi-pointer tests

    //#region 🖱️Button tests

    #[test]
    fn native_and_web_button_mappings_agree_where_both_define_a_button() {
        assert_eq!(pointer_button_from_winit(winit::event::MouseButton::Left), Some(PointerButton::Primary));
        assert_eq!(pointer_button_from_web(0), Some(PointerButton::Primary));
        assert_eq!(pointer_button_from_winit(winit::event::MouseButton::Right), Some(PointerButton::Secondary));
        assert_eq!(pointer_button_from_web(2), Some(PointerButton::Secondary));
    }

    #[test]
    fn buttons_outside_the_closed_set_map_to_none() {
        assert_eq!(pointer_button_from_winit(winit::event::MouseButton::Back), None);
        assert_eq!(pointer_button_from_web(4), None);
    }

    //#endregion 🖱️Button tests
}

//#endregion Tests
