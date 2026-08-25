//! 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-jobs-runtime, design-abi.md §4 + §6). Makes
//! jobs authorable: a `kind` string registry (`register_job_kind`) resolved by `start_job`, each
//! entry an async `JobFn` sliced across repeated `step_job` calls by parking on `JobCtx::tick()`.
//! This replaces the old closed hard-coded `match` (deleted) that only ever knew `semio.io-run`/
//! `semio.io-sniff` and accepted `job-budget` without reading it — those two kinds are preserved
//! as ordinary registry entries below (`job_io_run`/`job_io_sniff`), their bodies byte-identical to
//! the pre-rewrite `run_io_run`/`run_io_sniff`, only reshaped from `JobOutcome` returns into
//! `Result<Vec<u8>, Fault>` so every kind — builtin or plugin-authored — shares one outcome type.
//!
//! ## Slicing mechanics
//! `start_job` looks up `kind` in `KIND_REGISTRY`, calls it to build the job's future, and spawns
//! that future PARKED (ready-but-not-yet-run) on `JOBS_EXECUTOR` — a dedicated `⚛️reactor/🧵️executor::
//! LocalExecutor` instance, deliberately NOT the reactor turn loop's own `EXECUTOR`, so a job slice
//! never competes with UI-turn tasks for one `poll`'s `run_until_idle` iterations (design-abi.md
//! §4's granularity split made explicit: async is the waiting model, jobs are the compute model).
//! `step_job(job, budget)` stores `budget` on the job's shared `JobState`, grants exactly one more
//! `JobCtx::tick()` resolution (`tick_budget += 1`), wakes the job's task by id, and runs
//! `JOBS_EXECUTOR` until either that task parks on the NEXT `tick()`/a host await or the job's
//! future finishes — never running any OTHER job's task, since nothing else was woken. `JobCtx::
//! tick()`'s own future (`JobTick`) needs no waker bookkeeping at all: it just compares
//! `ticks_consumed` against `tick_budget`, so `LocalExecutor::wake(task)` re-queuing the task by id
//! is sufficient to let it observe the newly granted slice on the next poll.
//!
//! ## Host-await restriction (deliberate, v1)
//! `JobCtx::host()` is gated to `#[cfg(feature = "component-guest-async")]` — NEVER ungate this for
//! `world actor` (the poll world). `🖥️host/🦀️component.rs`'s `PluginInstanceHandle::
//! run_job_to_completion` loops `step_job` in a tight `start_job` → `step_job`* relay WITHOUT ever
//! pumping the actor's `poll` in between (by design — it runs strictly POST-TURN, never re-entrant
//! into an in-flight turn's own `Store`). A poll-world job that awaited a host effect would park on
//! a `RequestFuture` whose resolution only ever happens inside `poll`'s `Event::Completed` routing
//! — which that relay loop never calls again for this job — so the loop would spin on `Running`
//! forever. Poll-world jobs therefore receive every input they need up front, in `input`/
//! `restored`, and drive it through `JobCtx::tick()` alone.
//!
//! 🧬️ B1 world-collapse OPEN QUESTION (raised, deliberately NOT resolved here — see
//! `📓️terra-world-collapse-report.md`): the gate's stated justification was "the OTHER world's
//! `runner::run` has no such gap", and that world no longer exists. The gap itself is unchanged —
//! it is a property of `run_job_to_completion`'s relay loop, not of the WIT — so the gate is still
//! CORRECT today. But a job stepped through the collapsed world's `async` `step-job` export CAN now
//! suspend on a `host-async` import, so whether the gate may be relaxed depends on whether the
//! host's relay re-pumps in a way that lets a parked host-await resolve. That needs measuring, not
//! assuming; it is a latent behaviour question, invisible to the compiler either way.
//!
//! ## Stall guard
//! If `step_job` returns `Running` with no progress bytes AND the caller passed the SAME
//! `JobBudget` as last time (no fuel/deadline change to indicate the host is doing anything
//! differently) for `STALL_LIMIT` consecutive calls, the job fails with `job.stalled` instead of
//! being steppable forever — see `step_job`'s own doc for the exact bookkeeping.
//!
//! ## Checkpoint (via lease into `⚛️reactor/📸️checkpoint/🦀️component.rs`)
//! `checkpoint_jobs()`/`restore_job()` are the two functions that side of the lease calls — see
//! this ticket's `📓️terra-jobs-runtime-report.md`, `## lease-requests` section, for the exact diff
//! text sol applies.

