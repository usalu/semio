//! 🧵️ Run job: the interactive job that drives the numerical `EnergyJob` inline and publishes progress ticks.

use crate::kernel::SimulationConfig;
use crate::sim::{EnergyAdmissionRejected, EnergyJob};
use crate::sim_retirement::EnergyModelCloseCursor;
use crate::sim_wire::{EnergyJobCursor, EnergyJobStage, EnergyNumericalBounds, EnergyQualityTier};
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_job::{RetainedPayloadBuilder, JobPayloadStagedSource, JobPayloadAdmissionFault, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, StepContext};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_tool_run::{ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTickWireCursor, ToolRunTickPayloadSource};
use std::sync::Arc;
use std::mem::{size_of,MaybeUninit};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::sim_wire;
use super::{EnergyTickPublication, CaptureCensus, ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL, EnergySimulationRunCounter, EnergySimulationRunReason, EnergySimulationRunStage, ModelCapture, encode_surface_energy_payload, simulation_config_for};

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
    Activity,
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

/// 🧭️ The run's projection of the numerical cursor: timestep units, tier publications and the live meter.
#[derive(Clone, Copy, Debug)]
pub(super) struct EnergyRunCursor {
    pub(super) input_identity_bytes: u64,
    pub(super) input_identity_ready: bool,
    pub(super) stage: EnergyJobStage,
    pub(super) tier: EnergyQualityTier,
    timestep: u32,
    pub(super) total_timesteps: u32,
    warmup_hours: u32,
    pub(super) warmup_timesteps: u64,
    pub(super) run_timesteps: u64,
    pub(super) tiers_published: u64,
    pub(super) facility_electricity_kwh: f64,
    /// 🧱️ Set whenever a quality tier was published, so the next tick carries a fresh per-surface map.
    surface_payload_due: bool,
}

impl EnergyRunCursor {
    pub(super) fn new() -> Self {
        Self { input_identity_bytes: 0, input_identity_ready: false, stage: EnergyJobStage::Validate, tier: EnergyQualityTier::SteadyStateEstimate, timestep: 0, total_timesteps: 0, warmup_hours: 0, warmup_timesteps: 0, run_timesteps: 0, tiers_published: 0, facility_electricity_kwh: 0.0, surface_payload_due: false }
    }

