//! 🧵️ The job-side lifecycle every puzzle job shares: a bounded outbox that lends payload outcomes, a staged owner bundle that retires through the controlled owner, and counted ladders for engine-owned scaffolds.
use semio_framework_job::{InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, JobPublicationKind, RetainedJobPublication, RetainedPayloadBuilder, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{controlled::ControlledRetirement, RetireOwned};
use semio_framework_value::{RetirementDemand, ValueError};

/// ♻️ Moves a work's remaining owners into one derived bundle and retires it through the controlled owner, one granted turn at a time.
pub struct WorkClosing<T: RetireOwned + 'static> {
    staged: std::mem::ManuallyDrop<Option<T>>,
    retirement: std::mem::ManuallyDrop<Option<ControlledRetirement<T>>>,
}

impl<T: RetireOwned + 'static> Default for WorkClosing<T> {
    fn default() -> Self {
        Self { staged: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }
}

impl<T: RetireOwned + 'static> WorkClosing<T> {
    /// 🧺️ Parks the work's remaining owners; the first granted turn births their controlled retirement.
    pub fn stage(&mut self, owners: T) {
        debug_assert!(self.staged.is_none() && self.retirement.is_none(), "a work stages its remaining owners exactly once");
        *self.staged = Some(owners);
    }

    pub fn is_empty(&self) -> bool {
        self.staged.is_none() && self.retirement.is_none()
    }

    /// 📏️ Quotes the single turn the next close step spends.
    pub fn demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? });
        }
        Ok(RetirementDemand { depth: usize::from(self.staged.is_some()), ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let empty = RetainedCloneProgress::default();
        if self.is_empty() {
            return InteractiveJobCloseStep::Complete { progress: empty };
        }
        if grant.maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { progress: empty };
        }
        if let Some(owner) = self.retirement.as_mut() {
            let step = owner.step(grant);
            if owner.terminal_is_empty() {
                self.retirement.take();
            }
            return match step {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
                Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
            };
        }
        let Some(value) = self.staged.take() else { return InteractiveJobCloseStep::Complete { progress: empty } };
        match ControlledRetirement::new(value) {
            Ok(owner) => {
                *self.retirement = Some(owner);
                InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..empty } }
            }
            Err((error, value)) => {
                *self.staged = Some(value);
                InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() }
            }
        }
    }
}

impl<T: RetireOwned + 'static> Drop for WorkClosing<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "a puzzle command work must retire its remaining owners before it drops");
    }
}

/// 📤️ What one turn of a puzzle job settles: a lent control outcome, or bytes the job must publish before its next turn.
/// 📎️ `Prepare` stages the reserved route's raw wire as the retained commit output the host compares with the wire it admitted; the job completes only once [`JobOutbox::commit_prepared`] holds, so a build refusal can never follow a delivered completion.
#[derive(Debug, PartialEq, Eq)]
pub enum JobTurn {
    Yield,
    Cancelled,
    Complete,
    Prepare(Vec<u8>),
    Preview(Vec<u8>),
    Checkpoint { applied_progress: u64, state: Vec<u8> },
    Fault(Vec<u8>),
}

impl JobTurn {
    /// 🏁️ Whether the job is done after this turn.
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Complete | Self::Cancelled | Self::Fault(_))
    }
}

/// 🧭️ Where one publication stands between the job and its driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutboxPhase {
    Idle,
    Building,
    Delivered,
    Retiring,
}

/// 📤️ One original payload source and the publication that lends it; the source never moves while the loan lives.
pub struct JobOutbox {
    publication: RetainedJobPublication,
    commit: RetainedPayloadBuilder,
    commit_cursor: usize,
    committing: bool,
    source: Vec<u8>,
    kind: Option<JobPublicationKind>,
    retiring: bool,
}

impl Default for JobOutbox {
    fn default() -> Self {
        Self { publication: RetainedJobPublication::default(), commit: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput), commit_cursor: 0, committing: false, source: Vec::new(), kind: None, retiring: false }
    }
}