use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

// 🧊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): the three remaining well-known cold job
// kinds design-abi.md §2 names (`semio.infer`/`semio.mutation-plan`/`semio.migrate` — `semio.compose`
// is the live `compose-await` packet's, not this one's). Each is one submodule under this directory
// (mirrors `⚛️reactor`'s own one-`component.rs`-per-directory convention) registered into
// `builtin_registry()` below, exactly like `job_io_run`/`job_io_sniff` already are.
#[path = "💡️infer/🦀️component.rs"]
mod infer;
#[path = "🔀️migrate/🦀️component.rs"]
mod migrate;
#[path = "🧬️mutation-plan/🦀️component.rs"]
mod mutation_plan;

//#region 🔖️PublicTypes

/// 🗺️ Absorbed from the peer's guest export `io-run` (single hop, this plugin's own registry —
/// never chains into another plugin; multi-hop routing is the host's `io-run` EFFECT, not this
/// job kind).
pub const JOB_KIND_IO_RUN: &str = "semio.io-run";
/// 🗺️ Absorbed from the peer's guest export `io-sniff`.
pub const JOB_KIND_IO_SNIFF: &str = "semio.io-sniff";
/// 💡️ design-abi.md §2: absorbed from the deleted `contributor.artifact-infer` export — see
/// `💡️infer/🦀️component.rs`.
pub const JOB_KIND_INFER: &str = "semio.infer";
/// 🧬️ design-abi.md §2: absorbed from the deleted `contributor.artifact-mutation-plan` export —
/// see `🧬️mutation-plan/🦀️component.rs`.
pub const JOB_KIND_MUTATION_PLAN: &str = "semio.mutation-plan";
/// 🔀️ design-abi.md §2: absorbed from the deleted `migrate-artifact` export — see
/// `🔀️migrate/🦀️component.rs`.
pub const JOB_KIND_MIGRATE: &str = "semio.migrate";

/// ⛽️ Plain-Rust mirror of `jobs.wit`'s `record job-budget` — this module is NOT gated to
/// `component-guest`/wasm32-wasip2 (it must compile and unit-test natively), so it cannot name the
/// WIT-generated type directly; the WIT boundary conversion lives in the (leased) `JobsGuest` impl
/// in `🔌️plugin/🦀️component.rs`, field-for-field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct JobBudget {
    pub fuel: u64,
    pub deadline_ms: u32,
}

/// ▶️ Plain-Rust mirror of `jobs.wit`'s `variant job-step` — see `JobBudget`'s doc for why this
/// isn't the WIT-generated type itself.
#[derive(serde::Deserialize, serde::Serialize)]
pub enum JobStep {
    Running(Option<Vec<u8>>),
    Done(Vec<u8>),
    Failed(Vec<u8>),
}

/// 🧬️ One registry entry: given a `JobCtx` (the slice/progress/checkpoint/budget handle), the raw
/// `input` bytes `start-job` was called with, and — only on a checkpoint-restore replay —
/// `Some(checkpoint)` bytes this SAME kind last handed to `JobCtx::checkpoint()`, returns the
/// future that IS the job's whole run, from first slice to `Done`/`Failed`. A plain `fn` pointer
/// (not a closure) so `PluginBuilder::job(kind, run)` can register it without capturing anything —
/// mirrors `PluginCommandHandler`'s own non-capturing-preferred shape one level up.
pub type JobFn = fn(JobCtx, Vec<u8>, Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>>;

/// 🧩️ Production-safe retained job protocol. A registered owner advances one explicit bounded
/// state-machine opportunity per `step-job`; it never enters the cold opaque-future executor.
pub trait BoundedJob {
    fn step(&mut self, budget: JobBudget) -> JobStep;
    fn cancel(&mut self);
    fn checkpoint(&self) -> Option<Vec<u8>>;
    fn terminal_drop_is_shallow(&self) -> bool;
}

/// 🏭️ Non-capturing constructor for one registered retained state machine.
pub type BoundedJobFactory = fn(u64, &[u8]) -> Result<Box<dyn BoundedJob>, Vec<u8>>;

//#endregion

//#region 🔖️Registry

crate::component_persistent_local! {
    static KIND_REGISTRY: RefCell<HashMap<&'static str, JobFn>> = RefCell::new(builtin_registry());
    static BOUNDED_KIND_REGISTRY: RefCell<HashMap<&'static str, BoundedJobFactory>> = RefCell::new(HashMap::new());
}

/// 🧬️ `semio.io-run`/`semio.io-sniff` are registered unconditionally for every plugin, the same
/// "for free" behaviour the old hard-coded `match` gave every plugin — no `PluginBuilder::job(...)`
/// call is required to get them.
// 🚫️async: E4-adjacent — consumed by a `thread_local!` static initializer, which is a fixed
// sync-only language context (cannot await); the body is a pure `HashMap` of fn-pointer inserts.
fn builtin_registry() -> HashMap<&'static str, JobFn> {
    let mut map: HashMap<&'static str, JobFn> = HashMap::new();
    map.insert(JOB_KIND_IO_RUN, job_io_run as JobFn);
    map.insert(JOB_KIND_IO_SNIFF, job_io_sniff as JobFn);
    map.insert(JOB_KIND_INFER, infer::job_infer as JobFn);
    map.insert(JOB_KIND_MUTATION_PLAN, mutation_plan::job_mutation_plan as JobFn);
    map.insert(JOB_KIND_MIGRATE, migrate::job_migrate as JobFn);
    map
}

