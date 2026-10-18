//! 🔎️ Reserved outcome witnesses of the tool job contract: the owner witness, the reserved job outcomes and kinds, the receive and close helpers, the bounded fault text and the reducer fault detail. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToolOwnerWitness {
    owner_type_id: std::any::TypeId,
    pub owner_type_name: &'static str,
}

impl ToolOwnerWitness {
    pub fn of<A: ArtifactApp>() -> Self {
        Self { owner_type_id: std::any::TypeId::of::<A>(), owner_type_name: std::any::type_name::<A>() }
    }

    #[doc(hidden)]
    pub fn test_with_name(owner_type_name: &'static str) -> Self {
        Self { owner_type_id: std::any::TypeId::of::<()>(), owner_type_name }
    }
}

/// 🚦️ The semantic outcome whose original source bytes the job keeps stable until it is delivered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReservedOutcomeKind {
    Preview,
    Checkpoint { applied_progress: u64 },
    Fault,
    Commit,
}

/// 🧳️ Enclosing job owns this inline storage; each turn funds one original publication frontier.
pub struct ReservedJobOutcomes {
    pub(super) publication: RetainedJobPublication,
    commit: RetainedPayloadBuilder,
    commit_cursor: usize,
    delivered: bool,
}

/// 🧾️ Admits one received step receipt into the caller's wallet, refused or not.
pub fn receive_step(cx: &mut StepContext<'_>, step: Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
    match step {
        Ok(step) => {
            cx.consume_retained(step.progress())?;
            Ok(step)
        }
        Err(error) => {
            cx.consume_retained(error.retained_progress())?;
            Err(error)
        }
    }
}

/// 🔁️ Maps one paid close receipt onto the job close protocol; completion is the job's own terminal witness.
pub(crate) fn close_result(step:Result<RetainedCloneStep,ValueError>)->Result<InteractiveJobCloseStep,ValueError>{
    step.map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()})
}

impl ReservedJobOutcomes {
    pub fn new() -> Self {
        Self { publication: RetainedJobPublication::new(), commit: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput), commit_cursor: 0, delivered: false }
    }

    /// 📬️ Whether a lent outcome still owns its payload pages and must be retired before the next one.
    pub fn is_delivered(&self) -> bool {
        self.delivered
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty() && self.commit.terminal_is_empty()
    }

    /// ♻️ Retires the previous publication one paid turn at a time inside the wallet of a running step.
    pub fn retire(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        if !self.publication.terminal_is_empty() {
            let step = self.publication.close_step(cx.retained_grant());
            receive_step(cx, step)?;
        }
        if self.publication.terminal_is_empty() {
            self.delivered = false;
        }
        Ok(())
    }

    /// ✍️ Advances one paid frontier of the pending outcome and lends it once its original pages are sealed.
    pub fn advance<'a>(&'a mut self, kind: ReservedOutcomeKind, source: &[u8], cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let publication_kind = match kind {
            ReservedOutcomeKind::Preview => JobPublicationKind::Preview,
            ReservedOutcomeKind::Checkpoint { applied_progress } => JobPublicationKind::Checkpoint { applied_progress },
            ReservedOutcomeKind::Fault => JobPublicationKind::Fault,
            ReservedOutcomeKind::Commit => {
                if !self.commit.is_initialized() {
                    self.commit.advance_initialization(cx)?;
                    return Ok(None);
                }
                if self.commit_cursor < source.len() {
                    self.commit.append_original(cx, source, &mut self.commit_cursor)?;
                    return Ok(None);
                }
                if self.commit.published().is_none() {
                    self.commit.seal(cx)?;
                    return Ok(None);
                }
                let original = JobOutcomeBorrow::admit_complete(cx, None, self.commit.published())?;
                self.delivered |= original.is_some();
                return Ok(original);
            }
        };
        let original = self.publication.advance_from_source(publication_kind, source, cx)?;
        self.delivered |= original.is_some();
        Ok(original)
    }

    /// 🤝️ Resolves a descriptor against the same original pages this job lent.
    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, self.commit.published()),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
        }
    }

    /// 📏️ Quotes the next original page, builder or metadata turn without granting it.
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if !self.commit.terminal_is_empty() {
            return self.commit.retirement_demands();
        }
        Ok(RetirementDemand::default())
    }

    /// ♻️ Closes the publication before the commit pages, never both in one turn.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(grant);
        }
        if !self.commit.terminal_is_empty() {
            let step = self.commit.close_step_granted(grant)?;
            if self.commit.terminal_is_empty() {
                self.commit_cursor = 0;
                self.delivered = false;
            }
            return Ok(step);
        }
        self.delivered = false;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }
}

impl Default for ReservedJobOutcomes {
    fn default() -> Self {
        Self::new()
    }
}

/// 🧯️ One bounded fault text kept inline so its publication source never moves or allocates.
pub struct BoundedFaultText {
    pub(super) bytes: [u8; Self::MAXIMUM_BYTES],
    length: usize,
}

impl BoundedFaultText {
    const MAXIMUM_BYTES: usize = 480;

    pub fn new() -> Self {
        Self { bytes: [0; Self::MAXIMUM_BYTES], length: 0 }
    }

    /// ✍️ Stores the longest whole-character prefix of `text` that fits.
    pub fn set(&mut self, text: &str) {
        let mut length = text.len().min(Self::MAXIMUM_BYTES);
        while !text.is_char_boundary(length) {
            length -= 1;
        }
        self.bytes[..length].copy_from_slice(&text.as_bytes()[..length]);
        self.length = length;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.length]
    }
}

impl Default for BoundedFaultText {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔎️ Recovers the exact typed reducer fault while leaving framework-authored plain fault pages distinct.
pub fn reducer_fault_of_detail(detail: &[u8]) -> Option<Fault> {
    semio_framework_diagnostic::try_decode_fault_bytes(detail)
}
