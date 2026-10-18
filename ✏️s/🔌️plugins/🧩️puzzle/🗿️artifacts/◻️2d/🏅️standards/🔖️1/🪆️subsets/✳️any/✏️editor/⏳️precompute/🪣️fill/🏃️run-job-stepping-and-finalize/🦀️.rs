//! 🏃️ Run job stepping and finalize: the remaining operations of the fill run job and its interactive job implementation. See [`super`].

use super::*;

//#region ⏯️RunJob
impl Puzzle2dFillRunJob {
    pub(super) fn turn(&mut self, context: &mut StepContext<'_>) -> JobTurn {
        if self.closing || context.is_cancelled() {
            return JobTurn::Cancelled;
        }
        if let Some(placement) = self.retiring_placement.as_mut() {
            let step = placement.close_step(context.retained_grant());
            if matches!(step, InteractiveJobCloseStep::Refused { .. } | InteractiveJobCloseStep::Blocked) || context.consume_retained(step.progress()).is_err() {
                return fill_run_fault("puzzle2d-fill-placement-retirement");
            }
            if placement.terminal_is_empty() {
                self.retiring_placement = None;
            }
            return JobTurn::Yield;
        }
        if let Some(owed) = self.owed.take() {
            return self.settle_owed(context, owed);
        }
        loop {
            if self.finished {
                self.settle();
                return self.flush_then(context, FillRunOwed::Checkpoint);
            }
            if context.fuel_exhausted() || context.deadline_exceeded() || self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES {
                return self.flush(context).unwrap_or(JobTurn::Yield);
            }
            if let Some(capture) = self.capture.as_mut() {
                match capture.advance(self.document.value(), &self.kinds, FILL_RUN_CAPTURE_UNITS) {
                    Ok(None) => {}
                    Ok(Some(snapshot)) => {
                        let serial = capture.serial;
                        self.open_handles = capture.open_handles;
                        self.capture = None;
                        self.search = Some(BoardFillJob::with_operation(snapshot, u32::MAX, Operation::new(OperationId(serial), RevisionId(0), Generation(0), self.seed)));
                        self.stage = FillRunStage::Search;
                    }
                    Err(code) => return fill_run_fault(code),
                }
                continue;
            }
            let Some(search) = self.search.as_mut() else { return fill_run_fault("puzzle2d-fill-run-search-owner") };
            if search.stage() == BoardFillStage::Complete {
                let reason = self.stall_reason();
                self.stall(context, reason);
                continue;
            }
            let (outcome, event) = Self::search_batch(search, &mut self.search_sequence, context);
            let fault = match &outcome {
                Some(FillSearch::Fault) => search.take_fault(),
                _ => None,
            };
            self.stage = FillRunStage::of(search.stage());
            if let Some(event) = event {
                self.observe(context, event);
            }
            match outcome {
                None => {}
                Some(FillSearch::Cancelled) => return JobTurn::Cancelled,
                Some(outcome) => {
                    let checkpoint = matches!(outcome, FillSearch::Checkpoint);
                    match (checkpoint, fault) {
                        (true, _) => match self.accept(context) {
                            Ok(true) => return self.flush_then(context, FillRunOwed::Checkpoint),
                            Ok(false) => {}
                            Err(code) => return fill_run_fault(code),
                        },
                        (false, Some("placement-capacity")) => self.stall(context, FillRunReason::ArtifactCapacity),
                        (false, Some(code)) => return fill_run_fault(code),
                        (false, None) => return fill_run_fault("puzzle2d-fill-run-search-outcome"),
                    }
                }
            }
        }
    }
}

impl InteractiveJob for Puzzle2dFillRunJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx);
        self.outbox.settle(turn, cx)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        for job in [self.closing_search.as_mut(), self.search.as_mut()].into_iter().flatten() {
            InteractiveJob::begin_close(job);
        }
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.begin_close();
        if !self.outbox.terminal_is_empty() {
            return match self.outbox.close_step(grant) {
                Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) | semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)) => InteractiveJobCloseStep::Pending { progress },
                Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
            };
        }
        if grant.maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        let one = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(capture) = self.capture.as_mut() {
            let step = capture.close_one(child);
            if capture.is_closed() {
                self.capture = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } },
                step => step,
            };
        }
        if let Some(placement) = self.retiring_placement.as_mut() {
            let step = placement.close_step(child);
            if placement.terminal_is_empty() {
                self.retiring_placement = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } },
                step => step,
            };
        }
        for slot in [&mut self.closing_search, &mut self.search] {
            let Some(job) = slot.as_mut() else { continue };
            let step = InteractiveJob::close_step(job, child);
            if InteractiveJob::terminal_is_empty(job) {
                *slot = None;
            }
            return match step {
                InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } },
                step => step,
            };
        }
        if !self.owners_staged {
            self.owners_staged = true;
            let identity = self.writer.identity();
            self.close_owners.stage(Puzzle2dFillRunOwners {
                document: Arc::clone(&self.document),
                kinds: std::mem::take(&mut self.kinds),
                writer: std::mem::replace(&mut self.writer, ToolRunTickWriter::new(identity)),
                replay: self.replay.take(),
                placements: std::mem::take(&mut self.placements),
                target_regions: std::mem::take(&mut self.target_regions),
            });
            return InteractiveJobCloseStep::Pending { progress: one };
        }
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands()?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.capture.is_none() && self.search.is_none() && self.closing_search.is_none() && self.retiring_placement.is_none() && self.owners_staged && self.close_owners.is_empty()
    }
}

impl Puzzle2dFillRunJob {
    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.outbox.terminal_is_empty() {
            return self.outbox.retirement_demands();
        }
        if let Some(placement) = self.retiring_placement.as_ref() {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: placement.next_close_release_byte_demand(), depth: 1, ..Default::default() });
        }
        if let Some(capture) = self.capture.as_ref() {
            let demand = capture.close_demands()?;
            return Ok(semio_framework_value::RetirementDemand { depth: demand.depth.saturating_add(1), ..demand });
        }
        if let Some(job) = self.closing_search.as_ref().or(self.search.as_ref()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: job.next_close_copy_byte_demand()?, capacity_bytes: job.next_close_capacity_byte_demand(0)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()?.saturating_add(1) });
        }
        if self.owners_staged {
            return self.close_owners.demands(0);
        }
        Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })
    }
}
//#endregion ⏯️RunJob
