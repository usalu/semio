//! 🧪️ terra-brep-probe (packet brep-probe, ticket MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME):
//! answers Q1-Q3 of the probe spec (📓️luna-brep-await-spec.md §5/§7) with a REAL executor +
//! a modeled guest-internal async "kernel" that returns genuine `Pending` (not a single-poll
//! fast path). Two modules below:
//!   - `executor` (in `🦀️src/executor_patched.rs`): a byte-for-byte copy of the PRODUCTION file
//!     🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧵️executor/🦀️component.rs with a
//!     MINIMAL 3-site patch — see that file's module doc. `#[path]`-including the original
//!     VERBATIM was tried first and failed with 9 compiler errors: the mechanical async
//!     conversion broke it too (missing `.await` on two internal calls, and turned the raw-waker
//!     vtable's 4 functions `async fn`, which `core::task::RawWakerVTable` cannot accept — those
//!     must stay synchronous, a hard constraint, not a missing-`.await` oversight). This is
//!     itself a reportable finding, not just probe plumbing.
//!   - `jobs_harness`: a faithful reimplementation of the SLICING ALGORITHM in
//!     ⚛️reactor/💼️jobs/🦀️component.rs (JobState/JobTick/step_job, lines ~146-415) — the literal
//!     file cannot be vendored standalone (it depends on `semio_framework`/`dsl` crates out of
//!     scope for a self-contained spike crate per this packet's brief), so this module ports the
//!     exact counter/wake/stall-guard logic, algorithm-identical, with inline references to the
//!     original line numbers at each point that matters.

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

#[path = "executor_patched.rs"]
mod executor;

use executor::LocalExecutor;

/// ⏳️ Every method on `LocalExecutor` is itself an `async fn` that never actually awaits
/// anything internally (mirrors the real repo-wide "universal async" convention — see
/// 📓️terra-brep-probe-report.md's Q5 evidence: `semio-s-plugin-stdio` has ZERO `.await` in
/// 1600+ `async fn`s). `pollster::block_on` is the exact bridge the production code itself uses
/// (`⚙️engine/🦀️component.rs` line 129-134) — reusing it here (not reinventing a poller) keeps
/// this probe's driving mechanism identical to what the real 134 call sites already do today.
fn drive<F: Future>(f: F) -> F::Output {
    pollster::block_on(f)
}

//#region 🔖️KernelStub — models a guest-internal async kernel op with GENUINE multi-poll Pending.
// 🐛️ The REAL BrepKernel (✳️brep/🧬️schema/⚙️engine/🦀️component.rs, 184 internal async fn) never
// does this — grep confirms ZERO `.await` inside the entire ✳️brep/🧬️schema tree, so every real
// kernel future resolves Ready on its FIRST poll. This stub exists to answer the spec's literal
// question ("does the executor correctly drive a guest-internal future with real Pending
// returns") for the FORWARD-LOOKING case (a future kernel op, or any other guest-internal
// async source, that legitimately suspends), since the real kernel today can't exercise that
// path at all — see the report's headline finding.

/// 🪜️ Resolves to `Ready(remaining_at_call halving-style token)` only after being polled
/// `steps` times. Each `Pending` return stores the waker and — depending on `self_driving` —
/// either wakes itself immediately (models a cooperative multi-tick CPU computation that always
/// has more work queued) or leaves the waker parked untouched (models a future waiting on
/// something that may never arrive — Q3's "never-ready" case).
struct Countdown {
    remaining: u32,
    self_driving: bool,
    polls_seen: u32,
    parked_waker: Rc<RefCell<Option<Waker>>>,
}

impl Countdown {
    fn new(steps: u32, self_driving: bool, parked_waker: Rc<RefCell<Option<Waker>>>) -> Self {
        Self { remaining: steps, self_driving, polls_seen: 0, parked_waker }
    }
}

impl Future for Countdown {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        let this = self.get_mut();
        this.polls_seen += 1;
        if this.remaining == 0 {
            return Poll::Ready(this.polls_seen);
        }
        this.remaining -= 1;
        if this.self_driving {
            cx.waker().wake_by_ref();
        } else {
            *this.parked_waker.borrow_mut() = Some(cx.waker().clone());
        }
        Poll::Pending
    }
}
//#endregion

//#region 🔖️JobsHarness — faithful port of ⚛️reactor/💼️jobs/🦀️component.rs's slicing algorithm.
mod jobs_harness {
    use super::*;

