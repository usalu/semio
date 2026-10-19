//! 🚚️ Publication hand-off and job steps of the zlib codec: the delivered publication retirement and the interactive job steps of the deflate encoder. See [`super`].

use super::*;

//#region 🦑️DissolvedEngineCodec
//#region DeflateCodec
//#region 🧵️StreamingEncode
/// ♻️ Closes the lent publication from the next step's own wallet before encoding resumes.
pub(super) fn retire_delivered_publication<'a>(publication: &mut Option<Box<Publication>>, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
    let Some(active) = publication.as_mut() else { return Ok(None) };
    let step = match active.close_step(context.retained_grant()) { Ok(step) => step, Err(error) => { context.consume_retained(error.retained_progress())?; return Err(error) } };
    context.consume_retained(step.progress())?;
    if active.terminal_is_empty() {
        *publication = None;
    }
    Ok(None)
}

/// ➡️ One paid unit of the staged publication.
pub(super) fn drive_publication<'a>(publication: &'a mut Option<Box<Publication>>, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
    context.set_stage("deflate:publish");
    publication.as_mut().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "deflate job has no staged publication"))?.poll(context)
}

/// 🤝️ Resolves a lent outcome against the staged publication.
pub(super) fn borrow_publication_outcome<'a>(publication: &'a Option<Box<Publication>>, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
    use semio_framework_job::JobOutcomeKind;
    match descriptor.kind() {
        JobOutcomeKind::Yield => descriptor.yielded(),
        JobOutcomeKind::Cancelled => descriptor.cancelled(),
        _ => publication.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "deflate outcome has no staged publication"))?.borrow_outcome(descriptor),
    }
}

/// 📏️ The close frontier of the staged publication, then of the job's own backings.
pub(super) fn publication_demands(publication: &Option<Box<Publication>>, backing_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    match publication {
        Some(active) => active.retirement_demands(),
        None => Ok(semio_framework_value::RetirementDemand { release_bytes: backing_bytes, depth: usize::from(backing_bytes != 0), ..Default::default() }),
    }
}

impl DeflateEncodeJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.publication.as_ref().is_some_and(|publication| publication.is_delivered()) {
            return retire_delivered_publication(&mut self.publication, context);
        }
        if context.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(context);
        }
        if self.publication.is_some() {
            return drive_publication(&mut self.publication, context);
        }
        context.set_stage("deflate:encode");
        let literal_codes = build_codes(&fixed_lit_lengths());
        let distance_codes = build_codes(&fixed_dist_lengths());
        loop {
            if self.complete {
                self.stage_commit();
                return drive_publication(&mut self.publication, context);
            }
            let work = self.process_transition(&literal_codes, &distance_codes);
            context.consume_fuel(work as u64);
            if context.is_cancelled() {
                return semio_framework_job::JobOutcomeBorrow::admit_cancelled(context);
            }
            if self.complete {
                self.stage_commit();
                return drive_publication(&mut self.publication, context);
            }
            if self.position >= self.next_checkpoint {
                while self.next_checkpoint <= self.position {
                    self.next_checkpoint = self.next_checkpoint.saturating_add(self.checkpoint_interval);
                }
                self.stage_checkpoint();
                return drive_publication(&mut self.publication, context);
            }
            if context.should_yield() {
                return Ok(None);
            }
        }
    }
}

impl semio_framework_job::InteractiveJob for DeflateEncodeJob {
    fn step<'a>(&'a mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(context).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        borrow_publication_outcome(&self.publication, descriptor)
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            use semio_framework_job::InteractiveJobCloseStep;
            let demand = match self.close_demands() {
                Ok(demand) => demand,
                Err(error) => return Err(error),
            };
            if semio_framework_job::InteractiveJob::terminal_is_empty(self) {
                return Ok(deflate_close_complete());
            }
            if !deflate_close_admits_demand(grant, demand) {
                return Ok(deflate_close_idle());
            }
            if let Some(publication) = self.publication.as_mut() {
                let step = publication.close_step(grant)?;
                if publication.terminal_is_empty() {
                    self.publication = None;
                }
                return Ok(match step {
                    InteractiveJobCloseStep::Complete { progress } => InteractiveJobCloseStep::Pending { progress },
                    step => step,
                });
            }
            if let Some(released_bytes) = release_next_backing(&mut [&mut self.input, &mut self.writer.out, &mut self.head, &mut self.previous]) {
                return Ok(deflate_close_receipt(1, released_bytes));
            }
            self.pending = None;
            Ok(deflate_close_receipt(1, 0))
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
        self.publication.is_none() && self.input.capacity() == 0 && self.writer.out.capacity() == 0 && self.head.capacity() == 0 && self.previous.capacity() == 0 && self.pending.is_none()
    }
}

impl DeflateEncodeJob {
    /// ♻️ The physical bytes the next close turn releases, `0` when the quote is unavailable.
    pub(super) fn close_release_bytes(&self) -> usize {
        self.close_demands().map_or(0, |demand| demand.release_bytes)
    }

    /// 📏️ Quotes the next close turn: the staged publication, then the first backing still held.
    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let mut demand = publication_demands(&self.publication, next_backing_bytes(&[&self.input, &self.writer.out, &self.head, &self.previous]))?;
        if self.publication.is_none() && demand.release_bytes == 0 && !semio_framework_job::InteractiveJob::terminal_is_empty(self) {
            demand.depth = 1;
        }
        Ok(demand)
    }
}
//#endregion 🧵️StreamingEncode
//#endregion DeflateCodec
//#endregion 🦑️DissolvedEngineCodec
