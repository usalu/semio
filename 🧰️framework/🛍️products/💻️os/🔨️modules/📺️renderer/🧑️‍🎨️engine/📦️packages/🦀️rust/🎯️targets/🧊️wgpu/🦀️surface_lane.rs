//! 📐️ Generation-qualified, fixed-capacity resize preparation for primary presentation surfaces.

use semio_framework_async::Lane;
use semio_framework_job::{BatchDriveConfig, BatchJobParams, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobPayloadStream, MountedWorkerJobSession, StepContext, StepOutcome, WorkerJobCloseStep};
use semio_framework_trace::{Generation, InteractiveStage, OperationId};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};

//#region 🔖️SurfaceLane

pub(crate) const SURFACE_RESIZE_LANE_CAPACITY: usize = 64;
const SURFACE_RESIZE_STEP_FUEL: u64 = 1;
const SURFACE_RESIZE_STEP_BUDGET_MS: u64 = 1;

static SURFACE_LANE_OCCUPIED: [AtomicBool; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicBool::new(false) }; SURFACE_RESIZE_LANE_CAPACITY];
static SURFACE_LANE_GENERATIONS: [AtomicU64; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicU64::new(0) }; SURFACE_RESIZE_LANE_CAPACITY];
static SURFACE_LANE_ABANDONMENT: [AtomicPtr<MountedSurfaceResizeLane>; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicPtr::new(std::ptr::null_mut()) }; SURFACE_RESIZE_LANE_CAPACITY];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SurfaceLaneToken {
    slot: u16,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SurfaceResizeRequest {
    token: SurfaceLaneToken,
    metrics_generation: u64,
    physical_width: u32,
    physical_height: u32,
    scale_factor: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PreparedSurfaceResize {
    token: SurfaceLaneToken,
    metrics_generation: u64,
    physical_width: u32,
    physical_height: u32,
    logical_width: f32,
    logical_height: f32,
    scale_factor: f32,
}

impl PreparedSurfaceResize {
    pub(crate) fn metrics_generation(self) -> u64 {
        self.metrics_generation
    }

    pub(crate) fn logical_width(self) -> f32 {
        self.logical_width
    }

    pub(crate) fn logical_height(self) -> f32 {
        self.logical_height
    }

    pub(crate) fn scale_factor(self) -> f32 {
        self.scale_factor
    }

    pub(crate) fn suspended(self) -> bool {
        self.physical_width == 0 || self.physical_height == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResizePhase {
    LogicalWidth,
    LogicalHeight,
    Seal,
    Complete,
    Closing,
}

struct SurfaceResizeJob {
    request: Option<SurfaceResizeRequest>,
    candidate: Option<PreparedSurfaceResize>,
    logical_width: f32,
    logical_height: f32,
    phase: ResizePhase,
}

impl SurfaceResizeJob {
    fn new(request: SurfaceResizeRequest) -> Self {
        Self { request: Some(request), candidate: None, logical_width: 0.0, logical_height: 0.0, phase: ResizePhase::LogicalWidth }
    }

    fn take_candidate(&mut self) -> Option<PreparedSurfaceResize> {
        self.candidate.take()
    }
}

impl InteractiveJob for SurfaceResizeJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() || cx.deadline_exceeded() {
            self.begin_close();
            return StepOutcome::Cancelled;
        }
        let Some(request) = self.request else {
            return StepOutcome::Complete(CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::Checkpoint), output: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::Commit) });
        };
        match self.phase {
            ResizePhase::LogicalWidth => {
                self.logical_width = request.physical_width as f32 / request.scale_factor.max(f32::MIN_POSITIVE);
                self.phase = ResizePhase::LogicalHeight;
            }
            ResizePhase::LogicalHeight => {
                self.logical_height = request.physical_height as f32 / request.scale_factor.max(f32::MIN_POSITIVE);
                self.phase = ResizePhase::Seal;
            }
            ResizePhase::Seal => {
                self.candidate = Some(PreparedSurfaceResize {
                    token: request.token,
                    metrics_generation: request.metrics_generation,
                    physical_width: request.physical_width,
                    physical_height: request.physical_height,
                    logical_width: self.logical_width,
                    logical_height: self.logical_height,
                    scale_factor: request.scale_factor,
                });
                self.request = None;
                self.phase = ResizePhase::Complete;
            }
            ResizePhase::Complete => {
                return StepOutcome::Complete(CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::Checkpoint), output: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::Commit) });
            }
            ResizePhase::Closing => return StepOutcome::Cancelled,
        }
        cx.consume_fuel(SURFACE_RESIZE_STEP_FUEL);
        if cx.is_cancelled() || cx.deadline_exceeded() {
            self.begin_close();
            StepOutcome::Cancelled
        } else if self.phase == ResizePhase::Complete {
            StepOutcome::Complete(CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::Checkpoint), output: semio_framework_job::RetainedJobPayload::empty(JobPayloadStream::Commit) })
        } else {
            StepOutcome::Yield
        }
    }

    fn begin_close(&mut self) {
        self.phase = ResizePhase::Closing;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.phase = ResizePhase::Closing;
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if self.candidate.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.request.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == ResizePhase::Closing && self.request.is_none() && self.candidate.is_none()
    }
}

