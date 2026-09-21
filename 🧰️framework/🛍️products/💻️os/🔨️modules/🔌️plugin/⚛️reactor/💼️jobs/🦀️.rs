//! 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-jobs-runtime, design-abi.md §4 + §6). Makes
//! jobs authorable: a `kind` string registry (`register_bounded_job_kind`) resolved by
//! `start_job`, each entry a `BoundedJobFactory` that admits ONE explicit bounded state machine —
//! a `BoundedJob` whose `step(budget)` performs exactly one state action and returns
//! `Running`/`Done`/`Failed`. There is one registry, one admission path and one executor shape;
//! nothing in this module is `#[cfg(test)]`-gated, so the set of kinds a test build admits and the
//! set a shipped component admits are the same set by construction (`BUILTIN_JOB_KINDS`, pinned by
//! a law in `🧪️tests/🔬️unit`).
//!
//! ## Why the opaque-future executor is gone
//! The previous shape kept TWO registries: a `JobFn` registry of `async` bodies sliced by parking
//! on a `JobCtx::tick()` inside a private `ColdFutureExecutor`, and a bounded registry. Only the
//! bounded one was admitted in a shipped build — `spawn_job` dropped every `JobFn` on the floor
//! under a `cfg(not(test))` gate and parked the slot as `ExplicitStateMachineRequired`, so all five
//! builtin kinds (`semio.io-run`, `semio.io-sniff`, `semio.infer`, `semio.mutation-plan`,
//! `semio.migrate`) were refused with `job.explicit-state-machine-required` in every production
//! build, for every plugin, while the unit suite exercised a path that only existed under
//! `cfg(test)`. The cure is not a second admission door: it is that a builtin kind IS an explicit
//! bounded state machine, authored the same way `framework.reserved.tool` and `🏗️fem`'s mounted
//! visual jobs already are. `JobFn`, `JobCtx`, `JobTick` and the per-jobs `ColdFutureExecutor` are
//! deleted rather than gated.
//!
//! ## Slicing mechanics
//! `start_job` looks up `kind` in `KIND_REGISTRY` and calls the factory with the job id, the raw
//! `input` bytes and — only on a checkpoint-restore replay — the bytes this SAME kind last handed
//! back from `BoundedJob::checkpoint()`. A factory that refuses admission returns the fault bytes
//! the first `step_job` answers with. `step_job(job, budget)` advances the admitted owner by
//! exactly one state action; it never runs another job's work, because there is no shared executor
//! left to run.
//!
//! ## Budget
//! `JobBudget::fuel` is the work-unit grant for ONE step. Every state of a builtin machine
//! declares its price (`WORK_UNITS_VALIDATE` for a decode/validate action, `WORK_UNITS_EXECUTE`
//! for one unchunked native dispatch, `WORK_UNITS_RETIRE` for one bounded close action), and a
//! step granted less than the current state's price is refused with a typed
//! `job.<kind>.budget-exhausted` fault instead of overrunning the actor grant it was called under.
//! `deadline_ms` is the host's wall bound, honoured by construction: one state action per step.
//!
//! ## Stall guard
//! If `step_job` returns `Running` with no progress bytes AND the caller passed the SAME
//! `JobBudget` as last time (no fuel/deadline change to indicate the host is doing anything
//! differently) for `STALL_LIMIT` consecutive calls, the job fails with `job.stalled` instead of
//! being steppable forever — see `step_job`'s own doc for the exact bookkeeping.
//!
//! ## Checkpoint (via lease into `⚛️reactor/📸️checkpoint/🦀️.rs`)
//! `checkpoint_jobs()`/`restore_job()` are the two functions that side of the lease calls — see
//! this ticket's `📓️terra-jobs-runtime-report.md`, `## lease-requests` section, for the exact diff
//! text sol applies.

use semio_framework_value_derive::{FromValue, ToValue};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::task::{Context, Poll};

// 🧊️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): the three remaining well-known cold job
// kinds design-abi.md §2 names (`semio.infer`/`semio.mutation-plan`/`semio.migrate` — `semio.compose`
// is the live `compose-await` packet's, not this one's). Each is one submodule under this directory
// (mirrors `⚛️reactor`'s own one-`component.rs`-per-directory convention) registered into
// `builtin_registry()` below, exactly like `job_io_run`/`job_io_sniff` already are.
#[path = "💡️infer/🦀️.rs"]
mod infer;
#[path = "🔀️migrate/🦀️.rs"]
mod migrate;
#[path = "🧬️mutation-plan/🦀️.rs"]
mod mutation_plan;

