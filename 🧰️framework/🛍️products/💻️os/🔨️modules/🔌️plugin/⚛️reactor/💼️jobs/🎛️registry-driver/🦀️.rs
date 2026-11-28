//! 🎛️ Drives registered jobs through the real reactor registry one funded turn at a time, for laws that interleave other work between the turns of a job.

use crate::{abi, close_job, job_admission_demands, BoundedJob, IoRunControl, JobBody, JobStep, JOBS, OriginalJobAdmission, SqliteSnapshotControl, WORK_UNITS_EXECUTE};
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError};

const TURNS: usize = 100_000;

pub(crate) fn widest(left: RetainedCloneGrant, right: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: left.maximum_items.max(right.maximum_items), maximum_copy_bytes: left.maximum_copy_bytes.max(right.maximum_copy_bytes), maximum_capacity_bytes: left.maximum_capacity_bytes.max(right.maximum_capacity_bytes), maximum_release_bytes: left.maximum_release_bytes.max(right.maximum_release_bytes), maximum_depth: left.maximum_depth.max(right.maximum_depth) }
}

pub(crate) fn closing_grant(job: u64, policy: RetainedCloneGrant, force_close: bool, cx: &StepContext<'_>) -> Result<RetainedCloneGrant, ValueError> {
    JOBS.with(|jobs| {
        let jobs = jobs.borrow();
        let Some(slot) = jobs.iter().flatten().find(|slot| slot.job == job) else { return Ok(policy) };
        slot.validate_original(cx)?;
        let cancelling = force_close || (slot.cancelled && !slot.cancellation_fault_started);
        if cancelling && !slot.cancellation_signalled {
            return Ok(policy);
        }
        slot_grant(slot, policy, cancelling || slot.outcome.is_some())
    })
}

/// 🎟️ `base` widened to exactly one quoted retirement demand.
pub(crate) fn fund_demand(base: RetainedCloneGrant, demand: RetirementDemand) -> RetainedCloneGrant {
    widest(base, RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) })
}

/// 🎟️ `base` widened to the next close demand of one retained source: its cursor birth before the close began, its selected step after.
pub(crate) fn source_grant<T: semio_framework_value::retirement::RetireOwned>(base: RetainedCloneGrant, source: &Option<T>, closing: &Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>) -> RetainedCloneGrant {
    let demand = match closing {
        Some(owner) => owner.selected_demand(base.maximum_copy_bytes),
        None => semio_framework_value::retirement::retirement_cursor_birth_demand(source.as_ref().expect("a close without a cursor retains its source")),
    };
    fund_demand(base, demand.expect("retained source close demand"))
}

/// 🎟️ The one-turn grant that pays the next quoted demand of every owner a registry slot holds, so a driver never funds below what its owners publish.
pub(crate) fn slot_grant(slot: &crate::JobSlot, policy: RetainedCloneGrant, closing: bool) -> Result<RetainedCloneGrant, ValueError> {
    let copy = policy.maximum_copy_bytes;
    let mut grant = policy;
    let mut fund = |demand: RetirementDemand| {
        grant = widest(grant, RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: demand.copy_bytes,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: demand.depth,
        });
    };
    if let Some(source) = &slot.source_close {
        fund(source.selected_demand(copy)?);
    }
    if let Some(owner) = slot.failed_owner.as_ref().filter(|_| slot.cancellation_signalled) {
        fund(owner.retirement_demands(copy)?);
        fund(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() });
    }
    if closing {
        if let Some(output) = &slot.rejected_output_close {
            fund(output.selected_demand(copy)?);
        }
        if let Some(output) = &slot.outcome_close {
            fund(output.selected_demand(copy)?);
        }
        if let Some(JobBody::Refused(_, Some(cause))) = &slot.body {
            fund(cause.selected_demand(copy)?);
        }
    }
    if let Some(JobBody::Bounded(owner)) = &slot.body {
        fund(owner.retirement_demands(copy)?);
        fund(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() });
    }
    Ok(grant)
}

