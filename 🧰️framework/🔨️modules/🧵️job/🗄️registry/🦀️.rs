//! 🧵️ Canonical 🗄️registry implementation under its own native owner.
use semio_framework_job::*;
#[cfg(test)]
const TEST_RETAINED_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:32768,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128};
//#region 🗄️FixedOperationRegistry
/// 🗄️ Typed retained owner admitted to a fixed operation scheduler.
pub trait FixedOperationOwner {
    fn retained_bytes(&self) -> usize;
    fn cancel(&mut self);
    fn begin_close(&mut self);
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪪️ Exact scheduler identity. Reusing an operation id with another generation is never an ACK.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedOperationKey {
    pub operation: OperationId,
    pub generation: Generation,
}

impl FixedOperationKey {
    pub const fn new(operation: OperationId, generation: Generation) -> Self {
        Self { operation, generation }
    }
}

/// ↩️ Failed fixed-registry admission returns the exact owner unchanged.
#[derive(Debug)]
pub struct FixedOperationAdmissionRejected<T> {
    pub key: FixedOperationKey,
    pub owner: T,
}

struct FixedOperationEntry<T> {
    key: FixedOperationKey,
    admitted_bytes: usize,
    closing: bool,
    owner: T,
}

/// 🗄️ Fixed scheduler authority for retained operation owners. Slots and byte credit are admitted
/// once; no operation can resize the registry or detach an owner without its exact identity.
pub struct FixedOperationRegistry<T, const CAPACITY: usize> {
    slots: Box<[Option<FixedOperationEntry<T>>]>,
    maximum_bytes: usize,
    retained_bytes: usize,
    occupied: usize,
    close_cursor: usize,
    allocation_admitted: bool,
}

impl<T: FixedOperationOwner, const CAPACITY: usize> FixedOperationRegistry<T, CAPACITY> {
    pub const MAXIMUM_SLOTS: usize = 64;

    pub fn new(maximum_bytes: usize) -> Self {
        let mut slots = Vec::new();
        let allocation_admitted = CAPACITY > 0 && CAPACITY <= Self::MAXIMUM_SLOTS && slots.try_reserve_exact(CAPACITY).is_ok();
        if allocation_admitted {
            slots.resize_with(CAPACITY, || None);
        }
        Self { slots: slots.into_boxed_slice(), maximum_bytes, retained_bytes: 0, occupied: 0, close_cursor: 0, allocation_admitted }
    }

    fn index(&self, key: FixedOperationKey) -> usize {
        ((key.operation.0 ^ key.generation.0.rotate_left(17)) as usize) % CAPACITY.max(1)
    }

    pub fn can_admit(&self, key: FixedOperationKey, retained_bytes: usize) -> bool {
        let Some(next_retained_bytes) = self.retained_bytes.checked_add(retained_bytes) else { return false };
        if !self.allocation_admitted || self.occupied == CAPACITY || next_retained_bytes > self.maximum_bytes {
            return false;
        }
        self.slots[self.index(key)].is_none()
    }

    pub fn admit(&mut self, key: FixedOperationKey, owner: T) -> Result<(), FixedOperationAdmissionRejected<T>> {
        let retained_bytes = owner.retained_bytes();
        if !self.can_admit(key, retained_bytes) {
            return Err(FixedOperationAdmissionRejected { key, owner });
        }
        let index = self.index(key);
        self.slots[index] = Some(FixedOperationEntry { key, admitted_bytes: retained_bytes, closing: false, owner });
        self.retained_bytes = self.retained_bytes.checked_add(retained_bytes).expect("fixed operation byte admission was checked before exact owner insertion");
        self.occupied += 1;
        Ok(())
    }

    pub fn get(&self, key: FixedOperationKey) -> Option<&T> {
        self.slots.get(self.index(key))?.as_ref().filter(|entry| entry.key == key && !entry.closing).map(|entry| &entry.owner)
    }