    /// ⛽️ Mirrors `JobBudget` (jobs/component.rs lines 89-93) — not needed for this probe's
    /// budget-echo tests, kept minimal (just the stall-guard-relevant equality).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct JobBudget {
        pub fuel: u64,
    }

    pub enum JobStep {
        Running(Option<Vec<u8>>),
        Done(Vec<u8>),
        Failed(String),
    }

    /// 🪪️ Mirrors `JobState` (lines 152-171) minus fields this probe doesn't exercise
    /// (checkpoint/outcome-as-Result — kept as plain String error for simplicity).
    struct JobState {
        tick_budget: u64,
        ticks_consumed: u64,
        progress: Option<Vec<u8>>,
        outcome: Option<Result<Vec<u8>, String>>,
        last_budget_seen: Option<JobBudget>,
        stall_count: u32,
    }

    impl Default for JobState {
        fn default() -> Self {
            Self { tick_budget: 0, ticks_consumed: 0, progress: None, outcome: None, last_budget_seen: None, stall_count: 0 }
        }
    }

    /// ⏳️ Mirrors `JobCtx` (lines 181-232) — `tick()`/`progress()` only, the two primitives this
    /// probe's job bodies need.
    #[derive(Clone)]
    pub struct JobCtx {
        state: Rc<RefCell<JobState>>,
    }

    impl JobCtx {
        pub async fn tick(&self) {
            JobTick { state: self.state.clone() }.await
        }

        pub fn progress(&self, bytes: Vec<u8>) {
            self.state.borrow_mut().progress = Some(bytes);
        }
    }

    /// ⏳️ Mirrors `JobTick` (lines 234-254) EXACTLY: ignores the waker, decides Ready-vs-Pending
    /// purely from the two counters — because the real `step_job` re-queues by id via
    /// `LocalExecutor::wake`, not via a stored waker.
    struct JobTick {
        state: Rc<RefCell<JobState>>,
    }

    impl Future for JobTick {
        type Output = ();
        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
            let mut state = self.state.borrow_mut();
            if state.ticks_consumed < state.tick_budget {
                state.ticks_consumed += 1;
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    /// ▶️ Mirrors `SLICE_MAX_ITERATIONS` (line 282).
    const SLICE_MAX_ITERATIONS: u32 = 64;
    /// 🛑️ Mirrors `STALL_LIMIT` (line 286).
    const STALL_LIMIT: u32 = 3;

    pub struct Job {
        executor: LocalExecutor,
        task: executor::TaskId,
        state: Rc<RefCell<JobState>>,
    }

    /// 📥️ Mirrors `spawn_job` (lines 311-332): builds the job's future via `run`, spawns it
    /// PARKED on a dedicated `LocalExecutor` (mirrors `JOBS_EXECUTOR`, lines 274-278 — a SEPARATE
    /// instance per job here since this probe runs one job at a time, same isolation property).
    pub fn start_job<F, Fut>(run: F) -> Job
    where
        F: FnOnce(JobCtx) -> Fut,
        Fut: Future<Output = Result<Vec<u8>, String>> + 'static,
    {
        let executor = LocalExecutor::default();
        let state = Rc::new(RefCell::new(JobState::default()));
        let ctx = JobCtx { state: state.clone() };
        let future = run(ctx);
        let outcome_state = state.clone();
        let task = drive(executor.spawn(async move {
            let result = future.await;
            outcome_state.borrow_mut().outcome = Some(result);
        }));
        Job { executor, task, state }
    }

    /// ▶️ Mirrors `step_job` (lines 355-415) EXACTLY: grant one more tick, wake, run_until_idle,
    /// read back outcome/progress, run the stall guard, return Running/Done/Failed.
    pub fn step_job(job: &Job, budget: JobBudget) -> JobStep {
        let budget_static = {
            let mut state = job.state.borrow_mut();
            let is_static = state.last_budget_seen == Some(budget);
            state.last_budget_seen = Some(budget);
            state.progress = None;
            state.tick_budget += 1;
            is_static
        };

        drive(job.executor.wake(job.task));
        drive(job.executor.run_until_idle(SLICE_MAX_ITERATIONS));

        let (outcome, progress, stalled) = {
            let mut state = job.state.borrow_mut();
            let outcome = state.outcome.take();
            let progress = state.progress.clone();
            if outcome.is_none() {
                if progress.is_none() && budget_static {
                    state.stall_count += 1;
                } else {
                    state.stall_count = 0;
                }
            }
            (outcome, progress, state.stall_count >= STALL_LIMIT)
        };

        if let Some(outcome) = outcome {
            return match outcome {
                Ok(bytes) => JobStep::Done(bytes),
                Err(message) => JobStep::Failed(message),
            };
        }
        if stalled {
            return JobStep::Failed(format!("job.stalled after {STALL_LIMIT} consecutive no-progress static-budget calls"));
        }
        JobStep::Running(progress)
    }
}
//#endregion

//#region 🔖️Q1 — LocalExecutor drives a guest task's multi-step await, progress across pumps.
fn q1_local_executor_drives_multi_pending_await() -> bool {
    println!("\n=== Q1: LocalExecutor drives a guest task's multi-Pending await across pumps ===");
    let executor = LocalExecutor::default();
    let result: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(None));
    let result_inner = result.clone();
    let poll_log: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
    let poll_log_inner = poll_log.clone();

    // 🌱️ A self-driving guest task: awaits a Countdown that returns Pending 5 times before
    // Ready, waking itself synchronously each time (models cooperative multi-step guest-internal
    // compute) — genuine `Pending`, not a fast path (`polls_seen` proves it, logged below).
    let parked_waker = Rc::new(RefCell::new(None));
    drive(executor.spawn(async move {
        let polls_seen = Countdown::new(5, true, parked_waker).await;
        poll_log_inner.borrow_mut().push(polls_seen);
        result_inner.borrow_mut().replace(polls_seen);
    }));

    // ▶️ Pump with a TIGHT iteration cap (1 per pump) so completion REQUIRES multiple
    // `run_until_idle` calls — proving progress across pumps, not a single-poll fast path.
    let mut pumps = 0;
    let mut still_pending = true;
    while still_pending && pumps < 20 {
        still_pending = drive(executor.run_until_idle(1));
        pumps += 1;
        println!("  pump #{pumps}: run_until_idle(1) -> pending={still_pending}, result={:?}", result.borrow());
    }

    let final_result = *result.borrow();
    let polls = poll_log.borrow().first().copied();
    let pass = final_result == Some(6) // 5 Pending + 1 final Ready poll = 6 polls total
        && polls == Some(6)
        && pumps > 1; // MUST have taken more than one pump — proves multi-pump driving, not single-poll
    println!("  final result = {final_result:?}, total polls_seen = {polls:?}, pumps required = {pumps}");
    println!("  Q1 {}", if pass { "PASS" } else { "FAIL" });
    pass
}
//#endregion