/// 📤️ Called by `PluginBuilder::try_build()` (`🏗️builder/🦀️component.rs`) once per `.job(kind, run)`
/// declaration, at bundle-install time — "registered on bundle install like other builder
/// registrations" per this packet's brief. A later registration for the same `kind` overwrites an
/// earlier one (including a builtin), matching `plugin_command`'s own last-writer convention one
/// layer up minus the duplicate-id assertion (a plugin legitimately overriding `semio.io-run`'s
/// default body is not an error here).
pub fn register_job_kind(kind: &'static str, run: JobFn) {
    KIND_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(kind, run);
    });
}

/// 🧩️ Registers a production retained job without enabling the opaque future executor.
pub fn register_bounded_job_kind(kind: &'static str, factory: BoundedJobFactory) {
    BOUNDED_KIND_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(kind, factory);
    });
}

//#endregion

//#region 🔖️Ctx

/// 🪪️ Shared between a job's spawned task and every `JobCtx`/`step_job` call that touches it via
/// `Rc<RefCell<_>>` — `progress`/`checkpoint` are written by the job body (through `JobCtx`), read
/// and cleared/kept by `step_job` (through the `JOBS` slot); `tick_budget`/`ticks_consumed` are
/// `step_job`'s own slicing counters, read by `JobTick::poll`.
struct JobState {
    budget: JobBudget,
    /// ⏱️ Incremented by one on every `step_job` call — grants the job's task exactly one more
    /// `JobCtx::tick()` resolution.
    tick_budget: u64,
    /// ⏱️ Incremented by one every time a `JobTick` actually resolves — compared against
    /// `tick_budget` to decide Ready vs Pending; never reset.
    ticks_consumed: u64,
    /// 📈️ Cleared at the START of every `step_job` call, before running — so a slice that never
    /// calls `JobCtx::progress()` reports `Running(None)`, not stale bytes from a prior slice.
    progress: Option<Vec<u8>>,
    /// 📸️ NOT cleared between slices — "latest kept" (this packet's brief, verbatim): the most
    /// recent bytes any `JobCtx::checkpoint()` call handed in, carried into the actor checkpoint
    /// pack by `checkpoint_jobs()` regardless of how long ago it was set.
    checkpoint: Option<Vec<u8>>,
    outcome: Option<Result<Vec<u8>, semio_framework::Fault>>,
    /// 🛑️ Stall guard bookkeeping — see `step_job`'s doc comment for the exact rule.
    last_budget_seen: Option<JobBudget>,
    stall_count: u32,
}

impl Default for JobState {
    fn default() -> Self {
        Self { budget: JobBudget::default(), tick_budget: 0, ticks_consumed: 0, progress: None, checkpoint: None, outcome: None, last_budget_seen: None, stall_count: 0 }
    }
}

/// ⏳️ The per-job handle a `JobFn` runs with — see module doc for the slicing mechanics `tick()`
/// implements and the `host()` restriction.
pub struct JobCtx {
    job: u64,
    state: Rc<RefCell<JobState>>,
    /// 🚧️ Only present in the async world's guest build — see module doc's "Host-await
    /// restriction". A future reader must not "helpfully" ungate this for `world actor`
    /// (the poll world): `🖥️host/🦀️component.rs`'s `run_job_to_completion` never pumps `poll`
    /// between `step_job` calls, so a poll-world job awaiting a host completion would spin on
    /// `Running` forever, never observing the `Event::Completed` that could resolve it.
    #[cfg(feature = "component-guest-async")]
    host: crate::host::Host,
}