    pub fn get_mut(&mut self, key: FixedOperationKey) -> Option<&mut T> {
        let index = self.index(key);
        self.slots.get_mut(index)?.as_mut().filter(|entry| entry.key == key && !entry.closing).map(|entry| &mut entry.owner)
    }

    pub fn get_operation(&self, operation: OperationId) -> Option<(FixedOperationKey, &T)> {
        self.slots.iter().flatten().find(|entry| entry.key.operation == operation && !entry.closing).map(|entry| (entry.key, &entry.owner))
    }

    pub fn get_operation_mut(&mut self, operation: OperationId) -> Option<(FixedOperationKey, &mut T)> {
        self.slots.iter_mut().flatten().find(|entry| entry.key.operation == operation && !entry.closing).map(|entry| (entry.key, &mut entry.owner))
    }

    pub fn take(&mut self, key: FixedOperationKey) -> Option<T> {
        let index = self.index(key);
        if self.slots.get(index)?.as_ref().is_none_or(|entry| entry.key != key || entry.closing) {
            return None;
        }
        let entry = self.slots[index].take().expect("exact fixed operation owner remains admitted");
        self.retained_bytes -= entry.admitted_bytes;
        self.occupied -= 1;
        Some(entry.owner)
    }

    pub fn cancel(&mut self, key: FixedOperationKey) -> bool {
        let index = self.index(key);
        let Some(entry) = self.slots.get_mut(index).and_then(Option::as_mut).filter(|entry| entry.key == key) else { return false };
        entry.owner.cancel();
        entry.owner.begin_close();
        entry.closing = true;
        true
    }

    pub fn cancel_stale_step(&mut self, operation: OperationId, live_generation: Generation) -> bool {
        if !self.allocation_admitted {
            return false;
        }
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % CAPACITY;
        let Some(entry) = self.slots[index].as_mut() else { return false };
        if entry.key.operation != operation || entry.key.generation == live_generation {
            return false;
        }
        entry.owner.cancel();
        entry.owner.begin_close();
        entry.closing = true;
        true
    }

    pub fn begin_close_step(&mut self) -> bool {
        if !self.allocation_admitted {
            return false;
        }
        let index = self.close_cursor;
        self.close_cursor = (self.close_cursor + 1) % CAPACITY;
        let Some(entry) = self.slots[index].as_mut() else { return false };
        if !entry.closing {
            entry.owner.cancel();
            entry.owner.begin_close();
            entry.closing = true;
        }
        true
    }

    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        if self.is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        if !self.allocation_admitted{return Ok(InteractiveJobCloseStep::Blocked)}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let index=self.close_cursor;let idle=self.slots[index].as_ref().is_none_or(|entry|!entry.closing);let terminal=self.slots[index].as_ref().is_some_and(|entry|entry.closing&&entry.owner.terminal_is_empty());
        if idle||terminal{
            let progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(progress)?;cx.consume_fuel(1);
            if terminal{let entry=self.slots[index].take().expect("terminal fixed operation keeps its original slot");self.retained_bytes-=entry.admitted_bytes;self.occupied-=1;drop(entry)}
            self.close_cursor=(index+1)%CAPACITY;return Ok(if self.is_empty(){InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}})
        }
        let entry=self.slots[index].as_mut().unwrap();let step=entry.owner.close_step(cx)?.admit(grant,entry.owner.terminal_is_empty())?;
        Ok(match step{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})
    }

    pub fn is_empty(&self) -> bool {
        self.occupied == 0
    }

    pub fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }
}

impl<T, const CAPACITY: usize> Drop for FixedOperationRegistry<T, CAPACITY> {
    fn drop(&mut self) {
        assert_eq!(self.occupied, 0, "fixed operation registry reached Drop before every exact owner was terminal-empty");
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️fixed-operation-registry/🦀️.rs"]
mod fixed_operation_registry_tests;
//#endregion 🗄️FixedOperationRegistry
