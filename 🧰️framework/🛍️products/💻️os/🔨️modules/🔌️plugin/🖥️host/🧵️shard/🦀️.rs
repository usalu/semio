//! 🧵️ `ShardLoop` — `design-runtime.md` §2/§"ShardTransport": the logic one shard runs, IN-PROCESS.
//! Owns a set of live [`super::GuestInstance`]s, pulls [`ShardFrame`]s off a
//! [`semio_framework_actor::ShardTransport`], groups their envelopes per actor, drives
//! [`super::GuestRuntime::execute_turn`]/[`super::GuestRuntime::step_job`], and sends the resulting
//! [`semio_framework::kernel::TurnResult`]/[`super::JobStep`] back over the SAME transport as bytes.
//!
//! MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1c, one-pool-worker-runtime): a `ShardLoop` no longer
//! implies a dedicated OS thread — [`executor::ShardExecutor`] drives one behind a plain [`Mutex`],
//! scheduled as jobs onto the shared, process-wide `semio_framework_async::WorkerPool` (see that
//! module's own doc for the single-flight scheduling protocol). `pump`/`pump_primed`'s own contract
//! is UNCHANGED by that move: "drain and execute everything currently buffered in one call" is
//! exactly what one `WorkerPool` job burst wants, whether it used to be one iteration of a dedicated
//! thread's park/pump loop or is now one pool-scheduled job. Written so the identical type can also
//! be driven over stdio by a helper PROCESS (`ProcessTransport`) or a browser Worker — the only thing
//! that changes across "in-process pool-scheduled", "child process", "web worker" is which
//! [`ShardTransports`] variant `ShardLoop::new` receives; `pump`'s own body never branches on which
//! one it got (only the closed-set enum's own delegation impl does — O1/R1's dyn replacement, packet
//! host-dedyn).
//!
//! terra-shard-grants: the wire carries [`ShardFrame`], not raw [`Envelope`] bytes — the kernel's
//! DRR-computed, throttle-scaled per-turn budget now travels WITH the envelopes it grants
//! ([`ShardFrame::Grant`]) instead of `pump` re-deriving one from local constants.

// 🏃️ `ShardExecutor` — one `ShardLoop` per shard, scheduled onto the shared `WorkerPool` (P1c; no
// dedicated OS thread since). Declared here (not in `🖥️host/🦀️.rs`, a file this packet's
// boundary excludes) — `#[path]` on a submodule resolves relative to THIS file's own directory, so
// this reaches `🧵️shard/🧵️executor/🦀️.rs` without any edit to the crate-root module tree.
#[path = "🧵️executor/🦀️.rs"]
pub mod executor;

#[path = "🔁️lifecycle/🦀️.rs"]
mod lifecycle;
use lifecycle::AdmittedAuthority;
pub use lifecycle::{ShardActorAllocation, ShardRegistrationReason, ShardRegistrationRejected};

use super::{GuestInstance, GuestRuntime, GuestRuntimes, JobBudget, JobStep, PluginHostError, TurnFault};
#[cfg(test)]
use super::{GuestInstanceState, MockGuestRuntime, PackageHash, PackageId, PackageRef};
use semio_framework::kernel::{Budget, Effect, Event, JobPlacement, RequestOutcome, TurnResult};
use semio_framework_actor::{ActorId, Envelope, JobCheckpoint, JobCommitCandidate, JobOperation, JobPublication, JobReplayRequest, JobStepOutcome, JobTurn, Payload, ShardTransport};
use semio_framework_trace::{Generation, InteractiveStage, OperationId, Watchdog};
use std::collections::{BTreeSet, HashMap};
use std::mem::{MaybeUninit, size_of};
use std::sync::Arc;
#[cfg(test)]
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

//#region 📨️ShardFrame
/// 📨️ terra-shard-grants: what actually crosses a [`ShardTransport`] INBOUND (host → shard) —
/// replacing raw [`Envelope`] pack bytes so the kernel's DRR-computed, throttle-scaled per-turn
/// [`semio_framework_actor::Budget`] can travel WITH the envelopes it grants, instead of
/// `ShardLoop::pump` re-deriving one from its own local constants (the deleted `budget_for`
/// closure / `TURN_BUDGET` / `JOB_STEP_BUDGET`). Same pack encoding on the thread transport and
/// [`super::process_transport::ProcessTransport`]/`StdioTransport` — `design-runtime.md` §2's
/// "thread-or-process, same wire" promise.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(target_pointer_width = "64", expect(clippy::large_enum_variant, reason = "An admitted frame retains its envelope inline through validation and handoff without allocating an additional wrapper."))]
pub enum ShardFrame {
    /// 📌️ Announces that `actor` is now live on this shard. A `GuestInstance` cannot cross a
    /// transport (`wasmtime::Store` is not serializable), so an INCOMING `Register` has no
    /// instantiation side effect in `ShardLoop::pump` — the real instantiate/[`ShardLoop::register`]
    /// call always happens locally. This frame exists for a coordinator on the OTHER end (a router
    /// in front of several `ShardExecutor`s, not built by this packet) to keep its own
    /// actor→shard routing table in sync with what actually landed here, over the SAME wire
    /// `Grant`/`Envelope` already use.
    Register { actor: ActorId },
    /// ✂️ Mirrors `Register` for teardown — UNLIKE `Register`, an incoming `Unregister` DOES have
    /// real behavior in `ShardLoop::pump`: it calls [`ShardLoop::unregister`] directly, since the
    /// state to do so (`self.instances`) is already local.
    Unregister { actor: ActorId },
    /// ⚖️ The DRR-computed, throttle-scaled budget for `actor`'s next turn(s), plus the envelopes
    /// it must be spent on — mirrors `semio_framework_actor::TurnGrant` field-for-field (that type
    /// additionally carries `shard`, which the shard receiving this frame already knows is itself).
    /// `ShardLoop::pump` remembers `budget` as `actor`'s "last granted budget" (`Self::
    /// granted_budget`) — used for THIS grant's own envelopes, any later standalone `Envelope`
    /// frame for the same actor, and that actor's job steps.
    Grant { actor: ActorId, budget: semio_framework_actor::Budget, envelopes: Vec<Envelope> },
    /// 🔌️ Passthrough for one raw envelope, budget-less — kept so the web `ShardClient`/
    /// `WorkerTransport` (and any other not-yet-migrated caller) can adopt this wire incrementally
    /// in a later packet without both ends changing atomically; this is NOT redundant with `Grant`,
    /// do not remove it. Runs under the actor's LAST granted budget, falling back to the
    /// Maintenance lane's default for an actor that was never granted one.
    Envelope(Envelope),
}

