//! 🎞️ Frame generation and os host: the render frame operation id, the frame generation and scheduler redraw helpers, the host event enqueue, the `OsHost` window delegate and impls, the cursor and ime mapping and `run_native`. See [`super`].

use super::*;

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
pub(super) fn advance_frame_generation(generation: &mut u64) -> bool {
    let Some(next) = generation.checked_add(1) else { return false };
    *generation = next;
    true
}

pub(super) fn collect_scheduler_redraw(scheduler: &mut ui_render::FrameScheduler, pending_reason: &mut Option<InvalidationReason>, now: f64) -> Option<InvalidationReason> {
    let reason = ui_host::should_request_redraw(scheduler, now)?;
    if let Some(pending) = pending_reason.as_mut() {
        pending.insert(reason);
    } else {
        *pending_reason = Some(reason);
    }
    Some(reason)
}

pub(super) fn invalidate_scheduler_redraw(
    scheduler: &mut ui_render::FrameScheduler,
    pending_reason: &mut Option<InvalidationReason>,
    now: f64,
    reason: InvalidationReason,
) -> Option<InvalidationReason> {
    scheduler.invalidate(reason);
    collect_scheduler_redraw(scheduler, pending_reason, now)
}

/// 🔢️ Whether THIS enqueue may renumber the frame generation.
///
/// The generation names the INPUT STATE a build is answering, and a build, its presentation witness
/// and its raster witness are all pinned to it — so it may advance freely while nothing is building
/// ([`FrameGenerationHold::Free`]) and must not move at all while a build is live
/// ([`FrameGenerationHold::UnderLiveBuild`]).
///
/// 🩸️ Every pointer move used to renumber it unconditionally. `FrameBuildHandle::poll_runtime_on_original_caller`
/// reads a moved generation as `frame build superseded` and CANCELS the in-flight build from phase 0
/// — 28 supersessions per converging edit, a measured 26 % tax on a chain the input had nothing to do
/// with (10.69 s with the pointer still against 14.51 s with it moving 5×/s, ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §6). The event is
/// still enqueued and the scheduler still invalidated either way: holding the NUMBER is not dropping
/// the INPUT, and the build that finishes is immediately followed by one admitted at the newest
/// generation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FrameGenerationHold {
    Free,
    UnderLiveBuild,
}

/// 📥️ The exact mounted event-callback core, split from `OsHost` only so its latency contract can
/// be stress-tested without constructing a platform window or GPU surface.
pub(super) fn enqueue_host_event(events: &mut ui_host::EventQueue, scheduler: &mut ui_render::FrameScheduler, ui_token: ui_host::UiThreadToken, frame_generation: &mut u64, hold: FrameGenerationHold, event: DispatchEvent) -> ui_host::EnqueueOutcome {
    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_event", render_frame_operation_id(), semio_framework_trace::Generation(*frame_generation), semio_framework_trace::InteractiveStage::UiEvent);
    let mut next_generation = *frame_generation;
    if hold == FrameGenerationHold::Free && !advance_frame_generation(&mut next_generation) {
        return ui_host::EnqueueOutcome::Overflow;
    }
    let outcome = events.enqueue(ui_token, event);
    if outcome != ui_host::EnqueueOutcome::Accepted {
        return outcome;
    }
    *frame_generation = next_generation;
    scheduler.invalidate(InvalidationReason::INPUT_STATE);
    outcome
}

/// 📐️ The mounted resize-callback core, isolated for the same window-free latency proof as
/// [`enqueue_host_event`]. GPU surface reconfiguration remains the immediate platform-only step.
pub(super) fn enqueue_host_metrics(
    events: &mut ui_host::EventQueue,
    scheduler: &mut ui_render::FrameScheduler,
    ui_token: ui_host::UiThreadToken,
    frame_generation: &mut u64,
    hold: FrameGenerationHold,
    physical_width: u32,
    physical_height: u32,
    scale_factor: f32,
) -> ui_host::EnqueueOutcome {
    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_metrics", render_frame_operation_id(), semio_framework_trace::Generation(*frame_generation), semio_framework_trace::InteractiveStage::UiEvent);
    let mut next_generation = *frame_generation;
    if hold == FrameGenerationHold::Free && !advance_frame_generation(&mut next_generation) {
        return ui_host::EnqueueOutcome::Overflow;
    }
    let outcome = events.enqueue_metrics(ui_token, physical_width, physical_height, scale_factor);
    if outcome != ui_host::EnqueueOutcome::Accepted {
        return outcome;
    }
    *frame_generation = next_generation;
    scheduler.invalidate(InvalidationReason::VIEWPORT);
    outcome
}

