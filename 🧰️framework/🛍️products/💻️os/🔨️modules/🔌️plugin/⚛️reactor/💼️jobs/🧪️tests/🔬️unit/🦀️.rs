use super::*;

/// ⛽️ A grant that covers any state action a builtin declares, so a law about the state WALK is
/// never accidentally a law about the budget gate.
const FULL_GRANT: JobBudget = JobBudget { fuel: WORK_UNITS_EXECUTE, deadline_ms: 1 };
/// ⛽️ A grant that covers a validate action and nothing more.
const VALIDATE_GRANT: JobBudget = JobBudget { fuel: WORK_UNITS_VALIDATE, deadline_ms: 1 };

// 🚫️async: E1 pure fixture encoder consumed by the sync `format!` call sites below; the payload it
// encodes has no await of its own.
fn io_hop_input(source: &str, target: &str, text: &str) -> Vec<u8> {
    let payload = dsl::os_pack::json::to_json_string(&semio_framework::io_schema::IoPayload::Text(text.to_string()));
    format!("{{\"source\":\"{source}\",\"target\":\"{target}\",\"payload\":{payload}}}").into_bytes()
}

// 🚫️async: E1 pure fault reader consumed by every sync assertion below.
fn fault_code(bytes: &[u8]) -> String {
    dsl::decode_fault_bytes(bytes).code.0
}

//#region 🔖️PreRewriteParity

#[semio_framework_async_macros::async_test]
async fn step_job_on_an_unknown_id_fails_without_panicking() {
    match step_job(999, FULL_GRANT).await {
        JobStep::Failed(_) => {}
        _ => panic!("an unregistered job id must fail, not succeed"),
    }
}

#[semio_framework_async_macros::async_test]
async fn cancel_job_removes_a_pending_record_so_a_later_step_fails() {
    start_job(1, JOB_KIND_IO_RUN, b"{}").await;
    cancel_job(1).await;
    match step_job(1, FULL_GRANT).await {
        JobStep::Failed(_) => {}
        _ => panic!("a cancelled job must not still be steppable"),
    }
}

#[semio_framework_async_macros::async_test]
async fn step_job_on_an_unknown_kind_fails_with_a_named_fault() {
    start_job(2, "semio.not-a-real-kind", b"{}").await;
    match step_job(2, FULL_GRANT).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.unknown-kind"),
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
    match step_job(3, FULL_GRANT).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.io-run.decode", "must reach the io-run decode state, not job.unknown-kind"),
        _ => panic!("garbage io-run input must fail, got a non-Failed step"),
    }
}

#[semio_framework_async_macros::async_test]
async fn io_sniff_dispatches_through_the_registry_and_keeps_its_decode_fault_code() {
    start_job(4, JOB_KIND_IO_SNIFF, b"not json").await;
    match step_job(4, FULL_GRANT).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.io-sniff.decode", "must reach the io-sniff decode state, not job.unknown-kind"),
        _ => panic!("garbage io-sniff input must fail"),
    }
}

//#endregion

//#region 🔖️AdmittedSet

/// 📜️ The admitted-set law. `builtin_registry()` is the ONLY producer of the builtin entries and
/// it carries no `cfg` branch of any kind, so the set this test observes is by construction the
/// set a `wasm32-wasip2` release component observes: a builtin added without a bounded state
/// machine cannot reach `BUILTIN_JOB_KINDS` without failing here first. This is the law PZ1 §3's
/// measurement asked for — before it, `spawn_job` parked every builtin as
/// `ExplicitStateMachineRequired` under `#[cfg(not(test))]` while this suite exercised a
/// `#[cfg(test)]`-only executor, so the two sets differed silently in every shipped build.
#[semio_framework_async_macros::async_test]
async fn the_admitted_builtin_set_is_exactly_the_declared_builtin_set() {
    let mut registered: Vec<&str> = builtin_registry().into_keys().collect();
    registered.sort_unstable();
    let mut declared: Vec<&str> = BUILTIN_JOB_KINDS.to_vec();
    declared.sort_unstable();
    assert_eq!(registered, declared, "builtin_registry() and BUILTIN_JOB_KINDS must name the same kinds");
    for kind in BUILTIN_JOB_KINDS {
        assert!(job_kind_is_admitted(kind), "{kind} must resolve to an admitted bounded state machine");
    }
}

/// 📜️ Every builtin kind really enters its own state machine: not one of them answers
/// `job.unknown-kind` or the retired `job.explicit-state-machine-required`, whatever its input
/// decodes to. `b"{}"` is deliberately valid JSON and invalid for all five, so each kind must fail
/// with ITS OWN decode fault rather than with an admission refusal.
#[semio_framework_async_macros::async_test]
async fn no_builtin_kind_is_refused_before_its_own_state_machine_runs() {
    for (index, kind) in BUILTIN_JOB_KINDS.into_iter().enumerate() {
        let job = 40_000 + index as u64;
        start_job(job, kind, b"{}").await;
        let step = step_job(job, FULL_GRANT).await;
        let code = match &step {
            JobStep::Failed(bytes) => fault_code(bytes),
            _ => String::new(),
        };
        assert_ne!(code, "job.unknown-kind", "{kind} must be admitted, not refused as an unknown kind");
        assert_ne!(code, "job.explicit-state-machine-required", "{kind} must not park as a dead route");
        cancel_job(job).await;
    }
}

