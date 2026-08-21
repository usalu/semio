//! 🧵️ INTERACTIVE-JOB-RUNTIME-REFACTOR packet P3b (Phase 3, ui-thread-isolation) — the real, if
//! deliberately narrow, `semio_framework_job::InteractiveJob` seam for `AppRuntime::frame()`.
//!
//! **What this genuinely moves off the UI thread.** `frame()`'s World3D wheel-zoom settle sweep
//! (scanning `world3d_camera_dispatch_deadlines_ms` for expired entries) and its node-graph wheel-zoom
//! deadline check are pure arithmetic over owned, `Send` values with no `Rc`/`RefCell`/GPU/thread-local
//! involvement. [`FrameBuildJob`] runs that scan as a real [`semio_framework_job::InteractiveJob`],
//! submitted onto `crate::renderer_worker_pool()` via [`semio_framework_job::run_on_worker`] (native) —
//! a genuine job, on the real pool, not a synchronous call dressed up to look like one. The scan is
//! O(open deadlines); a document with many live World3D viewports is the case this is worth doing for.
//! **`frame()` still re-validates every candidate the job reports against LIVE state before acting on
//! it** (see `📓️p3b-frame-building.md` §4) — the worker's output is a candidate list, not an
//! authoritative replacement, so a stale result (the job is one or more frames behind) can only cause a
//! harmless one-tick delay, never a dropped or duplicated dispatch. Deliberately excludes the caret-
//! blink toggle the earlier draft of this job also computed: that state is a RELATIVE timer
//! (toggle-if-≥500ms-elapsed), and re-validating a relative toggle against a stale snapshot can silently
//! under- or double-toggle across a stall in a way the World3D scan's re-validation cannot — moving it
//! off-thread safely needs an absolute "next flip due at" schedule instead, left to Phase 5.
//!
//! **What this does NOT move — read `📓️p3b-frame-building.md` before assuming more.** The expensive
//! part of "building" — `shell::ShellState::render_chrome`'s layout/text-shaping/tessellation — is
//! excluded on purpose: it takes `&mut ui_wgpu::wgpu::GpuContext` directly (lazy glyph/raster texture
//! upload happens mid-layout, not after), and it reads/writes upwards of a dozen `thread_local!` UI
//! caches in `🧱️elements/Shell/🧊️component.rs` (tooltip hover, dialog stack, tour state, prefs, find
//! items, boot-hub env, content focus) that are genuinely per-OS-thread storage — moving that call to a
//! worker thread would silently read/write a DIFFERENT, empty set of thread-locals than the UI thread,
//! not just a `!Send` compile error. Neither blocker is fixable inside this packet's boundary or risk
//! budget (the first needs a `ui_wgpu` seam change, outside `🖱️ui/🖥️host/**`+`📺️renderer/**`; the
//! second needs auditing/threading through ~15 ambient statics across an 11,000-line file). This job is
//! the seam Phase 5 plugs the real extraction into once those are resolved — not a finished migration.
//!
//! **Platform constraint.** `crate::renderer_worker_pool()` is native-only
//! (`#[cfg(not(target_arch = "wasm32"))]`) — wasm32 has no second OS thread in this crate's model, so
//! [`FrameBuildHandle::poll_and_resubmit`] runs the identical [`InteractiveJob`] to completion inline,
//! synchronously, via [`semio_framework_job::run_to_completion`] on that target. This is not a gap: a
//! "worker" is meaningless when there is no second thread to put it on, and the job protocol's own
//! `run_to_completion`/`run_on_worker` split exists precisely so both paths drive the identical impl
//! (design ticket packet P2a item 6).

use semio_framework_job::{root_cancel_token, BatchDriveConfig, BatchJobParams, CommitCandidate, InteractiveJob, StepContext, StepOutcome, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_MS};
use semio_framework_trace::{Generation, InteractiveStage, OperationId};
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;
#[cfg(target_arch = "wasm32")]
use semio_framework_job::run_to_completion;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{Receiver, TryRecvError};

//#region 📥️FrameBuildInputs
/// 📥️ The `Send`-safe slice of `AppRuntime` this job needs — cloned out of `self` once per submission
/// (all `f64`/`bool`/small `HashMap<String, f64>`, cheap to copy; never the `Rc<RefCell<AppRuntime>>`
/// itself, never `ShellState`, never `GpuContext`).
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

fn batch_params(operation: OperationId, generation: Generation) -> BatchJobParams {
    BatchJobParams {
        operation,
        generation,
        cancel: root_cancel_token(),
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
    in_flight: Option<Receiver<StepOutcome>>,
    last: FrameDirectives,
}

impl FrameBuildHandle {
    // 🚧️ Two cfg-gated bodies (matching `os_host.rs`'s own `OsClock::new` precedent) rather than one
    // literal with a cfg-gated field inline — the same struct-literal shape, less doubt about a
    // pattern this file cannot itself compile-check today (see module doc §7 of the report).
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn new() -> Self {
        Self { in_flight: None, last: FrameDirectives::default() }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn new() -> Self {
        Self { last: FrameDirectives::default() }
    }

    /// 🔁️ Native: drains the in-flight job's `Receiver` with `try_recv` (never `recv`), adopts its
    /// result if `Complete`, then submits a fresh job for `inputs` only if nothing is currently in
    /// flight. Wasm32: no pool exists (see module doc) — runs the job to completion inline and returns
    /// its result immediately, same signature, so `winit_app.rs`'s call site never branches on target.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn poll_and_resubmit(&mut self, inputs: FrameBuildInputs, operation: OperationId, generation: Generation) -> FrameDirectives {
        if let Some(receiver) = &self.in_flight {
            match receiver.try_recv() {
                Ok(StepOutcome::Complete(candidate)) => {
                    self.last = decode_directives(&candidate.output);
                    self.in_flight = None;
                }
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    self.in_flight = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        if self.in_flight.is_none() {
            let pool = crate::renderer_worker_pool();
            let receiver = semio_framework_job::run_on_worker(&pool, Lane::Interactive, FrameBuildJob::new(inputs), batch_params(operation, generation));
            self.in_flight = Some(receiver);
        }
        self.last.clone()
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn poll_and_resubmit(&mut self, inputs: FrameBuildInputs, operation: OperationId, generation: Generation) -> FrameDirectives {
        let mut job = FrameBuildJob::new(inputs);
        if let StepOutcome::Complete(candidate) = run_to_completion(&mut job, &batch_params(operation, generation)) {
            self.last = decode_directives(&candidate.output);
        }
        self.last.clone()
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
}
