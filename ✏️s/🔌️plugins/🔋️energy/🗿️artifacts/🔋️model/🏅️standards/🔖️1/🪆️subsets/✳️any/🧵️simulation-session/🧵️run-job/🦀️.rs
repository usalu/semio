//! 🧵️ Run job: the interactive job that drives the numerical `EnergyJob` inline and publishes progress ticks.

use semio_framework_job_publication::{RetainedJobPublication, JobPublicationKind};
use crate::kernel::SimulationConfig;
use crate::sim::{EnergyAdmissionRejected, EnergyJob};
use crate::sim_retirement::EnergyModelCloseCursor;
use crate::sim_wire::{EnergyJobCursor, EnergyJobStage, EnergyNumericalBounds, EnergyQualityTier};
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, OperationId, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, StepBudget, StepContext};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_tool_run::{ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter};
use std::sync::Arc;
use super::{CaptureCensus, ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL, EnergySimulationRunCounter, EnergySimulationRunReason, EnergySimulationRunStage, ModelCapture, encode_surface_energy_payload, simulation_config_for};

//#region 🧵️RunJob
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnergyRunPhase {
    Census,
    Capture,
    Simulate,
    Settled,
}

/// 🏁️ How the run ended; the lent outcome carries only empty marker payloads because the tool-run driver reads the ticks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnergyRunSettled {
    Complete,
    Fault,
    Cancelled,
}

/// 🎟️ What one bounded grant of the run produced.
enum EnergyRunGrant {
    Continue,
    Timestep,
    Settled(EnergyRunSettled),
}

/// 🧭️ What one internal step decided before any outcome is lent to the caller.
enum EnergyRunAction {
    Yield,
    Cancelled,
    Publish,
    Settled(EnergyRunSettled),
}

/// 🏷️ Empty marker payloads lent as the settled outcome's payloads.
struct EnergyRunMarkers {
    pub(super) state: RetainedJobPayload,
    output: RetainedJobPayload,
    fault: RetainedJobPayload,
}

impl EnergyRunMarkers {
    pub(super) fn new() -> Self {
        Self { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput), fault: RetainedJobPayload::empty(JobPayloadStream::Fault) }
    }
}

/// 📮️ One encoded tick lent as a preview: the publication binds the source bytes until they are paid back.
struct EnergyTickPublication {
    single: RetainedJobPublication,
    pub(super) bytes: Vec<u8>,
    delivered: bool,
}

impl EnergyTickPublication {
    pub(super) fn new(bytes: Vec<u8>) -> Box<Self> {
        Box::new(Self { single: RetainedJobPublication::new(), bytes, delivered: false })
    }

    fn poll<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let result = self.single.advance_from_source(JobPublicationKind::Preview, &self.bytes, cx)?;
        if result.is_some() {
            self.delivered = true;
        }
        Ok(result)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        self.single.borrow_outcome(descriptor)
    }

    fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.single.terminal_is_empty() {
            return self.single.retirement_demands();
        }
        Ok(RetirementDemand { release_bytes: self.bytes.capacity(), depth: usize::from(self.bytes.capacity() != 0), ..Default::default() })
    }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let demand = match self.retirement_demands() {
            Ok(demand) => demand,
            Err(error) => return InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        };
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        if !self.single.terminal_is_empty() {
            return match self.single.close_step(grant) {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
                Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
            };
        }
        if self.bytes.capacity() != 0 {
            let released_bytes = self.bytes.capacity();
            self.bytes = Vec::new();
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.single.terminal_is_empty() && self.bytes.capacity() == 0
    }
}

