//! 🪟️ The event loop that does nothing when nothing changed — ticket
//! `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet `os-host`, master plan §5.
//! `WinitApp` is the `ApplicationHandler` **replacing `SemioApp`**: `ControlFlow::Poll`, the
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

use crate::os_host::OsHost;
use crate::AppInteractionState;
use ui_host::{RedrawOutcome, WindowDelegate, WindowMetrics};
use ui_render::{CursorRequest, DispatchEvent, EventModifiers, ImeEvent, InvalidationReason, PointerButton};

/// ⏱️ P1e: this process has exactly one renderer frame callback (`OsHost::redraw`, below) — one
/// `OperationId` allocated once, lazily, on first frame, rather than a fresh one per call (an
/// `OperationId` names a logical operation across its `Generation`s, not one single call).
fn render_frame_operation_id() -> semio_framework_trace::OperationId {
    static ID: std::sync::OnceLock<semio_framework_trace::OperationId> = std::sync::OnceLock::new();
    *ID.get_or_init(semio_framework_trace::allocate_operation_id)
}

/// 📮️ Runtime completions one host tick may apply before it must go on and present.
const RUNTIME_APPLY_TICK_CREDITS: u32 = 8;

/// 🔢️ Advances a mounted frame generation once and permanently refuses exhaustion.
fn advance_frame_generation(generation: &mut u64) -> bool {
    let Some(next) = generation.checked_add(1) else { return false };
    *generation = next;
    true
}

/// 🔢️ Whether THIS enqueue may renumber the frame generation.
///
/// The generation names the INPUT STATE a build is answering, and a build, its presentation witness
/// and its raster witness are all pinned to it — so it may advance freely while nothing is building
/// ([`FrameGenerationHold::Free`]) and must not move at all while a build is live
/// ([`FrameGenerationHold::UnderLiveBuild`]).
///
/// 🩸️ Every pointer move used to renumber it unconditionally. `FrameBuildHandle::poll_runtime_and_resubmit`
/// reads a moved generation as `frame build superseded` and CANCELS the in-flight build from phase 0
/// — 28 supersessions per converging edit, a measured 26 % tax on a chain the input had nothing to do
/// with (10.69 s with the pointer still against 14.51 s with it moving 5×/s, ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §6). The event is
/// still enqueued and the scheduler still invalidated either way: holding the NUMBER is not dropping
/// the INPUT, and the build that finishes is immediately followed by one admitted at the newest
/// generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FrameGenerationHold {
    Free,
    UnderLiveBuild,
}

/// 📥️ The exact mounted event-callback core, split from `OsHost` only so its latency contract can
/// be stress-tested without constructing a platform window or GPU surface.
fn enqueue_host_event(events: &mut ui_host::EventQueue, scheduler: &mut ui_render::FrameScheduler, ui_token: ui_host::UiThreadToken, frame_generation: &mut u64, hold: FrameGenerationHold, event: DispatchEvent) -> ui_host::EnqueueOutcome {
    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_event", render_frame_operation_id(), semio_framework_trace::Generation(*frame_generation), semio_framework_trace::InteractiveStage::UiEvent);
    if hold == FrameGenerationHold::Free && !advance_frame_generation(frame_generation) {
        return ui_host::EnqueueOutcome::Overflow;
    }
    scheduler.invalidate(InvalidationReason::INPUT_STATE);
    events.enqueue(ui_token, event)
}

