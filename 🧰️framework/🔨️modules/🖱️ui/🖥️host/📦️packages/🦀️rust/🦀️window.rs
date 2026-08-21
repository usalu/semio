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
//! 2. On wasm32, [`ClipboardHost`]'s browser clipboard read is genuinely `Promise`-based
//!    (`BrowserClipboard::read_text_async`) — the one real `async fn` in this crate, driven via
//!    `wasm_bindgen_futures::spawn_local` by whatever owns the canvas, never via `block_on` on the
//!    render/event thread (U-program R4 forbids that outright and this crate does not want a bridge
//!    here anyway).
//!
//! **Never `ControlFlow::Poll`, never an unconditional `request_redraw`.** [`should_request_redraw`]
//! is the single decision point both [`NativeHost`] and the browser [`CanvasHost`] funnel through —
//! it is [`ui_render::FrameScheduler::should_render`] under a name that reads at the call site; a
//! clean window drains no dirty mask and neither host asks for a frame.

use ui_render::{CursorRequest, DispatchEvent, FrameScheduler, ImeDirective, InvalidationReason, PhysicalSize};

//#region 🔖️Host

//#region 📐️WindowMetrics

/// 📐️ Physical size plus the scale factor that relates it to logical pixels — the pair every
/// scale-factor-change or resize needs to hand a delegate in one shot, platform-neutral so it is
/// testable with no `winit`/`web_sys` in scope at all.
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
/// [`CanvasHost`]'s `requestAnimationFrame` callback funnel through one obviously-shared decision
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

/// 🌐️ Ported from `wgpu-old`'s `🦀️cursor.rs::apply_canvas_cursor`, same dedup rule.
#[cfg(target_arch = "wasm32")]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn apply_canvas_cursor(canvas: &web_sys::HtmlCanvasElement, cursor: CursorRequest, last: &mut Option<CursorRequest>) {
    use wasm_bindgen::JsCast;
    if *last == Some(cursor) {
        return;
    }
    *last = Some(cursor);
    if let Some(element) = canvas.dyn_ref::<web_sys::HtmlElement>() {
        let _ = element.style().set_property("cursor", cursor_css_for(cursor));
    }
}

//#endregion 🖱️Cursor

//#region 📋️Clipboard

/// 📋️ Wraps `arboard`/the browser Clipboard API behind an interface so neither leaks past this crate
/// (repo convention: external libraries stay behind an interface — see root `CLAUDE.md`).
pub trait ClipboardHost {
    fn write_text(&mut self, text: &str);
    fn read_text(&mut self) -> Option<String>;
}

