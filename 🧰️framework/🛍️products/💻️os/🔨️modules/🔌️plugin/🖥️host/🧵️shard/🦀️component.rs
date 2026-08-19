//! 🧵️ `ShardLoop` — `design-runtime.md` §2/§"ShardTransport": the loop a thread shard runs
//! in-process. Owns a set of live [`super::GuestInstance`]s, pulls [`ShardFrame`]s off a
//! [`semio_framework_actor::ShardTransport`], groups their envelopes per actor, drives
//! [`super::GuestRuntime::execute_turn`]/[`super::GuestRuntime::step_job`], and sends the resulting
//! [`semio_framework::kernel::TurnResult`]/[`super::JobStep`] back over the SAME transport as bytes.
//!
//! Written so the identical type can later be driven over stdio by a helper process (packet P1,
//! `ProcessTransport`) — the only thing that changes between "thread shard" and "process shard" is
//! which [`ShardTransports`] variant `ShardLoop::new` receives; `pump`'s own body never branches on
//! which one it got (only the closed-set enum's own delegation impl does — O1/R1's dyn replacement,
//! packet host-dedyn). `ProcessTransport` itself is out of this packet's scope (`📌️important.md`'s
//! sequencing: "`semio-shard` `[[bin]]` runs over stdio" is P1, not B1b) — this is the seam, not the
//! process.
//!
//! terra-shard-grants: the wire carries [`ShardFrame`], not raw [`Envelope`] bytes — the kernel's
//! DRR-computed, throttle-scaled per-turn budget now travels WITH the envelopes it grants
//! ([`ShardFrame::Grant`]) instead of `pump` re-deriving one from local constants.

// 🏃️ terra-shard-grants: `ShardExecutor` — one `ShardLoop` per dedicated OS thread. Declared here
// (not in `🖥️host/🦀️component.rs`, a file this packet only touches for narrow Part A fallout) —
// `#[path]` on a submodule resolves relative to THIS file's own directory, so this reaches
// `🧵️shard/🏃️executor.rs` without any edit to the crate-root module tree.
#[path = "🏃️executor.rs"]
pub mod executor;

