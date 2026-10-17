//! 🔍️ Revalidate job of the reconstruction session: the interactive job that recomputes the quality report of a stored reconstruction. See [`super`].

use super::*;

//#region 🔍️RevalidateJob
/// ✅️ The finalize `revalidateJob`: when the head's reconstruction inputs no longer equal the run's, every
/// provisional op is withdrawn with one danger step; otherwise the provisional result stands.
pub struct ReconstructionRevalidateJob {
    pub(super) identity: ToolRunIdentity,
    pub(super) head: Option<Arc<RemodelingSnapshot>>,
    run_inputs: Option<RemodelingContentDigest>,
    pub(super) provisional: u32,
    pub(super) owed: Option<Owed>,
    pub(super) outbox: Outbox,
    pub(super) closing: bool,
}

impl ReconstructionRevalidateJob {
    pub fn new(identity: ToolRunIdentity, head: Arc<RemodelingSnapshot>, checkpoint: Option<&[u8]>, provisional: u32) -> Self {
        Self { identity, head: Some(head), run_inputs: checkpoint.and_then(ReconstructionRunCheckpoint::decode).map(|checkpoint| checkpoint.inputs), provisional, owed: None, outbox: Outbox::default(), closing: false }
    }
}

impl InteractiveJob for ReconstructionRevalidateJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.closing || cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx);
        }
        match self.outbox.phase(cx)? {
            OutboxPhase::Building => return self.outbox.advance(cx),
            OutboxPhase::Delivered | OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            OutboxPhase::Idle => {}
        }
        if let Some(owed) = self.owed.take() {
            return match owed {
                Owed::Complete => JobOutcomeBorrow::admit_complete(cx, None, None),
                Owed::Checkpoint | Owed::Fault => {
                    self.outbox.begin(JobPublicationKind::Fault, Vec::new());
                    cx.consume_fuel(1);
                    Ok(None)
                }
            };
        }
        let Some(head) = self.head.take() else {
            self.outbox.begin(JobPublicationKind::Fault, Vec::new());
            cx.consume_fuel(1);
            return Ok(None);
        };
        let unchanged = self.run_inputs == Some(reconstruction_inputs(&head));
        let mut writer = ToolRunTickWriter::with_provisional_base(self.identity, self.provisional);
        if !unchanged {
            writer.retract_to(0);
            let _ = writer.step(ToolRunStepKind::Danger, ReconstructionRunStage::Products.index(), ReconstructionRunReason::InputsChanged.code(), None, &[ToolRunStepArg::Unsigned(u64::from(self.provisional))]);
            let _ = writer.step(ToolRunStepKind::Danger, ReconstructionRunStage::Products.index(), TOOL_RUN_REASON_CONFLICT, None, &[ToolRunStepArg::Unsigned(u64::from(self.provisional))]);
        }
        cx.consume_fuel(1);
        writer.progress(ToolRunProgress {
            identity: self.identity,
            sequence: 0,
            state: ToolRunState::Finalizing,
            stage: ReconstructionRunStage::Products.index(),
            completed: 1,
            total: Some(1),
            counters: Vec::new(),
            units_per_second: 0.0,
            conflicts: if unchanged { 0 } else { self.provisional },
            steps: ToolRunStepRing::new(),
        });
        match writer.finish().and_then(|tick| tick.encode().ok()) {
            Some(bytes) => {
                self.owed = Some(Owed::Complete);
                self.outbox.begin(JobPublicationKind::Preview, bytes);
            }
            None => self.outbox.begin(JobPublicationKind::Fault, Vec::new()),
        }
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        borrow_remodel_outcome(&self.outbox, descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.closing = true;
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        if let Some(step) = close_outbox_turn(&mut self.outbox, grant) {
            return step;
        }
        if self.head.take().is_some() || self.owed.take().is_some() {
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        }
        InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.outbox.retirement_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.outbox.retirement_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.outbox.retirement_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        let publication = self.outbox.retirement_demands()?.depth;
        Ok(if publication == 0 { usize::from(!self.terminal_is_empty()) } else { publication })
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.head.is_none() && self.owed.is_none() && self.outbox.terminal_is_empty()
    }
}
//#endregion 🔍️RevalidateJob