/// 🧭️ The run's projection of the numerical cursor: timestep units, tier publications and the live meter.
#[derive(Clone, Copy, Debug)]
pub(super) struct EnergyRunCursor {
    pub(super) stage: EnergyJobStage,
    pub(super) tier: EnergyQualityTier,
    timestep: u32,
    total_timesteps: u32,
    warmup_hours: u32,
    warmup_timesteps: u64,
    run_timesteps: u64,
    tiers_published: u64,
    pub(super) facility_electricity_kwh: f64,
    /// 🧱️ Set whenever a quality tier was published, so the next tick carries a fresh per-surface map.
    surface_payload_due: bool,
}

impl EnergyRunCursor {
    pub(super) fn new() -> Self {
        Self { stage: EnergyJobStage::Validate, tier: EnergyQualityTier::SteadyStateEstimate, timestep: 0, total_timesteps: 0, warmup_hours: 0, warmup_timesteps: 0, run_timesteps: 0, tiers_published: 0, facility_electricity_kwh: 0.0, surface_payload_due: false }
    }

    fn completed(&self) -> u64 {
        self.warmup_timesteps + self.run_timesteps
    }

    fn is_warmup(stage: EnergyJobStage) -> bool {
        matches!(stage, EnergyJobStage::WarmupTimestep | EnergyJobStage::WarmupConvergence)
    }

    fn publish_tier(&mut self, writer: &mut ToolRunTickWriter) {
        let kwh = (self.facility_electricity_kwh * 1_000.0).round() / 1_000.0;
        let args = [ToolRunStepArg::Float(kwh), ToolRunStepArg::Unsigned(self.run_timesteps), ToolRunStepArg::Unsigned(u64::from(self.total_timesteps))];
        let _ = writer.step(ToolRunStepKind::Success, EnergySimulationRunStage::of(self.stage).index(), EnergySimulationRunReason::of_tier(self.tier).code(), None, &args);
        self.tiers_published += 1;
        self.surface_payload_due = true;
    }

    /// 🔭️ Folds one numerical cursor; `true` when exactly one warmup or run timestep was computed.
    fn observe(&mut self, cursor: EnergyJobCursor, writer: &mut ToolRunTickWriter) -> bool {
        let warmup = self.stage == EnergyJobStage::WarmupTimestep && cursor.stage == EnergyJobStage::WarmupConvergence;
        let run = cursor.timestep > self.timestep;
        self.warmup_timesteps += u64::from(warmup);
        self.run_timesteps += u64::from(cursor.timestep.saturating_sub(self.timestep));
        if Self::is_warmup(self.stage) && !Self::is_warmup(cursor.stage) && self.warmup_timesteps < u64::from(cursor.warmup_hours) {
            let _ = writer.step(ToolRunStepKind::Info, EnergySimulationRunStage::Warmup.index(), EnergySimulationRunReason::WarmupConverged.code(), None, &[ToolRunStepArg::Unsigned(self.warmup_timesteps)]);
        }
        self.timestep = cursor.timestep;
        self.total_timesteps = cursor.total_timesteps;
        self.warmup_hours = cursor.warmup_hours;
        self.facility_electricity_kwh = cursor.facility_electricity_kwh;
        if cursor.tier != self.tier {
            self.publish_tier(writer);
            self.tier = cursor.tier;
        }
        self.stage = cursor.stage;
        warmup || run
    }

    pub(super) fn progress(&self, identity: ToolRunIdentity, state: ToolRunState, stage: EnergySimulationRunStage) -> ToolRunProgress {
        let values = [self.warmup_timesteps, self.run_timesteps, self.tiers_published, (self.facility_electricity_kwh * 1_000.0).round().max(0.0) as u64];
        ToolRunProgress {
            identity,
            sequence: 0,
            state,
            stage: stage.index(),
            completed: self.completed(),
            total: (stage.index() >= EnergySimulationRunStage::Run.index()).then(|| self.warmup_timesteps + u64::from(self.total_timesteps)),
            counters: EnergySimulationRunCounter::ALL.iter().zip(values).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        }
    }
}