pub(crate) struct SurfaceResizeLaneAdmissionRejected {
    operation: Option<OperationId>,
}

impl SurfaceResizeLaneAdmissionRejected {
    pub(crate) fn close_step(&mut self) -> bool {
        self.operation.take().is_some();
        self.operation.is_none()
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.operation.is_none()
    }
}

enum SurfaceResizeSession {
    Admitted(MountedWorkerJobSession<SurfaceResizeJob>),
    Rejected(semio_framework_job::WorkerJobSessionAdmissionRejected<SurfaceResizeJob>),
}

pub(crate) struct MountedSurfaceResizeLane {
    operation: Option<OperationId>,
    token: Option<SurfaceLaneToken>,
    pending: Option<SurfaceResizeRequest>,
    session: Option<SurfaceResizeSession>,
    ready: Option<PreparedSurfaceResize>,
    metrics_generation: u64,
    session_closing: bool,
    closing: bool,
}

impl MountedSurfaceResizeLane {
    pub(crate) fn try_new(operation: OperationId) -> Result<Self, SurfaceResizeLaneAdmissionRejected> {
        let Some((slot, generation)) = SURFACE_LANE_OCCUPIED.iter().enumerate().find_map(|(slot, occupied)| {
            if occupied.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
                return None;
            }
            let Some(generation) = SURFACE_LANE_GENERATIONS[slot].load(Ordering::Acquire).checked_add(1) else {
                occupied.store(false, Ordering::Release);
                return None;
            };
            SURFACE_LANE_GENERATIONS[slot].store(generation, Ordering::Release);
            Some((slot, generation))
        }) else {
            return Err(SurfaceResizeLaneAdmissionRejected { operation: Some(operation) });
        };
        Ok(Self { operation: Some(operation), token: Some(SurfaceLaneToken { slot: slot as u16, generation }), pending: None, session: None, ready: None, metrics_generation: 0, session_closing: false, closing: false })
    }

    pub(crate) fn enqueue(&mut self, physical_width: u32, physical_height: u32, scale_factor: f32) -> Result<Option<SurfaceResizeRequest>, SurfaceResizeRequest> {
        let Some(token) = self.token else {
            return Err(SurfaceResizeRequest { token: SurfaceLaneToken { slot: 0, generation: 0 }, metrics_generation: self.metrics_generation, physical_width, physical_height, scale_factor });
        };
        let Some(metrics_generation) = self.metrics_generation.checked_add(1) else {
            return Err(SurfaceResizeRequest { token, metrics_generation: self.metrics_generation, physical_width, physical_height, scale_factor });
        };
        let request = SurfaceResizeRequest { token, metrics_generation, physical_width, physical_height, scale_factor };
        if self.closing || !scale_factor.is_finite() || scale_factor <= 0.0 {
            return Err(request);
        }
        self.metrics_generation = metrics_generation;
        if let Some(session) = self.session.as_mut() {
            match session {
                SurfaceResizeSession::Admitted(session) => session.begin_close(),
                SurfaceResizeSession::Rejected(rejected) => rejected.begin_close(),
            }
            self.session_closing = true;
        }
        Ok(self.pending.replace(request))
    }

    pub(crate) fn drive_one(&mut self) {
        if (self.session_closing || matches!(self.session.as_ref(), Some(SurfaceResizeSession::Rejected(_)))) && self.drive_session_close_one() {
            return;
        }
        if let Some(session) = self.session.as_mut() {
            let SurfaceResizeSession::Admitted(session) = session else { return };
            if let Some(outcome) = session.checked_out_outcome() {
                if outcome.is_terminal() {
                    let candidate = session.checked_out_job_mut().and_then(SurfaceResizeJob::take_candidate);
                    let _ = session.take_checked_out_outcome();
                    if let Some(candidate) = candidate {
                        if self.token == Some(candidate.token) && candidate.metrics_generation == self.metrics_generation {
                            self.ready = Some(candidate);
                        }
                    }
                    session.begin_close();
                    self.session_closing = true;
                } else {
                    let _ = session.take_checked_out_outcome();
                    let _ = session.resume();
                }
                return;
            }
            let _ = session.pump_one(&crate::renderer_worker_pool(), Lane::Interactive);
            return;
        }
        let Some(request) = self.pending.take() else { return };
        let Some(operation) = self.operation else {
            self.pending = Some(request);
            return;
        };
        let cancel = semio_framework_job::root_cancel_token();
        let params = BatchJobParams {
            operation,
            generation: Generation(request.metrics_generation),
            cancel,
            config: BatchDriveConfig { site: "os_renderer_surface_resize", stage: InteractiveStage::InteractiveStep, fuel_per_step: SURFACE_RESIZE_STEP_FUEL, step_budget_ms: SURFACE_RESIZE_STEP_BUDGET_MS },
            now_ms: semio_framework_job::default_now_ms,
        };
        self.session = Some(match MountedWorkerJobSession::try_new(SurfaceResizeJob::new(request), params) {
            Ok(session) => SurfaceResizeSession::Admitted(session),
            Err(mut rejected) => {
                rejected.begin_close();
                SurfaceResizeSession::Rejected(rejected)
            }
        });
        self.session_closing = matches!(self.session.as_ref(), Some(SurfaceResizeSession::Rejected(_)));
    }

    fn drive_session_close_one(&mut self) -> bool {
        let Some(session) = self.session.as_mut() else { return false };
        let complete = match session {
            SurfaceResizeSession::Admitted(session) => {
                if matches!(session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), WorkerJobCloseStep::Complete) {
                    session.terminal_is_empty()
                } else {
                    false
                }
            }
            SurfaceResizeSession::Rejected(rejected) => {
                if matches!(rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Complete) {
                    rejected.terminal_is_empty()
                } else {
                    false
                }
            }
        };
        if complete {
            self.session = None;
            self.session_closing = false;
        }
        true
    }

    pub(crate) fn take_ready(&mut self) -> Option<PreparedSurfaceResize> {
        self.ready.take().filter(|candidate| self.token == Some(candidate.token) && candidate.metrics_generation == self.metrics_generation)
    }

    pub(crate) fn restore_ready(&mut self, candidate: PreparedSurfaceResize) -> Result<(), PreparedSurfaceResize> {
        if self.ready.is_some() || self.token != Some(candidate.token) || candidate.metrics_generation != self.metrics_generation {
            return Err(candidate);
        }
        self.ready = Some(candidate);
        Ok(())
    }

    pub(crate) fn has_work(&self) -> bool {
        self.pending.is_some() || self.session.is_some() || self.ready.is_some()
    }

    pub(crate) fn begin_close(&mut self) {
        self.closing = true;
        if let Some(session) = self.session.as_mut() {
            match session {
                SurfaceResizeSession::Admitted(session) => session.begin_close(),
                SurfaceResizeSession::Rejected(rejected) => rejected.begin_close(),
            }
            self.session_closing = true;
        }
    }

    pub(crate) fn close_step(&mut self) -> bool {
        self.begin_close();
        if self.drive_session_close_one() {
            return false;
        }
        if self.ready.take().is_some() {
            return false;
        }
        if self.pending.take().is_some() {
            return false;
        }
        if let Some(token) = self.token.take() {
            let slot = token.slot as usize;
            if SURFACE_LANE_GENERATIONS[slot].load(Ordering::Acquire) == token.generation {
                SURFACE_LANE_OCCUPIED[slot].store(false, Ordering::Release);
            }
            return false;
        }
        self.operation.take();
        self.terminal_is_empty()
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.operation.is_none() && self.token.is_none() && self.pending.is_none() && self.session.is_none() && self.ready.is_none()
    }

    pub(crate) fn close_abandoned_step() -> bool {
        let Some((slot, pointer)) = SURFACE_LANE_ABANDONMENT.iter().enumerate().find_map(|(slot, owner)| {
            let pointer = owner.swap(std::ptr::null_mut(), Ordering::AcqRel);
            (!pointer.is_null()).then_some((slot, pointer))
        }) else {
            return true;
        };
        let mut lane = unsafe { Box::from_raw(pointer) };
        if lane.close_step() && lane.terminal_is_empty() {
            drop(lane);
        } else {
            SURFACE_LANE_ABANDONMENT[slot].store(Box::into_raw(lane), Ordering::Release);
        }
        false
    }
}