//#region 🔖️Q2 — job step (register_job_kind-shaped) drives a >=3-step await via JobCtx::tick().
fn q2_job_step_drives_multi_tick_kernel_await() -> bool {
    println!("\n=== Q2: job step (JobCtx::tick() slicing) drives a >=3-step guest-internal await ===");
    use jobs_harness::*;

    // 🧬️ Job body shaped exactly like the spec's §5 example (`brep_job_await_box_prim`): each
    // slice does `ctx.tick().await` (parks until the NEXT step_job call), THEN awaits a
    // guest-internal kernel-shaped future inline. 3 ticks -> requires >=3 step_job calls.
    let job = start_job(|ctx: JobCtx| async move {
        for i in 1..=3u8 {
            ctx.tick().await;
            // 🪜️ Inline guest-internal "kernel op": 2 genuine Pending returns, self-driving
            // (matches Q1's shape) — proves the SAME JOBS_EXECUTOR-equivalent instance drives
            // both the tick() parking AND a hand-rolled multi-poll future inside one slice.
            let parked_waker = Rc::new(RefCell::new(None));
            let polls = Countdown::new(2, true, parked_waker).await;
            ctx.progress(vec![i, polls as u8]);
        }
        Ok(vec![0xFF])
    });

    let mut steps = 0;
    let mut outcome_bytes: Option<Vec<u8>> = None;
    let mut progress_log = Vec::new();
    loop {
        steps += 1;
        match step_job(&job, JobBudget { fuel: steps as u64 }) {
            JobStep::Running(progress) => {
                println!("  step-job #{steps}: Running(progress={progress:?})");
                if let Some(p) = progress {
                    progress_log.push(p);
                }
                if steps > 10 {
                    println!("  Q2 FAIL: exceeded 10 step-job calls without completion");
                    return false;
                }
            }
            JobStep::Done(bytes) => {
                println!("  step-job #{steps}: Done({bytes:?})");
                outcome_bytes = Some(bytes);
                break;
            }
            JobStep::Failed(message) => {
                println!("  step-job #{steps}: Failed({message})");
                println!("  Q2 FAIL: job failed unexpectedly");
                return false;
            }
        }
    }

    // 🩹️ NOTE: slice 3's `ctx.progress(...)` call still runs (verified by the real algorithm's
    // own semantics, jobs/component.rs lines 401-407) but is swallowed: when `outcome` resolves
    // Some on the SAME slice, `step_job` returns `Done(bytes)`, never `Running(progress)` — so
    // only slices that finish WITHOUT also completing the job surface progress here. 2 of 3
    // slices doing so (the last is absorbed into `Done`) is the CORRECT, faithful behavior, not
    // a probe bug — matches production exactly.
    let pass = outcome_bytes == Some(vec![0xFF]) && steps >= 3 && progress_log.len() == 2;
    println!("  total step-job calls = {steps}, progress slices observed = {}", progress_log.len());
    println!("  Q2 {}", if pass { "PASS" } else { "FAIL" });
    pass
}
//#endregion

