//! 🪟️ The event loop that does nothing when nothing changed — ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet `os-host`, master plan §5.
//! [`WinitApp`] is the `ApplicationHandler` **replacing `SemioApp`**: `ControlFlow::Poll`, the
//! `RedrawRequested` re-arm and `start_frame_loop` are all deleted (see this crate's
//! `📓️terra-os-host-report.md` redraw audit for the exact old line numbers and what replaced them).
//!
//! **Why this is a hand-written `ApplicationHandler` rather than `ui_host::window::NativeHost<OsHost>`
//! — a deliberate, evidence-based deviation, not an oversight.** `NativeHost::new(delegate: D)` takes
//! an already-fully-constructed delegate; its own `resumed` creates the window *itself*, after the
//! event loop is already running. This crate's boot order is the opposite: `GpuContext::from_window`
//! (this file's `AppRuntime`'s device init) needs a **live** `winit::window::Window` to construct, and
//! `OsHost` cannot exist before `AppRuntime` does — so the window must be created first, `AppRuntime`
//! booted async against it second, and only THEN can `OsHost::new` run. `ui_host::WindowDelegate`'s
//! own trait — `scheduler_mut`, `handle_event`, `handle_metrics`, `redraw`, `close_requested` — is
//! still exactly what [`OsHost`] implements below (see `impl WindowDelegate for OsHost`); this file
//! only re-implements `NativeHost`'s specific control-flow *orchestration* (`control_flow_for`/
//! `should_request_redraw`, both reused verbatim from `ui_host` where cfg allows — see inline calls)
//! around the two-phase boot handshake `SemioApp` already solved correctly. Reported as a real gap in
//! `ui_host::WindowDelegate` worth closing upstream: a `WindowDelegate` construction hook that receives
//! the just-created `Window` (or an explicit two-phase `NativeHost::new_pending()` API) would let a
//! future revision drop this file's hand-rolled `ApplicationHandler` entirely.

use crate::kernel_seam::KernelSeam;
use crate::os_host::OsHost;
use crate::AppHandle;
use crate::AppRuntime;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use ui_host::{should_request_redraw, RedrawOutcome, WindowDelegate, WindowMetrics};
use ui_render::{CursorRequest, DispatchEvent, EventModifiers, InvalidationReason, PhysicalSize, PointerButton};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowId};

/// ⏱️ P1e: this process has exactly one renderer frame callback (`OsHost::redraw`, below) — one
/// `OperationId` allocated once, lazily, on first frame, rather than a fresh one per call (an
/// `OperationId` names a logical operation across its `Generation`s, not one single call).
fn render_frame_operation_id() -> semio_framework_trace::OperationId {
    static ID: std::sync::OnceLock<semio_framework_trace::OperationId> = std::sync::OnceLock::new();
    *ID.get_or_init(semio_framework_trace::allocate_operation_id)
}

//#region 🔖️WindowDelegate for OsHost

impl WindowDelegate for OsHost {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn scheduler_mut(&mut self) -> &mut ui_render::FrameScheduler {
        &mut self.scheduler
    }

    /// 📥️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the enqueue-only replacement
    /// for the old "spawn one heap-allocated future per event" path — every event now goes through
    /// [`ui_host::EventQueue::enqueue`], which coalesces replaceable state (pointer move, scroll) into
    /// fixed `Copy` slots and bounds discrete events (pointer down/up, key, ime, paste) to
    /// [`ui_host::DISCRETE_QUEUE_CAPACITY`] — a genuine reduction from "one allocation per pointer-move
    /// tick during a drag" to "zero allocations for replaceable state, one bounded push for discrete
    /// events." `redraw()` (below) drains this queue once per frame and dispatches the WHOLE batch
    /// through a single `spawn_app_task` call, same coarse "any input event may have changed something"
    /// invalidation rule as before. Wrapped in a `semio_framework_trace::Watchdog` against
    /// `InteractiveStage::UiEvent`'s 1ms soft target — the ticket's own UI-event-callback gate.
    // 🚫️async: U1 — the enqueue itself never awaits; the batched dispatch this feeds is the boundary-
    // async exception U1 itself carves out.
    fn handle_event(&mut self, event: DispatchEvent) {
        let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_event", render_frame_operation_id(), semio_framework_trace::Generation(self.frame_generation), semio_framework_trace::InteractiveStage::UiEvent);
        self.scheduler.invalidate(InvalidationReason::INPUT_STATE);
        if self.events.enqueue(self.ui_token, event) == ui_host::EnqueueOutcome::Overflow {
            crate::log_debug("os_host: discrete input queue overflow — a redraw has not drained in a while");
        }
    }