pub(crate) enum SurfaceResizeAuthority {
    Mounted(MountedSurfaceResizeLane),
    Rejected(SurfaceResizeLaneAdmissionRejected),
}

impl SurfaceResizeAuthority {
    pub(crate) fn new(operation: OperationId) -> Self {
        match MountedSurfaceResizeLane::try_new(operation) {
            Ok(lane) => Self::Mounted(lane),
            Err(rejected) => Self::Rejected(rejected),
        }
    }

    pub(crate) fn enqueue(&mut self, physical_width: u32, physical_height: u32, scale_factor: f32) -> Result<Option<SurfaceResizeRequest>, SurfaceResizeRequest> {
        match self {
            Self::Mounted(lane) => lane.enqueue(physical_width, physical_height, scale_factor),
            Self::Rejected(_) => Err(SurfaceResizeRequest { token: SurfaceLaneToken { slot: 0, generation: 0 }, metrics_generation: 0, physical_width, physical_height, scale_factor }),
        }
    }

    pub(crate) fn drive_one(&mut self) {
        if let Self::Mounted(lane) = self {
            lane.drive_one();
        }
    }

    pub(crate) fn take_ready(&mut self) -> Option<PreparedSurfaceResize> {
        match self {
            Self::Mounted(lane) => lane.take_ready(),
            Self::Rejected(_) => None,
        }
    }

