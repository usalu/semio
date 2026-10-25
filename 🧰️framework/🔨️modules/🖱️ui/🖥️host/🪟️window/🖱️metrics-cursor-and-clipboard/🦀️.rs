//! 🪟️ Window metrics, cursor and clipboard: the window metrics, the redraw gating, the cursor icons and css, the clipboard host, the native clipboard job and the browser clipboard. See [`super`].

use super::*;

//#region 🔖️Host
//#region 📐️WindowMetrics
/// 📐️ Physical size plus the scale factor that relates it to logical pixels — the pair every
/// scale-factor-change or resize needs to hand a delegate in one shot, platform-neutral so it is
/// testable with no platform SDK in scope at all.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowMetrics {
    pub physical: PhysicalSize,
    pub scale_factor: f32,
}

impl WindowMetrics {
    /// 📏️ The logical (DPI-independent) size a layout pass actually consumes. `(0.0, 0.0)` for a
    /// degenerate `scale_factor` rather than dividing by zero — mirrors [`PhysicalSize::is_zero`]'s
    /// own "park, don't error" convention.
    pub fn logical_size(&self) -> (f32, f32) {
        if self.scale_factor <= 0.0 {
            return (0.0, 0.0);
        }
        (self.physical.width as f32 / self.scale_factor, self.physical.height as f32 / self.scale_factor)
    }
}
//#endregion 📐️WindowMetrics

//#region 🎯️Redraw gating
/// 🎯️ Whether a wake should turn into exactly one redraw — a thin, platform-neutral name for
/// [`FrameScheduler::should_render`] so both [`NativeHost::about_to_wait`] and the browser
/// [`CanvasHost`]'s frame callback funnel through one obviously-shared decision
/// point. Returning `None` for a clean window is the entire defect this crate exists to fix — see
/// `🦀️schedule.rs`'s own docstring.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn should_request_redraw(scheduler: &mut FrameScheduler, now: f64) -> Option<InvalidationReason> {
    scheduler.should_render(now)
}
//#endregion 🎯️Redraw gating

//#region 🖱️Cursor
/// 🖱️ [`ui_render::CursorRequest`] only distinguishes the five cursors a [`ui_render::DispatchOutcome`]
/// can actually request — the elaborate theme-aware SVG cursor taxonomy `wgpu-old`'s own `🦀️cursor.rs`
/// drew from a product-specific `HitKind` enum belongs one layer up (whatever product crate builds a
/// `DispatchOutcome` in the first place), not in this platform-neutral host.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn cursor_icon_for(cursor: CursorRequest) -> winit::window::CursorIcon {
    use winit::window::CursorIcon as I;
    match cursor {
        CursorRequest::Default => I::Default,
        CursorRequest::Pointer => I::Pointer,
        CursorRequest::Text => I::Text,
        CursorRequest::Grab => I::Grab,
        CursorRequest::Grabbing => I::Grabbing,
    }
}

/// 🌐️ The CSS `cursor` keyword equivalent of [`cursor_icon_for`], for the browser canvas host.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn cursor_css_for(cursor: CursorRequest) -> &'static str {
    match cursor {
        CursorRequest::Default => "default",
        CursorRequest::Pointer => "pointer",
        CursorRequest::Text => "text",
        CursorRequest::Grab => "grab",
        CursorRequest::Grabbing => "grabbing",
    }
}

/// 👆️ Ported from `wgpu-old`'s `🦀️cursor.rs::apply_window_cursor`: a real OS cursor call is one of the
/// few things worth deduplicating by hand rather than trusting the platform, since a naive per-event
/// caller would otherwise re-issue it on every single pointer-move tick.
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn apply_window_cursor(window: &winit::window::Window, cursor: CursorRequest, last: &mut Option<CursorRequest>) {
    if *last == Some(cursor) {
        return;
    }
    *last = Some(cursor);
    window.set_cursor(cursor_icon_for(cursor));
}
//#endregion 🖱️Cursor

//#region 📋️Clipboard
/// 📎️ Platform-neutral clipboard surface. Browser implementations stage asynchronous owned
/// requests and expose only already-settled text through `read_text`.
pub trait ClipboardHost {
    fn write_text(&mut self, text: &str);
    fn read_text(&mut self) -> Option<String>;
}

#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
pub enum NativeClipboardOperation {
    Read,
    Write(String),
}