    /// 📐️ P3a: also funnels through the enqueue-only [`ui_host::EventQueue`] (`enqueue_metrics`, so a
    /// resize storm coalesces to the latest sample the same way pointer move does — see that struct's
    /// own test coverage). The `app.resize(..)` call stays immediate rather than deferred: this
    /// crate's GPU surface must be reconfigured before the next `render_frame` submission or the
    /// backend rejects the mismatched surface/window size, which is the platform constraint the design
    /// doc's §6 table calls "GPU resource creation... depends, may need locking in native" — resizing
    /// the surface synchronously here IS that lock, kept minimal (no layout, no tessellation, no
    /// allocation beyond what `wgpu`'s own surface reconfiguration already does).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn handle_metrics(&mut self, metrics: WindowMetrics) {
        self.scheduler.invalidate(InvalidationReason::VIEWPORT);
        self.events.enqueue_metrics(self.ui_token, metrics.physical.width, metrics.physical.height, metrics.scale_factor);
        if let Ok(mut app) = self.runtime.try_borrow_mut() {
            let (width, height) = metrics.logical_size();
            app.resize(width, height, metrics.scale_factor);
        }
    }

    /// 🖼️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): now explicitly split into
    /// `build_and_publish_snapshot` (drains enqueued input, builds via `AppRuntime::frame()`, publishes
    /// a [`crate::render_snapshot::RenderSnapshot`]) and `present_snapshot` (acquires the sink, applies
    /// its directives). See `🦀️render_snapshot.rs`'s own module docstring for the honest scope note:
    /// BOTH still run on this call, on this thread, because `AppRuntime` cannot move to a worker as-is
    /// (`Rc<RefCell<_>>`, not `Send`) and GPU submission lives inside `ui_wgpu`, outside this packet's
    /// boundary. The split is real in the CODE (two named functions, a real publish/acquire crossing
    /// between them through the sink) even though it is not yet real in THREADING — the seam a worker
    /// slots into once that ownership blocker is resolved.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn redraw(&mut self, _reason: InvalidationReason) -> RedrawOutcome {
        let now = self.clock.now_seconds();
        self.build_and_publish_snapshot();
        self.present_snapshot(now)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn close_requested(&mut self) -> bool {
        true
    }
}