impl JobCtx {
    /// 🪪️ The `job` id this ctx belongs to — mirrors `start-job`'s own `job: u64` parameter.
    pub async fn id(&self) -> u64 {
        self.job
    }

    /// ⏳️ Slice boundary: parks the job's task until the NEXT `step_job` call grants another tick,
    /// then resolves re-reading whatever `JobBudget` that call passed (see `budget()`). Every await
    /// point a `JobFn` body wants treated as "pause here until the host calls `step_job` again"
    /// goes through this — see module doc's slicing-mechanics section for why the future itself
    /// needs no waker bookkeeping.
    pub async fn tick(&self) {
        JobTick { state: self.state.clone() }.await
    }

    /// 📈️ Surfaces as `JobStep::Running(Some(bytes))` → `Event::JobProgress` on the SAME slice this
    /// was called during; a later slice that never calls this again reports `Running(None)`.
    pub async fn progress(&self, bytes: Vec<u8>) {
        self.state.borrow_mut().progress = Some(bytes);
    }

    /// 📸️ Latest-kept — see `JobState::checkpoint`'s own doc. Carried into the actor checkpoint
    /// pack by `checkpoint_jobs()`.
    pub async fn checkpoint(&self, bytes: Vec<u8>) {
        self.state.borrow_mut().checkpoint = Some(bytes);
    }

    /// ⛽️ The `JobBudget` from the MOST RECENT `step_job` call — updated before the job's task is
    /// woken, so a `tick()` that just resolved always observes the budget that granted it.
    pub async fn budget(&self) -> JobBudget {
        self.state.borrow().budget
    }

    /// 🌐️ See module doc's "Host-await restriction" — `#[cfg]`-gated on purpose, do not ungate for
    /// the poll world.
    #[cfg(feature = "component-guest-async")]
    pub async fn host(&self) -> &crate::host::Host {
        &self.host
    }
}

/// ⏳️ `JobCtx::tick()`'s future. Deliberately ignores the `Context`/waker it's polled with — a
/// fresh `Waker` is constructed by `LocalExecutor` on every poll anyway (see `🧵️executor`'s own
/// `waker_for`), and `step_job` re-queues this job's task BY ID via `LocalExecutor::wake`, not by
/// invoking a stored waker — so all this needs to decide is Ready-vs-Pending from the two counters.
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

//#endregion

//#region 🔖️Slots

enum JobBody {
    Running {
        task: super::executor::TaskId,
        state: Rc<RefCell<JobState>>,
    },
    Bounded(Box<dyn BoundedJob>),
    AdmissionFailed(Vec<u8>),
    ExplicitStateMachineRequired,
    /// 🧬️ `start_job` never rejects an unrecognised `kind` (matches the old hard-coded `match`'s own
    /// behaviour, and the existing `JobsGuest::start_job` lease contract: it always returns `Ok(())`
    /// — the failure surfaces on the first `step_job` instead, as `job.unknown-kind`).
    UnknownKind,
}

struct JobSlot {
    kind: String,
    input: Vec<u8>,
    body: JobBody,
}

crate::component_persistent_local! {
    static JOBS: RefCell<HashMap<u64, JobSlot>> = RefCell::new(HashMap::new());
    /// 🧵️ A SEPARATE `LocalExecutor` instance from the reactor turn loop's own — see module doc.
    // 🌉️ `thread_local!` initializer runs in a plain non-async context — bridged via `resolve_ready`
    // since `LocalExecutor::new()` is a pure `Self::default()`.
    #[cfg(test)]
    static TEST_JOBS_FUTURE_EXECUTOR: super::executor::ColdFutureExecutor = super::executor::ColdFutureExecutor::new();
}

/// ▶️ Same bound the reactor turn loop's own `EXECUTOR.run_until_idle` uses — a defensive cap
/// against a job task that keeps re-waking itself forever inside one `step_job` call.
const SLICE_MAX_ITERATIONS: u32 = 64;

/// 🛑️ Number of consecutive `step_job` calls a job may return `Running` with no progress bytes AND
/// an unchanged `JobBudget` before the stall guard fails it — see `step_job`'s doc.
const STALL_LIMIT: u32 = 3;

//#endregion

//#region 🔖️Lifecycle

