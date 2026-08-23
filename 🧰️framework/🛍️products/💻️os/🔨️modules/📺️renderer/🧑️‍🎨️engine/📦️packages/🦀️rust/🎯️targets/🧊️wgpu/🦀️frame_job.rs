//! 🧵️ P3/P5 mounted frame coordinator. [`FrameBuildJob`] incrementally derives deadline candidates,
//! and [`FrameBuildHandle::poll_runtime_and_resubmit`] couples that protocol to the complete
//! `AppRuntime::frame` transaction. Native submits one generation at a time to the process worker
//! pool, publishes only matching completions, and never waits while polling. Frame construction owns
//! shell traversal, layout/tessellation, engine-scene directives, and prepared-packet creation;
//! `AppPresenter` retains platform/GPU presentation authority.
//!
//! The deadline candidates remain non-authoritative: `AppRuntime::frame` revalidates them against live
//! state before applying them. The wasm implementation drives the job synchronously only after the
//! renderer has booted inside its dedicated Worker isolate; calls from a browser UI isolate fail
//! closed and never execute the transaction inline.

use semio_framework_job::{root_cancel_token, BatchDriveConfig, BatchJobParams, CancelToken, CommitCandidate, InteractiveJob, StepContext, StepOutcome, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_MS};
use semio_framework_trace::{Generation, InteractiveStage, OperationId};
use std::collections::{hash_map::IntoIter, HashMap};
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{Receiver, TryRecvError};

//#region 📥️FrameBuildInputs
/// 📥️ The `Send`-safe slice of `AppRuntime` this job needs — cloned out of `self` once per submission
/// (all `f64`/`bool`/small `HashMap<String, f64>`, cheap to copy; never `AppRuntime`, `ShellState`,
/// or `GpuContext`).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FrameBuildInputs {
    pub world3d_camera_dispatch_deadlines_ms: HashMap<String, f64>,
    pub wheel_zoom_deadline_ms: f64,
    pub now_ms: f64,
}
//#endregion 📥️FrameBuildInputs

//#region 📤️FrameDirectives
/// 📤️ What the worker found — a CANDIDATE list, not an authoritative replacement.
/// `expired_world3d_surfaces` names surfaces the job believes are past their settle deadline as of its
/// (possibly stale) `FrameBuildInputs` snapshot; `frame()` re-checks each one against the LIVE
/// `AppRuntime::world3d_camera_dispatch_deadlines_ms` before removing it or dispatching a camera action
/// — see this module's own doc comment for why that re-validation, not blind trust, is what makes a
/// stale result safe.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct FrameDirectives {
    pub expired_world3d_surfaces: Vec<String>,
    pub wheel_zoom_deadline_cleared: bool,
}
//#endregion 📤️FrameDirectives

//#region 🧩️FrameBuildJob
/// 🧩️ The deadline-sweep computation as a real [`InteractiveJob`] — one call to `step` always reaches
/// [`StepOutcome::Complete`] (the work is O(open deadlines), never large enough to need more than one
/// step; `cx.consume_fuel` is still called so a future, larger stage added to this same job under
/// Phase 5 inherits real yield-on-overrun behaviour instead of it being bolted on later).
pub(crate) struct FrameBuildJob {
    deadlines: IntoIter<String, f64>,
    expired: Vec<String>,
    wheel_zoom_deadline_cleared: bool,
    now_ms: f64,
    invalid: bool,
    complete: Option<FrameDirectives>,
}

impl FrameBuildJob {
    pub(crate) fn new(inputs: FrameBuildInputs) -> Self {
        let invalid = inputs.world3d_camera_dispatch_deadlines_ms.len() > 256 || inputs.world3d_camera_dispatch_deadlines_ms.keys().any(|key| key.len() > 256);
        Self {
            deadlines: inputs.world3d_camera_dispatch_deadlines_ms.into_iter(),
            expired: Vec::with_capacity(256),
            wheel_zoom_deadline_cleared: inputs.wheel_zoom_deadline_ms > 0.0 && inputs.now_ms >= inputs.wheel_zoom_deadline_ms,
            now_ms: inputs.now_ms,
            invalid,
            complete: None,
        }
    }

