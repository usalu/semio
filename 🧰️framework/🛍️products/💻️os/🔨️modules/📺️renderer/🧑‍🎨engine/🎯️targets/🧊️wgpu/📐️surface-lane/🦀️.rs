//! 📐️ Generation-qualified, fixed-capacity resize preparation for primary presentation surfaces.

use semio_framework_async::{CancelTokenRetirement, CancelTokenRetirementError, Lane};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, OriginalTurnFailure, RetainedCloneProgress, RetainedCloneStep, StepContext};
use semio_framework_job_worker::{MountedWorkerJobSession, WorkerJobDemandError, WorkerJobPoll};
use semio_framework_job_worker_authority::{BatchDriveConfig, BatchJobParams};
use semio_framework_trace::{InteractiveStage, OperationId};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};

//#region 🔖️SurfaceLane

pub const SURFACE_RESIZE_LANE_CAPACITY: usize = 64;
const SURFACE_RESIZE_STEP_FUEL: u64 = 1;
const SURFACE_RESIZE_STEP_BUDGET_US: u64 = 1_000;

static SURFACE_LANE_OCCUPIED: [AtomicBool; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicBool::new(false) }; SURFACE_RESIZE_LANE_CAPACITY];
static SURFACE_LANE_GENERATIONS: [AtomicU64; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicU64::new(0) }; SURFACE_RESIZE_LANE_CAPACITY];
static SURFACE_LANE_ABANDONMENT: [AtomicPtr<MountedSurfaceResizeLane>; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicPtr::new(std::ptr::null_mut()) }; SURFACE_RESIZE_LANE_CAPACITY];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceLaneToken {
    slot: u16,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceResizeRequest {
    token: SurfaceLaneToken,
    metrics_generation: u64,
    physical_width: u32,
    physical_height: u32,
    scale_factor: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreparedSurfaceResize {
    token: SurfaceLaneToken,
    metrics_generation: u64,
    physical_width: u32,
    physical_height: u32,
    logical_width: f32,
    logical_height: f32,
    scale_factor: f32,
}

impl PreparedSurfaceResize {
    pub fn metrics_generation(self) -> u64 {
        self.metrics_generation
    }

    pub fn logical_width(self) -> f32 {
        self.logical_width
    }

    pub fn logical_height(self) -> f32 {
        self.logical_height
    }

    pub fn scale_factor(self) -> f32 {
        self.scale_factor
    }

    pub fn suspended(self) -> bool {
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
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if cx.is_cancelled() || self.phase == ResizePhase::Closing {
            self.begin_close();
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from);
        }
        let Some(request) = self.request else {
            return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(OriginalTurnFailure::from);
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
            ResizePhase::Complete => return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(OriginalTurnFailure::from),
            ResizePhase::Closing => return JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from),
        }
        cx.consume_fuel(SURFACE_RESIZE_STEP_FUEL);
        if cx.is_cancelled() {
            self.begin_close();
            JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from)
        } else if self.phase == ResizePhase::Complete {
            JobOutcomeBorrow::admit_complete(cx, None, None).map_err(OriginalTurnFailure::from)
        } else {
            JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "surface resize publishes only yield, completion or cancellation")),
        }
    }

    fn begin_close(&mut self) {
        self.phase = ResizePhase::Closing;
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.phase = ResizePhase::Closing;
        let copy_bytes = self.next_close_copy_byte_demand()?;
        if copy_bytes == 0 {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy_bytes {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..RetainedCloneProgress::default() };
        cx.consume_retained(progress)?;
        if self.candidate.take().is_none() {
            self.request = None;
        }
        cx.consume_fuel(1);
        Ok(InteractiveJobCloseStep::Pending { progress })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(if self.candidate.is_some() {
            size_of::<PreparedSurfaceResize>()
        } else if self.request.is_some() {
            size_of::<SurfaceResizeRequest>()
        } else {
            0
        })
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(usize::from(self.candidate.is_some() || self.request.is_some()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.phase == ResizePhase::Closing && self.request.is_none() && self.candidate.is_none()
    }
}

pub struct SurfaceResizeLaneAdmissionRejected {
    operation: Option<OperationId>,
}

impl SurfaceResizeLaneAdmissionRejected {
    pub fn close_step(&mut self) -> bool {
        self.operation.take();
        self.operation.is_none()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation.is_none()
    }
}

enum SurfaceResizeStep {
    Working,
    Finished(Option<PreparedSurfaceResize>),
}

/// 🎟️ One resize session under the initiating frame owner: the unadmitted job, parameters and cancellation alias, then the admitted mounted session with its paid outcome custody.
struct SurfaceResizeSession {
    job: Option<SurfaceResizeJob>,
    params: Option<BatchJobParams>,
    params_retirement: Option<CancelTokenRetirement>,
    mounted: Option<MountedWorkerJobSession<SurfaceResizeJob>>,
    receipt_taken: bool,
    acknowledged: bool,
    outcome_kind: Option<JobOutcomeKind>,
}

impl SurfaceResizeSession {
    fn admit_request(request: SurfaceResizeRequest, cx: &mut StepContext<'_>) -> Result<Option<Self>, OriginalTurnFailure> {
        let Some(now) = cx.now_us() else { return Ok(None) };
        let Some(cancel) = cx.admit_original_cancel_alias()? else { return Ok(None) };
        let params = BatchJobParams {
            operation: cx.operation(),
            generation: cx.generation(),
            cancel,
            now_us: cx.clock_source(),
            config: BatchDriveConfig {
                retained: cx.retained_grant(),
                site: "os_renderer_surface_resize",
                stage: InteractiveStage::InteractiveStep,
                fuel_per_step: cx.fuel_remaining().min(SURFACE_RESIZE_STEP_FUEL),
                step_budget_us: cx.deadline_us().saturating_sub(now).min(SURFACE_RESIZE_STEP_BUDGET_US),
            },
        };
        Ok(Some(Self { job: Some(SurfaceResizeJob::new(request)), params: Some(params), params_retirement: None, mounted: None, receipt_taken: false, acknowledged: false, outcome_kind: None }))
    }

    fn begin_close(&mut self) {
        if let Some(mounted) = self.mounted.as_mut() {
            mounted.begin_close();
        } else if let Some(job) = self.job.as_mut() {
            job.begin_close();
        }
    }

    fn drive(&mut self, cx: &mut StepContext<'_>) -> Result<SurfaceResizeStep, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(SurfaceResizeStep::Working);
        }
        let Some(mounted) = self.mounted.as_mut() else {
            if self.job.is_some() && self.params.is_some() {
                if let Some((mounted, _posted)) = MountedWorkerJobSession::try_admit_owned(&mut self.job, &mut self.params, &mut *cx)? {
                    self.mounted = Some(mounted);
                }
            }
            return Ok(SurfaceResizeStep::Working);
        };
        if !matches!(mounted.poll(), WorkerJobPoll::CheckedOut) {
            let _ = mounted.pump_one(&semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool(), Lane::Interactive, grant);
            return Ok(SurfaceResizeStep::Working);
        }
        self.receive(cx)
    }

    fn receive(&mut self, cx: &mut StepContext<'_>) -> Result<SurfaceResizeStep, OriginalTurnFailure> {
        for _ in 0..4 {
            let grant = cx.retained_grant();
            if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
                break;
            }
            let Some(mounted) = self.mounted.as_mut() else { break };
            if !self.receipt_taken {
                let received = mounted.take_checked_out_retained_step_receipt().filter(|(issued, progress)| progress.fits(*issued) && progress.fits(grant));
                let Some((_issued, progress)) = received else {
                    mounted.begin_close();
                    self.acknowledged = false;
                    self.outcome_kind = None;
                    return Ok(SurfaceResizeStep::Finished(None));
                };
                cx.consume_retained(progress)?;
                self.outcome_kind = mounted.checked_out_outcome_descriptor().map(JobOutcomeDescriptor::kind);
                self.receipt_taken = true;
                continue;
            }
            if !self.acknowledged {
                let step = mounted.acknowledge_checked_out_outcome(grant);
                cx.consume_retained(step.progress())?;
                cx.consume_fuel(step.progress().copied_items as u64);
                self.acknowledged = matches!(step, RetainedCloneStep::Complete(_));
                continue;
            }
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..RetainedCloneProgress::default() })?;
            cx.consume_fuel(1);
            let kind = self.outcome_kind.take();
            self.receipt_taken = false;
            self.acknowledged = false;
            return Ok(match kind {
                Some(JobOutcomeKind::Complete) => {
                    let candidate = mounted.checked_out_job_mut().and_then(SurfaceResizeJob::take_candidate);
                    mounted.begin_close();
                    SurfaceResizeStep::Finished(candidate)
                }
                None | Some(JobOutcomeKind::Yield) => {
                    if mounted.resume().is_err() {
                        mounted.begin_close();
                        SurfaceResizeStep::Finished(None)
                    } else {
                        SurfaceResizeStep::Working
                    }
                }
                Some(_) => {
                    mounted.begin_close();
                    SurfaceResizeStep::Finished(None)
                }
            });
        }
        Ok(SurfaceResizeStep::Working)
    }

    fn pay_item(cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(false);
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..RetainedCloneProgress::default() })?;
        cx.consume_fuel(1);
        Ok(true)
    }

    fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none() && self.params_retirement.is_none() && self.mounted.is_none()
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if cx.retirement_should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(false);
        }
        if let Some(mounted) = self.mounted.as_mut() {
            match mounted.poll() {
                WorkerJobPoll::Outcome | WorkerJobPoll::Terminal | WorkerJobPoll::Rejected => {
                    let _ = mounted.pump_one(&semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool(), Lane::Interactive, grant);
                }
                WorkerJobPoll::CheckedOut => {
                    if let Some((issued, progress)) = mounted.take_checked_out_retained_step_receipt() {
                        if !progress.fits(issued) || !progress.fits(grant) {
                            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "surface resize worker receipt exceeds its original receiving authority").with_retained_progress(progress).into());
                        }
                        cx.consume_retained(progress)?;
                    } else {
                        mounted.close_step(cx)?;
                    }
                }
                WorkerJobPoll::TerminalEmpty => {
                    if Self::pay_item(cx)? {
                        self.mounted = None;
                    }
                }
                _ => {
                    mounted.begin_close();
                    match mounted.has_original_cancel_alias_witness(cx.original_cancel_token()) {
                        Ok(true) => match mounted.return_original_cancel_alias_step(cx.original_cancel_token(), grant) {
                            Ok(Some(step)) => {
                                cx.consume_retained(step.progress())?;
                                cx.consume_fuel(step.progress().copied_items as u64);
                            }
                            Ok(None) | Err(WorkerJobDemandError::Contention(_)) => {}
                            Err(WorkerJobDemandError::Refused(error)) => return Err(OriginalTurnFailure::debit_original(error, cx)),
                        },
                        Ok(false) => {
                            mounted.close_step(cx)?;
                        }
                        Err(WorkerJobDemandError::Contention(_)) => {}
                        Err(WorkerJobDemandError::Refused(error)) => return Err(OriginalTurnFailure::debit_original(error, cx)),
                    }
                }
            }
            return Ok(false);
        }
        if let Some(job) = self.job.as_mut() {
            job.begin_close();
            if !job.terminal_is_empty() {
                job.close_step(cx)?;
            } else if Self::pay_item(cx)? {
                self.job = None;
            }
            return Ok(false);
        }
        if let Some(params) = self.params.take() {
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..RetainedCloneProgress::default() })?;
            cx.consume_fuel(1);
            self.params_retirement = Some(CancelTokenRetirement::from_token(params.cancel));
            return Ok(false);
        }
        if let Some(owner) = self.params_retirement.as_mut() {
            if !owner.terminal_is_empty() {
                match owner.return_alias_step(cx.original_cancel_token(), grant) {
                    Ok(step) => {
                        cx.consume_retained(step.progress())?;
                        cx.consume_fuel(step.progress().copied_items as u64);
                    }
                    Err(CancelTokenRetirementError::Blocked(_)) => {}
                    Err(CancelTokenRetirementError::Refused(error)) => return Err(OriginalTurnFailure::debit_original(error, cx)),
                }
                return Ok(false);
            }
            if Self::pay_item(cx)? {
                self.params_retirement = None;
            }
            return Ok(false);
        }
        Ok(true)
    }
}

