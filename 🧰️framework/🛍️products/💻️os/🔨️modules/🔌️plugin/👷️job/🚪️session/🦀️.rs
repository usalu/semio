//! 🚪️ Plugin callers admit worker sessions from their own funded identity and retire a refused source by paid turns.
use semio_framework_job::{BatchJobParams, BatchJobSession, InteractiveJob, InteractiveJobCloseStep, MountedWorkerJobSession, StepBudget, WorkerJobAdmissionContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError};

/// 🧾️ The original job and parameters a refused admission leaves with its caller until they are physically closed.
pub(crate) struct RefusedWorkerSource<J: InteractiveJob> {
    job: Option<J>,
    params: Option<BatchJobParams>,
    error: Option<ValueError>,
    closing: bool,
}

impl<J: InteractiveJob> RefusedWorkerSource<J> {
    /// 🛑️ The structural refusal, absent when the session slots or the grant were merely exhausted.
    pub(crate) fn error(&self) -> Option<&ValueError> {
        self.error.as_ref()
    }

    pub(crate) fn job(&self) -> Option<&J> {
        self.job.as_ref()
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.params.is_none()
    }

    /// 🪙️ Quotes the next original job close turn, else the metadata turn of the original parameters.
    pub(crate) fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(job) = self.job.as_ref() {
            return Ok(RetirementDemand { copy_bytes: job.next_close_copy_byte_demand()?, capacity_bytes: job.next_close_capacity_byte_demand(maximum_copy_bytes)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()? });
        }
        Ok(RetirementDemand { depth: usize::from(self.params.is_some()), ..Default::default() })
    }

    pub(crate) fn begin_close(&mut self) {
        if !self.closing {
            self.closing = true;
            if let Some(job) = self.job.as_mut() {
                job.begin_close();
            }
        }
    }

    /// ♻️ Retires the original job through its quoted close demand, then the original parameters.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.begin_close();
        let idle = RetainedCloneProgress::default();
        let item = RetainedCloneProgress { copied_items: 1, ..idle };
        if let Some(job) = self.job.as_mut() {
            if job.terminal_is_empty() {
                if grant.maximum_items == 0 {
                    return InteractiveJobCloseStep::Pending { progress: idle };
                }
                drop(self.job.take());
                return InteractiveJobCloseStep::Pending { progress: item };
            }
            let terminal = job.terminal_is_empty();
            return match job.close_step(grant).admit(grant, terminal) {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                step => step,
            };
        }
        if self.params.is_some() {
            if grant.maximum_items == 0 {
                return InteractiveJobCloseStep::Pending { progress: idle };
            }
            drop(self.params.take());
            return InteractiveJobCloseStep::Pending { progress: item };
        }
        InteractiveJobCloseStep::Complete { progress: idle }
    }
}

impl<J: InteractiveJob> Drop for RefusedWorkerSource<J> {
    fn drop(&mut self) {
        debug_assert!(std::thread::panicking() || self.terminal_is_empty(), "refused worker session source reached Drop before its exact close");
    }
}

fn refused<J: InteractiveJob>(job: Option<J>, params: Option<BatchJobParams>, error: Option<ValueError>) -> RefusedWorkerSource<J> {
    RefusedWorkerSource { job, params, error, closing: false }
}

macro_rules! admit_session {
    ($session:ident, $job:expr, $params:expr, $grant:expr) => {{
        let mut job = Some($job);
        let mut params = Some($params);
        let original = params.as_ref().expect("original session parameters");
        let (operation, generation, now_us) = (original.operation, original.generation, original.now_us);
        let mut recipient = RetainedCloneProgress::default();
        match WorkerJobAdmissionContext::new(operation, generation, StepBudget::new(1, u64::MAX, $grant), now_us, &mut recipient) {
            Ok(mut control) => match $session::try_admit_owned(&mut job, &mut params, &mut control) {
                Ok(Some((session, _admission))) => Ok(session),
                Ok(None) => Err(refused(job, params, None)),
                Err(error) => Err(refused(job, params, Some(error))),
            },
            Err(error) => Err(refused(job, params, Some(error))),
        }
    }};
}

/// 🚪️ Admits one mounted session under the caller's own preparation grant.
pub(crate) fn admit_mounted<J: InteractiveJob + 'static>(job: J, params: BatchJobParams, grant: RetainedCloneGrant) -> Result<MountedWorkerJobSession<J>, RefusedWorkerSource<J>> {
    admit_session!(MountedWorkerJobSession, job, params, grant)
}

/// 🚪️ Admits one caller-driven batch session under the caller's own preparation grant.
pub(crate) fn admit_batch<J: InteractiveJob + 'static>(job: J, params: BatchJobParams, grant: RetainedCloneGrant) -> Result<BatchJobSession<J>, RefusedWorkerSource<J>> {
    admit_session!(BatchJobSession, job, params, grant)
}