//#endregion

//#region 🔖️StateWalk

/// 🌗️ The two-state walk of a builtin, end to end on a hop that needs no registered io entry:
/// `Decode` reports the hop identity as progress, `Execute` finishes `Done` with the sniff
/// confidence rank. A source that is not the payload's own carrier is `Confidence::None`.
#[semio_framework_async_macros::async_test]
async fn a_builtin_walks_decode_then_execute_to_a_terminal_outcome() {
    let input = io_hop_input("s.stdio.binary@raw/*", "s.jobtest.walk@1/*", "hello");
    start_job(50, JOB_KIND_IO_SNIFF, &input).await;
    match step_job(50, FULL_GRANT).await {
        JobStep::Running(Some(progress)) => assert_eq!(progress, b"s.stdio.binary@raw/*->s.jobtest.walk@1/*".to_vec()),
        JobStep::Failed(bytes) => panic!("the decode state must report progress, not fail: {}", fault_code(&bytes)),
        _ => panic!("the decode state must be Running(Some(identity))"),
    }
    match step_job(50, FULL_GRANT).await {
        JobStep::Done(bytes) => assert_eq!(bytes, vec![semio_framework::io_schema::Confidence::None.rank()]),
        JobStep::Failed(bytes) => panic!("the execute state must finish, not fail: {}", fault_code(&bytes)),
        JobStep::Running(_) => panic!("the execute state is terminal, the native sniff call is atomic"),
    }
    match step_job(50, FULL_GRANT).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.unknown", "a finished job releases its id"),
        _ => panic!("a finished job must not still be steppable"),
    }
}

/// 📸️ A builtin interrupted after its decode state checkpoints `PHASE_DECODED`, and a restore from
/// those bytes resumes straight at `Execute` — the identical final output, one step sooner.
#[semio_framework_async_macros::async_test]
async fn a_builtin_checkpoint_restore_resumes_at_the_execute_state() {
    let input = io_hop_input("s.stdio.binary@raw/*", "s.jobtest.restore@1/*", "hello");

    start_job(51, JOB_KIND_IO_SNIFF, &input).await;
    step_job(51, FULL_GRANT).await;
    let baseline = match step_job(51, FULL_GRANT).await {
        JobStep::Done(bytes) => bytes,
        JobStep::Failed(bytes) => panic!("uninterrupted run must finish Done within 2 state actions, not fail: {}", fault_code(&bytes)),
        JobStep::Running(_) => panic!("uninterrupted run must finish Done within 2 state actions"),
    };

    start_job(52, JOB_KIND_IO_SNIFF, &input).await;
    step_job(52, FULL_GRANT).await;
    let entries = checkpoint_jobs().await;
    let entry = entries.iter().find(|entry| entry.job == 52).expect("job 52 must appear in checkpoint_jobs()");
    assert_eq!(entry.checkpoint.as_deref(), Some(PHASE_DECODED), "the decode state must have checkpointed PHASE_DECODED");
    let checkpoint = entry.checkpoint.clone();
    cancel_job(52).await;

    restore_job(52, JOB_KIND_IO_SNIFF, &input, checkpoint).await;
    let restored_final = match step_job(52, FULL_GRANT).await {
        JobStep::Done(bytes) => bytes,
        _ => panic!("a restore from PHASE_DECODED must finish Done on its FIRST step_job call"),
    };
    assert_eq!(restored_final, baseline, "checkpoint/restore must produce the identical final output");
}

//#endregion

//#region 🔖️BudgetAndCancellation

/// ⛽️ Budget exhaustion is a typed refusal, not an overrun: the execute state declares
/// `WORK_UNITS_EXECUTE` and a caller granting less is told which price it could not meet.
#[semio_framework_async_macros::async_test]
async fn a_state_action_granted_less_than_its_declared_price_is_refused() {
    let input = io_hop_input("s.stdio.binary@raw/*", "s.jobtest.budget@1/*", "hello");
    start_job(60, JOB_KIND_IO_SNIFF, &input).await;
    assert!(matches!(step_job(60, VALIDATE_GRANT).await, JobStep::Running(Some(_))), "a validate grant must cover the decode state");
    match step_job(60, VALIDATE_GRANT).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.io-sniff.budget-exhausted"),
        _ => panic!("an execute state granted a validate-sized budget must be refused, not run"),
    }

    start_job(61, JOB_KIND_IO_SNIFF, &input).await;
    match step_job(61, JobBudget::default()).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.io-sniff.budget-exhausted", "an empty grant cannot even pay for a validate action"),
        _ => panic!("a zero grant must be refused"),
    }
}