pub struct MountedSurfaceResizeLane {
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
    pub fn try_new(operation: OperationId) -> Result<Self, SurfaceResizeLaneAdmissionRejected> {
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

    pub fn enqueue(&mut self, physical_width: u32, physical_height: u32, scale_factor: f32) -> Result<Option<SurfaceResizeRequest>, SurfaceResizeRequest> {
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
            session.begin_close();
            self.session_closing = true;
        }
        Ok(self.pending.replace(request))
    }

    pub fn drive_one(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.session_closing && self.drive_session_close_one(cx)? {
            return Ok(());
        }
        if let Some(session) = self.session.as_mut() {
            if let SurfaceResizeStep::Finished(candidate) = session.drive(cx)? {
                if let Some(candidate) = candidate {
                    if self.token == Some(candidate.token) && candidate.metrics_generation == self.metrics_generation {
                        self.ready = Some(candidate);
                    }
                }
                self.session_closing = true;
            }
            return Ok(());
        }
        let Some(request) = self.pending else { return Ok(()) };
        if cx.should_yield() {
            return Ok(());
        }
        if let Some(session) = SurfaceResizeSession::admit_request(request, cx)? {
            self.pending = None;
            self.session = Some(session);
            self.session_closing = false;
        }
        Ok(())
    }