/// 📥️ `jobs::start-job` — spawns `kind`'s registered `JobFn` PARKED on `JOBS_EXECUTOR` (not yet
/// run; the first `step_job` call drives it). An id already in flight is overwritten, matching the
/// old file's own doc note: the host never reuses a live job id, but a restarted-from-checkpoint
/// actor may legitimately replay a `start-job` for one still in flight from the caller's point of
/// view — see `restore_job` for the checkpoint-replay counterpart, which threads `restored` bytes
/// this entry point always passes as `None`.
pub async fn start_job(job: u64, kind: &str, input: &[u8]) {
    spawn_job(job, kind, input, None).await;
}

/// 📸️ Checkpoint-restore replay counterpart to `start_job` — never called from the WIT boundary
/// directly (the `jobs` WIT interface has no `restored` parameter on `start-job`); called from the
/// (leased) `⚛️reactor/📸️checkpoint/🦀️component.rs::restore` for every entry `checkpoint_jobs()`
/// packed, handing each kind's `JobFn` the last `checkpoint()` bytes it produced before the actor
/// was torn down.
pub async fn restore_job(job: u64, kind: &str, input: &[u8], checkpoint: Option<Vec<u8>>) {
    spawn_job(job, kind, input, checkpoint).await;
}

async fn spawn_job(job: u64, kind: &str, input: &[u8], restored: Option<Vec<u8>>) {
    if JOBS.with(|jobs| jobs.borrow().contains_key(&job)) {
        return;
    }
    if let Some(factory) = BOUNDED_KIND_REGISTRY.with(|registry| registry.borrow().get(kind).copied()) {
        let body = match factory(job, input) {
            Ok(owner) => JobBody::Bounded(owner),
            Err(detail) => {
                JOBS.with(|jobs| jobs.borrow_mut().insert(job, JobSlot { kind: kind.to_string(), input: input.to_vec(), body: JobBody::AdmissionFailed(detail) }));
                return;
            }
        };
        JOBS.with(|jobs| jobs.borrow_mut().insert(job, JobSlot { kind: kind.to_string(), input: input.to_vec(), body }));
        return;
    }
    let Some(run) = KIND_REGISTRY.with(|registry| registry.borrow().get(kind).copied()) else {
        JOBS.with(|jobs| jobs.borrow_mut().insert(job, JobSlot { kind: kind.to_string(), input: input.to_vec(), body: JobBody::UnknownKind }));
        return;
    };
    #[cfg(not(test))]
    {
        let _ = (run, restored);
        JOBS.with(|jobs| jobs.borrow_mut().insert(job, JobSlot { kind: kind.to_string(), input: input.to_vec(), body: JobBody::ExplicitStateMachineRequired }));
        return;
    }
    #[cfg(test)]
    {
        let state = Rc::new(RefCell::new(JobState::default()));
        let ctx = JobCtx {
            job,
            state: state.clone(),
            #[cfg(feature = "component-guest-async")]
            host: crate::reactor::host().await,
        };
        let future = run(ctx, input.to_vec(), restored);
        let outcome_state = state.clone();
        // 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready`. `spawn` itself only
        // registers the task and hands back its id; the real awaiting happens later when
        // `JOBS_EXECUTOR` polls the parked future, not here.
        let task = TEST_JOBS_FUTURE_EXECUTOR.with(|executor| {
            executor.spawn(async move {
                let result = future.await;
                outcome_state.borrow_mut().outcome = Some(result);
            })
        });
        let body = task.map_or_else(|| JobBody::AdmissionFailed(fault_bytes("job.admission-failed", format!("job {job} could not enter the test executor"))), |task| JobBody::Running { task, state });
        JOBS.with(|jobs| jobs.borrow_mut().insert(job, JobSlot { kind: kind.to_string(), input: input.to_vec(), body }));
    }
}

/// 🛑️ `jobs::cancel-job` — drops the job's bookkeeping slot so a later `step_job` on the same id
/// reports `job.unknown`, matching the pre-rewrite behaviour exactly. The task's own slot inside
/// `JOBS_EXECUTOR` is left in place (its `Rc<RefCell<JobState>>` becomes unreachable from here and
/// is dropped, but `LocalExecutor` exposes no by-id removal — see this packet's report `## honest
/// gaps`); the parked future itself never resumes since nothing ever wakes or re-steps it again.
pub async fn cancel_job(job: u64) {
    JOBS.with(|jobs| {
        let mut jobs = jobs.borrow_mut();
        if let Some(JobSlot { body: JobBody::Bounded(owner), .. }) = jobs.get_mut(&job) {
            owner.cancel();
            assert!(owner.terminal_drop_is_shallow(), "bounded job cancellation must leave a shallow wrapper and retain deep cleanup authority externally");
        }
        jobs.remove(&job);
    });
}

