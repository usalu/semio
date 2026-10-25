//! 💼️ Job crossings: every original guest job call is funded by one issued retained turn and settles its actual spans back into it. See [`super`].

use super::{GuestInstance, GuestRuntime, GuestRuntimes};
use semio_framework_actor::{OriginalActorTurnLoan, OriginalActorTurnWork, OriginalGuestExecutionBudget, OriginalJobBudget, OriginalJobCallFailure, OriginalJobOutputState, OriginalJobSource, RetainedTurnInput};
use semio_framework_plugin_guest_contract::{identity_turn_fault, JobStep, TurnFault};
use semio_framework_plugin_interpreter_runtime::OriginalGuestJobCall;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError, ValueRefusalKind};

/// 🎟️ One funded crossing: the logical job budget and the independent backend instruction authorization of the same original input.
#[derive(Clone, Copy, Debug)]
pub struct JobCrossing {
    pub budget: OriginalJobBudget,
    pub execution: OriginalGuestExecutionBudget,
}

/// 🧾️ The issued retained turn that funds every crossing of one job operation; each actual receipt shrinks what the next crossing may spend.
#[derive(Debug)]
pub struct JobFunds {
    input: RetainedTurnInput,
    loan: OriginalActorTurnLoan,
    spent: RetainedCloneProgress,
    work: u64,
    instruction_fuel: u64,
}

impl JobFunds {
    /// 🪙️ A self-issued job turn carries this retained span grant — large enough for one relayed job output.
    pub const SELF_ISSUED_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 64 << 20, maximum_capacity_bytes: 64 << 20, maximum_release_bytes: 64 << 20, maximum_depth: 64 };

    /// 🎫️ Funds crossings from the Kernel-issued `input`, bounded by its semantic `loan`.
    pub fn issued(input: RetainedTurnInput, loan: OriginalActorTurnLoan, instruction_fuel: u64) -> Result<Self, ValueError> {
        loan.validate_for(input)?;
        Ok(Self { input, loan, spent: Default::default(), work: 0, instruction_fuel })
    }

    /// 🏭️ Issues a standalone job turn for a host that has no Kernel grant, on `clock` for `wall_us`.
    pub fn self_issued(operation: u64, clock: fn() -> Option<u64>, wall_us: u64, work_fuel: u64, instruction_fuel: u64) -> Result<Self, ValueError> {
        let input = self_issued_input(operation, clock, wall_us, work_fuel)?;
        Self::issued(input, OriginalActorTurnLoan { retained_policy: input.grant, work_fuel, deadline_us: input.work.absolute_deadline_us }, instruction_fuel)
    }

    fn crossing(&self) -> Result<JobCrossing, ValueError> {
        if self.loan.work_fuel == 0 {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "original job turn has no logical work left for another crossing"));
        }
        let deadline_us = self.loan.deadline_us;
        Ok(JobCrossing { budget: OriginalJobBudget { input: self.input, retained_policy: self.loan.retained_policy, fuel: self.loan.work_fuel, deadline_us, cancelled: false }, execution: OriginalGuestExecutionBudget { input: self.input, fuel: self.instruction_fuel, deadline_us } })
    }

    fn settle(&mut self, progress: RetainedCloneProgress, work: u64) -> Result<(), ValueError> {
        let spent = self.spent.checked_add(progress)?;
        let total = self.work.checked_add(work).ok_or_else(|| refused("original job work counter overflows"))?;
        self.input.return_original(spent, total)?;
        self.loan = self.loan.remaining(progress, work)?;
        self.spent = spent;
        self.work = total;
        Ok(())
    }

    fn settle_call<T>(&mut self, crossing: JobCrossing, call: OriginalGuestJobCall<T>, receipt: impl FnOnce(&T) -> (RetainedCloneProgress, u64, Option<OriginalJobCallFailure>)) -> Result<T, TurnFault> {
        call.execution.validate_for(crossing.execution).map_err(identity_turn_fault)?;
        let reply = call.original?;
        let (progress, work, refusal) = receipt(&reply);
        self.settle(progress, work).map_err(identity_turn_fault)?;
        match refusal {
            None => Ok(reply),
            Some(OriginalJobCallFailure::Authority(kind)) => Err(identity_turn_fault(ValueError::literal(kind, "original job call refused by its guest authority"))),
            Some(OriginalJobCallFailure::Semantic) => Err(TurnFault::Trapped("original job call refused by its guest semantics".to_string())),
        }
    }
}

fn self_issued_input(operation: u64, clock: fn() -> Option<u64>, wall_us: u64, work_fuel: u64) -> Result<RetainedTurnInput, ValueError> {
    let now = clock().ok_or_else(|| refused("self-issued turn requires its caller clock"))?;
    let deadline_us = now.checked_add(wall_us).ok_or_else(|| refused("self-issued deadline overflows its clock"))?;
    Ok(RetainedTurnInput { operation: operation.max(1), generation: 1, epoch: 1, work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: deadline_us, fuel: work_fuel }, grant: JobFunds::SELF_ISSUED_GRANT })
}