//#region 🔖️PublicTypes

/// 🗺️ Absorbed from the peer's guest export `io-run` (single hop, this plugin's own registry —
/// never chains into another plugin; multi-hop routing is the host's `io-run` EFFECT, not this
/// job kind).
pub const JOB_KIND_IO_RUN: &str = "semio.io-run";
/// 🗺️ Absorbed from the peer's guest export `io-sniff`.
pub const JOB_KIND_IO_SNIFF: &str = "semio.io-sniff";
/// 💡️ design-abi.md §2: absorbed from the deleted `contributor.artifact-infer` export — see
/// `💡️infer/🦀️.rs`.
pub const JOB_KIND_INFER: &str = "semio.infer";
/// 🧬️ design-abi.md §2: absorbed from the deleted `contributor.artifact-mutation-plan` export —
/// see `🧬️mutation-plan/🦀️.rs`.
pub const JOB_KIND_MUTATION_PLAN: &str = "semio.mutation-plan";
/// 🔀️ design-abi.md §2: absorbed from the deleted `migrate-artifact` export — see
/// `🔀️migrate/🦀️.rs`.
pub const JOB_KIND_MIGRATE: &str = "semio.migrate";

/// 📜️ Every kind a plugin gets for free, in the order `builtin_registry` installs them. This is
/// the list the admitted-set law compares the live registry against, so a builtin added without a
/// bounded state machine fails that law rather than shipping as a dead route.
pub const BUILTIN_JOB_KINDS: [&str; 5] = [JOB_KIND_IO_RUN, JOB_KIND_IO_SNIFF, JOB_KIND_INFER, JOB_KIND_MUTATION_PLAN, JOB_KIND_MIGRATE];

/// ⛽️ Plain-Rust mirror of `jobs.wit`'s `record job-budget` — this module is NOT gated to
/// `component-guest`/wasm32-wasip2 (it must compile and unit-test natively), so it cannot name the
/// WIT-generated type directly; the WIT boundary conversion lives in the (leased) `JobsGuest` impl
/// in `🔌️plugin/🦀️.rs`, field-for-field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Deserialize, FromValue, serde::Serialize, ToValue)]
pub struct JobBudget {
    pub fuel: u64,
    pub deadline_ms: u32,
}

/// ▶️ Plain-Rust mirror of `jobs.wit`'s `variant job-step` — see `JobBudget`'s doc for why this
/// isn't the WIT-generated type itself.
#[derive(serde::Deserialize, FromValue, serde::Serialize, ToValue)]
pub enum JobStep {
    Running(Option<Vec<u8>>),
    Done(Vec<u8>),
    Failed(Vec<u8>),
}

/// ⛽️ Price of one decode/validate state action: a bounded parse of `input` that allocates nothing
/// the request did not already carry.
pub const WORK_UNITS_VALIDATE: u64 = 1;
/// ⛽️ Price of one bounded retirement action — a single `close_step` page grant.
pub const WORK_UNITS_RETIRE: u64 = 1;
/// ⛽️ Price of one execute state action: a whole unchunked native dispatch
/// (`io_run`/`io_identify`/`wire_artifact_infer`/`wire_artifact_mutation_plan`/`migrate_document`).
/// None of those is itself chunked, so the machine declares the price up front and a caller whose
/// grant cannot cover it is refused before the call rather than after it overran.
pub const WORK_UNITS_EXECUTE: u64 = 1_024;
/// ⛽️ Price of one interactive-inference pump action: one bounded `MountedWorkerJobSession` step
/// submitted to the shared worker pool.
pub const WORK_UNITS_PUMP: u64 = 64;

/// 🧩️ Production job protocol. A registered owner advances one explicit bounded state-machine
/// opportunity per `step-job`; there is no other body shape, in any build.
pub trait BoundedJob {
    fn step(&mut self, budget: JobBudget) -> JobStep;
    fn cancel(&mut self);
    fn checkpoint(&self) -> Option<Vec<u8>>;
    fn terminal_drop_is_shallow(&self) -> bool;
}

