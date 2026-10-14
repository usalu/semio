//! ⏱️ The one resumable-operation contract every expensive B-Rep operation implements: a plan of
//! counted phases, one unit of real algorithm work per [`StagedOperation::advance`], monotone
//! [`StageProgress`], and a single result. The one-shot function of an operation is [`drive`] over
//! its job, so there is exactly one algorithm per operation (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`,
//! lane J; the boolean twin is `semio_framework_3d_brep_boolean::BooleanJob`).

use semio_framework_3d_brep_representation::arena::{FaceId, SolidId};
use semio_framework_3d_brep_representation::error::KernelError;
use semio_framework_3d_brep_representation::topology::history::OpRecorder;
use semio_framework_3d_brep_representation::topology::Body;

/// 📈 Progress of one staged operation. `done` never decreases, `total` is the plan known so far and
/// is only ever revised upward (a nested job reveals its own size when it is admitted), and a
/// finished operation has `done == total`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StageProgress {
    pub done: usize,
    pub total: usize,
    pub phase: &'static str,
}

/// 📦 What a finished staged operation produced, as arena ids of the body it ran in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StageOutput {
    Solid(SolidId),
    Face(FaceId),
    Faces(Vec<FaceId>),
    Solids(Vec<SolidId>),
}

/// 🔁 Outcome of one [`StagedOperation::advance`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StageStep {
    /// 🔁 One unit done, more remain.
    Working,
    /// ✅ The last unit ran; the operation's result.
    Done(StageOutput),
}

/// 🔌 A resumable operation. `advance` runs exactly one unit; the job owns every intermediate so a
/// host may keep it across turns and present the same body on every call.
pub trait StagedOperation: Send {
    /// 📈 Progress right now — safe to read between steps.
    fn progress(&self) -> StageProgress;
    /// ⏱️ Runs the next unit against `body`.
    fn advance(&mut self, body: &mut Body, rec: &mut OpRecorder) -> Result<StageStep, KernelError>;
}

/// ♾️ Runs every remaining unit — the one-shot façade of any staged operation.
pub fn drive<J: StagedOperation + ?Sized>(job: &mut J, body: &mut Body, rec: &mut OpRecorder) -> Result<StageOutput, KernelError> {
    loop {
        if let StageStep::Done(output) = job.advance(body, rec)? {
            return Ok(output);
        }
    }
}

/// ♾️ [`drive`] for an operation that yields one solid.
pub fn drive_solid<J: StagedOperation + ?Sized>(job: &mut J, body: &mut Body, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    match drive(job, body, rec)? {
        StageOutput::Solid(solid) => Ok(solid),
        other => Err(KernelError::Operation(format!("staged operation did not yield a solid: {other:?}"))),
    }
}

/// ♾️ [`drive`] for an operation that yields one face.
pub fn drive_face<J: StagedOperation + ?Sized>(job: &mut J, body: &mut Body, rec: &mut OpRecorder) -> Result<FaceId, KernelError> {
    match drive(job, body, rec)? {
        StageOutput::Face(face) => Ok(face),
        other => Err(KernelError::Operation(format!("staged operation did not yield a face: {other:?}"))),
    }
}

/// 🧭 Address of one unit inside a [`Plan`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    pub phase: usize,
    pub index: usize,
    pub last: bool,
}

/// 🗺️ Ordered, counted phases. Taking a unit advances the cursor past empty phases, so an operation
/// whose plan has zero edges to blend simply has no such phase to run.
#[derive(Clone, Debug)]
pub struct Plan {
    phases: Vec<(&'static str, usize)>,
    phase: usize,
    within: usize,
    done: usize,
}

impl Plan {
    /// 🗺️ A plan of `(tag, unit count)` phases in execution order.
    pub fn new(phases: &[(&'static str, usize)]) -> Self {
        Self { phases: phases.to_vec(), phase: 0, within: 0, done: 0 }
    }

    /// 🧮 Units in the whole plan as known now.
    pub fn total(&self) -> usize {
        self.phases.iter().map(|(_, count)| count).sum()
    }

    /// ➕ Revises one phase's unit count upward once a nested job reveals its own size.
    pub fn grow(&mut self, phase: usize, by: usize) {
        self.phases[phase].1 += by;
    }

    /// 🏁 Drops every planned unit that was never needed, so a finished operation reports
    /// `done == total` even when a nested job planned pessimistically (a boolean skips the imprint
    /// rows of coincident faces). The only way `total` ever shrinks, and only at completion.
    pub fn settle(&mut self) {
        if let Some(entry) = self.phases.get_mut(self.phase) {
            entry.1 = self.within;
        }
        for entry in self.phases.iter_mut().skip(self.phase + 1) {
            entry.1 = 0;
        }
    }

    /// 🎯 The next unit's address, or `None` when the plan is exhausted.
    pub fn take(&mut self) -> Option<Unit> {
        while self.phase < self.phases.len() && self.within >= self.phases[self.phase].1 {
            self.phase += 1;
            self.within = 0;
        }
        if self.phase >= self.phases.len() {
            return None;
        }
        let (phase, index) = (self.phase, self.within);
        self.within += 1;
        self.done += 1;
        Some(Unit { phase, index, last: self.done == self.total() })
    }

    /// 📈 Progress right now: the phase tag is the one the last taken unit belonged to.
    pub fn progress(&self) -> StageProgress {
        let tag = self.phases.get(self.phase).or(self.phases.last()).map_or("", |(tag, _)| *tag);
        StageProgress { done: self.done, total: self.total(), phase: tag }
    }
}

/// 🧵 Sums the work of consecutive nested stages into one plan phase: tracks how large the current
/// child has reported itself to be so the parent plan only ever grows by the difference.
#[derive(Clone, Copy, Debug, Default)]
pub struct ChildLedger {
    seen: usize,
}

impl ChildLedger {
    /// 🧵 A child was just admitted into a phase that budgeted one placeholder unit for it.
    pub fn admit(&mut self, plan: &mut Plan, phase: usize, child: &impl StagedOperation) {
        let total = child.progress().total;
        plan.grow(phase, total.saturating_sub(1));
        self.seen = total;
    }

    /// 🧵 The child advanced; account for any upward revision of its own plan.
    pub fn follow(&mut self, plan: &mut Plan, phase: usize, child: &impl StagedOperation) {
        let total = child.progress().total;
        if total > self.seen {
            plan.grow(phase, total - self.seen);
            self.seen = total;
        }
    }
}
