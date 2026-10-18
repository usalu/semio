//! 🪟️ Window/canvas hosting, the scheduler's wake transport, cursor application, clipboard,
//! and IME plumbing — the crate's other half of the U1 sync/async boundary that `🦀️.rs` and
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

use semio_framework_abi::{AbiBytes, AbiControl, AbiErrorCode, AbiEvent, AbiMessage, AbiOperation, AbiPage, AbiPort, AbiPortPoll, AbiReply, AbiReplyLedger, AbiRequest, AbiRequestId, AbiStatusCode, AbiWorkBudget};
use ui_render::{CursorRequest, DispatchEvent, FrameScheduler, ImeDirective, InvalidationReason, PhysicalSize};

//#region 🔖️Host

#[path = "🖱️metrics-cursor-and-clipboard/🦀️.rs"]
mod metrics_cursor_and_clipboard;
pub use metrics_cursor_and_clipboard::{
    cursor_css_for, should_request_redraw, BrowserClipboard, ClipboardHost, WindowMetrics,
};
#[cfg(not(target_arch="wasm32"))]
pub use metrics_cursor_and_clipboard::{
    apply_window_cursor, cursor_icon_for, NativeClipboardJob, NativeClipboardOperation,
};

#[path = "📁️file-dialog-ime-and-window-delegate/🦀️.rs"]
mod file_dialog_ime_and_window_delegate;
#[cfg(not(target_arch="wasm32"))]
pub use file_dialog_ime_and_window_delegate::{
    apply_ime_directive, select_native_paths, NativeFileDialogRequest,
};
pub use file_dialog_ime_and_window_delegate::{RedrawOutcome, WindowDelegate};

//#region 📁️FileDialog

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../🧪️tests/🔬️window-native-file-dialog/🦀️.rs"]
mod native_file_dialog_tests;

//#endregion 📁️FileDialog

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
                    Some(DispatchEvent::PointerMove { pointer, x: self.last_pointer_pos.0, y: self.last_pointer_pos.1, modifiers: self.modifiers })
                }
                E::MouseInput { device_id, state, button } => {
                    let pointer = event::pointer_info_for_mouse(&mut self.pointers, *device_id);
                    let button = event::pointer_button_from_winit(*button)?;
                    let (x, y) = self.last_pointer_pos;
                    Some(match state {
                        ElementState::Pressed => DispatchEvent::PointerDown { pointer, x, y, button, modifiers: self.modifiers },
                        ElementState::Released => DispatchEvent::PointerUp { pointer, x, y, button, modifiers: self.modifiers },
                    })
                }
                E::MouseWheel { delta, .. } => {
                    let (delta_x, delta_y) = event::normalize_wheel_delta_native(*delta);
                    let (x, y) = self.last_pointer_pos;
                    Some(DispatchEvent::Scroll { x, y, delta_x, delta_y, modifiers: self.modifiers })
                }
                E::Touch(touch) => {
                    let pointer = event::pointer_info_for_touch(&mut self.pointers, touch);
                    let x = touch.location.x as f32;
                    let y = touch.location.y as f32;
                    Some(match touch.phase {
                        TouchPhase::Started => DispatchEvent::PointerDown { pointer, x, y, button: ui_render::PointerButton::Primary, modifiers: self.modifiers },
                        TouchPhase::Moved => DispatchEvent::PointerMove { pointer, x, y, modifiers: self.modifiers },
                        TouchPhase::Ended => DispatchEvent::PointerUp { pointer, x, y, button: ui_render::PointerButton::Primary, modifiers: self.modifiers },
                        TouchPhase::Cancelled => DispatchEvent::PointerCancel { pointer },
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
                    let scale_factor = self.window.as_ref().map_or(1.0, |window| window.scale_factor());
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

#[path = "🌐️browser-host/🦀️.rs"]
mod browser;

pub use browser::*;

//#endregion 🌐️Browser

//#endregion 🔖️Host