    pub(super) fn completed(&self) -> u64 {
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

    /// 🔭️ Folds actual numerical activity while keeping identity bytes separate from timestep counters.
    fn observe(&mut self, cursor: EnergyJobCursor, writer: &mut ToolRunTickWriter) -> bool {
        let input_progress = cursor.input_identity_bytes as u64 / 4096 > self.input_identity_bytes / 4096 || cursor.input_identity_ready && !self.input_identity_ready;
        self.input_identity_bytes = cursor.input_identity_bytes as u64;
        self.input_identity_ready = cursor.input_identity_ready;
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
        warmup || run || input_progress
    }

    pub(super) fn progress(&self, identity: ToolRunIdentity, state: ToolRunState, stage: EnergySimulationRunStage) -> ToolRunProgress {
        let values = [self.warmup_timesteps, self.run_timesteps, self.tiers_published, (self.facility_electricity_kwh * 1_000.0).round().max(0.0) as u64, self.input_identity_bytes];
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
/// `EnergyJob` and steps it inline under the caller's physical work and deadline. It flushes a tick for input activity,
/// a computed timestep or settlement; the semantic completed count retains timestep units and the
/// settled outcome (`Complete` with the numerical commit candidate, `Fault`, `Cancelled`) follows on the next call.
pub struct EnergySimulationRunJob {
    identity: ToolRunIdentity,
    writer: Option<ToolRunTickWriter>,
    writer_close:Option<ControlledRetirement<ToolRunTickWriter>>,
    tick_pending:bool,
    surface_requested:bool,
    pending_state:ToolRunState,
    pub(super) snapshot: Option<Arc<EnergyModelSnapshot>>,
    pub(super) template: Option<SimulationConfig>,
    template_close:Option<ControlledRetirement<SimulationConfig>>,
    snapshot_close:Option<semio_framework_value::retirement::shared::SharedControlledRetirement<EnergyModelSnapshot>>,
    phase: EnergyRunPhase,
    census: CaptureCensus,
    capture: Option<ModelCapture>,
    capture_close: Option<EnergyModelCloseCursor>,
    pub(super) numerical: Option<EnergyJob>,
    rejected: Option<EnergyAdmissionRejected>,
    checkpoint: Option<sim_wire::EnergyWireLease>,
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
            writer: Some(ToolRunTickWriter::new(identity)),
            writer_close:None,
            tick_pending:false,
            surface_requested:false,
            pending_state:ToolRunState::Running,
            snapshot: Some(snapshot),
            template:Some(template),
            template_close:None,
            snapshot_close:None,
            phase: EnergyRunPhase::Census,
            census: CaptureCensus::new(),
            capture: None,
            capture_close: None,
            numerical: None,
            rejected: None,
            checkpoint: None,
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
        let _ = self.writer.as_mut().expect("original running tick writer").step(ToolRunStepKind::Danger, self.stage().index(), reason.code(), None, args);
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

    fn capture_one(&mut self, cx: &StepContext<'_>) -> EnergyRunGrant {
        let (Some(snapshot), Some(capture)) = (self.snapshot.as_ref(), self.capture.as_mut()) else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        match capture.step_one(&snapshot.model) {
            Ok(false) => EnergyRunGrant::Continue,
            Ok(true) => self.admit(cx),
            Err(_) => self.refuse(EnergySimulationRunReason::CaptureRejected, &[]),
        }
    }

    fn admit(&mut self, cx: &StepContext<'_>) -> EnergyRunGrant {
        let Some(capture) = self.capture.take() else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        let model = capture.finish();
        let config = simulation_config_for(self.template.as_ref().expect("original Energy template remains retained"), &model);
        let base = self.identity.base_revision[..8].try_into().map_or(0, u64::from_be_bytes);
        let operation = semio_framework_job::Operation::new(cx.operation(), semio_framework_job::RevisionId(base), cx.generation(), self.identity.id.run ^ base);
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

    /// 🦶️ Lends the actual original work and retained stock to the numerical child until its next timestep.
    fn simulate(&mut self, cx: &mut StepContext<'_>) -> Result<EnergyRunGrant, semio_framework_job::OriginalTurnFailure> {
        let Self { numerical, checkpoint, cursor, writer, phase, .. } = self;
        let Some(job) = numerical.as_mut() else { return Ok(EnergyRunGrant::Settled(EnergyRunSettled::Fault)) };
        if checkpoint.is_some() {
            acknowledge_checkpoint(job, checkpoint, cx)?;
            return Ok(EnergyRunGrant::Continue);
        }
        let writer=writer.as_mut().expect("original numerical tick writer");
        loop {
            if cx.should_yield()||cx.retained_grant().maximum_items==0 {
                return Ok(EnergyRunGrant::Continue);
            }
            let before = (cx.fuel_remaining(), cx.retained_progress());
            let grant = cx.retained_grant();
            let deadline = cx.deadline_us();
            let fuel = cx.fuel_remaining().min(semio_framework_job::INTERACTIVE_LANE_FUEL);
            let seen = cx.with_original_work_loan(fuel, deadline, |cx| cx.with_retained_loan(grant, |cx| {
                Ok(match InteractiveJob::step(job, cx)? {
                    None | Some(JobOutcomeBorrow::Yield { .. }) => EnergyChildSeen::Yield,
                    Some(JobOutcomeBorrow::PreviewReady { .. }) => EnergyChildSeen::Preview,
                    Some(JobOutcomeBorrow::CheckpointReady { .. }) => EnergyChildSeen::Checkpoint,
                    Some(JobOutcomeBorrow::Complete { .. }) => EnergyChildSeen::Complete,
                    Some(JobOutcomeBorrow::Fault { .. }) => EnergyChildSeen::Fault,
                    Some(JobOutcomeBorrow::Cancelled { .. }) => EnergyChildSeen::Cancelled,
                })
            }))?;
            let settled = match seen {
                EnergyChildSeen::Yield | EnergyChildSeen::Preview => None,
                EnergyChildSeen::Checkpoint => { acknowledge_checkpoint(job, checkpoint, cx)?; None },
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
                return Ok(EnergyRunGrant::Settled(outcome));
            }
            let completed = cursor.completed();
            if cursor.observe(job.cursor(), writer) {
                return Ok(if cursor.completed() > completed { EnergyRunGrant::Timestep } else { EnergyRunGrant::Activity });
            }
            if before == (cx.fuel_remaining(), cx.retained_progress()) { return Ok(EnergyRunGrant::Continue); }
        }
    }

    /// 🧭️ Decides this call: pay back nothing, keep delivering a staged tick, lend a settled outcome, or compute and stage the next tick.
    fn next_action(&mut self, cx: &mut StepContext<'_>) -> Result<EnergyRunAction, semio_framework_job::OriginalTurnFailure> {
        if self.closing || cx.is_cancelled() {
            return Ok(EnergyRunAction::Cancelled);
        }
        if self.publication.is_some() {
            return Ok(EnergyRunAction::Publish);
        }
        if self.tick_pending {
            let stage=self.stage();
            if let Some(publication)=EnergyTickPublication::admit(&mut self.writer,&self.cursor,self.identity,self.pending_state,stage,self.surface_requested,cx)?{self.publication=Some(publication);self.tick_pending=false;}
            return Ok(EnergyRunAction::Yield);
        }
        if let Some(settled) = self.settled {
            return Ok(EnergyRunAction::Settled(settled));
        }
        let mut computed = false;
        let mut activity = false;
        let settled = loop {
            if cx.should_yield() || cx.retained_grant().maximum_items==0 || self.phase == EnergyRunPhase::Settled {
                break None;
            }
            let before = (cx.fuel_remaining(), cx.retained_progress());
            let grant = match self.phase {
                EnergyRunPhase::Census => { cx.consume_fuel(1); self.census_one() },
                EnergyRunPhase::Capture => { cx.consume_fuel(1); self.capture_one(cx) },
                EnergyRunPhase::Simulate => self.simulate(cx)?,
                EnergyRunPhase::Settled => EnergyRunGrant::Continue,
            };
            match grant {
                EnergyRunGrant::Continue => { if before == (cx.fuel_remaining(), cx.retained_progress()) { break None; } }
                EnergyRunGrant::Activity => { activity = true; break None; }
                EnergyRunGrant::Timestep => {
                    computed = true;
                }
                EnergyRunGrant::Settled(outcome) => break Some(outcome),
            }
        };
        if !computed && !activity && settled.is_none() {
            return Ok(EnergyRunAction::Yield);
        }
        let state = if settled == Some(EnergyRunSettled::Complete) { ToolRunState::Complete } else { ToolRunState::Running };
        let complete = settled == Some(EnergyRunSettled::Complete);
        self.ticks_since_surface_payload = self.ticks_since_surface_payload.saturating_add(u64::from(computed));
        let interval_due = self.ticks_since_surface_payload >= ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL;
        self.surface_requested=std::mem::take(&mut self.cursor.surface_payload_due)||interval_due||complete;
        self.pending_state=state;
        self.settled=settled;
        self.tick_pending=true;
        Ok(EnergyRunAction::Yield)
    }

    /// 🤝️ Pays back the delivered tick turn by turn from the next call's own wallet before the run resumes.
    fn retire_delivered<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let publication = self.publication.as_mut().expect("a delivered tick publication is staged");
        if !cx.is_cancelled()&&publication.published_surface()&&self.ticks_since_surface_payload!=0{let demand=RetirementDemand{copy_bytes:size_of::<u64>(),depth:1,..Default::default()};let grant=cx.retained_grant();if grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_depth>=demand.depth{self.ticks_since_surface_payload=0;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;}return Ok(None);}
        if self.writer.is_none()&&!self.closing&&!cx.is_cancelled(){if let Some(writer)=publication.admit_successor(cx)?{self.writer=Some(writer);}return Ok(None);}
        if !self.closing&&!cx.is_cancelled()&&publication.has_more_ranges(self.numerical.as_ref().and_then(|job|job.per_surface_energy()))?{publication.advance_range(&mut self.writer,cx)?;return Ok(None);}
        let demand=publication.retirement_demands()?;
        semio_framework_job::close_original_retained_step(cx,demand,|grant|{
            let progress=self.publication.as_mut().expect("original delivered Energy tick remains retained").close_step(grant)?.progress();
            if self.publication.as_ref().is_some_and(|publication|publication.terminal_is_empty()){self.publication=None;}
            Ok(RetainedCloneStep::Progress(progress))
        })?;
        Ok(None)
    }

    /// 📏️ Quotes the next close frontier of this run.
    fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.checkpoint.is_some() { return checkpoint_demands(&self.checkpoint); }
        if let Some(publication) = self.publication.as_ref() {
            return publication.retirement_demands();
        }
        if let Some(close)=self.writer_close.as_ref(){if close.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<ToolRunTickWriter>>>(),depth:1,..Default::default()})}return Ok(RetirementDemand{copy_bytes:close.next_copy_byte_demand()?,capacity_bytes:close.next_capacity_byte_demand(usize::MAX)?,release_bytes:close.next_release_byte_demand()?,depth:close.next_depth_demand()?+1})}
        if self.writer.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<ControlledRetirement<ToolRunTickWriter>>()+size_of::<Option<ToolRunTickWriter>>(),depth:1,..Default::default()})}
        if let Some(job) = self.numerical.as_ref() {
            if InteractiveJob::terminal_is_empty(job){return Ok(RetirementDemand{copy_bytes:size_of::<Option<EnergyJob>>(),depth:1,..Default::default()})}
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
        if let Some(close)=self.snapshot_close.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<EnergyModelSnapshot>>>(),depth:1,..Default::default()})}else{close.next_demand(usize::MAX)}}
        if self.snapshot.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value::retirement::shared::SharedControlledRetirement<EnergyModelSnapshot>>()+size_of::<Option<Arc<EnergyModelSnapshot>>>(),depth:1,..Default::default()})}
        if let Some(close)=self.template_close.as_ref(){return if close.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<SimulationConfig>>>(),depth:1,..Default::default()})}else{close.selected_demand(usize::MAX)}}
        if self.template.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<ControlledRetirement<SimulationConfig>>()+size_of::<Option<SimulationConfig>>(),depth:1,..Default::default()})}
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

