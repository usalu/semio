//! 🚦️ Clock stride and job protocol: the clock stride, the checkpoint, commit candidate, fault and step outcome records, the close step, the thread transfer traits and the interactive job trait. See [`super`].

use super::*;

//#region 🪜️ClockStride
/// 🪜️ A step's clock, read for real only every `stride` calls. A job that checks its deadline after
/// every unit of work used to read the host clock just as often; inside a Wasm guest that read is a
/// component-model host call, and measured 2026-09-24 (ticket 26/09/23 slice G6) the `🀄️wfc` genesis
/// solve made 6.7 million of them in one `inference_run`. The stride is recalibrated at every real
/// read so reads land about [`ClockStride::TARGET_READ_INTERVAL_US`] apart, which is also the most a
/// deadline is overshot by. Every [`StepContext`] a driver builds carries one: the step begins at the
/// driver's own entry reading ([`ClockStride::begin`]), so a new step costs no read of its own, and the
/// calibrated stride carries from step to step. Law: `🧫️fixtures/🪜️clock-stride-law.json`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockStride {
    pub(super) stride: u32,
    pub(super) remaining: u32,
    last_read_us: Option<u64>,
}

impl ClockStride {
    pub const TARGET_READ_INTERVAL_US: u64 = 64;
    pub const MAXIMUM_STRIDE: u32 = 4_096;

    pub const fn new() -> Self {
        Self { stride: 1, remaining: 0, last_read_us: None }
    }

    /// 🏁️ Begins a step at `start_us`, the reading its driver made at entry: the step's first
    /// `stride - 1` reads answer it, and the calibrated stride is kept.
    pub fn begin(&mut self, start_us: Option<u64>) {
        if start_us.is_none() || matches!((start_us,self.last_read_us),(Some(now),Some(last)) if now<last) {
            self.remaining=0;
            return;
        }
        self.last_read_us = start_us;
        self.remaining = if start_us.is_some() { self.stride - 1 } else { 0 };
    }

    /// ⏱️ The clock as of the last real read of `clock`, which is at most one stride of calls old.
    pub fn read(&mut self, clock: fn() -> Option<u64>) -> Option<u64> {
        if self.remaining > 0 {
            self.remaining -= 1;
            return self.last_read_us;
        }
        let now = clock()?;
        if let Some(last) = self.last_read_us {
            if now<last {self.remaining=0;return None;}
            let elapsed = (now-last).max(1);
            let calibrated = u64::from(self.stride) * Self::TARGET_READ_INTERVAL_US / elapsed;
            self.stride = calibrated.clamp(1, u64::from(Self::MAXIMUM_STRIDE).min(u64::from(self.stride) * 2)) as u32;
        }
        self.last_read_us = Some(now);
        self.remaining = self.stride - 1;
        Some(now)
    }

    /// 🕰️ The last reading, without reading the clock.
    pub fn latest_us(&self) -> Option<u64> {
        self.last_read_us
    }

    /// 🪜️ How many calls the next real read is apart — the law's observable.
    pub fn stride(&self) -> u32 {
        self.stride
    }
}

impl Default for ClockStride {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🪜️ClockStride

//#region 🚦️StepOutcome
/// 📸️ A pause point where work is resumable but not yet committed — `state` is opaque, pack-encoded
/// (or, for a dependency-free job like [`TortureJob`], hand-rolled little-endian) bytes the job alone
/// interprets; `applied_progress` is the Puzzle 3D `FillBuilder.applied_count` pattern generalized: how
/// much of `state` is COMMITTED versus merely planned, so a caller can show "these N are done" without
/// decoding `state` itself.
#[derive(Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub state: RetainedJobPayload,
    pub applied_progress: u64,
}

/// 🏁️ Terminal success payload: the job's final persisted `state` plus its `output` — both opaque
/// bytes, so the runtime stays completely job-agnostic (design doc Decision 2).
#[derive(Debug, PartialEq, Eq)]
pub struct CommitCandidate {
    pub state: RetainedJobPayload,
    pub output: RetainedJobPayload,
}

/// 💥️ Opaque, job-specific error payload — never interpreted by the runtime, same reasoning as
/// [`CommitCandidate`]'s fields.
#[derive(Debug, PartialEq, Eq)]
pub struct JobFault {
    pub detail: RetainedJobPayload,
}