    pub(crate) fn take_directives(&mut self) -> Option<FrameDirectives> {
        self.complete.take()
    }

    fn close_step(&mut self) -> bool {
        if let Some(surface) = self.expired.last_mut() {
            if surface.pop().is_some() {
                return false;
            }
            self.expired.pop();
            return false;
        }
        if self.deadlines.next().is_some() {
            return false;
        }
        if let Some(directives) = self.complete.as_mut() {
            if !directives.close_step() {
                return false;
            }
            self.complete = None;
            return false;
        }
        true
    }
}

impl FrameDirectives {
    fn close_step(&mut self) -> bool {
        if let Some(surface) = self.expired_world3d_surfaces.last_mut() {
            if surface.pop().is_some() {
                return false;
            }
            self.expired_world3d_surfaces.pop();
            return false;
        }
        true
    }
}

impl InteractiveJob for FrameBuildJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if self.invalid {
            return StepOutcome::Fault(semio_framework_job::JobFault { detail: b"frame deadline credits exceeded".to_vec() });
        }
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        for _ in 0..16 {
            let Some((surface, deadline)) = self.deadlines.next() else {
                self.complete = Some(FrameDirectives { expired_world3d_surfaces: std::mem::take(&mut self.expired), wheel_zoom_deadline_cleared: self.wheel_zoom_deadline_cleared });
                return StepOutcome::Complete(CommitCandidate { state: Vec::new(), output: Vec::new() });
            };
            if self.now_ms >= deadline {
                self.expired.push(surface);
            }
            cx.consume_fuel(1);
            if cx.should_yield() || cx.is_cancelled() {
                return if cx.is_cancelled() { StepOutcome::Cancelled } else { StepOutcome::Yield };
            }
        }
        StepOutcome::Yield
    }
}
//#endregion 🧩️FrameBuildJob

//#region ⏱️Clock
/// ⏱️ `BatchJobParams::now_ms` is a plain `fn() -> u64` pointer (not a closure) — `crate::app_now_ms`
/// returns `f64` wall-clock milliseconds, so this wraps it to the integer form the job protocol wants
/// without changing `app_now_ms`'s own signature (used elsewhere for sub-millisecond arithmetic).
fn now_ms_u64() -> u64 {
    crate::app_now_ms() as u64
}

fn batch_params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
    BatchJobParams {
        operation,
        generation,
        cancel,
        config: BatchDriveConfig { site: "os_renderer_frame_build", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_ms: INTERACTIVE_LANE_WALL_MS },
        now_ms: now_ms_u64,
    }
}
//#endregion ⏱️Clock

//#region 📮️FrameBuildHandle
/// 📮️ The non-blocking poll/resubmit contract item 5 of the packet brief asks for: never waits on the
/// worker. `poll_and_resubmit` always returns immediately — either this tick's freshly-completed
/// directives, or (if the in-flight job hasn't finished, or none was in flight yet) the last
/// successfully computed ones. One job is in flight at a time; a still-running job is left alone (not
/// cancelled) and re-checked next call rather than submitting a second overlapping one.
pub(crate) struct FrameBuildHandle {
    #[cfg(not(target_arch = "wasm32"))]
    runtime_in_flight: Option<Receiver<RuntimeFrameResult>>,
    #[cfg(not(target_arch = "wasm32"))]
    completion_waker: Option<Arc<dyn Fn() + Send + Sync>>,
    latest_requested_generation: Generation,
    last_submitted_generation: Option<Generation>,
    cancel: CancelToken,
    closing: bool,
    #[cfg(target_arch = "wasm32")]
    active: Option<ActiveFrameBuild>,
}

#[cfg(not(target_arch = "wasm32"))]
struct RuntimeFrameResult {
    generation: Generation,
    active: Option<ActiveFrameBuild>,
    frame: Option<crate::AppFramePresentation>,
}

enum ActiveFramePhase {
    Deadlines(FrameBuildJob),
    ApplyPending(FrameDirectives),
    Build(crate::AppFrameTransaction),
    Prepare(crate::AppFramePreparation),
    Terminal,
}

