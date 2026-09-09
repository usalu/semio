use super::*;
use std::num::NonZeroU64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShardActorAllocation {
    actor: ActorId,
    registration: NonZeroU64,
}

impl ShardActorAllocation {
    pub fn key(self, shard: semio_framework_actor::ShardId) -> semio_framework_actor::activation::ActorShardKey {
        semio_framework_actor::activation::ActorShardKey { actor: self.actor, shard, registration: self.registration }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShardRegistrationReason {
    WrongActor,
    Occupied,
    Exhausted,
    Stopped,
}

pub struct ShardRegistrationRejected {
    pub actor: ActorId,
    pub instance: GuestInstance,
    pub reason: ShardRegistrationReason,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum LifecycleRetry {
    None,
    YieldPeer,
    Ready,
}

#[derive(Debug)]
pub(super) struct AdmittedAuthority {
    pub(super) allocation: Option<ShardActorAllocation>,
    pub(super) authority: DeferredAuthority,
    pub(super) budget: semio_framework_actor::Budget,
    pub(super) lane: semio_framework_actor::Lane,
    pub(super) owner_bytes: usize,
    pub(super) retry: LifecycleRetry,
}

impl DeferredAuthority {
    pub(super) fn actor(&self) -> u64 {
        match self {
            Self::Register { actor } | Self::Unregister { actor } => actor.0,
            Self::Event { actor, .. } | Self::JobStep { actor, .. } | Self::JobReplay { actor, .. } | Self::Suspend { actor, .. } | Self::Resume { actor, .. } => *actor,
            Self::Cancel(cursor) => cursor.actor,
        }
    }

    pub(super) fn revokes(&self) -> bool {
        matches!(self, Self::Unregister { .. } | Self::Cancel(_))
    }
}

impl AdmittedAuthority {
    pub(super) fn new(allocation: Option<ShardActorAllocation>, budget: semio_framework_actor::Budget, authority: DeferredAuthority, lane: semio_framework_actor::Lane, owner_bytes: usize) -> Self {
        Self { allocation, authority, budget, lane, owner_bytes, retry: LifecycleRetry::None }
    }

    pub(super) fn retries(&self) -> bool {
        self.retry != LifecycleRetry::None
    }
}

impl ShardLoop {
    pub(super) fn current_allocation(&self, actor: u64) -> Option<ShardActorAllocation> {
        self.allocations.get(&actor).copied()
    }

    pub(super) fn allocation_is_current(&self, allocation: Option<ShardActorAllocation>) -> bool {
        allocation.is_some_and(|allocation| self.current_allocation(allocation.actor.0) == Some(allocation))
    }

    #[expect(clippy::result_large_err, reason = "Registration refusal returns the exact live guest instance so the caller can retire its admitted runtime resources.")]
    pub fn register(&mut self, actor: ActorId, instance: GuestInstance) -> Result<ShardActorAllocation, ShardRegistrationRejected> {
        if instance.actor != actor {
            return Err(ShardRegistrationRejected { actor, instance, reason: ShardRegistrationReason::WrongActor });
        }
        if self.instances.contains_key(&actor.0) {
            return Err(ShardRegistrationRejected { actor, instance, reason: ShardRegistrationReason::Occupied });
        }
        let Some(registration) = NonZeroU64::new(self.next_registration) else {
            return Err(ShardRegistrationRejected { actor, instance, reason: ShardRegistrationReason::Exhausted });
        };
        self.next_registration = self.next_registration.checked_add(1).unwrap_or(0);
        let allocation = ShardActorAllocation { actor, registration };
        self.instances.insert(actor.0, instance);
        self.allocations.insert(actor.0, allocation);
        Ok(allocation)
    }

    pub(super) fn has_lifecycle_retry(&self) -> bool {
        [&self.pending_interactive, &self.pending_background].into_iter().any(|ring| (0..ring.len).any(|index| ring.get(index).is_some_and(AdmittedAuthority::retries)))
    }

    pub(super) fn retry_blocks(&self, candidate: &AdmittedAuthority) -> bool {
        !candidate.retries()
            && !candidate.authority.revokes()
            && candidate.allocation.is_some()
            && [&self.pending_interactive, &self.pending_background].into_iter().any(|ring| (0..ring.len).any(|index| ring.get(index).is_some_and(|owner| owner.retries() && owner.allocation == candidate.allocation)))
    }

    pub(super) fn select_lane(&self, ring: &FixedOwnerRing<AdmittedAuthority, SHARD_DEFERRED_ITEMS>) -> Option<usize> {
        let retry = (0..ring.len).find(|index| ring.get(*index).is_some_and(AdmittedAuthority::retries));
        if let Some(index) = retry {
            let owner = ring.get(index).expect("selected retry remains owned");
            if owner.retry == LifecycleRetry::Ready {
                return Some(index);
            }
            return (0..ring.len).find(|candidate| ring.get(*candidate).is_some_and(|peer| peer.allocation != owner.allocation && !self.retry_blocks(peer))).or(Some(index));
        }
        let first = (0..ring.len).find(|index| ring.get(*index).is_some_and(|owner| !self.retry_blocks(owner)))?;
        let DeferredAuthority::JobStep { actor, .. } = &ring.get(first)?.authority else { return Some(first) };
        for index in first..ring.len {
            let Some(owner) = ring.get(index) else { break };
            let DeferredAuthority::JobStep { actor: candidate, turn } = &owner.authority else { break };
            if candidate != actor || self.retry_blocks(owner) {
                break;
            }
            if self.job_placement.get(&(*actor, turn.job)) == Some(&JobPlacement::Exclusive) {
                return Some(index);
            }
        }
        Some(first)
    }

    pub(super) fn select_pending_authority(&mut self) -> Option<AdmittedAuthority> {
        let revocation = [&self.pending_interactive, &self.pending_background].into_iter().enumerate().find_map(|(lane, ring)| {
            (0..ring.len)
                .find(|index| {
                    ring.get(*index).is_some_and(|owner| {
                        owner.authority.revokes()
                            && self.allocation_is_current(owner.allocation)
                            && [&self.pending_interactive, &self.pending_background].into_iter().any(|pending| (0..pending.len).any(|index| pending.get(index).is_some_and(|retry| retry.retries() && retry.allocation == owner.allocation)))
                    })
                })
                .map(|index| (lane, index))
        });
        let (lane, index) = revocation.or_else(|| self.select_lane(&self.pending_interactive).map(|index| (0, index))).or_else(|| self.select_lane(&self.pending_background).map(|index| (1, index)))?;
        let ring = if lane == 0 { &mut self.pending_interactive } else { &mut self.pending_background };
        let (_, selected) = ring.pop_at(index)?;
        for offset in 0..ring.len {
            if let Some(slot) = ring.slots[ring.order[offset]].as_mut() {
                if slot.owner.retry == LifecycleRetry::YieldPeer {
                    slot.owner.retry = LifecycleRetry::Ready;
                }
            }
        }
        Some(selected)
    }

    pub(super) fn retain_lifecycle_retry(&mut self, mut owner: AdmittedAuthority) -> Result<(), PluginHostError> {
        owner.retry = LifecycleRetry::YieldPeer;
        let bytes = owner.owner_bytes;
        let ring = if Self::is_high_priority_lane(owner.lane) { &mut self.pending_interactive } else { &mut self.pending_background };
        if let Err(rejected) = ring.try_push(owner, bytes) {
            let _ = self.terminal_authorities.try_push(rejected.owner.authority, bytes).expect("retained lifecycle refusal owns one exact terminal input");
            return Err(PluginHostError::Plugin("lifecycle retry lost its reserved lane credit".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔁️lifecycle/🦀️.rs"]
mod tests;
