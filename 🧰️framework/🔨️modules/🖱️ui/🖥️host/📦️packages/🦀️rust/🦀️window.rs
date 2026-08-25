//! @emoji 🪟️ Window/canvas hosting, the scheduler's wake transport, cursor application, clipboard,
//! and IME plumbing — the crate's other half of the U1 sync/async boundary that `📦️glue.rs` and
//! `🦀️event.rs` both point back to this file for.
//!
//! **The U1 boundary, made explicit.** Every `winit::application::ApplicationHandler` callback below
//! is a plain sync `fn` — `winit` 0.30's own trait leaves no other option, which is itself the honest
//! proof that frame-driving code never needs to suspend (ruling U1). The *outer* async exception lives
//! in exactly two places, both outside any `ApplicationHandler` callback:
//! 1. [`NativeRuntime::new`]/[`run_native`] are plain sync too (winit's `run_app` blocks the calling
//!    thread; there is nothing to `.await` in event-loop *construction* on native), but a caller is
//!    free to `.await` its own async setup (adapter/device selection) *before* calling either — the
//!    delegate it hands in is constructed however it likes.
//! 2. On wasm32, [`BrowserClipboard`] turns clipboard work into correlated owned requests. The host
//!    shim settles them later as replies/events; no renderer callback blocks or owns a browser value.
//!
//! **Never `ControlFlow::Poll`, never an unconditional `request_redraw`.** [`should_request_redraw`]
//! is the single decision point both [`NativeHost`] and the browser [`CanvasHost`] funnel through —
//! it is [`ui_render::FrameScheduler::should_render`] under a name that reads at the call site; a
//! clean window drains no dirty mask and neither host asks for a frame.

use crate::abi::{AbiBytes, AbiControl, AbiErrorCode, AbiEvent, AbiMessage, AbiOperation, AbiPage, AbiPort, AbiPortPoll, AbiReply, AbiReplyLedger, AbiRequest, AbiRequestId, AbiStatusCode, AbiWorkBudget};
use ui_render::{CursorRequest, DispatchEvent, FrameScheduler, ImeDirective, InvalidationReason, PhysicalSize};

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
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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

/// 🖱️ Ported from `wgpu-old`'s `🦀️cursor.rs::apply_window_cursor`: a real OS cursor call is one of the
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

/// 📋️ Platform-neutral clipboard surface. Browser implementations stage asynchronous owned
/// requests and expose only already-settled text through `read_text`.
pub trait ClipboardHost {
    fn write_text(&mut self, text: &str);
    fn read_text(&mut self) -> Option<String>;
}

#[cfg(not(target_arch = "wasm32"))]
enum NativeClipboardOperation {
    Read,
    Write(String),
}