/// ⏯️ The simulation `runJob`: captures the run's base model in bounded grants, admits the numerical
/// `EnergyJob` and steps it inline under the caller's deadline. It consumes one unit of fuel per computed
/// timestep and flushes one tick whenever at least one timestep was computed or the run settled; the
/// settled outcome (`Complete` with the numerical commit candidate, `Fault`, `Cancelled`) follows on the next call.
pub struct EnergySimulationRunJob {
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
    pub(super) snapshot: Option<Arc<EnergyModelSnapshot>>,
    pub(super) template: SimulationConfig,
    phase: EnergyRunPhase,
    census: CaptureCensus,
    capture: Option<ModelCapture>,
    capture_close: Option<EnergyModelCloseCursor>,
    pub(super) numerical: Option<EnergyJob>,
    rejected: Option<EnergyAdmissionRejected>,
    pub(super) operation: OperationId,
    pub(super) generation: Generation,
    numerical_sequence: u64,
    cursor: EnergyRunCursor,
    /// 🧱️ Computed timesteps since the per-surface map was last published into a tick.
    ticks_since_surface_payload: u64,
    pub(super) settled: Option<EnergyRunSettled>,
    publication: Option<Box<EnergyTickPublication>>,
    markers: EnergyRunMarkers,
    closing: bool,
}

impl EnergySimulationRunJob {
    pub fn new(identity: ToolRunIdentity, snapshot: Arc<EnergyModelSnapshot>, template: SimulationConfig) -> Self {
        Self {
            identity,
            writer: ToolRunTickWriter::new(identity),
            snapshot: Some(snapshot),
            template,
            phase: EnergyRunPhase::Census,
            census: CaptureCensus::new(),
            capture: None,
            capture_close: None,
            numerical: None,
            rejected: None,
            operation: semio_framework_job::allocate_operation_id(),
            generation: Generation(u64::from(identity.generation)),
            numerical_sequence: 0,
            cursor: EnergyRunCursor::new(),
            ticks_since_surface_payload: 0,
            settled: None,
            publication: None,
            markers: EnergyRunMarkers::new(),
            closing: false,
        }
    }

    fn stage(&self) -> EnergySimulationRunStage {
        if self.numerical.is_some() {
            EnergySimulationRunStage::of(self.cursor.stage)
        } else {
            EnergySimulationRunStage::Capture
        }
    }

    fn refuse(&mut self, reason: EnergySimulationRunReason, args: &[ToolRunStepArg]) -> EnergyRunGrant {
        let _ = self.writer.step(ToolRunStepKind::Danger, self.stage().index(), reason.code(), None, args);
        self.phase = EnergyRunPhase::Settled;
        EnergyRunGrant::Settled(EnergyRunSettled::Fault)
    }

    fn census_one(&mut self) -> EnergyRunGrant {
        let Some(snapshot) = self.snapshot.as_ref() else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        match self.census.step_one(&snapshot.model) {
            Ok(false) => EnergyRunGrant::Continue,
            Ok(true) => {
                self.capture = Some(ModelCapture::new());
                self.phase = EnergyRunPhase::Capture;
                EnergyRunGrant::Continue
            }
            Err(_) => self.refuse(EnergySimulationRunReason::CaptureRejected, &[]),
        }
    }

    fn capture_one(&mut self) -> EnergyRunGrant {
        let (Some(snapshot), Some(capture)) = (self.snapshot.as_ref(), self.capture.as_mut()) else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        match capture.step_one(&snapshot.model) {
            Ok(false) => EnergyRunGrant::Continue,
            Ok(true) => self.admit(),
            Err(_) => self.refuse(EnergySimulationRunReason::CaptureRejected, &[]),
        }
    }

