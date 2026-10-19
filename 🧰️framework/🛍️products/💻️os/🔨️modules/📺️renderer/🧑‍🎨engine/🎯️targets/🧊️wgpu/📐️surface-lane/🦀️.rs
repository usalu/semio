//! 📐️ Generation-qualified, fixed-capacity resize preparation for primary presentation surfaces.

use semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneStep, RetainedCloneProgress, RetirementDemand, ValueError};
use semio_framework_job_worker::MountedWorkerJobSession;
use semio_framework_job_worker_authority::{BatchDriveConfig, WorkerJobCloseStep};
use semio_framework_trace::{InteractiveStage, OperationId};
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

//#region 🔖️SurfaceLane

pub const SURFACE_RESIZE_LANE_CAPACITY: usize = 64;
const SURFACE_RESIZE_STEP_FUEL: u64 = 1;
const SURFACE_RESIZE_STEP_BUDGET_US: u64 = 1_000;

static SURFACE_LANE_OCCUPIED: [AtomicBool; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicBool::new(false) }; SURFACE_RESIZE_LANE_CAPACITY];
static SURFACE_LANE_GENERATIONS: [AtomicU64; SURFACE_RESIZE_LANE_CAPACITY] = [const { AtomicU64::new(0) }; SURFACE_RESIZE_LANE_CAPACITY];

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
        if self.phase == ResizePhase::Closing || cx.is_cancelled() { self.begin_close(); return JobOutcomeBorrow::admit_cancelled(cx).map_err(OriginalTurnFailure::from); }
        if cx.should_yield() { return Ok(None); }
        let Some(request) = self.request else {
            return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into);
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
                return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into);
            }
            ResizePhase::Closing => return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into),
        }
        cx.consume_fuel(SURFACE_RESIZE_STEP_FUEL);
        if cx.is_cancelled() {
            self.begin_close();
            JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)
        } else if self.phase == ResizePhase::Complete {
            JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into)
        } else {
            JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)
        }

    }
    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.phase == ResizePhase::Complete => descriptor.complete(None, None),
            _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resize descriptor does not match its held state")),
        }
    }
    fn begin_close(&mut self) { self.phase = ResizePhase::Closing; }
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.begin_close();
        if self.terminal_is_empty() { return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() }); }
        let demand = RetirementDemand { copy_bytes: self.next_close_copy_byte_demand()?, depth: 1, ..Default::default() };
        semio_framework_job::close_original_retained_step(cx, demand, |_| {
            if self.candidate.is_some() { self.candidate = None; } else { self.request = None; }
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
        })
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(if self.candidate.is_some() { size_of::<Option<PreparedSurfaceResize>>() } else if self.request.is_some() { size_of::<Option<SurfaceResizeRequest>>() } else { 0 }) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(!self.terminal_is_empty())) }
    fn terminal_is_empty(&self) -> bool { self.phase == ResizePhase::Closing && self.request.is_none() && self.candidate.is_none() }
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