/// 📋️ Native clipboard operation whose only execution entry is `InteractiveJob::step` on a
/// caller-supplied process `WorkerPool` I/O lane.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeClipboardJob {
    operation: Option<NativeClipboardOperation>,
    closing: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeClipboardJob {
    pub fn read() -> Self {
        Self { operation: Some(NativeClipboardOperation::Read), closing: false }
    }

    pub fn write(text: String) -> Self {
        Self { operation: Some(NativeClipboardOperation::Write(text)), closing: false }
    }

    pub fn read_candidate(outcome: &semio_framework_job::StepOutcome) -> Option<String> {
        let semio_framework_job::StepOutcome::Complete(candidate) = outcome else { return None };
        let (&present, bytes) = candidate.output.page(0)?.split_first()?;
        (present == 1).then(|| String::from_utf8(bytes.to_vec()).ok()).flatten()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl semio_framework_job::InteractiveJob for NativeClipboardJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if cx.should_yield() {
            return semio_framework_job::StepOutcome::Yield;
        }
        cx.set_stage("NativeClipboardIo");
        use arboard as system_clipboard;
        let output = match self.operation.take() {
            Some(NativeClipboardOperation::Read) => {
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
            Some(NativeClipboardOperation::Write(text)) => {
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

/// 📋️ Owned asynchronous clipboard mailbox. It never retains a platform promise or callback.
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativeFileDialogRequest {
    Open { extensions: Vec<String>, multiple: bool },
    Save { filename: String, extensions: Vec<String> },
    Folder,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeFileDialogRequest {
    pub fn open(extensions: impl IntoIterator<Item = impl Into<String>>, multiple: bool) -> Self {
        Self::Open { extensions: normalize_dialog_extensions(extensions), multiple }
    }

    pub fn save(filename: impl Into<String>, extensions: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Save { filename: filename.into(), extensions: normalize_dialog_extensions(extensions) }
    }

    pub fn folder() -> Self {
        Self::Folder
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn normalize_dialog_extensions(extensions: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
    let mut normalized: Vec<String> = extensions.into_iter().map(Into::into).map(|extension: String| extension.trim().trim_start_matches('.').to_ascii_lowercase()).filter(|extension| !extension.is_empty()).collect();
    normalized.sort();
    normalized.dedup();
    normalized
}

/// 📂️ Runs the native picker behind an owned request/result boundary. The returned future is
/// polled by the renderer's I/O-lane task seam; no renderer callback waits for user interaction.
#[cfg(not(target_arch = "wasm32"))]
pub async fn select_native_paths(request: NativeFileDialogRequest) -> Vec<std::path::PathBuf> {
    use rfd::AsyncFileDialog;

    match request {
        NativeFileDialogRequest::Open { extensions, multiple } => {
            let dialog = if extensions.is_empty() { AsyncFileDialog::new() } else { AsyncFileDialog::new().add_filter("import", &extensions) };
            if multiple {
                dialog.pick_files().await.unwrap_or_default().into_iter().map(|file| file.path().to_path_buf()).collect()
            } else {
                dialog.pick_file().await.into_iter().map(|file| file.path().to_path_buf()).collect()
            }
        }
        NativeFileDialogRequest::Save { filename, extensions } => {
            let dialog = AsyncFileDialog::new().set_file_name(filename);
            let dialog = if extensions.is_empty() { dialog } else { dialog.add_filter("export", &extensions) };
            dialog.save_file().await.into_iter().map(|file| file.path().to_path_buf()).collect()
        }
        NativeFileDialogRequest::Folder => AsyncFileDialog::new().pick_folder().await.into_iter().map(|folder| folder.path().to_path_buf()).collect(),
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod native_file_dialog_tests {
    use super::*;

    #[test]
    fn request_schema_normalizes_extensions_deterministically() {
        assert_eq!(NativeFileDialogRequest::open([".JSON", " json ", "png", ""], true), NativeFileDialogRequest::Open { extensions: vec!["json".into(), "png".into()], multiple: true });
    }

    #[test]
    fn save_and_folder_requests_preserve_owned_values() {
        assert_eq!(NativeFileDialogRequest::save("studio.json", [".JSON"]), NativeFileDialogRequest::Save { filename: "studio.json".into(), extensions: vec!["json".into()] });
        assert_eq!(NativeFileDialogRequest::folder(), NativeFileDialogRequest::Folder);
    }

    #[test]
    fn selection_future_can_move_to_the_io_lane() {
        fn assert_send<T: Send>(_: T) {}
        assert_send(select_native_paths(NativeFileDialogRequest::folder()));
    }
}

//#endregion 📁️FileDialog

//#region 🈶️Ime

/// 🈶️ Applies a [`ImeDirective`] to a real window — `Enable` positions the platform IME candidate
/// window at the focused element's caret bounds (window-absolute, per `dispatch.rs`'s own docstring);
/// `Disable` turns IME off so plain `KeyboardInput` events resume (see `winit::Window::set_ime_allowed`'s
/// own docs on that trade-off).
#[cfg(not(target_arch = "wasm32"))]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn apply_ime_directive(window: &winit::window::Window, directive: ImeDirective) {
    match directive {
        ImeDirective::Enable { cursor_bounds } => {
            window.set_ime_allowed(true);
            window.set_ime_cursor_area(winit::dpi::PhysicalPosition::new(cursor_bounds.x as f64, cursor_bounds.y as f64), winit::dpi::PhysicalSize::new(cursor_bounds.w.max(1.0) as u32, cursor_bounds.h.max(1.0) as u32));
        }
        ImeDirective::Disable => window.set_ime_allowed(false),
    }
}

//#endregion 🈶️Ime

//#region 🧩️WindowDelegate

/// 📤️ What a redraw produced — the cursor/IME half of a [`ui_render::DispatchOutcome`] a delegate
/// accumulated while building the frame, handed back so [`NativeHost`]/[`CanvasHost`] can apply both
/// through the one dedup path ([`apply_window_cursor`]/[`apply_canvas_cursor`],
/// [`apply_ime_directive`]) without the delegate needing to know either exists.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedrawOutcome {
    pub cursor: CursorRequest,
    pub ime: Option<ImeDirective>,
}

/// 🧩️ What a product/runtime crate implements to receive this host's normalized events and
/// scheduler-gated redraw ticks — the one seam that keeps `🦀️window.rs` itself ignorant of
/// `ui_render::Dispatcher`/`FrameEngine`, which belong to later packets (`runtime-present`, `os-host`).
/// Nothing in this trait's signature ever names a platform SDK type.
pub trait WindowDelegate {
    /// ⏱️ The scheduler this delegate's own frame-building code invalidates — [`NativeHost`]/
    /// [`CanvasHost`] only ever read it through [`should_request_redraw`]/`next_deadline`, never
    /// mutate it directly.
    fn scheduler_mut(&mut self) -> &mut FrameScheduler;

    /// 📥️ One normalized input event, already free of any platform type.
    fn handle_event(&mut self, event: DispatchEvent);

    /// 📐️ A resize or scale-factor change.
    fn handle_metrics(&mut self, metrics: WindowMetrics);

    /// 🖼️ Build and present one frame for the accumulated `reason`, returning the cursor/IME state
    /// that frame settled on.
    fn redraw(&mut self, reason: InvalidationReason) -> RedrawOutcome;

    /// 🚪️ Whether the host may actually close the window — `true` (close immediately) unless a
    /// delegate overrides this to, say, confirm unsaved changes first.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn close_requested(&mut self) -> bool {
        true
    }
}

//#endregion 🧩️WindowDelegate

//#region 🪟️Native

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;
    use crate::event::{self, PointerRegistry};

    //#region 🕰️MonotonicClock

    /// 🕰️ Wall-clock seconds since this clock was constructed, monotonic per `std::time::Instant`'s
    /// own guarantee — the one clock [`should_request_redraw`] and `winit::event_loop::ControlFlow`'s
    /// `WaitUntil` share, so a deadline requested in scheduler-seconds converts back to a real
    /// `Instant` without drift between the two.
    #[derive(Clone)]
    pub struct MonotonicClock {
        origin: std::time::Instant,
    }

    impl MonotonicClock {
        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn new() -> Self {
            Self { origin: std::time::Instant::now() }
        }

        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn now_seconds(&self) -> f64 {
            self.origin.elapsed().as_secs_f64()
        }

        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn instant_at(&self, seconds: f64) -> std::time::Instant {
            self.origin + std::time::Duration::from_secs_f64(seconds.max(0.0))
        }
    }

    impl Default for MonotonicClock {
        fn default() -> Self {
            Self::new()
        }
    }

    //#endregion 🕰️MonotonicClock

    //#region 🚦️ControlFlow

    /// 🚦️ `WaitUntil(next deadline)` when one is pending, plain `Wait` otherwise — **never `Poll`**,
    /// the exact defect ruling this file's own docstring calls out. A caller sets this after every
    /// `ApplicationHandler` callback, mirroring winit's own recommended pattern.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn control_flow_for(scheduler: &FrameScheduler, clock: &MonotonicClock) -> winit::event_loop::ControlFlow {
        match scheduler.next_deadline() {
            Some(deadline) => winit::event_loop::ControlFlow::WaitUntil(clock.instant_at(deadline.due)),
            None => winit::event_loop::ControlFlow::Wait,
        }
    }

    //#endregion 🚦️ControlFlow

    //#region 🪟️NativeHost

    /// 📨️ The user-event type [`NativeRuntime`]'s event loop is parameterized over — carries no
    /// payload of its own; receiving one at all is the signal ([`WakeProxy::wake`] sent it).
    pub enum WakeMessage {
        Deadline,
    }

    /// 🪟️ The `winit::application::ApplicationHandler` that owns a single native window: creates it
    /// on `resumed`, normalizes every `WindowEvent` through `🦀️event.rs` before handing it to `D`,
    /// and gates every redraw through [`should_request_redraw`].
    pub struct NativeHost<D: WindowDelegate> {
        window: Option<winit::window::Window>,
        pointers: PointerRegistry,
        modifiers: ui_render::EventModifiers,
        last_pointer_pos: (f32, f32),
        last_cursor: Option<CursorRequest>,
        clock: MonotonicClock,
        pending_reason: Option<InvalidationReason>,
        delegate: D,
        /// 🎫️ Minted once, here — `NativeHost` is constructed on, and only ever driven from, the
        /// thread `winit::event_loop::EventLoop::run_app` blocks on (this crate's own U1 boundary).
        /// Not yet read anywhere (`WindowDelegate`'s methods don't take one — see
        /// `crate::enqueue::UiThreadToken`'s own doc for why that is a deliberate, scoped choice, not
        /// an oversight); kept as a field so the capability exists at the point of construction, ready
        /// for a future `WindowDelegate` signature that threads it through.
        _ui_token: crate::enqueue::UiThreadToken,
    }

    impl<D: WindowDelegate> NativeHost<D> {
        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn new(delegate: D) -> Self {
            Self {
                window: None,
                pointers: PointerRegistry::new(),
                modifiers: ui_render::EventModifiers::default(),
                last_pointer_pos: (0.0, 0.0),
                last_cursor: None,
                clock: MonotonicClock::new(),
                pending_reason: None,
                delegate,
                _ui_token: crate::enqueue::UiThreadToken::mint(),
            }
        }

        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        fn normalize(&mut self, event: &winit::event::WindowEvent) -> Option<DispatchEvent> {
            use winit::event::{ElementState, TouchPhase, WindowEvent as E};
            match event {
                E::CursorMoved { device_id, position } => {
                    self.last_pointer_pos = (position.x as f32, position.y as f32);
                    let pointer = event::pointer_info_for_mouse(&mut self.pointers, *device_id);
                    Some(DispatchEvent::PointerMove { pointer, x: self.last_pointer_pos.0, y: self.last_pointer_pos.1 })
                }
                E::MouseInput { device_id, state, button } => {
                    let pointer = event::pointer_info_for_mouse(&mut self.pointers, *device_id);
                    let button = event::pointer_button_from_winit(*button)?;
                    let (x, y) = self.last_pointer_pos;
                    Some(match state {
                        ElementState::Pressed => DispatchEvent::PointerDown { pointer, x, y, button },
                        ElementState::Released => DispatchEvent::PointerUp { pointer, x, y, button },
                    })
                }
                E::MouseWheel { delta, .. } => {
                    let (delta_x, delta_y) = event::normalize_wheel_delta_native(*delta);
                    let (x, y) = self.last_pointer_pos;
                    Some(DispatchEvent::Scroll { x, y, delta_x, delta_y })
                }
                E::Touch(touch) => {
                    let pointer = event::pointer_info_for_touch(&mut self.pointers, touch);
                    let x = touch.location.x as f32;
                    let y = touch.location.y as f32;
                    Some(match touch.phase {
                        TouchPhase::Started => DispatchEvent::PointerDown { pointer, x, y, button: ui_render::PointerButton::Primary },
                        TouchPhase::Moved => DispatchEvent::PointerMove { pointer, x, y },
                        TouchPhase::Ended | TouchPhase::Cancelled => DispatchEvent::PointerUp { pointer, x, y, button: ui_render::PointerButton::Primary },
                    })
                }
                E::KeyboardInput { event, .. } => {
                    let logical = event::logical_key_to_dispatch_string(&event.logical_key);
                    Some(event::key_dispatch_event(logical, self.modifiers, event.state == ElementState::Pressed))
                }
                E::Ime(ime) => Some(DispatchEvent::Ime(event::ime_event_from_winit(ime.clone()))),
                _ => None,
            }
        }
    }

    impl<D: WindowDelegate> winit::application::ApplicationHandler<WakeMessage> for NativeHost<D> {
        // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait; see this file's docstring.
        fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
            if self.window.is_none() {
                let attributes = winit::window::Window::default_attributes().with_title("semio");
                if let Ok(window) = event_loop.create_window(attributes) {
                    window.set_ime_allowed(false);
                    self.window = Some(window);
                }
            }
            event_loop.set_control_flow(control_flow_for(self.delegate.scheduler_mut(), &self.clock));
        }

        // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait; see this file's docstring.
        fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, window_id: winit::window::WindowId, event: winit::event::WindowEvent) {
            if self.window.as_ref().map(|window| window.id()) != Some(window_id) {
                return;
            }

            match &event {
                winit::event::WindowEvent::CloseRequested => {
                    if self.delegate.close_requested() {
                        event_loop.exit();
                    }
                    return;
                }
                winit::event::WindowEvent::Resized(size) => {
                    let scale_factor = self.window.as_ref().map(|window| window.scale_factor()).unwrap_or(1.0);
                    self.delegate.handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor: scale_factor as f32 });
                }
                winit::event::WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    if let Some(size) = self.window.as_ref().map(|window| window.inner_size()) {
                        self.delegate.handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor: *scale_factor as f32 });
                    }
                }
                winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                    self.modifiers = event::modifiers_from_winit(modifiers.state());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    if let Some(reason) = self.pending_reason.take() {
                        let outcome = self.delegate.redraw(reason);
                        if let Some(window) = self.window.as_ref() {
                            apply_window_cursor(window, outcome.cursor, &mut self.last_cursor);
                            if let Some(directive) = outcome.ime {
                                apply_ime_directive(window, directive);
                            }
                        }
                    }
                    event_loop.set_control_flow(control_flow_for(self.delegate.scheduler_mut(), &self.clock));
                    return;
                }
                _ => {}
            }

            if let Some(dispatch_event) = self.normalize(&event) {
                self.delegate.handle_event(dispatch_event);
            }
            event_loop.set_control_flow(control_flow_for(self.delegate.scheduler_mut(), &self.clock));
        }

        // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait; see this file's docstring.
        fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
            let now = self.clock.now_seconds();
            if let Some(reason) = should_request_redraw(self.delegate.scheduler_mut(), now) {
                self.pending_reason = Some(reason);
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            event_loop.set_control_flow(control_flow_for(self.delegate.scheduler_mut(), &self.clock));
        }

        // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait; see this file's docstring.
        fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _event: WakeMessage) {
            event_loop.set_control_flow(control_flow_for(self.delegate.scheduler_mut(), &self.clock));
        }
    }

    //#endregion 🪟️NativeHost

    //#region 🚀️NativeRuntime

    /// 📨️ A clonable, `Send`-capable handle a background thread uses to ask the event loop for a
    /// frame — the "wake transport" the packet brief asks for, wrapping `winit::event_loop::EventLoopProxy`
    /// so nothing above this file needs to name it.
    #[derive(Clone)]
    pub struct WakeProxy(winit::event_loop::EventLoopProxy<WakeMessage>);

    impl WakeProxy {
        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn wake(&self) {
            let _ = self.0.send_event(WakeMessage::Deadline);
        }
    }

    /// 🚀️ Owns the event loop before it starts running — split from [`run_native`] so a caller can
    /// obtain a [`WakeProxy`] (which needs a live `EventLoop` to mint) before handing control to
    /// `winit::event_loop::EventLoop::run_app`, which blocks the calling thread until the window
    /// closes.
    pub struct NativeRuntime {
        event_loop: winit::event_loop::EventLoop<WakeMessage>,
    }

    impl NativeRuntime {
        pub fn new() -> Result<Self, winit::error::EventLoopError> {
            let mut builder = winit::event_loop::EventLoop::<WakeMessage>::with_user_event();
            Ok(Self { event_loop: builder.build()? })
        }

        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn create_wake_proxy(&self) -> WakeProxy {
            WakeProxy(self.event_loop.create_proxy())
        }

        /// 🚀️ Blocks the calling thread for the lifetime of the window (winit's own contract for
        /// `run_app`) — call this last, after any async device/adapter setup the delegate needed has
        /// already completed.
        pub fn run<D: WindowDelegate>(self, delegate: D) -> Result<(), winit::error::EventLoopError> {
            let mut host = NativeHost::new(delegate);
            self.event_loop.run_app(&mut host)
        }
    }

    /// 🚀️ Convenience wrapper over [`NativeRuntime`] for a caller with no need for a [`WakeProxy`].
    pub fn run_native<D: WindowDelegate>(delegate: D) -> Result<(), winit::error::EventLoopError> {
        NativeRuntime::new()?.run(delegate)
    }

    //#endregion 🚀️NativeRuntime
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