//#region 🔖️Q3 — a never-ready guest-internal future inside a job step does NOT hang the host.
fn q3_never_ready_future_does_not_hang_the_host() -> bool {
    println!("\n=== Q3: a never-ready guest-internal future inside a job step does not hang the store ===");
    use jobs_harness::*;

    // 🪤️ Job body awaits a Countdown that is NOT self-driving — its waker is stored and then
    // NEVER invoked by anyone (models a guest-internal future stuck waiting on something that
    // never arrives). The job can never legitimately finish.
    let parked_waker: Rc<RefCell<Option<Waker>>> = Rc::new(RefCell::new(None));
    let parked_waker_for_job = parked_waker.clone();
    let job = start_job(move |ctx: JobCtx| {
        let parked_waker = parked_waker_for_job;
        async move {
            ctx.tick().await;
            let _ = Countdown::new(1_000_000, false, parked_waker).await; // never resolves
            Ok(vec![0])
        }
    });

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut steps = 0;
    let mut failed_with_stall = false;
    let mut every_call_returned_promptly = true;
    loop {
        steps += 1;
        let call_start = Instant::now();
        let step = step_job(&job, JobBudget { fuel: 1 }); // SAME budget every call -> stall guard eligible
        let call_elapsed = call_start.elapsed();
        if call_elapsed > Duration::from_millis(200) {
            every_call_returned_promptly = false;
        }
        match step {
            JobStep::Running(progress) => {
                println!("  step-job #{steps}: Running(progress={progress:?}) in {call_elapsed:?}");
            }
            JobStep::Failed(message) => {
                println!("  step-job #{steps}: Failed({message}) in {call_elapsed:?}");
                failed_with_stall = message.contains("job.stalled");
                break;
            }
            JobStep::Done(_) => {
                println!("  Q3 FAIL: a never-ready future must not report Done");
                return false;
            }
        }
        if Instant::now() > deadline {
            println!("  Q3 FAIL: exceeded 5s wall-clock budget — host-level hang detected");
            return false;
        }
    }

    let pass = failed_with_stall && every_call_returned_promptly && steps <= 10;
    println!("  total step-job calls before stall-fail = {steps}, every call < 200ms = {every_call_returned_promptly}");
    println!("  Q3 {}", if pass { "PASS" } else { "FAIL" });
    pass
}
//#endregion

fn main() {
    let q1 = q1_local_executor_drives_multi_pending_await();
    let q2 = q2_job_step_drives_multi_tick_kernel_await();
    let q3 = q3_never_ready_future_does_not_hang_the_host();

    println!("\n=== VERDICT ===");
    println!("Q1 (LocalExecutor multi-pump drive)   : {}", if q1 { "PASS" } else { "FAIL" });
    println!("Q2 (job-step multi-tick drive)         : {}", if q2 { "PASS" } else { "FAIL" });
    println!("Q3 (never-ready future does not hang)  : {}", if q3 { "PASS" } else { "FAIL" });

    if q1 && q2 && q3 {
        println!("VERDICT: guest-internal kernel-shaped awaits are driven correctly, no host hang.");
        std::process::exit(0);
    } else {
        println!("VERDICT: at least one probe FAILED — see above.");
        std::process::exit(1);
    }
}