/// 🏭️ Non-capturing constructor for one registered retained state machine: the job id, the raw
/// `start-job` input, and — only on a checkpoint-restore replay — the bytes this kind last handed
/// back from `BoundedJob::checkpoint()`. An `Err` carries the fault bytes the first `step_job`
/// answers with, so a refused admission surfaces exactly where a refused first step would.
pub type BoundedJobFactory = fn(u64, &[u8], Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>>;

/// 🔎️ One phase body of a builtin two-phase machine. Synchronous on purpose: a bounded step may
/// not suspend, so the framework call it wraps is settled inside the step by `settle_in_step`.
pub(crate) type BuiltinPhaseFn = fn(&[u8]) -> Result<Vec<u8>, semio_framework::Fault>;

//#endregion

//#region 🔖️Registry

crate::component_persistent_local! {
    static KIND_REGISTRY: RefCell<HashMap<&'static str, BoundedJobFactory>> = RefCell::new(builtin_registry());
}

/// 🧬️ Every builtin kind is registered unconditionally for every plugin, the same "for free"
/// behaviour the old hard-coded `match` gave every plugin — no `PluginBuilder::job(...)` call is
/// required to get them, and no build configuration changes which of them is admitted.
// 🚫️async: E4-adjacent — consumed by a `thread_local!` static initializer, which is a fixed
// sync-only language context (cannot await); the body is a pure `HashMap` of fn-pointer inserts.
fn builtin_registry() -> HashMap<&'static str, BoundedJobFactory> {
    let mut map: HashMap<&'static str, BoundedJobFactory> = HashMap::new();
    map.insert(JOB_KIND_IO_RUN, job_io_run as BoundedJobFactory);
    map.insert(JOB_KIND_IO_SNIFF, job_io_sniff as BoundedJobFactory);
    map.insert(JOB_KIND_INFER, infer::job_infer as BoundedJobFactory);
    map.insert(JOB_KIND_MUTATION_PLAN, mutation_plan::job_mutation_plan as BoundedJobFactory);
    map.insert(JOB_KIND_MIGRATE, migrate::job_migrate as BoundedJobFactory);
    map
}

/// 📤️ Called by `PluginBuilder::try_build()` (`🏗️builder/🦀️.rs`) once per `.job(kind, factory)`
/// declaration, at bundle-install time — "registered on bundle install like other builder
/// registrations" per this packet's brief. A later registration for the same `kind` overwrites an
/// earlier one (including a builtin), matching `plugin_command`'s own last-writer convention one
/// layer up minus the duplicate-id assertion (a plugin legitimately overriding `semio.io-run`'s
/// default body is not an error here).
pub fn register_bounded_job_kind(kind: &'static str, factory: BoundedJobFactory) {
    KIND_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(kind, factory);
    });
}

/// 📜️ Whether `kind` resolves to an admitted bounded state machine right now — the predicate the
/// admitted-set law reads, and the one a caller can ask before spawning.
pub fn job_kind_is_admitted(kind: &str) -> bool {
    KIND_REGISTRY.with(|registry| registry.borrow().contains_key(kind))
}

//#endregion

//#region 🔖️Slots

enum JobBody {
    Bounded(Box<dyn BoundedJob>),
    AdmissionFailed(Vec<u8>),
    UnknownKind,
}

struct JobSlot {
    kind: String,
    input: Vec<u8>,
    body: JobBody,
    last_budget_seen: Option<JobBudget>,
    stall_count: u32,
}

crate::component_persistent_local! {
    static JOBS: RefCell<HashMap<u64, JobSlot>> = RefCell::new(HashMap::new());
}

/// 🛑️ Number of consecutive `step_job` calls a job may return `Running` with no progress bytes AND
/// an unchanged `JobBudget` before the stall guard fails it — see `step_job`'s doc.
const STALL_LIMIT: u32 = 3;

//#endregion

//#region 🔖️Lifecycle

/// 📥️ `jobs::start-job` — admits `kind`'s registered state machine and parks it in the job table;
/// the first `step_job` call performs its first state action. An id already in flight is kept,
/// matching the old file's own doc note: the host never reuses a live job id, but a
/// restarted-from-checkpoint actor may legitimately replay a `start-job` for one still in flight
/// from the caller's point of view — see `restore_job` for the checkpoint-replay counterpart,
/// which threads `restored` bytes this entry point always passes as `None`.
pub async fn start_job(job: u64, kind: &str, input: &[u8]) {
    spawn_job(job, kind, input, None).await;
}

