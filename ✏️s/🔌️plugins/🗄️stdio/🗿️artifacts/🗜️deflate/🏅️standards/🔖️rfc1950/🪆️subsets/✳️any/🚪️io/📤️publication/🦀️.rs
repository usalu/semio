//! 📤️ Resumable publication of bounded DEFLATE checkpoint state and committed output bytes.

use semio_framework_job::{
    InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeView, JobPayloadStream, JobPublicationKind, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPublication, RetainedPayloadBuilder, StepContext,
};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

#[derive(Clone, Copy)]
pub enum Kind {
    Checkpoint(u64),
    Commit,
}

/// 🧯️ Maps one child close turn onto the parent job's close vocabulary; a child never completes its parent.
pub fn close_result(result: Result<RetainedCloneStep, ValueError>) -> InteractiveJobCloseStep {
    match result {
        Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
        Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
    }
}

/// 🧳️ One publication owns its source bytes and the original payload headers it lends to the consumer.
pub struct Publication {
    kind: Kind,
    single: RetainedJobPublication,
    state: RetainedPayloadBuilder,
    output: RetainedPayloadBuilder,
    primary: Vec<u8>,
    secondary: Vec<u8>,
    state_cursor: usize,
    output_cursor: usize,
    delivered: bool,
}

impl Publication {
    pub fn new(kind: Kind, primary: Vec<u8>, secondary: Vec<u8>) -> Box<Self> {
        Box::new(Self {
            kind,
            single: RetainedJobPublication::new(),
            state: RetainedPayloadBuilder::new(JobPayloadStream::CommitState),
            output: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput),
            primary,
            secondary,
            state_cursor: 0,
            output_cursor: 0,
            delivered: false,
        })
    }

    pub fn is_delivered(&self) -> bool {
        self.delivered
    }

    /// 📈️ The applied input progress a checkpoint publication certifies; `None` for a commit.
    #[cfg(test)]
    pub fn applied_progress(&self) -> Option<u64> {
        match self.kind {
            Kind::Checkpoint(applied_progress) => Some(applied_progress),
            Kind::Commit => None,
        }
    }

    /// 🤝️ The sealed commit output payload, readable only after the whole commit was delivered.
    #[cfg(test)]
    pub fn commit_output(&self) -> Option<&semio_framework_job::RetainedJobPayload> {
        if self.delivered && matches!(self.kind, Kind::Commit) { self.output.published() } else { None }
    }

    /// ✍️ Advances one paid unit toward the lent outcome and reports it once the payload can be borrowed.
    pub fn poll<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let kind = match self.kind {
            Kind::Checkpoint(applied_progress) => JobPublicationKind::Checkpoint { applied_progress },
            Kind::Commit => return self.poll_commit(cx),
        };
        let result = self.single.advance_from_source(kind, &self.primary, cx)?;
        if result.is_some() {
            self.delivered = true;
        }
        Ok(result)
    }

    fn poll_commit<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if !self.state.is_initialized() {
            self.state.advance_initialization(cx)?;
            return Ok(None);
        }
        if self.state_cursor < self.primary.len() {
            self.state.append_original(cx, &self.primary, &mut self.state_cursor)?;
            return Ok(None);
        }
        if self.state.published().is_none() {
            self.state.seal(cx)?;
            return Ok(None);
        }
        if !self.output.is_initialized() {
            self.output.advance_initialization(cx)?;
            return Ok(None);
        }
        if self.output_cursor < self.secondary.len() {
            self.output.append_original(cx, &self.secondary, &mut self.output_cursor)?;
            return Ok(None);
        }
        if self.output.published().is_none() {
            self.output.seal(cx)?;
            return Ok(None);
        }
        let result = JobOutcomeBorrow::admit_complete(cx, self.state.published(), self.output.published())?;
        if result.is_some() {
            self.delivered = true;
        }
        Ok(result)
    }

    /// 🤝️ Resolves the immutable descriptor against these same original payload headers.
    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match self.kind {
            Kind::Commit if self.delivered => descriptor.complete(self.state.published(), self.output.published()),
            Kind::Commit => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "deflate commit publication has not lent its payloads")),
            _ => self.single.borrow_outcome(descriptor),
        }
    }

    /// 📏️ Quotes the next close frontier: pages and headers first, then the source bytes.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.single.terminal_is_empty() {
            return self.single.retirement_demands();
        }
        if !self.state.terminal_is_empty() {
            return self.state.retirement_demands();
        }
        if !self.output.terminal_is_empty() {
            return self.output.retirement_demands();
        }
        if self.primary.capacity() != 0 {
            return Ok(RetirementDemand { release_bytes: self.primary.capacity(), depth: 1, ..Default::default() });
        }
        if self.secondary.capacity() != 0 {
            return Ok(RetirementDemand { release_bytes: self.secondary.capacity(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let demand = match self.retirement_demands() {
            Ok(demand) => demand,
            Err(error) => return InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        };
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        if !self.single.terminal_is_empty() {
            return close_result(self.single.close_step(grant));
        }
        if !self.state.terminal_is_empty() {
            return close_result(self.state.close_step_granted(grant));
        }
        if !self.output.terminal_is_empty() {
            return close_result(self.output.close_step_granted(grant));
        }
        if self.primary.capacity() != 0 {
            let released_bytes = self.primary.capacity();
            self.primary = Vec::new();
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } };
        }
        if self.secondary.capacity() != 0 {
            let released_bytes = self.secondary.capacity();
            self.secondary = Vec::new();
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() } };
        }
        if self.delivered {
            self.delivered = false;
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.single.terminal_is_empty() && self.state.terminal_is_empty() && self.output.terminal_is_empty() && self.primary.capacity() == 0 && self.secondary.capacity() == 0 && !self.delivered
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
