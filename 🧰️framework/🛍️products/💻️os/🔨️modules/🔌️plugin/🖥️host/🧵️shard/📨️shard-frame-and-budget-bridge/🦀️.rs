//! 📨️ Shard frame and budget bridge: the shard frame wire enum, the issued shard turn and retained recipient, the grant to budget conversions and the actor turn result mapping. See [`super`].

use super::*;

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
    Grant { actor: ActorId, budget: semio_framework_actor::Budget, retained: semio_framework_actor::RetainedTurnInput, retained_policy:semio_framework_value::RetainedCloneGrant,work_fuel:u64,deadline_us:u64, envelopes: Vec<Envelope> },
    /// 🔌️ One raw envelope remains retained until its actor supplies an original issued budget.
    Envelope(Envelope),
    MeshReceived{actor:ActorId,input:semio_framework_actor::RetainedTurnInput,receiving_sequence:u64,spent:semio_framework_value::RetainedCloneProgress,work_units_spent:u64,accepted:bool},
}

impl ShardFrame {
    pub(super) async fn tag(&self) -> u8 {
        match self {
            ShardFrame::Register { .. } => 0,
            ShardFrame::Unregister { .. } => 1,
            ShardFrame::Grant { .. } => 2,
            ShardFrame::Envelope(_) => 3,
            ShardFrame::MeshReceived{..}=>4,
        }
    }

