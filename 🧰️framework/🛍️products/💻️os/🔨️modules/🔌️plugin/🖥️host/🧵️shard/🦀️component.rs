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
// dedicated OS thread since). Declared here (not in `🖥️host/🦀️component.rs`, a file this packet's
// boundary excludes) — `#[path]` on a submodule resolves relative to THIS file's own directory, so
// this reaches `🧵️shard/🏃️executor.rs` without any edit to the crate-root module tree.
#[path = "🏃️executor.rs"]
pub mod executor;

use super::{GuestInstance, GuestRuntime, GuestRuntimes, JobBudget, JobStep, PluginHostError, TurnFault};
#[cfg(test)]
use super::{GuestInstanceState, MockGuestRuntime, PackageHash, PackageId, PackageRef};
use semio_framework::kernel::{Budget, Effect, Event, JobPlacement, RequestOutcome, TurnResult};
use semio_framework_actor::{ActorId, Envelope, JobCheckpoint, JobCommitCandidate, JobOperation, JobPublication, JobStepOutcome, JobTurn, Payload, ShardTransport};
use semio_framework_trace::{Generation, InteractiveStage, OperationId, Watchdog};
use std::collections::{BTreeSet, HashMap};
use std::mem::size_of;
use std::sync::Arc;
#[cfg(test)]
use std::sync::Mutex;