impl ShardFrame {
    async fn tag(&self) -> u8 {
        match self {
            ShardFrame::Register { .. } => 0,
            ShardFrame::Unregister { .. } => 1,
            ShardFrame::Grant { .. } => 2,
            ShardFrame::Envelope(_) => 3,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor::pack::write_u8(out, self.tag().await).await;
        match self {
            ShardFrame::Register { actor } => actor.pack_encode(out).await,
            ShardFrame::Unregister { actor } => actor.pack_encode(out).await,
            ShardFrame::Grant { actor, budget, envelopes } => {
                actor.pack_encode(out).await;
                budget.pack_encode(out).await;
                semio_framework_actor::pack::write_vec(out, envelopes, async |o, e| o.pack_encode(e).await).await;
            }
            ShardFrame::Envelope(envelope) => envelope.pack_encode(out).await,
        }
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor::pack::PackError> {
        let tag = semio_framework_actor::pack::read_u8(bytes, pos, "ShardFrame").await?;
        match tag {
            0 => Ok(ShardFrame::Register { actor: ActorId::pack_decode(bytes, pos).await? }),
            1 => Ok(ShardFrame::Unregister { actor: ActorId::pack_decode(bytes, pos).await? }),
            2 => Ok(ShardFrame::Grant {
                actor: ActorId::pack_decode(bytes, pos).await?,
                budget: semio_framework_actor::Budget::pack_decode(bytes, pos).await?,
                envelopes: semio_framework_actor::pack::read_vec(bytes, pos, "ShardFrame::Grant::envelopes", Envelope::pack_decode).await?,
            }),
            3 => Ok(ShardFrame::Envelope(Envelope::pack_decode(bytes, pos).await?)),
            other => Err(semio_framework_actor::pack::PackError::InvalidTag { what: "ShardFrame", tag: other, offset: *pos }),
        }
    }
}
//#endregion 📨️ShardFrame

//#region 🔀️BudgetBridge
/// ⛽️ `semio_framework_actor::Budget` (what a `Grant` carries) has no UI-frame-pacing field — that
/// concept only ever existed in the kernel crate's `Budget`/`reactor.wit`'s `budget` record, added
/// for wgpu-native's own turn pacing before the DRR scheduler existed. Documented gap, not a
/// fabricated value: a fixed, conservative default until whichever packet unifies the two
/// `Budget` vocabularies gives this a real per-turn source.
const GRANT_BUDGET_DEFAULT_MAX_FRAMES: u32 = 8;

/// 🔀️ A `Grant`'s DRR-computed [`semio_framework_actor::Budget`] → the
/// [`semio_framework::kernel::Budget`] `GuestRuntime::execute_turn` actually takes.
/// `wall_ms`→`deadline_ms` (both are "how long this turn may run", named differently per crate);
/// `max_frames` has no source field yet (see [`GRANT_BUDGET_DEFAULT_MAX_FRAMES`]).
async fn turn_budget_from_grant(budget: semio_framework_actor::Budget) -> Budget {
    Budget { fuel: budget.fuel, deadline_ms: budget.wall_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: GRANT_BUDGET_DEFAULT_MAX_FRAMES }
}

/// 🔀️ Same `Grant` budget, `GuestRuntime::step_job`'s shape — `JobBudget` only ever carried
/// `fuel`/`deadline_ms`, so this is a straight field mapping, no invented default needed.
async fn job_budget_from_grant(budget: semio_framework_actor::Budget) -> JobBudget {
    JobBudget { fuel: budget.fuel, deadline_ms: budget.wall_ms }
}

/// 🌉️ `semio_framework::kernel::TurnResult` (what `GuestRuntime::execute_turn` returns) →
/// `semio_framework_actor::TurnResult` (what the actor crate's `Kernel::complete` scheduler
/// bookkeeping wants) — the exact bridge the wgpu-native host's `KernelThreadState::
/// apply_turn_result` flagged as unreached ("bridging the two needs a real pack-encode step this
/// packet didn't reach"). Lives HERE, not in `🖥️host/🦀️.rs` — `📌️important.md` rule 17:
/// a concurrent packet series owns that file, and this ticket already absorbed several
/// half-landed collisions of exactly this shape ("the artifact moved, its registration did not").
///
/// `ui_patches`/`effects` stay opaque bytes on the actor-crate side. The bridge consumes the kernel
/// patch owner into a generation-qualified fixed transport lease and advances one fixed patch
/// operation per yield before publishing the complete lease token. `status` maps 1:1.
pub async fn to_actor_turn_result(mut result: TurnResult, session: u64, wall_us: u64, memory_bytes: u64) -> Result<semio_framework_actor::TurnResult, semio_framework::Fault> {
    to_actor_turn_result_in_place(&mut result, session, wall_us, memory_bytes).await
}

async fn to_actor_turn_result_in_place(result: &mut TurnResult, session: u64, wall_us: u64, memory_bytes: u64) -> Result<semio_framework_actor::TurnResult, semio_framework::Fault> {
    let status = match &result.status {
        semio_framework::kernel::TurnStatus::Idle => semio_framework_actor::TurnStatus::Idle,
        semio_framework::kernel::TurnStatus::MoreWork => semio_framework_actor::TurnStatus::MoreWork,
        semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint } => semio_framework_actor::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
        semio_framework::kernel::TurnStatus::Faulted(detail) => semio_framework_actor::TurnStatus::Faulted { detail: detail.clone() },
    };
    if let Err(reason) = result.validate_ui_patch_receipt() {
        while !result.ui_patches.close_step() {
            semio_framework_async::yield_once().await;
        }
        return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-patches-receipt"), reason));
    }
    let effects = serde_json::to_vec(&result.effects).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-effects-encode"), error.to_string()))?;
    let command_ingress = serde_json::to_vec(&result.command_ingress).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-ingress-encode"), error.to_string()))?;
    let mut owner = std::mem::take(&mut result.ui_patches);
    let ui_patches = if owner.is_empty() {
        while !owner.close_step() {
            semio_framework_async::yield_once().await;
        }
        Vec::new()
    } else {
        let mut patch_transport = match semio_framework::kernel::UiTurnPatchTransportProducer::try_new(session, owner) {
            Ok(producer) => producer,
            Err(mut owner) => {
                while !owner.close_step() {
                    semio_framework_async::yield_once().await;
                }
                return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-patches-admission"), "fixed turn patch transport admission refused the exact owner"));
            }
        };
        loop {
            match patch_transport.drive_one(session, false, false) {
                semio_framework::kernel::UiTurnPatchTransportStep::MoreWork | semio_framework::kernel::UiTurnPatchTransportStep::Blocked => semio_framework_async::yield_once().await,
                semio_framework::kernel::UiTurnPatchTransportStep::Ready => break,
                semio_framework::kernel::UiTurnPatchTransportStep::Cancelled | semio_framework::kernel::UiTurnPatchTransportStep::Stale => {
                    return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-patches-transport"), "fixed turn patch transport became stale before publication"));
                }
                semio_framework::kernel::UiTurnPatchTransportStep::Fault(reason) => return Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-patches-transport"), reason)),
            }
        }
        loop {
            match patch_transport.take_ready().map_err(|reason| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.turn-patches-publication"), reason))? {
                Some(token) => break token.to_vec(),
                None => semio_framework_async::yield_once().await,
            }
        }
    };
    Ok(semio_framework_actor::TurnResult {
        ui_patches,
        effects,
        command_ingress,
        cold_pair_ingress: result.cold_pair_ingress.clone(),
        lifecycle_receipt: result.lifecycle_receipt,
        ui_patch_receipt: result.ui_patch_receipt,
        next_wake: result.next_wake,
        status,
        usage: semio_framework_actor::Usage { fuel: result.fuel_used, wall_us, memory_bytes },
    })
}
//#endregion 🔀️BudgetBridge

/// 📤️ Owned pack-coded outcome sent from a shard to its scheduler-side consumer.
#[derive(Clone, Debug, PartialEq)]
pub enum ShardOutcome {
    Turn {
        actor: u64,
        result: semio_framework_actor::TurnResult,
    },
    Job {
        actor: u64,
        authority: JobTurn,
        request: JobReplayRequest,
        placement: JobPlacement,
        publication: JobPublication,
    },
    Fault {
        actor: u64,
        message: String,
    },
    /// 📸️ `Payload::Suspend`'s operation-bound checkpoint and committed progress boundary.
    Checkpoint {
        actor: u64,
        operation: JobOperation,
        checkpoint: JobCheckpoint,
    },
    /// ▶️ `Payload::Resume`'s success outcome after restoring its explicit checkpoint bytes.
    Resumed {
        actor: u64,
        operation: JobOperation,
    },
    /// 🛑️ `Payload::Cancel`'s outcome: every one of the actor's `running_jobs` was cancelled via
    /// [`super::GuestRuntime::cancel_job`] and its [`super::GuestInstance`] was unregistered
    /// (dropped) — see `ShardLoop::pump`'s dispatch arm for the semantics this variant confirms.
    Cancelled {
        actor: u64,
    },
}

impl ShardOutcome {
    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), semio_framework_actor::pack::PackError> {
        if let Self::Turn { result, .. } = self {
            if result.lifecycle_receipt.is_some_and(|receipt| !receipt.is_valid()) {
                return Err(semio_framework_actor::pack::PackError::InvalidLifecycle("invalid turn receipt authority"));
            }
        }
        match self {
            Self::Turn { actor, result } => {
                semio_framework_actor::pack::write_u8(out, 0).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                result.pack_encode(out).await?;
            }
            Self::Job { actor, authority, request, placement, publication } => {
                semio_framework_actor::pack::write_u8(out, 1).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                authority.pack_encode(out).await;
                request.pack_encode(out).await;
                semio_framework_actor::pack::write_u8(
                    out,
                    match placement {
                        JobPlacement::Inline => 0,
                        JobPlacement::Isolated => 1,
                        JobPlacement::Exclusive => 2,
                    },
                )
                .await;
                publication.pack_encode(out).await;
            }
            Self::Fault { actor, message } => {
                semio_framework_actor::pack::write_u8(out, 2).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                semio_framework_actor::pack::write_str(out, message).await;
            }
            Self::Checkpoint { actor, operation, checkpoint } => {
                semio_framework_actor::pack::write_u8(out, 3).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                operation.pack_encode(out).await;
                checkpoint.pack_encode(out).await;
            }
            Self::Resumed { actor, operation } => {
                semio_framework_actor::pack::write_u8(out, 4).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                operation.pack_encode(out).await;
            }
            Self::Cancelled { actor } => {
                semio_framework_actor::pack::write_u8(out, 5).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
            }
        }
        Ok(())
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor::pack::PackError> {
        let tag = semio_framework_actor::pack::read_u8(bytes, pos, "ShardOutcome").await?;
        let actor = semio_framework_actor::pack::read_u64(bytes, pos, "ShardOutcome::actor").await?;
        match tag {
            0 => Ok(Self::Turn { actor, result: semio_framework_actor::TurnResult::pack_decode(bytes, pos).await? }),
            1 => {
                let authority = JobTurn::pack_decode(bytes, pos).await?;
                let request = JobReplayRequest::pack_decode(bytes, pos).await?;
                let placement = match semio_framework_actor::pack::read_u8(bytes, pos, "ShardOutcome::Job::placement").await? {
                    0 => JobPlacement::Inline,
                    1 => JobPlacement::Isolated,
                    2 => JobPlacement::Exclusive,
                    tag => return Err(semio_framework_actor::pack::PackError::InvalidTag { what: "ShardOutcome::Job::placement", tag, offset: *pos }),
                };
                Ok(Self::Job { actor, authority, request, placement, publication: JobPublication::pack_decode(bytes, pos).await? })
            }
            2 => Ok(Self::Fault { actor, message: semio_framework_actor::pack::read_str(bytes, pos, "ShardOutcome::Fault::message").await? }),
            3 => Ok(Self::Checkpoint { actor, operation: JobOperation::pack_decode(bytes, pos).await?, checkpoint: JobCheckpoint::pack_decode(bytes, pos).await? }),
            4 => Ok(Self::Resumed { actor, operation: JobOperation::pack_decode(bytes, pos).await? }),
            5 => Ok(Self::Cancelled { actor }),
            other => Err(semio_framework_actor::pack::PackError::InvalidTag { what: "ShardOutcome", tag: other, offset: *pos }),
        }
    }
}

/// 🧵️ design-runtime.md §2. One `ShardLoop` per shard (an OS thread today, a `[[bin]]` process in
/// P1) — never shared across shards, since [`super::GuestRuntime`] instances are `Send + Sync` but a
/// [`GuestInstance`] is pinned to whichever shard activated it (`ShardTable`'s own pinning rule).
pub struct ShardLoop {
    runtime: Arc<GuestRuntimes>,
    transport: ShardTransports,
    instances: HashMap<u64, GuestInstance>,
    allocations: HashMap<u64, ShardActorAllocation>,
    next_registration: u64,
    /// 💼️ `(actor, job)` pairs admitted from an `Effect::SpawnJob` and not yet `Done`/`Failed` —
    /// MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1, the generic host-side executor
    /// `📓️terra-M5-report.md` §4(a) found missing entirely: nothing previously read a
    /// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ..}` and spawned/drove a job
    /// for it outside the three hardcoded kinds `PluginInstanceHandle::run_job_to_completion`
    /// calls directly. `pump()` steps every entry here exactly once per call — never loops a job
    /// to completion internally — so a job needing N steps needs N `pump()` calls, which is what
    /// proves resumability rather than a single-shot call.
    running_jobs: BTreeSet<(u64, u64)>,
    /// 🪪️ Active replay identity and sequence cursor for every running job.
    job_turns: HashMap<(u64, u64), JobTurn>,
    /// 📰️ Independently minted operation identity retained from spawn until terminal outcome.
    job_authorities: HashMap<(u64, u64), JobAuthority>,
    /// 📄️ Fixed, generation-qualified original spawn owners retained for live restart and replay.
    replay_seeds: Box<[Option<MountedReplaySeed>]>,
    replay_seed_refusals: Box<[Option<ReplaySpawnRefusal>]>,
    replay_seed_cursor: usize,
    next_job_operation: u64,
    /// 🚦 `JobPlacement` (inline/isolated/exclusive) captured per `running_jobs` entry at
    /// `Effect::SpawnJob` admission — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME K1: `placement` used
    /// to be matched and immediately discarded (`_` in the `Effect::SpawnJob` arm). `Exclusive`
    /// entries are routed to the FRONT of `to_step`'s per-pump order (see that construction site's
    /// doc comment) — the honest, in-shard-only approximation of "dedicated" access a single
    /// `ShardLoop` can give without a cross-shard/`Kernel`-level job-forwarding mechanism, which
    /// this packet's report flags as a `lease-request` rather than faking. Entries are removed
    /// alongside their matching `running_jobs` entry everywhere the latter is removed.
    job_placement: HashMap<(u64, u64), JobPlacement>,
    /// 📨️ `Event::JobCompleted` synthesized when a `running_jobs` entry reaches `Done`/`Failed`,
    /// queued per originating actor and delivered at the TOP of the NEXT `pump()` call (merged
    /// into that call's `events_by_actor`) — so a job's own actor sees the completion on its next
    /// turn even if no other envelope ever arrives for it, exactly the way a real `Event::
    /// Completed` would reach the guest's `RequestRegistry` (`job == req.0`, see `🌐host/
    /// 🦀️.rs`'s `Host::spawn_job` / `⚛️reactor/🦀️.rs`'s `Event::JobCompleted`
    /// routing step).
    terminal_authorities: FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    /// ⚖️ terra-shard-grants: the budget from the LAST [`ShardFrame::Grant`] seen for each actor —
    /// replaces the deleted `budget_for` closure / `TURN_BUDGET` / `JOB_STEP_BUDGET` constants.
    /// Read by [`Self::granted_budget`]; an actor with no entry (never granted) falls back to the
    /// Maintenance lane's default, per that method's own doc.
    granted_budgets: HashMap<u64, semio_framework_actor::Budget>,
    /// 🚦 terra-shard-lane (piece 1, `📓️terra-shard-lane-report.md`): the lane from the LAST
    /// [`Envelope`] this shard has dispatched for each actor (`Self::dispatch_envelope`, which
    /// sees every envelope bundled in a `ShardFrame::Grant` as well as every standalone
    /// `ShardFrame::Envelope`). The kernel's DRR `Scheduler` fixes one lane per actor at
    /// registration, so every envelope it ever routes to a given actor already carries that SAME
    /// lane — this recovers the classification `pump_primed`'s two priority queues need WITHOUT a
    /// breaking `ShardFrame::Grant` wire change (see [`Self::actor_lane`]'s own doc for why no
    /// field was added). Read by [`Self::actor_lane`]; an actor with no entry (never seen an
    /// envelope) falls back to Maintenance — same fallback convention as [`Self::granted_budget`].
    actor_lanes: HashMap<u64, semio_framework_actor::Lane>,
    pending_interactive: FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    pending_background: FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    rejected_frame: Option<(u64, Vec<u8>)>,
    terminal_frames: FixedOwnerRing<Vec<u8>, SHARD_DEFERRED_ITEMS>,
    terminal_frame_overflow: FixedOwnerRing<TerminalFrameOverflow, 1>,
    next_frame_epoch: u64,
    last_drive_consumed_epoch: Option<u64>,
}

#[derive(Clone, Copy)]
struct JobAuthority {
    turn: JobTurn,
    request: JobReplayRequest,
}

//#region 📄️FixedReplaySeed
const JOB_REPLAY_SEED_PAGE_BYTES: usize = semio_framework_actor::JOB_REPLAY_PAGE_BYTES;
const JOB_REPLAY_KIND_PAGE_CAPACITY: usize = 16;
const JOB_REPLAY_INPUT_PAGE_CAPACITY: usize = semio_framework_actor::JOB_REPLAY_RECORD_CAPACITY;
const JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY: usize = semio_framework_actor::JOB_REPLAY_RECORD_CAPACITY;
const JOB_REPLAY_SEED_SLOT_CAPACITY: usize = SHARD_DEFERRED_ITEMS;
const JOB_REPLAY_REFUSAL_SLOT_CAPACITY: usize = 512;
const JOB_REPLAY_SEED_PROCESS_PAGES: usize = semio_framework_actor::JOB_REPLAY_PROCESS_PAGE_CAPACITY;

static JOB_REPLAY_SEED_PAGES: AtomicUsize = AtomicUsize::new(0);
static JOB_REPLAY_ABI_BYTES: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static REPLAY_TEST_AUTHORITY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(test)]
pub(super) struct ReplayTestAuthority;

#[cfg(test)]
pub(super) fn replay_test_authority() -> ReplayTestAuthority {
    while REPLAY_TEST_AUTHORITY.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        std::thread::yield_now();
    }
    ReplayTestAuthority
}

#[cfg(test)]
impl Drop for ReplayTestAuthority {
    fn drop(&mut self) {
        REPLAY_TEST_AUTHORITY.store(false, Ordering::Release);
    }
}

struct FixedReplaySeedPage {
    storage: Option<Box<[MaybeUninit<u8>; JOB_REPLAY_SEED_PAGE_BYTES]>>,
    length: usize,
}

impl FixedReplaySeedPage {
    fn try_copy(bytes: &[u8]) -> Result<Self, ()> {
        if bytes.len() > JOB_REPLAY_SEED_PAGE_BYTES {
            return Err(());
        }
        JOB_REPLAY_SEED_PAGES.try_update(Ordering::AcqRel, Ordering::Acquire, |pages| pages.checked_add(1).filter(|pages| *pages <= JOB_REPLAY_SEED_PROCESS_PAGES)).map_err(|_| ())?;
        let mut storage = Box::new([MaybeUninit::uninit(); JOB_REPLAY_SEED_PAGE_BYTES]);
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), storage.as_mut_ptr().cast::<u8>(), bytes.len()) };
        Ok(Self { storage: Some(storage), length: bytes.len() })
    }

    fn bytes(&self) -> &[u8] {
        let storage = self.storage.as_ref().expect("fixed replay seed page owns backing");
        unsafe { std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), self.length) }
    }
}