fn funded<T>(job: u64, grant: RetainedCloneGrant, fuel: u64, run: impl FnOnce(&mut StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let mut receipt = RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(job), Generation(1), StepBudget::new(fuel, u64::MAX, grant), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
    run(&mut cx)
}

fn controlled<T>(grant: RetainedCloneGrant, run: impl FnOnce(&mut IoRunControl<'_, '_>, &mut SqliteSnapshotControl<'_>) -> T) -> T {
    let mut decode_callback = |_| true;
    let mut encode_callback = |_| true;
    let mut decode = semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(grant.maximum_capacity_bytes, &mut decode_callback);
    let mut encode = semio_framework_value::NativeEncodeControl::new(grant.maximum_capacity_bytes, &mut encode_callback);
    let mut original = IoRunControl::new(&mut decode, &mut encode, grant);
    let mut snapshot_callback = |_| true;
    let mut snapshot = SqliteSnapshotControl::new(&mut snapshot_callback, Default::default());
    run(&mut original, &mut snapshot)
}

/// 🪙️ The retained span grant of a driver that has no tighter policy to prove.
#[doc(hidden)]
pub const GENEROUS_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 16, maximum_copy_bytes: 16 << 20, maximum_capacity_bytes: 16 << 20, maximum_release_bytes: 16 << 20, maximum_depth: 64 };

/// 🧪️ One registered job under a caller that funds every turn with at least the registry's own quoted grant.
#[doc(hidden)]
pub struct RegistryJobDriver {
    job: u64,
    grant: RetainedCloneGrant,
    caller: Option<OriginalJobFixtureCaller>,
    admitted: bool,
}

impl RegistryJobDriver {
    pub fn new(job: u64, policy: RetainedCloneGrant) -> Self {
        let stock = widest(policy, RetainedCloneGrant {
            maximum_items: 1 << 24,
            maximum_copy_bytes: 128 << 20,
            maximum_capacity_bytes: 256 << 20,
            maximum_release_bytes: 512 << 20,
            maximum_depth: 4096,
        });
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(job), Generation(1), stock, semio_framework_job::default_now_us, u64::MAX, 1 << 36);
        caller.birth_returns(stock);
        Self { job, grant: policy, caller: Some(caller), admitted: false }
    }

    pub fn generous(job: u64) -> Self {
        Self::new(job, GENEROUS_GRANT)
    }

    /// 🎟️ One funded original turn of this driver's caller, for laws that call the registry entry points themselves.
    pub fn turn<R>(&mut self, grant: RetainedCloneGrant, fuel: u64, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        self.owned_turn(grant, fuel, run)
    }

    fn owned_turn<R>(&mut self, grant: RetainedCloneGrant, fuel: u64, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        let caller = self.caller.as_mut().expect("registry fixture owns its original caller");
        caller.refreshed_turn(grant, |cx| {
            let mut value = None;
            cx.with_original_work_loan(fuel, u64::MAX, |cx| {
                value = Some(run(cx)?);
                Ok(())
            })?;
            Ok(value.expect("funded registry fixture turn returned its result"))
        })
    }

    /// 🚀️ Admits the registry's actual request under the caller that will also step and close it.
    pub fn start(&mut self, kind: &str, input: Vec<u8>, checkpoint: Option<Vec<u8>>) -> Result<(), OriginalTurnFailure> {
        let job = self.job;
        let mut source = Some(OriginalJobAdmission { job, kind: kind.into(), input: Some(input), checkpoint });
        for _ in 0..TURNS {
            let admitted = self.owned_turn(self.grant, 1, |cx| Ok(abi::start(&mut source, cx)?))?;
            if admitted {
                self.admitted = true;
                return Ok(());
            }
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "registered job was never admitted by the reactor registry").into())
    }

    /// 🧾️ Everything this job's caller has been charged so far.
    pub fn receipt(&self) -> RetainedCloneProgress {
        self.caller.as_ref().expect("registry fixture owns its original caller").cumulative_receipt()
    }

    /// 🚀️ One admission turn of the registry's actual request under `grant`; a refused turn leaves the whole source with the caller.
    pub fn start_source(&mut self, source: &mut Option<OriginalJobAdmission>, grant: RetainedCloneGrant) -> Result<bool, OriginalTurnFailure> {
        let admitted = self.owned_turn(grant, 1, |cx| Ok(abi::start(source, cx)?))?;
        self.admitted |= admitted;
        Ok(admitted)
    }

    /// ▶️ Runs one funded registry turn through the same producer and caller as the guest ABI.
    pub fn step(&mut self) -> Result<JobStep, OriginalTurnFailure> {
        self.step_with_fuel(WORK_UNITS_EXECUTE)
    }

    /// ▶️ [`Self::step`] with the registry turn funded by `fuel` logical work units.
    pub fn step_with_fuel(&mut self, fuel: u64) -> Result<JobStep, OriginalTurnFailure> {
        self.step_via(fuel, abi::step)
    }

    /// ▶️ [`Self::step_with_fuel`] through the given guest crossing, such as [`abi::step_coarse`].
    pub fn step_via(&mut self, fuel: u64, crossing: fn(u64, &mut StepContext<'_>) -> Result<Option<JobStep>, OriginalTurnFailure>) -> Result<JobStep, OriginalTurnFailure> {
        let job = self.job;
        let policy = self.grant;
        let (admission, closure) = self.owned_turn(policy, 1, |cx| {
            Ok((job_admission_demands(job, cx).ok(), closing_grant(job, policy, false, cx)?))
        })?;
        if let Some(admission) = admission {
            self.grant = widest(policy, admission);
        }
        let grant = widest(self.grant, closure);
        let step = self.owned_turn(grant, fuel, |cx| crossing(job, cx))?.unwrap_or(JobStep::Running(None));
        if !matches!(step, JobStep::Running(_)) {
            self.admitted = false;
        }
        Ok(step)
    }

    /// 🛑️ Records cancellation on the same admitted original registry owner.
    pub fn cancel(&mut self) -> Result<bool, OriginalTurnFailure> {
        let job = self.job;
        if !self.admitted {
            return Ok(false);
        }
        self.owned_turn(self.grant, 1, |cx| Ok(abi::cancel(job, cx)?))
    }
}

impl Drop for RegistryJobDriver {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        let job = self.job;
        for _ in 0..TURNS {
            if !self.admitted {
                let caller = self.caller.take().expect("original registry caller");
                let retirement = caller.remaining_grant();
                caller.retire(retirement);
                return;
            }
            let policy = self.grant;
            let grant = self.owned_turn(policy, 1, |cx| Ok(closing_grant(job, policy, true, cx)?)).expect("original registry job close demand");
            self.admitted = !self.owned_turn(grant, WORK_UNITS_EXECUTE, |cx| cx.with_original_retirement(|cx| close_job(job, cx))).expect("paid original registry job close");
        }
        panic!("original registry job retained work after its bounded close");
    }
}

/// ▶️ Runs one turn of `job` funded with `fuel` logical work units and the generous grant, for jobs that are driven without the registry.
#[doc(hidden)]
pub fn step_bounded(job: &mut dyn BoundedJob, fuel: u64) -> Result<JobStep, OriginalTurnFailure> {
    funded(1, GENEROUS_GRANT, fuel, |cx| controlled(GENEROUS_GRANT, |original, snapshot| job.step(original, snapshot, cx)))
}