    fn admit(&mut self) -> EnergyRunGrant {
        let Some(capture) = self.capture.take() else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        let model = capture.finish();
        let config = simulation_config_for(&self.template, &model);
        let base = self.identity.base_revision[..8].try_into().map_or(0, u64::from_be_bytes);
        let operation = semio_framework_job::Operation::new(self.operation, semio_framework_job::RevisionId(base), self.generation, self.identity.id.run ^ base);
        self.snapshot = None;
        match EnergyJob::admit(operation, model, config, EnergyNumericalBounds::default()) {
            Ok(job) => {
                self.numerical = Some(job);
                self.phase = EnergyRunPhase::Simulate;
                EnergyRunGrant::Continue
            }
            Err(rejected) => {
                self.rejected = Some(rejected);
                self.refuse(EnergySimulationRunReason::AdmissionRejected, &[])
            }
        }
    }

    /// 🦶️ Steps the numerical job until one timestep is computed, the run settles or the caller must yield. Each
    /// numerical step gets its own context, because a step context grants at most one payload page.
    fn simulate(&mut self, cx: &mut StepContext<'_>) -> EnergyRunGrant {
        let Self { numerical, numerical_sequence, operation, generation, cursor, writer, phase, .. } = self;
        let Some(job) = numerical.as_mut() else { return EnergyRunGrant::Settled(EnergyRunSettled::Fault) };
        loop {
            if cx.should_yield() {
                return EnergyRunGrant::Continue;
            }
            let mut receipt = RetainedCloneProgress::default();
            let mut inner = StepContext::new(*operation, *generation, StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, cx.deadline_us(), cx.retained_grant()), cx.cancel_token(), semio_framework_job::default_now_us, numerical_sequence, &mut receipt);
            let seen = match InteractiveJob::step(job, &mut inner) {
                Ok(None | Some(JobOutcomeBorrow::Yield { .. })) => EnergyChildSeen::Yield,
                Ok(Some(JobOutcomeBorrow::PreviewReady { .. })) => EnergyChildSeen::Preview,
                Ok(Some(JobOutcomeBorrow::CheckpointReady { .. })) => EnergyChildSeen::Checkpoint,
                Ok(Some(JobOutcomeBorrow::Complete { .. })) => EnergyChildSeen::Complete,
                Ok(Some(JobOutcomeBorrow::Fault { .. })) | Err(_) => EnergyChildSeen::Fault,
                Ok(Some(JobOutcomeBorrow::Cancelled { .. })) => EnergyChildSeen::Cancelled,
            };
            drop(inner);
            if cx.consume_retained(receipt).is_err() {
                *phase = EnergyRunPhase::Settled;
                return EnergyRunGrant::Settled(EnergyRunSettled::Fault);
            }
            let settled = match seen {
                EnergyChildSeen::Yield | EnergyChildSeen::Preview => None,
                EnergyChildSeen::Checkpoint => (!acknowledge_checkpoint(job, *generation, cx)).then_some(EnergyRunSettled::Fault),
                EnergyChildSeen::Complete => {
                    cursor.observe(job.cursor(), writer);
                    cursor.publish_tier(writer);
                    Some(EnergyRunSettled::Complete)
                }
                EnergyChildSeen::Fault => {
                    let _ = writer.step(ToolRunStepKind::Danger, EnergySimulationRunStage::of(cursor.stage).index(), EnergySimulationRunReason::SimulationFaulted.code(), None, &[ToolRunStepArg::Unsigned(cursor.completed())]);
                    Some(EnergyRunSettled::Fault)
                }
                EnergyChildSeen::Cancelled => Some(EnergyRunSettled::Cancelled),
            };
            if let Some(outcome) = settled {
                *phase = EnergyRunPhase::Settled;
                return EnergyRunGrant::Settled(outcome);
            }
            if cursor.observe(job.cursor(), writer) {
                return EnergyRunGrant::Timestep;
            }
        }
    }

    /// 🧭️ Decides this call: pay back nothing, keep delivering a staged tick, lend a settled outcome, or compute and stage the next tick.
    fn next_action(&mut self, cx: &mut StepContext<'_>) -> EnergyRunAction {
        if self.closing || cx.is_cancelled() {
            return EnergyRunAction::Cancelled;
        }
        if self.publication.is_some() {
            return EnergyRunAction::Publish;
        }
        if let Some(settled) = self.settled {
            return EnergyRunAction::Settled(settled);
        }
        let mut computed = false;
        let settled = loop {
            if cx.should_yield() || self.phase == EnergyRunPhase::Settled {
                break None;
            }
            let grant = match self.phase {
                EnergyRunPhase::Census => self.census_one(),
                EnergyRunPhase::Capture => self.capture_one(),
                EnergyRunPhase::Simulate => self.simulate(cx),
                EnergyRunPhase::Settled => EnergyRunGrant::Continue,
            };
            match grant {
                EnergyRunGrant::Continue => {}
                EnergyRunGrant::Timestep => {
                    computed = true;
                    cx.consume_fuel(1);
                }
                EnergyRunGrant::Settled(outcome) => break Some(outcome),
            }
        };
        if !computed && settled.is_none() {
            return EnergyRunAction::Yield;
        }
        let state = if settled == Some(EnergyRunSettled::Complete) { ToolRunState::Complete } else { ToolRunState::Running };
        let complete = settled == Some(EnergyRunSettled::Complete);
        self.ticks_since_surface_payload = self.ticks_since_surface_payload.saturating_add(u64::from(computed));
        let interval_due = self.ticks_since_surface_payload >= ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL;
        if std::mem::take(&mut self.cursor.surface_payload_due) || interval_due || complete {
            if let Some(table) = self.numerical.as_ref().and_then(|job| job.per_surface_energy()) {
                if table.has_energy() {
                    self.writer.payload(encode_surface_energy_payload(table.summaries()));
                    self.ticks_since_surface_payload = 0;
                }
            }
        }
        self.writer.progress(self.cursor.progress(self.identity, state, self.stage()));
        self.settled = settled;
        match self.writer.finish().and_then(|tick| tick.encode().ok()) {
            Some(bytes) => {
                self.publication = Some(EnergyTickPublication::new(bytes));
                EnergyRunAction::Publish
            }
            None => {
                self.settled = Some(EnergyRunSettled::Fault);
                EnergyRunAction::Settled(EnergyRunSettled::Fault)
            }
        }
    }

    /// 🤝️ Pays back the delivered tick turn by turn from the next call's own wallet before the run resumes.
    fn retire_delivered<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let publication = self.publication.as_mut().expect("a delivered tick publication is staged");
        let step = publication.close_step(cx.retained_grant());
        cx.consume_retained(step.progress())?;
        if let InteractiveJobCloseStep::Refused { kind, progress } = step {
            return Err(ValueError::literal(kind, "energy tick publication close was refused").with_retained_progress(progress));
        }
        if publication.terminal_is_empty() {
            self.publication = None;
        }
        Ok(None)
    }

    /// 📏️ Quotes the next close frontier of this run.
    fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(job) = self.numerical.as_ref() {
            return Ok(RetirementDemand {
                copy_bytes: job.next_close_copy_byte_demand()?,
                capacity_bytes: job.next_close_capacity_byte_demand(usize::MAX)?,
                release_bytes: job.next_close_release_byte_demand()?,
                depth: job.next_close_depth_demand()?,
            });
        }
        if let Some(rejected) = self.rejected.as_ref() {
            return Ok(rejected.close_demands());
        }
        if let Some(close) = self.capture_close.as_ref() {
            return Ok(close.close_demands());
        }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