//#region 📨️ShardFrame
/// 📨️ terra-shard-grants: what actually crosses a [`ShardTransport`] INBOUND (host → shard) —
/// replacing raw [`Envelope`] pack bytes so the kernel's DRR-computed, throttle-scaled per-turn
/// [`semio_framework_actor::Budget`] can travel WITH the envelopes it grants, instead of
/// `ShardLoop::pump` re-deriving one from its own local constants (the deleted `budget_for`
/// closure / `TURN_BUDGET` / `JOB_STEP_BUDGET`). Same pack encoding on the thread transport and
/// [`super::process_transport::ProcessTransport`]/`StdioTransport` — `design-runtime.md` §2's
/// "thread-or-process, same wire" promise.
#[derive(Clone, Debug, PartialEq)]
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
/// packet didn't reach"). Lives HERE, not in `🖥️host/🦀️component.rs` — `📌️important.md` rule 17:
/// a concurrent packet series owns that file, and this ticket already absorbed several
/// half-landed collisions of exactly this shape ("the artifact moved, its registration did not").
///
/// `ui_patches`/`effects` stay OPAQUE bytes on the actor-crate side by design (that crate's own
/// "opaque seam" doc, module header) — re-encoded here as JSON, the same convention this file's
/// own `Payload::Event` handling already uses for every other kernel-type-crossing-the-actor-crate
/// -seam boundary (a real `pack_encode` for `Effect`/`UiPatch` is `🎠️kernel`'s own future work,
/// out of this packet's `path_scope`). `status` maps 1:1. `usage.fuel` comes from
/// `result.fuel_used`; `wall_us`/`memory_bytes` are host-measured and passed in — this crate has
/// no clock of its own (the actor crate's purity rule pushed clocks out to callers).
pub async fn to_actor_turn_result(result: &TurnResult, wall_us: u64, memory_bytes: u64) -> semio_framework_actor::TurnResult {
    let status = match &result.status {
        semio_framework::kernel::TurnStatus::Idle => semio_framework_actor::TurnStatus::Idle,
        semio_framework::kernel::TurnStatus::MoreWork => semio_framework_actor::TurnStatus::MoreWork,
        semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint } => semio_framework_actor::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
        semio_framework::kernel::TurnStatus::Faulted(detail) => semio_framework_actor::TurnStatus::Faulted { detail: detail.clone() },
    };
    semio_framework_actor::TurnResult {
        ui_patches: serde_json::to_vec(&result.ui_patches).unwrap_or_default(),
        effects: serde_json::to_vec(&result.effects).unwrap_or_default(),
        command_ingress: serde_json::to_vec(&result.command_ingress).unwrap_or_default(),
        next_wake: result.next_wake,
        status,
        usage: semio_framework_actor::Usage { fuel: result.fuel_used, wall_us, memory_bytes },
    }
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
    pub async fn pack_encode(&self, out: &mut Vec<u8>) {
        match self {
            Self::Turn { actor, result } => {
                semio_framework_actor::pack::write_u8(out, 0).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                result.pack_encode(out).await;
            }
            Self::Job { actor, authority, publication } => {
                semio_framework_actor::pack::write_u8(out, 1).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                authority.pack_encode(out).await;
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
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor::pack::PackError> {
        let tag = semio_framework_actor::pack::read_u8(bytes, pos, "ShardOutcome").await?;
        let actor = semio_framework_actor::pack::read_u64(bytes, pos, "ShardOutcome::actor").await?;
        match tag {
            0 => Ok(Self::Turn { actor, result: semio_framework_actor::TurnResult::pack_decode(bytes, pos).await? }),
            1 => Ok(Self::Job { actor, authority: JobTurn::pack_decode(bytes, pos).await?, publication: JobPublication::pack_decode(bytes, pos).await? }),
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
    job_authorities: HashMap<(u64, u64), JobTurn>,
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
    /// 🦀️component.rs`'s `Host::spawn_job` / `⚛️reactor/🦀️component.rs`'s `Event::JobCompleted`
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
    pending_interactive: FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    pending_background: FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    rejected_frame: Option<(u64, Vec<u8>)>,
    terminal_frames: FixedOwnerRing<Vec<u8>, SHARD_DEFERRED_ITEMS>,
    terminal_frame_overflow: FixedOwnerRing<TerminalFrameOverflow, 1>,
    next_frame_epoch: u64,
    last_drive_consumed_epoch: Option<u64>,
}

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
    slots: [Option<OwnerSlot<T>>; N],
    head: usize,
    tail: usize,
    len: usize,
    bytes: usize,
    byte_capacity: usize,
    next_generation: u64,
}

impl<T, const N: usize> FixedOwnerRing<T, N> {
    pub fn new(byte_capacity: usize) -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, tail: 0, len: 0, bytes: 0, byte_capacity, next_generation: 1 }
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
        let slot = self.tail;
        debug_assert!(self.slots[slot].is_none());
        self.slots[slot] = Some(OwnerSlot { generation, bytes, owner });
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        self.bytes += bytes;
        Ok(OwnerKey { slot, generation })
    }

    pub fn pop_front(&mut self) -> Option<(OwnerKey, T)> {
        if self.len == 0 {
            return None;
        }
        let slot = self.head;
        let entry = self.slots[slot].take().expect("FixedOwnerRing: occupied head invariant");
        self.head = (self.head + 1) % N;
        self.len -= 1;
        self.bytes -= entry.bytes;
        Some((OwnerKey { slot, generation: entry.generation }, entry.owner))
    }

    pub fn front(&self) -> Option<&T> {
        self.slots.get(self.head).and_then(Option::as_ref).map(|slot| &slot.owner)
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
pub enum DeferredAuthority {
    Register { actor: ActorId },
    Unregister { actor: ActorId },
    Event { actor: u64, event: Event },
    JobStep { actor: u64, turn: JobTurn },
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
    Full { limit: AdmissionLimit, bytes: Vec<u8> },
    TerminalCapacity { bytes: Vec<u8>, error: PluginHostError },
    Fault(PluginHostError),
}

#[derive(Debug)]
struct TerminalFrameOverflow {
    epoch: u64,
    bytes: Vec<u8>,
}

fn defer_completion(
    interactive: &mut FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    background: &mut FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    terminal: &mut FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    lane: semio_framework_actor::Lane,
    actor: u64,
    event: Event,
) -> Result<(), PluginHostError> {
    let bytes = size_of::<Event>()
        + match &event {
            Event::JobCompleted { result: RequestOutcome::Ok(bytes) | RequestOutcome::Err(bytes), .. } => bytes.capacity(),
            _ => unreachable!("ShardLoop: only job completions use generated authority admission"),
        };
    let ring = if ShardLoop::is_high_priority_lane(lane) { interactive } else { background };
    match ring.try_push(DeferredAuthority::Event { actor, event }, bytes) {
        Ok(_) => Ok(()),
        Err(rejected) => {
            let _ = terminal.try_push(rejected.owner, bytes).expect("ShardLoop: terminal completion ring owns every rejected completion");
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
            running_jobs: BTreeSet::new(),
            job_turns: HashMap::new(),
            job_authorities: HashMap::new(),
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
    async fn granted_budget(&self, actor: u64) -> semio_framework_actor::Budget {
        // 🚫️async: R10 residue shape 1 — `budget_for` is external/async, hoisted out of the
        // `unwrap_or_else` sync closure.
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
    /// sites outside this packet's `path_scope` (`💻️os/🖥️host/🎠️activation.rs`'s
    /// `NativeKernelRuntime::tick_and_dispatch` and `📺️renderer/🧑️‍🎨️engine/…/🎯️targets/🧊️wgpu/
    /// 🎠️runtime.rs`'s equivalent dispatch loop — both live, both construct `ShardFrame::Grant`
    /// directly, neither is `🔌️plugin/🖥️host/**`), and the information a `Grant`-level field would
    /// have carried is ALREADY present per-envelope (`Envelope.lane`, set once per actor by the
    /// same DRR `Scheduler` that would have supplied a `Grant`-level lane). See
    /// `📓️terra-shard-lane-report.md`'s "wire change avoided" note — a `lease-request` is open
    /// against those two files in case a `Grant`-level field is still wanted for a future packet.
    async fn actor_lane(&self, actor: u64) -> semio_framework_actor::Lane {
        self.actor_lanes.get(&actor).copied().unwrap_or(semio_framework_actor::Lane::Maintenance)
    }

    /// 📌️ Adds an already-instantiated actor to this shard's live set — called once per
    /// `Kernel::activate` that lands on this shard. `actor.0` (the bit-packed `u64`) is the map key
    /// throughout this type: `Envelope.to`/`ShardOutcome`'s tag both carry the SAME raw id, so no
    /// `RuntimeActorId` round-trip is needed at the boundary.
    pub fn register(&mut self, actor: ActorId, instance: GuestInstance) {
        self.instances.insert(actor.0, instance);
    }

    pub async fn is_registered(&self, actor: ActorId) -> bool {
        self.instances.contains_key(&actor.0)
    }

    /// ✂️ Releases an actor's instance (generation change on restart, or a real unload) — calls
    /// [`super::GuestRuntime::drop_instance`] so the pooling allocator reclaims its slab.
    pub async fn unregister(&mut self, actor: ActorId) {
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

    /// 🌀️ Drains every [`ShardFrame`] CURRENTLY buffered on the transport (never blocks past that
    /// — a shard must keep polling other actors, not stall on one slow producer), groups their
    /// envelopes by destination actor preserving arrival order (`execute_turn` takes `events:
    /// &[Event]`, i.e. one call per actor per pump, not per envelope), and runs exactly one
    /// `execute_turn`/`step_job` per actor that had at least one envelope or running job. Returns
    /// the number of actors driven this pump. Equivalent to `self.pump_primed(None)`.
    pub async fn pump(&mut self) -> Result<usize, PluginHostError> {
        self.pump_primed(None).await
    }

    /// 🤝️ Admits at most one transport frame and grants exactly one actor turn or one job-step
    /// opportunity. All other decoded authorities remain owned by this shard for a later grant.
    pub async fn drive_one(&mut self) -> ShardDrive {
        self.last_drive_consumed_epoch = None;
        match self.pump_primed(None).await {
            Ok(_) if self.has_pending_work() => ShardDrive::MoreWork { consumed_epoch: self.last_drive_consumed_epoch },
            Ok(_) => ShardDrive::Idle { consumed_epoch: self.last_drive_consumed_epoch },
            Err(error) => {
                ShardDrive::Fault { error, consumed_epoch: self.last_drive_consumed_epoch, work_remains: self.has_pending_work(), terminal_frame: !self.terminal_frames.is_empty(), terminal_overflow: !self.terminal_frame_overflow.is_empty() }
            }
        }
    }

    pub fn has_pending_work(&self) -> bool {
        !self.pending_interactive.is_empty() || !self.pending_background.is_empty() || self.rejected_frame.is_some() || !self.running_jobs.is_empty()
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
        let has_deferred = !self.pending_interactive.is_empty() || !self.pending_background.is_empty() || !self.running_jobs.is_empty();
        let mut frame = self.rejected_frame.take();
        if frame.is_none() {
            if let Some(bytes) = primed {
                frame = Some((self.claim_frame_epoch(), bytes));
            }
        }
        if frame.is_none() && !has_deferred {
            if let Some(bytes) = self.transport.recv().await {
                frame = Some((self.claim_frame_epoch(), bytes));
            }
        }
        if let Some((epoch, bytes)) = frame {
            if let Err(rejected) = self.consume_frame(bytes).await {
                match rejected {
                    FrameAdmissionError::Full { bytes, .. } => self.rejected_frame = Some((epoch, bytes)),
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

        let authority = if let Some((_, authority)) = self.pending_interactive.pop_front() {
            Some((semio_framework_actor::Lane::Interactive, authority))
        } else {
            self.pending_background.pop_front().map(|(_, authority)| (semio_framework_actor::Lane::Maintenance, authority))
        };
        let mut selected_step = None;
        if let Some((lane, authority)) = authority {
            match authority {
                DeferredAuthority::Register { actor: _ } => return Ok(1),
                DeferredAuthority::Unregister { actor } => {
                    if self.actor_generation_is_current(actor) {
                        self.unregister(actor).await;
                    }
                    return Ok(1);
                }
                DeferredAuthority::Event { actor, event } => {
                    if self.actor_generation_is_current(ActorId(actor)) {
                        self.execute_turn_for(actor, event).await?;
                    }
                    return Ok(1);
                }
                DeferredAuthority::JobStep { actor, turn } => selected_step = Some((actor, turn)),
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
        let selected_step = selected_step.or_else(|| self.running_jobs.iter().find_map(|pair| self.job_turns.get(pair).copied().map(|turn| (pair.0, turn))));
        if let Some((actor_id, turn)) = selected_step {
            let job = turn.job;
            let Some(authority) = self.job_authorities.get(&(actor_id, job)).copied() else {
                self.running_jobs.remove(&(actor_id, job));
                self.job_turns.remove(&(actor_id, job));
                self.job_placement.remove(&(actor_id, job));
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: job {job} has no independently admitted operation authority") }).await?;
                return Ok(1);
            };
            // 🔀️ Same E0502 reason as the turn-execution loop above — computed before `get_mut`.
            let job_budget = job_budget_from_grant(self.granted_budget(actor_id).await);
            let actor_lane = self.actor_lane(actor_id).await;
            let watchdog_stage = interactive_stage_for(actor_lane);
            let Some(instance) = self.instances.get_mut(&actor_id) else {
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
                        JobStep::Done { output } => {
                            let state = self.runtime.checkpoint(instance).await.unwrap_or_default();
                            self.running_jobs.remove(&(actor_id, job));
                            self.job_turns.remove(&(actor_id, job));
                            self.job_authorities.remove(&(actor_id, job));
                            self.job_placement.remove(&(actor_id, job));
                            defer_completion(&mut self.pending_interactive, &mut self.pending_background, &mut self.terminal_authorities, actor_lane, actor_id, Event::JobCompleted { job, result: RequestOutcome::Ok(output.clone()) })?;
                            JobStepOutcome::Complete { candidate: JobCommitCandidate { state, output } }
                        }
                        JobStep::Failed { error } => {
                            self.running_jobs.remove(&(actor_id, job));
                            self.job_turns.remove(&(actor_id, job));
                            self.job_authorities.remove(&(actor_id, job));
                            self.job_placement.remove(&(actor_id, job));
                            defer_completion(&mut self.pending_interactive, &mut self.pending_background, &mut self.terminal_authorities, actor_lane, actor_id, Event::JobCompleted { job, result: RequestOutcome::Err(error.clone()) })?;
                            JobStepOutcome::Fault { detail: error }
                        }
                    };
                    let mut published_turn = turn;
                    if matches!(step_outcome, JobStepOutcome::PreviewReady { .. }) {
                        published_turn.operation.preview_sequence = published_turn.operation.preview_sequence.saturating_add(1);
                    }
                    if !matches!(step_outcome, JobStepOutcome::Complete { .. } | JobStepOutcome::Cancelled | JobStepOutcome::Fault { .. }) {
                        self.job_turns.insert((actor_id, job), JobTurn { step_sequence: turn.step_sequence.saturating_add(1), ..published_turn });
                    }
                    ShardOutcome::Job { actor: actor_id, authority, publication: JobPublication { turn: published_turn, outcome: step_outcome } }
                }
                Err(fault) => {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_turns.remove(&(actor_id, job));
                    self.job_authorities.remove(&(actor_id, job));
                    self.job_placement.remove(&(actor_id, job));
                    defer_completion(&mut self.pending_interactive, &mut self.pending_background, &mut self.terminal_authorities, actor_lane, actor_id, Event::JobCompleted { job, result: RequestOutcome::Err(start_job_fault_bytes(&fault).await) })?;
                    ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault).await }
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
    async fn execute_turn_for(&mut self, actor_id: u64, event: Event) -> Result<(), PluginHostError> {
        let events = [event];
        // 🔀️ Computed BEFORE `get_mut` below — `self.granted_budget(actor_id)`/`self.actor_lane(..)`
        // need `&self` (the whole struct), which conflicts with the `&mut self.instances` borrow
        // `instance` holds for the rest of this call (E0502).
        let turn_budget = turn_budget_from_grant(self.granted_budget(actor_id).await);
        let actor_lane = self.actor_lane(actor_id).await;
        let watchdog_stage = interactive_stage_for(actor_lane);
        let Some(instance) = self.instances.get_mut(&actor_id) else {
            self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") }).await?;
            return Ok(());
        };
        // 👶️ host-dedyn: `GuestRuntime::execute_turn` is plain AFIT now (double-future collapsed)
        // — `.await`ed directly. `ShardLoop` is driven by a `WorkerPool` job now (P1c) rather than a
        // dedicated OS thread — that job's own `block_on` (`🏃️executor.rs`'s `ShardExecutor::run`) is
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
            self.runtime.execute_turn(instance, &events, turn_budget.await).await
        };
        let outcome = match turn_outcome {
            Ok(result) => {
                // 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, placement routing added K1): the
                // generic `Effect::SpawnJob`/`Effect::CancelJob` admission this packet closes — see
                // `running_jobs`'s own doc comment. `placement` (inline/isolated/exclusive) is
                // captured into `job_placement` and `Exclusive` is routed to the FRONT of
                // `to_step`'s per-pump order (below) — every placement still runs on the SAME
                // instance that spawned it (routing to a DIFFERENT pooled/exclusive INSTANCE needs
                // the actor pool `Kernel::activate`/`ShardTable` builds, `design-runtime.md` §1,
                // `🎭️actor` territory a single `ShardLoop` cannot reach on its own — documented gap,
                // not a silently faked one, see the K1 report's lease-request).
                for effect in &result.effects {
                    match effect {
                        Effect::SpawnJob { job, kind, input, placement } => {
                            let operation = self.next_job_operation;
                            self.next_job_operation = self.next_job_operation.wrapping_add(1).max(1);
                            let base_revision = result.ui_patches.iter().map(|patch| patch.revision.0).max().unwrap_or_default();
                            let authority = JobTurn {
                                job: *job,
                                operation: JobOperation { operation, base_revision, generation: u64::from(ActorId(actor_id).generation()), preview_sequence: 0, seed: operation.rotate_left(17) ^ actor_id ^ *job },
                                step_sequence: 0,
                            };
                            self.job_authorities.insert((actor_id, *job), authority);
                            self.job_turns.insert((actor_id, *job), authority);
                            match self.runtime.start_job(instance, *job, kind, input.clone()).await {
                                Ok(()) => {
                                    self.running_jobs.insert((actor_id, *job));
                                    self.job_placement.insert((actor_id, *job), *placement);
                                }
                                Err(fault) => {
                                    self.job_authorities.remove(&(actor_id, *job));
                                    self.job_turns.remove(&(actor_id, *job));
                                    defer_completion(
                                        &mut self.pending_interactive,
                                        &mut self.pending_background,
                                        &mut self.terminal_authorities,
                                        actor_lane,
                                        actor_id,
                                        Event::JobCompleted { job: *job, result: RequestOutcome::Err(start_job_fault_bytes(&fault).await) },
                                    )?;
                                }
                            }
                        }
                        Effect::CancelJob { job } => {
                            if self.running_jobs.contains(&(actor_id, *job)) {
                                match self.runtime.cancel_job(instance, *job).await {
                                    Ok(()) => {
                                        self.running_jobs.remove(&(actor_id, *job));
                                        self.job_turns.remove(&(actor_id, *job));
                                        self.job_authorities.remove(&(actor_id, *job));
                                        self.job_placement.remove(&(actor_id, *job));
                                    }
                                    Err(fault) => {
                                        let message = format!("ShardLoop::pump: cancel-job {job} failed; actor {actor_id} retired: {}", turn_fault_message(&fault).await);
                                        self.unregister(ActorId(actor_id)).await;
                                        self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message }).await?;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                ShardOutcome::Turn { actor: actor_id, result: to_actor_turn_result(&result, 0, 0).await }
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
            Err(TurnFault::DeadlineExceeded) => ShardOutcome::Turn {
                actor: actor_id,
                // 👥️ `presence: Vec::new()` — a deadline-exceeded turn never finished, so there is
                // no guest-computed presence (or effects/ui_patches) to carry, unlike the two
                // wire-shape-mismatch sites this packet's report flags (`🦀️component.rs`'s
                // `execute_turn`, `⏳️runtime.rs`'s `convert_poll_success`): nothing was dropped
                // here, there was simply nothing produced.
                result: to_actor_turn_result(
                    &TurnResult {
                        ui_patches: Vec::new(),
                        effects: Vec::new(),
                        presence: Vec::new(),
                        next_wake: None,
                        status: semio_framework::kernel::TurnStatus::MoreWork,
                        fuel_used: 0,
                        command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
                    },
                    0,
                    0,
                )
                .await,
            },
            Err(fault) => ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault).await },
        };
        self.send_outcome(&outcome).await?;
        Ok(())
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
        if let Err(error) = self.validate_frame(&frame) {
            return Err(self.retain_terminal_frame(bytes, error));
        }
        if let Err(limit) = self.preflight_frame(&frame, bytes.len()) {
            let deferred_empty = self.pending_interactive.is_empty() && self.pending_background.is_empty();
            if deferred_empty {
                let byte_len = bytes.len();
                return Err(self.retain_terminal_frame(bytes, PluginHostError::Plugin(format!("ShardLoop: one frame permanently exceeds deferred {limit:?} capacity ({byte_len} bytes); exact frame retained for terminal close"))));
            }
            return Err(FrameAdmissionError::Full { limit, bytes });
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
                    serde_json::from_slice::<Event>(bytes)?;
                }
                Payload::JobStep { turn } => self.validate_job_turn(envelope.to.0, *turn)?,
                Payload::Suspend { .. } | Payload::Resume { .. } | Payload::Cancel { .. } => {}
            }
        }
        Ok(())
    }

    fn enqueue_authority(&mut self, lane: semio_framework_actor::Lane, authority: DeferredAuthority, owner_bytes: usize) -> Result<(), PluginHostError> {
        let result = if Self::is_high_priority_lane(lane) { self.pending_interactive.try_push(authority, owner_bytes) } else { self.pending_background.try_push(authority, owner_bytes) };
        if let Err(rejected) = result {
            let _ = self.terminal_authorities.try_push(rejected.owner, owner_bytes).expect("ShardLoop: terminal authority ring owns every rejected authority");
            return Err(PluginHostError::Plugin(format!("ShardLoop: release admission rejected a preflighted {lane:?} authority at {:?}", rejected.limit)));
        }
        Ok(())
    }

    fn actor_generation_is_current(&self, actor: ActorId) -> bool {
        let current = self.instances.keys().copied().find(|raw| {
            let candidate = ActorId(*raw);
            candidate.plugin_ordinal() == actor.plugin_ordinal() && candidate.kind_tag() == actor.kind_tag() && candidate.ordinal() == actor.ordinal()
        });
        current.is_none_or(|raw| raw == actor.0)
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
            Payload::Event { bytes: event_bytes } => DeferredAuthority::Event { actor, event: serde_json::from_slice(&event_bytes)? },
            Payload::JobStep { turn } => {
                self.accept_job_turn(actor, turn)?;
                DeferredAuthority::JobStep { actor, turn }
            }
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
                    self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Cancel for actor {actor_id} lost its registered instance") }).await?;
                    return Ok(());
                }
            };
            if let Err(fault) = result {
                self.unregister(ActorId(actor_id)).await;
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor cancel-job {job} failed before retirement: {}", turn_fault_message(&fault).await) }).await?;
                return Ok(());
            }
            self.running_jobs.remove(&(actor_id, job));
            self.job_turns.remove(&(actor_id, job));
            self.job_authorities.remove(&(actor_id, job));
            self.job_placement.remove(&(actor_id, job));
            let authority = DeferredAuthority::Cancel(CancelCursor { actor: actor_id, after_job: Some(job), owner_bytes: cursor.owner_bytes });
            let result = if Self::is_high_priority_lane(lane) { self.pending_interactive.try_push(authority, cursor.owner_bytes) } else { self.pending_background.try_push(authority, cursor.owner_bytes) };
            if let Err(rejected) = result {
                let _ = self.terminal_authorities.try_push(rejected.owner, cursor.owner_bytes).expect("ShardLoop: terminal authority ring owns every rejected close cursor");
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

    async fn send_outcome(&self, outcome: &ShardOutcome) -> Result<(), PluginHostError> {
        let mut bytes = Vec::new();
        outcome.pack_encode(&mut bytes).await;
        self.transport.send(&bytes).await;
        Ok(())
    }

    pub async fn heartbeat(&self) -> u64 {
        self.transport.heartbeat().await
    }
}

async fn turn_fault_message(fault: &TurnFault) -> String {
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
async fn start_job_fault_bytes(fault: &TurnFault) -> Vec<u8> {
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
/// `MockGuestRuntime` (owned by `🖥️host/🦀️component.rs`, out of this packet's edit scope, and which
/// ignores its `budget` parameter entirely), this proves the property terra-shard-grants demanded:
/// "a Grant's budget is what the turn actually executes under (prove the constants are gone, not
/// merely unused)". `pub(crate)`, not private — moved out of `mod tests` so `GuestRuntimes::
/// Recording` (`🖥️host/🦀️component.rs`) can name it.
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
        Ok(TurnResult { ui_patches: vec![], effects: vec![], presence: vec![], next_wake: None, status: semio_framework::kernel::TurnStatus::Idle, fuel_used: 0, command_ingress: semio_framework::kernel::CommandIngressStatus::Idle })
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
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// 🧪️ [`LoopbackTransport`] (module level, above — moved there so [`ShardTransports`] can name
    /// it) hands this back from `paired()`; `pub(super)` so the parent `shard` module's own
    /// `LoopbackTransport::paired` can construct it.
    pub(super) struct LoopbackProbe {
        pub(super) inbound: Arc<Mutex<Vec<Vec<u8>>>>,
        pub(super) outbound: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    impl LoopbackProbe {
        async fn push_inbound(&self, bytes: Vec<u8>) {
            self.inbound.lock().expect("loopback lock").push(bytes);
        }
        async fn take_outbound(&self) -> Vec<Vec<u8>> {
            std::mem::take(&mut *self.outbound.lock().expect("loopback lock"))
        }
    }

    /// 👶️ host-dedyn: every `ShardLoop::pump()`/`pump_primed()` call below is wrapped in
    /// `semio_framework_async::block_on` — a `#[test] fn` body is a sanctioned executor entry point
    /// (R4 clause 5); every `GuestRuntime`/`ShardTransport` impl these tests drive resolves on its
    /// first poll, so `block_on` never actually parks.
    async fn pump(shard: &mut ShardLoop) -> Result<usize, PluginHostError> {
        semio_framework_async::block_on(shard.pump())
    }

    async fn decode_outcome(bytes: &[u8]) -> ShardOutcome {
        let mut pos = 0usize;
        let outcome = ShardOutcome::pack_decode(bytes, &mut pos).await.expect("decode outcome");
        assert_eq!(pos, bytes.len());
        outcome
    }

    async fn decode_outcomes(bytes: &[Vec<u8>]) -> Vec<ShardOutcome> {
        let mut outcomes = Vec::with_capacity(bytes.len());
        for bytes in bytes {
            outcomes.push(decode_outcome(bytes).await);
        }
        outcomes
    }

    async fn encode_event_envelope(to: ActorId, seq: u64, event: &Event) -> Vec<u8> {
        encode_payload_envelope(to, seq, Payload::Event { bytes: serde_json::to_vec(event).expect("encode event") }).await
    }

    /// ✉️ Generic envelope builder for `Suspend`/`Resume`/`Cancel` payload tests —
    /// `encode_event_envelope` above stays as a thin wrapper over this so existing tests are
    /// untouched. terra-shard-grants: wraps in `ShardFrame::Envelope` — the transport now carries
    /// `ShardFrame`, not raw `Envelope` bytes, so every test-side encoder must wrap here too.
    async fn encode_payload_envelope(to: ActorId, seq: u64, payload: Payload) -> Vec<u8> {
        let envelope = Envelope { to, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload };
        let mut bytes = Vec::new();
        ShardFrame::Envelope(envelope).pack_encode(&mut bytes).await;
        bytes
    }

    fn test_job_operation(actor: ActorId, job: u64, preview_sequence: u64) -> JobOperation {
        JobOperation { operation: actor.0 ^ job.rotate_left(17), base_revision: 7, generation: 3, preview_sequence, seed: 0x5eed ^ job }
    }

    fn test_job_turn(actor: ActorId, job: u64, step_sequence: u64, preview_sequence: u64) -> JobTurn {
        JobTurn { job, operation: test_job_operation(actor, job, preview_sequence), step_sequence }
    }

    #[semio_framework_async_macros::async_test]
    async fn pump_drives_one_turn_per_actor_and_reports_it_as_a_shard_outcome() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(7);
        let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([1u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        let mut scripted = MockGuestRuntime::idle_turn().await;
        scripted.fuel_used = 42;
        mock.script_turn(actor, scripted).await;

        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose).await).await;

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);
        assert!(shard.is_registered(actor).await);

        let driven = pump(&mut shard).await.expect("pump succeeds");
        assert_eq!(driven, 1, "exactly one actor had a buffered envelope");

        let outbound = probe.take_outbound().await;
        assert_eq!(outbound.len(), 1, "one ShardOutcome sent back");
        let outcome = decode_outcome(&outbound[0]).await;
        match outcome {
            ShardOutcome::Turn { actor: reported, result } => {
                assert_eq!(reported, 7);
                assert_eq!(result.usage.fuel, 42, "the scripted turn's own fuel_used must round-trip through ShardOutcome");
            }
            other => panic!("expected ShardOutcome::Turn, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn pump_reports_an_envelope_for_an_unregistered_actor_as_a_fault_not_a_silent_drop() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let (transport, probe) = LoopbackTransport::paired().await;
        let stranger = ActorId(99);
        probe.push_inbound(encode_event_envelope(stranger, 1, &Event::InstanceClose).await).await;

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        let driven = pump(&mut shard).await.expect("pump succeeds even with an unknown actor");
        assert_eq!(driven, 0, "an envelope for an unregistered actor drives nothing");

        let outbound = probe.take_outbound().await;
        assert_eq!(outbound.len(), 1);
        let outcome = decode_outcome(&outbound[0]).await;
        assert!(matches!(outcome, ShardOutcome::Fault { actor, .. } if actor == 99), "must surface as a Fault naming the actor, not vanish");
    }

    #[semio_framework_async_macros::async_test]
    async fn unregister_drops_the_instance_and_shrinks_actor_count() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(3);
        let package = PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([2u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
        let (transport, _probe) = LoopbackTransport::paired().await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);
        assert_eq!(shard.actor_count().await, 1);
        shard.unregister(actor).await;
        assert_eq!(shard.actor_count().await, 0);
        assert!(!shard.is_registered(actor).await);
    }

    /// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1's headline acceptance test — the mechanism
    /// `📓️terra-M5-report.md` §4(a) found entirely missing: "no code anywhere reads a
    /// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ...}` and spawns/drives a job
    /// for it". Spawns a job from a scripted turn's own emitted effect, steps it across THREE
    /// separate `pump()` calls (`Running`, `Running`, `Done` — never a single-shot call, which is
    /// what actually proves the `JobBudget` mechanism resumes rather than just completing once),
    /// and observes the completion reach the ORIGINATING actor as a real `Event::JobCompleted` on
    /// a LATER `execute_turn` call — not merely that `step_job` returned `Done` in isolation.
    #[semio_framework_async_macros::async_test]
    async fn spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(21);
        let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([9u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

        let job_id = 777u64;
        let mut spawning_turn = MockGuestRuntime::idle_turn().await;
        spawning_turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: b"seed-frames".to_vec(), placement: JobPlacement::Isolated });
        mock.script_turn(actor, spawning_turn).await;
        // 🔀️ `run_job_to_completion`'s own two-arm shape (Running.../Done) but with TWO `Running`
        // steps first — the resumability proof: a job that finished on step 1 would not
        // distinguish "the budget mechanism resumed it" from "it happened to be a one-shot call".
        mock.script_job_step(actor, JobStep::Running { progress: None }).await;
        mock.script_job_step(actor, JobStep::Running { progress: Some(b"halfway".to_vec()) }).await;
        mock.script_job_step(actor, JobStep::Done { output: b"reconstruction-complete".to_vec() }).await;
        // 🔚️ Whatever `execute_turn` call eventually receives the `Event::JobCompleted` (pump 4,
        // below) still needs a scripted outcome to return — an ordinary idle turn is enough since
        // this test only asserts what EVENTS that call was given, not its own output.
        mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;

        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose).await).await;
        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: test_job_turn(actor, job_id, 0, 0) }).await).await;

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        // Pump 1: runs the spawning turn, admits `Effect::SpawnJob` (`start_job`), and — because
        // the job lands in `running_jobs` before the step phase runs — takes its FIRST step in
        // this SAME pump (`Running`).
        let driven1 = pump(&mut shard).await.expect("pump 1");
        assert_eq!(driven1, 2, "one turn (the spawn) plus one job step (the first Running) this pump");

        probe.push_inbound(encode_payload_envelope(actor, 3, Payload::JobStep { turn: test_job_turn(actor, job_id, 1, 0) }).await).await;
        let driven2 = pump(&mut shard).await.expect("pump 2");
        assert_eq!(driven2, 1, "only the second Running step — no envelope, so no turn this pump");
        probe.push_inbound(encode_payload_envelope(actor, 4, Payload::JobStep { turn: test_job_turn(actor, job_id, 2, 1) }).await).await;
        let driven3 = pump(&mut shard).await.expect("pump 3");
        assert_eq!(driven3, 1, "the terminal Done step");

        // Pump 4: still no new envelope — but the Done step queued an `Event::JobCompleted` for
        // delivery, so the actor is driven ONE more time purely to receive it.
        let driven4 = pump(&mut shard).await.expect("pump 4");
        assert_eq!(driven4, 1, "the queued completion drives one more turn, with no job left to step");

        let outbound = probe.take_outbound().await;
        let outcomes = decode_outcomes(&outbound).await;
        let job_outcomes: Vec<&JobStepOutcome> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Job { publication, .. } if publication.turn.job == job_id => Some(&publication.outcome),
                _ => None,
            })
            .collect();
        let authorities: Vec<(JobTurn, JobTurn)> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Job { authority, publication, .. } if publication.turn.job == job_id => Some((*authority, publication.turn)),
                _ => None,
            })
            .collect();
        assert_eq!(authorities.len(), 3);
        assert!(authorities.iter().all(|(authority, publication)| {
            authority.job == job_id
                && authority.step_sequence == 0
                && authority.operation.preview_sequence == 0
                && authority.operation.operation == authorities[0].0.operation.operation
                && authority.operation.operation == publication.operation.operation
                && authority.operation.base_revision == publication.operation.base_revision
                && authority.operation.generation == publication.operation.generation
        }));
        assert_eq!(job_outcomes.len(), 3, "exactly three step_job calls were made — the resumability proof");
        assert!(matches!(job_outcomes[0], JobStepOutcome::Yield));
        assert!(matches!(job_outcomes[1], JobStepOutcome::PreviewReady { preview } if preview == b"halfway"));
        assert!(matches!(job_outcomes[2], JobStepOutcome::Complete { candidate } if candidate.output == b"reconstruction-complete"));

        // 🎯️ The actual end-to-end proof: the ORIGINATING actor's `execute_turn` was, at some
        // point, handed a real `Event::JobCompleted{job: 777, result: Ok(..)}` — not merely that
        // `step_job` internally returned `Done` (`job_outcomes` above already showed that; this is
        // the part M5 found completely missing: nothing delivered it back).
        let completed = mock.observed_events(actor).await.into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));
        match completed {
            Some(Event::JobCompleted { result: RequestOutcome::Ok(bytes), .. }) => {
                assert_eq!(bytes, b"reconstruction-complete", "the Done step's own bytes must round-trip into the delivered completion event");
            }
            other => panic!("expected Event::JobCompleted{{job: 777, result: Ok(..)}} to have reached the originating actor's execute_turn, got {other:?}"),
        }
    }

    /// 🛑️ A successful `Effect::CancelJob` removes the job in the same turn, before step.
    #[semio_framework_async_macros::async_test]
    async fn cancel_job_effect_stops_a_job_before_it_is_ever_stepped() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(22);
        let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([10u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

        let job_id = 888u64;
        let mut turn = MockGuestRuntime::idle_turn().await;
        turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        turn.effects.push(Effect::CancelJob { job: job_id });
        mock.script_turn(actor, turn).await;

        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose).await).await;
        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: test_job_turn(actor, job_id, 0, 0) }).await).await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        let driven = pump(&mut shard).await.expect("pump");
        assert_eq!(driven, 1, "only the turn itself — the job was cancelled before the step phase, so no step_job call happened");

        let outbound = probe.take_outbound().await;
        let outcomes = decode_outcomes(&outbound).await;
        assert!(!outcomes.iter().any(|outcome| matches!(outcome, ShardOutcome::Job { .. })), "a cancelled-before-first-step job must never produce a ShardOutcome::Job");
    }

    #[semio_framework_async_macros::async_test]
    async fn cancel_job_effect_failure_retires_the_actor_and_surfaces_the_typed_fault() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(23);
        let package = PackageRef { package: PackageId("remodel-cancel-failure".to_string()), hash: PackageHash([11u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        let job_id = 889u64;
        let mut turn = MockGuestRuntime::idle_turn().await;
        turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        turn.effects.push(Effect::CancelJob { job: job_id });
        mock.script_turn(actor, turn).await;
        mock.fail_next_cancel();

        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose).await).await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        assert_eq!(pump(&mut shard).await.expect("pump failed cancellation"), 1);
        assert!(!shard.is_registered(actor).await, "a failed hot-shard cancellation must retire the uncertain actor instance");
        assert!(!shard.running_jobs.contains(&(actor.0, job_id)));
        assert_eq!(mock.cancel_admissions(), 1);
        assert_eq!(mock.step_admissions(), 0, "the retired actor's job must never enter step-job");
        let outcomes = decode_outcomes(&probe.take_outbound().await).await;
        assert!(matches!(
            outcomes.as_slice(),
            [ShardOutcome::Fault { actor: reported, message }]
                if *reported == actor.0
                    && message == "ShardLoop::pump: cancel-job 889 failed; actor 23 retired: guest trapped: scripted cancel-job failure"
        ));

        probe.push_inbound(encode_event_envelope(actor, 2, &Event::InstanceClose).await).await;
        assert_eq!(pump(&mut shard).await.expect("post-retirement pump"), 0);
        assert_eq!(mock.cancel_admissions(), 1, "retirement must not retry cancellation");
        assert!(matches!(decode_outcomes(&probe.take_outbound().await).await.as_slice(), [ShardOutcome::Fault { actor: reported, .. }] if *reported == actor.0));
    }

    //#region 🔖️K1SuspendResumePlacement
    /// 📸️ `Payload::Suspend` dispatches to `GuestRuntime::checkpoint` and surfaces its bytes
    /// with the explicit operation identity and applied progress boundary.
    #[semio_framework_async_macros::async_test]
    async fn suspend_with_checkpoint_true_surfaces_checkpoint_bytes_in_the_outcome() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(31);
        let package = PackageRef { package: PackageId("suspend".to_string()), hash: PackageHash([5u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");

        let (transport, probe) = LoopbackTransport::paired().await;
        let operation = test_job_operation(actor, 0, 0);
        probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { operation, applied_progress: 73 }).await).await;

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        let driven = pump(&mut shard).await.expect("pump");
        assert_eq!(driven, 0, "Suspend is handled entirely in the drain loop, not the turn/step phases");

        let outbound = probe.take_outbound().await;
        assert_eq!(outbound.len(), 1);
        let outcome = decode_outcome(&outbound[0]).await;
        match outcome {
            ShardOutcome::Checkpoint { actor: reported, operation: reported_operation, checkpoint } => {
                assert_eq!(reported, 31);
                assert_eq!(reported_operation, operation);
                assert_eq!(checkpoint.state, b"mock-checkpoint:31".to_vec(), "MockGuestRuntime::checkpoint's own deterministic bytes must round-trip unmodified");
                assert_eq!(checkpoint.applied_progress, 73);
            }
            other => panic!("expected ShardOutcome::Checkpoint, got {other:?}"),
        }
    }

    /// 🎯️ Bench budget #7's "identical state hash" property: the EXACT bytes a `Suspend{checkpoint:
    /// true}` checkpoint produced, carried through a `Resume{checkpoint: Some(bytes)}` envelope,
    /// must be the bytes `GuestRuntime::restore` is called with — verified by reaching into the
    /// restored `GuestInstance`'s own mock state (this file is a descendant module of the host
    /// crate root that defines `GuestInstanceState`, so the private field is visible here) rather
    /// than trusting `Resumed` alone, since `MockGuestRuntime::restore` would return `Ok(())` for
    /// ANY bytes.
    #[semio_framework_async_macros::async_test]
    async fn suspend_then_resume_round_trips_byte_identical_checkpoint_state() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(32);
        let package = PackageRef { package: PackageId("suspend-resume".to_string()), hash: PackageHash([6u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");

        let (transport, probe) = LoopbackTransport::paired().await;
        let operation = test_job_operation(actor, 0, 0);
        probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { operation, applied_progress: 19 }).await).await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);
        pump(&mut shard).await.expect("pump suspend");

        let suspend_outbound = probe.take_outbound().await;
        let checkpoint = match decode_outcome(&suspend_outbound[0]).await {
            ShardOutcome::Checkpoint { checkpoint, .. } => checkpoint,
            other => panic!("expected ShardOutcome::Checkpoint, got {other:?}"),
        };

        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::Resume { operation, checkpoint: checkpoint.clone() }).await).await;
        pump(&mut shard).await.expect("pump resume");

        let resume_outbound = probe.take_outbound().await;
        let resume_outcome = decode_outcome(&resume_outbound[0]).await;
        assert!(matches!(resume_outcome, ShardOutcome::Resumed { actor: reported, operation: reported_operation } if reported == 32 && reported_operation == operation));

        let instance = shard.instances.get(&actor.0).expect("Resume must not drop the instance");
        let GuestInstanceState::Mock(mock_state) = &instance.state else { panic!("expected a Mock instance") };
        assert_eq!(mock_state.checkpoint.as_deref(), Some(checkpoint.state.as_slice()), "restore must have been called with the EXACT bytes checkpoint produced");
    }

    /// 🛑️ `Payload::Cancel` must cancel every one of the actor's `running_jobs` (via
    /// `GuestRuntime::cancel_job`) and unregister its instance — after which no further `step_job`
    /// call for that job can ever happen, since the (actor, job) pair no longer exists in
    /// `running_jobs` and the actor itself is no longer registered.
    #[semio_framework_async_macros::async_test]
    async fn cancel_unregisters_the_instance_and_no_further_step_job_happens() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(41);
        let package = PackageRef { package: PackageId("cancel-payload".to_string()), hash: PackageHash([7u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

        let job_id = 555u64;
        let mut turn = MockGuestRuntime::idle_turn().await;
        turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        mock.script_turn(actor, turn).await;
        mock.script_job_step(actor, JobStep::Running { progress: None }).await;

        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose).await).await;
        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: test_job_turn(actor, job_id, 0, 0) }).await).await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        let driven1 = pump(&mut shard).await.expect("pump 1");
        assert_eq!(driven1, 2, "the spawning turn plus the job's first (only scripted) step");
        probe.take_outbound().await;

        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::Cancel { seq: 0 }).await).await;
        let driven2 = pump(&mut shard).await.expect("pump 2 (cancel)");
        assert_eq!(driven2, 0, "Cancel is handled in the drain loop; the actor is unregistered before the turn/step phases run");
        assert!(!shard.is_registered(actor).await, "Cancel must unregister the actor's instance");
        assert_eq!(shard.actor_count().await, 0);

        let outbound2 = probe.take_outbound().await;
        assert_eq!(outbound2.len(), 1);
        let outcome = decode_outcome(&outbound2[0]).await;
        assert!(matches!(outcome, ShardOutcome::Cancelled { actor: reported } if reported == 41));

        // 🎯️ A third pump proves the job is truly dead: if `running_jobs` still held it, `step_job`
        // would be called again with an EMPTY scripted queue and fault loudly (`TurnFault::
        // Exhausted`) rather than silently succeeding — no such outcome appears.
        let driven3 = pump(&mut shard).await.expect("pump 3");
        assert_eq!(driven3, 0, "nothing left to drive: no envelopes, no running_jobs, no registered instance");
        assert!(probe.take_outbound().await.is_empty(), "no further outcome of any kind for the cancelled job");
    }

    #[semio_framework_async_macros::async_test]
    async fn actor_cancel_failure_retires_the_instance_and_reports_fault_instead_of_cancelled() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(42);
        let package = PackageRef { package: PackageId("cancel-payload-failure".to_string()), hash: PackageHash([8u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        let job_id = 556u64;
        let mut turn = MockGuestRuntime::idle_turn().await;
        turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        mock.script_turn(actor, turn).await;
        mock.script_job_step(actor, JobStep::Running { progress: None }).await;

        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose).await).await;
        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: test_job_turn(actor, job_id, 0, 0) }).await).await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);
        assert_eq!(pump(&mut shard).await.expect("pump live job"), 2);
        probe.take_outbound().await;

        mock.fail_next_cancel();
        probe.push_inbound(encode_payload_envelope(actor, 3, Payload::Cancel { seq: 0 }).await).await;
        assert_eq!(pump(&mut shard).await.expect("pump failed actor cancel"), 0);
        assert!(!shard.is_registered(actor).await);
        assert_eq!(mock.cancel_admissions(), 1);
        assert_eq!(mock.step_admissions(), 1, "retirement must prevent any later guest step");
        let outcomes = decode_outcomes(&probe.take_outbound().await).await;
        assert!(matches!(
            outcomes.as_slice(),
            [ShardOutcome::Fault { actor: reported, message }]
                if *reported == actor.0
                    && message == "ShardLoop::pump: actor cancel-job 556 failed before retirement: guest trapped: scripted cancel-job failure"
        ));
        assert!(!outcomes.iter().any(|outcome| matches!(outcome, ShardOutcome::Cancelled { .. })), "failed actor cancellation must never claim success");
    }

    /// 🚦 `JobPlacement::Exclusive` must be honoured rather than silently ignored: an `Exclusive`
    /// job admitted in the SAME pump as an `Inline` one is stepped FIRST — the shard-local routing
    /// this packet adds (see `to_step`'s own doc comment for why this is the honest in-shard-only
    /// approximation, not cross-shard dedicated placement).
    #[semio_framework_async_macros::async_test]
    async fn exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(51);
        let package = PackageRef { package: PackageId("placement".to_string()), hash: PackageHash([8u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");

        let inline_job = 61u64;
        let exclusive_job = 62u64;
        let mut turn = MockGuestRuntime::idle_turn().await;
        // 🔀️ Inline is pushed FIRST in spawn order, so a passing test proves the sort actually
        // reorders by placement rather than merely preserving admission order by coincidence.
        turn.effects.push(Effect::SpawnJob { job: inline_job, kind: "a".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        turn.effects.push(Effect::SpawnJob { job: exclusive_job, kind: "b".to_string(), input: Vec::new(), placement: JobPlacement::Exclusive });
        mock.script_turn(actor, turn).await;
        // 🧲️ The terminal exclusive step frees the actor slot so the inline step runs next.
        mock.script_job_step(actor, JobStep::Done { output: vec![6, 2] }).await;
        mock.script_job_step(actor, JobStep::Running { progress: None }).await;

        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose).await).await;
        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::JobStep { turn: test_job_turn(actor, inline_job, 0, 0) }).await).await;
        probe.push_inbound(encode_payload_envelope(actor, 3, Payload::JobStep { turn: test_job_turn(actor, exclusive_job, 0, 0) }).await).await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        let driven = pump(&mut shard).await.expect("pump");
        assert_eq!(driven, 2, "one actor turn plus exactly one bounded job step");
        assert_eq!(pump(&mut shard).await.expect("second pump"), 1, "the second job steps on the next actor turn");

        let outbound = probe.take_outbound().await;
        let outcomes = decode_outcomes(&outbound).await;
        let job_order: Vec<u64> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Job { publication, .. } => Some(publication.turn.job),
                _ => None,
            })
            .collect();
        assert_eq!(job_order, vec![exclusive_job, inline_job], "the Exclusive-placed job must be stepped before the Inline one despite being spawned second");
    }
    //#endregion 🔖️K1SuspendResumePlacement

    //#region 🔖️ShardFrameRoundTrips
    /// 🎯️ terra-shard-grants requirement: every `ShardFrame` variant round-trips through the pack
    /// codec, INCLUDING the `Envelope` passthrough — the variant the packet brief explicitly says
    /// must not be removed as redundant.
    macro_rules! shard_frame_round_trip {
        ($name:ident, $value:expr) => {
            #[semio_framework_async_macros::async_test]
            async fn $name() {
                let value: ShardFrame = $value;
                let mut bytes = Vec::new();
                value.pack_encode(&mut bytes).await;
                let mut pos = 0usize;
                let decoded = ShardFrame::pack_decode(&bytes, &mut pos).await.expect("pack_decode");
                assert_eq!(pos, bytes.len(), "pack_decode must consume exactly what pack_encode wrote");
                assert_eq!(decoded, value);
            }
        };
    }

    shard_frame_round_trip!(shard_frame_round_trip_register, ShardFrame::Register { actor: ActorId(7) });
    shard_frame_round_trip!(shard_frame_round_trip_unregister, ShardFrame::Unregister { actor: ActorId(9) });
    shard_frame_round_trip!(
        shard_frame_round_trip_grant,
        ShardFrame::Grant {
            actor: ActorId(11),
            budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive),
            envelopes: vec![Envelope {
                to: ActorId(11),
                from: semio_framework_actor::Origin::Kernel,
                lane: semio_framework_actor::Lane::Interactive,
                seq: 1,
                deadline_ms: None,
                coalesce: None,
                cancel_of: None,
                payload: Payload::Event { bytes: vec![1, 2, 3] },
            }],
        }
    );
    shard_frame_round_trip!(
        shard_frame_round_trip_envelope_passthrough,
        ShardFrame::Envelope(Envelope { to: ActorId(13), from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Background, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Cancel { seq: 4 } })
    );

    #[semio_framework_async_macros::async_test]
    async fn shard_outcome_owned_pack_round_trips_every_variant() {
        let operation = test_job_operation(ActorId(11), 1, 0);
        let turn = test_job_turn(ActorId(11), 44, 0, 0);
        let values = vec![
            ShardOutcome::Turn {
                actor: 11,
                result: semio_framework_actor::TurnResult {
                    ui_patches: vec![1],
                    effects: vec![2],
                    command_ingress: vec![3],
                    next_wake: Some(3),
                    status: semio_framework_actor::TurnStatus::MoreWork,
                    usage: semio_framework_actor::Usage { fuel: 4, wall_us: 5, memory_bytes: 6 },
                },
            },
            ShardOutcome::Job { actor: 11, authority: turn, publication: JobPublication { turn, outcome: JobStepOutcome::PreviewReady { preview: vec![7] } } },
            ShardOutcome::Fault { actor: 11, message: "fault".to_string() },
            ShardOutcome::Checkpoint { actor: 11, operation, checkpoint: JobCheckpoint { state: vec![8], applied_progress: 9 } },
            ShardOutcome::Resumed { actor: 11, operation },
            ShardOutcome::Cancelled { actor: 11 },
        ];
        for value in values {
            let mut bytes = Vec::new();
            value.pack_encode(&mut bytes).await;
            assert_eq!(decode_outcome(&bytes).await, value);
        }
    }

    /// 🎯️ A `Grant` with ZERO bundled envelopes must still record its budget — proven separately
    /// from the "budget actually executes under" test below, which needs at least one envelope to
    /// drive a turn.
    #[semio_framework_async_macros::async_test]
    async fn grant_with_no_envelopes_still_records_the_budget() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(61);
        let package = PackageRef { package: PackageId("grant-empty".to_string()), hash: PackageHash([20u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
        let (transport, probe) = LoopbackTransport::paired().await;
        let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        budget.fuel = 123_456;
        let mut bytes = Vec::new();
        ShardFrame::Grant { actor, budget, envelopes: vec![] }.pack_encode(&mut bytes).await;
        probe.push_inbound(bytes).await;

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);
        let driven = pump(&mut shard).await.expect("pump");
        assert_eq!(driven, 0, "no envelopes bundled — nothing to drive yet");
        assert_eq!(shard.granted_budget(actor.0).await.fuel, 123_456, "the Grant's budget must be recorded even with no envelopes");
    }
    //#endregion 🔖️ShardFrameRoundTrips

    //#region 🔖️GrantBudgetExecution
    // 👶️ host-dedyn: `RecordingRuntime` moved to module level (above `mod tests`) so
    // `GuestRuntimes::Recording` (`🖥️host/🦀️component.rs`) can name it — see that region's own doc.

    /// 🎯️ Headline property test for terra-shard-grants Part B: a `Grant`'s budget — NOT any
    /// leftover constant, since `TURN_BUDGET`/`JOB_STEP_BUDGET` are deleted from this file entirely
    /// — is what `execute_turn` actually receives. Two DIFFERENT `Grant`s (deliberately distinct
    /// `fuel` values) prove the budget travels PER-GRANT, not a single hardcoded number that would
    /// happen to match by coincidence.
    #[semio_framework_async_macros::async_test]
    async fn a_grants_budget_is_what_the_turn_actually_executes_under() {
        let runtime = Arc::new(RecordingRuntime::new().await);
        let actor = ActorId(71);
        let package = PackageRef { package: PackageId("grant-budget".to_string()), hash: PackageHash([21u8; 32]) };
        let compiled = runtime.compile(&package, &[]).await.expect("compile");
        let instance = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");

        let (transport, probe) = LoopbackTransport::paired().await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        let mut first_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        first_budget.fuel = 111_111;
        let envelope = Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() },
        };
        let mut bytes = Vec::new();
        ShardFrame::Grant { actor, budget: first_budget, envelopes: vec![envelope] }.pack_encode(&mut bytes).await;
        probe.push_inbound(bytes).await;
        pump(&mut shard).await.expect("pump 1");
        assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called").fuel, 111_111, "the FIRST Grant's own fuel must reach execute_turn, not a constant");

        let mut second_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background);
        second_budget.fuel = 222_222;
        let envelope2 = Envelope {
            to: actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 2,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() },
        };
        let mut bytes2 = Vec::new();
        ShardFrame::Grant { actor, budget: second_budget, envelopes: vec![envelope2] }.pack_encode(&mut bytes2).await;
        probe.push_inbound(bytes2).await;
        pump(&mut shard).await.expect("pump 2");
        assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called again").fuel, 222_222, "a DIFFERENT second Grant's fuel must reach execute_turn too — proving it travels per-Grant, not a fixed constant");

        let _ = probe.take_outbound().await;
    }

    /// 🎯️ Same property for job stepping: `step_job`'s `JobBudget` comes from the SAME actor's last
    /// granted budget (point 2 of the brief: "job steps take the owning actor's last granted budget
    /// on the Maintenance lane").
    #[semio_framework_async_macros::async_test]
    async fn job_step_uses_the_owning_actors_last_granted_budget() {
        let runtime = Arc::new(RecordingRuntime::new().await);
        let actor = ActorId(72);
        let package = PackageRef { package: PackageId("grant-job-budget".to_string()), hash: PackageHash([22u8; 32]) };
        let compiled = runtime.compile(&package, &[]).await.expect("compile");
        let instance = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("instantiate");

        let (transport, probe) = LoopbackTransport::paired().await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);

        let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance);
        budget.fuel = 333_333;
        let mut bytes = Vec::new();
        ShardFrame::Grant { actor, budget, envelopes: vec![] }.pack_encode(&mut bytes).await;
        probe.push_inbound(bytes).await;
        // 🔀️ An explicit `JobStep` re-arming, not a `SpawnJob` effect — simplest way to reach the
        // step phase without depending on `RecordingRuntime::execute_turn`'s effects (it always
        // returns none).
        let job_bytes = {
            let envelope = Envelope {
                to: actor,
                from: semio_framework_actor::Origin::Kernel,
                lane: semio_framework_actor::Lane::Maintenance,
                seq: 2,
                deadline_ms: None,
                coalesce: None,
                cancel_of: None,
                payload: Payload::JobStep { turn: test_job_turn(actor, 999, 0, 0) },
            };
            let mut out = Vec::new();
            ShardFrame::Envelope(envelope).pack_encode(&mut out).await;
            out
        };
        probe.push_inbound(job_bytes).await;

        pump(&mut shard).await.expect("pump");
        assert_eq!(runtime.last_job_budget.lock().unwrap().expect("step_job must have been called").fuel, 333_333, "step_job must run under the SAME actor's last granted budget, not a deleted JOB_STEP_BUDGET constant");
        let _ = probe.take_outbound().await;
    }

    /// 🎯️ An actor that was NEVER granted a budget still gets a real, principled one (the
    /// Maintenance lane's default) — never a panic, never a zeroed budget.
    #[semio_framework_async_macros::async_test]
    async fn an_actor_never_granted_a_budget_falls_back_to_the_maintenance_lane_default() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(73);
        let shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(LoopbackTransport::default())).await;
        let expected = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance);
        assert_eq!(shard.granted_budget(actor.0).await, expected);
    }
    //#endregion 🔖️GrantBudgetExecution

    //#region 🔖️RegisterUnregisterFrames
    /// 🛑️ An incoming `ShardFrame::Unregister` must unregister the actor exactly like calling
    /// [`ShardLoop::unregister`] directly — real behavior, unlike `Register`'s wire-symmetry-only
    /// role (see that variant's own doc).
    #[semio_framework_async_macros::async_test]
    async fn unregister_frame_drops_the_instance_exactly_like_the_direct_call() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(81);
        let package = PackageRef { package: PackageId("unreg-frame".to_string()), hash: PackageHash([23u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).await.expect("mock instantiate");
        let (transport, probe) = LoopbackTransport::paired().await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);
        assert!(shard.is_registered(actor).await);

        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes).await;
        probe.push_inbound(bytes).await;
        let driven = pump(&mut shard).await.expect("pump");
        assert_eq!(driven, 0, "Unregister is handled entirely in the drain loop");
        assert!(!shard.is_registered(actor).await, "an incoming Unregister frame must drop the instance");
    }

    /// 📌️ `Register` is decoded without error but has no LOCAL side effect (see its own doc) — this
    /// proves `pump` does not choke on it or mistake it for anything else.
    #[semio_framework_async_macros::async_test]
    async fn register_frame_is_accepted_without_error_and_has_no_local_side_effect() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(82);
        let (transport, probe) = LoopbackTransport::paired().await;
        let mut bytes = Vec::new();
        ShardFrame::Register { actor }.pack_encode(&mut bytes).await;
        probe.push_inbound(bytes).await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport)).await;
        let driven = pump(&mut shard).await.expect("pump must not error on a Register frame");
        assert_eq!(driven, 0);
        assert!(!shard.is_registered(actor).await, "Register never instantiates locally — see its own doc");
    }
    //#endregion 🔖️RegisterUnregisterFrames

    //#region 🔖️BudgetBridge
    #[semio_framework_async_macros::async_test]
    async fn to_actor_turn_result_maps_status_and_carries_host_measured_usage() {
        let kernel_result = TurnResult {
            ui_patches: vec![],
            effects: vec![],
            presence: vec![],
            next_wake: Some(42),
            status: semio_framework::kernel::TurnStatus::Faulted(b"trap".to_vec()),
            fuel_used: 999,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let bridged = to_actor_turn_result(&kernel_result, 1234, 5678).await;
        assert_eq!(bridged.next_wake, Some(42));
        assert_eq!(bridged.status, semio_framework_actor::TurnStatus::Faulted { detail: b"trap".to_vec() }, "status must map 1:1, and the actor crate's struct-variant Faulted (Part A) must be what this bridge constructs");
        assert_eq!(bridged.usage.fuel, 999, "usage.fuel comes from the kernel TurnResult's own fuel_used");
        assert_eq!(bridged.usage.wall_us, 1234, "wall_us is host-measured, passed straight through");
        assert_eq!(bridged.usage.memory_bytes, 5678, "memory_bytes is host-measured, passed straight through");
    }

    #[semio_framework_async_macros::async_test]
    async fn to_actor_turn_result_status_maps_idle_more_work_and_checkpoint_ready() {
        let checkpoint = JobCheckpoint { state: vec![4, 2], applied_progress: 9 };
        for (kernel_status, expected) in [
            (semio_framework::kernel::TurnStatus::Idle, semio_framework_actor::TurnStatus::Idle),
            (semio_framework::kernel::TurnStatus::MoreWork, semio_framework_actor::TurnStatus::MoreWork),
            (semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() }, semio_framework_actor::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() }),
        ] {
            let kernel_result = TurnResult { ui_patches: vec![], effects: vec![], presence: vec![], next_wake: None, status: kernel_status, fuel_used: 0, command_ingress: semio_framework::kernel::CommandIngressStatus::Idle };
            assert_eq!(to_actor_turn_result(&kernel_result, 0, 0).await.status, expected);
        }
    }
    //#endregion 🔖️BudgetBridge

    //#region 🔖️LanePriorityAndEpochYield
    /// 🎯️ terra-shard-lane piece 1's headline acceptance test — the mechanism, proven directly
    /// against `ShardLoop::pump()` without needing the full native bench: a shard already loaded
    /// with several Background-lane grants, PLUS an Interactive-lane grant for a different actor,
    /// all bundled into the SAME `pump()` call (`grants_per_tick` covering all of them in one kernel
    /// tick, exactly the scenario `📓️terra-shard-lane-report.md` diagnosed as head-of-line
    /// blocking). Every background actor's scripted turn is queued FIRST on the transport — the
    /// worst case for the OLD FIFO/HashMap-iteration-order pump — so a passing assertion that the
    /// interactive actor's `ShardOutcome::Turn` is the FIRST one sent proves the two-queue
    /// reordering actually reorders, not merely that it happens not to break the happy path.
    #[semio_framework_async_macros::async_test]
    async fn an_interactive_grant_is_executed_before_background_grants_queued_the_same_pump() {
        const BACKGROUND_ACTORS: u64 = 5;
        let mock = Arc::new(MockGuestRuntime::new().await);
        let (transport, probe) = LoopbackTransport::paired().await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;

        let package = PackageRef { package: PackageId("lane-priority".to_string()), hash: PackageHash([90u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let background_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background);

        // 🚦 Background actors first — queued on the wire ahead of the interactive one, the exact
        // arrival order that used to win under plain FIFO/HashMap-iteration-order draining.
        for offset in 0..BACKGROUND_ACTORS {
            let actor = ActorId(200 + offset);
            let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
            shard.register(actor, instance);
            mock.script_turn(actor, MockGuestRuntime::idle_turn().await).await;
            let envelope = Envelope {
                to: actor,
                from: semio_framework_actor::Origin::Kernel,
                lane: semio_framework_actor::Lane::Background,
                seq: 1,
                deadline_ms: None,
                coalesce: None,
                cancel_of: None,
                payload: Payload::Event { bytes: serde_json::to_vec(&Event::InstanceClose).expect("encode") },
            };
            let mut bytes = Vec::new();
            ShardFrame::Grant { actor, budget: background_budget, envelopes: vec![envelope] }.pack_encode(&mut bytes).await;
            probe.push_inbound(bytes).await;
        }

        // 🚦 The interactive actor's own grant, queued LAST.
        let interactive_actor = ActorId(999);
        let interactive_instance = mock.instantiate(&compiled, interactive_actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        shard.register(interactive_actor, interactive_instance);
        let mut interactive_turn = MockGuestRuntime::idle_turn().await;
        interactive_turn.fuel_used = 4242;
        mock.script_turn(interactive_actor, interactive_turn).await;
        let interactive_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        let interactive_envelope = Envelope {
            to: interactive_actor,
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Interactive,
            seq: 1,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Event { bytes: serde_json::to_vec(&Event::InstanceClose).expect("encode") },
        };
        let mut interactive_bytes = Vec::new();
        ShardFrame::Grant { actor: interactive_actor, budget: interactive_budget, envelopes: vec![interactive_envelope] }.pack_encode(&mut interactive_bytes).await;
        probe.push_inbound(interactive_bytes).await;

        let driven = pump(&mut shard).await.expect("pump");
        assert_eq!(driven, BACKGROUND_ACTORS as usize + 1, "one turn per actor, background plus interactive");

        let outbound = probe.take_outbound().await;
        assert_eq!(outbound.len(), BACKGROUND_ACTORS as usize + 1);
        let outcomes = decode_outcomes(&outbound).await;
        match &outcomes[0] {
            ShardOutcome::Turn { actor, result } => {
                assert_eq!(*actor, interactive_actor.0, "the Interactive-lane grant must be the FIRST ShardOutcome sent, despite every Background-lane grant having been queued on the wire BEFORE it");
                assert_eq!(result.usage.fuel, 4242, "must be the interactive actor's own scripted turn, not a background one that happens to share a position");
            }
            other => panic!("expected the first outcome to be the interactive actor's Turn, got {other:?}"),
        }
    }

    /// 🎯️ terra-shard-lane piece 2: a turn that raises `TurnFault::DeadlineExceeded` (what
    /// `WasmtimeRuntime::execute_turn` raises when the epoch deadline armed from
    /// `budget.deadline_ms` is hit — `📓️terra-shard-lane-report.md`) must surface as a graceful
    /// `ShardOutcome::Turn{status: MoreWork}`, NEVER `ShardOutcome::Fault` — and the actor must stay
    /// registered on the shard, ready to be re-granted on a later tick, exactly as a real epoch
    /// interrupt (which lands at a wasm-bytecode safe point and leaves the `Store` usable) allows.
    #[semio_framework_async_macros::async_test]
    async fn a_turn_that_hits_its_epoch_deadline_yields_more_work_not_a_fault_and_stays_registered() {
        let mock = Arc::new(MockGuestRuntime::new().await);
        let actor = ActorId(91);
        let package = PackageRef { package: PackageId("epoch-yield".to_string()), hash: PackageHash([91u8; 32]) };
        let compiled = mock.compile(&package, &[]).await.expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 2, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
        mock.script_deadline_exceeded(actor).await;

        let (transport, probe) = LoopbackTransport::paired().await;
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport)).await;
        shard.register(actor, instance);
        probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Event { bytes: serde_json::to_vec(&Event::InstanceClose).expect("encode") }).await).await;

        let driven = pump(&mut shard).await.expect("pump");
        assert_eq!(driven, 1);

        let outbound = probe.take_outbound().await;
        assert_eq!(outbound.len(), 1);
        let outcome = decode_outcome(&outbound[0]).await;
        match outcome {
            ShardOutcome::Turn { actor: reported, result } => {
                assert_eq!(reported, 91);
                assert_eq!(result.status, semio_framework_actor::TurnStatus::MoreWork, "a deadline-exceeded turn must yield MoreWork, not surface as a Fault the kernel would escalate/quarantine the actor for");
            }
            other => panic!("expected ShardOutcome::Turn{{status: MoreWork}}, got {other:?} — DeadlineExceeded must never become a Fault"),
        }
        assert!(shard.is_registered(actor).await, "the actor's instance must stay registered — an epoch interrupt does not lose state");
    }

    #[test]
    fn fixed_owner_ring_hands_back_items_and_bytes_at_the_exact_boundary() {
        let mut items = FixedOwnerRing::<u64, 2>::new(16);
        items.try_push(11, 8).expect("first exact owner");
        items.try_push(12, 8).expect("item and byte caps exactly full");
        let rejected = items.try_push(13, 0).expect_err("capacity plus one");
        assert_eq!(rejected.limit, AdmissionLimit::Items);
        assert_eq!(rejected.owner, 13, "the exact rejected owner is handed back");
        assert_eq!(items.pop_front().map(|(_, owner)| owner), Some(11));
        assert_eq!(items.len(), 1, "one scheduling grant pops one FIFO owner and leaves the next retained");

        let mut bytes = FixedOwnerRing::<u64, 2>::new(8);
        bytes.try_push(21, 8).expect("exact byte cap");
        let rejected = bytes.try_push(22, 1).expect_err("byte cap plus one");
        assert_eq!(rejected.limit, AdmissionLimit::Bytes);
        assert_eq!(rejected.owner, 22, "bytes plus one returns the exact authority");
    }

    #[test]
    fn fixed_owner_ring_generation_rejects_an_aba_key_after_slot_reuse() {
        let mut ring = FixedOwnerRing::<u64, 1>::new(8);
        let stale = ring.try_push(31, 1).expect("first generation");
        assert_eq!(ring.pop_front().map(|(_, owner)| owner), Some(31));
        let current = ring.try_push(32, 1).expect("reused physical slot");
        assert!(!ring.contains(stale), "an ABA-stale generation must not address current work");
        assert!(ring.contains(current));
    }

    #[test]
    fn interrupted_close_ring_releases_one_authority_per_grant() {
        let mut closes = FixedOwnerRing::<CancelCursor, 2>::new(2 * size_of::<CancelCursor>());
        closes.try_push(CancelCursor { actor: 41, after_job: None, owner_bytes: size_of::<CancelCursor>() }, size_of::<CancelCursor>()).expect("first close");
        closes.try_push(CancelCursor { actor: 42, after_job: None, owner_bytes: size_of::<CancelCursor>() }, size_of::<CancelCursor>()).expect("second close");
        assert!(closes.pop_front().is_some());
        assert_eq!(closes.len(), 1, "one scheduling grant releases exactly one close authority");
    }

    #[semio_framework_async_macros::async_test]
    async fn malformed_frame_terminalizes_once_without_self_resubmission_readiness() {
        let raw = vec![0xff, 0x7f, 0x01];
        let (transport, probe) = LoopbackTransport::paired().await;
        probe.push_inbound(raw.clone()).await;
        let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
        let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport)).await;

        match shard.drive_one().await {
            ShardDrive::Fault { consumed_epoch: Some(1), work_remains: false, terminal_frame: true, .. } => {}
            _ => panic!("malformed ingress must consume epoch one and terminalize without retained work"),
        }
        assert_eq!(shard.take_terminal_frame(), Some(raw), "terminal retrieval transfers the exact raw frame");
        assert!(shard.take_terminal_frame().is_none(), "one terminal owner is retrieved exactly once");
        assert!(!shard.has_pending_work(), "terminal ownership is observable, not a hot-resubmit readiness condition");
    }

    #[semio_framework_async_macros::async_test]
    async fn terminal_capacity_plus_one_parks_then_rearms_once_at_the_fifo_tail() {
        let (transport, probe) = LoopbackTransport::paired().await;
        let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
        let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport)).await;
        for epoch in 1..=SHARD_DEFERRED_ITEMS as u64 {
            let raw = vec![0xff, (epoch >> 8) as u8, epoch as u8];
            probe.push_inbound(raw).await;
            assert!(matches!(shard.drive_one().await, ShardDrive::Fault { consumed_epoch: Some(consumed), terminal_overflow: false, .. } if consumed == epoch));
        }
        let overflow_raw = vec![0xfe, 0x01, 0x01];
        probe.push_inbound(overflow_raw.clone()).await;
        match shard.drive_one().await {
            ShardDrive::Fault { consumed_epoch: None, work_remains: false, terminal_frame: true, terminal_overflow: true, .. } => {}
            _ => panic!("terminal capacity plus one must park without acknowledgement or readiness"),
        }
        assert!(!shard.has_pending_work(), "no retrieval means no self-resubmit readiness");
        assert_eq!(shard.terminal_frame_overflow.len(), 1, "one exact overflow owner is retained");

        let (oldest, rearmed_epoch) = shard.take_terminal_frame_and_rearm();
        assert_eq!(oldest, Some(vec![0xff, 0, 1]), "retrieval preserves the older FIFO head");
        assert_eq!(rearmed_epoch, Some(SHARD_DEFERRED_ITEMS as u64 + 1), "one freed slot re-arms the original overflow epoch exactly once");
        assert!(shard.terminal_frame_overflow.is_empty());
        assert_eq!(shard.terminal_frames.len(), SHARD_DEFERRED_ITEMS, "re-arm fills only the one freed terminal slot");
        for _ in 1..SHARD_DEFERRED_ITEMS {
            assert!(shard.take_terminal_frame().is_some());
        }
        assert_eq!(shard.take_terminal_frame(), Some(overflow_raw), "overflow appends after every older terminal owner");
        assert!(shard.take_terminal_frame().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn permanently_over_capacity_frame_uses_the_same_bounded_overflow_handoff() {
        let actor = ActorId(54);
        let (transport, probe) = LoopbackTransport::paired().await;
        let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
        let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(transport)).await;
        for index in 0..SHARD_DEFERRED_ITEMS {
            let raw = vec![0xfd, index as u8];
            shard.terminal_frames.try_push(raw.clone(), raw.len()).expect("fill terminal ring exactly");
        }
        let envelopes = (0..=SHARD_DEFERRED_ITEMS)
            .map(|seq| Envelope {
                to: actor,
                from: semio_framework_actor::Origin::Kernel,
                lane: semio_framework_actor::Lane::Background,
                seq: seq as u64,
                deadline_ms: None,
                coalesce: None,
                cancel_of: None,
                payload: Payload::Cancel { seq: seq as u64 },
            })
            .collect();
        let mut raw = Vec::new();
        ShardFrame::Grant { actor, budget: semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background), envelopes }.pack_encode(&mut raw).await;
        probe.push_inbound(raw.clone()).await;

        assert!(matches!(shard.drive_one().await, ShardDrive::Fault { consumed_epoch: None, terminal_overflow: true, work_remains: false, .. }));
        let (_, rearmed_epoch) = shard.take_terminal_frame_and_rearm();
        assert_eq!(rearmed_epoch, Some(1));
        for _ in 1..SHARD_DEFERRED_ITEMS {
            assert!(shard.take_terminal_frame().is_some());
        }
        assert_eq!(shard.take_terminal_frame(), Some(raw), "permanent-cap raw owner is retained without re-encoding");
    }

    #[test]
    fn terminal_overflow_slot_rejects_an_aba_key_after_rearm() {
        let mut overflow = FixedOwnerRing::<TerminalFrameOverflow, 1>::new(usize::MAX);
        let stale = overflow.try_push(TerminalFrameOverflow { epoch: 1, bytes: vec![1] }, 1).expect("first overflow generation");
        assert_eq!(overflow.pop_front().map(|(_, owner)| owner.epoch), Some(1));
        let current = overflow.try_push(TerminalFrameOverflow { epoch: 2, bytes: vec![2] }, 1).expect("rearmed overflow generation");
        assert!(!overflow.contains(stale), "an ABA-stale overflow generation cannot address the rearmed owner");
        assert!(overflow.contains(current));
    }

    #[semio_framework_async_macros::async_test]
    async fn transient_frame_rejection_keeps_its_original_epoch_until_admission() {
        let actor = ActorId(51);
        let mut bytes = Vec::new();
        ShardFrame::Register { actor }.pack_encode(&mut bytes).await;
        let runtime = Arc::new(GuestRuntimes::Mock(Arc::new(MockGuestRuntime::new().await)));
        let mut shard = ShardLoop::new(runtime, ShardTransports::Loopback(LoopbackTransport::default())).await;
        shard.rejected_frame = Some((9, bytes));

        match shard.drive_one().await {
            ShardDrive::Idle { consumed_epoch: Some(9) } => {}
            _ => panic!("a retained frame must acknowledge its original epoch only after admission"),
        }
    }

    #[test]
    fn grant_raw_credit_is_exact_at_bytes_plus_one() {
        let raw_bytes = 17usize;
        let credits = [split_frame_credit(raw_bytes, 3, 0), split_frame_credit(raw_bytes, 3, 1), split_frame_credit(raw_bytes, 3, 2)];
        assert_eq!(credits.iter().sum::<usize>(), raw_bytes, "nested authorities split the admitted Grant bytes exactly");
        let mut ring = FixedOwnerRing::<u8, 4>::new(raw_bytes);
        for (owner, bytes) in credits.into_iter().enumerate() {
            ring.try_push(owner as u8, bytes).expect("exact Grant byte credit");
        }
        let rejected = ring.try_push(4, 1).expect_err("Grant raw bytes plus one");
        assert_eq!(rejected.limit, AdmissionLimit::Bytes);
        assert_eq!(rejected.owner, 4);
    }

    #[test]
    fn suspend_and_resume_authorities_have_exact_item_and_byte_handback() {
        let actor = ActorId(52);
        let operation = test_job_operation(actor, 7, 0);
        let mut items = FixedOwnerRing::<DeferredAuthority, 1>::new(32);
        items.try_push(DeferredAuthority::Suspend { actor: actor.0, operation, applied_progress: 3 }, 16).expect("Suspend exact item cap");
        let rejected = items.try_push(DeferredAuthority::Resume { actor: actor.0, operation, checkpoint: JobCheckpoint { state: vec![1, 2], applied_progress: 3 } }, 16).expect_err("Resume item cap plus one");
        assert_eq!(rejected.limit, AdmissionLimit::Items);
        assert!(matches!(rejected.owner, DeferredAuthority::Resume { actor: owner, .. } if owner == actor.0));

        let mut bytes = FixedOwnerRing::<DeferredAuthority, 2>::new(16);
        bytes.try_push(DeferredAuthority::Resume { actor: actor.0, operation, checkpoint: JobCheckpoint { state: vec![3], applied_progress: 4 } }, 16).expect("Resume exact byte cap");
        let rejected = bytes.try_push(DeferredAuthority::Suspend { actor: actor.0, operation, applied_progress: 4 }, 1).expect_err("Suspend bytes plus one");
        assert_eq!(rejected.limit, AdmissionLimit::Bytes);
        assert!(matches!(rejected.owner, DeferredAuthority::Suspend { actor: owner, .. } if owner == actor.0));
    }

    #[test]
    fn mixed_lifecycle_authorities_remain_fifo_and_pop_one_per_grant() {
        let actor = ActorId(53);
        let operation = test_job_operation(actor, 8, 0);
        let mut ring = FixedOwnerRing::<DeferredAuthority, 3>::new(3);
        ring.try_push(DeferredAuthority::Register { actor }, 1).expect("Register");
        ring.try_push(DeferredAuthority::Suspend { actor: actor.0, operation, applied_progress: 5 }, 1).expect("Suspend");
        ring.try_push(DeferredAuthority::Resume { actor: actor.0, operation, checkpoint: JobCheckpoint { state: vec![], applied_progress: 5 } }, 1).expect("Resume");
        assert!(matches!(ring.pop_front(), Some((_, DeferredAuthority::Register { actor: owner })) if owner == actor));
        assert_eq!(ring.len(), 2, "one grant advances one mixed lifecycle authority");
        assert!(matches!(ring.pop_front(), Some((_, DeferredAuthority::Suspend { actor: owner, .. })) if owner == actor.0));
        assert!(matches!(ring.pop_front(), Some((_, DeferredAuthority::Resume { actor: owner, .. })) if owner == actor.0));
    }
    //#endregion 🔖️LanePriorityAndEpochYield
}