#[cfg(test)]
use super::{GuestInstanceState, MockGuestRuntime, PackageHash, PackageId, PackageRef};
use super::{GuestInstance, GuestRuntime, GuestRuntimes, JobBudget, JobStep, PluginHostError, TurnFault};
use semio_framework::kernel::{Budget, Effect, Event, JobPlacement, RequestOutcome, TurnResult};
use semio_framework_actor::{ActorId, Envelope, Payload, ShardTransport};
use std::collections::{BTreeSet, HashMap};
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
    fn tag(&self) -> u8 {
        match self {
            ShardFrame::Register { .. } => 0,
            ShardFrame::Unregister { .. } => 1,
            ShardFrame::Grant { .. } => 2,
            ShardFrame::Envelope(_) => 3,
        }
    }

    pub fn pack_encode(&self, out: &mut Vec<u8>) {
        semio_framework_actor::pack::write_u8(out, self.tag());
        match self {
            ShardFrame::Register { actor } => actor.pack_encode(out),
            ShardFrame::Unregister { actor } => actor.pack_encode(out),
            ShardFrame::Grant { actor, budget, envelopes } => {
                actor.pack_encode(out);
                budget.pack_encode(out);
                semio_framework_actor::pack::write_vec(out, envelopes, |o, e| e.pack_encode(o));
            }
            ShardFrame::Envelope(envelope) => envelope.pack_encode(out),
        }
    }

    pub fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor::pack::PackError> {
        let tag = semio_framework_actor::pack::read_u8(bytes, pos, "ShardFrame")?;
        match tag {
            0 => Ok(ShardFrame::Register { actor: ActorId::pack_decode(bytes, pos)? }),
            1 => Ok(ShardFrame::Unregister { actor: ActorId::pack_decode(bytes, pos)? }),
            2 => Ok(ShardFrame::Grant {
                actor: ActorId::pack_decode(bytes, pos)?,
                budget: semio_framework_actor::Budget::pack_decode(bytes, pos)?,
                envelopes: semio_framework_actor::pack::read_vec(bytes, pos, "ShardFrame::Grant::envelopes", Envelope::pack_decode)?,
            }),
            3 => Ok(ShardFrame::Envelope(Envelope::pack_decode(bytes, pos)?)),
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
fn turn_budget_from_grant(budget: semio_framework_actor::Budget) -> Budget {
    Budget { fuel: budget.fuel, deadline_ms: budget.wall_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: GRANT_BUDGET_DEFAULT_MAX_FRAMES }
}

/// 🔀️ Same `Grant` budget, `GuestRuntime::step_job`'s shape — `JobBudget` only ever carried
/// `fuel`/`deadline_ms`, so this is a straight field mapping, no invented default needed.
fn job_budget_from_grant(budget: semio_framework_actor::Budget) -> JobBudget {
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
pub fn to_actor_turn_result(result: &TurnResult, wall_us: u64, memory_bytes: u64) -> semio_framework_actor::TurnResult {
    let status = match &result.status {
        semio_framework::kernel::TurnStatus::Idle => semio_framework_actor::TurnStatus::Idle,
        semio_framework::kernel::TurnStatus::MoreWork => semio_framework_actor::TurnStatus::MoreWork,
        semio_framework::kernel::TurnStatus::CheckpointReady => semio_framework_actor::TurnStatus::CheckpointReady,
        semio_framework::kernel::TurnStatus::Faulted(detail) => semio_framework_actor::TurnStatus::Faulted { detail: detail.clone() },
    };
    semio_framework_actor::TurnResult {
        ui_patches: serde_json::to_vec(&result.ui_patches).unwrap_or_default(),
        effects: serde_json::to_vec(&result.effects).unwrap_or_default(),
        next_wake: result.next_wake,
        status,
        usage: semio_framework_actor::Usage { fuel: result.fuel_used, wall_us, memory_bytes },
    }
}
//#endregion 🔀️BudgetBridge

/// 📤️ One outcome `ShardLoop` sends back over the transport — tagged so a caller on the OTHER end
/// (a `ShardClient`, per `design-runtime.md` §"Web shard"/`ShardTable`) can tell a full turn result
/// apart from a single job step without probing the bytes.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ShardOutcome {
    Turn { actor: u64, result: TurnResult },
    Job { actor: u64, job: u64, step: JobStep },
    Fault { actor: u64, message: String },
    /// 📸️ `Payload::Suspend`'s outcome — `state` is [`super::GuestRuntime::checkpoint`]'s bytes
    /// (empty when `Suspend{checkpoint: false}` asked for no snapshot). A caller on the kernel side
    /// feeds `state` into `Kernel::suspend(actor, Some(state))` (K1's own gap this closes).
    Checkpoint { actor: u64, state: Vec<u8> },
    /// ▶️ `Payload::Resume`'s success outcome, sent after [`super::GuestRuntime::restore`] (when the
    /// envelope carried checkpoint bytes) or immediately (when it did not — nothing to restore).
    Resumed { actor: u64 },
    /// 🛑️ `Payload::Cancel`'s outcome: every one of the actor's `running_jobs` was cancelled via
    /// [`super::GuestRuntime::cancel_job`] and its [`super::GuestInstance`] was unregistered
    /// (dropped) — see `ShardLoop::pump`'s dispatch arm for the semantics this variant confirms.
    Cancelled { actor: u64 },
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
    pending_completions: HashMap<u64, Vec<Event>>,
    /// ⚖️ terra-shard-grants: the budget from the LAST [`ShardFrame::Grant`] seen for each actor —
    /// replaces the deleted `budget_for` closure / `TURN_BUDGET` / `JOB_STEP_BUDGET` constants.
    /// Read by [`Self::granted_budget`]; an actor with no entry (never granted) falls back to the
    /// Maintenance lane's default, per that method's own doc.
    granted_budgets: HashMap<u64, semio_framework_actor::Budget>,
}

impl ShardLoop {
    pub fn new(runtime: Arc<GuestRuntimes>, transport: ShardTransports) -> Self {
        Self { runtime, transport, instances: HashMap::new(), running_jobs: BTreeSet::new(), job_placement: HashMap::new(), pending_completions: HashMap::new(), granted_budgets: HashMap::new() }
    }

    /// ⚖️ `actor`'s last [`ShardFrame::Grant`]ed budget — used for both turn execution and job
    /// stepping (point 2 of the packet brief: "job steps take the owning actor's last granted
    /// budget on the Maintenance lane"). Falls back to `lane_defaults::budget_for(Lane::
    /// Maintenance)` — a real, already-designed floor from the actor crate's own vocabulary, not
    /// an invented magic constant — for an actor that has never been granted a budget at all (e.g.
    /// a standalone `ShardFrame::Envelope` arriving before any `Grant`, or a caller like the
    /// `semio-shard` `[[bin]]` that does not yet send `Grant` frames at all).
    fn granted_budget(&self, actor: u64) -> semio_framework_actor::Budget {
        self.granted_budgets.get(&actor).copied().unwrap_or_else(|| semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance))
    }

    /// 📌️ Adds an already-instantiated actor to this shard's live set — called once per
    /// `Kernel::activate` that lands on this shard. `actor.0` (the bit-packed `u64`) is the map key
    /// throughout this type: `Envelope.to`/`ShardOutcome`'s tag both carry the SAME raw id, so no
    /// `RuntimeActorId` round-trip is needed at the boundary.
    pub fn register(&mut self, actor: ActorId, instance: GuestInstance) {
        self.instances.insert(actor.0, instance);
    }

    pub fn is_registered(&self, actor: ActorId) -> bool {
        self.instances.contains_key(&actor.0)
    }

    /// ✂️ Releases an actor's instance (generation change on restart, or a real unload) — calls
    /// [`super::GuestRuntime::drop_instance`] so the pooling allocator reclaims its slab.
    pub fn unregister(&mut self, actor: ActorId) {
        if let Some(instance) = self.instances.remove(&actor.0) {
            self.runtime.drop_instance(instance);
        }
        self.running_jobs.retain(|&(job_actor, _)| job_actor != actor.0);
        self.job_placement.retain(|&(job_actor, _), _| job_actor != actor.0);
        self.pending_completions.remove(&actor.0);
    }

    pub fn actor_count(&self) -> usize {
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

    /// 🅿️ Same as [`Self::pump`], but takes one frame's bytes that were ALREADY read off the
    /// transport (e.g. by `ShardExecutor`'s blocking park on `ThreadTransport::recv_deadline`)
    /// before the normal non-blocking drain loop continues — lets a blocking wait and this
    /// non-blocking drain share the exact same transport without losing whatever woke the wait.
    /// `primed: None` (what [`Self::pump`] passes) behaves identically to the pre-`ShardFrame`
    /// `pump()`.
    pub async fn pump_primed(&mut self, primed: Option<Vec<u8>>) -> Result<usize, PluginHostError> {
        // 💼️ Completions queued by the PREVIOUS `pump()` call (bottom of this function) are
        // delivered as ordinary events on THIS call — the same channel envelope-sourced events
        // arrive on, so the originating actor's `execute_turn` sees `Event::JobCompleted` exactly
        // like any other inbound event, with no separate delivery path.
        let mut events_by_actor: HashMap<u64, Vec<Event>> = std::mem::take(&mut self.pending_completions);
        let mut jobs_by_actor: Vec<(u64, u64)> = Vec::new();

        if let Some(bytes) = primed {
            self.consume_frame(&bytes, &mut events_by_actor, &mut jobs_by_actor).await?;
        }
        while let Some(bytes) = self.transport.recv().await {
            self.consume_frame(&bytes, &mut events_by_actor, &mut jobs_by_actor).await?;
        }

        let mut driven = 0usize;
        for (actor_id, events) in events_by_actor {
            // 🔀️ Computed BEFORE `get_mut` below — `self.granted_budget(actor_id)` needs `&self`
            // (the whole struct), which conflicts with the `&mut self.instances` borrow `instance`
            // holds for the rest of this iteration (E0502).
            let turn_budget = turn_budget_from_grant(self.granted_budget(actor_id));
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") }).await?;
                continue;
            };
            // 👶️ host-dedyn: `GuestRuntime::execute_turn` is plain AFIT now (double-future
            // collapsed) — `.await`ed directly. This loop's own thread root (`🏃️executor.rs`'s
            // `ShardExecutor::spawn`, `👶️child/🦀️main.rs`'s `main`) is the `block_on` boundary that
            // turns the plain OS thread into an executor; every impl `ShardLoop` is ever handed
            // resolves on its first poll (see `GuestRuntime`'s own doc comment), so this never
            // actually parks.
            let outcome = match self.runtime.execute_turn(instance, &events, turn_budget).await {
                Ok(result) => {
                    // 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1, placement routing added K1):
                    // the generic `Effect::SpawnJob`/`Effect::CancelJob` admission this packet
                    // closes — see `running_jobs`'s own doc comment. `placement` (inline/isolated/
                    // exclusive) is captured into `job_placement` and `Exclusive` is routed to the
                    // FRONT of `to_step`'s per-pump order (below) — every placement still runs on
                    // the SAME instance that spawned it (routing to a DIFFERENT pooled/exclusive
                    // INSTANCE needs the actor pool `Kernel::activate`/`ShardTable` builds,
                    // `design-runtime.md` §1, `🎭️actor` territory a single `ShardLoop` cannot reach
                    // on its own — documented gap, not a silently faked one, see the K1 report's
                    // lease-request).
                    for effect in &result.effects {
                        match effect {
                            Effect::SpawnJob { job, kind, input, placement } => match self.runtime.start_job(instance, *job, kind, input.clone()).await {
                                Ok(()) => {
                                    self.running_jobs.insert((actor_id, *job));
                                    self.job_placement.insert((actor_id, *job), *placement);
                                }
                                Err(fault) => {
                                    self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job: *job, result: RequestOutcome::Err(start_job_fault_bytes(&fault)) });
                                }
                            },
                            Effect::CancelJob { job } => {
                                if self.running_jobs.remove(&(actor_id, *job)) {
                                    self.job_placement.remove(&(actor_id, *job));
                                    let _ = self.runtime.cancel_job(instance, *job).await;
                                }
                            }
                            _ => {}
                        }
                    }
                    ShardOutcome::Turn { actor: actor_id, result }
                }
                Err(fault) => ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) },
            };
            self.send_outcome(&outcome).await?;
            driven += 1;
        }

        // 💼️ Step every job still live — both envelope-driven (`Payload::JobStep`, explicit
        // external re-arming) and self-tracked (`running_jobs`, admitted from a `SpawnJob` effect
        // THIS pump or an earlier one — including one admitted just above, in the SAME pump call
        // that started it). ONE `step_job` per job per `pump()`, deliberately never a loop to
        // completion here (that is `PluginInstanceHandle::run_job_to_completion`'s DIFFERENT,
        // deliberately-synchronous relay for the three hardcoded io/infer kinds) — a job needing N
        // steps needs N `pump()` calls, which is the entire resumability proof.
        let mut to_step: Vec<(u64, u64)> = jobs_by_actor;
        for &pair in &self.running_jobs {
            if !to_step.contains(&pair) {
                to_step.push(pair);
            }
        }
        // 🚦 MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME K1: `Exclusive`-placed jobs step FIRST this
        // pump, ahead of `Inline`/`Isolated` ones — a stable sort, so relative order within each
        // group is otherwise unchanged. This is the honest, IN-SHARD-ONLY approximation of
        // "dedicated" access `ShardLoop` can give on its own (priority within this shard's single
        // step phase); it is NOT cross-shard/thread isolation — that needs `Kernel::request_exclusive`
        // plus a job-forwarding envelope between shards, neither of which exists yet (K1 report's
        // lease-request). `job_placement` may not have an entry for an externally re-armed
        // `Payload::JobStep` that this shard never admitted itself (no `SpawnJob` seen) — such a job
        // sorts as non-exclusive, which is correct: this shard has no placement to honour for it.
        to_step.sort_by_key(|pair| if matches!(self.job_placement.get(pair), Some(JobPlacement::Exclusive)) { 0u8 } else { 1u8 });

        for (actor_id, job) in to_step {
            // 🔀️ Same E0502 reason as the turn-execution loop above — computed before `get_mut`.
            let job_budget = job_budget_from_grant(self.granted_budget(actor_id));
            let Some(instance) = self.instances.get_mut(&actor_id) else {
                self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: actor {actor_id} is not registered on this shard") }).await?;
                continue;
            };
            let outcome = match self.runtime.step_job(instance, job, job_budget).await {
                Ok(step) => {
                    match &step {
                        JobStep::Running { .. } => {}
                        JobStep::Done { output: bytes } => {
                            self.running_jobs.remove(&(actor_id, job));
                            self.job_placement.remove(&(actor_id, job));
                            self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job, result: RequestOutcome::Ok(bytes.clone()) });
                        }
                        JobStep::Failed { error: bytes } => {
                            self.running_jobs.remove(&(actor_id, job));
                            self.job_placement.remove(&(actor_id, job));
                            self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job, result: RequestOutcome::Err(bytes.clone()) });
                        }
                    }
                    ShardOutcome::Job { actor: actor_id, job, step }
                }
                Err(fault) => {
                    self.running_jobs.remove(&(actor_id, job));
                    self.job_placement.remove(&(actor_id, job));
                    self.pending_completions.entry(actor_id).or_default().push(Event::JobCompleted { job, result: RequestOutcome::Err(start_job_fault_bytes(&fault)) });
                    ShardOutcome::Fault { actor: actor_id, message: turn_fault_message(&fault) }
                }
            };
            self.send_outcome(&outcome).await?;
            driven += 1;
        }

        Ok(driven)
    }

    /// 📨️ Decodes one [`ShardFrame`] and dispatches it — the drain loop's per-frame body, factored
    /// out so both [`Self::pump_primed`]'s "one primed frame, then the non-blocking drain" shape
    /// and `ShardFrame::Grant`'s own per-envelope loop (below) can share it.
    async fn consume_frame(&mut self, bytes: &[u8], events_by_actor: &mut HashMap<u64, Vec<Event>>, jobs_by_actor: &mut Vec<(u64, u64)>) -> Result<(), PluginHostError> {
        let mut pos = 0usize;
        let frame = ShardFrame::pack_decode(bytes, &mut pos).map_err(|error| PluginHostError::Plugin(format!("ShardLoop::pump: malformed frame: {error:?}")))?;
        match frame {
            // 📌️ Wire-symmetry only — see `ShardFrame::Register`'s own doc for why an INCOMING
            // `Register` has no local state to mutate (a `GuestInstance` cannot cross a transport).
            ShardFrame::Register { actor: _ } => {}
            ShardFrame::Unregister { actor } => self.unregister(actor),
            ShardFrame::Grant { actor, budget, envelopes } => {
                self.granted_budgets.insert(actor.0, budget);
                for envelope in envelopes {
                    self.dispatch_envelope(envelope, events_by_actor, jobs_by_actor).await?;
                }
            }
            ShardFrame::Envelope(envelope) => self.dispatch_envelope(envelope, events_by_actor, jobs_by_actor).await?,
        }
        Ok(())
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
    async fn dispatch_envelope(&mut self, envelope: Envelope, events_by_actor: &mut HashMap<u64, Vec<Event>>, jobs_by_actor: &mut Vec<(u64, u64)>) -> Result<(), PluginHostError> {
        match envelope.payload {
            Payload::Event { bytes: event_bytes } => {
                let event: Event = serde_json::from_slice(&event_bytes)?;
                events_by_actor.entry(envelope.to.0).or_default().push(event);
            }
            Payload::JobStep { job } => jobs_by_actor.push((envelope.to.0, job)),
            // 📸️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME K1: `checkpoint:bool` gates whether a
            // snapshot is actually taken — `false` is a plain "stop scheduling me" suspend with
            // nothing to persist, so `state` comes back empty rather than calling `checkpoint`
            // for bytes nobody asked for. `ActorStatus::Suspended`/`Kernel::suspend` themselves
            // live in `🎭️actor` and are out of this file's path_scope — a caller on the OTHER
            // end of the transport is the one that turns `ShardOutcome::Checkpoint` into a
            // `Kernel::suspend(actor, Some(state))` call.
            Payload::Suspend { checkpoint } => {
                let actor_id = envelope.to.0;
                let outcome = match self.instances.get_mut(&actor_id) {
                    None => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Suspend for actor {actor_id} which is not registered on this shard") },
                    Some(instance) if checkpoint => match self.runtime.checkpoint(instance).await {
                        Ok(state) => ShardOutcome::Checkpoint { actor: actor_id, state },
                        Err(error) => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Suspend checkpoint failed for actor {actor_id}: {error}") },
                    },
                    Some(_) => ShardOutcome::Checkpoint { actor: actor_id, state: Vec::new() },
                };
                self.send_outcome(&outcome).await?;
            }
            // ▶️ Mirrors `Suspend`: `checkpoint: None` means "resume as-is, nothing to restore"
            // (the actor was never asked for a snapshot, or the caller intentionally cold-starts
            // it) — `restore` is only called when bytes actually arrived.
            Payload::Resume { checkpoint } => {
                let actor_id = envelope.to.0;
                let outcome = match self.instances.get_mut(&actor_id) {
                    None => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Resume for actor {actor_id} which is not registered on this shard") },
                    Some(instance) => match checkpoint {
                        Some(state) => match self.runtime.restore(instance, &state).await {
                            Ok(()) => ShardOutcome::Resumed { actor: actor_id },
                            Err(error) => ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Resume restore failed for actor {actor_id}: {error}") },
                        },
                        None => ShardOutcome::Resumed { actor: actor_id },
                    },
                };
                self.send_outcome(&outcome).await?;
            }
            // 🛑️ `Payload::Cancel { seq }` carries no doc comment of its own beyond the enum-level
            // one (`✉️Envelope` region, `🎭️actor/🦀️component.rs`) and there is no OTHER caller
            // anywhere in the tree to infer intent from — read and confirmed empty before writing
            // this. Grouped with `Suspend`/`Resume` (actor lifecycle), not `JobStep` (single-job
            // control, which already has its own guest-side path via `Effect::CancelJob`), so
            // this is read as actor-level teardown: cancel EVERY job this actor has running via
            // `GuestRuntime::cancel_job`, then unregister (drop) its instance outright — `seq` is
            // not consumed (no documented meaning to key behavior off), only surfaced as
            // the actor id already is. If a future packet's doc comment reveals a narrower
            // per-job meaning for `seq`, this arm is the one to revisit.
            Payload::Cancel { seq: _ } => {
                let actor_id = envelope.to.0;
                if self.instances.contains_key(&actor_id) {
                    let jobs: Vec<u64> = self.running_jobs.iter().filter(|&&(job_actor, _)| job_actor == actor_id).map(|&(_, job)| job).collect();
                    if let Some(instance) = self.instances.get_mut(&actor_id) {
                        for job in jobs {
                            let _ = self.runtime.cancel_job(instance, job).await;
                        }
                    }
                    self.unregister(ActorId(actor_id));
                    self.send_outcome(&ShardOutcome::Cancelled { actor: actor_id }).await?;
                } else {
                    self.send_outcome(&ShardOutcome::Fault { actor: actor_id, message: format!("ShardLoop::pump: Cancel for actor {actor_id} which is not registered on this shard") }).await?;
                }
            }
        }
        Ok(())
    }

    async fn send_outcome(&self, outcome: &ShardOutcome) -> Result<(), PluginHostError> {
        let bytes = serde_json::to_vec(outcome)?;
        self.transport.send(&bytes).await;
        Ok(())
    }

    pub async fn heartbeat(&self) -> u64 {
        self.transport.heartbeat().await
    }
}