/// 📸️ Checkpoint-restore replay counterpart to `start_job` — never called from the WIT boundary
/// directly (the `jobs` WIT interface has no `restored` parameter on `start-job`); called from the
/// (leased) `⚛️reactor/📸️checkpoint/🦀️.rs::restore` for every entry `checkpoint_jobs()`
/// packed, handing each kind's factory the last `checkpoint()` bytes it produced before the actor
/// was torn down.
pub async fn restore_job(job: u64, kind: &str, input: &[u8], checkpoint: Option<Vec<u8>>) {
    spawn_job(job, kind, input, checkpoint).await;
}

async fn spawn_job(job: u64, kind: &str, input: &[u8], restored: Option<Vec<u8>>) {
    if JOBS.with(|jobs| jobs.borrow().contains_key(&job)) {
        return;
    }
    let Some(factory) = KIND_REGISTRY.with(|registry| registry.borrow().get(kind).copied()) else {
        insert_slot(job, kind, input, JobBody::UnknownKind);
        return;
    };
    let body = match factory(job, input, restored.as_deref()) {
        Ok(owner) => JobBody::Bounded(owner),
        Err(detail) => JobBody::AdmissionFailed(detail),
    };
    insert_slot(job, kind, input, body);
}

// 🚫️async: E4 pure table insert consumed by `spawn_job`'s three terminal arms; `HashMap::insert`
// inside a `LocalKey::with` closure is a fixed sync-only language context.
fn insert_slot(job: u64, kind: &str, input: &[u8], body: JobBody) {
    JOBS.with(|jobs| jobs.borrow_mut().insert(job, JobSlot { kind: kind.to_string(), input: input.to_vec(), body, last_budget_seen: None, stall_count: 0 }));
}

/// 🛑️ `jobs::cancel-job` — hands the admitted owner its cancellation, asserts it released every
/// deep resource it held, and drops the bookkeeping slot so a later `step_job` on the same id
/// reports `job.unknown`, matching the pre-rewrite behaviour exactly.
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
/// unrecognised kind is `Failed(job.unknown-kind)`; an id whose factory refused admission is
/// `Failed(<the factory's own fault bytes>)` (and the slot is dropped in every one of those cases,
/// matching the pre-rewrite one-shot-failure behaviour). Otherwise the admitted owner performs
/// exactly one state action under `budget`.
///
/// Stall guard: compares `budget` against the previous call's — if unchanged AND the owner
/// returned `Running` with no progress bytes, `stall_count` increments; any step with new progress
/// OR a changed budget resets it to zero. Reaching `STALL_LIMIT` fails the job as `job.stalled`
/// instead of returning `Running` forever.
pub async fn step_job(job: u64, budget: JobBudget) -> JobStep {
    let Some(outcome) = JOBS.with(|jobs| {
        let mut jobs = jobs.borrow_mut();
        let slot = jobs.get_mut(&job)?;
        let JobBody::Bounded(owner) = &mut slot.body else { return None };
        let budget_static = slot.last_budget_seen == Some(budget);
        slot.last_budget_seen = Some(budget);
        let step = owner.step(budget);
        let stalled = match &step {
            JobStep::Running(None) if budget_static => {
                slot.stall_count += 1;
                slot.stall_count >= STALL_LIMIT
            }
            JobStep::Running(_) => {
                slot.stall_count = 0;
                false
            }
            JobStep::Done(_) | JobStep::Failed(_) => false,
        };
        Some((step, stalled, owner.terminal_drop_is_shallow()))
    }) else {
        return refuse_unstepped(job);
    };
    let (step, stalled, shallow) = outcome;
    if stalled {
        let kind = JOBS.with(|jobs| jobs.borrow().get(&job).map(|slot| slot.kind.clone())).unwrap_or_default();
        JOBS.with(|jobs| jobs.borrow_mut().remove(&job));
        return JobStep::Failed(fault_bytes("job.stalled", format!("job {job} ({kind}) made no progress across {STALL_LIMIT} consecutive step-job calls with an unchanged budget")));
    }
    if matches!(step, JobStep::Done(_) | JobStep::Failed(_)) {
        if !shallow {
            JOBS.with(|jobs| jobs.borrow_mut().remove(&job));
            return JobStep::Failed(fault_bytes("job.bounded-false-terminal", format!("bounded job {job} returned a terminal outcome while retaining a deep wrapper owner")));
        }
        JOBS.with(|jobs| drop(jobs.borrow_mut().remove(&job)));
    }
    step
}