    pub(crate) fn restore_ready(&mut self, candidate: PreparedSurfaceResize) -> Result<(), PreparedSurfaceResize> {
        match self {
            Self::Mounted(lane) => lane.restore_ready(candidate),
            Self::Rejected(_) => Err(candidate),
        }
    }

    pub(crate) fn has_work(&self) -> bool {
        match self {
            Self::Mounted(lane) => lane.has_work(),
            Self::Rejected(rejected) => !rejected.terminal_is_empty(),
        }
    }

    pub(crate) fn begin_close(&mut self) {
        if let Self::Mounted(lane) = self {
            lane.begin_close();
        }
    }

    pub(crate) fn close_step(&mut self) -> bool {
        match self {
            Self::Mounted(lane) => lane.close_step(),
            Self::Rejected(rejected) => rejected.close_step(),
        }
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Mounted(lane) => lane.terminal_is_empty(),
            Self::Rejected(rejected) => rejected.terminal_is_empty(),
        }
    }
}

impl Drop for MountedSurfaceResizeLane {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        self.begin_close();
        let Some(token) = self.token else {
            debug_assert!(false, "surface resize lane without a token requires incremental close before drop");
            return;
        };
        let slot = token.slot as usize;
        if !SURFACE_LANE_ABANDONMENT[slot].load(Ordering::Acquire).is_null() {
            debug_assert!(false, "surface resize abandonment slot already owns its exact generation");
            return;
        }
        let lane = Box::new(Self {
            operation: self.operation.take(),
            token: self.token.take(),
            pending: self.pending.take(),
            session: self.session.take(),
            ready: self.ready.take(),
            metrics_generation: self.metrics_generation,
            session_closing: true,
            closing: true,
        });
        SURFACE_LANE_ABANDONMENT[slot].store(Box::into_raw(lane), Ordering::Release);
    }
}

