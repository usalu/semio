//! 🎟️ Kernel reservations remain unscheduled until their exact physical shard binding arrives.

use super::*;
use std::num::NonZeroU64;

#[derive(Clone, Debug)]
pub struct KernelActivationRequest {
    pub package: PackageId,
    pub plugin_ordinal: u16,
    pub kind: ActorKind,
    pub lane: Lane,
    pub window: Option<WindowId>,
    pub event: ActivationEvent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelActivationFault {
    Exhausted,
    Occupied,
    ForeignReservation,
    WrongBinding,
    NotActivating,
}

#[derive(Debug)]
pub struct KernelActivationRefused {
    pub request: KernelActivationRequest,
    pub reason: KernelActivationFault,
}

#[derive(Debug)]
pub(super) struct ReservationAuthority;

#[derive(Debug)]
pub struct KernelActivationReservation {
    actor: ActorId,
    shard: ShardId,
    authority: Arc<ReservationAuthority>,
}

impl KernelActivationReservation {
    pub fn actor(&self) -> ActorId {
        self.actor
    }
    pub fn shard(&self) -> ShardId {
        self.shard
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActorShardKey {
    pub actor: ActorId,
    pub shard: ShardId,
    pub registration: NonZeroU64,
}

#[derive(Debug)]
pub struct KernelActivationRejected {
    pub reservation: KernelActivationReservation,
    pub reason: KernelActivationFault,
}

impl Kernel {
    pub async fn reserve_activation(&mut self, request: KernelActivationRequest) -> Result<KernelActivationReservation, KernelActivationRefused> {
        let ordinal = self.next_ordinal.get(&request.package).copied().unwrap_or(0);
        let Some(next) = ordinal.checked_add(1) else { return Err(KernelActivationRefused { request, reason: KernelActivationFault::Exhausted }) };
        let actor = ActorId::new(request.plugin_ordinal, request.kind.tag().await, ordinal, 0).await;
        if self.actors.contains_key(&actor) {
            return Err(KernelActivationRefused { request, reason: KernelActivationFault::Occupied });
        }
        let budget = lane_defaults::budget_for(request.lane);
        let shard = if request.lane == Lane::Interactive {
            let avoid = self.saturated_shards().await;
            self.shards.pin_avoiding(actor, &avoid).await
        } else {
            self.shards.pin(actor).await
        };
        let authority = Arc::new(ReservationAuthority);
        self.scheduler.register_actor_state(actor, request.package.clone(), request.lane, budget, shard, false).await;
        self.next_ordinal.insert(request.package.clone(), next);
        self.actors.insert(
            actor,
            ActorMeta {
                kind: request.kind,
                package: request.package,
                capabilities: Vec::new(),
                budget,
                status: ActorStatus::Activating,
                failure: FailureState::new(),
                metrics: ActorMetrics::default(),
                window: request.window,
                reservation: Some(Arc::clone(&authority)),
                transport_key: None,
            },
        );
        Ok(KernelActivationReservation { actor, shard, authority })
    }

    pub fn reservation_is_current(&self, reservation: &KernelActivationReservation) -> bool {
        self.actors.get(&reservation.actor).and_then(|meta| meta.reservation.as_ref()).is_some_and(|authority| Arc::ptr_eq(authority, &reservation.authority))
    }

    pub async fn bind_activation(&mut self, reservation: KernelActivationReservation, key: ActorShardKey) -> Result<ActorId, KernelActivationRejected> {
        if !self.reservation_is_current(&reservation) {
            return Err(KernelActivationRejected { reservation, reason: KernelActivationFault::ForeignReservation });
        }
        if self.actors.get(&reservation.actor).is_none_or(|meta| meta.status != ActorStatus::Activating) {
            return Err(KernelActivationRejected { reservation, reason: KernelActivationFault::NotActivating });
        }
        if key.actor != reservation.actor || key.shard != reservation.shard || self.shards.shard_of(reservation.actor).await != Some(reservation.shard) {
            return Err(KernelActivationRejected { reservation, reason: KernelActivationFault::WrongBinding });
        }
        let meta = self.actors.get_mut(&reservation.actor).expect("exact reservation owns its metadata");
        meta.reservation = None;
        meta.transport_key = Some(key);
        meta.status = ActorStatus::Active;
        self.scheduler.set_active(reservation.actor, true).await;
        Ok(reservation.actor)
    }

    pub async fn abort_activation(&mut self, reservation: KernelActivationReservation) -> Result<(), KernelActivationRejected> {
        if !self.reservation_is_current(&reservation) {
            return Err(KernelActivationRejected { reservation, reason: KernelActivationFault::ForeignReservation });
        }
        self.scheduler.unregister_actor(reservation.actor).await;
        self.shards.unpin(reservation.actor).await;
        self.actors.remove(&reservation.actor);
        Ok(())
    }

    pub fn transport_key(&self, actor: ActorId) -> Option<ActorShardKey> {
        self.actors.get(&actor).and_then(|meta| meta.transport_key)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🎠️activation-reservation/🦀️.rs"]
mod tests;