/// 📐️ The mounted resize-callback core, isolated for the same window-free latency proof as
/// [`enqueue_host_event`]. GPU surface reconfiguration remains the immediate platform-only step.
fn enqueue_host_metrics(events: &mut ui_host::EventQueue, scheduler: &mut ui_render::FrameScheduler, ui_token: ui_host::UiThreadToken, frame_generation: &mut u64, hold: FrameGenerationHold, physical_width: u32, physical_height: u32, scale_factor: f32) {
    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_metrics", render_frame_operation_id(), semio_framework_trace::Generation(*frame_generation), semio_framework_trace::InteractiveStage::UiEvent);
    if hold == FrameGenerationHold::Free && !advance_frame_generation(frame_generation) {
        return;
    }
    scheduler.invalidate(InvalidationReason::VIEWPORT);
    events.enqueue_metrics(ui_token, physical_width, physical_height, scale_factor);
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
        crate::log_debug(&format!("[DEBUG] os_host handle_event {event:?} gen={}", self.frame_generation));
        let hold = self.frame_generation_hold();
        if enqueue_host_event(&mut self.events, &mut self.scheduler, self.ui_token, &mut self.frame_generation, hold, event) == ui_host::EnqueueOutcome::Overflow {
            crate::log_debug("os_host: discrete input queue overflow — a redraw has not drained in a while");
        }
    }

    /// 📐️ P5e funnels metrics through both the input queue and the fixed generation-qualified
    /// surface lane. This callback only publishes the newest scalar owner; worker preparation and the
    /// one UI-capability surface step advance from redraw opportunities.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn handle_metrics(&mut self, metrics: WindowMetrics) {
        let hold = self.frame_generation_hold();
        enqueue_host_metrics(&mut self.events, &mut self.scheduler, self.ui_token, &mut self.frame_generation, hold, metrics.physical.width, metrics.physical.height, metrics.scale_factor);
        let (width, height) = metrics.logical_size();
        let _ = self.surface_resize.enqueue(metrics.physical.width, metrics.physical.height, metrics.scale_factor);
        let dpr = metrics.scale_factor;
        let _ = self.runtime.enqueue_apply(Some("window-metrics"), true, crate::RuntimeApply::Resize { width, height, dpr });
    }

    /// 🖼️ P3/P5 mounted split: `build_and_publish_snapshot` drains bounded input and non-blockingly
    /// polls/submits the worker-owned `AppRuntime::frame` transaction. `present_snapshot` acquires
    /// the latest immutable snapshot and applies only UI-capability directives. A stalled worker
    /// therefore preserves the last valid presentation without extending this callback.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn redraw(&mut self, _reason: InvalidationReason) -> RedrawOutcome {
        let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_present", render_frame_operation_id(), semio_framework_trace::Generation(self.frame_generation), semio_framework_trace::InteractiveStage::UiPresent);
        self.redraw_core()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn close_requested(&mut self) -> bool {
        true
    }
}