// 🚫️async: E1 pure error mapping consumed by `step_job`'s non-bounded arm; every branch is a table
// read plus a `Fault` constructor, both sync.
fn refuse_unstepped(job: u64) -> JobStep {
    let Some((kind, body)) = JOBS.with(|jobs| jobs.borrow_mut().remove(&job).map(|slot| (slot.kind, slot.body))) else {
        return JobStep::Failed(fault_bytes("job.unknown", format!("no job registered for id {job}")));
    };
    match body {
        JobBody::AdmissionFailed(detail) => JobStep::Failed(detail),
        JobBody::UnknownKind | JobBody::Bounded(_) => JobStep::Failed(fault_bytes("job.unknown-kind", format!("job kind {kind:?} has no admitted explicit bounded state machine"))),
    }
}

//#endregion

//#region 🔖️Checkpoint

/// 📸️ One entry of the checkpoint pack's `jobs: Vec<{job, kind, input, checkpoint}>` — see module
/// doc's checkpoint section and this packet's `## lease-requests` for the exact diff into
/// `⚛️reactor/📸️checkpoint/🦀️.rs` that embeds these.
#[derive(Clone, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
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
                    JobBody::Bounded(owner) => owner.checkpoint(),
                    JobBody::UnknownKind | JobBody::AdmissionFailed(_) => None,
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

/// 🌉️ Settles one framework call inside a bounded step. Every native dispatch a builtin kind
/// drives (`io_run`/`io_identify`/`wire_artifact_infer`/`wire_artifact_mutation_plan`/
/// `migrate_document`) is `async` by signature only — `io-async-signatures` made the SIGNATURES
/// uniform without adding a suspension point to any of those bodies — and a bounded step has no
/// executor to park on, by design (module doc, "Why the opaque-future executor is gone"). So the
/// future is polled exactly once; a `Pending` is not a hang here but a typed
/// `job.<kind>.suspended` refusal naming the guarantee a future edit broke.
fn settle_in_step(fault_prefix: &str, future: impl Future<Output = Result<Vec<u8>, semio_framework::Fault>>) -> Result<Vec<u8>, semio_framework::Fault> {
    let waker = std::task::Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = Box::pin(future);
    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => Err(fault(&format!("{fault_prefix}.suspended"), "a bounded job step drove a framework call that suspended; a builtin job kind may only drive calls that complete within one step")),
    }
}

//#endregion

//#region 🔖️TwoPhase
/// 🌗️ Shared 2-state machine for `semio.io-run`/`semio.io-sniff`/`semio.infer`'s registry route/
/// `semio.mutation-plan`/`semio.migrate`: none of their underlying native calls is itself chunked
/// — real sub-call preemption is the same "blocked upstream" gap the dormant WFC solver names
/// (`✏️s/🔌️plugins/🌀️procedural/🦀️.rs`'s own comment) — but the JOB ITSELF is genuinely sliceable:
/// two real `step_job` calls, monotonic progress on the first, a correct checkpoint/restore resume,
/// and a declared price per state, not a single call dressed up as a job. `Decode` runs on the
/// FIRST step only (validates `input` and reports identity-shaped progress bytes, then makes
/// `PHASE_DECODED` its checkpoint so a restore starts straight at `Execute`); `Execute` runs on the
/// step after, whether that is the second step of an uninterrupted run or the first step after a
/// restore. Both phases independently re-read `input` from scratch (never threading a decoded value
/// across the step boundary) — a second cheap parse is harmless and keeps `restore` correct without
/// a second, richer checkpoint payload.
pub(crate) const PHASE_DECODED: &[u8] = b"phase.decoded";

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TwoPhaseState {
    Decode,
    Execute,
    Complete,
}

pub(crate) struct TwoPhaseBoundedJob {
    fault_prefix: &'static str,
    state: TwoPhaseState,
    input: Vec<u8>,
    decode: BuiltinPhaseFn,
    execute: BuiltinPhaseFn,
    cancelled: bool,
}

impl TwoPhaseBoundedJob {
    /// 🎟️ Admits the machine, starting at `Execute` when the restore bytes say this kind already
    /// reported `PHASE_DECODED` before the actor was torn down.
    pub(crate) fn admit(fault_prefix: &'static str, input: &[u8], restored: Option<&[u8]>, decode: BuiltinPhaseFn, execute: BuiltinPhaseFn) -> Self {
        let state = if restored == Some(PHASE_DECODED) { TwoPhaseState::Execute } else { TwoPhaseState::Decode };
        Self { fault_prefix, state, input: input.to_vec(), decode, execute, cancelled: false }
    }

