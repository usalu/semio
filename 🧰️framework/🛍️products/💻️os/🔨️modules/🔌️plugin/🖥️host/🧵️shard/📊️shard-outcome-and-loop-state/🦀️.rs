//! 📊️ Shard outcome and loop state: the shard outcome with its wire pack, the shard loop record and the job authority. See [`super`].

use super::*;

/// 📤️ Owned pack-coded outcome sent from a shard to its scheduler-side consumer.
#[derive(Clone, Debug, PartialEq)]
pub enum ShardOutcome {
    OriginalMesh{actor:u64,input:semio_framework_actor::RetainedTurnInput,retained_policy:semio_framework_value::RetainedCloneGrant,work_fuel:u64,deadline_us:u64,receiving_sequence:u64,source_spent:semio_framework_value::RetainedCloneProgress,source_work_units_spent:u64,bytes:[u8;semio_framework_ui_runtime::UI_NATIVE_MESH_MESSAGE_BYTES]},
    /// 🧾️ One original raw call is returned before the Kernel may issue a continuation.
    OriginalCall{actor:u64,returned:OriginalActorGuestReturn,continuation:bool,call:Option<semio_framework_plugin_guest_contract::original_actor_context::OriginalActorFixedCall>},
    /// 🔁️ One granted turn's result, with the step authority of every live job the turn's
    /// `Effect::SpawnJob`s admitted — the host needs a job's exact `JobTurn` to grant its steps
    /// (`Payload::JobStep`), and only this shard mints it.
    Turn {
        actor: u64,
        result: semio_framework_actor::TurnResult,
        jobs: Vec<JobTurn>,
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
    /// ⏸️ The grant ended before the guest's turn returned: a resumable (owned) instance was cut on
    /// its wall or fuel grant and owns the host's next call, which must resume it with no events —
    /// an `Event::Wake` grant (see [`turn_events`]). Distinct from a turn whose guest answered
    /// `MoreWork`: that turn COMPLETED and a host may hand it back to its caller, while a preempted
    /// actor refuses every new event until a later grant completes its turn.
    Preempted {
        actor: u64,
    },
}

impl ShardOutcome {
    /// 🎭️ The actor this outcome answers — the granted one, or the one whose deferred
    /// `Event::JobCompleted` turn this shard ran without a grant of its own.
    pub fn actor(&self) -> u64 {
        match self {
            Self::OriginalMesh{actor,..}|Self::OriginalCall{actor,..}|Self::Turn { actor, .. } | Self::Job { actor, .. } | Self::Fault { actor, .. } | Self::Checkpoint { actor, .. } | Self::Resumed { actor, .. } | Self::Cancelled { actor } | Self::Preempted { actor } => *actor,
        }
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), semio_framework_actor::pack::PackError> {
        if let Self::Turn { result, .. } = self {
            if result.lifecycle_receipt.is_some_and(|receipt| !receipt.is_valid()) {
                return Err(semio_framework_actor::pack::PackError::InvalidLifecycle("invalid turn receipt authority"));
            }
        }
        match self {
            Self::OriginalMesh{actor,input,retained_policy,work_fuel,deadline_us,receiving_sequence,source_spent,source_work_units_spent,bytes}=>{semio_framework_actor::pack::write_u8(out,8).await;semio_framework_actor::pack::write_u64(out,*actor).await;semio_framework_actor::OriginalActorGuestReturn{input:*input,spent:*source_spent,work_units_spent:*source_work_units_spent,claimed_remaining:None,claimed_remaining_fuel:None}.pack_encode(out).await;semio_framework_actor::retained_turn::pack_encode_grant(out,*retained_policy).await;semio_framework_actor::pack::write_u64(out,*work_fuel).await;semio_framework_actor::pack::write_u64(out,*deadline_us).await;semio_framework_actor::pack::write_u64(out,*receiving_sequence).await;out.extend_from_slice(bytes);},
            Self::OriginalCall{actor,returned,continuation,call}=>{semio_framework_actor::pack::write_u8(out,7).await;semio_framework_actor::pack::write_u64(out,*actor).await;returned.pack_encode(out).await;semio_framework_actor::pack::write_bool(out,*continuation).await;semio_framework_actor::pack::write_bool(out,call.is_some()).await;if let Some(call)=call{let mut receipt=semio_framework_plugin_guest_contract::original_actor_context::OriginalActorFixedReceipt::empty();receipt.write(*call);out.extend_from_slice(receipt.bytes())}},
            Self::Turn { actor, result, jobs } => {
                semio_framework_actor::pack::write_u8(out, 0).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
                result.pack_encode(out).await?;
                semio_framework_actor::pack::write_u64(out, jobs.len() as u64).await;
                for job in jobs {
                    job.pack_encode(out).await;
                }
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
            Self::Preempted { actor } => {
                semio_framework_actor::pack::write_u8(out, 6).await;
                semio_framework_actor::pack::write_u64(out, *actor).await;
            }
        }
        Ok(())
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor::pack::PackError> {
        let tag = semio_framework_actor::pack::read_u8(bytes, pos, "ShardOutcome").await?;
        let actor = semio_framework_actor::pack::read_u64(bytes, pos, "ShardOutcome::actor").await?;
        match tag {
            0 => {
                let result = semio_framework_actor::TurnResult::pack_decode(bytes, pos).await?;
                let count = semio_framework_actor::pack::read_u64(bytes, pos, "ShardOutcome::Turn::jobs").await?;
                if count > SHARD_TURN_ADMITTED_JOBS_MAXIMUM as u64 {
                    return Err(semio_framework_actor::pack::PackError::InvalidTag { what: "ShardOutcome::Turn::jobs", tag: u8::MAX, offset: *pos });
                }
                let mut jobs = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    jobs.push(JobTurn::pack_decode(bytes, pos).await?);
                }
                Ok(Self::Turn { actor, result, jobs })
            }
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
            6 => Ok(Self::Preempted { actor }),
            7=>{let returned=OriginalActorGuestReturn::pack_decode(bytes,pos).await?;let continuation=semio_framework_actor::pack::read_bool(bytes,pos,"OriginalCall::continuation").await?;let call=if semio_framework_actor::pack::read_bool(bytes,pos,"OriginalCall::fixed").await?{let end=pos.checked_add(136).ok_or(semio_framework_actor::pack::PackError::InvalidRetainedTurn("original fixed call extent overflow"))?;let source=bytes.get(*pos..end).ok_or(semio_framework_actor::pack::PackError::InvalidRetainedTurn("original fixed call is truncated"))?;let call=semio_framework_plugin_guest_contract::original_actor_context::OriginalActorFixedReceipt::read(source).map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("original fixed call header is malformed"))?;*pos=end;Some(call)}else{None};Ok(Self::OriginalCall{actor,returned,continuation,call})},
            8=>{let raw=OriginalActorGuestReturn::pack_decode(bytes,pos).await?;let retained_policy=semio_framework_actor::retained_turn::pack_decode_grant(bytes,pos).await?;let work_fuel=semio_framework_actor::pack::read_u64(bytes,pos,"OriginalMesh::work-fuel").await?;let deadline_us=semio_framework_actor::pack::read_u64(bytes,pos,"OriginalMesh::deadline-us").await?;let receiving_sequence=semio_framework_actor::pack::read_u64(bytes,pos,"OriginalMesh::sequence").await?;let end=pos.checked_add(semio_framework_ui_runtime::UI_NATIVE_MESH_MESSAGE_BYTES).ok_or(semio_framework_actor::pack::PackError::InvalidRetainedTurn("original mesh fixed extent overflow"))?;let source=bytes.get(*pos..end).ok_or(semio_framework_actor::pack::PackError::InvalidRetainedTurn("original mesh packet truncated"))?;let mut packet=[0;semio_framework_ui_runtime::UI_NATIVE_MESH_MESSAGE_BYTES];packet.copy_from_slice(source);*pos=end;Ok(Self::OriginalMesh{actor,input:raw.input,retained_policy,work_fuel,deadline_us,receiving_sequence,source_spent:raw.spent,source_work_units_spent:raw.work_units_spent,bytes:packet})},
            other => Err(semio_framework_actor::pack::PackError::InvalidTag { what: "ShardOutcome", tag: other, offset: *pos }),
        }
    }
}

/// 🧵️ design-runtime.md §2. One `ShardLoop` per shard (an OS thread today, a `[[bin]]` process in
/// P1) — never shared across shards, since [`super::GuestRuntime`] instances are `Send + Sync` but a
/// [`GuestInstance`] is pinned to whichever shard activated it (`ShardTable`'s own pinning rule).
pub struct ShardLoop {
    pub(super) identity_issuer:OriginalShardIdentityIssuer,
    pub(super) identities:HashMap<ShardActorAllocation,OriginalShardIdentity>,
    pub runtime: Arc<GuestRuntimes>,
    pub(super) transport: ShardTransports,
    pub instances: HashMap<u64, GuestInstance>,
    pub allocations: HashMap<u64, ShardActorAllocation>,
    pub next_registration: u64,
    /// 💼️ `(actor, job)` pairs admitted from an `Effect::SpawnJob` and not yet `Done`/`Failed` —
    /// MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME J1, the generic host-side executor
    /// `📓️terra-M5-report.md` §4(a) found missing entirely: nothing previously read a
    /// `TurnResult.effects` entry matching `Effect::SpawnJob{kind, ..}` and spawned/drove a job
    /// for it outside the three hardcoded kinds `PluginInstanceHandle::run_job_to_completion`
    /// calls directly. `pump()` steps every entry here exactly once per call — never loops a job
    /// to completion internally — so a job needing N steps needs N `pump()` calls, which is what
    /// proves resumability rather than a single-shot call.
    pub running_jobs: BTreeSet<(u64, u64)>,
    /// 🪪️ Active replay identity and sequence cursor for every running job.
    pub job_turns: HashMap<(u64, u64), JobTurn>,
    /// 📰️ Independently minted operation identity retained from spawn until terminal outcome.
    pub job_authorities: HashMap<(u64, u64), JobAuthority>,
    /// 📄️ Fixed, generation-qualified original spawn owners retained for live restart and replay.
    pub replay_seeds: Box<[Option<MountedReplaySeed>]>,
    pub replay_seed_refusals: Box<[Option<ReplaySpawnRefusal>]>,
    pub(super) replay_seed_cursor: usize,
    pub(super) next_job_operation: u64,
    /// 🚦 `JobPlacement` (inline/isolated/exclusive) captured per `running_jobs` entry at
    /// `Effect::SpawnJob` admission — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME K1: `placement` used
    /// to be matched and immediately discarded (`_` in the `Effect::SpawnJob` arm). `Exclusive`
    /// entries are routed to the FRONT of `to_step`'s per-pump order (see that construction site's
    /// doc comment) — the honest, in-shard-only approximation of "dedicated" access a single
    /// `ShardLoop` can give without a cross-shard/`Kernel`-level job-forwarding mechanism, which
    /// this packet's report flags as a `lease-request` rather than faking. Entries are removed
    /// alongside their matching `running_jobs` entry everywhere the latter is removed.
    pub job_placement: HashMap<(u64, u64), JobPlacement>,
    /// 📨️ `Event::JobCompleted` synthesized when a `running_jobs` entry reaches `Done`/`Failed`,
    /// queued per originating actor and delivered at the TOP of the NEXT `pump()` call (merged
    /// into that call's `events_by_actor`) — so a job's own actor sees the completion on its next
    /// turn even if no other envelope ever arrives for it, exactly the way a real `Event::
    /// Completed` would reach the guest's `RequestRegistry` (`job == req.0`, see `🌐host/
    /// 🦀️.rs`'s `Host::spawn_job` / `⚛️reactor/🦀️.rs`'s `Event::JobCompleted`
    /// routing step).
    pub(super) terminal_authorities: FixedOwnerRing<DeferredAuthority, SHARD_DEFERRED_ITEMS>,
    /// ⚖️ Each actor retains one original issued batch ledger; resource views borrow its Budget.
    pub granted_budgets: HashMap<u64, IssuedShardTurn>,
    /// 🚦 terra-shard-lane (piece 1, `📓️terra-shard-lane-report.md`): the lane from the LAST
    /// [`Envelope`] this shard has dispatched for each actor (`Self::dispatch_envelope`, which
    /// sees every envelope bundled in a `ShardFrame::Grant` as well as every standalone
    /// `ShardFrame::Envelope`). The kernel's DRR `Scheduler` fixes one lane per actor at
    /// registration, so every envelope it ever routes to a given actor already carries that SAME
    /// lane — this recovers the classification `pump_primed`'s two priority queues need WITHOUT a
    /// breaking `ShardFrame::Grant` wire change (see [`Self::actor_lane`]'s own doc for why no
    /// field was added). Read by [`Self::actor_lane`]; an actor with no entry (never seen an
    /// envelope) falls back to Maintenance for queue classification.
    pub actor_lanes: HashMap<u64, semio_framework_actor::Lane>,
    pub pending_interactive: FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    pub pending_background: FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>,
    pub rejected_frame: Option<(u64, Vec<u8>)>,
    pub terminal_frames: FixedOwnerRing<Vec<u8>, SHARD_DEFERRED_ITEMS>,
    pub terminal_frame_overflow: FixedOwnerRing<TerminalFrameOverflow, 1>,
    pub(super) next_frame_epoch: u64,
    pub(super) last_drive_consumed_epoch: Option<u64>,
}

#[derive(Clone, Copy)]
pub struct JobAuthority {
    pub turn: JobTurn,
    pub request: JobReplayRequest,
}