    fn drive_session_close_one(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let Some(session) = self.session.as_mut() else {
            self.session_closing = false;
            return Ok(false);
        };
        if session.close_step(cx)? && session.terminal_is_empty() {
            self.session = None;
            self.session_closing = false;
        }
        Ok(true)
    }

    pub fn take_ready(&mut self) -> Option<PreparedSurfaceResize> {
        self.ready.take().filter(|candidate| self.token == Some(candidate.token) && candidate.metrics_generation == self.metrics_generation)
    }

    pub fn restore_ready(&mut self, candidate: PreparedSurfaceResize) -> Result<(), PreparedSurfaceResize> {
        if self.ready.is_some() || self.token != Some(candidate.token) || candidate.metrics_generation != self.metrics_generation {
            return Err(candidate);
        }
        self.ready = Some(candidate);
        Ok(())
    }

    pub fn has_work(&self) -> bool {
        self.pending.is_some() || self.session.is_some() || self.ready.is_some()
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        if let Some(session) = self.session.as_mut() {
            session.begin_close();
            self.session_closing = true;
        }
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.begin_close();
        if self.drive_session_close_one(cx)? {
            return Ok(false);
        }
        if self.ready.take().is_some() {
            return Ok(false);
        }
        if self.pending.take().is_some() {
            return Ok(false);
        }
        if let Some(token) = self.token.take() {
            let slot = token.slot as usize;
            if SURFACE_LANE_GENERATIONS[slot].load(Ordering::Acquire) == token.generation {
                SURFACE_LANE_OCCUPIED[slot].store(false, Ordering::Release);
            }
            return Ok(false);
        }
        self.operation.take();
        Ok(self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation.is_none() && self.token.is_none() && self.pending.is_none() && self.session.is_none() && self.ready.is_none()
    }

    pub fn close_abandoned_step(cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let Some((slot, pointer)) = SURFACE_LANE_ABANDONMENT.iter().enumerate().find_map(|(slot, owner)| {
            let pointer = owner.swap(std::ptr::null_mut(), Ordering::AcqRel);
            (!pointer.is_null()).then_some((slot, pointer))
        }) else {
            return Ok(true);
        };
        let mut lane = unsafe { Box::from_raw(pointer) };
        let closed = lane.close_step(cx);
        if matches!(closed, Ok(true)) && lane.terminal_is_empty() {
            drop(lane);
        } else {
            SURFACE_LANE_ABANDONMENT[slot].store(Box::into_raw(lane), Ordering::Release);
        }
        closed.map(|_| false)
    }
}

pub enum SurfaceResizeAuthority {
    Mounted(MountedSurfaceResizeLane),
    Rejected(SurfaceResizeLaneAdmissionRejected),
}

impl SurfaceResizeAuthority {
    pub fn new(operation: OperationId) -> Self {
        match MountedSurfaceResizeLane::try_new(operation) {
            Ok(lane) => Self::Mounted(lane),
            Err(rejected) => Self::Rejected(rejected),
        }
    }

