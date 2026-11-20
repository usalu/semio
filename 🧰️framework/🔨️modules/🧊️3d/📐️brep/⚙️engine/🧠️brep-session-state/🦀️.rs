//! 🧠️ Native B-Rep session and resumable general Boolean jobs.

use super::*;
use semio_framework_value::retirement::{RetireOwned, RetirementSupport};

/// 🧭️ A native B-Rep session.
#[repr(transparent)]
pub struct Brep {
    pub(super) native: BrepRegistry,
}

impl RetireOwned for Brep {
    type RetirementCursor = <BrepRegistry as RetireOwned>::RetirementCursor;

    fn retirement_ready(&self) -> bool {
        self.native.retirement_ready()
    }

    fn retirement_seed(self) -> Self::RetirementCursor {
        self.native.retirement_seed()
    }

    fn controlled_retirement_supported() -> bool {
        BrepRegistry::controlled_retirement_supported()
    }

    fn controlled_retirement_supported_in(path: &RetirementSupport<'_>) -> bool {
        BrepRegistry::controlled_retirement_supported_in(path)
    }
}

impl Default for Brep {
    fn default() -> Self {
        Self::new()
    }
}

impl Brep {
    /// 🏗️ Empty native kernel session.
    pub fn new() -> Self {
        Self { native: BrepRegistry::new() }
    }
}

/// ⏱️ A retained resumable boolean plus the operation recorder its whole run accumulates into —
/// the recorder must outlive every step, which is why the job owns it rather than borrowing the
/// caller's (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`).
pub struct BrepBooleanJob {
    pub(super) job: BooleanJob,
    pub(super) rec: OpRecorder,
}

semio_framework_value::artifact_retire_struct!(BrepBooleanJob {job,rec});

impl BrepBooleanJob {
    /// 📈️ Progress right now — safe to read between steps and after termination.
    pub fn progress(&self) -> BooleanProgress {
        self.job.progress()
    }

    /// 🛑️ Retires this boolean at the next observable boundary.
    pub fn cancel(&mut self) {
        self.job.cancel();
    }

    /// ✅️ True once the job reached a terminal phase.
    pub fn is_terminal(&self) -> bool {
        self.job.is_terminal()
    }
}

/// 🔀️ What [`Brep::boolean_job_sync`] admitted.
pub enum BrepBooleanAdmission {
    /// ⚡️ A fast path answered exactly and in place; there is nothing to resume.
    Answered(GeometryHandle),
    /// ⏱️ The general exact engine, as a budgetable job.
    Job(BrepBooleanJob),
}

/// ⏱️ Outcome of one [`Brep::step_boolean_job_sync`].
pub enum BrepBooleanStep {
    /// 🔁 Budget spent, work remains.
    Working(BooleanProgress),
    /// ✅️ Terminal: the result solid, registered as a live handle.
    Ready(GeometryHandle),
    /// 🛑️ Terminal: the job was cancelled; nothing is produced.
    Cancelled(BooleanProgress),
    /// ❌️ Terminal: the exact kernel refused the operands or its result; the caller localizes the refusal.
    Failed(BrepError),
}