/// 📋️ Native clipboard operation whose only execution entry is `InteractiveJob::step` on a
/// caller-supplied process `WorkerPool` I/O lane.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeClipboardJob {
    #[doc(hidden)] pub operation: std::mem::ManuallyDrop<Option<NativeClipboardOperation>>,
    closing: bool,
    completed: bool,
    cancelled: bool,
    fault: semio_framework_job_payload::RetainedPayloadBuilder,
    fault_cursor: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeClipboardJob {
    pub fn read() -> Self {
        Self { operation: std::mem::ManuallyDrop::new(Some(NativeClipboardOperation::Read)), closing: false, completed:false, cancelled:false, fault:semio_framework_job_payload::RetainedPayloadBuilder::new(semio_framework_job_payload::JobPayloadStream::Fault), fault_cursor:0 }
    }

    pub fn write(text: String) -> Self {
        Self { operation: std::mem::ManuallyDrop::new(Some(NativeClipboardOperation::Write(text))), closing: false, completed:false, cancelled:false, fault:semio_framework_job_payload::RetainedPayloadBuilder::new(semio_framework_job_payload::JobPayloadStream::Fault), fault_cursor:0 }
    }

    pub fn read_candidate<'a>(outcome: &semio_framework_job::JobOutcomeView<'a>) -> Option<&'a str> {
        let semio_framework_job::JobOutcomeView::Complete {output:Some(output),..}=outcome else{return None};
        let (&present,bytes)=output.page(0)?.split_first()?;
        (present==1).then(||std::str::from_utf8(bytes).ok()).flatten()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_job::InteractiveJob for NativeClipboardJob {
    fn step<'a>(&'a mut self, cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::JobOutcomeBorrow;
        let grant=cx.retained_grant();
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        if self.cancelled||cx.is_cancelled(){
            let outcome=JobOutcomeBorrow::admit_cancelled(cx)?;
            if outcome.is_some(){self.cancelled=true}
            return Ok(outcome)
        }
        if self.completed{return JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)}
        if cx.should_yield(){return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        cx.set_stage("ClipboardIo");
        if let Some(NativeClipboardOperation::Write(text))=self.operation.as_ref(){
            let mut clipboard=arboard::Clipboard::new().map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native clipboard write surface unavailable"))?;
            clipboard.set_text(text.as_str()).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native clipboard write failed"))?;
            self.completed=true;
            cx.consume_fuel(1);
            return JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)
        }
        let diagnostic=b"native clipboard read requires original bounded platform recipient";
        if !self.fault.advance_initialization(cx)?{return Ok(None)}
        if !self.fault.append_original(cx,diagnostic,&mut self.fault_cursor)?{return Ok(None)}
        if !self.fault.seal(cx)?{return Ok(None)}
        JobOutcomeBorrow::admit_fault(cx,self.fault.published().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original clipboard diagnostic was not sealed"))?).map_err(Into::into)
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

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::{InteractiveJobCloseStep as Close,RetainedCloneProgress as Progress,RetainedCloneStep};
        if !self.closing{return Ok(Close::Blocked)}
        if !self.fault.terminal_is_empty(){
            let mut demand=self.fault.retirement_demands()?;
            demand.depth=demand.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"clipboard fault close depth overflow"))?;
            return semio_framework_job::close_original_retained_step(cx,demand,|grant|self.fault.close_step_granted(semio_framework_job::RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}));
        }
        if self.operation.is_none(){return Ok(Close::Complete{progress:Progress::default()})}
        let bytes=match self.operation.as_ref(){Some(NativeClipboardOperation::Write(text))=>text.capacity(),_=>0};
        let step=semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{release_bytes:bytes,depth:1,..Default::default()},|_|{
            *self.operation=None;
            Ok(RetainedCloneStep::Complete(Progress{copied_items:1,released_bytes:bytes,..Progress::default()}))
        })?;
        Ok(if self.operation.is_none(){Close::Complete{progress:step.progress()}}else{step})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return Ok(self.fault.retirement_demands()?.copy_bytes)}Ok(0)}
    fn next_close_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return Ok(self.fault.retirement_demands()?.capacity_bytes)}Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return Ok(self.fault.retirement_demands()?.release_bytes)}Ok(match self.operation.as_ref(){Some(NativeClipboardOperation::Write(text))=>text.capacity(),_=>0})}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{if !self.fault.terminal_is_empty(){return self.fault.retirement_demands()?.depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"clipboard fault close depth overflow"))}Ok(usize::from(self.operation.is_some()))}
    fn terminal_is_empty(&self)->bool{self.closing&&self.operation.is_none()&&self.fault.terminal_is_empty()}

}

/// 📫️ Owned asynchronous clipboard mailbox. It never retains a platform promise or callback.
#[derive(Default)]
pub struct BrowserClipboard {
    staged_write: Option<String>,
    settled_read: Option<String>,
}

impl ClipboardHost for BrowserClipboard {
    fn write_text(&mut self, text: &str) {
        self.staged_write = Some(text.to_owned());
    }

    fn read_text(&mut self) -> Option<String> {
        self.settled_read.take()
    }
}

impl BrowserClipboard {
    pub fn take_staged_write(&mut self) -> Option<String> {
        self.staged_write.take()
    }

    pub fn settle_read(&mut self, text: String) {
        self.settled_read = Some(text);
    }
}
//#endregion 📋️Clipboard

//#region 📁️FileDialog
/// 📁️ Platform-neutral request schema for a native file dialog.
#[cfg(not(target_arch = "wasm32"))]
impl Drop for NativeClipboardJob{fn drop(&mut self){assert!(std::thread::panicking()||(self.operation.is_none()&&self.fault.terminal_is_empty()),"original clipboard owner reached Drop before funded closure");}}
//#endregion 📁️FileDialog
//#endregion 🔖️Host