/// 🧭️ How one numerical step ended, decided before the lent outcome borrow is released.
enum EnergyChildSeen {
    Yield,
    Preview,
    Checkpoint,
    Complete,
    Fault,
    Cancelled,
}

/// 📮️ Retires the numerical job's retained restore checkpoint from the run's wallet: a run restarts rather than restores.
fn acknowledge_checkpoint(job: &mut EnergyJob, generation: Generation, cx: &mut StepContext<'_>) -> bool {
    let mut lease = match job.take_checkpoint_packet(generation) {
        Ok(Some(lease)) => lease,
        Ok(None) => return true,
        Err(_) => return false,
    };
    for _ in 0..64 {
        if lease.packet().terminal_is_empty() {
            return job.ack_checkpoint_packet(lease).is_ok();
        }
        match lease.packet_mut().close_step(cx.retained_grant()) {
            Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => {
                if cx.consume_retained(progress).is_err() {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    false
}

impl InteractiveJob for EnergySimulationRunJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.publication.as_ref().is_some_and(|publication| publication.delivered) {
            return self.retire_delivered(cx);
        }
        match self.next_action(cx) {
            EnergyRunAction::Yield => Ok(None),
            EnergyRunAction::Cancelled => JobOutcomeBorrow::admit_cancelled(cx),
            EnergyRunAction::Publish => self.publication.as_mut().expect("a staged tick publication").poll(cx),
            EnergyRunAction::Settled(EnergyRunSettled::Complete) => {
                let admitted = JobOutcomeBorrow::admit_complete(cx, Some(&self.markers.state), Some(&self.markers.output))?;
                if admitted.is_some() {
                    self.settled = None;
                }
                Ok(admitted)
            }
            EnergyRunAction::Settled(EnergyRunSettled::Fault) => {
                let admitted = JobOutcomeBorrow::admit_fault(cx, &self.markers.fault)?;
                if admitted.is_some() {
                    self.settled = None;
                }
                Ok(admitted)
            }
            EnergyRunAction::Settled(EnergyRunSettled::Cancelled) => {
                let admitted = JobOutcomeBorrow::admit_cancelled(cx)?;
                if admitted.is_some() {
                    self.settled = None;
                }
                Ok(admitted)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(Some(&self.markers.state), Some(&self.markers.output)),
            JobOutcomeKind::Fault => descriptor.fault(&self.markers.fault),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } => self
                .publication
                .as_ref()
                .ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "energy run outcome has no staged tick publication"))?
                .borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(job) = self.numerical.as_mut() {
            InteractiveJob::begin_close(job);
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.begin_close();
        let one = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let idle = InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        if self.terminal_is_empty() {
            return InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if let Some(publication) = self.publication.as_mut() {
            let step = publication.close_step(grant);
            if publication.terminal_is_empty() {
                self.publication = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if let Some(job) = self.numerical.as_mut() {
            return match InteractiveJob::close_step(job, grant) {
                InteractiveJobCloseStep::Complete { progress } => {
                    self.numerical = None;
                    InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } }
                }
                pending => pending,
            };
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return idle;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            return match rejected.close_step(grant) {
                InteractiveJobCloseStep::Complete { .. } if rejected.terminal_is_empty() => {
                    self.rejected = None;
                    InteractiveJobCloseStep::Pending { progress: one }
                }
                InteractiveJobCloseStep::Complete { .. } => InteractiveJobCloseStep::Blocked,
                pending => pending,
            };
        }
        if let Some(capture) = self.capture.take() {
            self.capture_close = Some(EnergyModelCloseCursor::new(capture.finish()));
            return InteractiveJobCloseStep::Pending { progress: one };
        }
        if let Some(close) = self.capture_close.as_mut() {
            return match close.close_step(grant) {
                InteractiveJobCloseStep::Complete { .. } => {
                    self.capture_close = None;
                    InteractiveJobCloseStep::Pending { progress: one }
                }
                pending => pending,
            };
        }
        if self.snapshot.take().is_some() {
            return InteractiveJobCloseStep::Pending { progress: one };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.publication.is_none() && self.numerical.is_none() && self.rejected.is_none() && self.capture.is_none() && self.capture_close.is_none() && self.snapshot.is_none()
    }
}
//#endregion 🧵️RunJob