impl Drop for FixedReplaySeedPage {
    fn drop(&mut self) {
        if self.storage.take().is_some() {
            JOB_REPLAY_SEED_PAGES.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

struct FixedReplaySeed {
    request: JobReplayRequest,
    kind: [Option<FixedReplaySeedPage>; JOB_REPLAY_KIND_PAGE_CAPACITY],
    input: [Option<FixedReplaySeedPage>; JOB_REPLAY_INPUT_PAGE_CAPACITY],
    checkpoint: [Option<FixedReplaySeedPage>; JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY],
    kind_pages: usize,
    input_pages: usize,
    checkpoint_pages: usize,
    kind_length: usize,
    input_length: usize,
    checkpoint_length: usize,
    close_cursor: usize,
}

impl FixedReplaySeed {
    fn new(request: JobReplayRequest, kind_length: usize, input_length: usize) -> Result<Self, ()> {
        if kind_length.div_ceil(JOB_REPLAY_SEED_PAGE_BYTES) > JOB_REPLAY_KIND_PAGE_CAPACITY || input_length.div_ceil(JOB_REPLAY_SEED_PAGE_BYTES) > JOB_REPLAY_INPUT_PAGE_CAPACITY {
            return Err(());
        }
        Ok(Self {
            request,
            kind: std::array::from_fn(|_| None),
            input: std::array::from_fn(|_| None),
            checkpoint: std::array::from_fn(|_| None),
            kind_pages: 0,
            input_pages: 0,
            checkpoint_pages: 0,
            kind_length,
            input_length,
            checkpoint_length: 0,
            close_cursor: 0,
        })
    }

    fn copy_kind_page(&mut self, bytes: &[u8], cursor: &mut usize) -> Result<bool, ()> {
        if *cursor == bytes.len() {
            return Ok(true);
        }
        let end = cursor.checked_add(JOB_REPLAY_SEED_PAGE_BYTES).unwrap_or(bytes.len()).min(bytes.len());
        let page = FixedReplaySeedPage::try_copy(&bytes[*cursor..end])?;
        let slot = self.kind.get_mut(self.kind_pages).ok_or(())?;
        *slot = Some(page);
        self.kind_pages += 1;
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    fn copy_input_page(&mut self, bytes: &[u8], cursor: &mut usize) -> Result<bool, ()> {
        if *cursor == bytes.len() {
            return Ok(true);
        }
        let end = cursor.checked_add(JOB_REPLAY_SEED_PAGE_BYTES).unwrap_or(bytes.len()).min(bytes.len());
        let page = FixedReplaySeedPage::try_copy(&bytes[*cursor..end])?;
        let slot = self.input.get_mut(self.input_pages).ok_or(())?;
        *slot = Some(page);
        self.input_pages += 1;
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    fn copy_checkpoint_page(&mut self, bytes: &[u8], cursor: &mut usize) -> Result<bool, ()> {
        if bytes.len().div_ceil(JOB_REPLAY_SEED_PAGE_BYTES) > JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY {
            return Err(());
        }
        self.checkpoint_length = bytes.len();
        if *cursor == bytes.len() {
            return Ok(true);
        }
        let end = cursor.checked_add(JOB_REPLAY_SEED_PAGE_BYTES).unwrap_or(bytes.len()).min(bytes.len());
        let page = FixedReplaySeedPage::try_copy(&bytes[*cursor..end])?;
        let slot = self.checkpoint.get_mut(self.checkpoint_pages).ok_or(())?;
        *slot = Some(page);
        self.checkpoint_pages += 1;
        *cursor = end;
        Ok(*cursor == bytes.len())
    }

    fn close_one(&mut self) -> bool {
        let total = JOB_REPLAY_KIND_PAGE_CAPACITY + JOB_REPLAY_INPUT_PAGE_CAPACITY + JOB_REPLAY_CHECKPOINT_PAGE_CAPACITY;
        for offset in 0..total {
            let index = (self.close_cursor + offset) % total;
            let page = if index < JOB_REPLAY_KIND_PAGE_CAPACITY {
                self.kind[index].take()
            } else if index < JOB_REPLAY_KIND_PAGE_CAPACITY + JOB_REPLAY_INPUT_PAGE_CAPACITY {
                self.input[index - JOB_REPLAY_KIND_PAGE_CAPACITY].take()
            } else {
                self.checkpoint[index - JOB_REPLAY_KIND_PAGE_CAPACITY - JOB_REPLAY_INPUT_PAGE_CAPACITY].take()
            };
            let Some(page) = page else { continue };
            drop(page);
            if index < JOB_REPLAY_KIND_PAGE_CAPACITY {
                self.kind_pages -= 1;
            } else if index < JOB_REPLAY_KIND_PAGE_CAPACITY + JOB_REPLAY_INPUT_PAGE_CAPACITY {
                self.input_pages -= 1;
            } else {
                self.checkpoint_pages -= 1;
            }
            self.close_cursor = (index + 1) % total;
            return false;
        }
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReplaySeedPhase {
    CaptureKind,
    CaptureInput,
    Checkpoint,
    CaptureCheckpoint,
    RetireCheckpointOwner,
    Start,
    RetireKindOwner,
    ActivateAuthority,
    ActivateTurn,
    ActivateRunning,
    ActivatePlacement,
    ActivateReady,
    Retained,
    MaterializeKind,
    MaterializeInput,
    MaterializeCheckpoint,
    Restore,
    RetireMaterializedCheckpoint,
    PrepareReplayKind,
    Restart,
    RetireReplayKind,
    Closing,
    RetireSeedShell,
    RetireMountedShell,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ReplaySeedCloseReason {
    Completed,
    Cancelled,
    ActorLost,
    Fault { stage: &'static str, detail: String },
}

struct MountedReplaySeed {
    actor: u64,
    job: u64,
    authority: JobTurn,
    placement: JobPlacement,
    worker_count: u16,
    worker_slot: u16,
    phase: ReplaySeedPhase,
    close_reason: Option<ReplaySeedCloseReason>,
    seed: Option<FixedReplaySeed>,
    kind_owner: Option<String>,
    input_owner: Option<Vec<u8>>,
    checkpoint_owner: Option<Vec<u8>>,
    kind_cursor: usize,
    input_cursor: usize,
    checkpoint_cursor: usize,
    materialized_kind: Option<Vec<u8>>,
    materialized_input: Option<Vec<u8>>,
    materialized_checkpoint: Option<Vec<u8>>,
    replay_kind_owner: Option<String>,
    materialize_page: usize,
    abi_reserved: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReplaySpawnRefusalPhase {
    RetireInput,
    RetireKind,
    Publish,
    RetireShell,
}

struct ReplaySpawnRefusal {
    actor: u64,
    job: u64,
    reason: &'static [u8],
    input: Option<Vec<u8>>,
    kind: Option<String>,
    phase: ReplaySpawnRefusalPhase,
    publish: bool,
}

impl ReplaySpawnRefusal {
    fn new(actor: u64, job: u64, reason: &'static [u8], kind: String, input: Vec<u8>) -> Self {
        Self { actor, job, reason, input: Some(input), kind: Some(kind), phase: ReplaySpawnRefusalPhase::RetireInput, publish: true }
    }
}

impl MountedReplaySeed {
    fn new(actor: u64, job: u64, authority: JobTurn, request: JobReplayRequest, placement: JobPlacement, kind: String, input: Vec<u8>) -> Result<Self, (String, Vec<u8>)> {
        let seed = match FixedReplaySeed::new(request, kind.len(), input.len()) {
            Ok(seed) => seed,
            Err(()) => return Err((kind, input)),
        };
        Ok(Self {
            actor,
            job,
            authority,
            placement,
            worker_count: 0,
            worker_slot: 0,
            phase: ReplaySeedPhase::CaptureKind,
            close_reason: None,
            seed: Some(seed),
            kind_owner: Some(kind),
            input_owner: Some(input),
            checkpoint_owner: None,
            kind_cursor: 0,
            input_cursor: 0,
            checkpoint_cursor: 0,
            materialized_kind: None,
            materialized_input: None,
            materialized_checkpoint: None,
            replay_kind_owner: None,
            materialize_page: 0,
            abi_reserved: 0,
        })
    }

    fn release_abi(&mut self, bytes: usize) -> Result<(), PluginHostError> {
        let remaining = self.abi_reserved.checked_sub(bytes).ok_or_else(|| PluginHostError::Plugin("ShardLoop::replay: local ABI accounting underflow".into()))?;
        JOB_REPLAY_ABI_BYTES.try_update(Ordering::AcqRel, Ordering::Acquire, |owned| owned.checked_sub(bytes)).map_err(|_| PluginHostError::Plugin("ShardLoop::replay: process ABI accounting underflow".into()))?;
        self.abi_reserved = remaining;
        Ok(())
    }
}

impl Drop for MountedReplaySeed {
    fn drop(&mut self) {
        let bytes = std::mem::take(&mut self.abi_reserved);
        if bytes != 0 {
            JOB_REPLAY_ABI_BYTES.fetch_sub(bytes, Ordering::AcqRel);
        }
    }
}

fn try_replay_abi_buffer(bytes: usize) -> Result<Vec<u8>, ()> {
    JOB_REPLAY_ABI_BYTES.try_update(Ordering::AcqRel, Ordering::Acquire, |owned| owned.checked_add(bytes).filter(|owned| *owned <= SHARD_DEFERRED_BYTES)).map_err(|_| ())?;
    let mut buffer = Vec::new();
    if buffer.try_reserve_exact(bytes).is_err() {
        JOB_REPLAY_ABI_BYTES.fetch_sub(bytes, Ordering::AcqRel);
        return Err(());
    }
    Ok(buffer)
}
//#endregion 📄️FixedReplaySeed

pub enum ShardDrive {
    Idle { consumed_epoch: Option<u64> },
    MoreWork { consumed_epoch: Option<u64> },
    Blocked,
    Fault { error: PluginHostError, consumed_epoch: Option<u64>, work_remains: bool, terminal_frame: bool, terminal_overflow: bool },
}

//#region 🚦DeferredOwnerRing
pub(super) const SHARD_DEFERRED_ITEMS: usize = 256;
pub(super) const SHARD_DEFERRED_BYTES: usize = 16 * 1024 * 1024;
pub(super) const SHARD_FRAME_MAX_BYTES: usize = SHARD_DEFERRED_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct OwnerKey {
    slot: usize,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmissionLimit {
    Items,
    Bytes,
}

#[derive(Debug)]
pub(super) struct AdmissionRejected<T> {
    pub limit: AdmissionLimit,
    pub owner: T,
}

struct OwnerSlot<T> {
    generation: u64,
    bytes: usize,
    owner: T,
}

pub(super) struct FixedOwnerRing<T, const N: usize> {
    slots: Box<[Option<OwnerSlot<T>>]>,
    order: Box<[usize]>,
    tail: usize,
    len: usize,
    bytes: usize,
    byte_capacity: usize,
    next_generation: u64,
}

impl<T, const N: usize> FixedOwnerRing<T, N> {
    pub fn new(byte_capacity: usize) -> Self {
        Self { slots: std::iter::repeat_with(|| None).take(N).collect(), order: vec![0; N].into_boxed_slice(), tail: 0, len: 0, bytes: 0, byte_capacity, next_generation: 1 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.len
    }

    pub fn can_admit(&self, items: usize, bytes: usize) -> Result<(), AdmissionLimit> {
        if items > N.saturating_sub(self.len) {
            return Err(AdmissionLimit::Items);
        }
        if bytes > self.byte_capacity.saturating_sub(self.bytes) {
            return Err(AdmissionLimit::Bytes);
        }
        Ok(())
    }

    pub fn try_push(&mut self, owner: T, bytes: usize) -> Result<OwnerKey, AdmissionRejected<T>> {
        if self.len == N {
            return Err(AdmissionRejected { limit: AdmissionLimit::Items, owner });
        }
        if bytes > self.byte_capacity.saturating_sub(self.bytes) {
            return Err(AdmissionRejected { limit: AdmissionLimit::Bytes, owner });
        }
        let generation = self.next_generation;
        self.next_generation = self.next_generation.wrapping_add(1).max(1);
        let slot = (0..N).map(|offset| (self.tail + offset) % N).find(|slot| self.slots[*slot].is_none()).expect("FixedOwnerRing: admitted item has one empty physical slot");
        debug_assert!(self.slots[slot].is_none());
        self.slots[slot] = Some(OwnerSlot { generation, bytes, owner });
        self.order[self.len] = slot;
        self.tail = (slot + 1) % N;
        self.len += 1;
        self.bytes += bytes;
        Ok(OwnerKey { slot, generation })
    }

    pub fn pop_front(&mut self) -> Option<(OwnerKey, T)> {
        self.pop_at(0)
    }

    fn get(&self, offset: usize) -> Option<&T> {
        if offset >= self.len {
            return None;
        }
        self.slots[self.order[offset]].as_ref().map(|slot| &slot.owner)
    }

    fn pop_at(&mut self, offset: usize) -> Option<(OwnerKey, T)> {
        if self.len == 0 {
            return None;
        }
        if offset >= self.len {
            return None;
        }
        let slot = self.order[offset];
        let entry = self.slots[slot].take().expect("FixedOwnerRing: occupied head invariant");
        self.order.copy_within(offset + 1..self.len, offset);
        self.len -= 1;
        self.bytes -= entry.bytes;
        Some((OwnerKey { slot, generation: entry.generation }, entry.owner))
    }

    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    #[cfg(test)]
    fn contains(&self, key: OwnerKey) -> bool {
        self.slots.get(key.slot).and_then(Option::as_ref).is_some_and(|slot| slot.generation == key.generation)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CancelCursor {
    actor: u64,
    after_job: Option<u64>,
    owner_bytes: usize,
}

#[derive(Debug)]
#[expect(clippy::large_enum_variant, reason = "The fixed owner ring retains complete event authority in preallocated slots through admission, cancellation, and retirement.")]
pub enum DeferredAuthority {
    Register { actor: ActorId },
    Unregister { actor: ActorId },
    Event { actor: u64, event: Event },
    JobStep { actor: u64, turn: JobTurn },
    JobReplay { actor: u64, turn: JobTurn, request: JobReplayRequest, worker_count: u16, worker_slot: u16 },
    Cancel(CancelCursor),
    Suspend { actor: u64, operation: JobOperation, applied_progress: u64 },
    Resume { actor: u64, operation: JobOperation, checkpoint: JobCheckpoint },
}

fn split_frame_credit(raw_bytes: usize, items: usize, index: usize) -> usize {
    if items == 0 {
        return 0;
    }
    raw_bytes / items + usize::from(index < raw_bytes % items)
}

enum FrameAdmissionError {
    Full { bytes: Vec<u8> },
    TerminalCapacity { bytes: Vec<u8>, error: PluginHostError },
    Fault(PluginHostError),
}

#[derive(Debug)]
struct TerminalFrameOverflow {
    epoch: u64,
    bytes: Vec<u8>,
}

fn defer_completion(
    interactive: &mut FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    background: &mut FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    terminal: &mut FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    lane: semio_framework_actor::Lane,
    allocation: Option<ShardActorAllocation>,
    actor: u64,
    event: Event,
) -> Result<(), PluginHostError> {
    let bytes = size_of::<Event>()
        + match &event {
            Event::JobCompleted { result: RequestOutcome::Ok(bytes) | RequestOutcome::Err(bytes), .. } => bytes.capacity(),
            _ => unreachable!("ShardLoop: only job completions use generated authority admission"),
        };
    let ring = if ShardLoop::is_high_priority_lane(lane) { interactive } else { background };
    let owner = AdmittedAuthority::new(allocation, semio_framework_actor::lane_defaults::budget_for(lane), DeferredAuthority::Event { actor, event }, lane, bytes);
    match ring.try_push(owner, bytes) {
        Ok(_) => Ok(()),
        Err(rejected) => {
            let _ = terminal.try_push(rejected.owner.authority, bytes).expect("ShardLoop: terminal completion ring owns every rejected completion");
            Err(PluginHostError::Plugin(format!("ShardLoop: completion {:?} capacity retained one terminal event for actor {actor}", rejected.limit)))
        }
    }
}
//#endregion 🚦DeferredOwnerRing

impl ShardLoop {
    pub async fn new(runtime: Arc<GuestRuntimes>, transport: ShardTransports) -> Self {
        Self {
            runtime,
            transport,
            instances: HashMap::new(),
            allocations: HashMap::new(),
            next_registration: 1,
            running_jobs: BTreeSet::new(),
            job_turns: HashMap::new(),
            job_authorities: HashMap::new(),
            replay_seeds: std::iter::repeat_with(|| None).take(JOB_REPLAY_SEED_SLOT_CAPACITY).collect(),
            replay_seed_refusals: std::iter::repeat_with(|| None).take(JOB_REPLAY_REFUSAL_SLOT_CAPACITY).collect(),
            replay_seed_cursor: 0,
            next_job_operation: 1,
            job_placement: HashMap::new(),
            terminal_authorities: FixedOwnerRing::new(SHARD_DEFERRED_BYTES.saturating_mul(SHARD_DEFERRED_ITEMS)),
            granted_budgets: HashMap::new(),
            actor_lanes: HashMap::new(),
            pending_interactive: FixedOwnerRing::new(SHARD_DEFERRED_BYTES),
            pending_background: FixedOwnerRing::new(SHARD_DEFERRED_BYTES),
            rejected_frame: None,
            terminal_frames: FixedOwnerRing::new(SHARD_FRAME_MAX_BYTES.saturating_mul(SHARD_DEFERRED_ITEMS)),
            terminal_frame_overflow: FixedOwnerRing::new(usize::MAX),
            next_frame_epoch: 1,
            last_drive_consumed_epoch: None,
        }
    }

    /// ⚖️ `actor`'s last [`ShardFrame::Grant`]ed budget — used for both turn execution and job
    /// stepping (point 2 of the packet brief: "job steps take the owning actor's last granted
    /// budget on the Maintenance lane"). Falls back to `lane_defaults::budget_for(Lane::
    /// Maintenance)` — a real, already-designed floor from the actor crate's own vocabulary, not
    /// an invented magic constant — for an actor that has never been granted a budget at all (e.g.
    /// a standalone `ShardFrame::Envelope` arriving before any `Grant`, or a caller like the
    /// `semio-shard` `[[bin]]` that does not yet send `Grant` frames at all).
    fn granted_budget(&self, actor: u64) -> semio_framework_actor::Budget {
        match self.granted_budgets.get(&actor).copied() {
            Some(budget) => budget,
            None => semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance),
        }
    }

    /// 🚦 terra-shard-lane piece 1: true for the two lanes that must jump a shard's queue ahead of
    /// any Background/Maintenance grant — Interactive (direct user input) and UserVisible
    /// (visible-but-not-actively-touched UI). `pump_primed`'s two queues partition on this.
    fn is_high_priority_lane(lane: semio_framework_actor::Lane) -> bool {
        matches!(lane, semio_framework_actor::Lane::Interactive | semio_framework_actor::Lane::UserVisible)
    }

    /// 🚦 `actor`'s last-known scheduling lane — see [`Self::actor_lanes`]'s own doc for why this
    /// is recovered from envelopes already on the wire rather than a new `ShardFrame::Grant` field:
    /// a `Grant`-level `lane` field would have needed to break TWO `ShardFrame::Grant` construction
    /// sites outside this packet's `path_scope` (`💻️os/🖥️host/🎠️activation/🦀️.rs`'s
    /// `NativeKernelRuntime::tick_and_dispatch` and `📺️renderer/🧑‍🎨engine/…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/
    /// 🎠️runtime.rs`'s equivalent dispatch loop — both live, both construct `ShardFrame::Grant`
    /// directly, neither is `🔌️plugin/🖥️host/**`), and the information a `Grant`-level field would
    /// have carried is ALREADY present per-envelope (`Envelope.lane`, set once per actor by the
    /// same DRR `Scheduler` that would have supplied a `Grant`-level lane). See
    /// `📓️terra-shard-lane-report.md`'s "wire change avoided" note — a `lease-request` is open
    /// against those two files in case a `Grant`-level field is still wanted for a future packet.
    fn actor_lane(&self, actor: u64) -> semio_framework_actor::Lane {
        self.actor_lanes.get(&actor).copied().unwrap_or(semio_framework_actor::Lane::Maintenance)
    }

    #[cfg(test)]
    fn pop_next_authority(ring: &mut FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>, placements: &HashMap<(u64, u64), JobPlacement>) -> Option<(OwnerKey, DeferredAuthority)> {
        let actor = match ring.get(0) {
            Some(DeferredAuthority::JobStep { actor, .. }) => *actor,
            _ => return ring.pop_front(),
        };
        let mut selected = 0;
        for offset in 0..ring.len {
            let Some(DeferredAuthority::JobStep { actor: candidate, turn }) = ring.get(offset) else { break };
            if *candidate != actor {
                break;
            }
            if placements.get(&(actor, turn.job)) == Some(&JobPlacement::Exclusive) {
                selected = offset;
                break;
            }
        }
        ring.pop_at(selected)
    }

    fn consume_replay_opportunity(&mut self, actor: u64) -> bool {
        if !self.actor_generation_is_current(ActorId(actor)) {
            return false;
        }
        let budget = self.granted_budgets.entry(actor).or_insert_with(|| semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance));
        if budget.fuel == 0 || budget.wall_ms == 0 {
            return false;
        }
        budget.fuel -= 1;
        true
    }

    fn consume_replay_close_opportunity(&mut self, actor: u64) -> bool {
        let budget = self.granted_budgets.entry(actor).or_insert_with(|| semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance));
        if budget.fuel == 0 || budget.wall_ms == 0 {
            return false;
        }
        budget.fuel -= 1;
        true
    }

    async fn drive_replay_refusal(&mut self) -> Result<bool, PluginHostError> {
        let Some(index) = self.replay_seed_refusals.iter().position(Option::is_some) else {
            return Ok(false);
        };
        let actor = self.replay_seed_refusals[index].as_ref().expect("selected replay refusal").actor;
        if !self.consume_replay_close_opportunity(actor) {
            return Ok(false);
        }
        let refusal = self.replay_seed_refusals[index].as_mut().expect("selected replay refusal");
        match refusal.phase {
            ReplaySpawnRefusalPhase::RetireInput => {
                drop(refusal.input.take().expect("rejected input closes once"));
                refusal.phase = ReplaySpawnRefusalPhase::RetireKind;
            }
            ReplaySpawnRefusalPhase::RetireKind => {
                drop(refusal.kind.take().expect("rejected kind closes once"));
                refusal.phase = ReplaySpawnRefusalPhase::Publish;
            }
            ReplaySpawnRefusalPhase::Publish => {
                let actor = refusal.actor;
                let job = refusal.job;
                let reason = refusal.reason.to_vec();
                let publish = refusal.publish;
                refusal.phase = ReplaySpawnRefusalPhase::RetireShell;
                if publish {
                    let lane = self.actor_lane(actor);
                    defer_completion(&mut self.pending_interactive, &mut self.pending_background, &mut self.terminal_authorities, lane, self.allocations.get(&actor).copied(), actor, Event::JobCompleted { job, result: RequestOutcome::Err(reason) })?;
                }
            }
            ReplaySpawnRefusalPhase::RetireShell => {
                let refusal = self.replay_seed_refusals[index].take().expect("terminal replay refusal");
                drop(refusal);
            }
        }
        Ok(true)
    }

    fn validate_replay_request(&self, actor: u64, turn: JobTurn, request: JobReplayRequest) -> Result<(), PluginHostError> {
        let Some(seed) = self.replay_seeds.iter().flatten().find(|seed| seed.actor == actor && seed.job == turn.job) else {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: no retained seed for actor {actor}, job {}", turn.job)));
        };
        let retained = seed.seed.as_ref().ok_or_else(|| PluginHostError::Plugin(format!("ShardLoop::replay: seed for actor {actor}, job {} is closing", turn.job)))?;
        if seed.authority.operation.operation != turn.operation.operation
            || seed.authority.operation.base_revision != turn.operation.base_revision
            || seed.authority.operation.generation != turn.operation.generation
            || seed.authority.operation.seed != turn.operation.seed
            || turn.step_sequence != 0
            || retained.request != request
        {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: route, seed, generation, or first ordinal mismatch for actor {actor}, job {}", turn.job)));
        }
        Ok(())
    }

    fn begin_replay_seed(&mut self, actor: u64, turn: JobTurn, request: JobReplayRequest, worker_count: u16, worker_slot: u16) -> Result<(), PluginHostError> {
        self.validate_replay_request(actor, turn, request)?;
        if worker_count == 0 || worker_slot == u16::MAX {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: invalid worker identity for actor {actor}, job {}", turn.job)));
        }
        let seed = self.replay_seeds.iter_mut().flatten().find(|seed| seed.actor == actor && seed.job == turn.job).expect("validated replay seed remains mounted");
        if !matches!(seed.phase, ReplaySeedPhase::Retained) {
            return Err(PluginHostError::Plugin(format!("ShardLoop::replay: seed for actor {actor}, job {} is busy", turn.job)));
        }
        seed.authority = turn;
        seed.worker_count = worker_count;
        seed.worker_slot = worker_slot;
        seed.kind_cursor = 0;
        seed.input_cursor = 0;
        seed.checkpoint_cursor = 0;
        seed.materialize_page = 0;
        seed.phase = ReplaySeedPhase::MaterializeKind;
        Ok(())
    }

    fn begin_replay_seed_close(&mut self, index: usize, reason: ReplaySeedCloseReason) {
        Self::begin_replay_seed_close_owned(&mut self.replay_seeds, &mut self.running_jobs, &mut self.job_turns, &mut self.job_authorities, &mut self.job_placement, index, reason);
    }

    fn begin_replay_seed_close_owned(
        replay_seeds: &mut [Option<MountedReplaySeed>],
        running_jobs: &mut BTreeSet<(u64, u64)>,
        job_turns: &mut HashMap<(u64, u64), JobTurn>,
        job_authorities: &mut HashMap<(u64, u64), JobAuthority>,
        job_placement: &mut HashMap<(u64, u64), JobPlacement>,
        index: usize,
        reason: ReplaySeedCloseReason,
    ) {
        let Some(seed) = replay_seeds[index].as_mut() else { return };
        if seed.close_reason.is_none() {
            seed.close_reason = Some(reason);
        }
        if !matches!(seed.phase, ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell) {
            seed.phase = ReplaySeedPhase::Closing;
        }
        let key = (seed.actor, seed.job);
        running_jobs.remove(&key);
        job_turns.remove(&key);
        job_authorities.remove(&key);
        job_placement.remove(&key);
    }

    fn fail_replay_seed(&mut self, index: usize, stage: &'static str, detail: String) -> PluginHostError {
        self.begin_replay_seed_close(index, ReplaySeedCloseReason::Fault { stage, detail: detail.clone() });
        PluginHostError::Plugin(detail)
    }

    fn close_replay_job(&mut self, actor: u64, job: u64, reason: ReplaySeedCloseReason) -> bool {
        let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor && seed.job == job)) else { return false };
        self.begin_replay_seed_close(index, reason);
        true
    }

    fn retire_actor_replay_owners(&mut self, actor: u64) {
        for index in 0..JOB_REPLAY_SEED_SLOT_CAPACITY {
            if self.replay_seeds[index].as_ref().is_some_and(|seed| seed.actor == actor) {
                self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
            }
        }
        for refusal in self.replay_seed_refusals.iter_mut().flatten().filter(|refusal| refusal.actor == actor) {
            refusal.publish = false;
        }
    }

    fn close_replay_seed_one(replay_seeds: &mut [Option<MountedReplaySeed>], index: usize) -> Result<(), PluginHostError> {
        let phase = replay_seeds[index].as_ref().expect("closing replay seed").phase;
        match phase {
            ReplaySeedPhase::Closing => {
                let seed = replay_seeds[index].as_mut().expect("closing replay seed");
                if let Some(owner) = seed.replay_kind_owner.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").kind_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.materialized_checkpoint.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").checkpoint_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.materialized_input.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").input_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.materialized_kind.take() {
                    let capacity = seed.seed.as_ref().expect("closing fixed seed").kind_length;
                    drop(owner);
                    seed.release_abi(capacity)?;
                } else if let Some(owner) = seed.checkpoint_owner.take() {
                    drop(owner);
                } else if let Some(owner) = seed.input_owner.take() {
                    drop(owner);
                } else if let Some(owner) = seed.kind_owner.take() {
                    drop(owner);
                } else if seed.seed.as_mut().expect("closing fixed seed").close_one() {
                    seed.phase = ReplaySeedPhase::RetireSeedShell;
                }
            }
            ReplaySeedPhase::RetireSeedShell => {
                let seed = replay_seeds[index].as_mut().expect("terminal replay seed");
                drop(seed.seed.take().expect("terminal fixed seed"));
                seed.phase = ReplaySeedPhase::RetireMountedShell;
            }
            ReplaySeedPhase::RetireMountedShell => {
                drop(replay_seeds[index].take().expect("terminal replay seed"));
            }
            ReplaySeedPhase::Retained => replay_seeds[index].as_mut().expect("stale retained replay seed").phase = ReplaySeedPhase::Closing,
            _ => unreachable!("replay close opportunity requires a closing phase"),
        }
        Ok(())
    }

    async fn drive_replay_seed(&mut self) -> Result<bool, PluginHostError> {
        let Some(index) = (0..JOB_REPLAY_SEED_SLOT_CAPACITY)
            .map(|offset| (self.replay_seed_cursor + offset) % JOB_REPLAY_SEED_SLOT_CAPACITY)
            .find(|index| self.replay_seeds[*index].as_ref().is_some_and(|seed| !matches!(seed.phase, ReplaySeedPhase::Retained) || !self.instances.contains_key(&seed.actor)))
        else {
            return Ok(false);
        };
        let actor = self.replay_seeds[index].as_ref().expect("selected replay seed").actor;
        let closing = self.replay_seeds[index].as_ref().is_some_and(|seed| matches!(seed.phase, ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell) || !self.instances.contains_key(&seed.actor));
        if if closing { !self.consume_replay_close_opportunity(actor) } else { !self.consume_replay_opportunity(actor) } {
            return Ok(false);
        }
        self.replay_seed_cursor = (index + 1) % JOB_REPLAY_SEED_SLOT_CAPACITY;
        let phase = self.replay_seeds[index].as_ref().expect("selected replay seed").phase;
        if !self.instances.contains_key(&actor) && !matches!(phase, ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell) {
            self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
            return Ok(true);
        }
        match phase {
            ReplaySeedPhase::CaptureKind => {
                let copied = {
                    let seed = self.replay_seeds[index].as_mut().expect("capture seed");
                    seed.seed.as_mut().expect("capture fixed seed").copy_kind_page(seed.kind_owner.as_ref().expect("capture kind").as_bytes(), &mut seed.kind_cursor)
                };
                match copied {
                    Ok(true) => self.replay_seeds[index].as_mut().expect("capture seed").phase = ReplaySeedPhase::CaptureInput,
                    Ok(false) => {}
                    Err(()) => return Err(self.fail_replay_seed(index, "capture-kind", format!("ShardLoop::replay: kind page admission refused for actor {actor}"))),
                }
            }
            ReplaySeedPhase::CaptureInput => {
                let copied = {
                    let seed = self.replay_seeds[index].as_mut().expect("capture seed");
                    seed.seed.as_mut().expect("capture fixed seed").copy_input_page(seed.input_owner.as_ref().expect("capture input"), &mut seed.input_cursor)
                };
                match copied {
                    Ok(true) => self.replay_seeds[index].as_mut().expect("capture seed").phase = ReplaySeedPhase::Checkpoint,
                    Ok(false) => {}
                    Err(()) => return Err(self.fail_replay_seed(index, "capture-input", format!("ShardLoop::replay: input page admission refused for actor {actor}"))),
                }
            }
            ReplaySeedPhase::Checkpoint => {
                let checkpoint = match self.instances.get_mut(&actor) {
                    Some(instance) => match self.runtime.checkpoint(instance).await {
                        Ok(checkpoint) => checkpoint,
                        Err(fault) => return Err(self.fail_replay_seed(index, "checkpoint", format!("ShardLoop::replay: checkpoint failed for actor {actor}: {fault}"))),
                    },
                    None => {
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                };
                let seed = self.replay_seeds[index].as_mut().expect("checkpoint seed");
                seed.checkpoint_owner = Some(checkpoint);
                seed.phase = ReplaySeedPhase::CaptureCheckpoint;
            }
            ReplaySeedPhase::CaptureCheckpoint => {
                let copied = {
                    let seed = self.replay_seeds[index].as_mut().expect("checkpoint seed");
                    seed.seed.as_mut().expect("capture fixed seed").copy_checkpoint_page(seed.checkpoint_owner.as_ref().expect("checkpoint owner"), &mut seed.checkpoint_cursor)
                };
                match copied {
                    Ok(true) => self.replay_seeds[index].as_mut().expect("checkpoint seed").phase = ReplaySeedPhase::RetireCheckpointOwner,
                    Ok(false) => {}
                    Err(()) => return Err(self.fail_replay_seed(index, "capture-checkpoint", format!("ShardLoop::replay: checkpoint page admission refused for actor {actor}"))),
                }
            }
            ReplaySeedPhase::RetireCheckpointOwner => {
                let seed = self.replay_seeds[index].as_mut().expect("checkpoint seed");
                drop(seed.checkpoint_owner.take().expect("captured checkpoint owner retires once"));
                seed.phase = ReplaySeedPhase::Start;
            }
            ReplaySeedPhase::Start => {
                let (job, kind, input) = {
                    let seed = self.replay_seeds[index].as_mut().expect("start seed");
                    (seed.job, seed.kind_owner.take().expect("original kind checked out once"), seed.input_owner.take().expect("original input transfers once"))
                };
                let result = match self.instances.get_mut(&actor) {
                    Some(instance) => self.runtime.start_job(instance, job, &kind, input).await,
                    None => {
                        let seed = self.replay_seeds[index].as_mut().expect("lost start seed");
                        seed.kind_owner = Some(kind);
                        seed.input_owner = Some(input);
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                };
                let seed = self.replay_seeds[index].as_mut().expect("started seed");
                seed.kind_owner = Some(kind);
                if let Err(fault) = result {
                    return Err(self.fail_replay_seed(index, "live-start", format!("ShardLoop::replay: live start failed for actor {actor}, job {job}: {}", turn_fault_message(&fault))));
                }
                seed.phase = ReplaySeedPhase::RetireKindOwner;
            }
            ReplaySeedPhase::RetireKindOwner => {
                let seed = self.replay_seeds[index].as_mut().expect("started seed");
                drop(seed.kind_owner.take().expect("fixed kind pages supersede original owner"));
                seed.phase = ReplaySeedPhase::ActivateAuthority;
            }
            ReplaySeedPhase::ActivateAuthority => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.job_authorities.insert((actor, seed.job), JobAuthority { turn: seed.authority, request: seed.seed.as_ref().expect("retained seed").request });
                seed.phase = ReplaySeedPhase::ActivateTurn;
            }
            ReplaySeedPhase::ActivateTurn => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.job_turns.insert((actor, seed.job), seed.authority);
                seed.phase = ReplaySeedPhase::ActivateRunning;
            }
            ReplaySeedPhase::ActivateRunning => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.running_jobs.insert((actor, seed.job));
                seed.phase = ReplaySeedPhase::ActivatePlacement;
            }
            ReplaySeedPhase::ActivatePlacement => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                self.job_placement.insert((actor, seed.job), seed.placement);
                seed.phase = ReplaySeedPhase::ActivateReady;
            }
            ReplaySeedPhase::ActivateReady => {
                let seed = self.replay_seeds[index].as_mut().expect("activate seed");
                let replay = seed.worker_count != 0;
                let authority = seed.authority;
                seed.phase = ReplaySeedPhase::Retained;
                if replay {
                    if let Err(error) = self.send_outcome(&ShardOutcome::Resumed { actor, operation: authority.operation }).await {
                        return Err(self.fail_replay_seed(index, "resume-publication", format!("ShardLoop::replay: resume publication failed for actor {actor}: {error}")));
                    }
                }
            }
            ReplaySeedPhase::MaterializeKind => {
                let seed = self.replay_seeds[index].as_mut().expect("materialize seed");
                let fixed = seed.seed.as_ref().expect("materialize fixed seed");
                match seed.materialized_kind.as_mut() {
                    None => {
                        let buffer = match try_replay_abi_buffer(fixed.kind_length) {
                            Ok(buffer) => buffer,
                            Err(()) => return Err(self.fail_replay_seed(index, "materialize-kind", "ShardLoop::replay: kind ABI admission refused".to_string())),
                        };
                        seed.abi_reserved += fixed.kind_length;
                        seed.materialized_kind = Some(buffer);
                    }
                    Some(buffer) if seed.materialize_page < fixed.kind_pages => {
                        let page = fixed.kind[seed.materialize_page].as_ref().expect("fixed kind page");
                        buffer.extend_from_slice(page.bytes());
                        seed.materialize_page += 1;
                    }
                    Some(_) => {
                        seed.materialize_page = 0;
                        seed.phase = ReplaySeedPhase::MaterializeInput;
                    }
                }
            }
            ReplaySeedPhase::MaterializeInput => {
                let seed = self.replay_seeds[index].as_mut().expect("materialize seed");
                let fixed = seed.seed.as_ref().expect("materialize fixed seed");
                match seed.materialized_input.as_mut() {
                    None => {
                        let buffer = match try_replay_abi_buffer(fixed.input_length) {
                            Ok(buffer) => buffer,
                            Err(()) => return Err(self.fail_replay_seed(index, "materialize-input", "ShardLoop::replay: input ABI admission refused".to_string())),
                        };
                        seed.abi_reserved += fixed.input_length;
                        seed.materialized_input = Some(buffer);
                    }
                    Some(buffer) if seed.materialize_page < fixed.input_pages => {
                        let page = fixed.input[seed.materialize_page].as_ref().expect("fixed input page");
                        buffer.extend_from_slice(page.bytes());
                        seed.materialize_page += 1;
                    }
                    Some(_) => {
                        seed.materialize_page = 0;
                        seed.phase = ReplaySeedPhase::MaterializeCheckpoint;
                    }
                }
            }
            ReplaySeedPhase::MaterializeCheckpoint => {
                let seed = self.replay_seeds[index].as_mut().expect("materialize seed");
                let fixed = seed.seed.as_ref().expect("materialize fixed seed");
                match seed.materialized_checkpoint.as_mut() {
                    None => {
                        let buffer = match try_replay_abi_buffer(fixed.checkpoint_length) {
                            Ok(buffer) => buffer,
                            Err(()) => return Err(self.fail_replay_seed(index, "materialize-checkpoint", "ShardLoop::replay: checkpoint ABI admission refused".to_string())),
                        };
                        seed.abi_reserved += fixed.checkpoint_length;
                        seed.materialized_checkpoint = Some(buffer);
                    }
                    Some(buffer) if seed.materialize_page < fixed.checkpoint_pages => {
                        let page = fixed.checkpoint[seed.materialize_page].as_ref().expect("fixed checkpoint page");
                        buffer.extend_from_slice(page.bytes());
                        seed.materialize_page += 1;
                    }
                    Some(_) => {
                        seed.materialize_page = 0;
                        seed.phase = ReplaySeedPhase::Restore;
                    }
                }
            }
            ReplaySeedPhase::Restore => {
                let checkpoint = self.replay_seeds[index].as_ref().expect("restore seed").materialized_checkpoint.as_ref().expect("materialized checkpoint");
                match self.instances.get_mut(&actor) {
                    Some(instance) => {
                        if let Err(fault) = self.runtime.restore(instance, checkpoint).await {
                            return Err(self.fail_replay_seed(index, "restore", format!("ShardLoop::replay: restore failed for actor {actor}: {fault}")));
                        }
                    }
                    None => {
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                }
                self.replay_seeds[index].as_mut().expect("restored seed").phase = ReplaySeedPhase::RetireMaterializedCheckpoint;
            }
            ReplaySeedPhase::RetireMaterializedCheckpoint => {
                let seed = self.replay_seeds[index].as_mut().expect("restored seed");
                drop(seed.materialized_checkpoint.take().expect("restored checkpoint retires once"));
                let bytes = seed.seed.as_ref().expect("retained seed").checkpoint_length;
                seed.release_abi(bytes)?;
                seed.phase = ReplaySeedPhase::PrepareReplayKind;
            }
            ReplaySeedPhase::PrepareReplayKind => {
                let (bytes, capacity) = {
                    let seed = self.replay_seeds[index].as_mut().expect("prepare replay kind");
                    (seed.materialized_kind.take().expect("materialized replay kind"), seed.seed.as_ref().expect("retained seed").kind_length)
                };
                match String::from_utf8(bytes) {
                    Ok(kind) => {
                        let seed = self.replay_seeds[index].as_mut().expect("prepared replay kind");
                        seed.replay_kind_owner = Some(kind);
                        seed.phase = ReplaySeedPhase::Restart;
                    }
                    Err(_) => {
                        let released = self.replay_seeds[index].as_mut().expect("invalid replay kind").release_abi(capacity);
                        if let Err(error) = released {
                            return Err(self.fail_replay_seed(index, "prepare-kind-accounting", error.to_string()));
                        }
                        return Err(self.fail_replay_seed(index, "prepare-kind", "ShardLoop::replay: retained kind is not UTF-8".to_string()));
                    }
                }
            }
            ReplaySeedPhase::Restart => {
                let (job, kind, input, input_bytes) = {
                    let seed = self.replay_seeds[index].as_mut().expect("restart seed");
                    let kind = seed.replay_kind_owner.take().expect("prepared replay kind checks out once");
                    let input = seed.materialized_input.take().expect("materialized replay input");
                    let input_bytes = seed.seed.as_ref().expect("retained seed").input_length;
                    (seed.job, kind, input, input_bytes)
                };
                let result = match self.instances.get_mut(&actor) {
                    Some(instance) => self.runtime.start_job(instance, job, &kind, input).await,
                    None => {
                        let seed = self.replay_seeds[index].as_mut().expect("lost restart seed");
                        seed.replay_kind_owner = Some(kind);
                        seed.materialized_input = Some(input);
                        self.begin_replay_seed_close(index, ReplaySeedCloseReason::ActorLost);
                        return Ok(true);
                    }
                };
                let released = {
                    let seed = self.replay_seeds[index].as_mut().expect("restarted seed");
                    seed.replay_kind_owner = Some(kind);
                    seed.release_abi(input_bytes)
                };
                if let Err(fault) = result {
                    return Err(self.fail_replay_seed(index, "restart", format!("ShardLoop::replay: restart failed for actor {actor}, job {job}: {}", turn_fault_message(&fault))));
                }
                if let Err(error) = released {
                    return Err(self.fail_replay_seed(index, "restart-accounting", error.to_string()));
                }
                self.replay_seeds[index].as_mut().expect("restarted seed").phase = ReplaySeedPhase::RetireReplayKind;
            }
            ReplaySeedPhase::RetireReplayKind => {
                let seed = self.replay_seeds[index].as_mut().expect("restarted seed");
                drop(seed.replay_kind_owner.take().expect("fixed kind pages supersede replay ABI owner"));
                let bytes = seed.seed.as_ref().expect("retained seed").kind_length;
                seed.release_abi(bytes)?;
                seed.phase = ReplaySeedPhase::ActivateAuthority;
            }
            ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell | ReplaySeedPhase::Retained => Self::close_replay_seed_one(&mut self.replay_seeds, index)?,
        }
        Ok(true)
    }

    /// 📌️ Adds an already-instantiated actor to this shard's live set — called once per
    /// `Kernel::activate` that lands on this shard. `actor.0` (the bit-packed `u64`) is the map key
    /// throughout this type: `Envelope.to`/`ShardOutcome`'s tag both carry the SAME raw id, so no
    /// `RuntimeActorId` round-trip is needed at the boundary.
    pub async fn is_registered(&self, actor: ActorId) -> bool {
        self.instances.contains_key(&actor.0)
    }

    /// ✂️ Releases an actor's instance (generation change on restart, or a real unload) — calls
    /// [`super::GuestRuntime::drop_instance`] so the pooling allocator reclaims its slab.
    pub async fn unregister(&mut self, actor: ActorId) {
        self.retire_actor_replay_owners(actor.0);
        self.allocations.remove(&actor.0);
        self.granted_budgets.remove(&actor.0);
        if let Some(instance) = self.instances.remove(&actor.0) {
            self.runtime.drop_instance(instance).await;
        }
        self.running_jobs.retain(|&(job_actor, _)| job_actor != actor.0);
        self.job_turns.retain(|&(job_actor, _), _| job_actor != actor.0);
        self.job_authorities.retain(|&(job_actor, _), _| job_actor != actor.0);
        self.job_placement.retain(|&(job_actor, _), _| job_actor != actor.0);
        self.actor_lanes.remove(&actor.0);
    }

    pub async fn actor_count(&self) -> usize {
        self.instances.len()
    }

    /// 🌀️ Admits at most one [`ShardFrame`] currently buffered on the transport and grants
    /// exactly one deferred authority. Returns `1` when an authority or replay opportunity was
    /// handled and `0` for frame-only admission or idle. Equivalent to `self.pump_primed(None)`.
    pub async fn pump(&mut self) -> Result<usize, PluginHostError> {
        self.pump_primed(None).await
    }

    /// 🤝️ Admits at most one transport frame and grants exactly one actor turn or one job-step
    /// opportunity. All other decoded authorities remain owned by this shard for a later grant.
    pub async fn drive_one(&mut self) -> ShardDrive {
        self.drive_one_primed(None).await
    }

    pub(super) async fn drive_one_primed(&mut self, primed: Option<Vec<u8>>) -> ShardDrive {
        self.last_drive_consumed_epoch = None;
        match self.pump_primed(primed).await {
            Ok(_) if self.has_pending_work() => ShardDrive::MoreWork { consumed_epoch: self.last_drive_consumed_epoch },
            Ok(_) => ShardDrive::Idle { consumed_epoch: self.last_drive_consumed_epoch },
            Err(error) => {
                ShardDrive::Fault { error, consumed_epoch: self.last_drive_consumed_epoch, work_remains: self.has_pending_work(), terminal_frame: !self.terminal_frames.is_empty(), terminal_overflow: !self.terminal_frame_overflow.is_empty() }
            }
        }
    }

    pub(super) fn can_accept_primed_frame(&self) -> bool {
        !self.has_pending_work() || self.has_lifecycle_retry()
    }

    pub fn has_pending_work(&self) -> bool {
        !self.pending_interactive.is_empty()
            || !self.pending_background.is_empty()
            || self.rejected_frame.is_some()
            || self.replay_seed_refusals.iter().any(Option::is_some)
            || self.replay_seeds.iter().flatten().any(|seed| !matches!(seed.phase, ReplaySeedPhase::Retained) || !self.instances.contains_key(&seed.actor))
    }

    pub fn take_terminal_frame(&mut self) -> Option<Vec<u8>> {
        self.terminal_frames.pop_front().map(|(_, bytes)| bytes)
    }

    pub(super) fn take_terminal_frame_and_rearm(&mut self) -> (Option<Vec<u8>>, Option<u64>) {
        let frame = self.terminal_frames.pop_front().map(|(_, bytes)| bytes);
        if frame.is_none() {
            return (None, None);
        }
        let Some(overflow) = self.terminal_frame_overflow.front() else {
            return (frame, None);
        };
        if self.terminal_frames.can_admit(1, overflow.bytes.len()).is_err() {
            return (frame, None);
        }
        let (_, overflow) = self.terminal_frame_overflow.pop_front().expect("ShardLoop: terminal overflow front");
        let epoch = overflow.epoch;
        let byte_len = overflow.bytes.len();
        let _ = self.terminal_frames.try_push(overflow.bytes, byte_len).expect("ShardLoop: freed terminal capacity owns one overflow frame");
        (frame, Some(epoch))
    }

    pub fn take_terminal_completion(&mut self) -> Option<(u64, Event)> {
        if !matches!(self.terminal_authorities.front(), Some(DeferredAuthority::Event { .. })) {
            return None;
        }
        let (_, DeferredAuthority::Event { actor, event }) = self.terminal_authorities.pop_front().expect("ShardLoop: terminal completion front") else { unreachable!("ShardLoop: terminal completion kind changed") };
        Some((actor, event))
    }

    pub fn take_terminal_authority(&mut self) -> Option<DeferredAuthority> {
        self.terminal_authorities.pop_front().map(|(_, authority)| authority)
    }

    fn claim_frame_epoch(&mut self) -> u64 {
        let epoch = self.next_frame_epoch;
        self.next_frame_epoch = self.next_frame_epoch.checked_add(1).expect("ShardLoop: ingress epoch exhausted");
        epoch
    }

    /// 🅿️ Same as [`Self::pump`], but takes one frame's bytes that were ALREADY read off the
    /// transport (e.g. by `ShardExecutor`'s blocking park on `ThreadTransport::recv_deadline`)
    /// before the normal non-blocking drain loop continues — lets a blocking wait and this
    /// non-blocking drain share the exact same transport without losing whatever woke the wait.
    /// `primed: None` (what [`Self::pump`] passes) behaves identically to the pre-`ShardFrame`
    /// `pump()`.
    pub async fn pump_primed(&mut self, primed: Option<Vec<u8>>) -> Result<usize, PluginHostError> {
        let has_deferred = !self.pending_interactive.is_empty()
            || !self.pending_background.is_empty()
            || self.replay_seed_refusals.iter().any(Option::is_some)
            || self.replay_seeds.iter().flatten().any(|seed| !matches!(seed.phase, ReplaySeedPhase::Retained) || !self.instances.contains_key(&seed.actor));
        let mut frame = self.rejected_frame.take();
        if frame.is_none() {
            if let Some(bytes) = primed {
                frame = Some((self.claim_frame_epoch(), bytes));
            }
        }
        if frame.is_none() && (!has_deferred || self.has_lifecycle_retry()) {
            if let Some(bytes) = self.transport.recv().await {
                frame = Some((self.claim_frame_epoch(), bytes));
            }
        }
        if let Some((epoch, bytes)) = frame {
            if let Err(rejected) = self.consume_frame(bytes).await {
                match rejected {
                    FrameAdmissionError::Full { bytes } => self.rejected_frame = Some((epoch, bytes)),
                    FrameAdmissionError::TerminalCapacity { bytes, error } => {
                        let byte_len = bytes.len();
                        let _ = self.terminal_frame_overflow.try_push(TerminalFrameOverflow { epoch, bytes }, byte_len).expect("ShardLoop: one terminal overflow owner while drive is parked");
                        return Err(error);
                    }
                    FrameAdmissionError::Fault(error) => {
                        self.last_drive_consumed_epoch = Some(epoch);
                        return Err(error);
                    }
                }
            } else {
                self.last_drive_consumed_epoch = Some(epoch);
            }
        }

        if self.drive_replay_refusal().await? {
            return Ok(1);
        }
        let mut selected_step = None;
        if let Some(owner) = self.select_pending_authority() {
            if matches!(owner.authority, DeferredAuthority::Register { .. }) {
                return Ok(1);
            }
            if !self.allocation_is_current(owner.allocation) {
                if owner.allocation.is_none() && !matches!(owner.authority, DeferredAuthority::Unregister { .. }) {
                    self.send_outcome(&ShardOutcome::Fault { actor: owner.authority.actor(), message: "deferred authority has no registered actor allocation".into() }).await?;
                }
                return Ok(1);
            }
            let lane = owner.lane;
            if let DeferredAuthority::Event { actor, event } = &owner.authority {
                if self.execute_turn_for(*actor, event, owner.budget, lane).await? {
                    self.retain_lifecycle_retry(owner)?;
                }
                return Ok(1);
            }
            match owner.authority {
                DeferredAuthority::Register { .. } | DeferredAuthority::Event { .. } => unreachable!("selected non-event authority"),
                DeferredAuthority::Unregister { actor } => {
                    self.unregister(actor).await;
                    return Ok(1);
                }
                DeferredAuthority::JobStep { actor, turn } => {
                    self.accept_job_turn(actor, turn)?;
                    selected_step = Some((actor, turn));
                }
                DeferredAuthority::JobReplay { actor, turn, request, worker_count, worker_slot } => {
                    self.begin_replay_seed(actor, turn, request, worker_count, worker_slot)?;
                    return Ok(1);
                }
                DeferredAuthority::Cancel(cursor) => {
                    self.cancel_one(cursor, lane).await?;
                    return Ok(1);
                }
                DeferredAuthority::Suspend { actor, operation, applied_progress } => {
                    self.suspend_one(actor, operation, applied_progress).await?;
                    return Ok(1);
                }
                DeferredAuthority::Resume { actor, operation, checkpoint } => {
                    self.resume_one(actor, operation, checkpoint).await?;
                    return Ok(1);
                }
            }
        }
        if selected_step.is_none() && self.drive_replay_seed().await? {
            return Ok(1);
        }
        let selected_step = selected_step;
        if let Some((actor_id, turn)) = selected_step {
            let job = turn.job;
            let Some(authority) = self.job_authorities.get(&(actor_id, job)).copied() else {
                let message = format!("ShardLoop::pump: job {job} has no independently admitted operation authority");
                if !self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-authority", detail: message.clone() }) {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_turns.remove(&(actor_id, job));
                    self.job_placement.remove(&(actor_id, job));
                }
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(1);
            };
            let Some(placement) = self.job_placement.get(&(actor_id, job)).copied() else {
                let message = format!("ShardLoop::pump: job {job} has no retained placement authority");
                if !self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-placement", detail: message.clone() }) {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_turns.remove(&(actor_id, job));
                    self.job_authorities.remove(&(actor_id, job));
                }
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(1);
            };
            if turn.step_sequence == u64::MAX || turn.operation.preview_sequence == u64::MAX {
                let message = format!("ShardLoop::pump: job {job} exhausted its checked replay sequence");
                if !self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-sequence", detail: message.clone() }) {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_turns.remove(&(actor_id, job));
                    self.job_authorities.remove(&(actor_id, job));
                    self.job_placement.remove(&(actor_id, job));
                }
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(1);
            }
            // 🔀️ Same E0502 reason as the turn-execution loop above — computed before `get_mut`.
            let job_budget = job_budget_from_grant(self.granted_budget(actor_id));
            let actor_lane = self.actor_lane(actor_id);
            let watchdog_stage = interactive_stage_for(actor_lane);
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::ActorLost);
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") }).await?;
                return Ok(1);
            };
            // 🐕️ P1c: same watchdog treatment as `Self::execute_turn_for` — see that call site's doc.
            let job_outcome = {
                let _watchdog = Watchdog::start("plugin-host.shard.step_job", OperationId(actor_id), Generation(job), watchdog_stage);
                self.runtime.step_job(instance, job, job_budget.await).await
            };
            let outcome = match job_outcome {
                Ok(step) => {
                    let step_outcome = match step {
                        JobStep::Running { progress: Some(preview) } => JobStepOutcome::PreviewReady { preview },
                        JobStep::Running { progress: None } => JobStepOutcome::Yield,
                        JobStep::Done { output } => match self.runtime.checkpoint(instance).await {
                            Ok(state) => {
                                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Completed);
                                defer_completion(
                                    &mut self.pending_interactive,
                                    &mut self.pending_background,
                                    &mut self.terminal_authorities,
                                    actor_lane,
                                    self.allocations.get(&actor_id).copied(),
                                    actor_id,
                                    Event::JobCompleted { job, result: RequestOutcome::Ok(output.clone()) },
                                )?;
                                JobStepOutcome::Complete { candidate: JobCommitCandidate { state, output } }
                            }
                            Err(fault) => {
                                self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "terminal-checkpoint", detail: fault.to_string() });
                                let detail = start_job_fault_bytes(&TurnFault::from(fault));
                                defer_completion(
                                    &mut self.pending_interactive,
                                    &mut self.pending_background,
                                    &mut self.terminal_authorities,
                                    actor_lane,
                                    self.allocations.get(&actor_id).copied(),
                                    actor_id,
                                    Event::JobCompleted { job, result: RequestOutcome::Err(detail.clone()) },
                                )?;
                                JobStepOutcome::Fault { detail }
                            }
                        },
                        JobStep::Failed { error } => {
                            self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "guest-failed", detail: String::from_utf8_lossy(&error).into_owned() });
                            defer_completion(
                                &mut self.pending_interactive,
                                &mut self.pending_background,
                                &mut self.terminal_authorities,
                                actor_lane,
                                self.allocations.get(&actor_id).copied(),
                                actor_id,
                                Event::JobCompleted { job, result: RequestOutcome::Err(error.clone()) },
                            )?;
                            JobStepOutcome::Fault { detail: error }
                        }
                    };
                    let mut published_turn = turn;
                    if matches!(step_outcome, JobStepOutcome::PreviewReady { .. }) {
                        published_turn.operation.preview_sequence += 1;
                    }
                    if !matches!(step_outcome, JobStepOutcome::Complete { .. } | JobStepOutcome::Cancelled | JobStepOutcome::Fault { .. }) {
                        self.job_turns.insert((actor_id, job), JobTurn { step_sequence: turn.step_sequence + 1, ..published_turn });
                    }
                    ShardOutcome::Job { actor: actor_id, authority: authority.turn, request: authority.request, placement, publication: JobPublication { turn: published_turn, outcome: step_outcome } }
                }
                Err(fault) => {
                    self.close_replay_job(actor_id, job, ReplaySeedCloseReason::Fault { stage: "step-job", detail: turn_fault_message(&fault) });
                    defer_completion(
                        &mut self.pending_interactive,
                        &mut self.pending_background,
                        &mut self.terminal_authorities,
                        actor_lane,
                        self.allocations.get(&actor_id).copied(),
                        actor_id,
                        Event::JobCompleted { job, result: RequestOutcome::Err(start_job_fault_bytes(&fault)) },
                    )?;
                    ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) }
                }
            };
            self.send_outcome(&outcome).await?;
            return Ok(1);
        }

        Ok(0)
    }

    /// 🚦 One actor's turn: takes its collected `events` out of `events_by_actor`, runs
    /// [`super::GuestRuntime::execute_turn`], admits `SpawnJob`/`CancelJob` effects, and sends the
    /// resulting [`ShardOutcome`]. A failed guest cancellation retires the actor before reuse.
    async fn execute_turn_for(&mut self, actor_id: u64, event: &Event, granted: semio_framework_actor::Budget, actor_lane: semio_framework_actor::Lane) -> Result<bool, PluginHostError> {
        let events = std::slice::from_ref(event);
        // 🔀️ Computed BEFORE `get_mut` below — `self.granted_budget(actor_id)`/`self.actor_lane(..)`
        // need `&self` (the whole struct), which conflicts with the `&mut self.instances` borrow
        // `instance` holds for the rest of this call (E0502).
        let turn_budget = turn_budget_from_grant(granted);
        let watchdog_stage = interactive_stage_for(actor_lane);
        let Some(instance) = self.instances.get_mut(&actor_id) else {
            self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") }).await?;
            return Ok(false);
        };
        // 👶️ host-dedyn: `GuestRuntime::execute_turn` is plain AFIT now (double-future collapsed)
        // — `.await`ed directly. `ShardLoop` is driven by a `WorkerPool` job now (P1c) rather than a
        // dedicated OS thread — that job's own `block_on` (`🧵️executor/🦀️.rs`'s `ShardExecutor::run`) is
        // the executor boundary; every impl `ShardLoop` is ever handed resolves on its first poll
        // (see `GuestRuntime`'s own doc comment), so this never actually parks.
        //
        // 🐕️ P1c: `Watchdog` wraps ONLY the guest call itself (not the effect-admission bookkeeping
        // below) — `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` is 8ms; every lane's own
        // `lane_defaults::budget_for` grants MORE than that to UserVisible (16ms)/Background
        // (50ms)/Maintenance (200ms) turns BY DESIGN (the epoch-interruption ceiling, not a soft
        // target), so those lanes are EXPECTED to record a violation on a turn that spends its full
        // grant — see `📓️p1c-actor-shards.md`'s "turn paths exceeding 8ms" section. This packet only
        // wires the recording; making a single guest call internally resumable within 8ms slices is
        // Phase 2's job-protocol work, not this one's.
        let turn_outcome = {
            let _watchdog = Watchdog::start("plugin-host.shard.execute_turn", OperationId(actor_id), Generation(0), watchdog_stage);
            self.runtime.execute_turn(instance, events, turn_budget.await).await
        };
        let outcome = match turn_outcome {
            Ok(mut result) => {
                let base_revision = result.ui_patches.iter().map(|patch| patch.revision.0).max().unwrap_or_default();
                let bridged = to_actor_turn_result_in_place(&mut result, actor_id, 0, 0).await;
                // 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, placement routing added K1): the
                // generic `Effect::SpawnJob`/`Effect::CancelJob` admission this packet closes — see
                // `running_jobs`'s own doc comment. `placement` (inline/isolated/exclusive) is
                // captured into `job_placement` and `Exclusive` is routed to the FRONT of
                // `to_step`'s per-pump order (below) — every placement still runs on the SAME
                // instance that spawned it (routing to a DIFFERENT pooled/exclusive INSTANCE needs
                // the actor pool `Kernel::activate`/`ShardTable` builds, `design-runtime.md` §1,
                // `🎭️actor` territory a single `ShardLoop` cannot reach on its own — documented gap,
                // not a silently faked one, see the K1 report's lease-request).
                for effect in std::mem::take(&mut result.effects) {
                    match effect {
                        Effect::SpawnJob { job, kind, input, placement } => {
                            if self.next_job_operation == u64::MAX {
                                defer_completion(
                                    &mut self.pending_interactive,
                                    &mut self.pending_background,
                                    &mut self.terminal_authorities,
                                    actor_lane,
                                    self.allocations.get(&actor_id).copied(),
                                    actor_id,
                                    Event::JobCompleted { job, result: RequestOutcome::Err(b"checked replay operation identity exhausted".to_vec()) },
                                )?;
                                continue;
                            }
                            let operation = self.next_job_operation;
                            self.next_job_operation += 1;
                            let authority =
                                JobTurn { job, operation: JobOperation { operation, base_revision, generation: u64::from(ActorId(actor_id).generation()) + 1, preview_sequence: 0, seed: operation.rotate_left(17) ^ actor_id ^ job }, step_sequence: 0 };
                            let request = JobReplayRequest::from_spawn(&kind, &input);
                            let mounted = if self.replay_seeds.iter().any(Option::is_none) { MountedReplaySeed::new(actor_id, job, authority, request, placement, kind, input) } else { Err((kind, input)) };
                            match mounted {
                                Ok(seed) => {
                                    let slot = self.replay_seeds.iter_mut().find(|slot| slot.is_none()).expect("preflighted fixed replay seed slot");
                                    *slot = Some(seed);
                                }
                                Err((kind, input)) => {
                                    let reason: &'static [u8] = if self.replay_seeds.iter().any(Option::is_none) { b"fixed replay seed exceeds admitted page capacity" } else { b"fixed replay seed registry refused the exact spawn owner" };
                                    let refusal = ReplaySpawnRefusal::new(actor_id, job, reason, kind, input);
                                    let slot = self.replay_seed_refusals.iter_mut().find(|slot| slot.is_none()).expect("one empty refusal slot per granted maximum effect");
                                    *slot = Some(refusal);
                                }
                            }
                        }
                        Effect::CancelJob { job } => {
                            if let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor_id && seed.job == job && !matches!(seed.phase, ReplaySeedPhase::Retained))) {
                                Self::begin_replay_seed_close_owned(&mut self.replay_seeds, &mut self.running_jobs, &mut self.job_turns, &mut self.job_authorities, &mut self.job_placement, index, ReplaySeedCloseReason::Cancelled);
                            } else if self.running_jobs.contains(&(actor_id, job)) {
                                match self.runtime.cancel_job(instance, job).await {
                                    Ok(()) => {
                                        if let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor_id && seed.job == job)) {
                                            Self::begin_replay_seed_close_owned(&mut self.replay_seeds, &mut self.running_jobs, &mut self.job_turns, &mut self.job_authorities, &mut self.job_placement, index, ReplaySeedCloseReason::Cancelled);
                                        } else {
                                            self.running_jobs.remove(&(actor_id, job));
                                            self.job_turns.remove(&(actor_id, job));
                                            self.job_authorities.remove(&(actor_id, job));
                                            self.job_placement.remove(&(actor_id, job));
                                        }
                                    }
                                    Err(fault) => {
                                        let message = format!("ShardLoop::pump: cancel-job {job} failed; actor {actor_id} retired: {}", turn_fault_message(&fault));
                                        self.unregister(ActorId(actor_id)).await;
                                        self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                                        return Ok(false);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                match bridged {
                    Ok(result) => ShardOutcome::Turn { actor: actor_id, result },
                    Err(fault) => ShardOutcome::Fault { actor: actor_id, message: fault.message },
                }
            }
            // 🛑️ terra-shard-lane piece 2: a background/maintenance turn that ran past its
            // epoch-armed `budget.wall_ms` (`turn_budget_from_grant`'s `deadline_ms`, armed in
            // `WasmtimeRuntime::execute_turn`/`step_job` via `store.set_epoch_deadline`, ticked by
            // `EpochTicker` every 1 ms) must be RE-GRANTED next tick, not treated as a failure — an
            // epoch interrupt lands at a wasm-bytecode safe point, so the wasmtime `Store` inside
            // `self.instances[&actor_id]` stays perfectly usable and nothing here unregisters it or
            // clears its state. Sending `ShardOutcome::Fault` for this (the OLD behavior, still
            // correct for every OTHER `TurnFault` variant below) would have the kernel's
            // failure-escalation path quarantine an actor purely for being preempted by the exact
            // per-turn wall budget this ticket's own DRR scheduler assigned it — see
            // `📓️terra-shard-lane-report.md`.
            Err(TurnFault::DeadlineExceeded) => {
                // 👥️ `presence: Vec::new()` — a deadline-exceeded turn never finished, so there is
                // no guest-computed presence (or effects/ui_patches) to carry, unlike the two
                // wire-shape-mismatch sites this packet's report flags (`🦀️.rs`'s
                // `execute_turn`, `⏳️runtime/🦀️.rs`'s `convert_poll_success`): nothing was dropped
                // here, there was simply nothing produced.
                let result = TurnResult {
                    ui_patches: semio_framework::kernel::UiTurnPatches::default(),
                    effects: Vec::new(),
                    presence: Vec::new(),
                    next_wake: None,
                    status: semio_framework::kernel::TurnStatus::MoreWork,
                    fuel_used: 0,
                    command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
                    cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
                    lifecycle_receipt: None,
                    ui_patch_receipt: None,
                };
                match to_actor_turn_result(result, actor_id, 0, 0).await {
                    Ok(result) => ShardOutcome::Turn { actor: actor_id, result },
                    Err(fault) => ShardOutcome::Fault { actor: actor_id, message: fault.message },
                }
            }
            Err(fault) if super::retryable_lifecycle_turn(&fault, events) => return Ok(true),
            Err(fault) => ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) },
        };
        self.send_outcome(&outcome).await?;
        Ok(false)
    }

    /// 📨️ Decodes one [`ShardFrame`] and dispatches it — the drain loop's per-frame body, factored
    /// out so both [`Self::pump_primed`]'s "one primed frame, then the non-blocking drain" shape
    /// and `ShardFrame::Grant`'s own per-envelope loop (below) can share it.
    async fn consume_frame(&mut self, bytes: Vec<u8>) -> Result<(), FrameAdmissionError> {
        if bytes.len() > SHARD_FRAME_MAX_BYTES {
            let byte_len = bytes.len();
            return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin(format!("ShardLoop: raw frame exceeds {SHARD_FRAME_MAX_BYTES} bytes ({byte_len}); exact bytes retained for terminal close"))));
        }
        let mut pos = 0usize;
        let frame = match ShardFrame::pack_decode(&bytes, &mut pos).await {
            Ok(frame) if pos == bytes.len() => frame,
            Ok(_) => {
                return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin("ShardLoop::pump: malformed frame has trailing bytes; exact bytes retained for terminal close".to_string())));
            }
            Err(error) => {
                return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin(format!("ShardLoop::pump: malformed frame: {error:?}; exact bytes retained for terminal close"))));
            }
        };
        let target = match &frame {
            ShardFrame::Grant { actor, .. } | ShardFrame::Unregister { actor } => Some(*actor),
            ShardFrame::Envelope(envelope) => Some(envelope.to),
            ShardFrame::Register { .. } => None,
        };
        if let Some(actor) = target.filter(|actor| !self.actor_generation_is_current(*actor)) {
            self.send_outcome(&ShardOutcome::Fault { actor: actor.0, message: "actor is not registered on this shard".into() }).await.map_err(FrameAdmissionError::Fault)?;
            return Ok(());
        }
        if let Err(error) = self.validate_frame(&frame) {
            return Err(self.retain_terminal_frame(bytes, error));
        }
        if let Err(limit) = self.preflight_frame(&frame, bytes.len()) {
            let deferred_empty = self.pending_interactive.is_empty() && self.pending_background.is_empty();
            if deferred_empty {
                let byte_len = bytes.len();
                return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin(format!("ShardLoop: one frame permanently exceeds deferred {limit:?} capacity ({byte_len} bytes); exact frame retained for terminal close"))));
            }
            return Err(FrameAdmissionError::Full { bytes });
        }
        match frame {
            ShardFrame::Register { actor } => self.enqueue_authority(semio_framework_actor::Lane::Maintenance, DeferredAuthority::Register { actor }, bytes.len()).map_err(FrameAdmissionError::Fault)?,
            ShardFrame::Unregister { actor } => self.enqueue_authority(semio_framework_actor::Lane::Maintenance, DeferredAuthority::Unregister { actor }, bytes.len()).map_err(FrameAdmissionError::Fault)?,
            ShardFrame::Grant { actor, budget, envelopes } => {
                if !self.actor_generation_is_current(actor) {
                    return Ok(());
                }
                self.granted_budgets.insert(actor.0, budget);
                let item_count = envelopes.len();
                for (index, envelope) in envelopes.into_iter().enumerate() {
                    if let Err(error) = self.dispatch_envelope(envelope, split_frame_credit(bytes.len(), item_count, index)).await {
                        return Err(self.retain_terminal_frame(bytes, error));
                    }
                }
            }
            ShardFrame::Envelope(envelope) => {
                if self.actor_generation_is_current(envelope.to) {
                    if let Err(error) = self.dispatch_envelope(envelope, bytes.len()).await {
                        return Err(self.retain_terminal_frame(bytes, error));
                    }
                }
            }
        }
        Ok(())
    }

    fn retain_terminal_frame(&mut self, bytes: Vec<u8>, error: PluginHostError) -> FrameAdmissionError {
        let byte_len = bytes.len();
        match self.terminal_frames.try_push(bytes, byte_len) {
            Ok(_) => FrameAdmissionError::Fault(error),
            Err(rejected) => FrameAdmissionError::TerminalCapacity { bytes: rejected.owner, error },
        }
    }

    fn preflight_frame(&self, frame: &ShardFrame, raw_bytes: usize) -> Result<(), AdmissionLimit> {
        let envelopes: &[Envelope] = match frame {
            ShardFrame::Grant { envelopes, .. } => envelopes,
            ShardFrame::Envelope(envelope) => std::slice::from_ref(envelope),
            ShardFrame::Register { .. } | ShardFrame::Unregister { .. } => return self.pending_background.can_admit(1, raw_bytes),
        };
        let mut interactive_items = 0usize;
        let mut interactive_bytes = 0usize;
        let mut background_items = 0usize;
        let mut background_bytes = 0usize;
        for (index, envelope) in envelopes.iter().enumerate() {
            let credit = split_frame_credit(raw_bytes, envelopes.len(), index);
            if Self::is_high_priority_lane(envelope.lane) {
                interactive_items = interactive_items.saturating_add(1);
                interactive_bytes = interactive_bytes.saturating_add(credit);
            } else {
                background_items = background_items.saturating_add(1);
                background_bytes = background_bytes.saturating_add(credit);
            }
        }
        self.pending_interactive.can_admit(interactive_items, interactive_bytes)?;
        self.pending_background.can_admit(background_items, background_bytes)
    }

    fn validate_frame(&self, frame: &ShardFrame) -> Result<(), PluginHostError> {
        let envelopes: &[Envelope] = match frame {
            ShardFrame::Grant { envelopes, .. } => envelopes,
            ShardFrame::Envelope(envelope) => std::slice::from_ref(envelope),
            ShardFrame::Register { .. } | ShardFrame::Unregister { .. } => return Ok(()),
        };
        for envelope in envelopes {
            match &envelope.payload {
                Payload::Event { bytes } => {
                    serde_json::from_slice::<Event>(bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
                }
                Payload::JobStep { turn } => self.validate_job_turn(envelope.to.0, *turn)?,
                Payload::JobReplay { turn, request, .. } => self.validate_replay_request(envelope.to.0, *turn, *request)?,
                Payload::Suspend { .. } | Payload::Resume { .. } | Payload::Cancel { .. } => {}
            }
        }
        Ok(())
    }

    fn enqueue_authority(&mut self, lane: semio_framework_actor::Lane, authority: DeferredAuthority, owner_bytes: usize) -> Result<(), PluginHostError> {
        let authority = AdmittedAuthority::new(self.current_allocation(authority.actor()), self.granted_budget(authority.actor()), authority, lane, owner_bytes);
        let result = if Self::is_high_priority_lane(lane) { self.pending_interactive.try_push(authority, owner_bytes) } else { self.pending_background.try_push(authority, owner_bytes) };
        if let Err(rejected) = result {
            let _ = self.terminal_authorities.try_push(rejected.owner.authority, owner_bytes).expect("ShardLoop: terminal authority ring owns every rejected authority");
            return Err(PluginHostError::Plugin(format!("ShardLoop: release admission rejected a preflighted {lane:?} authority at {:?}", rejected.limit)));
        }
        Ok(())
    }

    fn actor_generation_is_current(&self, actor: ActorId) -> bool {
        self.instances.contains_key(&actor.0) && self.allocations.contains_key(&actor.0)
    }

    /// ✉️ One [`Envelope`]'s payload, dispatched — the exact per-envelope body `pump()` used to run
    /// directly off `Envelope::pack_decode`'s output before `ShardFrame` wrapped it; unchanged
    /// behavior, just reachable from BOTH `ShardFrame::Envelope` and each of `ShardFrame::Grant`'s
    /// bundled envelopes now.
    ///
    /// `Payload::Event`'s bytes are this file's own JSON encoding of `semio_framework::kernel::Event`
    /// — `semio_framework_actor::Payload`'s own doc comment calls this "pack-encoded", the eventual
    /// intended format once `🎠️kernel` grows a `pack_encode`/`pack_decode` for `Event`/`TurnResult`
    /// (not yet built, `🎠️kernel` is out of this packet's `path_scope`); JSON is what every OTHER
    /// wire boundary in this crate already uses (`IoRouter`/`EffectEventMarshal`), so this is a
    /// documented, consistent placeholder, not an invented one-off.
    async fn dispatch_envelope(&mut self, envelope: Envelope, owner_bytes: usize) -> Result<(), PluginHostError> {
        if !self.actor_generation_is_current(envelope.to) {
            return Ok(());
        }
        // 🚦 terra-shard-lane piece 1: records the LAST-seen lane for `envelope.to`, covering both
        // standalone `ShardFrame::Envelope` frames and every envelope bundled inside a
        // `ShardFrame::Grant` (`Self::consume_frame`'s `Grant` arm calls this per envelope) — see
        // `Self::actor_lane`'s own doc for why this, not a `ShardFrame::Grant`-level field, is where
        // the lane classification comes from.
        let actor = envelope.to.0;
        let lane = envelope.lane;
        self.actor_lanes.insert(actor, lane);
        let authority = match envelope.payload {
            Payload::Event { bytes: event_bytes } => DeferredAuthority::Event { actor, event: serde_json::from_slice(&event_bytes).map_err(|error| PluginHostError::Json(error.to_string()))? },
            Payload::JobStep { turn } => DeferredAuthority::JobStep { actor, turn },
            Payload::JobReplay { turn, request, worker_count, worker_slot } => DeferredAuthority::JobReplay { actor, turn, request, worker_count, worker_slot },
            Payload::Suspend { operation, applied_progress } => DeferredAuthority::Suspend { actor, operation, applied_progress },
            Payload::Resume { operation, checkpoint } => DeferredAuthority::Resume { actor, operation, checkpoint },
            Payload::Cancel { seq: _ } => DeferredAuthority::Cancel(CancelCursor { actor, after_job: None, owner_bytes }),
        };
        self.enqueue_authority(lane, authority, owner_bytes)
    }

    async fn suspend_one(&mut self, actor_id: u64, operation: JobOperation, applied_progress: u64) -> Result<(), PluginHostError> {
        if !self.actor_generation_is_current(ActorId(actor_id)) {
            return Ok(());
        }
        let outcome = match self.instances.get_mut(&actor_id) {
            None => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Suspend for actor {actor_id} which is not registered on this shard") },
            Some(instance) => match self.runtime.checkpoint(instance).await {
                Ok(state) => ShardOutcome::Checkpoint { actor: actor_id, operation, checkpoint: JobCheckpoint { state, applied_progress } },
                Err(error) => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Suspend checkpoint failed for actor {actor_id}: {error}") },
            },
        };
        self.send_outcome(&outcome).await
    }

    async fn resume_one(&mut self, actor_id: u64, operation: JobOperation, checkpoint: JobCheckpoint) -> Result<(), PluginHostError> {
        if !self.actor_generation_is_current(ActorId(actor_id)) {
            return Ok(());
        }
        let outcome = match self.instances.get_mut(&actor_id) {
            None => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Resume for actor {actor_id} which is not registered on this shard") },
            Some(instance) => match self.runtime.restore(instance, &checkpoint.state).await {
                Ok(()) => ShardOutcome::Resumed { actor: actor_id, operation },
                Err(error) => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Resume restore failed for actor {actor_id}: {error}") },
            },
        };
        self.send_outcome(&outcome).await
    }

    async fn cancel_one(&mut self, cursor: CancelCursor, lane: semio_framework_actor::Lane) -> Result<(), PluginHostError> {
        let actor_id = cursor.actor;
        if !self.actor_generation_is_current(ActorId(actor_id)) {
            return Ok(());
        }
        let job = self.running_jobs.range((actor_id, cursor.after_job.map_or(0, |job| job.saturating_add(1)))..).next().copied().filter(|(job_actor, _)| *job_actor == actor_id).map(|(_, job)| job);
        if let Some(job) = job {
            let result = match self.instances.get_mut(&actor_id) {
                Some(instance) => self.runtime.cancel_job(instance, job).await,
                None => {
                    self.retire_actor_replay_owners(actor_id);
                    self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Cancel for actor {actor_id} lost its registered instance") }).await?;
                    return Ok(());
                }
            };
            if let Err(fault) = result {
                self.unregister(ActorId(actor_id)).await;
                let message = format!("ShardLoop::pump: actor cancel-job {job} failed before retirement: {}", turn_fault_message(&fault));
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                return Ok(());
            }
            if let Some(index) = self.replay_seeds.iter().position(|seed| seed.as_ref().is_some_and(|seed| seed.actor == actor_id && seed.job == job)) {
                self.begin_replay_seed_close(index, ReplaySeedCloseReason::Cancelled);
            } else {
                self.running_jobs.remove(&(actor_id, job));
                self.job_turns.remove(&(actor_id, job));
                self.job_authorities.remove(&(actor_id, job));
                self.job_placement.remove(&(actor_id, job));
            }
            let authority = DeferredAuthority::Cancel(CancelCursor { actor: actor_id, after_job: Some(job), owner_bytes: cursor.owner_bytes });
            let authority = AdmittedAuthority::new(self.current_allocation(actor_id), self.granted_budget(actor_id), authority, lane, cursor.owner_bytes);
            let result = if Self::is_high_priority_lane(lane) { self.pending_interactive.try_push(authority, cursor.owner_bytes) } else { self.pending_background.try_push(authority, cursor.owner_bytes) };
            if let Err(rejected) = result {
                let _ = self.terminal_authorities.try_push(rejected.owner.authority, cursor.owner_bytes).expect("ShardLoop: terminal authority ring owns every rejected close cursor");
                return Err(PluginHostError::Plugin(format!("ShardLoop: interrupted close handback rejected at {:?}; exact cursor retained", rejected.limit)));
            }
            return Ok(());
        }
        self.unregister(ActorId(actor_id)).await;
        self.send_outcome(&ShardOutcome::Cancelled { actor: actor_id }).await
    }

    /// 🔐️ Validates a requested turn against the active replay cursor before publication.
    fn accept_job_turn(&mut self, actor: u64, turn: JobTurn) -> Result<(), PluginHostError> {
        self.validate_job_turn(actor, turn)?;
        let key = (actor, turn.job);
        self.job_turns.entry(key).or_insert(turn);
        Ok(())
    }

    fn validate_job_turn(&self, actor: u64, turn: JobTurn) -> Result<(), PluginHostError> {
        match self.job_turns.get(&(actor, turn.job)) {
            None if turn.step_sequence == 0 => Ok(()),
            None => Err(PluginHostError::Plugin(format!("ShardLoop::job bridge: first step for actor {actor}, job {} had sequence {}, expected 0", turn.job, turn.step_sequence))),
            Some(active)
                if active.operation.operation == turn.operation.operation
                    && active.operation.base_revision == turn.operation.base_revision
                    && active.operation.generation == turn.operation.generation
                    && active.operation.preview_sequence == turn.operation.preview_sequence
                    && active.operation.seed == turn.operation.seed
                    && active.step_sequence == turn.step_sequence =>
            {
                Ok(())
            }
            Some(active) => Err(PluginHostError::Plugin(format!("ShardLoop::job bridge: stale or non-deterministic turn for actor {actor}, job {}: active={active:?}, requested={turn:?}", turn.job))),
        }
    }

    async fn send_outcome(&mut self, outcome: &ShardOutcome) -> Result<(), PluginHostError> {
        let mut bytes = Vec::new();
        outcome.pack_encode(&mut bytes).await.map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        self.transport.send(&bytes).await;
        Ok(())
    }

    pub async fn heartbeat(&self) -> u64 {
        self.transport.heartbeat().await
    }
}

