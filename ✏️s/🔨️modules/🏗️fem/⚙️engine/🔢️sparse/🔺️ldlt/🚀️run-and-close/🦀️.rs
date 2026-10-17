//! 🚀️ The LDLT job run loop, close turns and its interactive step.

use super::*;

impl LdltJob {
    fn run(&mut self, context: &mut StepContext<'_>) -> NumericalRun {
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return NumericalRun::Fault;
        }
        if self.state.admission_fault {
            return NumericalRun::Fault;
        }
        if self.state.checkpoint_due || self.checkpoint_writer.is_some() {
            context.set_stage("fem.ldlt.checkpoint-page");
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
            return self.desk.lend_checkpoint(state, self.state.column as u64);
        }
        if self.state.column == self.state.a.n {
            context.set_stage("fem.ldlt.output-entry");
            if context.should_yield() {
                return NumericalRun::Yield;
            }
            if self.output_writer.is_none() {
                context.consume_fuel(1);
                self.output_writer = Some(RetainedJobPayloadWriter::new(JobPayloadStream::CommitOutput));
                self.output_page_cursor = 0;
                return NumericalRun::Yield;
            }
            context.consume_fuel(1);
            let Some(writer) = self.output_writer.as_mut() else {
                return NumericalRun::Fault;
            };
            if writer.staged_page_len().is_none() {
                return match writer.begin_staged_page(context) {
                    Ok(()) => NumericalRun::Yield,
                    Err(_) => NumericalRun::Fault,
                };
            }
            let complete = match Self::advance_output_entry(&mut self.state, self.operation, writer) {
                Ok(complete) => complete,
                Err(_) => return NumericalRun::Fault,
            };
            if !complete {
                return NumericalRun::Yield;
            }
            let Some(writer) = self.output_writer.take() else {
                return NumericalRun::Fault;
            };
            let output = match writer.finish() {
                Ok(output) => output,
                Err(writer) => {
                    self.output_writer = Some(writer);
                    return NumericalRun::Fault;
                }
            };
            return self.desk.lend_complete(output);
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
        });
        if context.should_yield() {
            return NumericalRun::Yield;
        }
        context.consume_fuel(1);
        if self.advance_column_microcursor().is_err() {
            return NumericalRun::Fault;
        }
        if context.is_cancelled() {
            return NumericalRun::Cancelled;
        }
        NumericalRun::Yield
    }

    fn largest_owner_bytes(&self) -> usize {
        let state = &self.state;
        [state.a.colptr.capacity() * size_of::<u32>(), state.a.rowind.capacity() * size_of::<u32>(), state.a.vals.capacity() * size_of::<f64>(), state.d.capacity() * size_of::<f64>()]
            .into_iter()
            .chain(state.l_cols.iter().map(|column| column.capacity() * size_of::<(u32, f64)>()))
            .chain(state.row_lists.iter().map(|row| row.capacity() * size_of::<usize>()))
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
        if let Some(demand) = numerical_writer_demand(&self.checkpoint_writer) {
            return demand.map(Some);
        }
        if let Some(demand) = numerical_writer_demand(&self.output_writer) {
            return demand.map(Some);
        }
        Ok(None)
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