/// 🛑️ Cancellation mid-walk: the owner's own next state action refuses with a typed fault, and the
/// job table has released the id so the host's next `step-job` is `job.unknown`.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_builtin_mid_walk_refuses_its_next_state_action() {
    let input = io_hop_input("s.stdio.binary@raw/*", "s.jobtest.cancel@1/*", "hello");
    let mut owner = KIND_REGISTRY.with(|registry| registry.borrow().get(JOB_KIND_IO_SNIFF).copied()).expect("io-sniff is admitted")(70, &input, None).expect("io-sniff admits");
    assert!(matches!(owner.step(FULL_GRANT), JobStep::Running(Some(_))), "the decode state must run before cancellation");
    owner.cancel();
    assert!(owner.terminal_drop_is_shallow(), "a cancelled builtin must leave a shallow wrapper");
    match owner.step(FULL_GRANT) {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.io-sniff.cancelled"),
        _ => panic!("a cancelled builtin must refuse its next state action"),
    }

    start_job(71, JOB_KIND_IO_SNIFF, &input).await;
    assert!(matches!(step_job(71, FULL_GRANT).await, JobStep::Running(Some(_))));
    cancel_job(71).await;
    match step_job(71, FULL_GRANT).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.unknown", "the id must be free, not still bound to the cancelled owner"),
        _ => panic!("a job cancelled mid-walk must not still be steppable"),
    }
}

//#endregion

//#region 🔖️BoundedFixtures

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

fn fixed_bounded_fixture(_job: u64, input: &[u8], restored: Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    if input != b"fixed" {
        return Err(b"fixed-admission".to_vec());
    }
    Ok(Box::new(FixedBoundedFixture { step: restored.and_then(|bytes| bytes.first().copied()).unwrap_or(0), cancelled: false }))
}

struct NeverProgressesFixture;

impl BoundedJob for NeverProgressesFixture {
    fn step(&mut self, _budget: JobBudget) -> JobStep {
        JobStep::Running(None)
    }

    fn cancel(&mut self) {}

    fn checkpoint(&self) -> Option<Vec<u8>> {
        None
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        true
    }
}

fn never_progresses_fixture(_job: u64, _input: &[u8], _restored: Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    Ok(Box::new(NeverProgressesFixture))
}

#[semio_framework_async_macros::async_test]
async fn a_registered_bounded_kind_advances_one_state_action_and_retains_its_admission_fault() {
    register_bounded_job_kind("test.fixed-bounded", fixed_bounded_fixture);
    start_job(9_001, "test.fixed-bounded", b"fixed").await;
    assert!(matches!(step_job(9_001, FULL_GRANT).await, JobStep::Running(Some(bytes)) if bytes == [1]));
    assert!(matches!(step_job(9_001, FULL_GRANT).await, JobStep::Done(bytes) if bytes == [2]));

    start_job(9_002, "test.fixed-bounded", b"rejected").await;
    assert!(matches!(step_job(9_002, FULL_GRANT).await, JobStep::Failed(bytes) if bytes == b"fixed-admission"));
}

/// 📸️ A plugin-declared kind receives its own checkpoint bytes on restore through the SAME factory
/// the first admission used — the third parameter `spawn_job` threads and the bounded path used to
/// drop on the floor.
#[semio_framework_async_macros::async_test]
async fn a_registered_bounded_kind_is_restored_through_its_own_factory() {
    register_bounded_job_kind("test.fixed-bounded", fixed_bounded_fixture);
    start_job(9_003, "test.fixed-bounded", b"fixed").await;
    step_job(9_003, FULL_GRANT).await;
    let entries = checkpoint_jobs().await;
    let checkpoint = entries.iter().find(|entry| entry.job == 9_003).expect("job 9_003 must appear in checkpoint_jobs()").checkpoint.clone();
    assert_eq!(checkpoint.as_deref(), Some([1u8].as_slice()));
    cancel_job(9_003).await;

    restore_job(9_003, "test.fixed-bounded", b"fixed", checkpoint).await;
    assert!(matches!(step_job(9_003, FULL_GRANT).await, JobStep::Done(bytes) if bytes == [2]), "a restore must resume past the state the checkpoint recorded");
}

#[semio_framework_async_macros::async_test]
async fn the_stall_guard_fires_after_repeated_no_progress_static_budget_steps() {
    register_bounded_job_kind("test.never-progresses", never_progresses_fixture);
    start_job(30, "test.never-progresses", b"[]").await;

    let same_budget = JobBudget { fuel: 100, deadline_ms: 50 };
    for call in 0..STALL_LIMIT {
        match step_job(30, same_budget).await {
            JobStep::Running(None) => {}
            _ => panic!("call {call} must still be Running(None) before the stall limit"),
        }
    }
    match step_job(30, same_budget).await {
        JobStep::Failed(bytes) => assert_eq!(fault_code(&bytes), "job.stalled"),
        _ => panic!("the stall guard must fire once STALL_LIMIT consecutive no-progress static-budget calls have elapsed"),
    }
}

//#endregion