pub(crate) struct ReplayLifecycleProjection {
    pub state: &'static str,
    pub first_reason: Option<String>,
    pub release_opportunities: usize,
}

pub(crate) fn exercise_replay_lifecycle_trace(events: &[String]) -> Result<ReplayLifecycleProjection, String> {
    let pages_before = JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire);
    let abi_before = JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire);
    let actor = 1;
    let job = 1;
    let kind = "relay.lifecycle.fixture".to_string();
    let input = Vec::new();
    let request = JobReplayRequest::from_spawn(&kind, &input);
    let authority = JobTurn { job, operation: JobOperation { operation: 1, base_revision: 0, generation: 1, preview_sequence: 0, seed: 1 }, step_sequence: 0 };
    let mut replay_seeds = vec![Some(MountedReplaySeed::new(actor, job, authority, request, JobPlacement::Inline, kind, input).map_err(|_| "replay lifecycle fixture admission refused".to_string())?)];
    let mut running_jobs = BTreeSet::new();
    let mut job_turns = HashMap::new();
    let mut job_authorities = HashMap::new();
    let mut job_placement = HashMap::new();
    let mut first_reason = None;
    let mut release_opportunities = 0;
    for event in events {
        if let Some(stage) = event.strip_prefix("fault:") {
            let stage = match stage {
                "capture-kind" => "capture-kind",
                "capture-input" => "capture-input",
                "checkpoint" => "checkpoint",
                "capture-checkpoint" => "capture-checkpoint",
                "live-start" => "live-start",
                "materialize-kind" => "materialize-kind",
                "materialize-input" => "materialize-input",
                "materialize-checkpoint" => "materialize-checkpoint",
                "restore" => "restore",
                "prepare-kind" => "prepare-kind",
                "restart" => "restart",
                other => return Err(format!("unknown replay lifecycle fault stage {other:?}")),
            };
            ShardLoop::begin_replay_seed_close_owned(&mut replay_seeds, &mut running_jobs, &mut job_turns, &mut job_authorities, &mut job_placement, 0, ReplaySeedCloseReason::Fault { stage, detail: event.clone() });
        } else if event == "cancel" {
            ShardLoop::begin_replay_seed_close_owned(&mut replay_seeds, &mut running_jobs, &mut job_turns, &mut job_authorities, &mut job_placement, 0, ReplaySeedCloseReason::Cancelled);
        } else if event == "release" && replay_seeds[0].is_some() {
            ShardLoop::close_replay_seed_one(&mut replay_seeds, 0).map_err(|error| error.to_string())?;
            release_opportunities += 1;
        }
        if first_reason.is_none() {
            first_reason = replay_seeds[0].as_ref().and_then(|seed| seed.close_reason.as_ref()).map(|reason| match reason {
                ReplaySeedCloseReason::Completed => "completed".to_string(),
                ReplaySeedCloseReason::Cancelled => "cancelled".to_string(),
                ReplaySeedCloseReason::ActorLost => "actor-lost".to_string(),
                ReplaySeedCloseReason::Fault { stage, .. } => format!("fault:{stage}"),
            });
        }
    }
    let state = replay_seeds[0].as_ref().map_or("Empty", |seed| match seed.phase {
        ReplaySeedPhase::Closing | ReplaySeedPhase::RetireSeedShell | ReplaySeedPhase::RetireMountedShell => "Closing",
        ReplaySeedPhase::Retained => "Retained",
        _ => "CaptureKind",
    });
    drop(replay_seeds);
    if JOB_REPLAY_SEED_PAGES.load(Ordering::Acquire) != pages_before || JOB_REPLAY_ABI_BYTES.load(Ordering::Acquire) != abi_before {
        return Err("replay lifecycle fixture accounting did not return to baseline".to_string());
    }
    Ok(ReplayLifecycleProjection { state, first_reason, release_opportunities })
}

