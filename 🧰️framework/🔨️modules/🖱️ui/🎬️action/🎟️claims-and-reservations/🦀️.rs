//! 🎟️ Claims and reservations: the action and batch reservations, the claims and claimed drafts, the prepared batches and their reservation operations. See [`super`].

use super::*;

pub struct BoundedActionReservation<'a> {
    pub(super) queue: &'a mut BoundedActionQueue,
    pub(super) builder: BoundedActionBuilder,
}

pub struct BoundedActionBatchReservation<'a> {
    pub(super) queue: &'a mut BoundedActionQueue,
    pub(super) actions: [Option<BoundedAction>; ACTION_BATCH_ITEM_CAPACITY],
    pub(super) item_credits: usize,
    pub(super) byte_credits: usize,
    pub(super) len: usize,
    pub(super) declared_bytes: usize,
    pub(super) bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundedActionClaim {
    pub(super) slot: u16,
    pub(super) epoch: u64,
    pub(super) byte_credits: usize,
}

#[derive(Clone, Copy)]
pub(super) struct BoundedActionClaimSlot {
    pub(super) epoch: u64,
    pub(super) byte_credits: usize,
}

pub struct BoundedClaimedActionReservation<'a> {
    pub(super) queue: &'a mut BoundedActionQueue,
    pub(super) claim: BoundedActionClaim,
    pub(super) builder: BoundedActionBuilder,
}

pub struct BoundedClaimedActionDraft {
    pub(super) claim: BoundedActionClaim,
    pub(super) builder: BoundedActionBuilder,
}

pub struct PreparedClaimedAction {
    pub(super) claim: BoundedActionClaim,
    pub(super) action: BoundedAction,
}

pub struct BoundedActionClaimBatch {
    pub(super) claims: [Option<BoundedActionClaim>; ACTION_CLAIM_BATCH_CAPACITY],
    pub(super) len: u8,
}

pub struct PreparedClaimedActionBatch {
    pub(super) claims: BoundedActionClaimBatch,
    pub(super) actions: [Option<PreparedClaimedAction>; ACTION_CLAIM_BATCH_CAPACITY],
    pub(super) len: u8,
}

impl BoundedClaimedActionDraft {
    pub fn builder(&mut self) -> &mut BoundedActionBuilder {
        &mut self.builder
    }

    pub fn finish(self) -> Result<PreparedClaimedAction, BoundedActionFault> {
        Ok(PreparedClaimedAction { claim: self.claim, action: self.builder.finish()? })
    }

    pub fn claim(&self) -> BoundedActionClaim {
        self.claim
    }
}

impl BoundedActionClaimBatch {
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn claim(&self, index: usize) -> Option<BoundedActionClaim> {
        self.claims.get(index).copied().flatten()
    }

    pub fn take_last(&mut self) -> Option<BoundedActionClaim> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.claims[usize::from(self.len)].take()
    }
}

impl PreparedClaimedActionBatch {
    pub fn new(claims: BoundedActionClaimBatch) -> Self {
        Self { claims, actions: std::array::from_fn(|_| None), len: 0 }
    }

    pub fn claim(&self, index: usize) -> Option<BoundedActionClaim> {
        self.claims.claim(index)
    }

    pub fn push(&mut self, prepared: PreparedClaimedAction) -> Result<(), BoundedActionFault> {
        let index = usize::from(self.len);
        if index == self.claims.len() || self.claims.claim(index) != Some(prepared.claim) {
            return Err(BoundedActionFault::Structure);
        }
        self.actions[index] = Some(prepared);
        self.len += 1;
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        usize::from(self.len) == self.claims.len()
    }

    pub fn take_last_claim(&mut self) -> Option<BoundedActionClaim> {
        let claim = self.claims.take_last()?;
        let index = self.claims.len();
        self.actions[index] = None;
        self.len = self.len.min(self.claims.len() as u8);
        Some(claim)
    }