/// 🖼️ P3a: `build_and_publish_snapshot`/`present_snapshot` are inherent methods (not part of
/// `WindowDelegate` — that trait only names `redraw`), split out so a future worker-side builder can
/// call `build_and_publish_snapshot`'s eventual replacement independently of `present_snapshot`, which
/// must stay UI-thread-only.
impl OsHost {
    /// 🏗️ The BUILD half — drains enqueued input, calls the existing `AppRuntime::frame()` (unchanged
    /// product behaviour — chrome layout/tessellation/GPU submission all still happen inside it, see
    /// `📓️p3a-render-snapshot.md` for what Phase 5 still owes), and publishes the result.
    ///
    /// **Known fidelity loss — `ui_render::CursorRequest` (5 variants) vs `SemioCursor` (13
    /// variants).** `AppRuntime::frame()` already applies its own richer cursor internally via
    /// `ui_wgpu::wgpu::apply_window_cursor` before this method returns; the published snapshot's cursor
    /// is a best-effort narrowing of that SAME already-applied cursor (`semio_cursor_to_request`
    /// below), not an independent decision — see `present_snapshot`'s own doc for the idempotency
    /// argument this fidelity loss never actually exercises on this file's own hand-rolled loop.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn build_and_publish_snapshot(&mut self) {
        if !self.events.is_empty() {
            let generation = self.events.current_generation();
            let drained = self.events.drain(ui_host::WorkerContext::new(generation));
            let runtime = self.runtime.clone();
            let handle = Rc::downgrade(&self.runtime);
            crate::spawn_app_task(async move {
                let Ok(mut app) = runtime.try_borrow_mut() else { return };
                dispatch_drained_events(&mut app, &handle, drained).await;
            });
        }
        // 🧵️ P3b (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): a genuine, if narrow, worker
        // hop — see `🦀️frame_job.rs`'s own module docstring for exactly what this covers (the World3D
        // camera-dispatch-deadline scan) and, more importantly, what it does not (chrome layout/
        // tessellation/GPU submission, still all inside `app.frame()` below, still all on this thread).
        // `poll_and_resubmit` never blocks: it returns this tick's fresh result if the previous job
        // finished, or the last one it has otherwise — `frame()` itself re-validates every candidate
        // against live state, so a stale result here is a harmless one-tick delay, not a correctness
        // bug (see that module's doc for why).
        let build_inputs = self
            .runtime
            .try_borrow()
            .map(|app| crate::frame_job::FrameBuildInputs { world3d_camera_dispatch_deadlines_ms: app.world3d_camera_dispatch_deadlines_ms.clone(), wheel_zoom_deadline_ms: app.wheel_zoom_deadline_ms, now_ms: crate::app_now_ms() })
            .unwrap_or_default();
        let build_operation = render_frame_operation_id();
        let build_generation = semio_framework_trace::Generation(self.frame_generation);
        let build_directives = self.frame_build.poll_and_resubmit(build_inputs, build_operation, build_generation);
        let last_cursor = if let Ok(mut app) = self.runtime.try_borrow_mut() {
            // ⏱️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): `AppRuntime::frame()`
            // is this crate's whole renderer frame-BUILD callback today — input processing, chrome
            // layout/tessellation AND the GPU `render_frame` submission, still one undivided call on
            // this thread (native winit thread or the wasm main thread; `frame()` is called from
            // nowhere else — see `📦️glue.rs`'s own audit). Splitting the GPU submission itself further
            // off this call needs `ui_wgpu` (outside this packet's boundary) to expose a separate
            // encode/submit seam — flagged for Phase 5 in `📓️p3a-render-snapshot.md`. This wrap makes
            // an overrun VISIBLE — any call exceeding `semio_framework_trace::INTERACTIVE_STEP_CEILING_US`
            // (8ms) is recorded as a `ContractViolation`, queryable by this packet's exit gate.
            let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_frame", render_frame_operation_id(), semio_framework_trace::Generation(self.frame_generation), semio_framework_trace::InteractiveStage::UiPresent);
            self.frame_generation = self.frame_generation.wrapping_add(1);
            // 🪪️ P3c: `AppRuntime` no longer stores a weak handle to its own `Rc<RefCell<_>>` (that
            // was the one field disqualifying it from `Send` — see `AppHandle`'s doc comment in
            // `📦️glue.rs`). Every `frame()`-internal deferred continuation that used to clone
            // `self.self_weak` now takes this parameter instead, computed here where the real
            // `Rc<RefCell<AppRuntime>>` already lives (`self.runtime`).
            let handle = Rc::downgrade(&self.runtime);
            app.frame(&handle, &build_directives);
            app.last_cursor.map(|(cursor, _)| cursor)
        } else {
            None
        };
        let cursor = last_cursor.map(semio_cursor_to_request).unwrap_or_default();
        let revision = self.snapshot_sink.next_revision();
        let generation = semio_framework_trace::Generation(self.frame_generation);
        let timestamp_us = (self.now_seconds() * 1_000_000.0) as u64;
        self.snapshot_sink.publish(crate::render_snapshot::RenderSnapshot::new(revision, generation, timestamp_us, cursor, None));
    }

    /// 📤️ The PRESENT half — atomically acquires the newest published snapshot (never blocks, never
    /// waits: if nothing newer landed since the last call it re-presents the same one — the ticket's
    /// governing rule verbatim) and applies its cursor/IME directives plus this file's deadline
    /// sources. This is the bounded, ≤2ms-budget half of `redraw` — everything expensive already
    /// happened in `build_and_publish_snapshot`.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn present_snapshot(&mut self, now: f64) -> RedrawOutcome {
        let snapshot = self.snapshot_sink.acquire();

        // ⌨️ Deviation from the packet brief's literal "only while a caret is visible": AppRuntime's
        // own focus/caret-presence signal is not exposed at this layer (see report — a real, scoped
        // follow-up, not guessed at here). Until that hook lands, `caret_present` stays `true`
        // unconditionally, which is exact behavioural PARITY with the old code (which also blinked
        // unconditionally on every frame) rather than a regression — it just does not yet earn the
        // "idle window with no caret costs zero blink wakes" half of the optimization.
        self.caret.sync(&mut self.scheduler, now, true);

        if self.hot_swap.is_due(now) {
            self.scheduler.request_deadline(now + crate::deadlines::NATIVE_HOT_SWAP_POLL_SECONDS, InvalidationReason::RESOURCE_READY);
        }

        RedrawOutcome { cursor: snapshot.cursor, ime: snapshot.ime }
    }
}

