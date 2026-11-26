//! 🐾️ Executes a dictionary-backed schema operator through its paid retained job.

use super::{ControlledRetirement, Dictionary, EvalError, OperatorPlanAdmission, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, ValueError, ValueRefusalKind};
use semio_framework_job::{OriginalTurnFailure, OriginalTurnPrimary, StepContext};
use semio_framework_value::{RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand};
use semio_os_kernel_neural_engine_model::{OperatorJob, OperatorJobStep, OperatorProgress, OperatorWorkFailure};

#[path = "fields.rs"]
mod fields;
pub(super) use fields::compile;

pub(super) const PLAN_BYTES: usize = size_of::<SchemaJob>();
pub(super) const PLAN_COPY_BYTES: usize = size_of::<Dictionary>();

/// 🧮️ Transfers the original input after admitting the job shell and one program root alias.
pub(super) fn admit(program: &Dictionary, input: Dictionary, grant: RetainedCloneGrant) -> Result<(OperatorPlanAdmission, RetainedCloneProgress), (EvalError, Dictionary)> {
    let progress = RetainedCloneProgress {
        copied_items: 1,
        copied_bytes: PLAN_COPY_BYTES,
        retained_capacity_bytes: PLAN_BYTES,
        ..Default::default()
    };
    if grant.maximum_depth == 0 || !progress.fits(grant) {
        return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit, "schema program plan is not funded").into(), input));
    }
    let frame = match Box::<SchemaJob>::try_new_uninit() {
        Ok(frame) => frame,
        Err(_) => return Err((ValueError::literal(ValueRefusalKind::AllocationFailed, "schema program plan allocation failed").into(), input)),
    };
    let job = Box::write(frame, SchemaJob {
        projection: Some(fields::Projection::new(program.clone(), input)),
        output: None,
        retiring: None,
        receipt: Default::default(),
        units: 0,
        state: JobState::Running,
    });
    Ok((OperatorPlanAdmission::Job(job), progress))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum JobState { Running, Done, Cancelled }

struct SchemaJob {
    projection: Option<fields::Projection>,
    output: Option<Dictionary>,
    retiring: Option<ControlledRetirement<(Option<fields::Projection>, Option<Dictionary>)>>,
    receipt: RetainedCloneProgress,
    units: usize,
    state: JobState,
}

impl SchemaJob {
    fn next_demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        self.projection.as_ref().map_or(Ok(Default::default()), |projection| projection.next_demands(copy))
    }

    fn close_demands(&self, copy: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = &self.retiring { return owner.selected_demand(copy); }
        Ok(RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    fn remember_failure(&mut self, primary: impl Into<OriginalTurnPrimary>, cx: &mut StepContext<'_>) -> OperatorWorkFailure {
        self.state = JobState::Cancelled;
        let mut failure = OriginalTurnFailure::from(primary.into());
        failure.effects = self.receipt;
        failure.receipt_refusal = cx.consume_retained(self.receipt).err();
        failure.into()
    }
}

impl OperatorJob for SchemaJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<(OperatorJobStep, RetainedCloneProgress), OperatorWorkFailure> {
        self.receipt = Default::default();
        if cx.is_cancelled() { self.cancel(); }
        if self.state == JobState::Cancelled { return Ok((OperatorJobStep::Cancelled(self.progress()), self.receipt)); }
        let grant = cx.retained_grant();
        if self.state != JobState::Running || cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok((OperatorJobStep::Working(self.progress()), self.receipt));
        }
        let projection = self.projection.as_mut().expect("running schema program owns its projection");
        let output = &mut self.output;
        let mut started = false;
        let turn = cx.with_original_effect(grant, |_, effect| {
            started = true;
            match projection.advance(grant.maximum_copy_bytes, effect) {
                RetainedWorkPoll::Pending { work } => RetainedWorkPoll::Pending { work },
                RetainedWorkPoll::Complete(turn) => RetainedWorkPoll::Complete(RetainedWorkOutcome {
                    result: turn.result.map(|produced| *output = produced),
                    work: turn.work,
                }),
            }
        });
        if started { self.receipt = projection.step_progress(); }
        let (poll, spent) = match turn {
            Ok(Some(turn)) => turn,
            Ok(None) => return Ok((OperatorJobStep::Working(self.progress()), self.receipt)),
            Err(error) => return Err(self.remember_failure(error, cx)),
        };
        cx.consume_work(spent);
        if let RetainedWorkPoll::Complete(turn) = poll {
            turn.result.map_err(|error| self.remember_failure(error, cx))?;
        }
        if let Err(error) = cx.consume_retained(self.receipt) {
            self.state = JobState::Cancelled;
            let mut failure = OriginalTurnFailure::from(error);
            failure.effects = self.receipt;
            return Err(failure.into());
        }
        self.units = self.units.saturating_add(self.receipt.copied_items.max(usize::from(spent != 0)));
        let step = if let Some(output) = self.output.take() {
            self.state = JobState::Done;
            OperatorJobStep::Done(output)
        } else {
            OperatorJobStep::Working(self.progress())
        };
        Ok((step, self.receipt))
    }

    fn progress(&self) -> OperatorProgress {
        let phase = match self.state {
            JobState::Running => self.projection.as_ref().expect("running schema program owns its projection").phase(),
            JobState::Done => "complete",
            JobState::Cancelled => "cancelled",
        };
        OperatorProgress {
            units_done: self.units,
            units_total: self.units.saturating_add(usize::from(self.state == JobState::Running)),
            phase,
        }
    }
    fn normal_step_progress(&self) -> RetainedCloneProgress { self.receipt }
    fn next_step_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.next_demands(0)?.copy_bytes) }
    fn next_step_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.next_demands(copy)?.capacity_bytes) }
    fn next_step_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.next_demands(0)?.release_bytes) }
    fn next_step_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.next_demands(0)?.depth) }
    fn cancel(&mut self) { if self.state == JobState::Running { self.state = JobState::Cancelled; } }
    fn terminal_is_empty(&self) -> bool { self.projection.is_none() && self.output.is_none() && self.retiring.is_none() }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.close_demands(copy)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        self.cancel();
        if let Some(owner) = &mut self.retiring {
            let result = owner.step(effect.grant().maximum_copy_bytes, effect);
            if owner.terminal_is_empty() { self.retiring = None; }
            return result;
        }
        let result = (|| {
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            let grant = effect.grant();
            if grant.maximum_items == 0 || grant.maximum_depth == 0 || !effect.performed(1)? {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            self.retiring = Some(ControlledRetirement::retain((self.projection.take(), self.output.take())));
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        })();
        super::RetirementPollExit::settle(effect, result)
    }
}