fn turn_fault_message(fault: &TurnFault) -> String {
    fault.to_string()
}

/// 🐕️ P1c: maps an actor's [`semio_framework_actor::Lane`] onto the [`InteractiveStage`] family
/// [`Watchdog::start`] reports overruns under. `Interactive`/`UserVisible` map onto their own
/// dedicated stages (this crate has no separate "UI event" vs "UI present" distinction — a shard's
/// interactive turn IS the step); `Background`/`Maintenance` share `BackgroundStep` since neither has
/// a tighter soft target than the other in `semio_framework_trace`'s own vocabulary.
fn interactive_stage_for(lane: semio_framework_actor::Lane) -> InteractiveStage {
    match lane {
        semio_framework_actor::Lane::Interactive => InteractiveStage::InteractiveStep,
        semio_framework_actor::Lane::UserVisible => InteractiveStage::UserVisibleSimStep,
        semio_framework_actor::Lane::Background | semio_framework_actor::Lane::Maintenance => InteractiveStage::BackgroundStep,
    }
}

/// 🧯️ Encodes a host-side `TurnFault` (a `start-job` admission failure, or a `step_job` runtime
/// fault) into the same `dsl::encode_fault_bytes` wire shape `Event::JobCompleted{result: Err
/// (bytes), ..}`'s bytes always carry — every other fault-bearing `RequestOutcome::Err` in this
/// crate already uses this encoding, so the guest's `crate::host::outcome_to_result` decodes it
/// exactly like a normal `Event::Completed` failure, with no special-casing for jobs.
fn start_job_fault_bytes(fault: &TurnFault) -> Vec<u8> {
    dsl::encode_fault_bytes(&semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("job.host-fault"), fault.to_string()))
}

