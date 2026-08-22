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
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;
#[cfg(target_arch = "wasm32")]
use semio_framework_job::run_to_completion;
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
    inputs: FrameBuildInputs,
}

impl FrameBuildJob {
    pub(crate) fn new(inputs: FrameBuildInputs) -> Self {
        Self { inputs }
    }

    /// 🔎️ Reuses `crate::sweep_expired_camera_dispatch_deadlines` (the same pure, independently-tested
    /// function `frame()` used to call inline) against a throwaway clone of the candidate map — this
    /// job only ever reads `self.inputs`, never mutates the live `AppRuntime` field directly.
    fn compute(&self) -> FrameDirectives {
        let mut candidates = self.inputs.world3d_camera_dispatch_deadlines_ms.clone();
        let expired = crate::sweep_expired_camera_dispatch_deadlines(&mut candidates, self.inputs.now_ms);
        let wheel_zoom_deadline_cleared = self.inputs.wheel_zoom_deadline_ms > 0.0 && self.inputs.now_ms >= self.inputs.wheel_zoom_deadline_ms;
        FrameDirectives { expired_world3d_surfaces: expired, wheel_zoom_deadline_cleared }
    }
}

impl InteractiveJob for FrameBuildJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        let directives = self.compute();
        cx.consume_fuel((self.inputs.world3d_camera_dispatch_deadlines_ms.len() as u64).max(1));
        let output = match serde_json::to_vec(&directives) {
            Ok(bytes) => bytes,
            Err(error) => return StepOutcome::Fault(semio_framework_job::JobFault { detail: error.to_string().into_bytes() }),
        };
        StepOutcome::Complete(CommitCandidate { state: Vec::new(), output })
    }
}

fn decode_directives(bytes: &[u8]) -> FrameDirectives {
    serde_json::from_slice(bytes).unwrap_or_default()
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
}

#[cfg(not(target_arch = "wasm32"))]
struct RuntimeFrameResult {
    generation: Generation,
    frame: Option<crate::AppFramePresentation>,
}

#[cfg(any(not(target_arch = "wasm32"), test))]
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
        Self { latest_requested_generation: Generation(0), last_submitted_generation: None, cancel: root_cancel_token(), closing: false }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn set_completion_waker(&mut self, waker: Arc<dyn Fn() + Send + Sync>) {
        self.completion_waker = Some(waker);
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn set_completion_waker(&mut self, _waker: Arc<dyn Fn() + Send + Sync>) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn poll_runtime_and_resubmit(&mut self, runtime: crate::RuntimeMailbox, inputs: FrameBuildInputs, operation: OperationId, generation: Generation, dpr: f32) -> Option<crate::AppFramePresentation> {
        if self.closing {
            return None;
        }
        self.latest_requested_generation = generation;
        let mut completed = None;
        if let Some(receiver) = &self.runtime_in_flight {
            match receiver.try_recv() {
                Ok(result) => {
                    if generation_is_fresh(self.latest_requested_generation, result.generation) {
                        completed = result.frame;
                    }
                    self.runtime_in_flight = None;
                }
                Err(TryRecvError::Disconnected) => {
                    self.runtime_in_flight = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        if self.runtime_in_flight.is_none() && self.last_submitted_generation != Some(generation) {
            self.cancel = root_cancel_token();
            let cancel = self.cancel.clone();
            let (sender, receiver) = std::sync::mpsc::channel();
            let handle = runtime.downgrade();
            let waker = self.completion_waker.clone();
            crate::renderer_worker_pool().submit(
                Lane::Interactive,
                Box::new(move || {
                    if cancel.is_cancelled_now() {
                        let _ = sender.send(RuntimeFrameResult { generation, frame: None });
                        return;
                    }
                    runtime.apply_pending();
                    let mut job = FrameBuildJob::new(inputs);
                    let directives = match semio_framework_job::run_to_completion(&mut job, &batch_params(operation, generation, cancel.clone())) {
                        StepOutcome::Complete(candidate) => decode_directives(&candidate.output),
                        _ => FrameDirectives::default(),
                    };
                    let frame = (!cancel.is_cancelled_now()).then(|| ()).and_then(|_| runtime.try_lock().ok()).and_then(|mut app| {
                        runtime.update_frame_inputs(&app);
                        app.interaction_available().then(|| app.frame(&handle, &directives, generation, dpr).prepare()).flatten()
                    });
                    let _ = sender.send(RuntimeFrameResult { generation, frame });
                    if let Some(waker) = waker {
                        waker();
                    }
                }),
            );
            self.runtime_in_flight = Some(receiver);
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
        let mut job = FrameBuildJob::new(inputs);
        let directives = match run_to_completion(&mut job, &batch_params(operation, generation, self.cancel.clone())) {
            StepOutcome::Complete(candidate) => decode_directives(&candidate.output),
            _ => FrameDirectives::default(),
        };
        runtime.apply_pending();
        let handle = runtime.downgrade();
        runtime.try_lock().ok().and_then(|mut app| {
            runtime.update_frame_inputs(&app);
            app.interaction_available().then(|| app.frame(&handle, &directives, generation, dpr).prepare()).flatten()
        })
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
                Ok(_) | Err(TryRecvError::Disconnected) => self.runtime_in_flight = None,
                Err(TryRecvError::Empty) => return false,
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
        self.closing
            && {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.runtime_in_flight.is_none() && self.completion_waker.is_none()
                }
                #[cfg(target_arch = "wasm32")]
                {
                    true
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

    #[test]
    fn not_yet_expired_deadlines_are_kept() {
        // 🐛 `inputs()` fixes `wheel_zoom_deadline_ms` at 500.0 and the world3d deadline at 1_000.0 —
        // "not yet expired" for BOTH needs `now_ms` before the earlier of the two. Caught by the
        // standalone verify crate's real `cargo test` run (`🧪️frame-job-verify`), not by inspection —
        // see `📓️p3b-frame-building.md` §7 for why this file itself cannot be `cargo test`-ed directly.
        let directives = FrameBuildJob::new(inputs(100.0)).compute();
        assert!(directives.expired_world3d_surfaces.is_empty());
        assert!(!directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn expired_deadline_is_reported() {
        let directives = FrameBuildJob::new(inputs(1_000.0)).compute();
        assert_eq!(directives.expired_world3d_surfaces, vec!["s1".to_string()]);
        assert!(directives.wheel_zoom_deadline_cleared);
    }

    #[test]
    fn multiple_surfaces_expire_independently() {
        let mut inputs = inputs(1_000.0);
        inputs.world3d_camera_dispatch_deadlines_ms.insert("s2".to_string(), 5_000.0);
        let mut directives = FrameBuildJob::new(inputs).compute();
        directives.expired_world3d_surfaces.sort();
        assert_eq!(directives.expired_world3d_surfaces, vec!["s1".to_string()]);
    }

    #[test]
    fn directives_round_trip_through_json_encoding() {
        let directives = FrameBuildJob::new(inputs(1_000.0)).compute();
        let bytes = serde_json::to_vec(&directives).unwrap();
        assert_eq!(decode_directives(&bytes), directives);
    }

    #[test]
    fn malformed_bytes_decode_to_a_safe_default_rather_than_panicking() {
        assert_eq!(decode_directives(b"not json"), FrameDirectives::default());
    }

    #[test]
    fn stale_runtime_frame_generation_is_rejected() {
        assert!(generation_is_fresh(Generation(7), Generation(7)));
        assert!(!generation_is_fresh(Generation(8), Generation(7)));
    }
}