impl JobOutbox {
    /// 📎️ Whether the staged commit output is sealed and will ride on the job's completion.
    pub fn commit_prepared(&self) -> bool {
        self.commit.published().is_some()
    }

    pub fn phase(&self, cx: &StepContext<'_>) -> Result<OutboxPhase, ValueError> {
        if self.committing && !self.commit_prepared() {
            return Ok(OutboxPhase::Building);
        }
        let Some(kind) = self.kind else { return Ok(OutboxPhase::Idle) };
        if self.retiring {
            return Ok(OutboxPhase::Retiring);
        }
        let demand = self.publication.advance_demands(kind, &self.source, cx)?;
        let delivered = self.publication.published().is_some() && demand.copy_bytes == 0 && demand.capacity_bytes == 0 && demand.release_bytes == 0 && demand.depth == 0;
        Ok(if delivered { OutboxPhase::Delivered } else { OutboxPhase::Building })
    }

    pub fn advance<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.committing {
            if !self.commit.is_initialized() {
                self.commit.advance_initialization(cx)?;
            } else if self.commit_cursor < self.source.len() {
                self.commit.append_original(cx, &self.source, &mut self.commit_cursor)?;
            } else {
                self.commit.seal(cx)?;
            }
            return Ok(None);
        }
        let kind = self.kind.expect("a building outbox owns its publication kind");
        self.publication.advance_from_source(kind, &self.source, cx)
    }

    pub fn retire_step(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        self.retiring = true;
        match self.publication.close_step(cx.retained_grant())? {
            RetainedCloneStep::Progress(progress) => cx.consume_retained(progress),
            RetainedCloneStep::Complete(progress) => {
                cx.consume_retained(progress)?;
                self.kind = None;
                self.retiring = false;
                self.source = Vec::new();
                Ok(())
            }
        }
    }

    /// 🎫️ Settles a turn: control outcomes are lent at once, payload outcomes stage their bytes for the next paid turns.
    pub fn settle<'a>(&'a mut self, turn: JobTurn, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let (kind, source) = match turn {
            JobTurn::Yield => return JobOutcomeBorrow::admit_yield(cx),
            JobTurn::Cancelled => return JobOutcomeBorrow::admit_cancelled(cx),
            JobTurn::Complete => return JobOutcomeBorrow::admit_complete(cx, None, self.commit.published()),
            JobTurn::Prepare(bytes) => {
                self.source = bytes;
                self.committing = true;
                cx.consume_fuel(1);
                return Ok(None);
            }
            JobTurn::Preview(bytes) => (JobPublicationKind::Preview, bytes),
            JobTurn::Checkpoint { applied_progress, state } => (JobPublicationKind::Checkpoint { applied_progress }, state),
            JobTurn::Fault(bytes) => (JobPublicationKind::Fault, bytes),
        };
        self.source = source;
        self.kind = Some(kind);
        cx.consume_fuel(1);
        Ok(None)
    }

    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, self.commit.published()),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
        }
    }

    /// ♻️ Closes the lent publication first, then the staged commit pages, then clears the staging flags.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if self.kind.is_some() || !self.publication.terminal_is_empty() {
            self.retiring = true;
            let step = self.publication.close_step(grant)?;
            if self.publication.terminal_is_empty() {
                self.kind = None;
                self.retiring = false;
                if !self.committing {
                    self.source = Vec::new();
                }
            }
            return Ok(step);
        }
        if !self.commit.terminal_is_empty() {
            let step = self.commit.close_step_granted(grant)?;
            if self.commit.terminal_is_empty() {
                self.commit_cursor = 0;
                self.committing = false;
                self.source = Vec::new();
            }
            return Ok(step);
        }
        self.commit_cursor = 0;
        self.committing = false;
        self.source = Vec::new();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }

    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.kind.is_some() || !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if !self.commit.terminal_is_empty() {
            return self.commit.retirement_demands();
        }
        Ok(RetirementDemand::default())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.kind.is_none() && !self.committing && self.publication.terminal_is_empty() && self.commit.terminal_is_empty()
    }
}