//#region 🚚️ShardTransports
/// 🚚️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn): the closed-set enum replacing every
/// `Box<dyn ShardTransport>` in this crate. `ShardTransport` (O1/R1: `async fn` in a trait cannot be
/// `dyn`-dispatched) is declared OUTSIDE this crate (`semio_framework_actor::ShardTransport`, this
/// packet's path scope forbids touching `🔌️plugin/**` outside `🖥️host/`), so `#[dyn_enum]` cannot be
/// applied to it — the macro's bare-invocation mechanism (`📓️terra-dyn-enum-macro-report.md`,
/// finding 1) only works when the trait's OWN crate emits the captured delegation macro. This family
/// is therefore hand-written unconditionally, not merely because of cfg-gated variants (`GuestRuntimes`'
/// own reason, next region) — a second, independent reason the macro doesn't apply here, worth
/// recording for the ~50 remaining families since some of them will hit this exact wall.
pub enum ShardTransports {
    SharedThread(executor::SharedThreadTransport),
    Process(super::process_transport::ProcessTransport),
    Stdio(super::process_transport::StdioTransport),
    #[cfg(test)]
    Loopback(LoopbackTransport),
}

impl ShardTransport for ShardTransports {
    async fn send(&self, bytes: &[u8]) {
        match self {
            Self::SharedThread(t) => t.send(bytes).await,
            Self::Process(t) => t.send(bytes).await,
            Self::Stdio(t) => t.send(bytes).await,
            #[cfg(test)]
            Self::Loopback(t) => t.send(bytes).await,
        }
    }