/// 🎫️ Issues the standalone turn budget of a headless host that runs a guest turn without any Kernel grant.
pub fn self_issued_budget(operation: u64, clock: fn() -> Option<u64>, wall_us: u64, work_fuel: u64, fuel: u64, max_effects: u32, max_patch_bytes: u32, max_frames: u32) -> Result<semio_framework::kernel::Budget, ValueError> {
    let retained = self_issued_input(operation, clock, wall_us, work_fuel)?;
    Ok(semio_framework::kernel::Budget { retained, retained_policy: retained.grant, work_fuel, fuel, deadline_us: retained.work.absolute_deadline_us, max_effects, max_patch_bytes, max_frames })
}

fn refused(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, message)
}

impl GuestRuntimes {
    /// 📥️ Admits `kind` and `input` as guest job `job`.
    pub async fn start_job_funded(&self, inst: &mut GuestInstance, funds: &mut JobFunds, job: u64, kind: String, input: Vec<u8>) -> Result<(), TurnFault> {
        let crossing = funds.crossing().map_err(identity_turn_fault)?;
        let source = OriginalJobSource { job, kind, input: Some(input), checkpoint: None };
        let call = self.start_job(inst, Some(source), crossing.budget, crossing.execution).await;
        let reply = funds.settle_call(crossing, call, |reply| (reply.spent, reply.work_units_spent, reply.refusal))?;
        if reply.admitted { Ok(()) } else { Err(TurnFault::Trapped(format!("guest job {job} was not admitted"))) }
    }

    /// ▶️ Steps guest job `job` once and collects the output it offers, page by page, into one `JobStep`.
    pub async fn step_job_funded(&self, inst: &mut GuestInstance, funds: &mut JobFunds, job: u64) -> Result<JobStep, TurnFault> {
        let crossing = funds.crossing().map_err(identity_turn_fault)?;
        let call = self.step_job(inst, job, crossing.budget, crossing.execution).await;
        let reply = funds.settle_call(crossing, call, |reply| (reply.spent, reply.work_units_spent, reply.refusal))?;
        let Some(outcome) = reply.outcome else {
            return Ok(JobStep::Running { progress: None });
        };
        let bytes = self.collect_job_output(inst, funds, job, outcome.byte_length).await?;
        Ok(match outcome.state {
            OriginalJobOutputState::Running => JobStep::Running { progress: (!bytes.is_empty()).then_some(bytes) },
            OriginalJobOutputState::Done => JobStep::Done { output: bytes },
            OriginalJobOutputState::Failed => JobStep::Failed { error: bytes },
        })
    }

    /// 🛑️ Admits the cancellation of guest job `job`.
    pub async fn cancel_job_funded(&self, inst: &mut GuestInstance, funds: &mut JobFunds, job: u64) -> Result<(), TurnFault> {
        let crossing = funds.crossing().map_err(identity_turn_fault)?;
        let call = self.cancel_job(inst, job, crossing.budget, crossing.execution).await;
        let reply = funds.settle_call(crossing, call, |reply| (reply.spent, reply.work_units_spent, reply.refusal))?;
        if reply.admitted { Ok(()) } else { Err(TurnFault::Trapped(format!("guest job {job} cancellation was not admitted"))) }
    }

    async fn collect_job_output(&self, inst: &mut GuestInstance, funds: &mut JobFunds, job: u64, expected: u64) -> Result<Vec<u8>, TurnFault> {
        let mut bytes = Vec::new();
        loop {
            let crossing = funds.crossing().map_err(identity_turn_fault)?;
            let call = self.read_job_output(inst, job, crossing.budget, crossing.execution).await;
            let reply = funds.settle_call(crossing, call, |reply| (reply.spent, reply.work_units_spent, reply.refusal))?;
            let Some(chunk) = reply.chunk else { break };
            let offered = chunk.bytes.is_some();
            if let Some(page) = chunk.bytes {
                if chunk.offset != bytes.len() as u64 {
                    return Err(TurnFault::Trapped(format!("guest job {job} offered an output page at offset {} after {} bytes", chunk.offset, bytes.len())));
                }
                bytes.extend_from_slice(&page);
            }
            if chunk.output_closed {
                break;
            }
            if !offered {
                return Err(TurnFault::Trapped(format!("guest job {job} output stalled without a page or a close")));
            }
        }
        if bytes.len() as u64 != expected {
            return Err(TurnFault::Trapped(format!("guest job {job} output holds {} bytes but its outcome announced {expected}", bytes.len())));
        }
        let crossing = funds.crossing().map_err(identity_turn_fault)?;
        let call = self.acknowledge_job_output(inst, job, crossing.budget, crossing.execution).await;
        let ack = funds.settle_call(crossing, call, |reply| (reply.spent, reply.work_units_spent, reply.refusal))?;
        if ack.acknowledged { Ok(bytes) } else { Err(TurnFault::Trapped(format!("guest job {job} output acknowledgement was refused"))) }
    }
}