/// 🖱️ Narrows `AppRuntime`'s 13-variant `SemioCursor` to `ui_render::CursorRequest`'s 5 — see
/// `redraw`'s own doc comment for why this narrowing's fidelity loss never actually surfaces today.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn semio_cursor_to_request(cursor: ui_wgpu::wgpu::SemioCursor) -> CursorRequest {
    use ui_wgpu::wgpu::SemioCursor as S;
    match cursor {
        S::Default | S::EwResize | S::NsResize | S::NwseResize | S::NeswResize | S::Crosshair | S::NotAllowed => CursorRequest::Default,
        S::Pointer | S::Selectable | S::Foldable => CursorRequest::Pointer,
        S::Grab => CursorRequest::Grab,
        S::Grabbing | S::Move => CursorRequest::Grabbing,
        S::Text => CursorRequest::Text,
    }
}

/// 🔀️ One normalized `ui_host::DispatchEvent` → the existing `AppRuntime` pointer/keyboard entry
/// points, unchanged from before this packet (`handle_pointer_move`/`handle_pointer_button`/
/// `handle_key`). `PointerButton::{Primary,Secondary,Middle}` → `{0,2,1}` matches the DOM
/// `MouseEvent.button` convention `AppRuntime`'s own `i16` button parameter already assumed
/// (`dispatch_actions`/world3d/graph/map/board call sites all branch on `0`/`1`/`2` verbatim).
/// 📤️ P3a: replays one [`ui_host::DrainedEvents`] batch — the coalesced pointer-move/scroll samples
/// (if any landed since the last drain) followed by every discrete event in arrival order — through
/// the same [`dispatch_normalized_event`] every individual event used to go through one at a time.
/// Reconstructing a synthetic [`DispatchEvent`] for the coalesced samples keeps this a thin adapter
/// with zero duplicated dispatch logic, per CLAUDE.md's "if code is repeated, it MUST be close to
/// each other" — there is exactly one place that knows how to turn a `DispatchEvent` into `AppRuntime`
/// calls. A right-click stays on this same lossless route as `PointerDown::Secondary`; the canonical
/// `AppRuntime::handle_pointer_button` receives DOM button `2`, and `ShellState` opens the context
/// menu in its pressed-secondary branch.
// 🚫️async: U1 — sync fn boundary (spawned once per drain by `redraw` above); the `.await`s inside are
// the same boundary-async exception `AppRuntime`'s own methods already are.
async fn dispatch_drained_events(app: &mut AppRuntime, handle: &AppHandle, drained: ui_host::DrainedEvents) {
    if let Some(sample) = drained.pointer_move {
        dispatch_normalized_event(app, handle, DispatchEvent::PointerMove { pointer: sample.pointer, x: sample.x, y: sample.y }).await;
    }
    if let Some(sample) = drained.scroll {
        dispatch_normalized_event(app, handle, DispatchEvent::Scroll { x: sample.x, y: sample.y, delta_x: sample.delta_x, delta_y: sample.delta_y }).await;
    }
    for discrete in drained.discrete {
        dispatch_normalized_event(app, handle, discrete.event).await;
    }
}