    // 🚫️async: E1 pure price table consumed by `step`'s sync budget gate.
    fn price(&self) -> u64 {
        match self.state {
            TwoPhaseState::Decode => WORK_UNITS_VALIDATE,
            TwoPhaseState::Execute => WORK_UNITS_EXECUTE,
            TwoPhaseState::Complete => 0,
        }
    }
}

impl BoundedJob for TwoPhaseBoundedJob {
    fn step(&mut self, budget: JobBudget) -> JobStep {
        if self.cancelled {
            self.state = TwoPhaseState::Complete;
            return JobStep::Failed(fault_bytes(&format!("{}.cancelled", self.fault_prefix), format!("{} was cancelled before its next state action", self.fault_prefix)));
        }
        let price = self.price();
        if budget.fuel < price {
            self.state = TwoPhaseState::Complete;
            return JobStep::Failed(fault_bytes(&format!("{}.budget-exhausted", self.fault_prefix), format!("{} needs {price} work units for its next state action and was granted {}", self.fault_prefix, budget.fuel)));
        }
        match self.state {
            TwoPhaseState::Decode => match (self.decode)(&self.input) {
                Ok(progress) => {
                    self.state = TwoPhaseState::Execute;
                    JobStep::Running(Some(progress))
                }
                Err(error) => {
                    self.state = TwoPhaseState::Complete;
                    JobStep::Failed(dsl::encode_fault_bytes(&error))
                }
            },
            TwoPhaseState::Execute => {
                self.state = TwoPhaseState::Complete;
                match (self.execute)(&self.input) {
                    Ok(bytes) => JobStep::Done(bytes),
                    Err(error) => JobStep::Failed(dsl::encode_fault_bytes(&error)),
                }
            }
            TwoPhaseState::Complete => JobStep::Failed(fault_bytes(&format!("{}.terminal", self.fault_prefix), format!("{} has no state action left to advance", self.fault_prefix))),
        }
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        matches!(self.state, TwoPhaseState::Execute).then(|| PHASE_DECODED.to_vec())
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        true
    }
}
//#endregion

//#region 🔖️BuiltinKinds

/// 🌉️ `input` is the JSON-encoded `{source, target, payload}` the WIT guest export `io-run` used
/// to take as three separate params; `Ok` carries the JSON-encoded `io_schema::IoPayload` result,
/// matching the old export's ok return exactly.
// 🧬️ `FromValue` only: this struct is decoded exclusively by `dsl::os_pack::json::from_json_str`
// (`from_json_str<T: FromValue>`), never by serde. The `serde::Deserialize` derive was vestigial and
// was the sole reason `io_schema::IoPayload` still had to implement `serde::Deserialize`.
#[derive(::semio_framework_value_derive::FromValue)]
struct IoRunInput {
    source: String,
    target: String,
    payload: semio_framework::io_schema::IoPayload,
}

// 🚫️async: E4 fn-pointer slot — registered into `BoundedJobFactory`, whose shape a factory must
// match exactly; the admission body is a pure constructor call.
fn job_io_run(_job: u64, input: &[u8], restored: Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    Ok(Box::new(TwoPhaseBoundedJob::admit("job.io-run", input, restored, decode_io_run, execute_io_run)))
}

// 🚫️async: E4 fn-pointer slot — see `job_io_run`'s own comment above; same factory shape.
fn job_io_sniff(_job: u64, input: &[u8], restored: Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    Ok(Box::new(TwoPhaseBoundedJob::admit("job.io-sniff", input, restored, decode_io_sniff, execute_io_sniff)))
}

/// 🔎️ Validates `input` decodes as `{source, target, payload}` and reports the hop identity as the
/// first state action's progress bytes, so a malformed request fails before the io registry is
/// ever consulted — the same decode fault code the pre-bounded `run_io_run` raised.
// 🚫️async: E4 phase slot — `BuiltinPhaseFn` is synchronous by contract (module doc, "Budget"); the
// hop parse it performs has no await of its own.
fn decode_io_run(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    decode_io_hop(input, "job.io-run", JOB_KIND_IO_RUN)
}

// 🚫️async: E4 phase slot — see `decode_io_run`; only the fault code and kind name differ.
fn decode_io_sniff(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    decode_io_hop(input, "job.io-sniff", JOB_KIND_IO_SNIFF)
}