/// 🧭️ How one numerical step ended, decided before the lent outcome borrow is released.
#[derive(Clone, Copy)]
enum EnergyChildSeen {
    Yield,
    Preview,
    Checkpoint,
    Complete,
    Fault,
    Cancelled,
}

/// 📮️ Retires the numerical job's retained restore checkpoint from the run's wallet: a run restarts rather than restores.
fn acknowledge_checkpoint(job: &mut EnergyJob, checkpoint: &mut Option<sim_wire::EnergyWireLease>, cx: &mut StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    let demand = checkpoint_demands(checkpoint)?;
    let generation = cx.generation();
    semio_framework_job::close_original_retained_step(cx, demand, |grant| {
        if checkpoint.as_ref().is_some_and(|lease| !lease.packet().terminal_is_empty()) {
            return checkpoint.as_mut().expect("original numerical checkpoint").packet_mut().close_step(grant);
        }
        if let Some(lease) = checkpoint.take() {
            if let Err(lease) = job.ack_checkpoint_packet(lease) {
                *checkpoint = Some(lease);
                return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "original Energy checkpoint acknowledgement refused its retained lease"));
            }
        } else {
            *checkpoint = job.take_checkpoint_packet(generation).map_err(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "original Energy checkpoint differs from its admitted generation"))?;
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    })?;
    Ok(checkpoint.is_none())
}