fn turn_fault_message(fault: &TurnFault) -> String {
    fault.to_string()
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
pub(crate) struct LoopbackTransport {
    inbound: Arc<std::sync::Mutex<Vec<Vec<u8>>>>,
    outbound: Arc<std::sync::Mutex<Vec<Vec<u8>>>>,
}

#[cfg(test)]
impl LoopbackTransport {
    /// Returns `(the transport ShardLoop::new takes ownership of, a probe this test keeps)`.
    fn paired() -> (Self, tests::LoopbackProbe) {
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
pub(crate) struct RecordingRuntime {
    last_turn_budget: Mutex<Option<Budget>>,
    last_job_budget: Mutex<Option<JobBudget>>,
}

#[cfg(test)]
impl RecordingRuntime {
    pub(crate) fn new() -> Self {
        Self { last_turn_budget: Mutex::new(None), last_job_budget: Mutex::new(None) }
    }
}

#[cfg(test)]
impl GuestRuntime for RecordingRuntime {
    fn compile(&self, package: &PackageRef, _bytes: &[u8]) -> Result<super::CompiledHandle, PluginHostError> {
        Ok(super::CompiledHandle { package_hash: package.hash.0, component: None })
    }
    fn instantiate(&self, _compiled: &super::CompiledHandle, actor: ActorId, _caps: &[super::BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        Ok(GuestInstance { actor, state: GuestInstanceState::Mock(super::MockInstanceState::default()) })
    }
    fn drop_instance(&self, _inst: GuestInstance) {}
    async fn execute_turn(&self, _inst: &mut GuestInstance, _events: &[Event], budget: Budget) -> Result<TurnResult, TurnFault> {
        *self.last_turn_budget.lock().expect("lock") = Some(budget);
        Ok(TurnResult { ui_patches: vec![], effects: vec![], next_wake: None, status: semio_framework::kernel::TurnStatus::Idle, fuel_used: 0 })
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
        fn push_inbound(&self, bytes: Vec<u8>) {
            self.inbound.lock().expect("loopback lock").push(bytes);
        }
        fn take_outbound(&self) -> Vec<Vec<u8>> {
            std::mem::take(&mut *self.outbound.lock().expect("loopback lock"))
        }
    }

    /// 👶️ host-dedyn: every `ShardLoop::pump()`/`pump_primed()` call below is wrapped in
    /// `semio_framework_async::block_on` — a `#[test] fn` body is a sanctioned executor entry point
    /// (R4 clause 5); every `GuestRuntime`/`ShardTransport` impl these tests drive resolves on its
    /// first poll, so `block_on` never actually parks.
    fn pump(shard: &mut ShardLoop) -> Result<usize, PluginHostError> {
        semio_framework_async::block_on(shard.pump())
    }

    fn encode_event_envelope(to: ActorId, seq: u64, event: &Event) -> Vec<u8> {
        encode_payload_envelope(to, seq, Payload::Event { bytes: serde_json::to_vec(event).expect("encode event") })
    }

    /// ✉️ Generic envelope builder for `Suspend`/`Resume`/`Cancel` payload tests —
    /// `encode_event_envelope` above stays as a thin wrapper over this so existing tests are
    /// untouched. terra-shard-grants: wraps in `ShardFrame::Envelope` — the transport now carries
    /// `ShardFrame`, not raw `Envelope` bytes, so every test-side encoder must wrap here too.
    fn encode_payload_envelope(to: ActorId, seq: u64, payload: Payload) -> Vec<u8> {
        let envelope = Envelope { to, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload };
        let mut bytes = Vec::new();
        ShardFrame::Envelope(envelope).pack_encode(&mut bytes);
        bytes
    }

    #[test]
    fn pump_drives_one_turn_per_actor_and_reports_it_as_a_shard_outcome() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(7);
        let package = PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([1u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");
        let mut scripted = MockGuestRuntime::idle_turn();
        scripted.fuel_used = 42;
        mock.script_turn(actor, scripted);

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport));
        shard.register(actor, instance);
        assert!(shard.is_registered(actor));

        let driven = pump(&mut shard).expect("pump succeeds");
        assert_eq!(driven, 1, "exactly one actor had a buffered envelope");

        let outbound = probe.take_outbound();
        assert_eq!(outbound.len(), 1, "one ShardOutcome sent back");
        let outcome: ShardOutcome = serde_json::from_slice(&outbound[0]).expect("decode outcome");
        match outcome {
            ShardOutcome::Turn { actor: reported, result } => {
                assert_eq!(reported, 7);
                assert_eq!(result.fuel_used, 42, "the scripted turn's own fuel_used must round-trip through ShardOutcome");
            }
            other => panic!("expected ShardOutcome::Turn, got {other:?}"),
        }
    }

    #[test]
    fn pump_reports_an_envelope_for_an_unregistered_actor_as_a_fault_not_a_silent_drop() {
        let mock = Arc::new(MockGuestRuntime::new());
        let (transport, probe) = LoopbackTransport::paired();
        let stranger = ActorId(99);
        probe.push_inbound(encode_event_envelope(stranger, 1, &Event::InstanceClose));

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        let driven = pump(&mut shard).expect("pump succeeds even with an unknown actor");
        assert_eq!(driven, 0, "an envelope for an unregistered actor drives nothing");

        let outbound = probe.take_outbound();
        assert_eq!(outbound.len(), 1);
        let outcome: ShardOutcome = serde_json::from_slice(&outbound[0]).expect("decode outcome");
        assert!(matches!(outcome, ShardOutcome::Fault { actor, .. } if actor == 99), "must surface as a Fault naming the actor, not vanish");
    }

    #[test]
    fn unregister_drops_the_instance_and_shrinks_actor_count() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(3);
        let package = PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([2u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("mock instantiate");
        let (transport, _probe) = LoopbackTransport::paired();
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);
        assert_eq!(shard.actor_count(), 1);
        shard.unregister(actor);
        assert_eq!(shard.actor_count(), 0);
        assert!(!shard.is_registered(actor));
    }

    /// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1's headline acceptance test — the mechanism
    /// `📓️terra-M5-report.md` §4(a) found entirely missing: "no code anywhere reads a
    /// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ...}` and spawns/drives a job
    /// for it". Spawns a job from a scripted turn's own emitted effect, steps it across THREE
    /// separate `pump()` calls (`Running`, `Running`, `Done` — never a single-shot call, which is
    /// what actually proves the `JobBudget` mechanism resumes rather than just completing once),
    /// and observes the completion reach the ORIGINATING actor as a real `Event::JobCompleted` on
    /// a LATER `execute_turn` call — not merely that `step_job` returned `Done` in isolation.
    #[test]
    fn spawn_job_effect_is_admitted_stepped_across_multiple_pumps_and_completion_reaches_the_originating_actor() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(21);
        let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([9u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");

        let job_id = 777u64;
        let mut spawning_turn = MockGuestRuntime::idle_turn();
        spawning_turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: b"seed-frames".to_vec(), placement: JobPlacement::Isolated });
        mock.script_turn(actor, spawning_turn);
        // 🔀️ `run_job_to_completion`'s own two-arm shape (Running.../Done) but with TWO `Running`
        // steps first — the resumability proof: a job that finished on step 1 would not
        // distinguish "the budget mechanism resumed it" from "it happened to be a one-shot call".
        mock.script_job_step(actor, JobStep::Running { progress: None });
        mock.script_job_step(actor, JobStep::Running { progress: Some(b"halfway".to_vec()) });
        mock.script_job_step(actor, JobStep::Done { output: b"reconstruction-complete".to_vec() });
        // 🔚️ Whatever `execute_turn` call eventually receives the `Event::JobCompleted` (pump 4,
        // below) still needs a scripted outcome to return — an ordinary idle turn is enough since
        // this test only asserts what EVENTS that call was given, not its own output.
        mock.script_turn(actor, MockGuestRuntime::idle_turn());

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock.clone())), ShardTransports::Loopback(transport));
        shard.register(actor, instance);

        // Pump 1: runs the spawning turn, admits `Effect::SpawnJob` (`start_job`), and — because
        // the job lands in `running_jobs` before the step phase runs — takes its FIRST step in
        // this SAME pump (`Running`).
        let driven1 = pump(&mut shard).expect("pump 1");
        assert_eq!(driven1, 2, "one turn (the spawn) plus one job step (the first Running) this pump");

        // Pump 2 and 3: no new envelopes at all — the job is driven PURELY from `running_jobs`,
        // proving `pump()` self-drives an admitted job without needing external re-arming.
        let driven2 = pump(&mut shard).expect("pump 2");
        assert_eq!(driven2, 1, "only the second Running step — no envelope, so no turn this pump");
        let driven3 = pump(&mut shard).expect("pump 3");
        assert_eq!(driven3, 1, "the terminal Done step");

        // Pump 4: still no new envelope — but the Done step queued an `Event::JobCompleted` for
        // delivery, so the actor is driven ONE more time purely to receive it.
        let driven4 = pump(&mut shard).expect("pump 4");
        assert_eq!(driven4, 1, "the queued completion drives one more turn, with no job left to step");

        let outbound = probe.take_outbound();
        let outcomes: Vec<ShardOutcome> = outbound.iter().map(|bytes| serde_json::from_slice(bytes).expect("decode outcome")).collect();
        let job_outcomes: Vec<&JobStep> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Job { job, step, .. } if *job == job_id => Some(step),
                _ => None,
            })
            .collect();
        assert_eq!(job_outcomes.len(), 3, "exactly three step_job calls were made — the resumability proof");
        assert!(matches!(job_outcomes[0], JobStep::Running { progress: None }));
        assert!(matches!(job_outcomes[1], JobStep::Running { progress: Some(bytes) } if bytes == b"halfway"));
        assert!(matches!(job_outcomes[2], JobStep::Done { output: bytes } if bytes == b"reconstruction-complete"));

        // 🎯️ The actual end-to-end proof: the ORIGINATING actor's `execute_turn` was, at some
        // point, handed a real `Event::JobCompleted{job: 777, result: Ok(..)}` — not merely that
        // `step_job` internally returned `Done` (`job_outcomes` above already showed that; this is
        // the part M5 found completely missing: nothing delivered it back).
        let completed = mock.observed_events(actor).into_iter().find(|event| matches!(event, Event::JobCompleted { job, .. } if *job == job_id));
        match completed {
            Some(Event::JobCompleted { result: RequestOutcome::Ok(bytes), .. }) => {
                assert_eq!(bytes, b"reconstruction-complete", "the Done step's own bytes must round-trip into the delivered completion event");
            }
            other => panic!("expected Event::JobCompleted{{job: 777, result: Ok(..)}} to have reached the originating actor's execute_turn, got {other:?}"),
        }
    }

    /// 🛑️ `Effect::CancelJob` removes a job from `running_jobs` in the SAME turn it is seen, so a
    /// job cancelled before its first step is never stepped at all — no stray `ShardOutcome::Job`
    /// for it, ever.
    #[test]
    fn cancel_job_effect_stops_a_job_before_it_is_ever_stepped() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(22);
        let package = PackageRef { package: PackageId("remodel".to_string()), hash: PackageHash([10u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");

        let job_id = 888u64;
        let mut turn = MockGuestRuntime::idle_turn();
        turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        turn.effects.push(Effect::CancelJob { job: job_id });
        mock.script_turn(actor, turn);

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);

        let driven = pump(&mut shard).expect("pump");
        assert_eq!(driven, 1, "only the turn itself — the job was cancelled before the step phase, so no step_job call happened");

        let outbound = probe.take_outbound();
        let outcomes: Vec<ShardOutcome> = outbound.iter().map(|bytes| serde_json::from_slice(bytes).expect("decode outcome")).collect();
        assert!(!outcomes.iter().any(|outcome| matches!(outcome, ShardOutcome::Job { .. })), "a cancelled-before-first-step job must never produce a ShardOutcome::Job");
    }

    //#region 🔖️K1SuspendResumePlacement
    /// 📸️ `Payload::Suspend { checkpoint: true }` must dispatch to `GuestRuntime::checkpoint` and
    /// surface its bytes in a `ShardOutcome::Checkpoint` — the K1 gap: this envelope used to fault
    /// out unconditionally instead of reaching `checkpoint` at all.
    #[test]
    fn suspend_with_checkpoint_true_surfaces_checkpoint_bytes_in_the_outcome() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(31);
        let package = PackageRef { package: PackageId("suspend".to_string()), hash: PackageHash([5u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("mock instantiate");

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { checkpoint: true }));

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);

        let driven = pump(&mut shard).expect("pump");
        assert_eq!(driven, 0, "Suspend is handled entirely in the drain loop, not the turn/step phases");

        let outbound = probe.take_outbound();
        assert_eq!(outbound.len(), 1);
        let outcome: ShardOutcome = serde_json::from_slice(&outbound[0]).expect("decode outcome");
        match outcome {
            ShardOutcome::Checkpoint { actor: reported, state } => {
                assert_eq!(reported, 31);
                assert_eq!(state, b"mock-checkpoint:31".to_vec(), "MockGuestRuntime::checkpoint's own deterministic bytes must round-trip unmodified");
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
    #[test]
    fn suspend_then_resume_round_trips_byte_identical_checkpoint_state() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(32);
        let package = PackageRef { package: PackageId("suspend-resume".to_string()), hash: PackageHash([6u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("mock instantiate");

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_payload_envelope(actor, 1, Payload::Suspend { checkpoint: true }));
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);
        pump(&mut shard).expect("pump suspend");

        let suspend_outbound = probe.take_outbound();
        let checkpoint_bytes = match serde_json::from_slice(&suspend_outbound[0]).expect("decode suspend outcome") {
            ShardOutcome::Checkpoint { state, .. } => state,
            other => panic!("expected ShardOutcome::Checkpoint, got {other:?}"),
        };

        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::Resume { checkpoint: Some(checkpoint_bytes.clone()) }));
        pump(&mut shard).expect("pump resume");

        let resume_outbound = probe.take_outbound();
        let resume_outcome: ShardOutcome = serde_json::from_slice(&resume_outbound[0]).expect("decode resume outcome");
        assert!(matches!(resume_outcome, ShardOutcome::Resumed { actor: reported } if reported == 32));

        let instance = shard.instances.get(&actor.0).expect("Resume must not drop the instance");
        let GuestInstanceState::Mock(mock_state) = &instance.state else { panic!("expected a Mock instance") };
        assert_eq!(mock_state.checkpoint.as_deref(), Some(checkpoint_bytes.as_slice()), "restore must have been called with the EXACT bytes checkpoint produced");
    }

    /// 🛑️ `Payload::Cancel` must cancel every one of the actor's `running_jobs` (via
    /// `GuestRuntime::cancel_job`) and unregister its instance — after which no further `step_job`
    /// call for that job can ever happen, since the (actor, job) pair no longer exists in
    /// `running_jobs` and the actor itself is no longer registered.
    #[test]
    fn cancel_unregisters_the_instance_and_no_further_step_job_happens() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(41);
        let package = PackageRef { package: PackageId("cancel-payload".to_string()), hash: PackageHash([7u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");

        let job_id = 555u64;
        let mut turn = MockGuestRuntime::idle_turn();
        turn.effects.push(Effect::SpawnJob { job: job_id, kind: "remodel.reconstruct".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        mock.script_turn(actor, turn);
        mock.script_job_step(actor, JobStep::Running { progress: None });

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);

        let driven1 = pump(&mut shard).expect("pump 1");
        assert_eq!(driven1, 2, "the spawning turn plus the job's first (only scripted) step");
        probe.take_outbound();

        probe.push_inbound(encode_payload_envelope(actor, 2, Payload::Cancel { seq: 0 }));
        let driven2 = pump(&mut shard).expect("pump 2 (cancel)");
        assert_eq!(driven2, 0, "Cancel is handled in the drain loop; the actor is unregistered before the turn/step phases run");
        assert!(!shard.is_registered(actor), "Cancel must unregister the actor's instance");
        assert_eq!(shard.actor_count(), 0);

        let outbound2 = probe.take_outbound();
        assert_eq!(outbound2.len(), 1);
        let outcome: ShardOutcome = serde_json::from_slice(&outbound2[0]).expect("decode cancel outcome");
        assert!(matches!(outcome, ShardOutcome::Cancelled { actor: reported } if reported == 41));

        // 🎯️ A third pump proves the job is truly dead: if `running_jobs` still held it, `step_job`
        // would be called again with an EMPTY scripted queue and fault loudly (`TurnFault::
        // Exhausted`) rather than silently succeeding — no such outcome appears.
        let driven3 = pump(&mut shard).expect("pump 3");
        assert_eq!(driven3, 0, "nothing left to drive: no envelopes, no running_jobs, no registered instance");
        assert!(probe.take_outbound().is_empty(), "no further outcome of any kind for the cancelled job");
    }

    /// 🚦 `JobPlacement::Exclusive` must be honoured rather than silently ignored: an `Exclusive`
    /// job admitted in the SAME pump as an `Inline` one is stepped FIRST — the shard-local routing
    /// this packet adds (see `to_step`'s own doc comment for why this is the honest in-shard-only
    /// approximation, not cross-shard dedicated placement).
    #[test]
    fn exclusive_placement_is_stepped_before_inline_placement_admitted_the_same_pump() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(51);
        let package = PackageRef { package: PackageId("placement".to_string()), hash: PackageHash([8u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).expect("mock instantiate");

        let inline_job = 61u64;
        let exclusive_job = 62u64;
        let mut turn = MockGuestRuntime::idle_turn();
        // 🔀️ Inline is pushed FIRST in spawn order, so a passing test proves the sort actually
        // reorders by placement rather than merely preserving admission order by coincidence.
        turn.effects.push(Effect::SpawnJob { job: inline_job, kind: "a".to_string(), input: Vec::new(), placement: JobPlacement::Inline });
        turn.effects.push(Effect::SpawnJob { job: exclusive_job, kind: "b".to_string(), input: Vec::new(), placement: JobPlacement::Exclusive });
        mock.script_turn(actor, turn);
        // 🧯️ `Running { progress: None }`, not `Done`/`Failed` — those two `JobStep` variants are a
        // PRE-EXISTING, out-of-path_scope serde bug (`send_outcome`'s `serde_json::to_vec` panics:
        // "cannot serialize tagged newtype variant JobStep::Done containing a sequence", the exact
        // same internally-tagged-newtype hazard `Running`'s own doc comment already names, just never
        // fixed for `Done`/`Failed` — see this packet's K1 report for the lease-request). This test
        // only needs to prove STEP ORDER, which `Running` proves identically without touching that
        // unrelated bug.
        mock.script_job_step(actor, JobStep::Running { progress: None });
        mock.script_job_step(actor, JobStep::Running { progress: None });

        let (transport, probe) = LoopbackTransport::paired();
        probe.push_inbound(encode_event_envelope(actor, 1, &Event::InstanceClose));
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);

        let driven = pump(&mut shard).expect("pump");
        assert_eq!(driven, 3, "one turn plus two job steps this pump");

        let outbound = probe.take_outbound();
        let outcomes: Vec<ShardOutcome> = outbound.iter().map(|bytes| serde_json::from_slice(bytes).expect("decode outcome")).collect();
        let job_order: Vec<u64> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Job { job, .. } => Some(*job),
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
            #[test]
            fn $name() {
                let value: ShardFrame = $value;
                let mut bytes = Vec::new();
                value.pack_encode(&mut bytes);
                let mut pos = 0usize;
                let decoded = ShardFrame::pack_decode(&bytes, &mut pos).expect("pack_decode");
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
        ShardFrame::Envelope(Envelope {
            to: ActorId(13),
            from: semio_framework_actor::Origin::Kernel,
            lane: semio_framework_actor::Lane::Background,
            seq: 2,
            deadline_ms: None,
            coalesce: None,
            cancel_of: None,
            payload: Payload::Cancel { seq: 4 },
        })
    );

    /// 🎯️ A `Grant` with ZERO bundled envelopes must still record its budget — proven separately
    /// from the "budget actually executes under" test below, which needs at least one envelope to
    /// drive a turn.
    #[test]
    fn grant_with_no_envelopes_still_records_the_budget() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(61);
        let package = PackageRef { package: PackageId("grant-empty".to_string()), hash: PackageHash([20u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("mock instantiate");
        let (transport, probe) = LoopbackTransport::paired();
        let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        budget.fuel = 123_456;
        let mut bytes = Vec::new();
        ShardFrame::Grant { actor, budget, envelopes: vec![] }.pack_encode(&mut bytes);
        probe.push_inbound(bytes);

        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);
        let driven = pump(&mut shard).expect("pump");
        assert_eq!(driven, 0, "no envelopes bundled — nothing to drive yet");
        assert_eq!(shard.granted_budget(actor.0).fuel, 123_456, "the Grant's budget must be recorded even with no envelopes");
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
    #[test]
    fn a_grants_budget_is_what_the_turn_actually_executes_under() {
        let runtime = Arc::new(RecordingRuntime::new());
        let actor = ActorId(71);
        let package = PackageRef { package: PackageId("grant-budget".to_string()), hash: PackageHash([21u8; 32]) };
        let compiled = runtime.compile(&package, &[]).expect("compile");
        let instance = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("instantiate");

        let (transport, probe) = LoopbackTransport::paired();
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport));
        shard.register(actor, instance);

        let mut first_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Interactive);
        first_budget.fuel = 111_111;
        let envelope = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq: 1, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() } };
        let mut bytes = Vec::new();
        ShardFrame::Grant { actor, budget: first_budget, envelopes: vec![envelope] }.pack_encode(&mut bytes);
        probe.push_inbound(bytes);
        pump(&mut shard).expect("pump 1");
        assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called").fuel, 111_111, "the FIRST Grant's own fuel must reach execute_turn, not a constant");

        let mut second_budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Background);
        second_budget.fuel = 222_222;
        let envelope2 = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).unwrap() } };
        let mut bytes2 = Vec::new();
        ShardFrame::Grant { actor, budget: second_budget, envelopes: vec![envelope2] }.pack_encode(&mut bytes2);
        probe.push_inbound(bytes2);
        pump(&mut shard).expect("pump 2");
        assert_eq!(runtime.last_turn_budget.lock().unwrap().expect("execute_turn must have been called again").fuel, 222_222, "a DIFFERENT second Grant's fuel must reach execute_turn too — proving it travels per-Grant, not a fixed constant");

        let _ = probe.take_outbound();
    }

    /// 🎯️ Same property for job stepping: `step_job`'s `JobBudget` comes from the SAME actor's last
    /// granted budget (point 2 of the brief: "job steps take the owning actor's last granted budget
    /// on the Maintenance lane").
    #[test]
    fn job_step_uses_the_owning_actors_last_granted_budget() {
        let runtime = Arc::new(RecordingRuntime::new());
        let actor = ActorId(72);
        let package = PackageRef { package: PackageId("grant-job-budget".to_string()), hash: PackageHash([22u8; 32]) };
        let compiled = runtime.compile(&package, &[]).expect("compile");
        let instance = runtime.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("instantiate");

        let (transport, probe) = LoopbackTransport::paired();
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Recording(runtime.clone())), ShardTransports::Loopback(transport));
        shard.register(actor, instance);

        let mut budget = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance);
        budget.fuel = 333_333;
        let mut bytes = Vec::new();
        ShardFrame::Grant { actor, budget, envelopes: vec![] }.pack_encode(&mut bytes);
        probe.push_inbound(bytes);
        // 🔀️ An explicit `JobStep` re-arming, not a `SpawnJob` effect — simplest way to reach the
        // step phase without depending on `RecordingRuntime::execute_turn`'s effects (it always
        // returns none).
        let job_bytes = {
            let envelope = Envelope { to: actor, from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Maintenance, seq: 2, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::JobStep { job: 999 } };
            let mut out = Vec::new();
            ShardFrame::Envelope(envelope).pack_encode(&mut out);
            out
        };
        probe.push_inbound(job_bytes);

        pump(&mut shard).expect("pump");
        assert_eq!(runtime.last_job_budget.lock().unwrap().expect("step_job must have been called").fuel, 333_333, "step_job must run under the SAME actor's last granted budget, not a deleted JOB_STEP_BUDGET constant");
        let _ = probe.take_outbound();
    }

    /// 🎯️ An actor that was NEVER granted a budget still gets a real, principled one (the
    /// Maintenance lane's default) — never a panic, never a zeroed budget.
    #[test]
    fn an_actor_never_granted_a_budget_falls_back_to_the_maintenance_lane_default() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(73);
        let shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(LoopbackTransport::default()));
        let expected = semio_framework_actor::lane_defaults::budget_for(semio_framework_actor::Lane::Maintenance);
        assert_eq!(shard.granted_budget(actor.0), expected);
    }
    //#endregion 🔖️GrantBudgetExecution

    //#region 🔖️RegisterUnregisterFrames
    /// 🛑️ An incoming `ShardFrame::Unregister` must unregister the actor exactly like calling
    /// [`ShardLoop::unregister`] directly — real behavior, unlike `Register`'s wire-symmetry-only
    /// role (see that variant's own doc).
    #[test]
    fn unregister_frame_drops_the_instance_exactly_like_the_direct_call() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(81);
        let package = PackageRef { package: PackageId("unreg-frame".to_string()), hash: PackageHash([23u8; 32]) };
        let compiled = mock.compile(&package, &[]).expect("mock compile");
        let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1, deadline_ms: 1, max_effects: 1, max_patch_bytes: 1, max_frames: 1 }).expect("mock instantiate");
        let (transport, probe) = LoopbackTransport::paired();
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        shard.register(actor, instance);
        assert!(shard.is_registered(actor));

        let mut bytes = Vec::new();
        ShardFrame::Unregister { actor }.pack_encode(&mut bytes);
        probe.push_inbound(bytes);
        let driven = pump(&mut shard).expect("pump");
        assert_eq!(driven, 0, "Unregister is handled entirely in the drain loop");
        assert!(!shard.is_registered(actor), "an incoming Unregister frame must drop the instance");
    }

    /// 📌️ `Register` is decoded without error but has no LOCAL side effect (see its own doc) — this
    /// proves `pump` does not choke on it or mistake it for anything else.
    #[test]
    fn register_frame_is_accepted_without_error_and_has_no_local_side_effect() {
        let mock = Arc::new(MockGuestRuntime::new());
        let actor = ActorId(82);
        let (transport, probe) = LoopbackTransport::paired();
        let mut bytes = Vec::new();
        ShardFrame::Register { actor }.pack_encode(&mut bytes);
        probe.push_inbound(bytes);
        let mut shard = ShardLoop::new(Arc::new(GuestRuntimes::Mock(mock)), ShardTransports::Loopback(transport));
        let driven = pump(&mut shard).expect("pump must not error on a Register frame");
        assert_eq!(driven, 0);
        assert!(!shard.is_registered(actor), "Register never instantiates locally — see its own doc");
    }
    //#endregion 🔖️RegisterUnregisterFrames

    //#region 🔖️BudgetBridge
    #[test]
    fn to_actor_turn_result_maps_status_and_carries_host_measured_usage() {
        let kernel_result = TurnResult { ui_patches: vec![], effects: vec![], next_wake: Some(42), status: semio_framework::kernel::TurnStatus::Faulted(b"trap".to_vec()), fuel_used: 999 };
        let bridged = to_actor_turn_result(&kernel_result, 1234, 5678);
        assert_eq!(bridged.next_wake, Some(42));
        assert_eq!(bridged.status, semio_framework_actor::TurnStatus::Faulted { detail: b"trap".to_vec() }, "status must map 1:1, and the actor crate's struct-variant Faulted (Part A) must be what this bridge constructs");
        assert_eq!(bridged.usage.fuel, 999, "usage.fuel comes from the kernel TurnResult's own fuel_used");
        assert_eq!(bridged.usage.wall_us, 1234, "wall_us is host-measured, passed straight through");
        assert_eq!(bridged.usage.memory_bytes, 5678, "memory_bytes is host-measured, passed straight through");
    }

    #[test]
    fn to_actor_turn_result_status_maps_idle_more_work_and_checkpoint_ready() {
        for (kernel_status, expected) in [
            (semio_framework::kernel::TurnStatus::Idle, semio_framework_actor::TurnStatus::Idle),
            (semio_framework::kernel::TurnStatus::MoreWork, semio_framework_actor::TurnStatus::MoreWork),
            (semio_framework::kernel::TurnStatus::CheckpointReady, semio_framework_actor::TurnStatus::CheckpointReady),
        ] {
            let kernel_result = TurnResult { ui_patches: vec![], effects: vec![], next_wake: None, status: kernel_status, fuel_used: 0 };
            assert_eq!(to_actor_turn_result(&kernel_result, 0, 0).status, expected);
        }
    }
    //#endregion 🔖️BudgetBridge
}
