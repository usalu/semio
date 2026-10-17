//! 🏃️ The subspace job run loop, its interactive step, close turns and the one-shot `subspace_iteration`.

use super::*;

// #region 🔖️SubspaceIteration
impl SubspaceIterationJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        if !self.state.factor_validation_complete {
            context.set_stage("fem.subspace.validate-factor-owner");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            if let Some(column) = self.state.k_factor.l_cols.get(self.state.factor_validation_cursor) {
                if column.capacity().saturating_mul(size_of::<(u32, f64)>()) > NUMERICAL_OWNER_PAGE_BYTES {
                    self.state.admission_fault = true;
                    self.state.factor_validation_complete = true;
                    return NumericalRun::Fault;
                }
                self.state.factor_validation_cursor += 1;
                return NumericalRun::Yield;
            }
            self.state.factor_validation_complete = true;
            return NumericalRun::Yield;
        }
        if self.state.admission_fault || self.state.n > SUBSPACE_MAXIMUM_ORDER || self.state.m > SUBSPACE_MAXIMUM_COLUMNS {
            return NumericalRun::Fault;
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.subspace.checkpoint-page");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            if self.checkpoint_writer.is_none() {
                self.state.checkpoint_due = false;
                self.checkpoint_cursor = NumericalPageCursor::new();
                self.checkpoint_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState));
                return NumericalRun::Yield;
            }
            let Some(writer) = self.checkpoint_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_checkpoint_entry(&self.state, &mut self.checkpoint_cursor, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.checkpoint_writer.take() else {
                return NumericalRun::Fault;
            };
            let state = match writer.finish() {
                Ok(state) => state,
                Err(writer) => {
                    self.checkpoint_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            return self.desk.lend_checkpoint(state, self.state.iteration as u64);
        }
        if self.state.preview_due {
            context.set_stage("fem.subspace.preview-entry");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            if self.preview_writer.is_none() {
                context.consume_fuel(1);
                self.preview_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::Preview));
                self.preview_page_cursor = 0;
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            let Some(writer) = self.preview_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_preview_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.preview_writer.take() else {
                return NumericalRun::Fault;
            };
            let preview = match writer.finish() {
                Ok(preview) => preview,
                Err(writer) => {
                    self.preview_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            self.preview_page_cursor = 0;
            self.state.preview_due = false;
            return self.desk.lend_preview(preview);
        }
        if self.state.iteration >= self.state.max_iter || self.state.converged {
            context.set_stage("fem.subspace.terminal-page");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            if self.terminal_writer.is_none() {
                self.state.publication_stage = 0;
                self.state.publication_first = 0;
                self.state.publication_second = 0;
                self.terminal_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
                self.terminal_page_cursor = 0;
                return NumericalRun::Yield;
            }
            let Some(writer) = self.terminal_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() && self.state.publication_stage < 4 {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_preview_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.terminal_writer.take() else {
                return NumericalRun::Fault;
            };
            let output = match writer.finish() {
                Ok(output) => output,
                Err(writer) => {
                    self.terminal_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            return self.desk.lend_complete(output);
        }
        context.set_stage(match self.state.work.stage {
            SubspaceStage::ReserveIteration => "fem.subspace.reserve-iteration",
            SubspaceStage::ApplyOperatorColumnRow => "fem.subspace.apply-operator-column-row",
            SubspaceStage::FactorForwardEntry => "fem.subspace.factor-forward-entry",
            SubspaceStage::FactorDiagonalEntry => "fem.subspace.factor-diagonal-entry",
            SubspaceStage::FactorBackwardEntry => "fem.subspace.factor-backward-entry",
            SubspaceStage::OrthogonalizePairElement => "fem.subspace.orthogonalize-pair-element",
            SubspaceStage::NormalizeColumnElement => "fem.subspace.normalize-column-element",
            SubspaceStage::ProjectedMatrixCellEntry => "fem.subspace.projected-matrix-cell-entry",
            SubspaceStage::JacobiFindPairCell => "fem.subspace.jacobi-find-pair-cell",
            SubspaceStage::JacobiRotateCell => "fem.subspace.jacobi-rotate-cell",
            SubspaceStage::JacobiConvergenceCell => "fem.subspace.jacobi-convergence-cell",
            SubspaceStage::ModeSortCompare => "fem.subspace.mode-sort-compare",
            SubspaceStage::ModePermuteElement => "fem.subspace.mode-permute-element",
            SubspaceStage::ResidualColumnRow => "fem.subspace.residual-column-row",
            SubspaceStage::ConvergenceMode => "fem.subspace.convergence-mode",
            SubspaceStage::PublishIteration => "fem.subspace.publish-iteration",
        });
        if context.should_yield() {
            return NumericalRun::Yield;
        }
        context.consume_fuel(1);
        let result = match self.state.work.stage {
            SubspaceStage::ReserveIteration => self.reserve_iteration_owner(),
            SubspaceStage::ApplyOperatorColumnRow => {
                self.advance_apply_operator();
                Ok(())
            }
            SubspaceStage::FactorForwardEntry => {
                self.advance_factor_forward();
                Ok(())
            }
            SubspaceStage::FactorDiagonalEntry => {
                self.advance_factor_diagonal();
                Ok(())
            }
            SubspaceStage::FactorBackwardEntry => {
                self.advance_factor_backward();
                Ok(())
            }
            SubspaceStage::OrthogonalizePairElement => {
                self.advance_orthogonalize();
                Ok(())
            }
            SubspaceStage::NormalizeColumnElement => {
                self.advance_normalize();
                Ok(())
            }
            SubspaceStage::ProjectedMatrixCellEntry => {
                self.advance_projected();
                Ok(())
            }
            SubspaceStage::JacobiFindPairCell => {
                self.advance_jacobi_pair();
                Ok(())
            }
            SubspaceStage::JacobiRotateCell => {
                self.advance_jacobi_rotation();
                Ok(())
            }
            SubspaceStage::JacobiConvergenceCell => {
                self.advance_jacobi_convergence();
                Ok(())
            }
            SubspaceStage::ModeSortCompare => {
                self.advance_mode_sort();
                Ok(())
            }
            SubspaceStage::ModePermuteElement => {
                self.advance_mode_permute();
                Ok(())
            }
            SubspaceStage::ResidualColumnRow => {
                self.advance_residual();
                Ok(())
            }
            SubspaceStage::ConvergenceMode => {
                self.advance_convergence();
                Ok(())
            }
            SubspaceStage::PublishIteration => {
                self.publish_iteration();
                Ok(())
            }
        };
        if result.is_err() {
            return NumericalRun::Fault;
        }
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        NumericalRun::Yield
    }

    fn largest_owner_bytes(&self) -> usize {
        let state = &self.state;
        [state.k_factor.d.capacity() * size_of::<f64>(), state.x.data.capacity() * size_of::<f64>(), state.prev_theta.capacity() * size_of::<f64>(), state.final_theta.capacity() * size_of::<f64>(), state.residuals.capacity() * size_of::<f64>()]
            .into_iter()
            .chain(state.k_factor.l_cols.iter().map(|column| column.capacity() * size_of::<(u32, f64)>()))
            .max()
            .unwrap_or(0)
    }

    fn rung_demand(&self) -> RetirementDemand {
        let mut demand = numerical_rung_demand(self.close_terminal_is_empty());
        if self.state.admission_fault {
            demand.release_bytes = demand.release_bytes.max(self.largest_owner_bytes());
        }
        demand
    }

    pub(super) fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(demand) = self.desk.retirement_demands() {
            return demand.map(Some);
        }
        for slot in [&self.checkpoint_writer, &self.preview_writer, &self.terminal_writer] {
            if let Some(demand) = numerical_writer_demand(slot) {
                return demand.map(Some);
            }
        }
        Ok(None)
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| self.rung_demand()))
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        if let Some(step) = self.desk.close_step(grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.checkpoint_writer, grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.preview_writer, grant) {
            return step;
        }
        if let Some(step) = numerical_writer_close(&mut self.terminal_writer, grant) {
            return step;
        }
        Ok(numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes)))
    }
}