//#region 🔖️WindowDelegate for OsHost
impl WindowDelegate for OsHost {
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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
    /// 🚫️async: U1 — the enqueue itself never awaits; the batched dispatch this feeds is the boundary-
    /// async exception U1 itself carves out.
    fn handle_event(&mut self, event: DispatchEvent) {
        let hold = self.frame_generation_hold();
        if enqueue_host_event(&mut self.events, &mut self.scheduler, self.ui_token, &mut self.frame_generation, hold, event) == ui_host::EnqueueOutcome::Overflow {
            semio_framework_os_renderer_wgpu_host_base::log_debug("os_host: discrete input queue overflow — a redraw has not drained in a while");
        }
    }

    /// 📐️ P5e funnels metrics through both the input queue and the fixed generation-qualified
    /// surface lane. This callback only publishes the newest scalar owner; worker preparation and the
    /// one UI-capability surface step advance from redraw opportunities.
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn handle_metrics(&mut self, metrics: WindowMetrics) {
        let hold = self.frame_generation_hold();
        if enqueue_host_metrics(&mut self.events, &mut self.scheduler, self.ui_token, &mut self.frame_generation, hold, metrics.physical.width, metrics.physical.height, metrics.scale_factor) != ui_host::EnqueueOutcome::Accepted {
            return;
        }
        let (width, height) = metrics.logical_size();
        let _ = self.surface_resize.enqueue(metrics.physical.width, metrics.physical.height, metrics.scale_factor);
        let dpr = metrics.scale_factor;
        let _ = self.runtime.enqueue_apply(Some("window-metrics"), true, semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimeApply::Resize { width, height, dpr });
    }

    /// 🖼️ P3/P5 mounted split: `build_and_publish_snapshot` drains bounded input and non-blockingly
    /// polls/submits the worker-owned `AppRuntime::frame` transaction. `present_snapshot` acquires
    /// the latest immutable snapshot and applies only UI-capability directives. A stalled worker
    /// therefore preserves the last valid presentation without extending this callback.
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn redraw(&mut self, _reason: InvalidationReason) -> RedrawOutcome {
        let _watchdog = semio_framework_trace::Watchdog::start("os_renderer_present", render_frame_operation_id(), semio_framework_trace::Generation(self.frame_generation), semio_framework_trace::InteractiveStage::UiPresent);
        self.present_snapshot(self.clock.now_seconds())
    }

    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn close_requested(&mut self) -> bool {
        true
    }
}

