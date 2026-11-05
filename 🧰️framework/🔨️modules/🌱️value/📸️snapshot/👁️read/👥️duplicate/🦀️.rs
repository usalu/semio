//! 👥️ Funded fixed-slot duplication of one exact captured immutable root.
use crate::*;

impl<T: Send + Sync + 'static> SnapshotRead<T> {
    /// 🧮️ Quotes the returned read, occupied slot, free-ring metadata and generation publication.
    pub fn duplicate_original_demands() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>() + size_of::<SnapshotReadLeaseSlot>() + 2 * size_of::<usize>() + 3 * size_of::<u64>(), depth: 2, ..Default::default() }
    }

    /// 🪪️ Publishes into caller custody before returning effects; the caller reserves and debits the reported work.
    pub fn duplicate_original(&self, output: &mut Option<Self>, grant: RetainedCloneGrant) -> RetainedWorkOutcome<RetainedCloneProgress> {
        let empty = RetainedCloneProgress::default();
        let failure = |message, work| RetainedWorkOutcome { result: Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)), work };
        if output.is_some() { return failure("original read duplication requires vacant output custody", 0); }
        let demand = Self::duplicate_original_demands();
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_depth < demand.depth { return RetainedWorkOutcome { result: Ok(empty), work: 0 }; }
        let result = (|| {
            let owner = self.owner.as_ref().ok_or_else(|| ValueError::invariant("original typed read has no captured root"))?;
            let lease = self.lease.as_ref().ok_or_else(|| ValueError::invariant("original typed read has no captured lease"))?;
            let mut state = lease.registry.state.try_lock().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "original typed read issuer is busy"))?;
            let index = lease.index as usize;
            if index >= SNAPSHOT_READ_LEASE_CAPACITY || !state.slots.resident() || state.occupied[index / 64] & (1 << (index % 64)) == 0 || lease.registry.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) != lease.generation { return Err(ValueError::invariant("original typed read generation is unavailable")); }
            let slot = unsafe { state.slots[index].assume_init_ref() };
            if slot.generation != lease.generation || Arc::as_ptr(&slot.owner) as *const () != Arc::as_ptr(owner) as *const () { return Err(ValueError::invariant("original typed read issuer root changed")); }
            if state.free_len == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original typed read issuer has no vacant slot")); }
            let generation = state.next_generation.checked_add(1).filter(|generation| *generation < 1u64 << 63).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original typed read generation exhausted"))?;
            let erased = Arc::clone(&slot.owner);
            let next = state.free[state.free_read] as usize;
            state.free_read = (state.free_read + 1) % SNAPSHOT_READ_LEASE_CAPACITY;
            state.free_len -= 1;
            state.next_generation = generation;
            state.slots[next].write(SnapshotReadLeaseSlot { generation, owner: erased });
            lease.registry.lease_generations[next].store(generation, std::sync::atomic::Ordering::Release);
            state.occupied[next / 64] |= 1 << (next % 64);
            *output = Some(Self { owner: Some(Arc::clone(owner)), lease: Some(SnapshotReadLease { registry: lease.registry.clone(), index: next as u16, generation }) });
            Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })
        })();
        RetainedWorkOutcome { result, work: 1 }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
