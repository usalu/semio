//! 📬️ Action queue: the `BoundedActionQueue` with the intent admission and the queue push, claim and drain operations. See [`super`].

use super::*;

pub struct BoundedActionQueue {
    slots: Box<[Option<BoundedAction>; ACTION_QUEUE_ITEM_CAPACITY]>,
    head: usize,
    pub(super) len: usize,
    pub(super) bytes: usize,
    pub(super) claims: Box<[Option<BoundedActionClaimSlot>; ACTION_CLAIM_CAPACITY]>,
    pub(super) claimed_items: usize,
    pub(super) claimed_bytes: usize,
    next_claim_epoch: u64,
    pub(super) admitted_seq: Vec<(String, u64)>,
}

impl Default for BoundedActionQueue {
    fn default() -> Self {
        Self { slots: Box::new(std::array::from_fn(|_| None)), head: 0, len: 0, bytes: 0, claims: Box::new(std::array::from_fn(|_| None)), claimed_items: 0, claimed_bytes: 0, next_claim_epoch: 1, admitted_seq: Vec::new() }
    }
}

/// 🚦️ Why an intent did not reach the queue — the two refusals React's runtime already applies to a
/// `UiIntent` before it becomes a command (`🧠️runtime/🎯️dispatch/🦀️.rs`'s `DispatchOutcome::Stale`
/// and the store's own per-surface `seq` ordering), named apart so a log or a metric can tell them
/// from a credit refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiIntentAdmission {
    Accepted,
    /// 🕰️ The gesture's revision trails the surface's current one by more than one.
    Stale,
    /// 🔁️ This surface already admitted this `seq` or a later one — a duplicate or a reorder.
    Duplicate,
}

impl BoundedActionQueue {
    /// 🔏️ Orders and de-duplicates one surface's intents by `seq`, and drops a stale one outright —
    /// the last gate before a gesture becomes a queued action. `current_revision` is the revision of
    /// the document the intent is being resolved against, not the one it was fired at.
    pub fn admit_intent(&mut self, intent: &UiIntentCommand, current_revision: u64) -> UiIntentAdmission {
        if intent_is_stale(intent.address.revision, current_revision) {
            return UiIntentAdmission::Stale;
        }
        self.admit_intent_seq(intent)
    }

    /// 🔃️ The ordering half of [`Self::admit_intent`] alone, for a host that reaches the queue
    /// without the live document in hand — the renderer's own `events::EventRouter::build_intent`
    /// already refused the stale ones against the tree it dispatched them on, which is the same place
    /// React's `Dispatcher` refuses them.
    pub fn admit_intent_seq(&mut self, intent: &UiIntentCommand) -> UiIntentAdmission {
        match self.admitted_seq.iter_mut().find(|(surface, _)| *surface == intent.address.surface) {
            Some((_, admitted)) if intent.seq <= *admitted => UiIntentAdmission::Duplicate,
            Some((_, admitted)) => {
                *admitted = intent.seq;
                UiIntentAdmission::Accepted
            }
            None => {
                self.admitted_seq.push((intent.address.surface.clone(), intent.seq));
                UiIntentAdmission::Accepted
            }
        }
    }

    pub fn admitted_seq(&self, surface: &str) -> u64 {
        self.admitted_seq.iter().find(|(name, _)| name == surface).map_or(0, |(_, seq)| *seq)
    }
}