//#endregion 🪟️Native

//#region 🌐️Browser

mod browser {
    use super::*;
    use crate::event::{decode_browser_host_event, BrowserHostEvent, CanvasId, ListenerId};

    pub const BROWSER_HOST_MAX_EVENT_BODY_BYTES: usize = 1_024;
    pub const BROWSER_HOST_EVENT_ENVELOPE_BYTES: usize = 27;
    pub const BROWSER_HOST_MAX_ENCODED_EVENT_BYTES: usize = BROWSER_HOST_MAX_EVENT_BODY_BYTES + BROWSER_HOST_EVENT_ENVELOPE_BYTES;
    pub const BROWSER_HOST_PAGE_ENVELOPE_BYTES: usize = 18;
    pub const BROWSER_HOST_MAX_PAGE_BODY_BYTES: usize = BROWSER_HOST_MAX_ENCODED_EVENT_BYTES - BROWSER_HOST_PAGE_ENVELOPE_BYTES;
    pub const BROWSER_HOST_INITIAL_POLL_BYTES: usize = BROWSER_HOST_MAX_EVENT_BODY_BYTES;
    pub const BROWSER_HOST_OPERATION_ATTACH: u16 = 1_793;
    pub const BROWSER_HOST_OPERATION_FRAME: u16 = 1_794;
    pub const BROWSER_HOST_OPERATION_CURSOR: u16 = 1_795;
    pub const BROWSER_HOST_OPERATION_CLIPBOARD_READ: u16 = 1_796;
    pub const BROWSER_HOST_OPERATION_CLIPBOARD_WRITE: u16 = 1_797;
    pub const BROWSER_HOST_OPERATION_DETACH: u16 = 1_798;

