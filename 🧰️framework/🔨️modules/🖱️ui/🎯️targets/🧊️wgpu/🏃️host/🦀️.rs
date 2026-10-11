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
        Key::Named(NamedKey::Home) => Some(KeyAction::Home),
        Key::Named(NamedKey::End) => Some(KeyAction::End),
        Key::Named(NamedKey::PageUp) => Some(KeyAction::PageUp),
        Key::Named(NamedKey::PageDown) => Some(KeyAction::PageDown),
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

#[cfg(not(target_arch = "wasm32"))]
pub enum ClipboardContent {
    Text(String),
    ImageRgba8 { width: u32, height: u32, bytes: Vec<u8> },
}

/// 📋️ Worker-owned native clipboard operation. Hosts submit it to the process `WorkerPool`
/// I/O lane and poll the returned receiver; no event callback executes or waits for `arboard`.
#[cfg(not(target_arch = "wasm32"))]
pub struct ClipboardIoJob {
    operation: std::mem::ManuallyDrop<Option<ClipboardIoOperation>>,
    closing: bool,
    completed: bool,
    cancelled: bool,
    fault: semio_framework_job::RetainedPayloadBuilder,
    fault_cursor: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl ClipboardIoJob {
    pub fn read() -> Self {
        Self { operation: std::mem::ManuallyDrop::new(Some(ClipboardIoOperation::Read)), closing: false, completed:false, cancelled:false, fault:semio_framework_job::RetainedPayloadBuilder::new(semio_framework_job::JobPayloadStream::Fault), fault_cursor:0 }
    }

    pub fn write(text: String) -> Self {
        Self { operation: std::mem::ManuallyDrop::new(Some(ClipboardIoOperation::Write(text))), closing: false, completed:false, cancelled:false, fault:semio_framework_job::RetainedPayloadBuilder::new(semio_framework_job::JobPayloadStream::Fault), fault_cursor:0 }
    }

    /// 📥️ Decodes a successful read candidate. Write candidates and empty clipboards return
    /// `None`; cancellation/fault/yield are not terminal results and also return `None`.
    pub fn read_candidate<'a>(outcome: &semio_framework_job::JobOutcomeView<'a>) -> Option<ClipboardContentBorrow<'a>> {
        let semio_framework_job::JobOutcomeView::Complete {output:Some(output),..}=outcome else{return None};
        decode_clipboard_content_borrow(output.page(0)?)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub enum ClipboardContentBorrow<'a>{Text(&'a str),ImageRgba8{width:u32,height:u32,bytes:&'a[u8]}}
#[cfg(not(target_arch = "wasm32"))]
fn decode_clipboard_content_borrow(page:&[u8])->Option<ClipboardContentBorrow<'_>>{
    let (&kind,bytes)=page.split_first()?;
    match kind{
        1=>std::str::from_utf8(bytes).ok().map(ClipboardContentBorrow::Text),
        2 if bytes.len()>=8=>{
            let width=u32::from_le_bytes(bytes[0..4].try_into().ok()?);
            let height=u32::from_le_bytes(bytes[4..8].try_into().ok()?);
            let expected=usize::try_from(width).ok()?.checked_mul(usize::try_from(height).ok()?)?.checked_mul(4)?;
            (bytes.len()==expected+8).then(||ClipboardContentBorrow::ImageRgba8{width,height,bytes:&bytes[8..]})
        }
        _=>None
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📋️clipboard-content/🦀️.rs"]
mod clipboard_content_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/♻️physical-job-close/🦀️.rs"]
pub(crate) mod physical_job_close_tests;

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_job::InteractiveJob for ClipboardIoJob {
    fn step<'a>(&'a mut self, cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_value::ValueError>{
        use semio_framework_job::JobOutcomeBorrow;
        let grant=cx.retained_grant();
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if self.cancelled||cx.is_cancelled(){
            let outcome=JobOutcomeBorrow::admit_cancelled(cx)?;
            if outcome.is_some(){self.cancelled=true}
            return Ok(outcome)
        }
        if self.completed{return JobOutcomeBorrow::admit_complete(cx,None,None)}
        if cx.should_yield(){return JobOutcomeBorrow::admit_yield(cx)}
        cx.set_stage("ClipboardIo");
        if let Some(ClipboardIoOperation::Write(text))=self.operation.as_ref(){
            let mut clipboard=arboard::Clipboard::new().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native clipboard write surface unavailable"))?;
            clipboard.set_text(text.as_str()).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native clipboard write failed"))?;
            self.completed=true;
            cx.consume_fuel(1);
            return JobOutcomeBorrow::admit_complete(cx,None,None)
        }
        let diagnostic=b"native clipboard read requires original bounded platform recipient";
        if !self.fault.advance_initialization(cx)?{return Ok(None)}
        if !self.fault.append_original(cx,diagnostic,&mut self.fault_cursor)?{return Ok(None)}
        if !self.fault.seal(cx)?{return Ok(None)}
        JobOutcomeBorrow::admit_fault(cx,self.fault.published().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original clipboard diagnostic was not sealed"))?)
    }

    fn borrow_outcome<'a>(&'a self, descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{
        match descriptor.kind(){
            semio_framework_job::JobOutcomeKind::Cancelled=>descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete if self.completed=>descriptor.complete(None,None),
            semio_framework_job::JobOutcomeKind::Yield=>descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Fault=>descriptor.fault(self.fault.published().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original clipboard fault absent"))?),
            _=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"clipboard descriptor does not match original outcome"))
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,grant:semio_framework_job::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep{
        use semio_framework_job::{InteractiveJobCloseStep as Close,RetainedCloneProgress as Progress,RetainedCloneStep};
        if !self.closing{return Close::Blocked}
        if !self.fault.terminal_is_empty(){
            let demand=match self.fault.retirement_demands(){Ok(demand)=>demand,Err(error)=>return Close::Refused{kind:error.kind,progress:error.retained_progress()}};
            if grant.maximum_depth<=demand.depth{return Close::Pending{progress:Progress::default()}}
            let child=semio_framework_job::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
            return match self.fault.close_step_granted(child){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Close::Pending{progress},Err(error)=>Close::Refused{kind:error.kind,progress:error.retained_progress()}}
        }
        if self.operation.is_none(){return Close::Complete{progress:Progress::default()}}
        let bytes=match self.operation.as_ref(){Some(ClipboardIoOperation::Write(text))=>text.capacity(),_=>0};
        if grant.maximum_items==0||bytes>grant.maximum_release_bytes||(bytes!=0&&grant.maximum_depth==0){return Close::Pending{progress:Progress::default()}}
        *self.operation=None;
        Close::Complete{progress:Progress{copied_items:1,released_bytes:bytes,..Progress::default()}}
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return Ok(self.fault.retirement_demands()?.copy_bytes)}Ok(0)}
    fn next_close_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return Ok(self.fault.retirement_demands()?.capacity_bytes)}Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return Ok(self.fault.retirement_demands()?.release_bytes)}Ok(match self.operation.as_ref(){Some(ClipboardIoOperation::Write(text))=>text.capacity(),_=>0})}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return self.fault.retirement_demands()?.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"clipboard fault close depth overflow"))}Ok(usize::from(self.operation.is_some()))}
    fn terminal_is_empty(&self)->bool{self.closing&&self.operation.is_none()&&self.fault.terminal_is_empty()}

}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ClipboardIoJob{fn drop(&mut self){assert!(std::thread::panicking()||(self.operation.is_none()&&self.fault.terminal_is_empty()),"original clipboard owner reached Drop before funded closure");}}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub async fn clipboard_write_text(text: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(text);
    }
}

/** 📋️ The wasm mirror of `clipboard_read_text` above — `async` because the browser's Clipboard API
 * is Promise-based with no synchronous escape hatch; a caller drives this from a
 * owned browser-local task (see `report-w3-clipboard-dnd.md`), since the OS
 * clipboard permission prompt/read can't resolve within one synchronous per-frame call. */
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub async fn clipboard_read_text() -> Option<String> {
    let promise = web_sys::window()?.navigator().clipboard().read_text();
    semio_framework_async::browser::JsFuture::from(promise).await.ok()?.as_string()
}
//#endregion 🔖️ClipboardHost
// #endregion host