    async fn recv(&self) -> Option<Vec<u8>> {
        match self {
            Self::SharedThread(t) => t.recv().await,
            Self::Process(t) => t.recv().await,
            Self::Stdio(t) => t.recv().await,
            #[cfg(test)]
            Self::Loopback(t) => t.recv().await,
        }
    }

    async fn heartbeat(&self) -> u64 {
        match self {
            Self::SharedThread(t) => t.heartbeat().await,
            Self::Process(t) => t.heartbeat().await,
            Self::Stdio(t) => t.heartbeat().await,
            #[cfg(test)]
            Self::Loopback(t) => t.heartbeat().await,
        }
    }

    async fn kill(&self) {
        match self {
            Self::SharedThread(t) => t.kill().await,
            Self::Process(t) => t.kill().await,
            Self::Stdio(t) => t.kill().await,
            #[cfg(test)]
            Self::Loopback(t) => t.kill().await,
        }
    }
}

impl From<executor::SharedThreadTransport> for ShardTransports {
    fn from(t: executor::SharedThreadTransport) -> Self {
        Self::SharedThread(t)
    }
}
impl From<super::process_transport::ProcessTransport> for ShardTransports {
    fn from(t: super::process_transport::ProcessTransport) -> Self {
        Self::Process(t)
    }
}
impl From<super::process_transport::StdioTransport> for ShardTransports {
    fn from(t: super::process_transport::StdioTransport) -> Self {
        Self::Stdio(t)
    }
}
#[cfg(test)]
impl From<LoopbackTransport> for ShardTransports {
    fn from(t: LoopbackTransport) -> Self {
        Self::Loopback(t)
    }
}
//#endregion 🚚️ShardTransports