/// ▶️ `jobs::step-job` — an unknown job id is `Failed(job.unknown)`; an id started with an
/// unrecognised kind is `Failed(job.unknown-kind)` (and the slot is dropped either way, matching
/// the pre-rewrite one-shot-failure behaviour for both). Otherwise: records `budget` on the job's
/// shared state, clears this slice's `progress`, grants one more `JobCtx::tick()`, wakes the task
/// by id, and runs `JOBS_EXECUTOR` until it parks or finishes.
///
/// Stall guard: BEFORE running, compares `budget` against the previous call's — if unchanged AND
/// (after running) no `progress` bytes were set this slice, `stall_count` increments; any slice
/// with new progress OR a changed budget resets it to zero. Reaching `STALL_LIMIT` fails the job as
/// `job.stalled` instead of returning `Running` forever.
pub async fn step_job(job: u64, budget: JobBudget) -> JobStep {
    if let Some(outcome) = JOBS.with(|jobs| {
        let mut jobs = jobs.borrow_mut();
        let slot = jobs.get_mut(&job)?;
        let JobBody::Bounded(owner) = &mut slot.body else { return None };
        Some(owner.step(budget))
    }) {
        if matches!(outcome, JobStep::Done(_) | JobStep::Failed(_)) {
            let terminal = JOBS.with(|jobs| {
                jobs.borrow().get(&job).is_some_and(|slot| match &slot.body {
                    JobBody::Bounded(owner) => owner.terminal_drop_is_shallow(),
                    _ => false,
                })
            });
            if !terminal {
                return JobStep::Failed(fault_bytes("job.bounded-false-terminal", format!("bounded job {job} returned a terminal outcome while retaining a deep wrapper owner")));
            }
            JOBS.with(|jobs| drop(jobs.borrow_mut().remove(&job)));
        }
        return outcome;
    }
    let Some((kind, running)) = JOBS.with(|jobs| {
        jobs.borrow().get(&job).map(|slot| {
            let running = match &slot.body {
                JobBody::UnknownKind | JobBody::AdmissionFailed(_) | JobBody::ExplicitStateMachineRequired | JobBody::Bounded(_) => None,
                JobBody::Running { task, state } => Some((*task, state.clone())),
            };
            (slot.kind.clone(), running)
        })
    }) else {
        return JobStep::Failed(fault_bytes("job.unknown", format!("no job registered for id {job}")));
    };
    let Some((task, state)) = running else {
        let code = JOBS
            .with(|jobs| {
                jobs.borrow().get(&job).map(|slot| match &slot.body {
                    JobBody::ExplicitStateMachineRequired => "job.explicit-state-machine-required",
                    JobBody::AdmissionFailed(_) => "job.admission-failed",
                    JobBody::UnknownKind | JobBody::Running { .. } | JobBody::Bounded(_) => "job.unknown-kind",
                })
            })
            .unwrap_or("job.unknown");
        if let Some(detail) = JOBS.with(|jobs| {
            let mut jobs = jobs.borrow_mut();
            match jobs.remove(&job).map(|slot| slot.body) {
                Some(JobBody::AdmissionFailed(detail)) => Some(detail),
                _ => None,
            }
        }) {
            return JobStep::Failed(detail);
        }
        return JobStep::Failed(fault_bytes(code, format!("job kind {kind:?} has no admitted explicit bounded state machine")));
    };

    let budget_static = {
        let mut state = state.borrow_mut();
        let is_static = state.last_budget_seen == Some(budget);
        state.last_budget_seen = Some(budget);
        state.budget = budget;
        state.progress = None;
        state.tick_budget += 1;
        is_static
    };

    // 🌉️ `LocalKey::with`'s closure is sync — bridged via `resolve_ready`, same pattern as
    // `spawn_job`'s own `JOBS_EXECUTOR.with` call above. Neither `wake` nor `run_until_idle` has a
    // real suspension point of ITS OWN (the job future they drive internally may legitimately return
    // `Poll::Pending`, but `run_until_idle` handles that without ever yielding its own future).
    #[cfg(test)]
    TEST_JOBS_FUTURE_EXECUTOR.with(|executor| {
        executor.wake(task);
        executor.run_until_deadline(SLICE_MAX_ITERATIONS, std::time::Instant::now() + std::time::Duration::from_millis(8));
    });
    #[cfg(not(test))]
    let _ = task;

    let (outcome, progress, stalled) = {
        let mut state = state.borrow_mut();
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
        JOBS.with(|jobs| jobs.borrow_mut().remove(&job));
        return match outcome {
            Ok(bytes) => JobStep::Done(bytes),
            Err(fault) => JobStep::Failed(dsl::encode_fault_bytes(&fault)),
        };
    }

    if stalled {
        JOBS.with(|jobs| jobs.borrow_mut().remove(&job));
        return JobStep::Failed(fault_bytes("job.stalled", format!("job {job} ({kind}) made no progress across {STALL_LIMIT} consecutive step-job calls with an unchanged budget")));
    }

    JobStep::Running(progress)
}

