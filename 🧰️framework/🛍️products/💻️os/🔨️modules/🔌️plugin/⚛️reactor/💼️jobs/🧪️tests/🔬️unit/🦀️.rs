use super::*;

//#region 🔖️PreRewriteParity

#[semio_framework_async_macros::async_test]
async fn step_job_on_an_unknown_id_fails_without_panicking() {
    match step_job(999, JobBudget::default()).await {
        JobStep::Failed(_) => {}
        _ => panic!("an unregistered job id must fail, not succeed"),
    }
}

#[semio_framework_async_macros::async_test]
async fn cancel_job_removes_a_pending_record_so_a_later_step_fails() {
    start_job(1, JOB_KIND_IO_RUN, b"{}").await;
    cancel_job(1).await;
    match step_job(1, JobBudget::default()).await {
        JobStep::Failed(_) => {}
        _ => panic!("a cancelled job must not still be steppable"),
    }
}

#[semio_framework_async_macros::async_test]
async fn step_job_on_an_unknown_kind_fails_with_a_named_fault() {
    start_job(2, "semio.not-a-real-kind", b"{}").await;
    match step_job(2, JobBudget::default()).await {
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            assert_eq!(fault.code.0, "job.unknown-kind");
        }
        _ => panic!("an unknown job kind must fail"),
    }
}

/// 🗺️ `semio.io-run`/`semio.io-sniff` dispatch through the registry (not an "unknown kind"
/// fault) and preserve their pre-rewrite decode-failure fault codes exactly — the old file had
/// no fixture for a REAL io-run hop either (that coverage lives in `🖥️host`'s mock-backed
/// integration tests), so this is the same scope the pre-rewrite suite actually had.
#[semio_framework_async_macros::async_test]
async fn io_run_dispatches_through_the_registry_and_keeps_its_decode_fault_code() {
    start_job(3, JOB_KIND_IO_RUN, b"not json").await;
    match step_job(3, JobBudget::default()).await {
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            assert_eq!(fault.code.0, "job.io-run.decode", "must reach run_io_run, not job.unknown-kind");
        }
        _ => panic!("garbage io-run input must fail, got a non-Failed step"),
    }
}

#[semio_framework_async_macros::async_test]
async fn io_sniff_dispatches_through_the_registry_and_keeps_its_decode_fault_code() {
    start_job(4, JOB_KIND_IO_SNIFF, b"not json").await;
    match step_job(4, JobBudget::default()).await {
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            assert_eq!(fault.code.0, "job.io-sniff.decode", "must reach run_io_sniff, not job.unknown-kind");
        }
        _ => panic!("garbage io-sniff input must fail"),
    }
}

//#endregion

//#region 🔖️SlicingFixtures

/// 🧬️ Three ticks, three slices: waits for a grant, does one unit of work (`count += 1`,
/// progress/checkpoint bytes = `count`), then loops until `count` reaches 3. Resumable from
/// `restored` (a one-byte `count`), which is what the checkpoint/restore test below exercises.
fn resumable_counter_job(ctx: JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move {
        let mut count: u8 = restored.and_then(|bytes| bytes.first().copied()).unwrap_or(0);
        while count < 3 {
            ctx.tick().await;
            count += 1;
            ctx.checkpoint(vec![count]).await;
            ctx.progress(vec![count]).await;
        }
        Ok(vec![count, input.first().copied().unwrap_or(0)])
    })
}

#[semio_framework_async_macros::async_test]
async fn a_three_slice_job_returns_running_running_done_with_progress_each_slice() {
    register_job_kind("test.resumable-counter", resumable_counter_job);
    start_job(10, "test.resumable-counter", &[7]).await;

    match step_job(10, JobBudget { fuel: 1, deadline_ms: 1 }).await {
        JobStep::Running(Some(bytes)) => assert_eq!(bytes, vec![1]),
        _ => panic!("slice 1 must be Running(Some([1])), got a Done/Failed/None step"),
    }
    match step_job(10, JobBudget { fuel: 1, deadline_ms: 1 }).await {
        JobStep::Running(Some(bytes)) => assert_eq!(bytes, vec![2]),
        _ => panic!("slice 2 must be Running(Some([2]))"),
    }
    match step_job(10, JobBudget { fuel: 1, deadline_ms: 1 }).await {
        JobStep::Done(bytes) => assert_eq!(bytes, vec![3, 7]),
        _ => panic!("slice 3 must be Done([3, 7])"),
    }
}

#[semio_framework_async_macros::async_test]
async fn the_budget_a_tick_observes_is_whatever_step_job_most_recently_passed() {
    fn budget_echo_job(ctx: JobCtx, _input: Vec<u8>, _restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
        Box::pin(async move {
            ctx.tick().await;
            ctx.progress(vec![ctx.budget().await.fuel as u8]).await;
            ctx.tick().await;
            ctx.progress(vec![ctx.budget().await.fuel as u8]).await;
            ctx.tick().await;
            Ok(vec![0])
        })
    }
    register_job_kind("test.budget-echo", budget_echo_job);
    start_job(11, "test.budget-echo", b"[]").await;

    match step_job(11, JobBudget { fuel: 5, deadline_ms: 1 }).await {
        JobStep::Running(Some(bytes)) => assert_eq!(bytes, vec![5], "first tick must see the first budget"),
        _ => panic!("slice 1 must be Running"),
    }
    match step_job(11, JobBudget { fuel: 9, deadline_ms: 1 }).await {
        JobStep::Running(Some(bytes)) => assert_eq!(bytes, vec![9], "second tick must see the NEW budget, not the stale one"),
        _ => panic!("slice 2 must be Running"),
    }
}

