// #region host
//! 🪟️ winit window event bridge into pointer callbacks.

use crate::wgpu::input::{KeyAction, PointerCallbacks, PointerModifiers};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, NamedKey};

pub fn pointer_coords(_window: &winit::window::Window, position: winit::dpi::PhysicalPosition<f64>) -> (f32, f32) {
    (position.x as f32, position.y as f32)
}

pub fn modifiers_from_winit(modifiers: winit::keyboard::ModifiersState) -> PointerModifiers {
    PointerModifiers { shift: modifiers.shift_key(), ctrl: modifiers.control_key(), alt: modifiers.alt_key(), meta: modifiers.super_key() }
}

#[derive(Default)]
pub struct WindowInputState {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub pointer_down: bool,
    pub pointer_button: i16,
    pub modifiers: PointerModifiers,
}

pub fn dispatch_window_event(window: &winit::window::Window, event: &WindowEvent, input: &mut WindowInputState, callbacks: &PointerCallbacks) -> bool {
    match event {
        WindowEvent::ModifiersChanged(modifiers) => {
            input.modifiers = modifiers_from_winit(modifiers.state());
            true
        }
        WindowEvent::CursorMoved { position, .. } => {
            let (x, y) = pointer_coords(window, *position);
            input.pointer_x = x;
            input.pointer_y = y;
            (callbacks.on_move)(x, y, input.pointer_down, input.pointer_button, input.modifiers.clone());
            true
        }
        WindowEvent::MouseInput { state, button, .. } => {
            let down = *state == ElementState::Pressed;
            let btn = mouse_button_to_i16(*button);
            if down {
                input.pointer_down = true;
                input.pointer_button = btn;
            } else if input.pointer_down {
                input.pointer_down = false;
            }
            (callbacks.on_button)(input.pointer_x, input.pointer_y, down, btn, input.modifiers.clone());
            true
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let delta_y = match delta {
                MouseScrollDelta::LineDelta(_, y) => *y * 40.0,
                MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
            };
            (callbacks.on_wheel)(delta_y, input.pointer_x, input.pointer_y, input.modifiers.clone());
            true
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if let Key::Named(NamedKey::Space) = &event.logical_key {
                (callbacks.on_key)(KeyAction::Space(event.state == ElementState::Pressed), input.modifiers.clone());
                return true;
            }
            if event.state != ElementState::Pressed {
                return true;
            }
            let action = key_action_from_event(event);
            if let Some(action) = action {
                (callbacks.on_key)(action, input.modifiers.clone());
            }
            true
        }
        _ => false,
    }
}

fn mouse_button_to_i16(button: MouseButton) -> i16 {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 2,
        MouseButton::Middle => 1,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(id) => id as i16,
    }
}

fn key_action_from_event(event: &KeyEvent) -> Option<KeyAction> {
    match &event.logical_key {
        Key::Named(NamedKey::Backspace) => Some(KeyAction::Backspace),
        Key::Named(NamedKey::Delete) => Some(KeyAction::Delete),
        Key::Named(NamedKey::Enter) => Some(KeyAction::Enter),
        Key::Named(NamedKey::Escape) => Some(KeyAction::Escape),
        Key::Named(NamedKey::ArrowLeft) => Some(KeyAction::ArrowLeft),
        Key::Named(NamedKey::ArrowRight) => Some(KeyAction::ArrowRight),
        Key::Named(NamedKey::ArrowUp) => Some(KeyAction::ArrowUp),
        Key::Named(NamedKey::ArrowDown) => Some(KeyAction::ArrowDown),
        Key::Named(NamedKey::F11) => Some(KeyAction::Function(11)),
        Key::Named(NamedKey::Tab) => Some(KeyAction::Tab),
        Key::Character(ch) if ch.chars().count() == 1 => Some(KeyAction::Char(ch.to_string())),
        _ => None,
    }
}

//#region 🔖️ClipboardHost
/** 📋️ OS clipboard write for `events::UiCommand::ClipboardCopy`/`ClipboardCut` — a caller (e.g.
 * `framework/renderer/wgpu`'s `interpreter::apply_ui_commands`) hands over the already-computed
 * copied/cut `text` and this fn is the ONLY thing in either engine that touches a real clipboard
 * backend, matching this crate's "wrap external libraries behind an interface, never leak the
 * library's own types past it" convention. Native wraps `arboard::Clipboard::set_text` (silently
 * no-ops without a display/clipboard, e.g. headless CI — `Clipboard::new()`'s `Err` is swallowed
 * rather than propagated, since there is no sensible way for a UI copy gesture to surface a clipboard
 * backend failure back through this call chain). Wasm fires the async Clipboard API's `writeText`
 * without awaiting it: the underlying `Promise` already starts executing the instant it's created, so
 * not awaiting it just means this fn doesn't itself learn whether the write ultimately succeeded —
 * exactly like a browser's own Ctrl+C, which never blocks the UI thread on the OS clipboard settling. */
#[cfg(not(target_arch = "wasm32"))]
pub fn clipboard_write_text(text: &str) {
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        let _ = clipboard.set_text(text.to_string());
    }
}

#[cfg(target_arch = "wasm32")]
pub fn clipboard_write_text(text: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(text);
    }
}

/** 📋️ Blocking OS clipboard read for `events::UiCommand::ClipboardPasteRequested` — native only:
 * `arboard::Clipboard::get_text` is itself synchronous, so a caller can read the OS clipboard and
 * feed the result straight back into `engine::Ui::dispatch_event` as a `events::UiEvent::Paste`
 * within the very same call. `None` on any failure (no clipboard backend, or the clipboard doesn't
 * currently hold text) — a caller treats that identically to "user pasted nothing". */
#[cfg(not(target_arch = "wasm32"))]
pub fn clipboard_read_text() -> Option<String> {
    arboard::Clipboard::new().ok()?.get_text().ok()
}

/** 📋️ The wasm mirror of `clipboard_read_text` above — `async` because the browser's Clipboard API
 * is Promise-based with no synchronous escape hatch; a caller drives this from a
 * `wasm_bindgen_futures::spawn_local` task (see `report-w3-clipboard-dnd.md`), since the OS
 * clipboard permission prompt/read can't resolve within one synchronous per-frame call. */
#[cfg(target_arch = "wasm32")]
pub async fn clipboard_read_text() -> Option<String> {
    let promise = web_sys::window()?.navigator().clipboard().read_text();
    wasm_bindgen_futures::JsFuture::from(promise).await.ok()?.as_string()
}
//#endregion 🔖️ClipboardHost
// #endregion host