    /// 🔌️ A1 port specialization implemented by an owned host shim.
    pub trait BrowserHostPort: AbiPort {}
    impl<T: AbiPort> BrowserHostPort for T {}

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BrowserHostUnavailable {
        Window,
        Document,
        Canvas,
        Clipboard,
        Listener,
        Callback,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum BrowserHostError {
        Abi(AbiErrorCode),
        Unavailable(BrowserHostUnavailable),
        MalformedHostReply,
        Closed,
        Busy,
    }

    impl From<AbiErrorCode> for BrowserHostError {
        fn from(value: AbiErrorCode) -> Self {
            Self::Abi(value)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ClipboardOperation {
        Read,
        Write,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ClipboardRequest {
        pub request_id: AbiRequestId,
        pub generation: u32,
        pub operation: ClipboardOperation,
        pub text: Vec<u8>,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct FrameSchedulerEnvelope {
        pub canvas: CanvasId,
        pub generation: u32,
        pub timestamp_ms: f64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BrowserHostProgress {
        pub completed_units: u32,
        pub total_units: u32,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum BrowserHostStep {
        Progress(BrowserHostProgress),
        AwaitingHost,
        Event,
        Frame(FrameSchedulerEnvelope),
        Clipboard { request_id: AbiRequestId, text: Option<String> },
        Unavailable(BrowserHostUnavailable),
        Closed,
    }

    struct PendingEvent {
        event: AbiEvent,
        inspected_bytes: usize,
    }

    struct PendingReply {
        reply: AbiReply,
        inspected_bytes: usize,
    }

    struct PendingPage {
        page: AbiPage,
        inspected_bytes: usize,
    }

    /// 🧵️ Browser delegate split preserving worker frame preparation.
    pub trait BrowserWindowDelegate: WindowDelegate {
        fn enqueue_browser_frame(&mut self, reason: InvalidationReason);
        fn present_browser_frame(&mut self) -> RedrawOutcome;
    }

    /// 🌐️ Browser state machine over an owned A1 port.
    pub struct CanvasHost<P: BrowserHostPort, D: BrowserWindowDelegate> {
        canvas: CanvasId,
        generation: u32,
        port: P,
        delegate: D,
        replies: AbiReplyLedger,
        listener: Option<ListenerId>,
        outbound: Option<AbiMessage>,
        outbound_inspected_bytes: usize,
        pending_event: Option<PendingEvent>,
        pending_reply: Option<PendingReply>,
        pending_page: Option<PendingPage>,
        latest_metrics: Option<(AbiEvent, WindowMetrics)>,
        latest_pointer: Option<(AbiEvent, DispatchEvent)>,
        pending_dispatch: Option<(AbiEvent, DispatchEvent)>,
        pending_frame: Option<(AbiEvent, FrameSchedulerEnvelope)>,
        clipboard: Option<ClipboardRequest>,
        clipboard_mailbox: BrowserClipboard,
        last_cursor: Option<CursorRequest>,
        pending_cursor: Option<CursorRequest>,
        frame_pending: bool,
        next_request_id: u64,
        closing: bool,
        detach_sent: bool,
        detach_request: Option<AbiRequestId>,
        detach_acknowledged: bool,
        _ui_token: crate::enqueue::UiThreadToken,
    }

    impl<P: BrowserHostPort, D: BrowserWindowDelegate> CanvasHost<P, D> {
        pub fn new(canvas: CanvasId, port: P, delegate: D) -> Result<Self, BrowserHostError> {
            let request_id = AbiRequestId(canvas.get() as u64);
            let attach = request(BROWSER_HOST_OPERATION_ATTACH, request_id, 1, canvas_payload(canvas))?;
            let mut replies = AbiReplyLedger::new();
            replies.admit(request_id, 1)?;
            Ok(Self {
                canvas,
                generation: 1,
                port,
                delegate,
                replies,
                listener: None,
                outbound: Some(AbiMessage::Request(attach)),
                outbound_inspected_bytes: 0,
                pending_event: None,
                pending_reply: None,
                pending_page: None,
                latest_metrics: None,
                latest_pointer: None,
                pending_dispatch: None,
                pending_frame: None,
                clipboard: None,
                clipboard_mailbox: BrowserClipboard::default(),
                last_cursor: None,
                pending_cursor: None,
                frame_pending: false,
                next_request_id: canvas.get() as u64 + 1,
                closing: false,
                detach_sent: false,
                detach_request: None,
                detach_acknowledged: false,
                _ui_token: crate::enqueue::UiThreadToken::mint(),
            })
        }

        pub fn request_wake(&mut self) -> Result<bool, BrowserHostError> {
            if self.closing {
                return Err(BrowserHostError::Closed);
            }
            if self.frame_pending {
                return Ok(false);
            }
            let request_id = self.next_request();
            let frame = request(BROWSER_HOST_OPERATION_FRAME, request_id, self.generation, canvas_payload(self.canvas))?;
            self.stage_request(frame)?;
            self.frame_pending = true;
            Ok(true)
        }

        pub fn request_clipboard_read(&mut self) -> Result<AbiRequestId, BrowserHostError> {
            self.stage_clipboard(ClipboardOperation::Read, Vec::new())
        }

        pub fn request_clipboard_write(&mut self, text: String) -> Result<AbiRequestId, BrowserHostError> {
            self.clipboard_mailbox.write_text(&text);
            self.stage_clipboard(ClipboardOperation::Write, text.into_bytes())
        }

        pub fn cancel_clipboard(&mut self) -> Result<bool, BrowserHostError> {
            let Some(request) = self.clipboard.take() else { return Ok(false) };
            if let Some(message) = self.outbound.take() {
                if !matches!(&message, AbiMessage::Request(value) if value.request_id == request.request_id) {
                    self.outbound = Some(message);
                    self.clipboard = Some(request);
                    return Err(BrowserHostError::Busy);
                }
                self.replies.lose(request.request_id, request.generation)?;
                return Ok(true);
            }
            self.replies.lose(request.request_id, request.generation)?;
            self.outbound = Some(AbiMessage::Control(AbiControl::Cancel { request_id: request.request_id, generation: request.generation }));
            self.outbound_inspected_bytes = 0;
            Ok(true)
        }

        pub fn begin_close(&mut self) {
            self.closing = true;
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.closing
                && self.detach_sent
                && self.detach_acknowledged
                && self.listener.is_none()
                && self.outbound.is_none()
                && self.pending_event.is_none()
                && self.pending_reply.is_none()
                && self.pending_page.is_none()
                && self.latest_metrics.is_none()
                && self.latest_pointer.is_none()
                && self.pending_dispatch.is_none()
                && self.pending_frame.is_none()
                && self.clipboard.is_none()
                && self.pending_cursor.is_none()
        }

        pub fn delegate(&self) -> &D {
            &self.delegate
        }

        pub fn delegate_mut(&mut self) -> &mut D {
            &mut self.delegate
        }

        pub fn step(&mut self, budget: AbiWorkBudget) -> Result<BrowserHostStep, BrowserHostError> {
            validate_budget(budget)?;
            if self.closing {
                return self.close_step(budget);
            }
            if self.outbound.is_some() {
                let total = message_body_bytes(self.outbound.as_ref().expect("checked")).max(1);
                self.outbound_inspected_bytes += 1;
                if self.outbound_inspected_bytes < total {
                    return Ok(progress(self.outbound_inspected_bytes as u32, total as u32));
                }
                let message = self.outbound.take().expect("checked");
                self.port.try_send(message, one_credit(budget)).map_err(|rejection| {
                    self.outbound = Some(rejection.message);
                    self.outbound_inspected_bytes = total.saturating_sub(1);
                    BrowserHostError::Abi(rejection.code)
                })?;
                self.outbound_inspected_bytes = 0;
                return Ok(progress(1, 1));
            }
            if let Some(cursor) = self.pending_cursor.take() {
                self.stage_cursor(cursor)?;
                return Ok(progress(1, 1));
            }
            if let Some(pending) = self.pending_reply.as_mut() {
                pending.inspected_bytes += 1;
                let total = pending.reply.bytes.len().max(1);
                if pending.inspected_bytes < total {
                    return Ok(progress(pending.inspected_bytes as u32, total as u32));
                }
                let pending = self.pending_reply.take().expect("checked");
                return self.accept_reply(pending.reply);
            }
            if let Some(pending) = self.pending_page.as_mut() {
                pending.inspected_bytes += 1;
                let total = pending.page.bytes.len().max(1);
                if pending.inspected_bytes < total {
                    return Ok(progress(pending.inspected_bytes as u32, total as u32));
                }
                let page = self.pending_page.take().expect("checked").page;
                self.outbound = Some(AbiMessage::Control(AbiControl::Acknowledge { handle: page.handle, index: page.index }));
                self.outbound_inspected_bytes = 0;
                return Ok(progress(total as u32, total as u32));
            }
            if let Some((event, dispatch)) = self.pending_dispatch.take() {
                self.delegate.handle_event(dispatch);
                self.acknowledge_event(event)?;
                return Ok(BrowserHostStep::Event);
            }
            if self.pending_frame.is_some() && self.latest_metrics.is_some() {
                let (event, metrics) = self.latest_metrics.take().expect("checked");
                self.delegate.handle_metrics(metrics);
                self.acknowledge_event(event)?;
                return Ok(BrowserHostStep::Event);
            }
            if self.pending_frame.is_some() && self.latest_pointer.is_some() {
                let (event, dispatch) = self.latest_pointer.take().expect("checked");
                self.delegate.handle_event(dispatch);
                self.acknowledge_event(event)?;
                return Ok(BrowserHostStep::Event);
            }
            if let Some((event, frame)) = self.pending_frame.take() {
                self.frame_pending = false;
                if let Some(reason) = should_request_redraw(self.delegate.scheduler_mut(), frame.timestamp_ms / 1_000.0) {
                    self.delegate.enqueue_browser_frame(reason);
                    let outcome = self.delegate.present_browser_frame();
                    if self.last_cursor != Some(outcome.cursor) {
                        self.pending_cursor = Some(outcome.cursor);
                    }
                }
                self.acknowledge_event(event)?;
                return Ok(BrowserHostStep::Frame(frame));
            }
            if let Some(pending) = self.pending_event.as_mut() {
                pending.inspected_bytes += 1;
                let total = pending.event.bytes.len().max(1);
                if pending.inspected_bytes < total {
                    return Ok(progress(pending.inspected_bytes as u32, total as u32));
                }
                let pending = self.pending_event.take().expect("checked");
                self.classify_event(pending.event)?;
                return Ok(progress(total as u32, total as u32));
            }
            match self.port.poll(one_credit(budget))? {
                AbiPortPoll::Pending => {
                    if let Some((event, metrics)) = self.latest_metrics.take() {
                        self.delegate.handle_metrics(metrics);
                        self.acknowledge_event(event)?;
                        Ok(BrowserHostStep::Event)
                    } else if let Some((event, dispatch)) = self.latest_pointer.take() {
                        self.delegate.handle_event(dispatch);
                        self.acknowledge_event(event)?;
                        Ok(BrowserHostStep::Event)
                    } else {
                        Ok(BrowserHostStep::AwaitingHost)
                    }
                }
                AbiPortPoll::Closed => {
                    self.begin_close();
                    Ok(BrowserHostStep::Closed)
                }
                AbiPortPoll::Message(message) => self.accept_message(message),
            }
        }

        fn accept_message(&mut self, message: AbiMessage) -> Result<BrowserHostStep, BrowserHostError> {
            match message {
                AbiMessage::Event(event) => {
                    if event.generation != self.generation || event.bytes.len() > BROWSER_HOST_MAX_EVENT_BODY_BYTES {
                        let code = if event.generation < self.generation { AbiErrorCode::AbaHandle } else { AbiErrorCode::StaleGeneration };
                        return Err(BrowserHostError::Abi(code));
                    }
                    self.pending_event = Some(PendingEvent { event, inspected_bytes: 0 });
                    Ok(progress(1, 1))
                }
                AbiMessage::Reply(reply) => {
                    self.pending_reply = Some(PendingReply { reply, inspected_bytes: 0 });
                    Ok(progress(1, 1))
                }
                AbiMessage::Page(page) => {
                    if page.bytes.len() > BROWSER_HOST_MAX_PAGE_BODY_BYTES {
                        return Err(BrowserHostError::Abi(AbiErrorCode::LimitExceeded));
                    }
                    self.pending_page = Some(PendingPage { page, inspected_bytes: 0 });
                    Ok(progress(1, 1))
                }
                AbiMessage::Control(AbiControl::Close { handle }) => {
                    let listener = self.listener.ok_or(BrowserHostError::Unavailable(BrowserHostUnavailable::Listener))?;
                    validate_listener_handle(listener, handle)?;
                    self.begin_close();
                    Ok(BrowserHostStep::Closed)
                }
                _ => Err(BrowserHostError::Abi(AbiErrorCode::MalformedTag)),
            }
        }

        fn accept_reply(&mut self, reply: AbiReply) -> Result<BrowserHostStep, BrowserHostError> {
            self.replies.accept(&reply)?;
            if self.detach_request == Some(reply.request_id) {
                self.detach_request = None;
                self.detach_acknowledged = true;
                return if reply.status.code == AbiStatusCode::Ok { Ok(BrowserHostStep::Closed) } else { Ok(BrowserHostStep::Unavailable(unavailable_from_reply(&reply))) };
            }
            if reply.status.code != AbiStatusCode::Ok {
                let unavailable = unavailable_from_reply(&reply);
                if self.clipboard.as_ref().is_some_and(|request| request.request_id == reply.request_id) {
                    self.clipboard = None;
                }
                return Ok(BrowserHostStep::Unavailable(unavailable));
            }
            if self.listener.is_none() && reply.request_id == AbiRequestId(self.canvas.get() as u64) {
                self.listener = Some(parse_listener(reply.bytes.as_slice())?);
                return Ok(BrowserHostStep::Event);
            }
            if self.clipboard.as_ref().is_some_and(|request| request.request_id == reply.request_id) {
                let request = self.clipboard.take().expect("checked");
                let text = if request.operation == ClipboardOperation::Read {
                    let text = String::from_utf8(reply.bytes.into_vec()).map_err(|_| BrowserHostError::Abi(AbiErrorCode::InvalidUtf8))?;
                    self.clipboard_mailbox.settle_read(text.clone());
                    self.delegate.handle_event(DispatchEvent::Paste { text: text.clone() });
                    Some(text)
                } else {
                    None
                };
                return Ok(BrowserHostStep::Clipboard { request_id: request.request_id, text });
            }
            Ok(BrowserHostStep::Event)
        }

        fn classify_event(&mut self, event: AbiEvent) -> Result<(), BrowserHostError> {
            let listener = self.listener.ok_or(BrowserHostError::Unavailable(BrowserHostUnavailable::Listener))?;
            match decode_browser_host_event(&event, self.canvas, listener)? {
                BrowserHostEvent::Metrics(metrics) => {
                    if let Some((replaced, _)) = self.latest_metrics.replace((event, WindowMetrics { physical: PhysicalSize::new(metrics.width, metrics.height), scale_factor: metrics.scale_factor })) {
                        self.acknowledge_event(replaced)?;
                    }
                }
                BrowserHostEvent::Visibility { visible } => {
                    self.delegate.scheduler_mut().set_visible(visible);
                    self.acknowledge_event(event)?;
                }
                BrowserHostEvent::Frame { timestamp_ms } => {
                    self.pending_frame = Some((event, FrameSchedulerEnvelope { canvas: self.canvas, generation: self.generation, timestamp_ms }));
                }
                BrowserHostEvent::Dispatch(dispatch) => {
                    if matches!(dispatch, DispatchEvent::PointerMove { .. }) {
                        if let Some((replaced, _)) = self.latest_pointer.replace((event, dispatch)) {
                            self.acknowledge_event(replaced)?;
                        }
                    } else {
                        self.pending_dispatch = Some((event, dispatch));
                    }
                }
                BrowserHostEvent::Closed => self.begin_close(),
            }
            Ok(())
        }

        fn acknowledge_event(&mut self, event: AbiEvent) -> Result<(), BrowserHostError> {
            if self.outbound.is_some() {
                return Err(BrowserHostError::Busy);
            }
            self.outbound = Some(AbiMessage::Reply(AbiReply { request_id: event.request_id, generation: event.generation, status: crate::abi::AbiStatus::OK, bytes: AbiBytes::default() }));
            self.outbound_inspected_bytes = 0;
            Ok(())
        }

        fn stage_cursor(&mut self, cursor: CursorRequest) -> Result<(), BrowserHostError> {
            if self.last_cursor == Some(cursor) {
                return Ok(());
            }
            self.last_cursor = Some(cursor);
            let request_id = self.next_request();
            let mut bytes = canvas_payload(self.canvas);
            bytes.push(cursor_tag(cursor));
            self.stage_request(request(BROWSER_HOST_OPERATION_CURSOR, request_id, self.generation, bytes)?)
        }

        fn stage_clipboard(&mut self, operation: ClipboardOperation, text: Vec<u8>) -> Result<AbiRequestId, BrowserHostError> {
            if self.closing {
                return Err(BrowserHostError::Closed);
            }
            if self.clipboard.is_some() || self.outbound.is_some() {
                return Err(BrowserHostError::Busy);
            }
            let request_id = self.next_request();
            let mut bytes = canvas_payload(self.canvas);
            bytes.extend_from_slice(&text);
            let code = if operation == ClipboardOperation::Read { BROWSER_HOST_OPERATION_CLIPBOARD_READ } else { BROWSER_HOST_OPERATION_CLIPBOARD_WRITE };
            let request = request(code, request_id, self.generation, bytes)?;
            self.replies.admit(request_id, self.generation)?;
            self.outbound = Some(AbiMessage::Request(request));
            self.outbound_inspected_bytes = 0;
            self.clipboard = Some(ClipboardRequest { request_id, generation: self.generation, operation, text });
            Ok(request_id)
        }

        fn stage_request(&mut self, request: AbiRequest) -> Result<(), BrowserHostError> {
            if self.outbound.is_some() {
                return Err(BrowserHostError::Busy);
            }
            self.replies.admit(request.request_id, request.generation)?;
            self.outbound = Some(AbiMessage::Request(request));
            self.outbound_inspected_bytes = 0;
            Ok(())
        }

        fn next_request(&mut self) -> AbiRequestId {
            let request = AbiRequestId(self.next_request_id);
            self.next_request_id = self.next_request_id.checked_add(1).unwrap_or(1);
            request
        }

        fn close_step(&mut self, budget: AbiWorkBudget) -> Result<BrowserHostStep, BrowserHostError> {
            if let Some(request) = self.clipboard.take() {
                self.replies.lose(request.request_id, request.generation)?;
                self.outbound = Some(AbiMessage::Control(AbiControl::Cancel { request_id: request.request_id, generation: request.generation }));
                self.outbound_inspected_bytes = 0;
                return Ok(progress(1, 3));
            }
            if self.outbound.is_some() {
                let total = message_body_bytes(self.outbound.as_ref().expect("checked")).max(1);
                self.outbound_inspected_bytes += 1;
                if self.outbound_inspected_bytes < total {
                    return Ok(progress(self.outbound_inspected_bytes as u32, total as u32));
                }
                let message = self.outbound.take().expect("checked");
                self.port.try_send(message, one_credit(budget)).map_err(|rejection| {
                    self.outbound = Some(rejection.message);
                    self.outbound_inspected_bytes = total.saturating_sub(1);
                    BrowserHostError::Abi(rejection.code)
                })?;
                self.outbound_inspected_bytes = 0;
                return Ok(progress(1, 2));
            }
            if let Some(pending) = self.pending_reply.as_mut() {
                pending.inspected_bytes += 1;
                let total = pending.reply.bytes.len().max(1);
                if pending.inspected_bytes < total {
                    return Ok(progress(pending.inspected_bytes as u32, total as u32));
                }
                let pending = self.pending_reply.take().expect("checked");
                return self.accept_reply(pending.reply);
            }
            if let Some(pending) = self.pending_event.as_mut() {
                pending.inspected_bytes += 1;
                let total = pending.event.bytes.len().max(1);
                if pending.inspected_bytes < total {
                    return Ok(progress(pending.inspected_bytes as u32, total as u32));
                }
                let event = self.pending_event.take().expect("checked").event;
                self.acknowledge_event(event)?;
                return Ok(progress(total as u32, total as u32));
            }
            if let Some(pending) = self.pending_page.as_mut() {
                pending.inspected_bytes += 1;
                let total = pending.page.bytes.len().max(1);
                if pending.inspected_bytes < total {
                    return Ok(progress(pending.inspected_bytes as u32, total as u32));
                }
                let page = self.pending_page.take().expect("checked").page;
                self.outbound = Some(AbiMessage::Control(AbiControl::Acknowledge { handle: page.handle, index: page.index }));
                self.outbound_inspected_bytes = 0;
                return Ok(progress(total as u32, total as u32));
            }
            if let Some((event, _)) = self.pending_dispatch.take() {
                self.acknowledge_event(event)?;
                return Ok(progress(1, 1));
            }
            if let Some((event, _)) = self.latest_metrics.take() {
                self.acknowledge_event(event)?;
                return Ok(progress(1, 1));
            }
            if let Some((event, _)) = self.latest_pointer.take() {
                self.acknowledge_event(event)?;
                return Ok(progress(1, 1));
            }
            if let Some((event, _)) = self.pending_frame.take() {
                self.acknowledge_event(event)?;
                return Ok(progress(1, 1));
            }
            if let Some(listener) = self.listener.take() {
                self.outbound = Some(AbiMessage::Control(AbiControl::Close { handle: listener.handle() }));
                self.outbound_inspected_bytes = 0;
                return Ok(progress(1, 2));
            }
            if !self.detach_sent {
                let request_id = self.next_request();
                let detach = request(BROWSER_HOST_OPERATION_DETACH, request_id, self.generation, canvas_payload(self.canvas))?;
                self.replies.admit(request_id, self.generation)?;
                self.outbound = Some(AbiMessage::Request(detach));
                self.outbound_inspected_bytes = 0;
                self.detach_sent = true;
                self.detach_request = Some(request_id);
                self.pending_cursor = None;
                return Ok(progress(1, 1));
            }
            if self.detach_acknowledged {
                return Ok(BrowserHostStep::Closed);
            }
            match self.port.poll(one_credit(budget))? {
                AbiPortPoll::Pending => Ok(BrowserHostStep::AwaitingHost),
                AbiPortPoll::Closed => Err(BrowserHostError::Closed),
                AbiPortPoll::Message(AbiMessage::Event(event)) => {
                    if event.generation != self.generation || event.bytes.len() > BROWSER_HOST_MAX_EVENT_BODY_BYTES {
                        let code = if event.generation < self.generation { AbiErrorCode::AbaHandle } else { AbiErrorCode::StaleGeneration };
                        return Err(BrowserHostError::Abi(code));
                    }
                    self.pending_event = Some(PendingEvent { event, inspected_bytes: 0 });
                    Ok(progress(1, 1))
                }
                AbiPortPoll::Message(AbiMessage::Reply(reply)) => {
                    self.pending_reply = Some(PendingReply { reply, inspected_bytes: 0 });
                    Ok(progress(1, 1))
                }
                AbiPortPoll::Message(AbiMessage::Page(page)) => {
                    if page.bytes.len() > BROWSER_HOST_MAX_PAGE_BODY_BYTES {
                        return Err(BrowserHostError::Abi(AbiErrorCode::LimitExceeded));
                    }
                    self.pending_page = Some(PendingPage { page, inspected_bytes: 0 });
                    Ok(progress(1, 1))
                }
                AbiPortPoll::Message(AbiMessage::Control(AbiControl::Close { .. })) => Ok(progress(1, 1)),
                AbiPortPoll::Message(_) => Err(BrowserHostError::Abi(AbiErrorCode::MalformedTag)),
            }
        }
    }

    fn request(operation: u16, request_id: AbiRequestId, generation: u32, bytes: Vec<u8>) -> Result<AbiRequest, BrowserHostError> {
        Ok(AbiRequest { operation: AbiOperation::try_new(operation)?, request_id, generation, bytes: AbiBytes::try_new(bytes).map_err(|rejected| rejected.code)? })
    }

    fn canvas_payload(canvas: CanvasId) -> Vec<u8> {
        let mut bytes = vec![1];
        bytes.extend_from_slice(&canvas.get().to_le_bytes());
        bytes
    }

    fn parse_listener(bytes: &[u8]) -> Result<ListenerId, BrowserHostError> {
        if bytes.len() != 9 || bytes[0] != 1 {
            return Err(BrowserHostError::MalformedHostReply);
        }
        ListenerId::try_new(u32::from_le_bytes(bytes[1..5].try_into().expect("fixed")), u32::from_le_bytes(bytes[5..9].try_into().expect("fixed"))).map_err(BrowserHostError::Abi)
    }

    fn validate_listener_handle(listener: ListenerId, actual: crate::abi::AbiHandle) -> Result<(), BrowserHostError> {
        let expected = listener.handle();
        if actual.slot() != expected.slot() {
            Err(BrowserHostError::Abi(AbiErrorCode::UnknownHandle))
        } else if actual.generation() < expected.generation() {
            Err(BrowserHostError::Abi(AbiErrorCode::AbaHandle))
        } else if actual.generation() > expected.generation() {
            Err(BrowserHostError::Abi(AbiErrorCode::StaleGeneration))
        } else {
            Ok(())
        }
    }

    fn unavailable_from_reply(reply: &AbiReply) -> BrowserHostUnavailable {
        match reply.bytes.as_slice().first().copied() {
            Some(1) => BrowserHostUnavailable::Window,
            Some(2) => BrowserHostUnavailable::Document,
            Some(3) => BrowserHostUnavailable::Canvas,
            Some(4) => BrowserHostUnavailable::Clipboard,
            Some(5) => BrowserHostUnavailable::Listener,
            _ => BrowserHostUnavailable::Callback,
        }
    }

    fn cursor_tag(cursor: CursorRequest) -> u8 {
        match cursor {
            CursorRequest::Default => 0,
            CursorRequest::Pointer => 1,
            CursorRequest::Text => 2,
            CursorRequest::Grab => 3,
            CursorRequest::Grabbing => 4,
        }
    }

    fn validate_budget(budget: AbiWorkBudget) -> Result<(), BrowserHostError> {
        if budget.cancelled {
            Err(BrowserHostError::Abi(AbiErrorCode::Cancelled))
        } else if budget.interrupted {
            Err(BrowserHostError::Abi(AbiErrorCode::Interrupted))
        } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
            Err(BrowserHostError::Abi(AbiErrorCode::DeadlineExceeded))
        } else if budget.byte_credit == 0 {
            Err(BrowserHostError::Abi(AbiErrorCode::NoCredit))
        } else {
            Ok(())
        }
    }

    fn one_credit(budget: AbiWorkBudget) -> AbiWorkBudget {
        AbiWorkBudget { byte_credit: 1, ..budget }
    }

    fn progress(completed_units: u32, total_units: u32) -> BrowserHostStep {
        BrowserHostStep::Progress(BrowserHostProgress { completed_units, total_units })
    }

    fn message_body_bytes(message: &AbiMessage) -> usize {
        match message {
            AbiMessage::Request(value) => value.bytes.len(),
            AbiMessage::Reply(value) => value.bytes.len(),
            AbiMessage::Event(value) => value.bytes.len(),
            AbiMessage::Page(value) => value.bytes.len(),
            AbiMessage::Control(_) => 0,
        }
    }

    /// 🧠️ Low-level Wasm import adapter with an owned public surface.
    #[derive(Default)]
    pub struct LinearMemoryBrowserHostPort;

    #[cfg(all(target_arch = "wasm32", not(test)))]
    #[link(wasm_import_module = "semio_browser_host")]
    unsafe extern "C" {
        #[link_name = "send"]
        fn browser_host_send(message_pointer: *const u8, message_length: usize) -> i32;
        #[link_name = "poll"]
        fn browser_host_poll(message_pointer: *mut u8, message_capacity: usize) -> i32;
    }

    #[cfg(all(target_arch = "wasm32", test))]
    unsafe fn browser_host_send(message_pointer: *const u8, message_length: usize) -> i32 {
        linear_memory_test_import::send(message_pointer, message_length)
    }

    #[cfg(all(target_arch = "wasm32", test))]
    unsafe fn browser_host_poll(message_pointer: *mut u8, message_capacity: usize) -> i32 {
        linear_memory_test_import::poll(message_pointer, message_capacity)
    }

    #[cfg(all(target_arch = "wasm32", test))]
    pub(super) mod linear_memory_test_import {
        use std::cell::RefCell;
        use std::collections::VecDeque;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(crate) enum AfterProbe {
            None,
            Cancel,
            Close,
        }

        #[derive(Default)]
        struct State {
            incoming: VecDeque<Vec<u8>>,
            sent: Vec<Vec<u8>>,
            capacities: Vec<usize>,
            copies: usize,
            closed: bool,
            after_probe: Option<AfterProbe>,
        }

        thread_local! {
            static STATE: RefCell<State> = RefCell::new(State::default());
        }

        pub(crate) fn reset() {
            STATE.with(|state| *state.borrow_mut() = State::default());
        }

        pub(crate) fn enqueue(bytes: Vec<u8>) {
            STATE.with(|state| state.borrow_mut().incoming.push_back(bytes));
        }

        pub(crate) fn after_probe(action: AfterProbe) {
            STATE.with(|state| state.borrow_mut().after_probe = Some(action));
        }

        pub(crate) fn census() -> (Vec<usize>, usize, usize, bool) {
            STATE.with(|state| {
                let state = state.borrow();
                (state.capacities.clone(), state.copies, state.incoming.len(), state.closed)
            })
        }

        pub(crate) fn sent_lengths() -> Vec<usize> {
            STATE.with(|state| state.borrow().sent.iter().map(Vec::len).collect())
        }

        pub(super) unsafe fn send(pointer: *const u8, length: usize) -> i32 {
            let bytes = unsafe { std::slice::from_raw_parts(pointer, length) }.to_vec();
            STATE.with(|state| state.borrow_mut().sent.push(bytes));
            1
        }

        pub(super) unsafe fn poll(pointer: *mut u8, capacity: usize) -> i32 {
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                state.capacities.push(capacity);
                let Some(bytes) = state.incoming.front() else { return if state.closed { -1 } else { 0 } };
                let length = bytes.len();
                if length > capacity {
                    match state.after_probe.take().unwrap_or(AfterProbe::None) {
                        AfterProbe::None => {}
                        AfterProbe::Cancel => {
                            state.incoming.pop_front();
                        }
                        AfterProbe::Close => {
                            state.incoming.clear();
                            state.closed = true;
                        }
                    }
                    return length as i32;
                }
                unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer, length) };
                state.incoming.pop_front();
                state.copies += 1;
                length as i32
            })
        }
    }

    impl AbiPort for LinearMemoryBrowserHostPort {
        fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), crate::abi::AbiPortRejection> {
            if let Err(error) = validate_budget(budget) {
                let BrowserHostError::Abi(code) = error else { unreachable!() };
                return Err(crate::abi::AbiPortRejection { code, message });
            }
            #[cfg(target_arch = "wasm32")]
            {
                let bytes = crate::abi::encode_abi_message(&message);
                if bytes.len() > BROWSER_HOST_MAX_ENCODED_EVENT_BYTES {
                    return Err(crate::abi::AbiPortRejection { code: AbiErrorCode::LimitExceeded, message });
                }
                if unsafe { browser_host_send(bytes.as_ptr(), bytes.len()) } == 1 {
                    Ok(())
                } else {
                    Err(crate::abi::AbiPortRejection { code: AbiErrorCode::Interrupted, message })
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            Err(crate::abi::AbiPortRejection { code: AbiErrorCode::Closed, message })
        }

        fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
            validate_budget(budget).map_err(|error| match error {
                BrowserHostError::Abi(code) => code,
                _ => AbiErrorCode::Interrupted,
            })?;
            #[cfg(target_arch = "wasm32")]
            {
                let mut bytes = vec![0; BROWSER_HOST_INITIAL_POLL_BYTES];
                let mut length = unsafe { browser_host_poll(bytes.as_mut_ptr(), bytes.len()) };
                if length == 0 {
                    return Ok(AbiPortPoll::Pending);
                }
                if length < 0 {
                    return Ok(AbiPortPoll::Closed);
                }
                let required = length as usize;
                if required > BROWSER_HOST_MAX_ENCODED_EVENT_BYTES {
                    return Err(AbiErrorCode::LimitExceeded);
                }
                if required > bytes.len() {
                    bytes.resize(required, 0);
                    length = unsafe { browser_host_poll(bytes.as_mut_ptr(), bytes.len()) };
                    if length == 0 {
                        return Ok(AbiPortPoll::Pending);
                    }
                    if length < 0 {
                        return Ok(AbiPortPoll::Closed);
                    }
                    if length as usize > BROWSER_HOST_MAX_ENCODED_EVENT_BYTES {
                        return Err(AbiErrorCode::LimitExceeded);
                    }
                    if length as usize != required {
                        return Err(AbiErrorCode::MalformedLength);
                    }
                }
                bytes.truncate(length as usize);
                let message = crate::abi::decode_abi_message(&bytes)?;
                match &message {
                    AbiMessage::Event(event) if event.bytes.len() > BROWSER_HOST_MAX_EVENT_BODY_BYTES => return Err(AbiErrorCode::LimitExceeded),
                    AbiMessage::Page(page) if page.bytes.len() > BROWSER_HOST_MAX_PAGE_BODY_BYTES => return Err(AbiErrorCode::LimitExceeded),
                    _ => {}
                }
                Ok(AbiPortPoll::Message(message))
            }
            #[cfg(not(target_arch = "wasm32"))]
            Ok(AbiPortPoll::Closed)
        }
    }
}

pub use browser::*;

//#endregion 🌐️Browser

//#endregion 🔖️Host

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    //#region 📐️Metrics tests

    #[test]
    fn scale_factor_change_produces_the_right_logical_metrics() {
        let metrics = WindowMetrics { physical: PhysicalSize::new(1600, 1200), scale_factor: 2.0 };
        assert_eq!(metrics.logical_size(), (800.0, 600.0));
    }

    #[test]
    fn a_zero_scale_factor_parks_instead_of_dividing_by_zero() {
        let metrics = WindowMetrics { physical: PhysicalSize::new(100, 100), scale_factor: 0.0 };
        assert_eq!(metrics.logical_size(), (0.0, 0.0));
    }

    //#endregion 📐️Metrics tests

    //#region 🎯️Redraw gating tests

    #[test]
    fn a_clean_window_never_requests_a_redraw() {
        let mut scheduler = FrameScheduler::new();
        assert_eq!(should_request_redraw(&mut scheduler, 0.0), None);
    }

    #[test]
    fn a_dirty_window_requests_exactly_one_redraw() {
        let mut scheduler = FrameScheduler::new();
        scheduler.invalidate(InvalidationReason::PAINT);
        assert!(should_request_redraw(&mut scheduler, 0.0).is_some());
        assert_eq!(should_request_redraw(&mut scheduler, 0.0), None, "must not double-fire");
    }

    #[test]
    fn a_due_deadline_requests_exactly_one_redraw() {
        let mut scheduler = FrameScheduler::new();
        scheduler.request_deadline(5.0, InvalidationReason::ANIMATION);
        assert_eq!(should_request_redraw(&mut scheduler, 4.0), None, "not due yet");
        assert!(should_request_redraw(&mut scheduler, 5.0).is_some());
        assert_eq!(should_request_redraw(&mut scheduler, 5.0), None);
    }

    //#endregion 🎯️Redraw gating tests

    //#region 🚦️Native control-flow tests

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn no_deadline_waits_indefinitely() {
        let scheduler = FrameScheduler::new();
        let clock = MonotonicClock::new();
        assert!(matches!(control_flow_for(&scheduler, &clock), winit::event_loop::ControlFlow::Wait));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_pending_deadline_waits_until_a_specific_instant() {
        let mut scheduler = FrameScheduler::new();
        scheduler.request_deadline(1.0, InvalidationReason::ANIMATION);
        let clock = MonotonicClock::new();
        assert!(matches!(control_flow_for(&scheduler, &clock), winit::event_loop::ControlFlow::WaitUntil(_)));
    }

    //#endregion 🚦️Native control-flow tests

    //#region 🖱️Cursor tests

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_cursor_request_maps_to_a_distinct_icon() {
        use std::collections::HashSet;
        let icons: HashSet<_> = [CursorRequest::Default, CursorRequest::Pointer, CursorRequest::Text, CursorRequest::Grab, CursorRequest::Grabbing].into_iter().map(|cursor| format!("{:?}", cursor_icon_for(cursor))).collect();
        assert_eq!(icons.len(), 5);
    }

    #[test]
    fn cursor_css_names_match_the_standard_css_keywords() {
        assert_eq!(cursor_css_for(CursorRequest::Grabbing), "grabbing");
        assert_eq!(cursor_css_for(CursorRequest::Default), "default");
    }

    //#endregion 🖱️Cursor tests

    //#region 🌐️Browser host laws

    #[derive(Default)]
    struct BrowserFixtureState {
        incoming: std::collections::VecDeque<AbiMessage>,
        sent: Vec<AbiMessage>,
        reject_next: bool,
        closed: bool,
    }

    #[derive(Clone, Default)]
    struct BrowserFixturePort(std::rc::Rc<std::cell::RefCell<BrowserFixtureState>>);

    impl AbiPort for BrowserFixturePort {
        fn try_send(&mut self, message: AbiMessage, _budget: AbiWorkBudget) -> Result<(), crate::abi::AbiPortRejection> {
            let mut state = self.0.borrow_mut();
            if state.reject_next {
                state.reject_next = false;
                return Err(crate::abi::AbiPortRejection { code: AbiErrorCode::Interrupted, message });
            }
            state.sent.push(message);
            Ok(())
        }

        fn poll(&mut self, _budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
            let mut state = self.0.borrow_mut();
            Ok(state.incoming.pop_front().map(AbiPortPoll::Message).unwrap_or(if state.closed { AbiPortPoll::Closed } else { AbiPortPoll::Pending }))
        }
    }

    struct BrowserFixtureDelegate {
        scheduler: FrameScheduler,
        events: Vec<DispatchEvent>,
        metrics: Vec<WindowMetrics>,
        frames: usize,
    }

    impl Default for BrowserFixtureDelegate {
        fn default() -> Self {
            Self { scheduler: FrameScheduler::new(), events: Vec::new(), metrics: Vec::new(), frames: 0 }
        }
    }

    impl WindowDelegate for BrowserFixtureDelegate {
        fn scheduler_mut(&mut self) -> &mut FrameScheduler {
            &mut self.scheduler
        }

        fn handle_event(&mut self, event: DispatchEvent) {
            self.events.push(event);
        }

        fn handle_metrics(&mut self, metrics: WindowMetrics) {
            self.metrics.push(metrics);
        }

        fn redraw(&mut self, _reason: InvalidationReason) -> RedrawOutcome {
            RedrawOutcome { cursor: CursorRequest::Pointer, ime: None }
        }
    }

    impl BrowserWindowDelegate for BrowserFixtureDelegate {
        fn enqueue_browser_frame(&mut self, _reason: InvalidationReason) {
            self.frames += 1;
        }

        fn present_browser_frame(&mut self) -> RedrawOutcome {
            self.redraw(InvalidationReason::PAINT)
        }
    }

    fn browser_fixture() -> (CanvasHost<BrowserFixturePort, BrowserFixtureDelegate>, BrowserFixturePort) {
        let port = BrowserFixturePort::default();
        let host = CanvasHost::new(crate::event::CanvasId::try_new(1).unwrap(), port.clone(), BrowserFixtureDelegate::default()).unwrap();
        (host, port)
    }

    fn attach_browser(host: &mut CanvasHost<BrowserFixturePort, BrowserFixtureDelegate>, port: &BrowserFixturePort) {
        while port.0.borrow().sent.is_empty() {
            host.step(AbiWorkBudget::credits(1)).unwrap();
        }
        let body = AbiBytes::try_new([vec![1], 1_u32.to_le_bytes().to_vec(), 1_u32.to_le_bytes().to_vec()].concat()).unwrap();
        port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: crate::abi::AbiStatus::OK, bytes: body }));
        host.step(AbiWorkBudget::credits(1)).unwrap();
        for _ in 0..8 {
            assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
        }
        assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Event);
    }

    fn browser_event(code: u16, sequence: u32, tail: Vec<u8>, generation: u32) -> AbiMessage {
        let mut bytes = vec![1];
        bytes.extend_from_slice(&1_u32.to_le_bytes());
        bytes.extend_from_slice(&1_u32.to_le_bytes());
        bytes.extend_from_slice(&1_u32.to_le_bytes());
        bytes.extend_from_slice(&tail);
        AbiMessage::Event(AbiEvent { request_id: AbiRequestId(100 + sequence as u64), generation, sequence, event: crate::abi::AbiEventCode::try_new(code).unwrap(), status: crate::abi::AbiStatus::OK, bytes: AbiBytes::try_new(bytes).unwrap() })
    }

    fn raw_browser_event(body_bytes: usize) -> AbiMessage {
        AbiMessage::Event(AbiEvent {
            request_id: AbiRequestId(900),
            generation: 1,
            sequence: 1,
            event: crate::abi::AbiEventCode::try_new(crate::event::BROWSER_EVENT_TEXT).unwrap(),
            status: crate::abi::AbiStatus::OK,
            bytes: AbiBytes::try_new(vec![0; body_bytes]).unwrap(),
        })
    }

    fn inspect_and_classify(host: &mut CanvasHost<BrowserFixturePort, BrowserFixtureDelegate>, bytes: usize) {
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
        for _ in 0..bytes {
            assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
        }
    }

    #[test]
    fn browser_missing_objects_and_interrupted_send_are_owned_failures() {
        let (mut host, port) = browser_fixture();
        port.0.borrow_mut().reject_next = true;
        for _ in 0..4 {
            assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
        }
        assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::Interrupted)));
        host.step(AbiWorkBudget::credits(1)).unwrap();
        port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: crate::abi::AbiStatus { code: AbiStatusCode::Failed, error: None }, bytes: AbiBytes::try_new(vec![2]).unwrap() }));
        host.step(AbiWorkBudget::credits(1)).unwrap();
        assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Unavailable(BrowserHostUnavailable::Document));
        for (code, expected) in [(1, BrowserHostUnavailable::Window), (3, BrowserHostUnavailable::Canvas), (4, BrowserHostUnavailable::Clipboard)] {
            let (mut host, port) = browser_fixture();
            while port.0.borrow().sent.is_empty() {
                host.step(AbiWorkBudget::credits(1)).unwrap();
            }
            port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id: AbiRequestId(1), generation: 1, status: crate::abi::AbiStatus { code: AbiStatusCode::Failed, error: None }, bytes: AbiBytes::try_new(vec![code]).unwrap() }));
            host.step(AbiWorkBudget::credits(1)).unwrap();
            assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Unavailable(expected));
        }
    }

    #[test]
    fn browser_resize_storm_is_latest_wins_and_one_byte_is_inspected_per_grant() {
        let (mut host, port) = browser_fixture();
        attach_browser(&mut host, &port);
        for (sequence, width) in [(1, 10_u32), (2, 20_u32)] {
            let mut tail = width.to_le_bytes().to_vec();
            tail.extend_from_slice(&30_u32.to_le_bytes());
            tail.extend_from_slice(&1_f32.to_le_bytes());
            let message = browser_event(crate::event::BROWSER_EVENT_METRICS, sequence, tail, 1);
            let length = match &message {
                AbiMessage::Event(event) => event.bytes.len(),
                _ => 0,
            };
            port.0.borrow_mut().incoming.push_back(message);
            inspect_and_classify(&mut host, length);
        }
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
        assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Event);
        assert_eq!(host.delegate().metrics, vec![WindowMetrics { physical: PhysicalSize::new(20, 30), scale_factor: 1.0 }]);
    }

    #[test]
    fn browser_frame_is_bounded_and_stale_generation_is_rejected() {
        let (mut host, port) = browser_fixture();
        attach_browser(&mut host, &port);
        assert_eq!(host.request_wake().unwrap(), true);
        assert_eq!(host.request_wake().unwrap(), false);
        let sent = port.0.borrow().sent.len();
        while port.0.borrow().sent.len() == sent {
            host.step(AbiWorkBudget::credits(1)).unwrap();
        }
        port.0.borrow_mut().incoming.push_back(browser_event(crate::event::BROWSER_EVENT_FRAME, 1, 16_f64.to_le_bytes().to_vec(), 2));
        assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::StaleGeneration)));
    }

    #[test]
    fn browser_frame_ack_and_cursor_are_separate_bounded_steps() {
        let (mut host, port) = browser_fixture();
        attach_browser(&mut host, &port);
        host.delegate_mut().scheduler.invalidate(InvalidationReason::PAINT);
        assert!(host.request_wake().unwrap());
        let sent = port.0.borrow().sent.len();
        while port.0.borrow().sent.len() == sent {
            host.step(AbiWorkBudget::credits(1)).unwrap();
        }
        let message = browser_event(crate::event::BROWSER_EVENT_FRAME, 1, 16_f64.to_le_bytes().to_vec(), 1);
        let length = match &message {
            AbiMessage::Event(event) => event.bytes.len(),
            _ => unreachable!(),
        };
        port.0.borrow_mut().incoming.push_back(message);
        inspect_and_classify(&mut host, length);
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Frame(_)));
        host.step(AbiWorkBudget::credits(1)).unwrap();
        host.step(AbiWorkBudget::credits(1)).unwrap();
        for _ in 0..6 {
            host.step(AbiWorkBudget::credits(1)).unwrap();
        }
        assert_eq!(host.delegate().frames, 1);
        assert!(port.0.borrow().sent.iter().any(|message| matches!(message, AbiMessage::Request(request) if request.operation.get() == BROWSER_HOST_OPERATION_CURSOR)));
    }

    #[test]
    fn browser_clipboard_cancel_before_during_after_and_close_are_deterministic() {
        let (mut before, before_port) = browser_fixture();
        attach_browser(&mut before, &before_port);
        before.request_clipboard_read().unwrap();
        assert_eq!(before.cancel_clipboard().unwrap(), true);
        assert_eq!(before.cancel_clipboard().unwrap(), false);

        let (mut during, during_port) = browser_fixture();
        attach_browser(&mut during, &during_port);
        during.request_clipboard_read().unwrap();
        let sent = during_port.0.borrow().sent.len();
        while during_port.0.borrow().sent.len() == sent {
            during.step(AbiWorkBudget::credits(1)).unwrap();
        }
        assert_eq!(during.cancel_clipboard().unwrap(), true);

        let (mut after, after_port) = browser_fixture();
        attach_browser(&mut after, &after_port);
        let request_id = after.request_clipboard_read().unwrap();
        let sent = after_port.0.borrow().sent.len();
        while after_port.0.borrow().sent.len() == sent {
            after.step(AbiWorkBudget::credits(1)).unwrap();
        }
        after_port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id, generation: 1, status: crate::abi::AbiStatus::OK, bytes: AbiBytes::try_new(b"x".to_vec()).unwrap() }));
        after.step(AbiWorkBudget::credits(1)).unwrap();
        assert!(matches!(after.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Clipboard { text: Some(text), .. } if text == "x"));
        assert_eq!(after.cancel_clipboard().unwrap(), false);

        during.begin_close();
        assert_eq!(during.step(AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(BrowserHostError::Abi(AbiErrorCode::Interrupted)));
        let mut detach_replied = false;
        for _ in 0..64 {
            during.step(AbiWorkBudget::credits(1)).unwrap();
            if !detach_replied {
                let detach = during_port.0.borrow().sent.iter().find_map(|message| match message {
                    AbiMessage::Request(request) if request.operation.get() == BROWSER_HOST_OPERATION_DETACH => Some((request.request_id, request.generation)),
                    _ => None,
                });
                if let Some((request_id, generation)) = detach {
                    assert!(!during.terminal_is_empty());
                    during_port.0.borrow_mut().incoming.push_back(AbiMessage::Reply(AbiReply { request_id, generation, status: crate::abi::AbiStatus::OK, bytes: AbiBytes::default() }));
                    detach_replied = true;
                }
            }
            if during.terminal_is_empty() {
                break;
            }
        }
        assert!(detach_replied);
        assert!(during.terminal_is_empty());
    }

    #[test]
    fn browser_listener_aba_and_event_decoder_terminal_laws() {
        let listener = crate::event::ListenerId::try_new(1, 2).unwrap();
        let event = match browser_event(crate::event::BROWSER_EVENT_CLOSE, 1, Vec::new(), 1) {
            AbiMessage::Event(event) => event,
            _ => unreachable!(),
        };
        assert_eq!(crate::event::decode_browser_host_event(&event, crate::event::CanvasId::try_new(1).unwrap(), listener), Err(AbiErrorCode::AbaHandle));
        assert_eq!(crate::event::CanvasId::try_new(0), Err(AbiErrorCode::UnknownHandle));
        let (mut host, port) = browser_fixture();
        attach_browser(&mut host, &port);
        port.0.borrow_mut().incoming.push_back(AbiMessage::Control(AbiControl::Close { handle: crate::abi::AbiHandle::try_new(1, 2).unwrap() }));
        assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::StaleGeneration)));
        port.0.borrow_mut().incoming.push_back(AbiMessage::Control(AbiControl::Close { handle: crate::abi::AbiHandle::try_new(2, 1).unwrap() }));
        assert_eq!(host.step(AbiWorkBudget::credits(1)), Err(BrowserHostError::Abi(AbiErrorCode::UnknownHandle)));
    }

    #[test]
    fn browser_linear_memory_exact_envelope_retry_and_preflight_laws() {
        use super::browser::linear_memory_test_import::{self as seam, AfterProbe};

        seam::reset();
        let empty = crate::abi::encode_abi_message(&raw_browser_event(0));
        assert_eq!(empty.len(), BROWSER_HOST_EVENT_ENVELOPE_BYTES);
        seam::enqueue(empty);
        let mut port = LinearMemoryBrowserHostPort;
        assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Event(event)) if event.bytes.is_empty()));
        assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 1, 0, false));

        seam::reset();
        let maximum = crate::abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES));
        assert_eq!(maximum.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES);
        seam::enqueue(maximum);
        assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Event(event)) if event.bytes.len() == BROWSER_HOST_MAX_EVENT_BODY_BYTES));
        assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES], 1, 0, false));
        assert_eq!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Pending);

        seam::reset();
        let maximum_send = raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES);
        port.try_send(maximum_send, AbiWorkBudget::credits(1)).unwrap();
        assert_eq!(seam::sent_lengths(), vec![BROWSER_HOST_MAX_ENCODED_EVENT_BYTES]);
        let oversized_send = raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES + 1);
        let rejection = port.try_send(oversized_send.clone(), AbiWorkBudget::credits(1)).unwrap_err();
        assert_eq!(rejection.code, AbiErrorCode::LimitExceeded);
        assert_eq!(rejection.message, oversized_send);
        assert_eq!(seam::sent_lengths(), vec![BROWSER_HOST_MAX_ENCODED_EVENT_BYTES]);

        seam::reset();
        let oversized = crate::abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES + 1));
        assert_eq!(oversized.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1);
        seam::enqueue(oversized);
        assert_eq!(port.poll(AbiWorkBudget::credits(1)), Err(AbiErrorCode::LimitExceeded));
        assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 0, 1, false));

        seam::reset();
        let page = AbiPage::try_new(crate::abi::AbiHandle::try_new(1, 1).unwrap(), 0, vec![7; BROWSER_HOST_MAX_PAGE_BODY_BYTES]).unwrap();
        let encoded_page = crate::abi::encode_abi_message(&AbiMessage::Page(page));
        assert_eq!(encoded_page.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES);
        seam::enqueue(encoded_page);
        assert!(matches!(port.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Message(AbiMessage::Page(page)) if page.bytes.len() == BROWSER_HOST_MAX_PAGE_BODY_BYTES));
        assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES], 1, 0, false));

        seam::reset();
        let oversized_page = AbiPage::try_new(crate::abi::AbiHandle::try_new(1, 1).unwrap(), 0, vec![7; BROWSER_HOST_MAX_PAGE_BODY_BYTES + 1]).unwrap();
        let encoded_page = crate::abi::encode_abi_message(&AbiMessage::Page(oversized_page));
        assert_eq!(encoded_page.len(), BROWSER_HOST_MAX_ENCODED_EVENT_BYTES + 1);
        seam::enqueue(encoded_page);
        assert_eq!(port.poll(AbiWorkBudget::credits(1)), Err(AbiErrorCode::LimitExceeded));
        assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES], 0, 1, false));

        for (action, expected, closed) in [(AfterProbe::Cancel, AbiPortPoll::Pending, false), (AfterProbe::Close, AbiPortPoll::Closed, true)] {
            seam::reset();
            seam::enqueue(crate::abi::encode_abi_message(&raw_browser_event(BROWSER_HOST_MAX_EVENT_BODY_BYTES)));
            seam::after_probe(action);
            assert_eq!(port.poll(AbiWorkBudget::credits(1)).unwrap(), expected);
            assert_eq!(seam::census(), (vec![BROWSER_HOST_INITIAL_POLL_BYTES, BROWSER_HOST_MAX_ENCODED_EVENT_BYTES], 0, 0, closed));
        }
    }

    #[test]
    fn browser_exact_event_and_page_are_credited_and_acknowledged_once() {
        let (mut host, port) = browser_fixture();
        attach_browser(&mut host, &port);
        let text_length = BROWSER_HOST_MAX_EVENT_BODY_BYTES - 15;
        let mut text = (text_length as u16).to_le_bytes().to_vec();
        text.extend(std::iter::repeat(b'x').take(text_length));
        let event = browser_event(crate::event::BROWSER_EVENT_TEXT, 1, text, 1);
        let event_bytes = match &event {
            AbiMessage::Event(event) => event.bytes.len(),
            _ => unreachable!(),
        };
        assert_eq!(event_bytes, BROWSER_HOST_MAX_EVENT_BODY_BYTES);
        port.0.borrow_mut().incoming.push_back(event);
        inspect_and_classify(&mut host, event_bytes);
        assert_eq!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Event);
        host.step(AbiWorkBudget::credits(1)).unwrap();
        let event_acks = port.0.borrow().sent.iter().filter(|message| matches!(message, AbiMessage::Reply(reply) if reply.request_id == AbiRequestId(101))).count();
        assert_eq!(event_acks, 1);

        let handle = crate::abi::AbiHandle::try_new(9, 1).unwrap();
        let page = AbiPage::try_new(handle, 0, vec![5; BROWSER_HOST_MAX_PAGE_BODY_BYTES]).unwrap();
        port.0.borrow_mut().incoming.push_back(AbiMessage::Page(page));
        assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
        for _ in 0..BROWSER_HOST_MAX_PAGE_BODY_BYTES {
            assert!(matches!(host.step(AbiWorkBudget::credits(1)).unwrap(), BrowserHostStep::Progress(_)));
        }
        host.step(AbiWorkBudget::credits(1)).unwrap();
        let page_acks = port.0.borrow().sent.iter().filter(|message| matches!(message, AbiMessage::Control(AbiControl::Acknowledge { handle: actual, index: 0 }) if *actual == handle)).count();
        assert_eq!(page_acks, 1);
    }

    //#endregion 🌐️Browser host laws
}

//#endregion Tests