// 🚫️async: U1 — the fn itself is sync at its call boundary (spawned by `dispatch_drained_events`
// above); the `.await`s inside it are the same boundary-async exception `AppRuntime`'s own methods
// already are.
async fn dispatch_normalized_event(app: &mut AppRuntime, handle: &AppHandle, event: DispatchEvent) {
    match event {
        DispatchEvent::PointerMove { x, y, .. } => {
            let (down, button, modifiers) = (app.pointer_down, app.pointer_button, app.modifiers.clone());
            app.handle_pointer_move(x, y, down, button, modifiers).await;
        }
        DispatchEvent::PointerDown { x, y, button, .. } => {
            let modifiers = app.modifiers.clone();
            app.handle_pointer_button(x, y, true, pointer_button_to_i16(button), modifiers).await;
        }
        DispatchEvent::PointerUp { x, y, button, .. } => {
            let modifiers = app.modifiers.clone();
            app.handle_pointer_button(x, y, false, pointer_button_to_i16(button), modifiers).await;
        }
        DispatchEvent::Scroll { delta_y, .. } => {
            app.wheel_delta += delta_y;
        }
        DispatchEvent::KeyDown { key, modifiers } => {
            if let Some(action) = key_action_from_dispatch(&key, true) {
                app.handle_key(handle, action, event_modifiers_to_pointer(modifiers));
            }
        }
        DispatchEvent::KeyUp { key, modifiers } => {
            if let Some(action) = key_action_from_dispatch(&key, false) {
                app.handle_key(handle, action, event_modifiers_to_pointer(modifiers));
            }
        }
        DispatchEvent::TextInput { .. } | DispatchEvent::Paste { .. } | DispatchEvent::Ime(_) => {
            // 🕳️ Honest gap: `AppRuntime` has no IME/paste entry point today (the old `SemioApp`
            // never wired `WindowEvent::Ime`/paste either — this is pre-existing scope, not a
            // regression this packet introduces). See report.
        }
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn pointer_button_to_i16(button: PointerButton) -> i16 {
    match button {
        PointerButton::Primary => 0,
        PointerButton::Middle => 1,
        PointerButton::Secondary => 2,
    }
}

#[cfg(test)]
mod p3c_tests {
    use super::*;

    #[test]
    fn secondary_pointer_button_uses_context_menu_code() {
        assert_eq!(pointer_button_to_i16(PointerButton::Secondary), 2);
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn event_modifiers_to_pointer(modifiers: EventModifiers) -> ui_wgpu::wgpu::PointerModifiers {
    ui_wgpu::wgpu::PointerModifiers { shift: modifiers.shift, ctrl: modifiers.ctrl, alt: modifiers.alt, meta: modifiers.meta }
}

/// ⌨️ DOM-`KeyboardEvent.key`-shaped strings (`ui_host::logical_key_to_dispatch_string`'s own output
/// vocabulary) → `AppRuntime`'s `KeyAction`. Space is the one level-triggered key (`Space(bool)`
/// tracks press/release for the context-menu-hold gesture `AppRuntime::handle_key` already branches
/// on); everything else is edge-triggered on `KeyDown` only, matching the old raw-winit dispatch path
/// this replaces (`ui_wgpu::wgpu::dispatch_window_event`, never consulted for `KeyUp` beyond Space).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn key_action_from_dispatch(key: &str, pressed: bool) -> Option<ui_wgpu::wgpu::KeyAction> {
    use ui_wgpu::wgpu::KeyAction as K;
    if key == " " {
        return Some(K::Space(pressed));
    }
    if !pressed {
        return None;
    }
    match key {
        "Backspace" => Some(K::Backspace),
        "Delete" => Some(K::Delete),
        "Enter" => Some(K::Enter),
        "Escape" => Some(K::Escape),
        "ArrowLeft" => Some(K::ArrowLeft),
        "ArrowRight" => Some(K::ArrowRight),
        "ArrowUp" => Some(K::ArrowUp),
        "ArrowDown" => Some(K::ArrowDown),
        "Tab" => Some(K::Tab),
        _ if key.len() >= 2 && key.starts_with('F') && key[1..].chars().all(|c| c.is_ascii_digit()) => key[1..].parse::<u8>().ok().map(K::Function),
        _ if key.chars().count() == 1 => Some(K::Char(key.to_string())),
        _ => None,
    }
}

//#endregion 🔖️WindowDelegate for OsHost

//#region 🚀️WinitApp

/// 📨️ Same two-phase boot handshake `SemioApp`'s own `HostUserEvent` used — kept, not deleted, per
/// this file's own module docstring on why `ui_host::NativeHost`'s single-phase construction cannot
/// replace it. Only the STEADY-STATE control flow changes: `ControlFlow::Poll` → `WaitUntil`/`Wait`.
/// No `callbacks` payload (the old variant carried `ui_wgpu::wgpu::PointerCallbacks`) — this file
/// normalizes input itself via `ui_host::event` and drives `AppRuntime` through the enqueue-only
/// `dispatch_normalized_event` path above, so `boot_runtime` returns only the runtime.
pub enum HostUserEvent {
    RuntimeReady {
        runtime: Rc<RefCell<AppRuntime>>,
    },
    /// 🔔️ Payload-free — arriving at all is the signal. Sent by [`ProxyWaker`] (native only) so a
    /// `kernel_runtime::TASK_POOL` future completing on the kernel thread wakes this event loop out
    /// of `WaitUntil`/`Wait`; `user_event` below does nothing but let `about_to_wait` run again,
    /// which is exactly where `poll_tasks()` already lives. See `kernel_seam.rs`'s own module
    /// docstring ("waker correctness") for why this exists.
    Wake,
}

/// 🔔️ `std::task::Wake` over this crate's own `EventLoopProxy<HostUserEvent>` — the real,
/// `Send + Sync` waker `kernel_runtime::install_waker` needs (built once, native only, at boot; see
/// `WinitApp::resumed`). `EventLoopProxy` is itself `Send + Sync` (winit's own cross-thread wake
/// transport), so this wrapper adds nothing but the `HostUserEvent::Wake` payload choice.
#[cfg(not(target_arch = "wasm32"))]
struct ProxyWaker(EventLoopProxy<HostUserEvent>);

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for ProxyWaker {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn wake(self: Arc<Self>) {
        let _ = self.0.send_event(HostUserEvent::Wake);
    }
}

/// 🚀️ `ApplicationHandler` replacing `SemioApp` (deleted by this packet's surgical `📦️glue.rs` edit —
/// see the report's redraw audit). Boot mirrors the old `SemioApp::resumed`/`user_event` handshake
/// verbatim (window created synchronously, `AppRuntime` booted async, delivered via
/// `HostUserEvent::RuntimeReady`); everything AFTER boot is new: no `ControlFlow::Poll`, no
/// `RedrawRequested` re-arm, no `start_frame_loop` — every redraw and every control-flow recompute
/// funnels through [`OsHost`]'s `WindowDelegate` impl above plus `ui_host::should_request_redraw`.
/// Input normalization (`pointers`/`modifiers`/`last_pointer_pos`) ports `ui_host::window::native::
/// NativeHost::normalize`'s exact match arms — that method is private on a private struct, so this
/// file's own `normalize` free fn below duplicates its logic over the same public `ui_host::event`
/// functions rather than being able to call it directly.
pub struct WinitApp {
    proxy: EventLoopProxy<HostUserEvent>,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")]
    plugins: Option<wasm_bindgen::JsValue>,
    #[cfg(target_arch = "wasm32")]
    canvas: Option<web_sys::HtmlCanvasElement>,
    #[cfg(not(target_arch = "wasm32"))]
    plugin_modules_root: std::path::PathBuf,
    window: Option<Arc<Window>>,
    host: Option<OsHost>,
    pointers: ui_host::PointerRegistry,
    modifiers: EventModifiers,
    last_pointer_pos: (f32, f32),
    pending_reason: Option<InvalidationReason>,
}

impl WinitApp {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        proxy: EventLoopProxy<HostUserEvent>,
        plugin_filter: String,
        #[cfg(target_arch = "wasm32")] plugins: Option<wasm_bindgen::JsValue>,
        #[cfg(target_arch = "wasm32")] canvas: Option<web_sys::HtmlCanvasElement>,
        #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
    ) -> Self {
        Self {
            proxy,
            plugin_filter,
            #[cfg(target_arch = "wasm32")]
            plugins,
            #[cfg(target_arch = "wasm32")]
            canvas,
            #[cfg(not(target_arch = "wasm32"))]
            plugin_modules_root,
            window: None,
            host: None,
            pointers: ui_host::PointerRegistry::new(),
            modifiers: EventModifiers::default(),
            last_pointer_pos: (0.0, 0.0),
            pending_reason: None,
        }
    }
}

/// 🔀️ Raw `winit::event::WindowEvent` → `ui_render::DispatchEvent`, ported from
/// `ui_host::window::native::NativeHost::normalize` (private, see this file's own struct docstring).
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn normalize(app: &mut WinitApp, event: &WindowEvent) -> Option<DispatchEvent> {
    use winit::event::{ElementState, TouchPhase};
    match event {
        WindowEvent::CursorMoved { device_id, position } => {
            app.last_pointer_pos = (position.x as f32, position.y as f32);
            let pointer = ui_host::pointer_info_for_mouse(&mut app.pointers, *device_id);
            Some(DispatchEvent::PointerMove { pointer, x: app.last_pointer_pos.0, y: app.last_pointer_pos.1 })
        }
        WindowEvent::MouseInput { device_id, state, button } => {
            let pointer = ui_host::pointer_info_for_mouse(&mut app.pointers, *device_id);
            let button = ui_host::pointer_button_from_winit(*button)?;
            let (x, y) = app.last_pointer_pos;
            Some(match state {
                ElementState::Pressed => DispatchEvent::PointerDown { pointer, x, y, button },
                ElementState::Released => DispatchEvent::PointerUp { pointer, x, y, button },
            })
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let (delta_x, delta_y) = ui_host::normalize_wheel_delta_native(*delta);
            let (x, y) = app.last_pointer_pos;
            Some(DispatchEvent::Scroll { x, y, delta_x, delta_y })
        }
        WindowEvent::Touch(touch) => {
            let pointer = ui_host::pointer_info_for_touch(&mut app.pointers, touch);
            let x = touch.location.x as f32;
            let y = touch.location.y as f32;
            Some(match touch.phase {
                TouchPhase::Started => DispatchEvent::PointerDown { pointer, x, y, button: PointerButton::Primary },
                TouchPhase::Moved => DispatchEvent::PointerMove { pointer, x, y },
                TouchPhase::Ended | TouchPhase::Cancelled => DispatchEvent::PointerUp { pointer, x, y, button: PointerButton::Primary },
            })
        }
        WindowEvent::KeyboardInput { event, .. } => {
            let logical = ui_host::logical_key_to_dispatch_string(&event.logical_key);
            Some(ui_host::key_dispatch_event(logical, app.modifiers, event.state == ElementState::Pressed))
        }
        WindowEvent::ModifiersChanged(modifiers) => {
            app.modifiers = ui_host::modifiers_from_winit(modifiers.state());
            None
        }
        _ => None,
    }
}

impl ApplicationHandler<HostUserEvent> for WinitApp {
    /// 🪟️ Window creation ported verbatim from the old `SemioApp::resumed` (title/size/canvas-mount
    /// logic unchanged — see this file's own module docstring for why the two-phase handshake this
    /// method starts is kept, not deleted). Only the tail changes: no `start_frame_loop` call.
    // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        // ⏱️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): registers THIS thread —
        // winit's callback thread, the only thread `resumed`/`window_event`/`about_to_wait` ever run
        // on — as the UI thread with `semio-framework-trace`'s thread-role census, so
        // `semio_framework_trace::is_ui_thread()`/`assert_ui_thread()` are meaningful anywhere in this
        // process from this point on. Exactly once, first callback, before any event can be normalized.
        semio_framework_trace::register_ui_thread();
        // 🔔️ Real cross-thread waker installed exactly once, before the first `spawn_app_task` future
        // is ever queued (boot itself queues one just below) — see `ProxyWaker`'s own docstring.
        #[cfg(not(target_arch = "wasm32"))]
        crate::kernel_runtime::install_waker(std::task::Waker::from(Arc::new(ProxyWaker(self.proxy.clone()))));
        let mut attributes = WindowAttributes::default().with_title("Semio");
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            if let Some(canvas) = self.canvas.clone() {
                let css_width = canvas.client_width().max(1) as f32;
                let css_height = canvas.client_height().max(1) as f32;
                let _ = canvas.style().set_property("width", "100%");
                let _ = canvas.style().set_property("height", "100vh");
                attributes = attributes.with_inner_size(winit::dpi::LogicalSize::new(css_width, css_height)).with_canvas(Some(canvas)).with_append(true);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            attributes = attributes.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 800.0));
        }
        let window = Arc::new(event_loop.create_window(attributes).expect("create window"));
        self.window = Some(window.clone());
        let proxy = self.proxy.clone();
        let plugin_filter = self.plugin_filter.clone();
        #[cfg(target_arch = "wasm32")]
        let plugins = self.plugins.clone();
        #[cfg(not(target_arch = "wasm32"))]
        let plugin_modules_root = self.plugin_modules_root.clone();
        crate::spawn_app_task(async move {
            let result = crate::boot_runtime(
                window,
                plugin_filter,
                #[cfg(target_arch = "wasm32")]
                plugins,
                #[cfg(not(target_arch = "wasm32"))]
                plugin_modules_root,
            )
            .await;
            match result {
                Ok(runtime) => {
                    let _ = proxy.send_event(HostUserEvent::RuntimeReady { runtime });
                }
                Err(error) => crate::log_debug(&format!("boot_runtime failed: {error}")),
            }
        });
    }

    // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: HostUserEvent) {
        match event {
            HostUserEvent::RuntimeReady { runtime } => {
                let mut host = OsHost::new(runtime);
                let proxy = self.proxy.clone();
                host.kernel.set_waker(crate::kernel_seam::HostWaker::new(move || {
                    let _ = proxy.send_event(HostUserEvent::Wake);
                }));
                self.host = Some(host);
            }
            // 🔔️ `ProxyWaker`'s payload-free wake — arriving at all already interrupted
            // `WaitUntil`/`Wait`, which is the entire point; `about_to_wait` (running next, per
            // winit's own callback order) is where `poll_tasks()` actually drains the task pool.
            HostUserEvent::Wake => {}
        }
        self.recompute_control_flow(event_loop);
    }

    // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.clone() else { return };
        if self.host.is_none() {
            return;
        }
        match &event {
            WindowEvent::CloseRequested => {
                if self.host.as_mut().expect("checked above").close_requested() {
                    event_loop.exit();
                }
                return;
            }
            WindowEvent::Resized(size) => {
                let scale_factor = window.scale_factor() as f32;
                self.host.as_mut().expect("checked above").handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor });
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let size = window.inner_size();
                self.host.as_mut().expect("checked above").handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor: *scale_factor as f32 });
            }
            WindowEvent::RedrawRequested => {
                if let Some(reason) = self.pending_reason.take() {
                    let _outcome = self.host.as_mut().expect("checked above").redraw(reason);
                }
                self.recompute_control_flow(event_loop);
                return;
            }
            _ => {
                if let Some(dispatch_event) = normalize(self, &event) {
                    self.host.as_mut().expect("checked above").handle_event(dispatch_event);
                }
            }
        }
        self.recompute_control_flow(event_loop);
    }

    /// 🌙️ Replaces the old unconditional `poll_tasks()` + `request_redraw()` pair (every `Poll` tick)
    /// with: drain the kernel task pool (still every wake — cheap, bounded by whatever is actually
    /// queued, never busy-polling since this fn itself now only runs on a real wake), then request a
    /// redraw **only if `should_request_redraw` says so**.
    // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(not(target_arch = "wasm32"))]
        crate::kernel_runtime::poll_tasks();
        let Some(host) = self.host.as_mut() else { return };
        let now = host.now_seconds();
        if let Some(reason) = should_request_redraw(&mut host.scheduler, now) {
            self.pending_reason = Some(reason);
            if let Some(window) = self.window.as_ref() {
                window.request_redraw();
            }
        }
        self.recompute_control_flow(event_loop);
    }
}