/// 📋️ Ported from `host.rs::clipboard_write_text`/`clipboard_read_text`'s native halves, made sync —
/// `arboard::Clipboard::get_text`/`set_text` were already synchronous; only the old fn signatures were
/// (needlessly) `async`.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeClipboard {
    inner: Option<arboard::Clipboard>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeClipboard {
    /// 📋️ `Clipboard::new()`'s `Err` (no display/clipboard backend, e.g. headless CI) is swallowed
    /// here rather than propagated — same rationale `host.rs` documented: there is no sensible way for
    /// a UI copy gesture to surface a clipboard backend failure back through this call chain.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self { inner: arboard::Clipboard::new().ok() }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for NativeClipboard {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ClipboardHost for NativeClipboard {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn write_text(&mut self, text: &str) {
        if let Some(clipboard) = self.inner.as_mut() {
            let _ = clipboard.set_text(text.to_string());
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_text(&mut self) -> Option<String> {
        self.inner.as_mut()?.get_text().ok()
    }
}

/// 🌐️ The browser Clipboard API is entirely `Promise`-based — `write_text` fires the write without
/// awaiting it (the `Promise` already starts executing the instant it's created, exactly like a
/// browser's own Ctrl+C never blocking the page), and `read_text` always returns `None` since a
/// synchronous read has no browser equivalent; a caller needing a paste result uses
/// [`BrowserClipboard::read_text_async`] instead.
#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct BrowserClipboard;

#[cfg(target_arch = "wasm32")]
impl ClipboardHost for BrowserClipboard {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn write_text(&mut self, text: &str) {
        if let Some(window) = web_sys::window() {
            let _ = window.navigator().clipboard().write_text(text);
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_text(&mut self) -> Option<String> {
        None
    }
}

#[cfg(target_arch = "wasm32")]
impl BrowserClipboard {
    /// 📋️ The one sanctioned `async fn` outside the event loop itself (see this file's own
    /// docstring) — a caller drives this via `wasm_bindgen_futures::spawn_local`, then feeds the
    /// result back in as a normal [`DispatchEvent::Paste`].
    pub async fn read_text_async() -> Option<String> {
        let promise = web_sys::window()?.navigator().clipboard().read_text();
        wasm_bindgen_futures::JsFuture::from(promise).await.ok()?.as_string()
    }
}

//#endregion 📋️Clipboard

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
/// Nothing in this trait's signature ever names a `winit` or `web_sys` type.
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
            Self { window: None, pointers: PointerRegistry::new(), modifiers: ui_render::EventModifiers::default(), last_pointer_pos: (0.0, 0.0), last_cursor: None, clock: MonotonicClock::new(), pending_reason: None, delegate, _ui_token: crate::enqueue::UiThreadToken::mint() }
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

#[cfg(target_arch = "wasm32")]
mod browser {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    //#region 🕰️BrowserClock

    /// 🕰️ `performance.now()`-backed clock — `std::time::Instant` is unavailable on
    /// `wasm32-unknown-unknown` (no monotonic clock without JS glue), so this crate's browser half
    /// needs its own. ⚠️ Needs the `"Performance"` `web-sys` feature — not yet in this crate's
    /// registrar-owned `Cargo.toml`; see this packet's report registrar-requests.
    pub struct BrowserClock {
        origin_ms: f64,
    }

    impl BrowserClock {
        pub fn new() -> Self {
            Self { origin_ms: performance_now() }
        }

        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn now_seconds(&self) -> f64 {
            (performance_now() - self.origin_ms) / 1000.0
        }
    }

    impl Default for BrowserClock {
        fn default() -> Self {
            Self::new()
        }
    }

    fn performance_now() -> f64 {
        web_sys::window().and_then(|window| window.performance()).map(|performance| performance.now()).unwrap_or(0.0)
    }

    //#endregion 🕰️BrowserClock

    //#region 🌐️CanvasHost

    struct CanvasHostState<D: WindowDelegate> {
        canvas: web_sys::HtmlCanvasElement,
        clock: BrowserClock,
        raf_pending: bool,
        document_hidden: bool,
        last_cursor: Option<CursorRequest>,
        /// 🔁️ Lives here rather than on [`CanvasHost`] itself so [`on_animation_frame`]'s own re-arm
        /// path (which only ever sees this `Rc<RefCell<..>>`, never the outer struct) can re-request a
        /// frame through the exact same [`request_wake_from_state`] the public API uses — one dedup
        /// path, not two.
        raf_closure: Option<Closure<dyn FnMut(f64)>>,
        delegate: D,
        /// 🎫️ Same rationale as `NativeHost`'s own field — minted once, here, at construction.
        _ui_token: crate::enqueue::UiThreadToken,
    }

    /// 🌐️ The canvas equivalent of [`super::NativeHost`] (see this crate's `native` module —
    /// unavailable on wasm32): `requestAnimationFrame` requested **only** when dirty or a deadline is
    /// due, `ResizeObserver` for size + `devicePixelRatio` for scale, and `visibilitychange`
    /// suspending [`FrameScheduler`]'s visibility flag (deadlines still accumulate while hidden — see
    /// `🦀️schedule.rs`'s own `should_render` docstring — only actual rendering is suspended).
    ///
    /// **Duplicate-`rAF`-request avoidance.** [`Self::request_wake`] is the *only* call site that ever
    /// invokes `requestAnimationFrame`, and it is guarded by `raf_pending`: every invalidation
    /// (an input event, a resize, a delegate-side `scheduler.invalidate`) calls it, but a second call
    /// while a frame is already in flight is a no-op — one pending `rAF` absorbs any number of
    /// invalidations that arrive before it fires, exactly mirroring
    /// [`FrameScheduler::should_render`]'s own N-invalidations-coalesce-into-one-frame contract. The
    /// callback itself clears `raf_pending` *before* doing anything else, so a fresh invalidation that
    /// arrives during the callback's own `redraw` call is free to schedule the *next* frame rather than
    /// being silently dropped.
    pub struct CanvasHost<D: WindowDelegate + 'static> {
        state: Rc<RefCell<CanvasHostState<D>>>,
        _resize_observer: web_sys::ResizeObserver,
        _resize_closure: Closure<dyn FnMut(js_sys::Array)>,
        _visibility_closure: Closure<dyn FnMut()>,
    }

    impl<D: WindowDelegate + 'static> CanvasHost<D> {
        pub fn new(canvas: web_sys::HtmlCanvasElement, delegate: D) -> Self {
            let state = Rc::new(RefCell::new(CanvasHostState { canvas: canvas.clone(), clock: BrowserClock::new(), raf_pending: false, document_hidden: false, last_cursor: None, raf_closure: None, delegate, _ui_token: crate::enqueue::UiThreadToken::mint() }));

            let raf_state = state.clone();
            let raf_closure = Closure::wrap(Box::new(move |timestamp_ms: f64| on_animation_frame(&raf_state, timestamp_ms)) as Box<dyn FnMut(f64)>);
            state.borrow_mut().raf_closure = Some(raf_closure);

            let resize_state = state.clone();
            let resize_closure = Closure::wrap(Box::new(move |_entries: js_sys::Array| on_resize(&resize_state)) as Box<dyn FnMut(js_sys::Array)>);
            let resize_observer = web_sys::ResizeObserver::new(resize_closure.as_ref().unchecked_ref()).expect("ResizeObserver::new");
            resize_observer.observe(&canvas);

            let visibility_state = state.clone();
            let visibility_closure = Closure::wrap(Box::new(move || on_visibility_change(&visibility_state)) as Box<dyn FnMut()>);
            if let Some(document) = web_sys::window().and_then(|window| window.document()) {
                let _ = document.add_event_listener_with_callback("visibilitychange", visibility_closure.as_ref().unchecked_ref());
            }

            let host = Self { state: state.clone(), _resize_observer: resize_observer, _resize_closure: resize_closure, _visibility_closure: visibility_closure };
            request_wake_from_state(&state);
            host
        }

        /// 🎯️ Call after handing the delegate any normalized event that might have invalidated it —
        /// see this type's own docstring for the dedup contract.
        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        pub fn request_wake(&self) {
            request_wake_from_state(&self.state);
        }
    }

    /// 🎯️ The one call site that ever invokes `requestAnimationFrame` — see [`CanvasHost`]'s own
    /// docstring for the full dedup contract. Both [`CanvasHost::request_wake`] (invalidation-driven)
    /// and [`on_animation_frame`]'s own re-arm path (deadline-driven) funnel through here.
    fn request_wake_from_state<D: WindowDelegate>(state: &Rc<RefCell<CanvasHostState<D>>>) {
        let mut host = state.borrow_mut();
        if host.raf_pending || host.document_hidden {
            return;
        }
        let Some(window) = web_sys::window() else { return };
        let Some(closure) = host.raf_closure.as_ref() else { return };
        if window.request_animation_frame(closure.as_ref().unchecked_ref()).is_ok() {
            host.raf_pending = true;
        }
    }

    /// 🈶️ IME positioning (unlike [`apply_ime_directive`]) has no browser call here at all —
    /// `winit::Window::set_ime_cursor_area`'s own docs mark Web unsupported, and the browser's native
    /// IME already follows the focused `contenteditable`/`<input>` element with no host-level call
    /// needed, so a `RedrawOutcome::ime` directive is simply not applied on this platform.
    ///
    /// 🕰️ Re-arming for a future deadline goes through the same [`request_wake_from_state`] path as an
    /// invalidation — a scaffold-quality wait, not a true zero-cost sleep: a pending deadline costs one
    /// no-op `rAF` callback per display refresh until it is due (no pixels repaint on those ticks, the
    /// `should_request_redraw` gate below still applies every time). A `setTimeout`-based long-wait
    /// path is a fair follow-up; see this packet's report deviations.
    fn on_animation_frame<D: WindowDelegate>(state: &Rc<RefCell<CanvasHostState<D>>>, _timestamp_ms: f64) {
        let has_pending_deadline = {
            let mut host = state.borrow_mut();
            host.raf_pending = false;
            let now = host.clock.now_seconds();
            if let Some(reason) = should_request_redraw(host.delegate.scheduler_mut(), now) {
                let outcome = host.delegate.redraw(reason);
                let canvas = host.canvas.clone();
                apply_canvas_cursor(&canvas, outcome.cursor, &mut host.last_cursor);
            }
            host.delegate.scheduler_mut().next_deadline().is_some()
        };
        if has_pending_deadline {
            request_wake_from_state(state);
        }
    }

    fn on_resize<D: WindowDelegate>(state: &Rc<RefCell<CanvasHostState<D>>>) {
        {
            let mut host = state.borrow_mut();
            let canvas = host.canvas.clone();
            let dpr = web_sys::window().map(|window| window.device_pixel_ratio()).unwrap_or(1.0);
            let width = (canvas.client_width().max(0) as f64 * dpr) as u32;
            let height = (canvas.client_height().max(0) as f64 * dpr) as u32;
            canvas.set_width(width);
            canvas.set_height(height);
            host.delegate.handle_metrics(WindowMetrics { physical: PhysicalSize::new(width, height), scale_factor: dpr as f32 });
        }
        request_wake_from_state(state);
    }

    /// 👁️ Whatever accumulated in the dirty mask while hidden (deadlines still fire per
    /// [`FrameScheduler::should_render`]'s own docstring) only becomes visible to `should_render` once
    /// `visible` flips back — so becoming visible again requests the wake that period was owed.
    fn on_visibility_change<D: WindowDelegate>(state: &Rc<RefCell<CanvasHostState<D>>>) {
        let hidden = web_sys::window().and_then(|window| window.document()).map(|document| document.hidden()).unwrap_or(false);
        {
            let mut host = state.borrow_mut();
            host.document_hidden = hidden;
            host.delegate.scheduler_mut().set_visible(!hidden);
        }
        if !hidden {
            request_wake_from_state(state);
        }
    }

    //#endregion 🌐️CanvasHost
}

#[cfg(target_arch = "wasm32")]
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
}

//#endregion Tests