/// ♻️ Closes a job: the outbox first, then the staged owner bundle.
pub fn job_close_step<T: RetireOwned + 'static>(outbox: &mut JobOutbox, owners: &mut WorkClosing<T>, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
    if !outbox.terminal_is_empty() {
        return match outbox.close_step(grant) {
            Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
            Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        };
    }
    owners.close_step(grant)
}

/// 📏️ Quotes the next close turn of a job: the outbox first, then the staged owner bundle.
pub fn job_close_demands<T: RetireOwned + 'static>(outbox: &JobOutbox, owners: &WorkClosing<T>, copy: usize) -> Result<RetirementDemand, ValueError> {
    if !outbox.terminal_is_empty() {
        return outbox.retirement_demands();
    }
    owners.demands(copy)
}

/// 🧪️ Test seams that drive a puzzle job the way its production wiring does.
#[cfg(test)]
pub mod testing {
    use super::*;
    use semio_framework_job::InteractiveJob;

    /// 🎟️ A grant no test turn can exhaust.
    pub fn unbounded_grant() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }
    }

    /// 🧾️ A fresh retained-progress wallet for a test step context; the test process owns it until exit.
    pub fn test_progress() -> &'static mut RetainedCloneProgress {
        Box::leak(Box::default())
    }

    /// 🧹️ Closes `job` to its terminal-empty shell and answers how many granted turns that took.
    pub fn close_job<J: InteractiveJob + ?Sized>(job: &mut J) -> usize {
        job.begin_close();
        let mut turns = 0;
        while !matches!(job.close_step(unbounded_grant()), InteractiveJobCloseStep::Complete { .. }) {
            turns += 1;
            assert!(turns < 1_000_000, "a job closes in a bounded number of turns");
        }
        assert!(job.terminal_is_empty(), "a closed job is terminal-empty");
        turns
    }
    fn payload_bytes(payload: &semio_framework_job::RetainedJobPayload) -> Vec<u8> {
        (0..payload.page_count()).flat_map(|index| payload.page(index).unwrap_or_default().to_vec()).collect()
    }

    /// ▶️ Steps `job` through its production `step` — paying turns included — until it lends one outcome, and reads that outcome as owned bytes.
    pub fn drive<J: InteractiveJob + ?Sized>(job: &mut J, budget: semio_framework_job::StepBudget, clock: fn() -> Option<u64>, sequence: &mut u64) -> JobTurn {
        for _ in 0..1_000_000 {
            let mut cx = StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), budget, semio_framework_job::root_cancel_token(), clock, sequence, test_progress());
            let turn = match job.step(&mut cx).expect("a job step") {
                None => continue,
                Some(JobOutcomeBorrow::Yield { .. }) => JobTurn::Yield,
                Some(JobOutcomeBorrow::Cancelled { .. }) => JobTurn::Cancelled,
                Some(JobOutcomeBorrow::Complete { .. }) => JobTurn::Complete,
                Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => JobTurn::Preview(payload_bytes(payload)),
                Some(JobOutcomeBorrow::CheckpointReady { state, applied_progress, .. }) => JobTurn::Checkpoint { applied_progress, state: payload_bytes(state) },
                Some(JobOutcomeBorrow::Fault { detail, .. }) => JobTurn::Fault(payload_bytes(detail)),
            };
            return turn;
        }
        panic!("a job lends an outcome within a bounded number of paying turns");
    }
    /// ▶️ Like [`drive`], and also answers the commit output the completing turn carried (`None` for every other outcome).
    pub fn drive_output<J: InteractiveJob + ?Sized>(job: &mut J, budget: semio_framework_job::StepBudget, clock: fn() -> Option<u64>, sequence: &mut u64) -> (JobTurn, Option<Vec<u8>>) {
        for _ in 0..1_000_000 {
            let mut cx = StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), budget, semio_framework_job::root_cancel_token(), clock, sequence, test_progress());
            match job.step(&mut cx).expect("a job step") {
                None => continue,
                Some(JobOutcomeBorrow::Complete { output, .. }) => return (JobTurn::Complete, output.map(payload_bytes)),
                Some(JobOutcomeBorrow::Yield { .. }) => return (JobTurn::Yield, None),
                Some(JobOutcomeBorrow::Cancelled { .. }) => return (JobTurn::Cancelled, None),
                Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => return (JobTurn::Preview(payload_bytes(payload)), None),
                Some(JobOutcomeBorrow::CheckpointReady { state, applied_progress, .. }) => return (JobTurn::Checkpoint { applied_progress, state: payload_bytes(state) }, None),
                Some(JobOutcomeBorrow::Fault { detail, .. }) => return (JobTurn::Fault(payload_bytes(detail)), None),
            }
        }
        panic!("a job lends an outcome within a bounded number of paying turns");
    }

    /// 🎟️ One close or maintenance turn funded exactly by what the owner quoted, capped at `items` items.
    pub fn funded_turn(demand: RetirementDemand, items: usize) -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
    }

    /// 🧹️ One cooperative-maintenance unit of `app`, funded from its own quote over a `body`-byte body.
    pub fn maintain_items<A: semio_framework_plugin::PluginApp + ?Sized>(app: &mut A, items: usize, body: usize) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::Fault> {
        let demand = app.maintenance_retirement_demands(body).map_err(|error| semio_framework_plugin::Fault::from(error.into_message()))?;
        app.maintenance_step(funded_turn(demand, items))
    }

    /// 🧹️ One single-item cooperative-maintenance unit of `app`.
    pub fn maintain<A: semio_framework_plugin::PluginApp + ?Sized>(app: &mut A, body: usize) -> Result<semio_framework_plugin::PluginLifecycleStep, semio_framework_plugin::Fault> {
        maintain_items(app, 1, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::InteractiveJob;

    struct EchoJob {
        raw: Vec<u8>,
        outbox: JobOutbox,
        closing: bool,
        owners: WorkClosing<Vec<u8>>,
    }

    impl EchoJob {
        fn new(raw: Vec<u8>) -> Self {
            Self { raw, outbox: JobOutbox::default(), closing: false, owners: WorkClosing::default() }
        }

        fn turn(&mut self) -> JobTurn {
            if !self.raw.is_empty() {
                return JobTurn::Prepare(std::mem::take(&mut self.raw));
            }
            JobTurn::Complete
        }
    }

    impl InteractiveJob for EchoJob {
        fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
            match self.outbox.phase(cx)? {
                OutboxPhase::Building => return self.outbox.advance(cx),
                OutboxPhase::Delivered | OutboxPhase::Retiring => {
                    self.outbox.retire_step(cx)?;
                    return Ok(None);
                }
                OutboxPhase::Idle => {}
            }
            let turn = self.turn();
            self.outbox.settle(turn, cx)
        }

        fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
            self.outbox.borrow_outcome(descriptor)
        }

        fn begin_close(&mut self) {
            if !std::mem::replace(&mut self.closing, true) {
                self.owners.stage(std::mem::take(&mut self.raw));
            }
        }

        fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
            self.begin_close();
            job_close_step(&mut self.outbox, &mut self.owners, grant)
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
        }

        fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
        }

        fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
        }

        fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
            Ok(job_close_demands(&self.outbox, &self.owners, 0)?.depth)
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
        }
    }

    /// 📎️ The host compares a reserved route's retained commit output with the wire it admitted: a job that holds the wire answers it back on completion, and an empty wire needs no output.
    #[test]
    fn a_prepared_commit_output_rides_on_the_completion_and_closes_exactly() {
        for raw in [Vec::new(), b"puzzle reserved wire".to_vec(), vec![0xA5; 1_000]] {
            let mut job = EchoJob::new(raw.clone());
            let mut sequence = 0;
            let budget = semio_framework_job::StepBudget::new(1, u64::MAX, testing::unbounded_grant());
            let (turn, output) = testing::drive_output(&mut job, budget, semio_framework_job::default_now_us, &mut sequence);
            assert_eq!(turn, JobTurn::Complete);
            assert_eq!(output, (!raw.is_empty()).then_some(raw), "the completion carries exactly the wire the job was admitted with");
            testing::close_job(&mut job);
        }
    }
}
