//! 🏁️ The PCG job run loop, its interactive step and the one-shot `pcg` entry.

use super::*;

// #region 🔖️Pcg
impl PcgJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        if self.closing || self.publication_fault.is_some() {
            return NumericalRun::Fault;
        }
        if self.terminal_published { return NumericalRun::Yield; }
        let publication_kind = self.publication.as_ref().map(|publication| publication.kind).or_else(|| {
            if self.state.stage == PcgStage::Complete { Some(PcgPublicationKind::Complete) }
            else if self.state.preview_due { Some(PcgPublicationKind::Preview) }
            else if self.state.checkpoint_due { Some(PcgPublicationKind::Checkpoint) }
            else { None }
        });
        if let Some(kind) = publication_kind {
            context.set_stage(match kind {
                PcgPublicationKind::Checkpoint => "fem.pcg.checkpoint-page",
                PcgPublicationKind::Preview => "fem.pcg.preview-page",
                PcgPublicationKind::Complete => "fem.pcg.complete-page",
            });
            return match self.step_publication(context, kind) {
                Ok(outcome) => outcome,
                Err(fault) => {
                    self.publication_fault = Some(fault);
                    NumericalRun::Fault
                }
            };
        }
        context.set_stage("fem.pcg");
        let mut units = 0;
        while units < self.state.batch_units && !context.should_yield() && self.state.stage != PcgStage::Complete && !self.state.preview_due && !self.state.checkpoint_due {
            match self.state.stage {
                PcgStage::InitializeDiagonal => self.step_diagonal(context, &mut units),
                PcgStage::InitialSpmv => self.step_spmv(context, &mut units, true),
                PcgStage::InitialResidual => self.step_initial_residual(context, &mut units),
                PcgStage::InitialPrecondition => self.step_precondition(context, &mut units, true),
                PcgStage::IterationSpmv => self.step_spmv(context, &mut units, false),
                PcgStage::IterationUpdate => self.step_update(context, &mut units),
                PcgStage::IterationPrecondition => self.step_precondition(context, &mut units, false),
                PcgStage::IterationDirection => self.step_direction(context, &mut units),
                PcgStage::Complete => {}
            }
            if context.is_cancelled() {
                return NumericalRun::Cancelled;
            }
        }
        NumericalRun::Yield
    }

    pub(super) fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(demand) = self.desk.retirement_demands() {
            return demand.map(Some);
        }
        if let Some(publication) = self.publication.as_ref() {
            return publication.writer.retirement_demands().map(Some);
        }
        Ok(None)
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| numerical_rung_demand(self.close_lane >= 8)))
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        if let Some(step) = self.desk.close_step(grant) {
            return step;
        }
        if let Some(publication) = self.publication.as_mut() {
            publication.writer.begin_close();
            let step = numerical_close_result(publication.writer.close_step(grant))?;
            if publication.writer.terminal_is_empty() {
                self.publication = None;
            }
            return Ok(step);
        }
        Ok(numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes)))
    }

    /// 🎟️ One close turn under an explicit grant; the parents that own a job (construction, restore) and the
    /// original-context `close_step` both admit it through the job's own quote.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let frontier = self.close_frontier()?;
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        if !numerical_close_gate(grant, frontier) {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        self.close_turn(grant)?.admit(grant, self.terminal_is_empty())
    }
}

impl InteractiveJob for PcgJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.desk.retire_due(cx, self.operation) {
            self.desk.retire_delivered(cx)?;
            return Ok(None);
        }
        let run = match self.desk.take_pending() {
            Some(run) => run,
            None => self.run(cx),
        };
        Ok(self.desk.admit(cx, run)?)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        self.desk.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(publication) = self.publication.as_mut() { publication.writer.begin_close(); }
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let demand = self.close_demand()?;
        numerical_close_original(cx, self, demand, Self::close_granted, Self::terminal_is_empty)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.close_lane == 8 && self.publication.is_none() && self.desk.terminal_is_empty()
    }
}

/// ➰️ Jacobi-preconditioned conjugate gradient — mutates `x0` in place, converges when
/// `‖r‖ / ‖b‖ < tol_rel` or `max_iter` is reached.
pub fn pcg(a: &Csr, b: &VecD, x0: &mut VecD, tol_rel: f64, max_iter: usize) -> PcgStats {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = PcgJob::new(operation, a.clone(), b.clone(), x0.clone(), tol_rel, max_iter, 1_024);
    let mut preview_sequence = 0;
    loop {
        match numerical_batch_step(&mut job, operation, &mut preview_sequence) {
            NumericalBatchEnd::Complete => {
                let (solution, stats) = job.solution();
                *x0 = solution.clone();
                numerical_batch_close(&mut job);
                return stats;
            }
            NumericalBatchEnd::Fault => {
                numerical_batch_close(&mut job);
                panic!("PCG batch execution faulted");
            }
            NumericalBatchEnd::Cancelled => {
                numerical_batch_close(&mut job);
                panic!("PCG batch execution cancelled");
            }
            NumericalBatchEnd::Continue => {}
        }
    }
}
// #endregion 🔖️Pcg