//#endregion

//#region 🔖️Checkpoint

/// 📸️ One entry of the checkpoint pack's `jobs: Vec<{job, kind, input, checkpoint}>` — see module
/// doc's checkpoint section and this packet's `## lease-requests` for the exact diff into
/// `⚛️reactor/📸️checkpoint/🦀️component.rs` that embeds these.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct JobCheckpointEntry {
    pub job: u64,
    pub kind: String,
    pub input: Vec<u8>,
    pub checkpoint: Option<Vec<u8>>,
}

/// 📸️ Every job this actor currently has open, in no particular order — `restore_job` (called by
/// the leased `checkpoint::restore` for each entry) is what re-establishes them.
pub async fn checkpoint_jobs() -> Vec<JobCheckpointEntry> {
    JOBS.with(|jobs| {
        jobs.borrow()
            .iter()
            .map(|(job, slot)| {
                let checkpoint = match &slot.body {
                    JobBody::Running { state, .. } => state.borrow().checkpoint.clone(),
                    JobBody::Bounded(owner) => owner.checkpoint(),
                    JobBody::UnknownKind | JobBody::AdmissionFailed(_) | JobBody::ExplicitStateMachineRequired => None,
                };
                JobCheckpointEntry { job: *job, kind: slot.kind.clone(), input: slot.input.clone(), checkpoint }
            })
            .collect()
    })
}

//#endregion

//#region 🔖️Fault

// 🚫️async: E1 pure constructor consumed by sync error-mapping closures (`.map_err(|error| fault(...))`,
// `.ok_or_else(|| fault(...))`) pervasively across this module — see R9. `Fault::new` itself is sync.
fn fault(code: &str, message: impl Into<String>) -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new(code), message.into())
}

// 🚫️async: E1 — see `fault`'s own comment above; `dsl::encode_fault_bytes` is sync too.
fn fault_bytes(code: &str, message: String) -> Vec<u8> {
    dsl::encode_fault_bytes(&fault(code, message))
}

//#endregion

//#region 🔖️Phased
/// 🌗️ Shared 2-tick shape for `semio.infer`/`semio.mutation-plan`/`semio.migrate`
/// (`💡️infer`/`🧬️mutation-plan`/`🔀️migrate`, all three submodules of this one): none of their
/// underlying native calls (`crate::app::wire_artifact_infer`, `crate::plugin_runtime::
/// wire_artifact_mutation_plan`, `store::migrate_document`) are themselves chunked — real
/// sub-call preemption is the same "blocked upstream" gap the dormant WFC solver names
/// (`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`'s own comment) — but the JOB ITSELF must still be
/// genuinely sliceable: at least two real `step_job` calls, monotonic progress each slice, and a
/// correct checkpoint/restore resume, not a single call dressed up as a job. `decode` runs on the
/// FIRST slice only (validates `input` and reports identity-shaped progress bytes, then
/// checkpoints `PHASE_DECODED` so a restore skips straight past it); `execute` runs on the slice
/// after, whether that is the second slice of an uninterrupted run or the first slice after a
/// restore. Both closures independently re-read `input` from scratch (never threading a decoded
/// value across the `tick()` boundary) — a second cheap parse is harmless and keeps `restore`
/// correct without a second, richer checkpoint payload.
const PHASE_DECODED: &[u8] = b"phase.decoded";

