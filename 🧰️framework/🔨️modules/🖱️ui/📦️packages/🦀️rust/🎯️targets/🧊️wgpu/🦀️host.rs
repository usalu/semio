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
#[cfg(not(target_arch = "wasm32"))]
enum ClipboardIoOperation {
    Read,
    Write(String),
}

/// 📋️ Worker-owned native clipboard operation. Hosts submit it to the process `WorkerPool`
/// I/O lane and poll the returned receiver; no event callback executes or waits for `arboard`.
#[cfg(not(target_arch = "wasm32"))]
pub struct ClipboardIoJob {
    operation: Option<ClipboardIoOperation>,
    closing: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl ClipboardIoJob {
    pub fn read() -> Self {
        Self { operation: Some(ClipboardIoOperation::Read), closing: false }
    }

    pub fn write(text: String) -> Self {
        Self { operation: Some(ClipboardIoOperation::Write(text)), closing: false }
    }

    /// 📥️ Decodes a successful read candidate. Write candidates and empty clipboards return
    /// `None`; cancellation/fault/yield are not terminal results and also return `None`.
    pub fn read_candidate(outcome: &semio_framework_job::StepOutcome) -> Option<String> {
        let semio_framework_job::StepOutcome::Complete(candidate) = outcome else { return None };
        let (&present, bytes) = candidate.output.page(0)?.split_first()?;
        (present == 1).then(|| String::from_utf8(bytes.to_vec()).ok()).flatten()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_job::InteractiveJob for ClipboardIoJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        cx.set_stage("ClipboardIo");
        use arboard as system_clipboard;
        let output = match self.operation.take() {
            Some(ClipboardIoOperation::Read) => {
                let mut writer = semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput);
                let source = semio_framework_job::JobPayloadPageSource::new();
                let mut page = match cx.admit_payload_page(&mut writer, source) {
                    Ok(page) => page,
                    Err(_) => return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
                };
                let text = system_clipboard::Clipboard::new().ok().and_then(|mut clipboard| clipboard.get_text().ok());
                let write = match text.as_ref() {
                    Some(text) if text.len() < semio_framework_job::JOB_PAYLOAD_PAGE_BYTES => page.write(&[1]).and_then(|_| page.write(text.as_bytes())),
                    Some(_) => page.write(&[0]),
                    None => page.write(&[0]),
                };
                if write.is_err() {
                    return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
                }
                page.commit();
                writer.finish().unwrap_or_else(|_| semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput))
            }
            Some(ClipboardIoOperation::Write(text)) => {
                if let Ok(mut clipboard) = system_clipboard::Clipboard::new() {
                    let _ = clipboard.set_text(text);
                }
                semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput)
            }
            None => semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        };
        cx.consume_fuel(1);
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState), output })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if self.operation.is_some() {
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.operation = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.operation.is_none()
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn clipboard_write_text(text: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(text);
    }
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