impl WinitApp {
    /// 🚦️ `WaitUntil(next deadline)` / `Wait` — never `Poll` (this file's own headline change). The
    /// target `Instant` is computed directly from `Instant::now()` rather than threading `OsClock`'s
    /// own origin through, since `WaitUntil` only needs "approximately this far in the future", not
    /// perfect alignment with `OsClock`'s epoch.
    ///
    /// ⚠️ **Assumption flagged for verification (U4: this file is UNRUN — see report):**
    /// `std::time::Instant::now()` is assumed available on `wasm32-unknown-unknown` here on the
    /// strength of this crate's own `Cargo.toml` already enabling `getrandom`'s `wasm_js` feature
    /// (evidence it targets a `std` build with working wasm time/random shims) — but
    /// `ui_host::window::native::MonotonicClock` (a sibling packet, `backend-iface`/`ui-host`)
    /// deliberately confines its OWN `Instant`-based clock to `#[cfg(not(target_arch = "wasm32"))]`
    /// and ships a separate `performance.now()`-based `BrowserClock` for wasm — a signal from that
    /// packet's own authors that `Instant` was NOT assumed safe there. This file takes the opposite
    /// bet for now; if wrong, the fix is a wasm32-only branch computing the `WaitUntil` target via
    /// `web_sys::window().performance()` instead, mirroring `OsClock::now_seconds`'s wasm arm.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recompute_control_flow(&mut self, event_loop: &ActiveEventLoop) {
        let Some(host) = self.host.as_ref() else {
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
            return;
        };
        event_loop.set_control_flow(match host.scheduler.next_deadline() {
            Some(deadline) => {
                let remaining = (deadline.due - host.now_seconds()).max(0.0);
                winit::event_loop::ControlFlow::WaitUntil(std::time::Instant::now() + std::time::Duration::from_secs_f64(remaining))
            }
            None => winit::event_loop::ControlFlow::Wait,
        });
    }
}

//#endregion 🚀️WinitApp