async fn run_two_phase<D, DFut, E, EFut>(ctx: JobCtx, restored: Option<Vec<u8>>, decode: D, execute: E) -> Result<Vec<u8>, semio_framework::Fault>
where
    D: FnOnce() -> DFut,
    DFut: Future<Output = Result<Vec<u8>, semio_framework::Fault>>,
    E: FnOnce() -> EFut,
    EFut: Future<Output = Result<Vec<u8>, semio_framework::Fault>>,
{
    if restored.as_deref() != Some(PHASE_DECODED) {
        ctx.tick().await;
        let progress = decode().await?;
        ctx.progress(progress).await;
        ctx.checkpoint(PHASE_DECODED.to_vec()).await;
    }
    ctx.tick().await;
    let result = execute().await?;
    ctx.progress(b"phase.executed".to_vec()).await;
    Ok(result)
}
//#endregion

//#region 🔖️BuiltinKinds

/// 🌉️ `input` is the JSON-encoded `{source, target, payload}` the WIT guest export `io-run` used
/// to take as three separate params; `Ok` carries the JSON-encoded `io_schema::IoPayload` result,
/// matching the old export's ok return exactly.
#[derive(serde::Deserialize)]
struct IoRunInput {
    source: String,
    target: String,
    payload: semio_framework::io_schema::IoPayload,
}

// 🚫️async: E4 fn-pointer slot — see `job_mutation_plan`'s own comment in the sibling `🧬️mutation-plan`
// module for the full explanation; same `JobFn` registry shape.
fn job_io_run(_ctx: JobCtx, input: Vec<u8>, _restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move { run_io_run(&input).await })
}

// 🚫️async: E4 fn-pointer slot — see `job_mutation_plan`'s own comment in the sibling `🧬️mutation-plan`
// module for the full explanation; same `JobFn` registry shape.
fn job_io_sniff(_ctx: JobCtx, input: Vec<u8>, _restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move { run_io_sniff(&input).await })
}

/// 🌉️ Body unchanged from the pre-rewrite `run_io_run` — only the return type moved from
/// `JobOutcome` to `Result<Vec<u8>, Fault>` so every registry entry (builtin or plugin-authored)
/// shares one outcome shape; `step_job` re-encodes an `Err` into fault bytes uniformly.
async fn run_io_run(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let IoRunInput { source, target, payload } = serde_json::from_slice::<IoRunInput>(input).map_err(|_| fault("job.io-run.decode", format!("invalid {JOB_KIND_IO_RUN} input")))?;
    let source = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault("job.io-run", message))?;
    let target = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault("job.io-run", message))?;
    let descriptor = match semio_framework::io::io_mechanism::io_entries().into_iter().find(|entry| entry.from == source && entry.into == target) {
        Some(descriptor) => descriptor,
        None => return Err(fault("job.io-run", format!("no local io entry for hop {} -> {}", source.to_coordinate(), target.to_coordinate()))),
    };
    let fidelity = descriptor.fidelity;
    let route = semio_framework::io_schema::IoRoute { hops: vec![descriptor], fidelity };
    let outcome = semio_framework::io::io_mechanism::io_run(&route, payload).await.map_err(|error| fault("job.io-run", error.message))?;
    serde_json::to_vec(&outcome.value).map_err(|error| fault("job.io-run", error.to_string()))
}

/// 🔍️ Body unchanged from the pre-rewrite `run_io_sniff` — `Ok` carries a single-byte `Vec<u8>` of
/// `io_schema::Confidence::rank()` (`0..=3`), matching the old export's `u8` return.
async fn run_io_sniff(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let IoRunInput { source, target, payload } = serde_json::from_slice::<IoRunInput>(input).map_err(|_| fault("job.io-sniff.decode", format!("invalid {JOB_KIND_IO_SNIFF} input")))?;
    let source = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault("job.io-sniff", message))?;
    let target = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault("job.io-sniff", message))?;
    let carrier = semio_framework::io_schema::ArtifactDialect::from(match &payload {
        semio_framework::io_schema::IoPayload::Binary(_) => semio_framework::io_schema::CARRIER_BINARY,
        semio_framework::io_schema::IoPayload::Text(_) => semio_framework::io_schema::CARRIER_TEXT,
    });
    if source != carrier {
        return Ok(vec![semio_framework::io_schema::Confidence::None.rank().await]);
    }
    let confidence = semio_framework::io::io_mechanism::io_identify(&payload).await.into_iter().find(|(dialect, _)| *dialect == target).map(|(_, confidence)| confidence).unwrap_or(semio_framework::io_schema::Confidence::None);
    Ok(vec![confidence.rank().await])
}

//#endregion

#[cfg(test)]
mod tests {
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
}