impl OsHost {
    /// 🔢️ The one place that answers whether an input callback may renumber the frame generation —
    /// the same `has_live_session()` predicate `redraw_core` already honours below, so the law
    /// ("it advances when input changes, and when a redraw finds no build to invalidate — never
    /// underneath a live one") is enforced at BOTH of its two writers instead of one.
    fn frame_generation_hold(&self) -> FrameGenerationHold {
        if self.frame_build.has_live_session() {
            FrameGenerationHold::UnderLiveBuild
        } else {
            FrameGenerationHold::Free
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn redraw_offscreen_worker(&mut self) -> RedrawOutcome {
        let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_offscreen_worker", render_frame_operation_id(), semio_framework_trace::Generation(self.frame_generation), semio_framework_trace::InteractiveStage::InteractiveStep);
        self.redraw_core()
    }

    fn redraw_core(&mut self) -> RedrawOutcome {
        let _ = crate::surface_lane::MountedSurfaceResizeLane::close_abandoned_step();
        // 🔢️ The frame generation names the INPUT STATE a build is answering, so it advances when input
        // changes (`enqueue_host_event`/`enqueue_host_metrics`) and when a redraw finds no build to
        // invalidate — never underneath a live one.
        //
        // 🩸️ `frame_ready` is only ever set by the native `HostUserEvent::FrameReady` proxy; the browser
        // worker has no such proxy, so this used to renumber the inputs on EVERY tick. A build was
        // therefore superseded one tick after it was admitted and could never take a second step: on
        // 6118 exactly one build ever ran (`frame build admitted generation=Generation(3)`), the next
        // tick superseded it, and the shell produced no further frame for the rest of the session
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`).
        if self.frame_ready {
            self.frame_ready = false;
        } else if !self.frame_build.has_live_session() {
            if !advance_frame_generation(&mut self.frame_generation) {
                self.present_fault = Some("frame generation exhausted".to_string());
            }
        }
        self.surface_resize.drive_one();
        if self.presenter.surface_resize_available() {
            if let Some(candidate) = self.surface_resize.take_ready() {
                if let Err(candidate) = self.presenter.begin_surface_resize(candidate) {
                    let _ = self.surface_resize.restore_ready(candidate);
                }
            }
        }
        if !self.presenter.surface_resize_step() || self.surface_resize.has_work() {
            self.scheduler.invalidate(InvalidationReason::VIEWPORT);
        }
        let now = self.clock.now_seconds();
        self.build_and_publish_snapshot();
        self.present_snapshot(now)
    }
}

/// 🖼️ `build_and_publish_snapshot`/`present_snapshot` are inherent methods because only
/// `present_snapshot` owns the UI presentation capability; worker frame construction is reached
/// through the bounded `FrameBuildHandle` mailbox.
impl OsHost {
    /// 🏗️ The coordinator half — drains bounded input, launches async reduction, polls the capacity-one
    /// frame mailbox, submits the next worker transaction, and publishes the newest available cursor
    /// revision. Product traversal, layout, tessellation, and prepared packet construction execute in
    /// the worker closure; only `AppPresenter::present` realizes GPU/platform directives here.
    ///
    /// **Known fidelity loss — `ui_render::CursorRequest` (5 variants) vs `SemioCursor` (13
    /// variants).** `AppRuntime::frame()` already applies its own richer cursor internally via
    /// `ui_wgpu::wgpu::apply_window_cursor` before this method returns; the published snapshot's cursor
    /// is a best-effort narrowing of that SAME already-applied cursor (`semio_cursor_to_request`
    /// below), not an independent decision — see `present_snapshot`'s own doc for the idempotency
    /// argument this fidelity loss never actually exercises on this file's own hand-rolled loop.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn build_and_publish_snapshot(&mut self) {
        if !self.events.is_empty() && self.runtime.has_lossless_capacity() {
            let generation = self.events.current_generation();
            let drained = self.events.drain_page(ui_host::WorkerContext::new(generation));
            let drained_shape = format!("move={} scroll={} metrics={} discrete={}", drained.pointer_move.is_some(), drained.scroll.is_some(), drained.metrics.is_some(), drained.discrete.iter().filter(|slot| slot.is_some()).count());
            let admitted = self.runtime.enqueue_apply(None, true, crate::RuntimeApply::DispatchEvents(Some(crate::RuntimeDispatchCursor::new(drained))));
            crate::log_debug(&format!("[DEBUG] os_host drain events generation={generation:?} {drained_shape} enqueue-apply={admitted}"));
        }
        // 🧵️ `poll_runtime_and_resubmit` never waits: it accepts a fresh completed frame or leaves the
        // last presentation in place, then schedules at most one worker-owned frame transaction.
        let build_inputs = self.runtime.frame_inputs(crate::app_now_ms());
        let build_operation = render_frame_operation_id();
        let build_generation = semio_framework_trace::Generation(self.frame_generation);
        self.runtime.observe_presentation_input_generation(build_generation.0);
        // 📮️ One bounded mailbox share per host tick, taken BEFORE the presentation gate — a completion
        // that arrives while no frame build can run (the interaction state is checked out, or a
        // presentation is still pending) must still be applied within the next frame.
        let _ = self.runtime.pump_pending_applies(RUNTIME_APPLY_TICK_CREDITS);
        let runtime = self.runtime.clone();
        // 🩺️ The presentation gate decides whether a frame build runs at all, and a frame build is the
        // ONLY thing that pumps the runtime mailbox — so a gate stuck shut is indistinguishable from
        // "input never dispatched" unless it says who is holding it.
        crate::log_debug_diagnostic_once_per_transition("frame-gate", self.presenter.has_pending_presentation(), &format!("[DEBUG] os_host frame gate blocked={} {} generation={build_generation:?}", self.presenter.has_pending_presentation(), self.presenter.presentation_gate_shape()));
        let frame_build = &mut self.frame_build;
        let _ = self.presenter.admit_next_frame(|| frame_build.poll_runtime_and_resubmit(runtime, build_inputs, build_operation, build_generation));
        // 🖼️ Drive the present cursor for the rest of this tick's interactive share instead of one
        // phase per redraw. `AppPresentCursor` walks begin-GPU → engine surfaces → uploads → command
        // pages → submit one phase at a time; at one phase per browser frame a single presentation
        // took seconds, during which `admit_next_frame` refuses to build the next frame at all, so
        // the shell advanced roughly one frame every two seconds. Every prepared GPU opportunity
        // inside the loop is still priced by its own two-millisecond ceiling
        // (`🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs` `admit_prepared_gpu_opportunity`), so the loop cannot
        // hide an over-ceiling submit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        #[cfg(target_arch = "wasm32")]
        let present_deadline_us = semio_framework_job::default_now_us().map(|now| now.saturating_add(semio_framework_job::INTERACTIVE_STEP_CEILING_US / 2));
        loop {
        match self.presenter.present_step() {
            Ok(crate::AppPresentStep::Complete { generation, cursor, fullscreen, cursor_wake }) => {
                if generation.0 != self.frame_generation {
                    self.scheduler.invalidate(InvalidationReason::INPUT_STATE);
                    return;
                }
                self.platform_fullscreen = fullscreen;
                if let Some(token) = cursor_wake {
                    if self.runtime.acknowledge_world_cursor_wake(&token) {
                        self.retain_cursor_wake_directive(token);
                        self.scheduler.invalidate(InvalidationReason::RESOURCE_READY);
                    }
                }
                let Some(revision) = self.snapshot_sink.next_revision() else {
                    self.present_fault = Some("render snapshot revision exhausted".to_string());
                    return;
                };
                self.snapshot_sink.publish(crate::render_snapshot::RenderSnapshot::new(revision, semio_cursor_to_request(cursor), None));
            }
            Ok(crate::AppPresentStep::Pending) => self.scheduler.invalidate(InvalidationReason::RESOURCE_READY),
            Ok(crate::AppPresentStep::Idle) => return,
            Err(error) => {
                crate::log_debug(&format!("[DEBUG] os_host present_step faulted: {error}"));
                self.present_fault = Some(error);
                if self.presenter.has_pending_presentation() {
                    self.scheduler.invalidate(InvalidationReason::RESOURCE_READY);
                }
                return;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        return;
        #[cfg(target_arch = "wasm32")]
        if present_deadline_us.is_none_or(|deadline| semio_framework_job::default_now_us().is_none_or(|now| now >= deadline)) {
            return;
        }
        }
    }

    /// 📤️ The PRESENT half — atomically acquires the newest published snapshot (never blocks, never
    /// waits: if nothing newer landed since the last call it re-presents the same one — the ticket's
    /// governing rule verbatim) and applies its cursor/IME directives plus this file's deadline
    /// sources. This is the bounded, ≤2ms-budget half of `redraw`; expensive frame construction
    /// happens in the worker transaction submitted by `build_and_publish_snapshot`.
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
/// 📤️ P3a: the runtime mailbox invokes this for one retained cursor item per worker turn.
pub(crate) async fn dispatch_normalized_event(app: &mut AppInteractionState, event: DispatchEvent) {
    crate::log_debug(&format!("[DEBUG] os_host dispatch_normalized_event {event:?}"));
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
        // 🖱️ The wheel's OWN point, not the pointer's last known one — see `AppWheel`.
        DispatchEvent::Scroll { x, y, delta_y, .. } => {
            app.wheel.accumulate(x, y, delta_y);
        }
        // ⌨️ `DispatchEvent`'s POINTER variants carry no modifier state — only the key variants do
        // (`PointerDown/Up/Move { pointer, x, y, button }`). This is therefore the one place the
        // shell can learn that shift/ctrl/alt/meta are held, and `app.modifiers` is what the three
        // pointer arms above read. Before this assignment `app.modifiers` was a closed loop —
        // written only by `handle_pointer_*` from the value those same arms had just handed it — so
        // it never left `PointerModifiers::default()`. Measured on 6118: every world3d intent carried
        // `mods=----`, so shift-click published `merge:"replace"` instead of `additive`, and
        // alt/shift + right-drag never reached `plan_world3d_drag`'s orbit/pan arms at all
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`).
        // A modifier KeyDown/KeyUp reports the FULL post-event state (`Shift` down → `shift: true`,
        // `Shift` up → `shift: false`), so mirroring both edges is exactly winit's
        // `WindowEvent::ModifiersChanged` for the normalized dispatch path.
        DispatchEvent::KeyDown { key, modifiers } => {
            app.modifiers = event_modifiers_to_pointer(modifiers);
            if (modifiers.ctrl || modifiers.meta) && key.eq_ignore_ascii_case("z") && app.undo_text_operation() {
                return;
            }
            if key == "Escape" {
                app.cancel_text_operations();
            }
            if let Some(action) = key_action_from_dispatch(&key, true) {
                app.handle_key(action, event_modifiers_to_pointer(modifiers)).await;
            }
        }
        DispatchEvent::KeyUp { key, modifiers } => {
            app.modifiers = event_modifiers_to_pointer(modifiers);
            if let Some(action) = key_action_from_dispatch(&key, false) {
                app.handle_key(action, event_modifiers_to_pointer(modifiers)).await;
            }
        }
        DispatchEvent::TextInput { text } | DispatchEvent::Paste { text } => {
            if let Err(error) = app.enqueue_text_operation(text) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::TextEditStart { stream, declared_bytes, .. } => {
            if let Err(error) = app.start_text_operation(stream, declared_bytes) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::TextEditChunk { stream, text } => {
            if let Err(error) = app.push_text_operation(stream, text) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::TextEditCommit { stream } => {
            if let Err(error) = app.commit_text_operation(stream) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::TextEditAbort { stream } => {
            if let Err(error) = app.abort_text_operation(stream) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::Ime(ImeEvent::Commit { text }) => {
            if let Err(error) = app.enqueue_text_operation(text) {
                app.text_fault = Some(error);
            }
        }
        DispatchEvent::Ime(_) => {}
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
#[path = "../../../🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs"]
mod p3c_tests;

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

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::{advance_frame_generation, DispatchEvent, EventModifiers, InvalidationReason, OsHost, PointerButton, WindowDelegate, WindowMetrics};
    use crate::os_host::OsHostRetirement;
    use crate::RuntimeMailbox;
    use std::sync::Arc;
    use ui_host::should_request_redraw;
    use ui_render::{PhysicalSize, PointerInfo};
    #[cfg(target_arch = "wasm32")]
    use ui_render::{PointerId, PointerKind};
    use winit::application::ApplicationHandler;
    use winit::event::WindowEvent;
    use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
    use winit::window::{Window, WindowAttributes, WindowId};

    //#region 🚀️WinitApp

    /// 📨️ Same two-phase boot handshake `SemioApp`'s own `HostUserEvent` used — kept, not deleted, per
    /// this file's own module docstring on why `ui_host::NativeHost`'s single-phase construction cannot
    /// replace it. Only the STEADY-STATE control flow changes: `ControlFlow::Poll` → `WaitUntil`/`Wait`.
    /// No `callbacks` payload (the old variant carried `ui_wgpu::wgpu::PointerCallbacks`) — this file
    /// normalizes input itself via `ui_host::event` and drives `AppRuntime` through the enqueue-only
    /// `dispatch_normalized_event` path above, so `boot_runtime` returns only the runtime.
    pub(crate) enum HostUserEvent {
        RuntimeReady {
            runtime: RuntimeMailbox,
            presenter: crate::AppPresenter,
        },
        FrameReady,
        /// 🔔️ Payload-free worker-completion signal. Receiving it invalidates the retained snapshot;
        /// no future is polled by the event loop.
        Wake,
    }

    /// 🚀️ `ApplicationHandler` replacing `SemioApp` (deleted by this packet's surgical `🦀️.rs` edit —
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
        retirement: Option<OsHostRetirement>,
        #[cfg(not(target_arch = "wasm32"))]
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
                retirement: None,
                #[cfg(not(target_arch = "wasm32"))]
                pointers: ui_host::PointerRegistry::new(),
                modifiers: EventModifiers::default(),
                last_pointer_pos: (0.0, 0.0),
                pending_reason: None,
            }
        }
    }

    //#region 🎛️WinitEventNormalization

    fn pointer_info_for_mouse(app: &mut WinitApp, device: winit::event::DeviceId) -> PointerInfo {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::pointer_info_for_mouse(&mut app.pointers, device)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (app, device);
            PointerInfo { id: PointerId(1_u64 << 62), kind: PointerKind::Mouse, pressure: None, tilt: None }
        }
    }

    fn pointer_info_for_touch(app: &mut WinitApp, touch: &winit::event::Touch) -> PointerInfo {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::pointer_info_for_touch(&mut app.pointers, touch)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = app;
            PointerInfo { id: PointerId((0b10_u64 << 62) | (touch.id & ((1_u64 << 62) - 1))), kind: PointerKind::Touch, pressure: touch.force.map(|force| force.normalized() as f32), tilt: None }
        }
    }

    fn pointer_button_from_winit(button: winit::event::MouseButton) -> Option<PointerButton> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::pointer_button_from_winit(button)
        }
        #[cfg(target_arch = "wasm32")]
        {
            match button {
                winit::event::MouseButton::Left => Some(PointerButton::Primary),
                winit::event::MouseButton::Right => Some(PointerButton::Secondary),
                winit::event::MouseButton::Middle => Some(PointerButton::Middle),
                _ => None,
            }
        }
    }