    pub async fn pack_encode_grant(grant: &semio_framework_actor::TurnGrant, resources: grant::ShardResourceBudget, out: &mut Vec<u8>) -> Result<(), semio_framework_actor::pack::PackError> {
        grant.original_input().validate().map_err(|_| semio_framework_actor::pack::PackError::InvalidRetainedTurn("operation identity is absent"))?;
        grant.loan.validate_for(grant.retained).map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("original grant semantic loan exceeds its source"))?;let semio_framework_actor::OriginalActorTurnLoan{retained_policy,work_fuel,deadline_us}=grant.loan;
        let mut budget = grant.budget;
        resources.apply(&mut budget);
        semio_framework_actor::pack::write_u8(out, 2).await;
        grant.actor.pack_encode(out).await;
        budget.pack_encode(out).await?;
        grant.retained.pack_encode(out).await?;semio_framework_actor::retained_turn::pack_encode_grant(out,retained_policy).await;semio_framework_actor::pack::write_u64(out,work_fuel).await;semio_framework_actor::pack::write_u64(out,deadline_us).await;
        semio_framework_actor::pack::write_vec(out, &grant.envelopes, async |envelope, out| envelope.pack_encode(out).await).await;
        Ok(())
    }

    pub async fn pack_encode(&self, out: &mut Vec<u8>) -> Result<(), semio_framework_actor::pack::PackError> {
        if let ShardFrame::Grant { retained,retained_policy,work_fuel,deadline_us,.. } = self {
            retained.validate().map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("operation identity is absent"))?;
            let loan=semio_framework_actor::OriginalActorTurnLoan{retained_policy:*retained_policy,work_fuel:*work_fuel,deadline_us:*deadline_us};
            loan.validate_for(*retained).map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("original grant semantic loan exceeds its source"))?;
        }
        semio_framework_actor::pack::write_u8(out, self.tag().await).await;
        match self {
            ShardFrame::Register { actor } => actor.pack_encode(out).await,
            ShardFrame::Unregister { actor } => actor.pack_encode(out).await,
            ShardFrame::Grant { actor, budget, retained, retained_policy,work_fuel,deadline_us,envelopes } => {
                actor.pack_encode(out).await;
                budget.pack_encode(out).await?;
                retained.pack_encode(out).await?;semio_framework_actor::retained_turn::pack_encode_grant(out,*retained_policy).await;semio_framework_actor::pack::write_u64(out,*work_fuel).await;semio_framework_actor::pack::write_u64(out,*deadline_us).await;
                semio_framework_actor::pack::write_vec(out, envelopes, async |o, e| o.pack_encode(e).await).await;
            }
            ShardFrame::Envelope(envelope) => envelope.pack_encode(out).await,
            ShardFrame::MeshReceived{actor,input,receiving_sequence,spent,work_units_spent,accepted}=>{actor.pack_encode(out).await;semio_framework_actor::OriginalActorGuestReturn{input:*input,spent:*spent,work_units_spent:*work_units_spent,claimed_remaining:None,claimed_remaining_fuel:None}.pack_encode(out).await;semio_framework_actor::pack::write_u64(out,*receiving_sequence).await;semio_framework_actor::pack::write_bool(out,*accepted).await;},
        }
        Ok(())
    }

    pub async fn pack_decode(bytes: &[u8], pos: &mut usize) -> Result<Self, semio_framework_actor::pack::PackError> {
        let tag = semio_framework_actor::pack::read_u8(bytes, pos, "ShardFrame").await?;
        match tag {
            0 => Ok(ShardFrame::Register { actor: ActorId::pack_decode(bytes, pos).await? }),
            1 => Ok(ShardFrame::Unregister { actor: ActorId::pack_decode(bytes, pos).await? }),
            2 => {
                let actor=ActorId::pack_decode(bytes,pos).await?;
                let budget=semio_framework_actor::Budget::pack_decode(bytes,pos).await?;
                let retained=semio_framework_actor::RetainedTurnInput::pack_decode(bytes,pos).await?;
                let loan=semio_framework_actor::OriginalActorTurnLoan::pack_decode(bytes,pos).await?;
                loan.validate_for(retained).map_err(|_|semio_framework_actor::pack::PackError::InvalidRetainedTurn("original grant semantic loan exceeds its source"))?;
                Ok(ShardFrame::Grant{actor,budget,retained,retained_policy:loan.retained_policy,work_fuel:loan.work_fuel,deadline_us:loan.deadline_us,envelopes:semio_framework_actor::pack::read_vec(bytes, pos, "ShardFrame::Grant::envelopes", Envelope::pack_decode).await?})
            },
            3 => Ok(ShardFrame::Envelope(Envelope::pack_decode(bytes, pos).await?)),
            4=>{let actor=ActorId::pack_decode(bytes,pos).await?;let raw=semio_framework_actor::OriginalActorGuestReturn::pack_decode(bytes,pos).await?;let receiving_sequence=semio_framework_actor::pack::read_u64(bytes,pos,"MeshReceived::sequence").await?;let accepted=semio_framework_actor::pack::read_bool(bytes,pos,"MeshReceived::accepted").await?;Ok(Self::MeshReceived{actor,input:raw.input,receiving_sequence,spent:raw.spent,work_units_spent:raw.work_units_spent,accepted})},
            other => Err(semio_framework_actor::pack::PackError::InvalidTag { what: "ShardFrame", tag: other, offset: *pos }),
        }
    }
}
//#endregion 📨️ShardFrame

//#region 🔀️BudgetBridge
/// 📦️ OS scheduling resources remain a sibling of the original Actor-issued batch.
#[derive(Debug,PartialEq)]
pub struct IssuedShardTurn{pub budget:semio_framework_actor::Budget,pub(super) retained_policy:semio_framework_value::RetainedCloneGrant,pub(super) work_fuel:u64,pub(super) deadline_us:u64,pub original:semio_framework_actor::IssuedRetainedBatch}