struct ActiveFrameBuild {
    runtime: crate::RuntimeMailbox,
    handle: crate::AppHandle,
    operation: OperationId,
    generation: Generation,
    dpr: f32,
    cancel: CancelToken,
    preview_sequence: u64,
    phase: ActiveFramePhase,
}

enum ActiveFrameStep {
    Pending,
    Complete(Option<crate::AppFramePresentation>),
}

fn retire_active_phase(phase: &mut ActiveFramePhase) -> bool {
    match phase {
        ActiveFramePhase::Deadlines(job) => job.close_step(),
        ActiveFramePhase::ApplyPending(directives) => directives.close_step(),
        ActiveFramePhase::Build(transaction) => transaction.close_step(),
        ActiveFramePhase::Prepare(preparation) => preparation.close_step() && preparation.terminal_is_empty(),
        ActiveFramePhase::Terminal => true,
    }
}

fn frame_phase_overran(before: u64, site: &'static str, operation: OperationId, generation: Generation) -> bool {
    semio_framework_trace::Watchdog::violation_count() > before && semio_framework_trace::Watchdog::violations().into_iter().rev().any(|violation| violation.site == site && violation.operation == operation && violation.generation == generation)
}

impl ActiveFrameBuild {
    fn new(runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, dpr: f32, cancel: CancelToken) -> Self {
        let handle = runtime.downgrade();
        Self { runtime, handle, operation, generation, dpr, cancel, preview_sequence: 0, phase: ActiveFramePhase::Deadlines(FrameBuildJob::new(inputs)) }
    }

    fn cancel(&self) {
        self.cancel.cancel_now();
    }

    fn retire_cancelled_phase(&mut self) -> bool {
        retire_active_phase(&mut self.phase)
    }

    fn quarantine_overrun(&self, site: &'static str) {
        self.runtime.record_frame_fault(site);
        self.cancel.cancel_now();
    }