//#endregion 🔖️SurfaceLane

#[cfg(test)]
mod tests {
    use super::*;

    fn request(generation: u64, width: u32) -> SurfaceResizeRequest {
        SurfaceResizeRequest { token: SurfaceLaneToken { slot: 0, generation: 1 }, metrics_generation: generation, physical_width: width, physical_height: 720, scale_factor: 2.0 }
    }

    #[test]
    fn resize_job_consumes_one_scalar_per_grant() {
        let mut job = SurfaceResizeJob::new(request(1, 1280));
        let operation = semio_framework_job::allocate_operation_id();
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut preview_sequence = 0;
        let mut cx = StepContext::new(operation, Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, semio_framework_job::default_now_ms, &mut preview_sequence);
        assert_eq!(job.step(&mut cx), StepOutcome::Yield);
        assert_eq!(job.phase, ResizePhase::LogicalHeight);
        assert_eq!(job.logical_width, 640.0);
        assert_eq!(job.logical_height, 0.0);
    }

    #[test]
    fn million_resize_samples_retain_only_the_latest_exact_request() {
        let operation = semio_framework_job::allocate_operation_id();
        let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
            Ok(lane) => lane,
            Err(_) => panic!("fixed resize lane must admit the first test surface"),
        };
        for width in 1..=1_000_000 {
            let result = lane.enqueue(width, 720, 2.0);
            assert!(result.is_ok());
        }
        assert_eq!(lane.metrics_generation, 1_000_000);
        assert!(lane.pending.is_some_and(|pending| pending.metrics_generation == 1_000_000 && pending.physical_width == 1_000_000 && pending.physical_height == 720 && pending.scale_factor == 2.0));
        for _ in 0..8 {
            if lane.close_step() {
                break;
            }
        }
        assert!(lane.terminal_is_empty());
    }

    #[test]
    fn zero_size_suspends_and_invalid_scale_returns_exact_producer() {
        let operation = semio_framework_job::allocate_operation_id();
        let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
            Ok(lane) => lane,
            Err(_) => panic!("fixed resize lane must admit the first test surface"),
        };
        assert!(lane.enqueue(0, 0, 2.0).is_ok());
        let rejected = match lane.enqueue(1280, 720, f32::NAN) {
            Ok(_) => panic!("non-finite scale must be rejected"),
            Err(rejected) => rejected,
        };
        assert_eq!(rejected.physical_width, 1280);
        assert!(rejected.scale_factor.is_nan());
        for _ in 0..8 {
            if lane.close_step() {
                break;
            }
        }
        assert!(lane.terminal_is_empty());
    }

    #[test]
    fn interrupted_lane_drop_is_rediscovered_and_incrementally_closed() {
        let operation = semio_framework_job::allocate_operation_id();
        let mut lane = match MountedSurfaceResizeLane::try_new(operation) {
            Ok(lane) => lane,
            Err(_) => panic!("fixed resize lane must admit the first test surface"),
        };
        assert!(lane.enqueue(1280, 720, 2.0).is_ok());
        let token = lane.token;
        drop(lane);
        for _ in 0..8 {
            if MountedSurfaceResizeLane::close_abandoned_step() {
                break;
            }
        }
        let token = match token {
            Some(token) => token,
            None => panic!("admitted lane owns a token"),
        };
        assert!(!SURFACE_LANE_OCCUPIED[token.slot as usize].load(Ordering::Acquire));
    }
}