    fn normalize_wheel_delta(delta: winit::event::MouseScrollDelta) -> (f32, f32) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::normalize_wheel_delta_native(delta)
        }
        #[cfg(target_arch = "wasm32")]
        {
            match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => (x * 40.0, y * 40.0),
                winit::event::MouseScrollDelta::PixelDelta(position) => (position.x as f32, position.y as f32),
            }
        }
    }

    fn modifiers_from_winit(state: winit::keyboard::ModifiersState) -> EventModifiers {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::modifiers_from_winit(state)
        }
        #[cfg(target_arch = "wasm32")]
        {
            EventModifiers { shift: state.shift_key(), ctrl: state.control_key(), alt: state.alt_key(), meta: state.super_key() }
        }
    }

    fn logical_key_to_dispatch_string(key: &winit::keyboard::Key) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        {
            ui_host::logical_key_to_dispatch_string(key)
        }
        #[cfg(target_arch = "wasm32")]
        {
            use winit::keyboard::Key;
            match key {
                Key::Character(value) => value.to_string(),
                Key::Named(named) => named_key_label(*named).to_string(),
                Key::Dead(Some(value)) => value.to_string(),
                _ => "Unidentified".to_string(),
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
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

    /// 📐️ Device pixels per logical pixel for the live window, or `1.0` before one exists. `winit`
    /// hands every pointer position in PHYSICAL pixels; the renderer hit-tests, lays out and paints
    /// in LOGICAL ones, so this is the native half of the ingress conversion the browser transport
    /// performs on its own side. Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn pointer_scale_factor(app: &WinitApp) -> f32 {
        app.window.as_ref().map_or(1.0, |window| {
            let scale = window.scale_factor() as f32;
            if scale.is_finite() && scale > 0.0 {
                scale
            } else {
                1.0
            }
        })
    }

    /// 🔀️ Raw `winit::event::WindowEvent` → `ui_render::DispatchEvent`, ported from
    /// `ui_host::window::native::NativeHost::normalize` (private, see this file's own struct docstring).
    /// Pointer and touch positions arrive PHYSICAL and leave LOGICAL — see [`pointer_scale_factor`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn normalize(app: &mut WinitApp, event: &WindowEvent) -> Option<DispatchEvent> {
        use winit::event::{ElementState, TouchPhase};
        let scale = pointer_scale_factor(app);
        match event {
            WindowEvent::CursorMoved { device_id, position } => {
                app.last_pointer_pos = (position.x as f32 / scale, position.y as f32 / scale);
                let pointer = pointer_info_for_mouse(app, *device_id);
                Some(DispatchEvent::PointerMove { pointer, x: app.last_pointer_pos.0, y: app.last_pointer_pos.1 })
            }
            WindowEvent::MouseInput { device_id, state, button } => {
                let pointer = pointer_info_for_mouse(app, *device_id);
                let button = pointer_button_from_winit(*button)?;
                let (x, y) = app.last_pointer_pos;
                Some(match state {
                    ElementState::Pressed => DispatchEvent::PointerDown { pointer, x, y, button },
                    ElementState::Released => DispatchEvent::PointerUp { pointer, x, y, button },
                })
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (delta_x, delta_y) = normalize_wheel_delta(*delta);
                let (x, y) = app.last_pointer_pos;
                Some(DispatchEvent::Scroll { x, y, delta_x, delta_y })
            }
            WindowEvent::Touch(touch) => {
                let pointer = pointer_info_for_touch(app, touch);
                let x = touch.location.x as f32 / scale;
                let y = touch.location.y as f32 / scale;
                Some(match touch.phase {
                    TouchPhase::Started => DispatchEvent::PointerDown { pointer, x, y, button: PointerButton::Primary },
                    TouchPhase::Moved => DispatchEvent::PointerMove { pointer, x, y },
                    TouchPhase::Ended | TouchPhase::Cancelled => DispatchEvent::PointerUp { pointer, x, y, button: PointerButton::Primary },
                })
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let logical = logical_key_to_dispatch_string(&event.logical_key);
                Some(ui_host::key_dispatch_event(logical, app.modifiers, event.state == ElementState::Pressed))
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                app.modifiers = modifiers_from_winit(modifiers.state());
                None
            }
            _ => None,
        }
    }

    //#endregion 🎛️WinitEventNormalization

    /// 🌓️ Publishes the OS light/dark preference winit reports into [`crate::set_host_appearance`],
    /// leaving that value's persisted-preference term untouched. `None` — a platform that reports no
    /// preference, and every wasm realm, where winit has no OS theme to read — changes NOTHING, so a
    /// value the page thread already forwarded survives window creation instead of being reset to
    /// light by it.
    // 🚫️async: U1 — called from winit's own sync `ApplicationHandler` callbacks.
    fn publish_system_appearance(theme: Option<winit::window::Theme>) {
        let Some(theme) = theme else { return };
        crate::set_host_appearance(crate::HostAppearance { system_dark: matches!(theme, winit::window::Theme::Dark), ..crate::host_appearance() });
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
            publish_system_appearance(window.theme());
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
                    Ok((runtime, presenter)) => {
                        let _ = proxy.send_event(HostUserEvent::RuntimeReady { runtime, presenter });
                    }
                    Err(error) => crate::log_debug(&format!("boot_runtime failed: {error}")),
                }
            });
        }

        // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
        fn user_event(&mut self, event_loop: &ActiveEventLoop, event: HostUserEvent) {
            match event {
                HostUserEvent::RuntimeReady { runtime, presenter } => {
                    let mut host = OsHost::new(runtime, presenter);
                    let proxy = self.proxy.clone();
                    #[cfg(not(target_arch = "wasm32"))]
                    host.runtime.set_waker(Arc::new(move || {
                        let _ = proxy.send_event(HostUserEvent::Wake);
                    }));
                    #[cfg(target_arch = "wasm32")]
                    host.runtime.set_waker(std::rc::Rc::new(move || {
                        let _ = proxy.send_event(HostUserEvent::Wake);
                    }));
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let proxy = self.proxy.clone();
                        host.frame_build.set_completion_waker(Arc::new(move || {
                            let _ = proxy.send_event(HostUserEvent::FrameReady);
                        }));
                    }
                    host.scheduler.invalidate(InvalidationReason::STRUCTURE);
                    self.host = Some(host);
                }
                HostUserEvent::FrameReady => {
                    if let Some(host) = self.host.as_mut() {
                        host.frame_ready = true;
                        host.scheduler.invalidate(InvalidationReason::RESOURCE_READY);
                    }
                }
                // 🔔️ Worker completion wake: invalidate only; no future is polled on this callback.
                HostUserEvent::Wake => {
                    if let Some(host) = self.host.as_mut() {
                        if !advance_frame_generation(&mut host.frame_generation) {
                            host.present_fault = Some("frame generation exhausted".to_string());
                        }
                        host.scheduler.invalidate(InvalidationReason::RESOURCE_READY);
                    }
                }
            }
            self.recompute_control_flow(event_loop);
        }

        // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
        fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
            let Some(window) = self.window.clone() else { return };
            if self.host.is_none() && self.retirement.is_none() {
                return;
            }
            match &event {
                WindowEvent::CloseRequested => {
                    let requested = self.host.as_mut().is_some_and(|host| host.close_requested());
                    if requested {
                        if let Some(host) = self.host.take() {
                            match host.try_into_retirement() {
                                Ok(retirement) => self.retirement = Some(retirement),
                                Err(mut host) => {
                                    host.present_fault = Some("host retirement abandonment registry refused admission".to_string());
                                    self.host = Some(host);
                                }
                            }
                        }
                        event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));
                    }
                    return;
                }
                WindowEvent::Resized(size) => {
                    let scale_factor = window.scale_factor() as f32;
                    if let Some(host) = self.host.as_mut() {
                        host.handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor });
                    }
                }
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    let size = window.inner_size();
                    if let Some(host) = self.host.as_mut() {
                        host.handle_metrics(WindowMetrics { physical: PhysicalSize::new(size.width, size.height), scale_factor: *scale_factor as f32 });
                    }
                }
                // 🌓️ The OS switched light/dark under a shell whose appearance preference is
                // `"system"`. The next `FrameBuildPhase::ThemeResolve` re-reads the published value,
                // so the only thing owed here is a redraw — the twin of React's ONE shared
                // `matchMedia("(prefers-color-scheme: dark)")` `change` listener
                // (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s `ensureElementsSurfaceChromeSystemListeners`).
                WindowEvent::ThemeChanged(theme) => {
                    publish_system_appearance(Some(*theme));
                    window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    if let Some(reason) = self.pending_reason.take() {
                        if let Some(host) = self.host.as_mut() {
                            let _outcome = host.redraw(reason);
                        }
                    }
                    self.recompute_control_flow(event_loop);
                    return;
                }
                _ => {
                    if let Some(dispatch_event) = normalize(self, &event) {
                        if let Some(host) = self.host.as_mut() {
                            host.handle_event(dispatch_event);
                        }
                    }
                }
            }
            self.recompute_control_flow(event_loop);
        }

        /// 🌙️ Requests a redraw only when the scheduler reports invalidation or a due deadline.
        /// Native futures run exclusively on the process worker pool; this callback never polls them.
        // 🚫️async: U1 — sync per winit's own `ApplicationHandler` trait.
        fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
            if !OsHostRetirement::close_abandoned_step() {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));
                return;
            }
            if let Some(retirement) = self.retirement.as_mut() {
                if retirement.close_step() && retirement.terminal_is_empty() {
                    self.retirement = None;
                    self.window = None;
                    event_loop.exit();
                } else {
                    event_loop.set_control_flow(winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_millis(1)));
                }
                return;
            }
            let Some(host) = self.host.as_mut() else { return };
            let now = host.now_seconds();
            if let Some(reason) = should_request_redraw(&mut host.scheduler, now) {
                self.pending_reason = Some(reason);
                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                    if reason.contains(InvalidationReason::RESOURCE_READY) {
                        let _ = host.take_cursor_wake_directive();
                    }
                }
            }
            self.recompute_control_flow(event_loop);
        }
    }

    impl WinitApp {
        /// 🚦️ `WaitUntil(next deadline)` / `Wait` — never `Poll` (this file's own headline change).
        /// `ControlFlow::wait_duration` selects winit's target clock while the scheduler remains in
        /// elapsed seconds, so native and browser builds share the same deadline policy.
        // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
        fn recompute_control_flow(&mut self, event_loop: &ActiveEventLoop) {
            let Some(host) = self.host.as_ref() else {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
                return;
            };
            event_loop.set_control_flow(match host.scheduler.next_deadline() {
                Some(deadline) => {
                    let remaining = (deadline.due - host.now_seconds()).max(0.0);
                    winit::event_loop::ControlFlow::wait_duration(std::time::Duration::from_secs_f64(remaining))
                }
                None => winit::event_loop::ControlFlow::Wait,
            });
        }
    }

    //#endregion 🚀️WinitApp
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::{HostUserEvent, WinitApp};

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-winit-app-callback-latency/🦀️.rs"]
mod callback_latency_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🔢️frame-generation-hold/🦀️.rs"]
mod frame_generation_hold_tests;