//#region 🧪️TestDoubles
/// 🧵️ In-process, single-actor loopback transport — an `mpsc`-free stand-in for
/// `design-runtime.md`'s `ThreadTransport`, precise enough to exercise `ShardLoop::pump`'s real
/// drain/group/dispatch/send logic end to end without needing a real thread. Its two buffers are
/// `Arc<Mutex<..>>` INTERNALLY (not the struct itself behind an `Arc`) so `LoopbackProbe::new` can
/// hand `ShardLoop::new` sole ownership of a [`ShardTransports::Loopback`] while keeping a separate
/// handle that can still inspect `outbound` afterward — `impl ShardTransport for
/// Arc<LoopbackTransport>` would hit `E0117` (neither `Arc` nor `ShardTransport` is local to this
/// crate, and `Arc` is not `#[fundamental]` the way `Box` is). `pub(crate)`, not private — moved out
/// of `mod tests` (below) so [`ShardTransports`] can name it in its own `#[cfg(test)]` variant.
#[cfg(test)]
#[derive(Default)]
pub struct LoopbackTransport {
    inbound: Arc<Mutex<Vec<Vec<u8>>>>,
    outbound: Arc<Mutex<Vec<Vec<u8>>>>,
}

#[cfg(test)]
impl LoopbackTransport {
    /// Returns `(the transport ShardLoop::new takes ownership of, a probe this test keeps)`.
    async fn paired() -> (Self, tests::LoopbackProbe) {
        let transport = Self::default();
        let probe = tests::LoopbackProbe { inbound: transport.inbound.clone(), outbound: transport.outbound.clone() };
        (transport, probe)
    }
}

#[cfg(test)]
impl ShardTransport for LoopbackTransport {
    async fn send(&self, bytes: &[u8]) {
        self.outbound.lock().expect("loopback lock").push(bytes.to_vec());
    }
    async fn recv(&self) -> Option<Vec<u8>> {
        self.inbound.lock().expect("loopback lock").pop()
    }
    async fn heartbeat(&self) -> u64 {
        0
    }
    async fn kill(&self) {}
}

/// 🧪️ A `GuestRuntime` that records the EXACT `Budget`/`JobBudget` it was invoked with — unlike
/// `MockGuestRuntime` (owned by `🖥️host/🦀️.rs`, out of this packet's edit scope, and which
/// ignores its `budget` parameter entirely), this proves the property terra-shard-grants demanded:
/// "a Grant's budget is what the turn actually executes under (prove the constants are gone, not
/// merely unused)". `pub(crate)`, not private — moved out of `mod tests` so `GuestRuntimes::
/// Recording` (`🖥️host/🦀️.rs`) can name it.
#[cfg(test)]
pub struct RecordingRuntime {
    last_turn_budget: Mutex<Option<Budget>>,
    last_job_budget: Mutex<Option<JobBudget>>,
}

#[cfg(test)]
impl RecordingRuntime {
    pub(crate) async fn new() -> Self {
        Self { last_turn_budget: Mutex::new(None), last_job_budget: Mutex::new(None) }
    }
}

#[cfg(test)]
impl GuestRuntime for RecordingRuntime {
    async fn compile(&self, package: &PackageRef, _bytes: &[u8]) -> Result<super::CompiledHandle, PluginHostError> {
        Ok(super::CompiledHandle { package_hash: package.hash.0, component: None, owned: None })
    }
    async fn instantiate(&self, _compiled: &super::CompiledHandle, actor: ActorId, _caps: &[super::BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        Ok(GuestInstance { actor, state: GuestInstanceState::Mock(super::MockInstanceState::default()) })
    }
    async fn drop_instance(&self, _inst: GuestInstance) {}
    async fn execute_turn(&self, _inst: &mut GuestInstance, _events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault> {
        *self.last_turn_budget.lock().expect("lock") = Some(budget);
        Ok(TurnResult {
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects: vec![],
            presence: vec![],
            next_wake: None,
            status: semio_framework::kernel::TurnStatus::Idle,
            fuel_used: 0,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
            lifecycle_receipt: None,
            ui_patch_receipt: None,
        })
    }
    async fn start_job(&self, _inst: &mut GuestInstance, _job: u64, _kind: &str, _input: Vec<u8>) -> Result<(), TurnFault> {
        Ok(())
    }
    async fn step_job(&self, _inst: &mut GuestInstance, _job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        *self.last_job_budget.lock().expect("lock") = Some(budget);
        Ok(JobStep::Running { progress: None })
    }
    async fn cancel_job(&self, _inst: &mut GuestInstance, _job: u64) -> Result<(), TurnFault> {
        Ok(())
    }
    async fn checkpoint(&self, _inst: &mut GuestInstance) -> Result<Vec<u8>, PluginHostError> {
        Ok(vec![])
    }
    async fn restore(&self, _inst: &mut GuestInstance, _state: &[u8]) -> Result<(), PluginHostError> {
        Ok(())
    }
}
//#endregion 🧪️TestDoubles

#[cfg(test)]
fn fixture_instance_close_request() -> semio_framework::kernel::ActorInstanceCloseRequest {
    semio_framework::kernel::ActorInstanceCloseRequest { lifetime: semio_framework::kernel::ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 }, request_sequence: 9 }
}

#[cfg(test)]
fn fixture_instance_close_event() -> Event {
    Event::InstanceClose(fixture_instance_close_request())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
