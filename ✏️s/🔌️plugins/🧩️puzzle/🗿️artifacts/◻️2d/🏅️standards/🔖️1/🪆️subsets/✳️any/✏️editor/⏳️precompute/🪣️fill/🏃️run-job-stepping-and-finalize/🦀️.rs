//! 🏃️ Run job stepping and finalize: the remaining operations of the fill run job and its interactive job implementation. See [`super`].

use super::*;

//#region ⏯️RunJob
impl Puzzle2dFillRunJob {
    pub(super) fn turn(&mut self, context: &mut StepContext<'_>) -> Result<JobTurn, semio_framework_job::OriginalTurnFailure> {
        if self.closing || context.is_cancelled() {
            return Ok(JobTurn::Cancelled);
        }
        if let Some(placement) = self.retiring_placement.as_mut() {
            let demand = semio_framework_value::RetirementDemand { release_bytes: placement.next_close_release_byte_demand(), depth: 1, ..Default::default() };
            semio_framework_job::close_original_retained_step(context, demand, |grant| Ok(match placement.close_step(grant) {
                InteractiveJobCloseStep::Complete { progress } => semio_framework_value::RetainedCloneStep::Complete(progress),
                step => semio_framework_value::RetainedCloneStep::Progress(step.progress()),
            }))?;
            if placement.terminal_is_empty() {
                self.retiring_placement = None;
            }
            return Ok(JobTurn::Yield);
        }
        if let Some(owed) = self.owed.take() {
            return Ok(self.settle_owed(context, owed));
        }
        loop {
            if self.finished {
                self.settle();
                return Ok(self.flush_then(context, FillRunOwed::Checkpoint));
            }
            if context.fuel_exhausted() || context.deadline_exceeded() || self.writer.pending_bytes() >= FILL_RUN_TICK_FLUSH_BYTES {
                return Ok(self.flush(context).unwrap_or(JobTurn::Yield));
            }
            if self.pending_snapshot.is_some() {
                let operation = Operation::new(OperationId(self.pending_search_serial), RevisionId(0), Generation(0), self.seed);
                if !BoardFillJob::admit_original(&mut self.pending_snapshot, u32::MAX, operation, &mut self.search, context)? { return Ok(JobTurn::Yield); }
                self.stage = FillRunStage::Search;
                continue;
            }
            if let Some(capture) = self.capture.as_mut() {
                if capture.ingress.is_none() && !BoardFillSnapshotIngress::admit_original(capture.suggestion_offset, &PUZZLE_2D_BOARD_NAMING, &mut capture.ingress, context)? { return Ok(JobTurn::Yield); }
                match capture.advance(self.document.as_ref().expect("original fill snapshot remains until close").value(), &self.kinds, FILL_RUN_CAPTURE_UNITS) {
                    Ok(None) => {}
                    Ok(Some(snapshot)) => {
                        self.pending_search_serial = capture.serial;
                        self.open_handles = capture.open_handles;
                        self.capture = None;
                        self.pending_snapshot = Some(snapshot);
                    }
                    Err(code) => return Ok(fill_run_fault(code)),
                }
                continue;
            }
            let Some(search) = self.search.as_mut() else { return Ok(fill_run_fault("puzzle2d-fill-run-search-owner")); };
            if search.stage() == BoardFillStage::Complete {
                let reason = self.stall_reason();
                self.stall(context, reason);
                continue;
            }
            let (outcome, event) = Self::search_batch(search, context)?;
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
                Some(FillSearch::Cancelled) => return Ok(JobTurn::Cancelled),
                Some(outcome) => {
                    let checkpoint = matches!(outcome, FillSearch::Checkpoint);
                    match (checkpoint, fault) {
                        (true, _) => match self.accept(context) {
                            Ok(true) => return Ok(self.flush_then(context, FillRunOwed::Checkpoint)),
                            Ok(false) => {}
                            Err(code) => return Ok(fill_run_fault(code)),
                        },
                        (false, Some("placement-capacity")) => self.stall(context, FillRunReason::ArtifactCapacity),
                        (false, Some(code)) => return Ok(fill_run_fault(code)),
                        (false, None) => return Ok(fill_run_fault("puzzle2d-fill-run-search-outcome")),
                    }
                }
            }
        }
    }
}

impl Puzzle2dFillRunJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        match self.outbox.phase(cx)? {
            crate::puzzle_job::OutboxPhase::Building => return self.outbox.advance(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
            crate::puzzle_job::OutboxPhase::Delivered | crate::puzzle_job::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            crate::puzzle_job::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx)?;
        self.outbox.settle(turn, cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

impl InteractiveJob for Puzzle2dFillRunJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
        for job in [self.closing_search.as_mut(), self.search.as_mut()].into_iter().flatten() {
            InteractiveJob::begin_close(job);
        }
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        semio_framework_job::close_original_nested_step(cx, |cx, _grant| {

        self.begin_close();
        if !self.outbox.terminal_is_empty() {
            let demand = self.outbox.retirement_demands()?;
            return semio_framework_job::close_original_retained_step(cx, demand, |grant| self.outbox.close_step(grant));
        }
        if let Some(snapshot) = self.pending_snapshot.as_mut() {
            let step = snapshot.close_step(cx)?;
            if snapshot.terminal_is_empty() { self.pending_snapshot = None; }
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        if let Some(capture) = self.capture.as_mut() {
            let step = capture.close_one(cx)?;
            if capture.is_closed() { self.capture = None; }
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        if let Some(placement) = self.retiring_placement.as_mut() {
            let demand = semio_framework_value::RetirementDemand { release_bytes: placement.next_close_release_byte_demand(), depth: 1, ..Default::default() };
            let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(match placement.close_step(grant) {
                InteractiveJobCloseStep::Complete { progress } => semio_framework_value::RetainedCloneStep::Complete(progress),
                step => semio_framework_value::RetainedCloneStep::Progress(step.progress()),
            }))?;
            if placement.terminal_is_empty() { self.retiring_placement = None; }
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        for slot in [&mut self.closing_search, &mut self.search] {
            let Some(job) = slot.as_mut() else { continue };
            let step = InteractiveJob::close_step(job, cx)?;
            if InteractiveJob::terminal_is_empty(job) { *slot = None; }
            return Ok(InteractiveJobCloseStep::Pending { progress: step.progress() });
        }
        if !self.owners_staged {
            self.owners_staged = true;
            let identity = self.writer.identity();
            self.close_owners.stage(Puzzle2dFillRunOwners {
                document: self.document.take(), kinds: std::mem::take(&mut self.kinds),
                writer: std::mem::replace(&mut self.writer, ToolRunTickWriter::new(identity)),
                replay: self.replay.take(), placements: std::mem::take(&mut self.placements), target_regions: std::mem::take(&mut self.target_regions),
            });
        }
        self.close_owners.close_step(cx)
    
        })
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
        self.closing && self.pending_snapshot.is_none() && self.document.is_none() && self.outbox.terminal_is_empty() && self.capture.is_none() && self.search.is_none() && self.closing_search.is_none() && self.retiring_placement.is_none() && self.owners_staged && self.close_owners.is_empty()
    }
}

impl Puzzle2dFillRunJob {
    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if !self.outbox.terminal_is_empty() {
            return self.outbox.retirement_demands();
        }
        if let Some(snapshot) = self.pending_snapshot.as_ref() { return snapshot.next_close_demands(0); }
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