/// 🚦️ What one [`InteractiveJob::step`] call reports. [`StepOutcome::Yield`]/[`StepOutcome::PreviewReady`]/
/// [`StepOutcome::CheckpointReady`] all mean "call `step` again"; [`StepOutcome::is_terminal`] marks
/// the other three.
#[derive(Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "Step outcomes move preadmitted payload pages inline; boxing would allocate outside the step grant.")]
pub enum StepOutcome {
    Yield,
    PreviewReady(RetainedJobPayload),
    CheckpointReady(Checkpoint),
    Complete(CommitCandidate),
    Cancelled,
    Fault(JobFault),
}

impl StepOutcome {
    pub fn is_terminal(&self) -> bool {
        matches!(self, StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_))
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            StepOutcome::Yield | StepOutcome::Cancelled => true,
            StepOutcome::PreviewReady(payload) => payload.terminal_is_empty(),
            StepOutcome::CheckpointReady(checkpoint) => checkpoint.state.terminal_is_empty(),
            StepOutcome::Complete(candidate) => candidate.state.terminal_is_empty() && candidate.output.terminal_is_empty(),
            StepOutcome::Fault(fault) => fault.detail.terminal_is_empty(),
        }
    }
}
//#endregion 🚦️StepOutcome

//#region 🧩️InteractiveJob
#[derive(Clone, Copy, Debug, PartialEq, Eq,semio_framework_value::RetireOwned)]
pub enum InteractiveJobCloseStep {
    Pending { progress: RetainedCloneProgress },
    Blocked,
    Complete { progress: RetainedCloneProgress },
}

impl InteractiveJobCloseStep {
    /// 📊️ Keeps every admitted turn's physical and logical receipt, including terminal release.
    pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Blocked=>RetainedCloneProgress::default()}}
    /// 🛡️ Checks independent authority and the original owner's terminal witness before publication.
    pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Result<Self,ValueError>{
        if !self.progress().fits(grant)||matches!(self,Self::Complete{..})&&!terminal_is_empty{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job close receipt or terminal witness refused").with_retained_progress(self.progress()))}else{Ok(self)}
    }
}

/// 🧵️ Whether a job may be handed to another thread. Every threaded target elaborates this to
/// `Send`, so `J: InteractiveJob` still proves `J: Send` for [`WorkerJobSessionInner`]'s
/// `unsafe impl` and every pool submission. The browser build (`wasm32`, not `wasip2`) has no
/// second thread to hand a job to at all — its jobs own `Rc<JsValue>` browser handles by design
/// (`🎯️targets/🧊️wgpu/🧵️frame-job`'s frame worker is the single JS worker thread) — so requiring
/// `Send` there would only forbid the one ownership model that target can have.
#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
pub trait JobThreadTransfer: Send {}

#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
impl<T: Send + ?Sized> JobThreadTransfer for T {}

/// 🧵️ Browser twin of the marker above — see its docstring.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub trait JobThreadTransfer {}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl<T: ?Sized> JobThreadTransfer for T {}

/// 🧩️ The protocol every interactive operation implements instead of a run-to-completion function
/// call — see the module doc's governing rule. `step` is bounded (checks [`StepContext::should_yield`]
/// and returns before the hard ceiling), cancellable ([`StepContext::is_cancelled`]) and explicitly
/// resumable (a fresh [`StepContext`] each call, job-owned state carries everything between calls).
pub trait InteractiveJob: JobThreadTransfer {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>;
    fn borrow_outcome<'a>(&'a self, descriptor:&'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>,ValueError>;
    fn begin_close(&mut self);
    /// 🎟️ Consumes physical and work effects once under the same original caller; successful progress reports that debit.
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
    /// 🧮️ Requests the next original payload work without granting it.
    fn next_close_copy_byte_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 📦️ Requests the actual natural frontier allocation for this bounded payload grant.
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes:usize) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// ♻️ Requests complete physical backing release independently of payload copying.
    fn next_close_release_byte_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 🪜️ Requests the exact original frontier depth without enlarging the caller's limit.
    fn next_close_depth_demand(&self) -> Result<usize,ValueError> {self.close_demand_without_owner()}
    /// 🔎️ Refuses undeclared retained demand instead of guessing a zero-credit owner.
    fn close_demand_without_owner(&self)->Result<usize,ValueError>{if self.terminal_is_empty(){Ok(0)}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"interactive job retained close demand is undeclared"))}}
    /// 🔔️ Registers the wake source that can advance a genuinely blocked close.
    fn register_close_wake(&self, _waker: &Waker) -> bool {
        false
    }
    fn terminal_is_empty(&self) -> bool;
}
//#endregion 🧩️InteractiveJob
