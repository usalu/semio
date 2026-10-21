//! 🚀️ The LDLT job run loop, close turns and its interactive step.

use super::*;

impl LdltJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> Result<NumericalRun, OriginalTurnFailure> {
        if context.is_cancelled() { return Ok(NumericalRun::Cancelled); }
        if context.should_yield() { return Ok(NumericalRun::Yield); }
        let result = self.run_admitted(context);
        context.consume_fuel(1);
        result
    }

    fn run_admitted(&mut self, context: &mut StepContext<'_>) -> Result<NumericalRun, OriginalTurnFailure> {
        if self.terminal_published { return Ok(NumericalRun::Yield); }
        if context.is_cancelled() {
            return Ok(NumericalRun::Cancelled);
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return Ok(NumericalRun::Fault);
        }
        if self.state.admission_fault {
            return Ok(NumericalRun::Fault);
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.ldlt.checkpoint-page");
            if self.checkpoint_writer.is_some() && self.checkpoint_cursor.field == u16::MAX {
                if !RetainedJobPayloadWriter::finish_funded(&mut self.checkpoint_writer, &mut self.desk.lent, context)? { return Ok(NumericalRun::Yield); }
                let payload = self.desk.lent.take().expect("finished numerical writer retained its original payload");
                return Ok(self.desk.lend_checkpoint(payload, self.state.column as u64));
            }

            if context.should_yield() {
                return Ok(NumericalRun::Yield);
            }
            if self.checkpoint_writer.is_none() {
                self.checkpoint_writer = RetainedJobPayloadWriter::new_funded(JobPayloadStream::CheckpointState, context)?;
                if self.checkpoint_writer.is_some() { self.state.checkpoint_due = false; self.checkpoint_cursor = NumericalPageCursor::new(); }
                return Ok(NumericalRun::Yield);
            }
            let Some(writer) = self.checkpoint_writer.as_mut() else {
                return Ok(NumericalRun::Fault);
            };
            if writer.staged_page_len().is_none() {
                writer.begin_staged_page_funded(context)?;
                return Ok(NumericalRun::Yield);
            }
            let complete = Self::advance_checkpoint_entry(&self.state, &mut self.checkpoint_cursor, writer, context)?;
            if !complete {
                return Ok(NumericalRun::Yield);
            }
            return Ok(NumericalRun::Yield);
        }
        if self.state.column == self.state.a.n {
            context.set_stage("fem.ldlt.output-entry");
            if self.output_writer.is_some() && self.state.publication_stage == 3 {
                let mut demand = RetainedJobPayloadWriter::finish_demand();
                demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "numerical terminal metadata overflow"))?;
                if !semio_s_fem_sparse::numerical_grant_covers(context.retained_grant(), demand) { return Ok(NumericalRun::Yield); }
                if !RetainedJobPayloadWriter::finish_funded(&mut self.output_writer, &mut self.desk.lent, context)? { return Ok(NumericalRun::Yield); }
                self.terminal_published = true;
                context.consume_retained(RetainedCloneProgress { copied_bytes: size_of::<bool>(), ..Default::default() })?;
                let payload = self.desk.lent.take().expect("finished numerical writer retained its original payload");
                return Ok(self.desk.lend_complete(payload));
            }

            if context.should_yield() {
                return Ok(NumericalRun::Yield);
            }
            if self.output_writer.is_none() {
                self.output_writer = RetainedJobPayloadWriter::new_funded(JobPayloadStream::CommitOutput, context)?;
                self.output_page_cursor = 0;
                return Ok(NumericalRun::Yield);
            }
            let Some(writer) = self.output_writer.as_mut() else {
                return Ok(NumericalRun::Fault);
            };
            if writer.staged_page_len().is_none() {
                writer.begin_staged_page_funded(context)?;
                return Ok(NumericalRun::Yield);
            }
            let complete = Self::advance_output_entry(&mut self.state, self.operation, writer, context)?;
            if !complete {
                return Ok(NumericalRun::Yield);
            }
            return Ok(NumericalRun::Yield);
        }
        context.set_stage(match self.state.cursor.stage {
            LdltColumnStage::ReserveColumn => "fem.ldlt.reserve-column",
            LdltColumnStage::SourceEntry => "fem.ldlt.source-entry",
            LdltColumnStage::ContributorLookup => "fem.ldlt.contributor-lookup",
            LdltColumnStage::ContributorEntry => "fem.ldlt.contributor-entry",
            LdltColumnStage::PivotRead => "fem.ldlt.pivot-read",
            LdltColumnStage::DiagonalCommit => "fem.ldlt.diagonal-commit",
            LdltColumnStage::EmitRow => "fem.ldlt.emit-row",
            LdltColumnStage::PublishColumn => "fem.ldlt.publish-column",
            LdltColumnStage::CompleteColumn => "fem.ldlt.complete-column",
            LdltColumnStage::SortRows => "fem.ldlt.sort-touched-rows",
        });
        if context.should_yield() {
            return Ok(NumericalRun::Yield);
        }
        if self.advance_column_microcursor(context).is_err() {
            return Ok(NumericalRun::Fault);
        }
        if context.is_cancelled() {
            return Ok(NumericalRun::Cancelled);
        }
        Ok(NumericalRun::Yield)
    }



    fn rung_demand(&self) -> RetirementDemand { numerical_rung_demand(self.close_terminal_is_empty()) }

    pub(super) fn close_frontier(&self) -> Result<Option<RetirementDemand>, ValueError> {
        if let Some(demand) = self.desk.retirement_demands() {
            return demand.map(Some);
        }
        if let Some(demand) = numerical_writer_demand(&self.checkpoint_writer) {
            return demand.map(Some);
        }
        if let Some(demand) = numerical_writer_demand(&self.output_writer) {
            return demand.map(Some);
        }
        ldlt_checkpoint_retirement_demand(&self.state)
    }

    fn close_demand(&self) -> Result<RetirementDemand, ValueError> {
        Ok(self.close_frontier()?.unwrap_or_else(|| self.rung_demand()))
    }

    fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
        let frontier = self.close_frontier()?;
        if !numerical_close_gate(grant, frontier) {
            return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        let step = if let Some(step) = self.desk.close_step(grant) {
            step?
        } else if let Some(step) = numerical_writer_close(&mut self.checkpoint_writer, grant) {
            step?
        } else if let Some(step) = numerical_writer_close(&mut self.output_writer, grant) {
            step?
        } else {
            numerical_ladder_turn(self.close_retained_step(grant.maximum_release_bytes))
        };
        step.admit(grant, self.terminal_is_empty())
    }
}

impl InteractiveJob for LdltJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {
        if self.desk.retire_due(cx, self.operation) {
            self.desk.retire_delivered(cx)?;
            return Ok(None);
        }
        let run = match self.desk.take_pending() {
            Some(run) => run,
            None => self.run(cx)?,
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
        if let Some(writer) = self.output_writer.as_mut() {
            writer.begin_close();
        }
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let demand = self.close_demand()?;
        numerical_close_original(cx, self, demand, Self::close_turn, Self::terminal_is_empty)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demand()?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.checkpoint_writer.is_none() && self.output_writer.is_none() && self.desk.terminal_is_empty() && self.close_terminal_is_empty()
    }
}