/// 📏️ Quotes the exact original checkpoint body or one lease transfer before mutation.
fn checkpoint_demands(checkpoint: &Option<sim_wire::EnergyWireLease>) -> Result<RetirementDemand, ValueError> {
    if let Some(lease) = checkpoint.as_ref() { if !lease.packet().terminal_is_empty() { return lease.packet().retirement_demands(); } }
    Ok(RetirementDemand { copy_bytes: size_of::<Option<sim_wire::EnergyWireLease>>(), depth: 1, ..Default::default() })
}

impl EnergySimulationRunJob {
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if self.publication.as_ref().is_some_and(|publication| publication.delivered) {
            return self.retire_delivered(cx);
        }
        match self.next_action(cx)? {
            EnergyRunAction::Yield => Ok(None),
            EnergyRunAction::Cancelled => JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into),
            EnergyRunAction::Publish => {let table=self.numerical.as_ref().and_then(|job|job.per_surface_energy());self.publication.as_mut().expect("a staged tick publication").poll(cx,table).map_err(Into::into)},
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
}

impl InteractiveJob for EnergySimulationRunJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

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

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{ semio_framework_job::close_original_nested_step(cx, |cx, _grant| {
        self.begin_close();
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        if self.checkpoint.is_some() {
            let job = self.numerical.as_mut().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original Energy checkpoint retains its numerical owner"))?;
            let before = cx.retained_progress();
            acknowledge_checkpoint(job, &mut self.checkpoint, cx)?;
            let after = cx.retained_progress();
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: after.copied_items - before.copied_items, copied_bytes: after.copied_bytes - before.copied_bytes, retained_capacity_bytes: after.retained_capacity_bytes - before.retained_capacity_bytes, released_bytes: after.released_bytes - before.released_bytes } });
        }
        if self.publication.is_none()&&self.writer_close.is_none()&&self.writer.is_none(){
            if let Some(job)=self.numerical.as_mut(){
                if !InteractiveJob::terminal_is_empty(job){let grant = cx.retained_grant(); return cx.with_retained_loan(grant, |same| InteractiveJob::close_step(job, same))}
            }
        }
        let demand=self.close_demands()?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
            let one=RetainedCloneProgress{copied_items:1,..Default::default()};
            let progress=if let Some(publication)=self.publication.as_mut(){
                let progress=publication.close_step(grant)?.progress();
                if publication.terminal_is_empty(){self.publication=None;}
                progress
            }else if let Some(close)=self.writer_close.as_mut(){
                if close.terminal_is_empty(){self.writer_close=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
                else{close.step(grant)?.progress()}
            }else if let Some(writer)=self.writer.take(){
                match ControlledRetirement::new(writer){Ok(close)=>self.writer_close=Some(close),Err((error,writer))=>{self.writer=Some(writer);return Err(error)}}
                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
            }else if self.numerical.is_some(){
                self.numerical=None;
                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
            }else if let Some(rejected)=self.rejected.as_mut(){
                let step=rejected.close_step(grant);
                if rejected.terminal_is_empty(){self.rejected=None;}
                step.progress()
            }else if let Some(capture)=self.capture.take(){
                self.capture_close=Some(EnergyModelCloseCursor::new(capture.finish()));one
            }else if let Some(close)=self.capture_close.as_mut(){
                let progress=close.close_step(grant).progress();
                if close.terminal_is_empty(){self.capture_close=None;}
                progress
            }else if let Some(close)=self.snapshot_close.as_mut(){
                if close.terminal_is_empty(){self.snapshot_close=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
                else{close.step(grant)?.progress()}
            }else if let Some(snapshot)=self.snapshot.take(){
                self.snapshot_close=Some(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(snapshot));
                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
            }else if let Some(close)=self.template_close.as_mut(){
                if close.terminal_is_empty(){self.template_close=None;RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}}
                else{close.step(grant)?.progress()}
            }else if let Some(template)=self.template.take(){
                match ControlledRetirement::new(template){Ok(close)=>self.template_close=Some(close),Err((error,template))=>{self.template=Some(template);return Err(error)}}
                RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}
            }else{Default::default()};
            Ok(RetainedCloneStep::Progress(progress))
        })?;
        Ok(match step{InteractiveJobCloseStep::Pending{progress}if self.terminal_is_empty()=>InteractiveJobCloseStep::Complete{progress},step=>step})
    }) }

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
        self.closing && self.checkpoint.is_none() && self.writer.is_none() && self.writer_close.is_none() && self.publication.is_none() && self.numerical.is_none() && self.rejected.is_none() && self.capture.is_none() && self.capture_close.is_none() && self.snapshot.is_none() && self.snapshot_close.is_none() && self.template.is_none() && self.template_close.is_none()
    }
}
//#endregion 🧵️RunJob