impl OsHost {
    /// 🛶️ Event closure requires real host, frame worker, presenter and snapshot publication quiescence.
    pub(super) fn original_event_terminal_is_empty(&self)->bool{self.events.is_empty()&&!self.frame_ready&&!self.surface_resize.has_work()&&!crate::os_host::component_surface_close_occupied()&&self.frame_build.original_event_terminal_is_empty()&&!self.presenter.has_pending_presentation()&&self.original_snapshot.is_none()&&self.snapshot_sink.original_event_terminal_is_empty()&&self.runtime.original_event_terminal_is_empty()}
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
    pub fn redraw_offscreen_worker(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<crate::os_host::BrowserRedrawOutcome>,semio_framework_job::OriginalTurnFailure>{
        let _=self.scheduler.should_render(self.clock.now_seconds());
        self.prepare_redraw(cx)?;
        if !self.snapshot_sink.publish_on_original_caller(cx)?{return Ok(None)}
        self.build_and_publish_snapshot_on_original_caller(cx)?;
        if !self.snapshot_sink.publish_on_original_caller(cx)?{return Ok(None)}
        let _=self.present_snapshot(self.clock.now_seconds());
        let Some(snapshot)=self.snapshot_sink.acquire_output_on_original_caller(cx)?else{return Ok(None)};
        let cursor_css=semio_framework_ui_wgpu_hosting::cursor::semio_cursor_css(snapshot.snapshot().accepted_cursor,snapshot.snapshot().accepted_theme_dark);
        Ok(Some(crate::os_host::BrowserRedrawOutcome{cursor_css,media_slots:snapshot}))
    }

    /// 🔢️ The frame generation names the INPUT STATE a build is answering, so it advances when input
    /// changes (`enqueue_host_event`/`enqueue_host_metrics`) and when a redraw finds no build to
    /// invalidate — never underneath a live one.
    ///
    /// 🩸️ `frame_ready` is only ever set by the native `HostUserEvent::FrameReady` proxy; the browser
    /// worker has no such proxy, so this used to renumber the inputs on EVERY tick. A build was
    /// therefore superseded one tick after it was admitted and could never take a second step: on
    /// 6118 exactly one build ever ran (`frame build admitted generation=Generation(3)`), the next
    /// tick superseded it, and the shell produced no further frame for the rest of the session
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`).
    fn prepare_redraw(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let _ = semio_framework_os_renderer_wgpu_host_plumbing::surface_lane::MountedSurfaceResizeLane::close_abandoned_step(cx)?;
        if self.frame_ready {
            self.frame_ready = false;
        } else if !self.frame_build.has_live_session() {
            if !advance_frame_generation(&mut self.frame_generation) {
                self.present_fault = Some("frame generation exhausted".to_string());
            }
        }
        self.surface_resize.drive_one(cx)?;
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
        Ok(())
    }

    pub(super) fn redraw_on_original_caller(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RedrawOutcome,semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(RedrawOutcome{cursor:CursorRequest::Default,ime:None})}
        self.prepare_redraw(cx)?;
        let now = self.clock.now_seconds();
        if !self.snapshot_sink.publish_on_original_caller(cx)?{return Ok(RedrawOutcome{cursor:CursorRequest::Default,ime:None})}
        self.build_and_publish_snapshot_on_original_caller(cx)?;
        Ok(self.present_snapshot(now))
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
    /// 🖱️ The snapshot keeps the generic five-value host cursor beside the accepted 14-value product
    /// cursor and its accepted theme. Native already applies the product pair during presentation;
    /// the browser projects the same pair through `semio_cursor_css` after this publication.
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    ///
    /// 🧵️ `poll_runtime_on_original_caller` never waits: it accepts a fresh completed frame or leaves the
    /// last presentation in place, then schedules at most one worker-owned frame transaction.
    ///
    /// 🩺️ The presentation gate decides whether a frame build runs at all, and a frame build is the
    /// ONLY thing that pumps the runtime mailbox — so a gate stuck shut is indistinguishable from
    /// "input never dispatched" unless it says who is holding it.
    ///
    /// 🖼️ Drive the present cursor for the rest of this tick's interactive share instead of one
    /// phase per redraw. `AppPresentCursor` walks begin-GPU → engine surfaces → uploads → command
    /// pages → submit one phase at a time; at one phase per browser frame a single presentation
    /// took seconds, during which `admit_next_frame` refuses to build the next frame at all, so
    /// the shell advanced roughly one frame every two seconds. Every prepared GPU opportunity
    /// inside the loop is still priced by its own two-millisecond ceiling
    /// (`🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎮️execution/🦀️.rs` `admit_prepared_gpu_opportunity`), so the loop cannot
    /// hide an over-ceiling submit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn pump_frame_inputs(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if !self.presenter.holds_presented_input_publication(){let _=self.runtime.pump_pending_applies(RUNTIME_APPLY_TICK_CREDITS,cx)?;}
        #[cfg(not(target_arch="wasm32"))]{let _=self.runtime.pump_native_asset();let _=self.with_asset_decode_context(|runtime,asset_cx|runtime.asset_decode_step(asset_cx))?;}
        if self.advance_component_surface_close(cx)?{self.scheduler.invalidate(InvalidationReason::RESOURCE_READY);}
        if !self.events.is_empty(){let _=self.runtime.enqueue_dispatch_on_original_caller(&mut self.events,self.frame_generation,cx)?;}Ok(())
    }

    /// 🎟️ Business authority is lent unchanged while presentation generation only selects freshness.
    fn build_and_publish_snapshot_on_original_caller(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(())}
        self.pump_frame_inputs(cx)?;
        if !crate::os_host::component_surface_close_pending(){
            let inputs=self.runtime.frame_inputs(semio_framework_os_renderer_wgpu_host_base::app_now_ms());
            let generation=semio_framework_trace::Generation(self.frame_generation);
            self.runtime.observe_presentation_input_generation(generation.0);
            let runtime=self.runtime.clone();
            let mut failure=None;let frame_build=&mut self.frame_build;
            let _=self.presenter.admit_next_frame(||match frame_build.poll_runtime_on_original_caller(runtime,inputs,generation,cx){Ok(frame)=>frame,Err(original)=>{failure=Some(original);None}});
            if let Some(original)=failure{return Err(original)}
        }
        self.publish_presented_snapshot_on_original_caller(cx)?;
        Ok(())
    }


    /// 📮️ The actual completed media body enters paid snapshot publication without serialization or a vector clone.
    fn publish_presented_snapshot_on_original_caller(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(())}
        if self.original_snapshot.is_some(){if !self.snapshot_sink.retain_original_publication(&mut self.original_snapshot,cx)?{self.scheduler.invalidate(InvalidationReason::RESOURCE_READY)}return Ok(())}
        if !self.snapshot_sink.publish_on_original_caller(cx)?{self.scheduler.invalidate(InvalidationReason::RESOURCE_READY);return Ok(())}
        let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(())}
        let present_deadline_us=cx.now_us().map_or(cx.deadline_us(),|now|now.saturating_add(semio_framework_job::INTERACTIVE_STEP_CEILING_US / 2));
        let present_fuel=cx.fuel_remaining();let mut presented=None;
        cx.with_original_work_loan(present_fuel,present_deadline_us,|present_cx|{presented=Some(self.presenter.present_step(present_cx));Ok(())})?;
        match presented.expect("the present step ran inside its work loan"){
            Ok(semio_framework_os_renderer_wgpu_frame_presentation::AppPresentStep::Complete{generation,cursor,theme_dark,fullscreen,cursor_wake,retained_control_deadline,shell_clock_deadline,has_animated_primitives,media_slots})=>{
                let _latency=semio_framework_os_renderer_wgpu_host_base::frame_latency::FrameLatencyTimer::start(semio_framework_os_renderer_wgpu_host_base::frame_latency::FrameLatencyAuthority::renderer_frame(generation.0),semio_framework_os_renderer_wgpu_host_base::frame_latency::FrameLatencyStage::SnapshotPublish,1);
                self.original_snapshot=Some(semio_framework_os_renderer_wgpu_host_plumbing::render_snapshot::RenderSnapshot::new(0,semio_cursor_to_request(cursor),cursor,theme_dark,None).with_media_slots(media_slots));
                cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
                self.animation_clock.accept(has_animated_primitives);self.animation_clock.sync(&mut self.scheduler,self.clock.now_seconds());self.runtime.publish_retained_control_deadline(retained_control_deadline);self.runtime.publish_shell_clock_deadline(shell_clock_deadline);self.platform_fullscreen=fullscreen;
                if let Some(token)=cursor_wake{if self.runtime.acknowledge_world_cursor_wake(&token){self.retain_cursor_wake_directive(token);self.scheduler.invalidate(InvalidationReason::RESOURCE_READY)}}
                if generation.0!=self.frame_generation{self.scheduler.invalidate(InvalidationReason::INPUT_STATE)}
            },
            Ok(semio_framework_os_renderer_wgpu_frame_presentation::AppPresentStep::Pending|semio_framework_os_renderer_wgpu_frame_presentation::AppPresentStep::RetryRuntime)=>self.scheduler.invalidate(InvalidationReason::RESOURCE_READY),
            Ok(semio_framework_os_renderer_wgpu_frame_presentation::AppPresentStep::AwaitingRuntime|semio_framework_os_renderer_wgpu_frame_presentation::AppPresentStep::Idle)=>{},
            Err(original)=>return Err(original),
        }Ok(())
    }

    /// 📤️ The PRESENT half — atomically acquires the newest published snapshot (never blocks, never
    /// waits: if nothing newer landed since the last call it re-presents the same one — the ticket's
    /// governing rule verbatim) and applies its cursor/IME directives plus this file's deadline
    /// sources. This is the bounded, ≤2ms-budget half of `redraw`; expensive frame construction
    /// happens in the worker transaction submitted by `build_and_publish_snapshot`.
    /// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn present_snapshot(&mut self, now: f64) -> RedrawOutcome {
        let snapshot = self.snapshot_sink.acquire();
        semio_framework_os_renderer_wgpu_host_plumbing::deadlines::sync_presented_deadlines(&mut self.scheduler, &mut self.animation_clock, self.runtime.retained_control_deadline(now), now, self.presenter.awaiting_runtime());

        #[cfg(not(target_arch = "wasm32"))]
        if self.hot_swap.is_due(now) {
            self.scheduler.request_deadline(now + semio_framework_os_renderer_wgpu_host_plumbing::deadlines::NATIVE_HOT_SWAP_POLL_SECONDS, InvalidationReason::RESOURCE_READY);
        }

        RedrawOutcome { cursor: snapshot.cursor, ime: snapshot.ime }
    }
}

/// 🖱️ Narrows `AppRuntime`'s cursor for the generic host outcome. The accepted product
/// cursor and theme remain intact in `RenderSnapshot` for browser presentation.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn semio_cursor_to_request(cursor: semio_framework_ui_wgpu_hosting::cursor::SemioCursor) -> CursorRequest {
    use semio_framework_ui_wgpu_hosting::cursor::SemioCursor as S;
    match cursor {
        S::Default | S::EwResize | S::NsResize | S::NwseResize | S::NeswResize | S::Crosshair | S::CrosshairCentered | S::NotAllowed => CursorRequest::Default,
        S::Pointer | S::Selectable | S::Foldable => CursorRequest::Pointer,
        S::Grab => CursorRequest::Grab,
        S::Grabbing | S::Move => CursorRequest::Grabbing,
        S::Text => CursorRequest::Text,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn ime_event_from_winit(ime: &winit::event::Ime) -> ImeEvent {
    match ime {
        winit::event::Ime::Enabled => ImeEvent::Start,
        winit::event::Ime::Preedit(text, cursor) => ImeEvent::Update { text: text.clone(), cursor: cursor.map(|(_, end)| end).unwrap_or_default() },
        winit::event::Ime::Commit(text) => ImeEvent::Commit { text: text.clone() },
        winit::event::Ime::Disabled => ImeEvent::Cancel,
    }
}
//#endregion 🔖️WindowDelegate for OsHost

/// 🚀️ The native entry: installs the runtime assets and the host temporal formatter, then runs the winit event loop with a [`WinitApp`] until it exits. Lives beside [`WinitApp`], the one item it needs from above the probe crate.
#[cfg(not(target_arch = "wasm32"))]
pub fn run_native(plugin_filter: &str, plugin_modules_root: std::path::PathBuf, services:Vec<semio_framework_os_kernel::os_directory::client::InstalledServiceContributionV1>) {
    semio_framework_os_renderer_wgpu_host_base::runtime_assets::install_native_runtime_assets();
    semio_framework_os_renderer_wgpu_native_platform::install_native_host_temporal_formatter(std::sync::Arc::new(semio_framework_os_renderer_wgpu_native_platform::native_temporal::SystemTemporalFormatter::default()));
    let event_loop = winit::event_loop::EventLoop::<HostUserEvent>::with_user_event().build().expect("event loop");
    let proxy = event_loop.create_proxy();
    let caller = semio_framework_os_renderer_wgpu_host_plumbing::native_caller::NativeFrameEventIssuer::new(semio_framework_job::default_now_us, semio_framework_os_renderer_wgpu_host_plumbing::native_caller::NativeFrameRootPolicyV1::initiating_event()).expect("native initiating event policy");
    let mut app = WinitApp::new(proxy, plugin_filter.to_string(), plugin_modules_root, services, caller);
    let _ = event_loop.run_app(&mut app);
}