    pub fn restore_last_claim(&mut self, claim: BoundedActionClaim) -> Result<(), BoundedActionFault> {
        let index = self.claims.len();
        if index == ACTION_CLAIM_BATCH_CAPACITY || self.claims.claims[index].is_some() {
            return Err(BoundedActionFault::Structure);
        }
        self.claims.claims[index] = Some(claim);
        self.claims.len += 1;
        Ok(())
    }
}

impl BoundedActionBatchReservation<'_> {
    pub fn action(&mut self, controller_id: &str, action: &str, byte_credits: usize, build: impl FnOnce(&mut BoundedActionBuilder) -> Result<(), BoundedActionFault>) -> Result<(), BoundedActionFault> {
        if self.len == self.item_credits || self.len == ACTION_BATCH_ITEM_CAPACITY {
            return Err(BoundedActionFault::ItemCredits);
        }
        let next = self.declared_bytes.checked_add(byte_credits).ok_or(BoundedActionFault::ByteCredits)?;
        if byte_credits > ACTION_ITEM_BYTE_CAPACITY || next > self.byte_credits {
            return Err(BoundedActionFault::ByteCredits);
        }
        let mut builder = BoundedActionBuilder::new(controller_id, action, byte_credits)?;
        build(&mut builder)?;
        let action = builder.finish()?;
        self.declared_bytes = next;
        self.bytes = self.bytes.checked_add(action.owned_bytes()).ok_or(BoundedActionFault::ByteCredits)?;
        self.actions[self.len] = Some(action);
        self.len += 1;
        Ok(())
    }

    pub fn publish(mut self) -> Result<(), BoundedActionFault> {
        if self.len != self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        self.publish_staged();
        Ok(())
    }

    pub fn publish_with(mut self, commit: impl FnOnce()) -> Result<(), BoundedActionFault> {
        if self.len != self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        commit();
        self.publish_staged();
        Ok(())
    }

    pub fn publish_with_checked(mut self, commit: impl FnOnce() -> bool) -> Result<(), BoundedActionFault> {
        if self.len != self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        if !commit() {
            return Err(BoundedActionFault::Structure);
        }
        self.publish_staged();
        Ok(())
    }

    pub fn publish_partial(mut self) -> Result<(), BoundedActionFault> {
        if self.len > self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        self.publish_staged();
        Ok(())
    }

    pub fn publish_partial_with_checked(mut self, commit: impl FnOnce() -> bool) -> Result<(), BoundedActionFault> {
        if self.len > self.item_credits || self.bytes > self.byte_credits {
            return Err(BoundedActionFault::ItemCredits);
        }
        if !commit() {
            return Err(BoundedActionFault::Structure);
        }
        self.publish_staged();
        Ok(())
    }

    fn publish_staged(&mut self) {
        for index in 0..self.len {
            let mut action = self.actions[index].take().expect("reserved batch action");
            action.batch_remaining = (self.len - index) as u8;
            self.queue.push_reserved(action);
        }
    }
}

impl BoundedActionReservation<'_> {
    pub fn builder(&mut self) -> &mut BoundedActionBuilder {
        &mut self.builder
    }

    pub fn publish(self) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        self.queue.push_reserved(action);
        Ok(())
    }

    pub fn publish_with(self, commit: impl FnOnce()) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        commit();
        self.queue.push_reserved(action);
        Ok(())
    }
}

impl BoundedClaimedActionReservation<'_> {
    pub fn builder(&mut self) -> &mut BoundedActionBuilder {
        &mut self.builder
    }

    pub fn publish(self) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        self.queue.publish_claimed(self.claim, action)
    }

    pub fn publish_with_checked(self, commit: impl FnOnce() -> bool) -> Result<(), BoundedActionFault> {
        let action = self.builder.finish()?;
        self.queue.validate_claim(self.claim)?;
        if action.owned_bytes() > self.claim.byte_credits {
            return Err(BoundedActionFault::ByteCredits);
        }
        if !commit() {
            return Err(BoundedActionFault::Structure);
        }
        self.queue.publish_claimed(self.claim, action)
    }
}