    fn step(&mut self) -> ActiveFrameStep {
        if self.cancel.is_cancelled_now() {
            if self.retire_cancelled_phase() {
                self.phase = ActiveFramePhase::Terminal;
                return ActiveFrameStep::Complete(None);
            }
            return ActiveFrameStep::Pending;
        }
        match &mut self.phase {
            ActiveFramePhase::Deadlines(job) => {
                let now = now_ms_u64();
                let violations = semio_framework_trace::Watchdog::violation_count();
                let outcome = semio_framework_job::drive_step(
                    job,
                    "os_renderer.frame.deadlines",
                    self.operation,
                    self.generation,
                    InteractiveStage::InteractiveStep,
                    semio_framework_job::StepBudget::new(16, now.saturating_add(INTERACTIVE_LANE_WALL_MS)),
                    self.cancel.clone(),
                    now_ms_u64,
                    &mut self.preview_sequence,
                );
                if frame_phase_overran(violations, "os_renderer.frame.deadlines", self.operation, self.generation) {
                    self.quarantine_overrun("os_renderer.frame.deadlines overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                match outcome {
                    StepOutcome::Complete(_) => {
                        let directives = job.take_directives().unwrap_or_default();
                        self.phase = ActiveFramePhase::ApplyPending(directives);
                        ActiveFrameStep::Pending
                    }
                    StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => ActiveFrameStep::Pending,
                    StepOutcome::Cancelled | StepOutcome::Fault(_) => {
                        self.cancel.cancel_now();
                        ActiveFrameStep::Pending
                    }
                }
            }
            ActiveFramePhase::ApplyPending(directives) => {
                let violations = semio_framework_trace::Watchdog::violation_count();
                let applied = {
                    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer.frame.apply_pending", self.operation, self.generation, InteractiveStage::InteractiveStep);
                    self.runtime.apply_pending_step()
                };
                if frame_phase_overran(violations, "os_renderer.frame.apply_pending", self.operation, self.generation) {
                    self.quarantine_overrun("os_renderer.frame.apply_pending overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                if applied {
                    return ActiveFrameStep::Pending;
                }
                self.phase = ActiveFramePhase::Build(crate::AppFrameTransaction::new(std::mem::take(directives), self.generation, self.dpr));
                ActiveFrameStep::Pending
            }
            ActiveFramePhase::Build(transaction) => {
                let violations = semio_framework_trace::Watchdog::violation_count();
                let transaction_step = {
                    let _watchdog = semio_framework_trace::Watchdog::start("os_renderer.frame.transaction", self.operation, self.generation, InteractiveStage::InteractiveStep);
                    let now = now_ms_u64();
                    let mut context = StepContext::new(self.operation, self.generation, semio_framework_job::StepBudget::new(1, now.saturating_add(INTERACTIVE_LANE_WALL_MS)), self.cancel.clone(), now_ms_u64, &mut self.preview_sequence);
                    transaction.step(&self.runtime, &self.handle, &mut context)
                };
                if frame_phase_overran(violations, "os_renderer.frame.transaction", self.operation, self.generation) {
                    if let crate::AppFrameTransactionStep::Complete(frame) = transaction_step {
                        let preparation = frame.into_preparation();
                        self.phase = ActiveFramePhase::Prepare(preparation);
                    }
                    self.quarantine_overrun("os_renderer.frame.transaction overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                match transaction_step {
                    crate::AppFrameTransactionStep::Complete(frame) => {
                        self.phase = ActiveFramePhase::Prepare(frame.into_preparation());
                        ActiveFrameStep::Pending
                    }
                    crate::AppFrameTransactionStep::Pending => ActiveFrameStep::Pending,
                    crate::AppFrameTransactionStep::Fault => {
                        self.cancel.cancel_now();
                        ActiveFrameStep::Pending
                    }
                }
            }
            ActiveFramePhase::Prepare(preparation) => {
                let violations = semio_framework_trace::Watchdog::violation_count();
                let outcome = preparation.drive_step(self.operation, self.generation, self.cancel.clone(), &mut self.preview_sequence);
                if frame_phase_overran(violations, "os_renderer.prepare.worker", self.operation, self.generation) {
                    self.quarantine_overrun("os_renderer.prepare.worker overran the interactive ceiling");
                    return ActiveFrameStep::Pending;
                }
                match outcome {
                    StepOutcome::Complete(_) => {
                        let frame = preparation.take_presentation();
                        self.phase = ActiveFramePhase::Terminal;
                        ActiveFrameStep::Complete(frame)
                    }
                    StepOutcome::Yield | StepOutcome::PreviewReady(_) | StepOutcome::CheckpointReady(_) => ActiveFrameStep::Pending,
                    StepOutcome::Cancelled | StepOutcome::Fault(_) => {
                        self.cancel.cancel_now();
                        ActiveFrameStep::Pending
                    }
                }
            }
            ActiveFramePhase::Terminal => ActiveFrameStep::Complete(None),
        }
    }
}

fn generation_is_fresh(requested: Generation, completed: Generation) -> bool {
    requested == completed
}

impl FrameBuildHandle {
    // 🚧️ Two cfg-gated bodies (matching `os_host.rs`'s own `OsClock::new` precedent) rather than one
    // literal with a cfg-gated field inline — the same struct-literal shape, less doubt about a
    // pattern this file cannot itself compile-check today (see module doc §7 of the report).
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn new() -> Self {
        Self { runtime_in_flight: None, completion_waker: None, latest_requested_generation: Generation(0), last_submitted_generation: None, cancel: root_cancel_token(), closing: false }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn new() -> Self {
        Self { latest_requested_generation: Generation(0), last_submitted_generation: None, cancel: root_cancel_token(), closing: false, active: None }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn set_completion_waker(&mut self, waker: Arc<dyn Fn() + Send + Sync>) {
        if !self.closing {
            self.completion_waker = Some(waker);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn set_completion_waker(&mut self, _waker: Arc<dyn Fn() + Send + Sync>) {}

    #[cfg(not(target_arch = "wasm32"))]
    fn submit_active(&mut self, mut active: ActiveFrameBuild) {
        let generation = active.generation;
        let (sender, receiver) = std::sync::mpsc::channel();
        let waker = self.completion_waker.clone();
        crate::renderer_worker_pool().submit(
            Lane::Interactive,
            Box::new(move || {
                let (active, frame) = match active.step() {
                    ActiveFrameStep::Pending => (Some(active), None),
                    ActiveFrameStep::Complete(frame) => (None, frame),
                };
                let _ = sender.send(RuntimeFrameResult { generation, active, frame });
                if let Some(waker) = waker {
                    waker();
                }
            }),
        );
        self.runtime_in_flight = Some(receiver);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, dpr: f32) -> Option<crate::AppFramePresentation> {
        if self.closing {
            return None;
        }
        self.latest_requested_generation = generation;
        let mut completed = None;
        let mut returned_active = None;
        if let Some(receiver) = &self.runtime_in_flight {
            match receiver.try_recv() {
                Ok(result) => {
                    if generation_is_fresh(self.latest_requested_generation, result.generation) {
                        completed = result.frame;
                    }
                    returned_active = result.active;
                    self.runtime_in_flight = None;
                }
                Err(TryRecvError::Disconnected) => {
                    self.runtime_in_flight = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        if let Some(active) = returned_active.as_ref() {
            if !generation_is_fresh(self.latest_requested_generation, active.generation) {
                active.cancel();
            }
        }
        if let Some(active) = returned_active {
            self.submit_active(active);
            return completed;
        }
        if self.runtime_in_flight.is_none() && self.last_submitted_generation != Some(generation) {
            self.cancel = root_cancel_token();
            self.submit_active(ActiveFrameBuild::new(runtime, inputs, operation, generation, dpr, self.cancel.clone()));
            self.last_submitted_generation = Some(generation);
        }
        completed
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, dpr: f32) -> Option<crate::AppFramePresentation> {
        if self.closing || web_sys::window().is_some() {
            return None;
        }
        self.latest_requested_generation = generation;
        if let Some(active) = self.active.as_ref() {
            if !generation_is_fresh(generation, active.generation) {
                active.cancel();
            }
        }
        if let Some(active) = self.active.as_mut() {
            let active_generation = active.generation;
            let result = active.step();
            if let ActiveFrameStep::Complete(frame) = result {
                self.active = None;
                return generation_is_fresh(generation, active_generation).then_some(frame).flatten();
            }
            return None;
        }
        if self.last_submitted_generation != Some(generation) {
            self.cancel = root_cancel_token();
            self.active = Some(ActiveFrameBuild::new(runtime, inputs, operation, generation, dpr, self.cancel.clone()));
            self.last_submitted_generation = Some(generation);
        }
        None
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if !self.closing {
            self.closing = true;
            self.cancel.cancel_now();
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(receiver) = &self.runtime_in_flight {
            match receiver.try_recv() {
                Ok(mut result) => {
                    self.runtime_in_flight = None;
                    if let Some(active) = result.active.take() {
                        active.cancel();
                        self.submit_active(active);
                        return false;
                    }
                }
                Err(TryRecvError::Disconnected) => self.runtime_in_flight = None,
                Err(TryRecvError::Empty) => return false,
            }
            return false;
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(active) = self.active.as_mut() {
            active.cancel();
            if matches!(active.step(), ActiveFrameStep::Complete(_)) {
                self.active = None;
            }
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.completion_waker.take().is_some() {
            return false;
        }
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.closing && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.runtime_in_flight.is_none() && self.completion_waker.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                self.active.is_none()
            }
        }
    }
}
//#endregion 📮️FrameBuildHandle

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs(now_ms: f64) -> FrameBuildInputs {
        FrameBuildInputs { world3d_camera_dispatch_deadlines_ms: HashMap::from([("s1".to_string(), 1_000.0)]), wheel_zoom_deadline_ms: 500.0, now_ms }
    }

    fn compute(inputs: FrameBuildInputs) -> FrameDirectives {
        let mut job = FrameBuildJob::new(inputs);
        let outcome = semio_framework_job::run_to_completion(&mut job, &batch_params(OperationId(1), Generation(1), root_cancel_token()));
        assert!(matches!(outcome, StepOutcome::Complete(_)));
        job.take_directives().expect("completed directives")
    }

    #[test]
    fn not_yet_expired_deadlines_are_kept() {
        // 🐛 `inputs()` fixes `wheel_zoom_deadline_ms` at 500.0 and the world3d deadline at 1_000.0 —
        // "not yet expired" for BOTH needs `now_ms` before the earlier of the two. Caught by the
        // standalone verify crate's real `cargo test` run (`🧪️frame-job-verify`), not by inspection —
        // see `📓️p3b-frame-building.md` §7 for why this file itself cannot be `cargo test`-ed directly.
        let directives = compute(inputs(100.0));
        assert!(directives.expired_world3d_surfaces.is_empty());
        assert!(!directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn expired_deadline_is_reported() {
        let directives = compute(inputs(1_000.0));
        assert_eq!(directives.expired_world3d_surfaces, vec!["s1".to_string()]);
        assert!(directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn multiple_surfaces_expire_independently() {
        let mut inputs = inputs(1_000.0);
        inputs.world3d_camera_dispatch_deadlines_ms.insert("s2".to_string(), 5_000.0);
        let mut directives = compute(inputs);
        directives.expired_world3d_surfaces.sort();
        assert_eq!(directives.expired_world3d_surfaces, vec!["s1".to_string()]);
    }

    #[test]
    fn deadline_scan_yields_at_a_fixed_page_boundary() {
        let mut values = HashMap::new();
        for index in 0..33 {
            values.insert(format!("surface-{index}"), 1.0);
        }
        let mut job = FrameBuildJob::new(FrameBuildInputs { world3d_camera_dispatch_deadlines_ms: values, wheel_zoom_deadline_ms: 0.0, now_ms: 2.0 });
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(1), Generation(1), semio_framework_job::StepBudget::new(2_000_000, u64::MAX), root_cancel_token(), semio_framework_job::default_now_ms, &mut sequence);
        assert_eq!(job.step(&mut cx), StepOutcome::Yield);
        assert!(job.take_directives().is_none());
    }

    #[test]
    fn stale_runtime_frame_generation_is_rejected() {
        assert!(generation_is_fresh(Generation(7), Generation(7)));
        assert!(!generation_is_fresh(Generation(8), Generation(7)));
    }

    #[test]
    fn cancellation_retires_deadline_apply_and_build_phases_to_terminal_empty() {
        let mut deadlines = HashMap::new();
        for index in 0..33 {
            deadlines.insert(format!("surface-{index}"), 1.0);
        }
        let mut phases = [
            ActiveFramePhase::Deadlines(FrameBuildJob::new(FrameBuildInputs { world3d_camera_dispatch_deadlines_ms: deadlines, wheel_zoom_deadline_ms: 0.0, now_ms: 2.0 })),
            ActiveFramePhase::ApplyPending(FrameDirectives { expired_world3d_surfaces: vec!["a".repeat(256), "b".repeat(256)], wheel_zoom_deadline_cleared: false }),
            ActiveFramePhase::Build(FrameDirectives { expired_world3d_surfaces: vec!["c".repeat(256)], wheel_zoom_deadline_cleared: false }),
        ];
        for phase in &mut phases {
            for _ in 0..2_000 {
                if retire_active_phase(phase) {
                    break;
                }
            }
            assert!(retire_active_phase(phase));
        }
    }

    #[test]
    fn cancellation_retires_empty_preparation_to_terminal_empty() {
        let build =
            crate::AppFrameBuild { input: ui_wgpu::wgpu::PreparedRenderInput::new(1, 1, ui_wgpu::wgpu::DrawList::default(), None, 0.0), engine_packets: Vec::new(), cursor: ui_wgpu::wgpu::SemioCursor::Default, theme_dark: false, fullscreen: None };
        let mut phase = ActiveFramePhase::Prepare(build.into_preparation());
        for _ in 0..100 {
            if retire_active_phase(&mut phase) {
                break;
            }
        }
        assert!(retire_active_phase(&mut phase));
    }
}