// 🚫️async: E1 shared pure parse consumed by the two sync phase slots above; `code` is the kind's
// fault-code stem, so a malformed envelope keeps the pre-bounded `<stem>.decode` code and an
// unparseable dialect coordinate keeps the bare `<stem>` code the execute body used to raise.
fn decode_io_hop(input: &[u8], code: &str, kind: &str) -> Result<Vec<u8>, semio_framework::Fault> {
    let decode_code = format!("{code}.decode");
    let input_text = std::str::from_utf8(input).map_err(|_| fault(&decode_code, format!("invalid {kind} input")))?;
    let IoRunInput { source, target, .. } = dsl::os_pack::json::from_json_str::<IoRunInput>(input_text).map_err(|_| fault(&decode_code, format!("invalid {kind} input")))?;
    let source = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault(code, message))?;
    let target = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault(code, message))?;
    Ok(format!("{}->{}", source.to_coordinate(), target.to_coordinate()).into_bytes())
}

// 🚫️async: E4 phase slot — see `decode_io_run`; the native call inside is settled by `settle_in_step`.
fn execute_io_run(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    settle_in_step("job.io-run", run_io_run(input))
}

// 🚫️async: E4 phase slot — see `decode_io_run`; the native call inside is settled by `settle_in_step`.
fn execute_io_sniff(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    settle_in_step("job.io-sniff", run_io_sniff(input))
}

/// 🌉️ Body unchanged from the pre-rewrite `run_io_run` — only the return type moved from
/// `JobOutcome` to `Result<Vec<u8>, Fault>` so every registry entry (builtin or plugin-authored)
/// shares one outcome shape; `step_job` re-encodes an `Err` into fault bytes uniformly.
async fn run_io_run(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|_| fault("job.io-run.decode", format!("invalid {JOB_KIND_IO_RUN} input")))?;
    let IoRunInput { source, target, payload } = dsl::os_pack::json::from_json_str::<IoRunInput>(input_text).map_err(|_| fault("job.io-run.decode", format!("invalid {JOB_KIND_IO_RUN} input")))?;
    let source = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault("job.io-run", message))?;
    let target = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault("job.io-run", message))?;
    let descriptor = match semio_framework::io::io_mechanism::io_entries().into_iter().find(|entry| entry.from == source && entry.into == target) {
        Some(descriptor) => descriptor,
        None => return Err(fault("job.io-run", format!("no local io entry for hop {} -> {}", source.to_coordinate(), target.to_coordinate()))),
    };
    let fidelity = descriptor.fidelity;
    let route = semio_framework::io_schema::IoRoute { hops: vec![descriptor], fidelity };
    let outcome = semio_framework::io::io_mechanism::io_run(&route, payload).await.map_err(|error| fault("job.io-run", error.message))?;
    Ok(dsl::os_pack::json::to_json_string(&outcome.value).into_bytes())
}

/// 🔍️ Body unchanged from the pre-rewrite `run_io_sniff` — `Ok` carries a single-byte `Vec<u8>` of
/// `io_schema::Confidence::rank()` (`0..=3`), matching the old export's `u8` return.
async fn run_io_sniff(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|_| fault("job.io-sniff.decode", format!("invalid {JOB_KIND_IO_SNIFF} input")))?;
    let IoRunInput { source, target, payload } = dsl::os_pack::json::from_json_str::<IoRunInput>(input_text).map_err(|_| fault("job.io-sniff.decode", format!("invalid {JOB_KIND_IO_SNIFF} input")))?;
    let source = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault("job.io-sniff", message))?;
    let target = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault("job.io-sniff", message))?;
    let carrier = semio_framework::io_schema::ArtifactDialect::from(match &payload {
        semio_framework::io_schema::IoPayload::Binary(_) => semio_framework::io_schema::CARRIER_BINARY,
        semio_framework::io_schema::IoPayload::Text(_) => semio_framework::io_schema::CARRIER_TEXT,
    });
    if source != carrier {
        return Ok(vec![semio_framework::io_schema::Confidence::None.rank()]);
    }
    let confidence = semio_framework::io::io_mechanism::io_identify(&payload).await.into_iter().find(|(dialect, _)| *dialect == target).map_or(semio_framework::io_schema::Confidence::None, |(_, confidence)| confidence);
    Ok(vec![confidence.rank()])
}

//#endregion

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