#[semio_framework_async_macros::async_test]
async fn cancelling_a_job_mid_slice_frees_its_slot_for_the_id() {
    register_job_kind("test.resumable-counter", resumable_counter_job);
    start_job(12, "test.resumable-counter", b"[]").await;
    match step_job(12, JobBudget { fuel: 1, deadline_ms: 1 }).await {
        JobStep::Running(_) => {}
        _ => panic!("job must be mid-slice (parked) before cancelling"),
    }
    cancel_job(12).await;
    match step_job(12, JobBudget { fuel: 1, deadline_ms: 1 }).await {
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            assert_eq!(fault.code.0, "job.unknown", "the id must be free, not still bound to the cancelled task");
        }
        _ => panic!("a job cancelled mid-slice must not still be steppable"),
    }
}

#[semio_framework_async_macros::async_test]
async fn checkpoint_restore_resumes_and_matches_an_uninterrupted_run() {
    register_job_kind("test.resumable-counter", resumable_counter_job);

    // Uninterrupted baseline.
    start_job(20, "test.resumable-counter", &[42]).await;
    step_job(20, JobBudget::default()).await;
    step_job(20, JobBudget::default()).await;
    let baseline = match step_job(20, JobBudget::default()).await {
        JobStep::Done(bytes) => bytes,
        _ => panic!("uninterrupted run must finish Done within 3 slices"),
    };

    // Interrupted: one real slice, capture the checkpoint pack, simulate the actor tearing
    // down (cancel_job — the same bookkeeping a trap-then-restart would leave behind), then
    // replay through restore_job exactly like the leased checkpoint::restore would.
    start_job(21, "test.resumable-counter", &[42]).await;
    step_job(21, JobBudget::default()).await;
    let entries = checkpoint_jobs().await;
    let entry = entries.iter().find(|entry| entry.job == 21).expect("job 21 must appear in checkpoint_jobs()");
    assert_eq!(entry.kind, "test.resumable-counter");
    assert_eq!(entry.input, vec![42]);
    let checkpoint = entry.checkpoint.clone();
    cancel_job(21).await;

    restore_job(21, "test.resumable-counter", &[42], checkpoint).await;
    step_job(21, JobBudget::default()).await;
    let restored_final = match step_job(21, JobBudget::default()).await {
        JobStep::Done(bytes) => bytes,
        _ => panic!("restored run must finish Done within 2 more slices"),
    };
    assert_eq!(restored_final, baseline, "checkpoint/restore must produce the identical final output");
}

#[semio_framework_async_macros::async_test]
async fn the_stall_guard_fires_after_repeated_no_progress_static_budget_slices() {
    fn never_progresses_job(ctx: JobCtx, _input: Vec<u8>, _restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
        Box::pin(async move {
            loop {
                ctx.tick().await;
            }
        })
    }
    register_job_kind("test.never-progresses", never_progresses_job);
    start_job(30, "test.never-progresses", b"[]").await;

    let same_budget = JobBudget { fuel: 100, deadline_ms: 50 };
    for call in 0..STALL_LIMIT {
        match step_job(30, same_budget).await {
            JobStep::Running(None) => {}
            _ => panic!("call {call} must still be Running(None) before the stall limit"),
        }
    }
    match step_job(30, same_budget).await {
        JobStep::Failed(bytes) => {
            let fault = dsl::decode_fault_bytes(&bytes);
            assert_eq!(fault.code.0, "job.stalled");
        }
        _ => panic!("the stall guard must fire once STALL_LIMIT consecutive no-progress static-budget calls have elapsed"),
    }
}

struct FixedBoundedFixture {
    step: u8,
    cancelled: bool,
}

impl BoundedJob for FixedBoundedFixture {
    fn step(&mut self, _budget: JobBudget) -> JobStep {
        self.step += 1;
        if self.cancelled {
            JobStep::Failed(b"cancelled".to_vec())
        } else if self.step == 1 {
            JobStep::Running(Some(vec![1]))
        } else {
            JobStep::Done(vec![2])
        }
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        Some(vec![self.step])
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        true
    }
}

fn fixed_bounded_fixture(_job: u64, input: &[u8]) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    if input != b"fixed" {
        return Err(b"fixed-admission".to_vec());
    }
    Ok(Box::new(FixedBoundedFixture { step: 0, cancelled: false }))
}

#[semio_framework_async_macros::async_test]
async fn production_bounded_job_path_advances_one_explicit_state_action_and_retains_admission_fault() {
    register_bounded_job_kind("test.fixed-bounded", fixed_bounded_fixture);
    start_job(9_001, "test.fixed-bounded", b"fixed").await;
    assert!(matches!(step_job(9_001, JobBudget { fuel: 1, deadline_ms: 1 }).await, JobStep::Running(Some(bytes)) if bytes == [1]));
    assert!(matches!(step_job(9_001, JobBudget { fuel: 1, deadline_ms: 1 }).await, JobStep::Done(bytes) if bytes == [2]));

    start_job(9_002, "test.fixed-bounded", b"rejected").await;
    assert!(matches!(step_job(9_002, JobBudget { fuel: 1, deadline_ms: 1 }).await, JobStep::Failed(bytes) if bytes == b"fixed-admission"));
}

//#endregion