impl BoundedActionQueue {
    pub fn reserve<'a>(&'a mut self, controller_id: &str, action: &str, byte_credits: usize) -> Result<BoundedActionReservation<'a>, BoundedActionFault> {
        if self.len.checked_add(self.claimed_items).is_none_or(|items| items >= ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(BoundedActionFault::ItemCredits);
        }
        if byte_credits > ACTION_ITEM_BYTE_CAPACITY || self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(byte_credits)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        let builder = BoundedActionBuilder::new(controller_id, action, byte_credits)?;
        Ok(BoundedActionReservation { queue: self, builder })
    }

    pub fn reserve_batch(&mut self, item_credits: usize, byte_credits: usize) -> Result<BoundedActionBatchReservation<'_>, BoundedActionFault> {
        if item_credits == 0 || item_credits > ACTION_BATCH_ITEM_CAPACITY || self.len.checked_add(self.claimed_items).and_then(|len| len.checked_add(item_credits)).is_none_or(|len| len > ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(BoundedActionFault::ItemCredits);
        }
        if byte_credits > ACTION_QUEUE_BYTE_CAPACITY || self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(byte_credits)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        Ok(BoundedActionBatchReservation { queue: self, actions: std::array::from_fn(|_| None), item_credits, byte_credits, len: 0, declared_bytes: 0, bytes: 0 })
    }

    pub fn claim(&mut self, byte_credits: usize) -> Result<BoundedActionClaim, BoundedActionFault> {
        if byte_credits > ACTION_ITEM_BYTE_CAPACITY || self.len.checked_add(self.claimed_items).is_none_or(|items| items >= ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(if byte_credits > ACTION_ITEM_BYTE_CAPACITY { BoundedActionFault::ByteCredits } else { BoundedActionFault::ItemCredits });
        }
        if self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(byte_credits)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        let slot = self.claims.iter().position(Option::is_none).ok_or(BoundedActionFault::ItemCredits)?;
        let epoch = self.next_claim_epoch;
        self.next_claim_epoch = self.next_claim_epoch.wrapping_add(1).max(1);
        self.claims[slot] = Some(BoundedActionClaimSlot { epoch, byte_credits });
        self.claimed_items += 1;
        self.claimed_bytes += byte_credits;
        Ok(BoundedActionClaim { slot: slot as u16, epoch, byte_credits })
    }

    pub fn claim_batch(&mut self, byte_credits: &[usize]) -> Result<BoundedActionClaimBatch, BoundedActionFault> {
        if byte_credits.is_empty() || byte_credits.len() > ACTION_CLAIM_BATCH_CAPACITY {
            return Err(BoundedActionFault::ItemCredits);
        }
        let total_bytes = byte_credits.iter().try_fold(0usize, |sum, bytes| {
            if *bytes > ACTION_ITEM_BYTE_CAPACITY {
                return Err(BoundedActionFault::ByteCredits);
            }
            sum.checked_add(*bytes).ok_or(BoundedActionFault::ByteCredits)
        })?;
        if self.len.checked_add(self.claimed_items).and_then(|items| items.checked_add(byte_credits.len())).is_none_or(|items| items > ACTION_QUEUE_ITEM_CAPACITY) {
            return Err(BoundedActionFault::ItemCredits);
        }
        if self.bytes.checked_add(self.claimed_bytes).and_then(|bytes| bytes.checked_add(total_bytes)).is_none_or(|bytes| bytes > ACTION_QUEUE_BYTE_CAPACITY) {
            return Err(BoundedActionFault::ByteCredits);
        }
        let mut free = [0usize; ACTION_CLAIM_BATCH_CAPACITY];
        let mut free_len = 0usize;
        for (index, claim) in self.claims.iter().enumerate() {
            if claim.is_none() {
                free[free_len] = index;
                free_len += 1;
                if free_len == byte_credits.len() {
                    break;
                }
            }
        }
        if free_len != byte_credits.len() {
            return Err(BoundedActionFault::ItemCredits);
        }
        let mut claims = [None; ACTION_CLAIM_BATCH_CAPACITY];
        for (index, bytes) in byte_credits.iter().copied().enumerate() {
            let slot = free[index];
            let epoch = self.next_claim_epoch;
            self.next_claim_epoch = self.next_claim_epoch.wrapping_add(1).max(1);
            self.claims[slot] = Some(BoundedActionClaimSlot { epoch, byte_credits: bytes });
            claims[index] = Some(BoundedActionClaim { slot: slot as u16, epoch, byte_credits: bytes });
        }
        self.claimed_items += byte_credits.len();
        self.claimed_bytes += total_bytes;
        Ok(BoundedActionClaimBatch { claims, len: byte_credits.len() as u8 })
    }

    pub fn reserve_claimed<'a>(&'a mut self, claim: BoundedActionClaim, controller_id: &str, action: &str) -> Result<BoundedClaimedActionReservation<'a>, BoundedActionFault> {
        self.validate_claim(claim)?;
        let builder = BoundedActionBuilder::new(controller_id, action, claim.byte_credits)?;
        Ok(BoundedClaimedActionReservation { queue: self, claim, builder })
    }

    pub fn draft_claimed(&self, claim: BoundedActionClaim, controller_id: &str, action: &str) -> Result<BoundedClaimedActionDraft, BoundedActionFault> {
        self.validate_claim(claim)?;
        Ok(BoundedClaimedActionDraft { claim, builder: BoundedActionBuilder::new(controller_id, action, claim.byte_credits)? })
    }

    pub fn publish_prepared_claimed(&mut self, prepared: PreparedClaimedAction) -> Result<(), BoundedActionFault> {
        self.publish_claimed(prepared.claim, prepared.action)
    }

    pub fn publish_prepared_claimed_batch(&mut self, mut prepared: PreparedClaimedActionBatch) -> Result<(), BoundedActionFault> {
        if !prepared.is_complete() {
            return Err(BoundedActionFault::ItemCredits);
        }
        for index in 0..prepared.claims.len() {
            let action = prepared.actions[index].as_ref().ok_or(BoundedActionFault::Structure)?;
            let claim = prepared.claims.claim(index).ok_or(BoundedActionFault::Structure)?;
            self.validate_claim(claim)?;
            if action.claim != claim || action.action.owned_bytes() > claim.byte_credits {
                return Err(BoundedActionFault::Structure);
            }
        }
        let len = prepared.claims.len();
        for index in 0..len {
            let mut action = prepared.actions[index].take().expect("validated claimed batch action");
            let slot = self.validate_claim(action.claim)?;
            self.claims[slot] = None;
            self.claimed_items -= 1;
            self.claimed_bytes -= action.claim.byte_credits;
            action.action.batch_remaining = (len - index) as u8;
            self.push_reserved(action.action);
        }
        prepared.claims.len = 0;
        prepared.len = 0;
        Ok(())
    }

    pub fn release_claim(&mut self, claim: BoundedActionClaim) -> Result<(), BoundedActionFault> {
        let slot = self.validate_claim(claim)?;
        self.claims[slot] = None;
        self.claimed_items -= 1;
        self.claimed_bytes -= claim.byte_credits;
        Ok(())
    }

    pub(super) fn validate_claim(&self, claim: BoundedActionClaim) -> Result<usize, BoundedActionFault> {
        let slot = usize::from(claim.slot);
        let Some(owned) = self.claims.get(slot).and_then(Option::as_ref) else {
            return Err(BoundedActionFault::Structure);
        };
        if owned.epoch != claim.epoch || owned.byte_credits != claim.byte_credits {
            return Err(BoundedActionFault::Structure);
        }
        Ok(slot)
    }

    pub(super) fn publish_claimed(&mut self, claim: BoundedActionClaim, action: BoundedAction) -> Result<(), BoundedActionFault> {
        let slot = self.validate_claim(claim)?;
        if action.owned_bytes() > claim.byte_credits {
            return Err(BoundedActionFault::ByteCredits);
        }
        self.claims[slot] = None;
        self.claimed_items -= 1;
        self.claimed_bytes -= claim.byte_credits;
        self.push_reserved(action);
        Ok(())
    }

    pub(super) fn push_reserved(&mut self, action: BoundedAction) {
        let bytes = self.bytes.checked_add(action.owned_bytes()).expect("reserved action byte credits");
        debug_assert!(self.len < ACTION_QUEUE_ITEM_CAPACITY && bytes <= ACTION_QUEUE_BYTE_CAPACITY);
        let index = (self.head + self.len) % ACTION_QUEUE_ITEM_CAPACITY;
        self.slots[index] = Some(action);
        self.len += 1;
        self.bytes = bytes;
    }

    pub fn pop_front(&mut self) -> Option<BoundedAction> {
        if self.len == 0 {
            return None;
        }
        let action = self.slots[self.head].take();
        self.head = (self.head + 1) % ACTION_QUEUE_ITEM_CAPACITY;
        self.len -= 1;
        if let Some(action) = action.as_ref() {
            self.bytes -= action.owned_bytes();
        }
        action
    }

    pub fn front_batch_len(&self) -> Result<Option<usize>, BoundedActionFault> {
        if self.len == 0 {
            return Ok(None);
        }
        let remaining = self.slots[self.head].as_ref().ok_or(BoundedActionFault::Structure)?.batch_remaining();
        if remaining == 0 || remaining > ACTION_BATCH_ITEM_CAPACITY || remaining > self.len {
            return Err(BoundedActionFault::Structure);
        }
        for offset in 0..remaining {
            let index = (self.head + offset) % ACTION_QUEUE_ITEM_CAPACITY;
            if self.slots[index].as_ref().map(BoundedAction::batch_remaining) != Some(remaining - offset) {
                return Err(BoundedActionFault::Structure);
            }
        }
        Ok(Some(remaining))
    }

    fn shorten_batch_before(&mut self, index: usize) {
        if index >= self.len {
            return;
        }
        let slot = (self.head + index) % ACTION_QUEUE_ITEM_CAPACITY;
        let Some(mut expected) = self.slots[slot].as_ref().map(|action| action.batch_remaining.saturating_add(1)) else { return };
        let mut offset = index;
        while offset > 0 && usize::from(expected) <= ACTION_BATCH_ITEM_CAPACITY {
            offset -= 1;
            let previous = (self.head + offset) % ACTION_QUEUE_ITEM_CAPACITY;
            let Some(action) = self.slots[previous].as_mut() else { break };
            if action.batch_remaining != expected {
                break;
            }
            action.batch_remaining -= 1;
            expected = expected.saturating_add(1);
        }
    }

    pub fn pop_back(&mut self) -> Option<BoundedAction> {
        if self.len == 0 {
            return None;
        }
        self.shorten_batch_before(self.len - 1);
        let index = (self.head + self.len - 1) % ACTION_QUEUE_ITEM_CAPACITY;
        let action = self.slots[index].take();
        self.len -= 1;
        if let Some(action) = action.as_ref() {
            self.bytes -= action.owned_bytes();
        }
        action
    }

    /// 🧾️ Removes one owned action by logical FIFO index while preserving every remaining entry's
    /// order and byte accounting. Token-addressed host journals use this when a later publication
    /// becomes releasable before an earlier refusing surface.
    pub fn remove_at(&mut self, index: usize) -> Option<BoundedAction> {
        if index >= self.len {
            return None;
        }
        self.shorten_batch_before(index);
        let slot = (self.head + index) % ACTION_QUEUE_ITEM_CAPACITY;
        let action = self.slots[slot].take();
        for offset in index..self.len - 1 {
            let current = (self.head + offset) % ACTION_QUEUE_ITEM_CAPACITY;
            let next = (self.head + offset + 1) % ACTION_QUEUE_ITEM_CAPACITY;
            self.slots[current] = self.slots[next].take();
        }
        self.len -= 1;
        if let Some(action) = action.as_ref() {
            self.bytes -= action.owned_bytes();
        }
        action
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0 && self.claimed_items == 0 && self.claimed_bytes == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }

    pub fn claimed_items(&self) -> usize {
        self.claimed_items
    }

    pub fn claimed_bytes(&self) -> usize {
        self.claimed_bytes
    }

    pub fn close_claim_step(&mut self) -> bool {
        let Some(slot) = self.claims.iter().position(Option::is_some) else {
            return true;
        };
        let claim = self.claims[slot].take().expect("claim slot found above");
        self.claimed_items -= 1;
        self.claimed_bytes -= claim.byte_credits;
        false
    }
}