impl IssuedShardTurn{
 pub fn new(budget:semio_framework_actor::Budget,input:semio_framework_actor::RetainedTurnInput,retained_policy:semio_framework_value::RetainedCloneGrant,work_fuel:u64,deadline_us:u64,envelopes:usize)->Result<Self,semio_framework_value::ValueError>{if deadline_us==0||deadline_us>input.work.absolute_deadline_us||work_fuel>input.work.fuel||retained_policy.maximum_items>input.grant.maximum_items||retained_policy.maximum_copy_bytes>input.grant.maximum_copy_bytes||retained_policy.maximum_capacity_bytes>input.grant.maximum_capacity_bytes||retained_policy.maximum_release_bytes>input.grant.maximum_release_bytes||retained_policy.maximum_depth>input.grant.maximum_depth{return Err(Self::refusal("original shard policy exceeds its held total source authority"))}Ok(Self{budget,retained_policy,work_fuel,deadline_us,original:semio_framework_actor::IssuedRetainedBatch::new(input,envelopes)?})}
 /// 🎟️ Issues the whole held source authority of `input` as the shard policy, work fuel and deadline.
 pub fn whole(budget:semio_framework_actor::Budget,input:semio_framework_actor::RetainedTurnInput,envelopes:usize)->Result<Self,semio_framework_value::ValueError>{Self::new(budget,input,input.grant,input.work.fuel,input.work.absolute_deadline_us,envelopes)}
 pub fn envelope_recipient(&mut self,index:usize)->Result<IssuedShardRetainedRecipient<'_>,semio_framework_value::ValueError>{Ok(IssuedShardRetainedRecipient{original:self.original.envelope_recipient(index)?})}
 fn resume_envelope_recipient(&mut self,index:usize)->Result<IssuedShardRetainedRecipient<'_>,semio_framework_value::ValueError>{Ok(IssuedShardRetainedRecipient{original:self.original.resume_envelope_recipient(index)?})}
 pub fn refusal(message:&'static str)->semio_framework_value::ValueError{semio_framework_actor::IssuedRetainedBatch::refusal(message)}
}

impl std::ops::Deref for IssuedShardTurn{type Target=semio_framework_actor::IssuedRetainedBatch;fn deref(&self)->&Self::Target{&self.original}}

impl std::ops::DerefMut for IssuedShardTurn{fn deref_mut(&mut self)->&mut Self::Target{&mut self.original}}

/// 🫴️ The Services extension borrows the canonical original Actor recipient.
pub struct IssuedShardRetainedRecipient<'a>{pub(super) original:semio_framework_actor::IssuedRetainedRecipient<'a>}

impl<'a> std::ops::Deref for IssuedShardRetainedRecipient<'a>{type Target=semio_framework_actor::IssuedRetainedRecipient<'a>;fn deref(&self)->&Self::Target{&self.original}}

impl std::ops::DerefMut for IssuedShardRetainedRecipient<'_>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.original}}

impl semio_framework_os_services::ComputeRetainedRecipient for IssuedShardRetainedRecipient<'_> {
    fn remaining_grant(&self) -> Result<semio_framework_value::RetainedCloneGrant, semio_framework_value::ValueError> { self.original.remaining_grant() }
    fn record_progress(&mut self, progress: semio_framework_value::RetainedCloneProgress) -> Result<(), semio_framework_value::ValueError> { self.original.record_progress(progress) }
}

pub async fn turn_budget_from_grant(budget: &semio_framework_actor::Budget, retained: semio_framework_actor::RetainedTurnInput,retained_policy:semio_framework_value::RetainedCloneGrant,work_fuel:u64,deadline_us:u64) -> Budget {
    Budget { retained, retained_policy,work_fuel,fuel: budget.fuel, deadline_us, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames }
}

/// ⏯️ The events one granted turn hands the guest. A turn this shard cut on its wall deadline was
/// answered `MoreWork` (see `execute_turn_for`'s `DeadlineExceeded` arm) and left the owned guest
/// mid-call, owning the host's next call; the kernel host continues it by granting the actor an
/// `Event::Wake`, which is exactly "resume with no events". Any OTHER event reaching a mid-flight
/// turn is passed through unchanged, so the runtime refuses it loudly instead of dropping it.
pub fn turn_events(event: &Event, in_flight: bool) -> &[Event] {
    if in_flight && matches!(event, Event::Wake) {
        &[]
    } else {
        std::slice::from_ref(event)
    }
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

pub(super) async fn to_actor_turn_result_in_place(result: &mut TurnResult, session: u64, wall_us: u64, memory_bytes: u64) -> Result<semio_framework_actor::TurnResult, semio_framework::Fault> {
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
        retained_receipt: result.retained_receipt,
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

/// 🧮️ Most live jobs one turn can admit: one per granted effect.
pub(super) const SHARD_TURN_ADMITTED_JOBS_MAXIMUM: usize = 1_024;