pub struct MountedSurfaceResizeLane {
    operation: Option<OperationId>,
    token: Option<SurfaceLaneToken>,
    pending: Option<SurfaceResizeRequest>,
    session: Option<MountedWorkerJobSession<SurfaceResizeJob>>,
    source: Option<RefusedWorkerSource<SurfaceResizeJob>>,
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
        Ok(Self { operation: Some(operation), token: Some(SurfaceLaneToken { slot: slot as u16, generation }), pending: None, session: None, source: None, ready: None, metrics_generation: 0, session_closing: false, closing: false })
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
        if self.source.is_some() { self.session_closing = true; }
        Ok(self.pending.replace(request))
    }

    pub fn drive_one(&mut self, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
        if self.session_closing { self.drive_session_close_one(cx)?; return Ok(()); }
        if let Some(source) = self.source.as_mut() {
            let config = BatchDriveConfig { retained: cx.retained_grant(), site: "os_renderer_surface_resize", stage: InteractiveStage::InteractiveStep, fuel_per_step: SURFACE_RESIZE_STEP_FUEL, step_budget_us: SURFACE_RESIZE_STEP_BUDGET_US };
            if let Some(session) = source.admit_pending(config, cx)? { self.session = Some(session); assert!(source.terminal_is_empty()); self.source = None; }
            return Ok(());
        }
        if let Some(session) = self.session.as_mut() {
            if let Some(kind) = session.checked_out_outcome_descriptor().map(JobOutcomeDescriptor::kind) {
                if matches!(kind, JobOutcomeKind::Complete) {
                    let candidate = session.checked_out_job_mut().and_then(SurfaceResizeJob::take_candidate);
                    if let Some(candidate) = candidate { if self.token == Some(candidate.token) && candidate.metrics_generation == self.metrics_generation { self.ready = Some(candidate); } }
                    session.begin_close(); self.session_closing = true;
                } else if matches!(kind, JobOutcomeKind::Cancelled | JobOutcomeKind::Fault) {
                    session.begin_close(); self.session_closing = true;
                } else {
                    let demand = RetirementDemand { depth: 2, ..Default::default() };
                    semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(session.acknowledge_checked_out_outcome(grant)))?;
                    if session.checked_out_outcome_descriptor().is_none() { session.resume().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resize lost its original checked-out worker"))?; }
                }
                return Ok(());
            }
            if session.checked_out_job().is_some() { session.resume().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resize lost its empty-outcome worker"))?; }
            else { session.pump_one_for_interactive_turn(cx)?; }
            return Ok(());
        }
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(()); }
        if self.pending.is_some() { cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?; cx.consume_fuel(1); }
        if let Some(request) = self.pending.take() { self.source = Some(RefusedWorkerSource::pending(SurfaceResizeJob::new(request), (cx.operation(), cx.generation()))); }
        Ok(())
    }

    fn drive_session_close_one(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if let Some(source) = self.source.as_mut() {
            source.close_step(cx)?;
            if source.terminal_is_empty() { self.source = None; }
            return Ok(true);
        }
        let Some(session) = self.session.as_mut() else { self.session_closing = false; return Ok(false); };
        let witness = cx.borrowed_original_cancel_token()?;
        match session.has_original_cancel_alias_witness(witness) {
            Ok(true) => {
                let demand = RetirementDemand { depth: 1, ..Default::default() };
                semio_framework_job::close_original_retained_step(cx, demand, |grant| {
                    session.return_original_cancel_alias_step(witness, grant).map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(|error| match error { semio_framework_job_worker::WorkerJobDemandError::Refused(error) => error, semio_framework_job_worker::WorkerJobDemandError::Contention(_) => ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "resize cancellation return is contended") })
                })?;
                return Ok(true);
            }
            Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_)) => return Ok(true),
            Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error)) => return Err(error.into()),
            Ok(false) => {}
        }
        if matches!(session.close_step(cx)?, WorkerJobCloseStep::Complete { .. }) && session.terminal_is_empty() { self.session = None; self.session_closing = false; }
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
        self.pending.is_some() || self.source.is_some() || self.session.is_some() || self.ready.is_some()
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        if self.source.is_some() { self.session_closing = true; }
        if let Some(session) = self.session.as_mut() {
            session.begin_close();
            self.session_closing = true;
        }
    }

    pub fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.begin_close();
        if self.drive_session_close_one(cx)? { return Ok(false); }
        if self.terminal_is_empty() { return Ok(true); }
        let demand = RetirementDemand { depth: 1, ..Default::default() };
        semio_framework_job::close_original_retained_step(cx, demand, |_| {
            if self.ready.take().is_none() && self.pending.take().is_none() {
                if let Some(token) = self.token.take() { let slot = token.slot as usize; if SURFACE_LANE_GENERATIONS[slot].load(Ordering::Acquire) == token.generation { SURFACE_LANE_OCCUPIED[slot].store(false, Ordering::Release); } } else { self.operation = None; }
            }
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        })?;
        Ok(self.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.operation.is_none() && self.token.is_none() && self.pending.is_none() && self.source.is_none() && self.session.is_none() && self.ready.is_none()
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
        if let Self::Mounted(lane) = self { lane.drive_one(cx)?; }
        Ok(())
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
            Self::Rejected(rejected) => {
                if rejected.terminal_is_empty() { return Ok(true); }
                semio_framework_job::close_original_retained_step(cx, RetirementDemand { depth: 1, ..Default::default() }, |_| { rejected.close_step(); Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })) })?;
                Ok(rejected.terminal_is_empty())
            }
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
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "surface resize lane requires original paid close before drop"); }
}

//#endregion 🔖️SurfaceLane

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-surface-lane-unit/🦀️.rs"]
mod tests;