    pub fn enqueue(&mut self, physical_width: u32, physical_height: u32, scale_factor: f32) -> Result<Option<SurfaceResizeRequest>, SurfaceResizeRequest> {
        match self {
            Self::Mounted(lane) => lane.enqueue(physical_width, physical_height, scale_factor),
            Self::Rejected(_) => Err(SurfaceResizeRequest { token: SurfaceLaneToken { slot: 0, generation: 0 }, metrics_generation: 0, physical_width, physical_height, scale_factor }),
        }
    }

    pub fn drive_one(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        match self {
            Self::Mounted(lane) => lane.drive_one(cx),
            Self::Rejected(_) => Ok(()),
        }
    }

    pub fn take_ready(&mut self) -> Option<PreparedSurfaceResize> {
        match self {
            Self::Mounted(lane) => lane.take_ready(),
            Self::Rejected(_) => None,
        }
    }

    pub fn restore_ready(&mut self, candidate: PreparedSurfaceResize) -> Result<(), PreparedSurfaceResize> {
        match self {
            Self::Mounted(lane) => lane.restore_ready(candidate),
            Self::Rejected(_) => Err(candidate),
        }
    }

    pub fn has_work(&self) -> bool {
        match self {
            Self::Mounted(lane) => lane.has_work(),
            Self::Rejected(rejected) => !rejected.terminal_is_empty(),
        }
    }

    pub fn begin_close(&mut self) {
        if let Self::Mounted(lane) = self {
            lane.begin_close();
        }
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        match self {
            Self::Mounted(lane) => lane.close_step(cx),
            Self::Rejected(rejected) => Ok(rejected.close_step()),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
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
            debug_assert!(std::thread::panicking(), "surface resize lane without a token requires incremental close before drop");
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
#[path = "../../../🧪️tests/🔬️wgpu-surface-lane-unit/🦀️.rs"]
mod tests;