impl InteractiveJob for SubspaceIterationJob {
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
        if let Some(writer) = self.checkpoint_writer.as_mut() {
            writer.begin_close();
        }
        if let Some(writer) = self.preview_writer.as_mut() {
            writer.begin_close();
        }
        if let Some(writer) = self.terminal_writer.as_mut() {
            writer.begin_close();
        }
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
        self.checkpoint_writer.is_none() && self.preview_writer.is_none() && self.terminal_writer.is_none() && self.desk.terminal_is_empty() && self.close_terminal_is_empty()
    }
}

impl SubspaceIterationJob {
    /// 🎟️ One close turn under an explicit grant: the owners of a child job and the original-context `close_step`
    /// both admit it through the job's own quote.
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

/// 🎯️ Finds the lowest `p` generalized eigenpairs through the retained subspace cursor.
pub fn subspace_iteration(k_factor: &LdltFactor, b: &Csr, n: usize, p: usize, max_iter: usize) -> EigenPairs {
    let operation = Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    let mut job = SubspaceIterationJob::new(operation, k_factor.clone(), b.clone(), n, p, max_iter);
    let mut preview_sequence = 0;
    loop {
        match numerical_batch_step(&mut job, operation, &mut preview_sequence) {
            NumericalBatchEnd::Complete => {
                let solution = job.solution();
                numerical_batch_close(&mut job);
                return solution;
            }
            NumericalBatchEnd::Fault => {
                numerical_batch_close(&mut job);
                panic!("subspace batch adapter faulted")
            }
            NumericalBatchEnd::Cancelled | NumericalBatchEnd::Continue => {}
        }
    }
}
// #endregion 🔖️SubspaceIteration
