//! 🎛️ Drives registered jobs through the real reactor registry one funded turn at a time, for laws that interleave other work between the turns of a job.

use crate::{cancel_job, job_admission_demands, start_job, step_job, BoundedJob, IoRunControl, JobStep, OriginalJobAdmission, SqliteSnapshotControl, WORK_UNITS_EXECUTE};
use semio_framework_async::poll::resolve_ready;
use semio_framework_job::{Generation, OperationId, OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::ValueError;

const TURNS: usize = 100_000;

fn widest(left: RetainedCloneGrant, right: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: left.maximum_items.max(right.maximum_items), maximum_copy_bytes: left.maximum_copy_bytes.max(right.maximum_copy_bytes), maximum_capacity_bytes: left.maximum_capacity_bytes.max(right.maximum_capacity_bytes), maximum_release_bytes: left.maximum_release_bytes.max(right.maximum_release_bytes), maximum_depth: left.maximum_depth.max(right.maximum_depth) }
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
}

impl RegistryJobDriver {
    pub fn new(job: u64, policy: RetainedCloneGrant) -> Self {
        Self { job, grant: policy }
    }

    pub fn generous(job: u64) -> Self {
        Self::new(job, GENEROUS_GRANT)
    }

    /// 🚀️ Admits `kind` with `input` as the driven job, widening the funding until the registry accepts it.
    pub fn start(&mut self, kind: &str, input: Vec<u8>, checkpoint: Option<Vec<u8>>) -> Result<(), OriginalTurnFailure> {
        let job = self.job;
        let mut source = Some(OriginalJobAdmission { job, kind: kind.into(), input: Some(input), checkpoint });
        for _ in 0..TURNS {
            let grant = self.grant;
            let (admitted, quote) = funded(job, grant, 1, |cx| (resolve_ready(start_job(&mut source, cx)), job_admission_demands(job, cx).ok()));
            if admitted? {
                return Ok(());
            }
            if let Some(quote) = quote {
                self.grant = widest(grant, quote);
            }
        }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "registered job was never admitted by the reactor registry").into())
    }

    /// ▶️ Runs one funded turn of the job.
    pub fn step(&mut self) -> Result<JobStep, OriginalTurnFailure> {
        let job = self.job;
        if let Some(quote) = funded(job, self.grant, 1, |cx| job_admission_demands(job, cx).ok()) {
            self.grant = widest(self.grant, quote);
        }
        let grant = self.grant;
        funded(job, grant, WORK_UNITS_EXECUTE, |cx| controlled(grant, |original, snapshot| resolve_ready(step_job(job, original, snapshot, cx))))
    }

    /// 🛑️ Records the cancellation of the job on its registry owner.
    pub fn cancel(&mut self) -> Result<bool, ValueError> {
        let job = self.job;
        funded(job, self.grant, 1, |cx| resolve_ready(cancel_job(job, cx)))
    }
}

/// ▶️ Runs one turn of `job` funded with `fuel` logical work units and the generous grant, for jobs that are driven without the registry.
#[doc(hidden)]
pub fn step_bounded(job: &mut dyn BoundedJob, fuel: u64) -> Result<JobStep, OriginalTurnFailure> {
    funded(1, GENEROUS_GRANT, fuel, |cx| controlled(GENEROUS_GRANT, |original, snapshot| job.step(original, snapshot, cx)))
}
