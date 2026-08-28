//! 🎟️ Neutral resident authority shared by retained document roots and their output obligations.

use std::mem::size_of;
use std::sync::{atomic::{AtomicU8, Ordering}, Mutex, MutexGuard, TryLockError};

//#region 🎟️Ledger
pub const UI_RESIDENT_SLOTS: usize = 64;
pub const UI_RESIDENT_SURFACE_BYTES: usize = 8 * 1024 * 1024;
pub const UI_RESIDENT_AGGREGATE_BYTES: usize = 4 * UI_RESIDENT_SURFACE_BYTES;
pub const UI_RESIDENT_SURFACE_ITEMS: usize = 4097;
pub const UI_RESIDENT_AGGREGATE_ITEMS: usize = 131076;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiResidentLimits { pub items: usize, pub bytes: usize }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiResidentSnapshot { pub items: usize, pub bytes: usize, pub used_slots: usize }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiResidentProgress { pub progressed: bool, pub complete: bool, pub released_permits: usize, pub returned_items: usize, pub returned_bytes: usize }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiResidentFault { Capacity, InvalidLimits, Contended, Poisoned, Owner, CounterOverflow, StaticBacking }

impl UiResidentFault {
    pub const fn reason(self) -> &'static str {
        match self { Self::Capacity => "resident capacity exhausted", Self::InvalidLimits => "resident limits exceed the fixed contract", Self::Contended => "resident ledger is busy", Self::Poisoned => "resident ledger is poisoned", Self::Owner => "resident permit is not the exact active owner", Self::CounterOverflow => "resident ledger counter overflow", Self::StaticBacking => "runtime resident backing differs from its admitted domain" }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct UiResidentKey { pub(crate) slot: usize, pub(crate) epoch: u64 }

#[derive(Clone, Copy)]
struct ResidentSlot { epoch: u64, limits: UiResidentLimits, owners: u8 }
impl ResidentSlot { const EMPTY: Self = Self { epoch: 0, limits: UiResidentLimits { items: 0, bytes: 0 }, owners: 0 }; }

struct ResidentLedger { slots: [ResidentSlot; UI_RESIDENT_SLOTS], snapshot: UiResidentSnapshot, cursor: usize, runtime_backing: usize }
const CONTRACT_BACKING_BYTES: usize = size_of::<Mutex<ResidentLedger>>() + size_of::<[AtomicU8; UI_RESIDENT_SLOTS]>() + super::resident_static_backing_bytes() + crate::action::resident_static_backing_bytes();
const _: () = assert!(CONTRACT_BACKING_BYTES < UI_RESIDENT_AGGREGATE_BYTES);
static RESIDENT_LEDGER: Mutex<ResidentLedger> = Mutex::new(ResidentLedger { slots: [ResidentSlot::EMPTY; UI_RESIDENT_SLOTS], snapshot: UiResidentSnapshot { items: 0, bytes: CONTRACT_BACKING_BYTES, used_slots: 0 }, cursor: 0, runtime_backing: 0 });
static RETURNS: [AtomicU8; UI_RESIDENT_SLOTS] = [const { AtomicU8::new(0) }; UI_RESIDENT_SLOTS];

fn ledger() -> Result<MutexGuard<'static, ResidentLedger>, UiResidentFault> {
    RESIDENT_LEDGER.try_lock().map_err(|error| match error { TryLockError::WouldBlock => UiResidentFault::Contended, TryLockError::Poisoned(_) => UiResidentFault::Poisoned })
}

impl ResidentLedger {
    fn exact(&self, key: UiResidentKey, limits: UiResidentLimits, owner: u8) -> Result<&ResidentSlot, UiResidentFault> {
        let slot = self.slots.get(key.slot).ok_or(UiResidentFault::Owner)?;
        if !matches!(owner, 1 | 2) || slot.epoch != key.epoch || slot.limits != limits || slot.owners & owner != owner { return Err(UiResidentFault::Owner); }
        Ok(slot)
    }
    fn release(&mut self, key: UiResidentKey, limits: UiResidentLimits, owner: u8) -> Result<UiResidentProgress, UiResidentFault> {
        let remaining = self.exact(key, limits, owner)?.owners & !owner;
        let mut step = UiResidentProgress { progressed: true, complete: true, released_permits: 1, ..Default::default() };
        if remaining == 0 {
            let items = self.snapshot.items.checked_sub(limits.items).ok_or(UiResidentFault::CounterOverflow)?;
            let bytes = self.snapshot.bytes.checked_sub(limits.bytes).ok_or(UiResidentFault::CounterOverflow)?;
            let used_slots = self.snapshot.used_slots.checked_sub(1).ok_or(UiResidentFault::CounterOverflow)?;
            self.snapshot = UiResidentSnapshot { items, bytes, used_slots };
            self.slots[key.slot].limits = UiResidentLimits::default();
            step.returned_items = limits.items; step.returned_bytes = limits.bytes;
        }
        self.slots[key.slot].owners = remaining;
        Ok(step)
    }
}
//#endregion 🎟️Ledger

//#region 🪪️Permit
#[derive(Debug)]
pub struct UiResidentPermit { pub(crate) key: Option<UiResidentKey>, limits: UiResidentLimits, owner: u8 }

/// 🔎️ Read-only ledger observation neither constructs permits nor grants publication authority.
pub struct UiResidentObservation { guard: MutexGuard<'static, ResidentLedger> }
impl UiResidentObservation {
    pub fn snapshot(&self) -> UiResidentSnapshot { self.guard.snapshot }
    pub fn owns(&self, permit: &UiResidentPermit) -> bool { permit.key.is_some_and(|key| self.guard.exact(key, permit.limits, permit.owner).is_ok()) }
}

impl UiResidentPermit {
    pub const fn required_reservation_bytes() -> usize { size_of::<Self>() + size_of::<ResidentSlot>() }
    pub const fn contract_backing_bytes() -> usize { CONTRACT_BACKING_BYTES }
    pub fn fixed_backing_bytes() -> Result<usize, UiResidentFault> { Ok(CONTRACT_BACKING_BYTES + ledger()?.runtime_backing) }

    /// 🗃️ One runtime domain joins the same aggregate without consuming a dynamic root slot.
    pub fn try_register_runtime_backing(bytes: usize, admitted_bytes: usize) -> Result<bool, UiResidentFault> {
        if bytes == 0 || admitted_bytes < size_of::<usize>() * 2 { return Ok(false); }
        let mut ledger = ledger()?;
        if ledger.runtime_backing != 0 {
            return if ledger.runtime_backing == bytes { Ok(true) } else { Err(UiResidentFault::StaticBacking) };
        }
        let total = ledger.snapshot.bytes.checked_add(bytes).filter(|total| *total <= UI_RESIDENT_AGGREGATE_BYTES).ok_or(UiResidentFault::Capacity)?;
        ledger.runtime_backing = bytes;
        ledger.snapshot.bytes = total;
        Ok(true)
    }
    pub fn limits(&self) -> UiResidentLimits { self.limits }
    pub(crate) fn root_key(&self) -> Option<UiResidentKey> { if self.owner == 1 { self.key } else { None } }
    pub fn terminal_is_empty(&self) -> bool { self.key.is_none() }
    pub fn snapshot() -> Result<UiResidentSnapshot, UiResidentFault> { Ok(ledger()?.snapshot) }
    pub fn try_observe() -> Result<UiResidentObservation, UiResidentFault> { Ok(UiResidentObservation { guard: ledger()? }) }
    pub fn has_pending_returns() -> bool { RETURNS.iter().any(|pending| pending.load(Ordering::Acquire) != 0) }

    /// 🎟️ The fixed metadata reservation is admitted before mutating aggregate credit.
    pub fn try_reserve(limits: UiResidentLimits, target: &mut Option<Self>, admitted_bytes: usize) -> Result<bool, UiResidentFault> {
        if target.is_some() || admitted_bytes < Self::required_reservation_bytes() { return Ok(false); }
        if limits.items > UI_RESIDENT_SURFACE_ITEMS || limits.bytes > UI_RESIDENT_SURFACE_BYTES { return Err(UiResidentFault::InvalidLimits); }
        let mut ledger = ledger()?;
        let items = ledger.snapshot.items.checked_add(limits.items).filter(|total| *total <= UI_RESIDENT_AGGREGATE_ITEMS).ok_or(UiResidentFault::Capacity)?;
        let bytes = ledger.snapshot.bytes.checked_add(limits.bytes).filter(|total| *total <= UI_RESIDENT_AGGREGATE_BYTES).ok_or(UiResidentFault::Capacity)?;
        let index = ledger.slots.iter().enumerate().position(|(index, slot)| slot.owners == 0 && slot.epoch != u64::MAX && RETURNS[index].load(Ordering::Acquire) == 0).ok_or(UiResidentFault::Capacity)?;
        let epoch = ledger.slots[index].epoch.checked_add(1).ok_or(UiResidentFault::CounterOverflow)?;
        let used_slots = ledger.snapshot.used_slots.checked_add(1).ok_or(UiResidentFault::CounterOverflow)?;
        ledger.slots[index] = ResidentSlot { epoch, limits, owners: 1 };
        ledger.snapshot = UiResidentSnapshot { items, bytes, used_slots };
        *target = Some(Self { key: Some(UiResidentKey { slot: index, epoch }), limits, owner: 1 });
        Ok(true)
    }

    pub fn try_shrink(&mut self, limits: UiResidentLimits) -> Result<bool, UiResidentFault> {
        let key = self.key.ok_or(UiResidentFault::Owner)?;
        if limits.items > self.limits.items || limits.bytes > self.limits.bytes { return Err(UiResidentFault::InvalidLimits); }
        let mut ledger = ledger()?;
        if self.owner != 1 || ledger.exact(key, self.limits, self.owner)?.owners != 1 { return Err(UiResidentFault::Owner); }
        let items = ledger.snapshot.items.checked_sub(self.limits.items).and_then(|total| total.checked_add(limits.items)).ok_or(UiResidentFault::CounterOverflow)?;
        let bytes = ledger.snapshot.bytes.checked_sub(self.limits.bytes).and_then(|total| total.checked_add(limits.bytes)).ok_or(UiResidentFault::CounterOverflow)?;
        ledger.snapshot.items = items; ledger.snapshot.bytes = bytes;
        ledger.slots[key.slot].limits = limits;
        self.limits = limits;
        Ok(true)
    }

    pub fn split_output_into(&mut self, target: &mut Option<Self>, admitted_bytes: usize) -> Result<bool, UiResidentFault> {
        if target.is_some() || admitted_bytes < size_of::<Self>() { return Ok(false); }
        let key = self.key.ok_or(UiResidentFault::Owner)?;
        let mut ledger = ledger()?;
        if self.owner != 1 || ledger.exact(key, self.limits, self.owner)?.owners != 1 { return Err(UiResidentFault::Owner); }
        ledger.slots[key.slot].owners = 3;
        *target = Some(Self { key: Some(key), limits: self.limits, owner: 2 });
        Ok(true)
    }

    /// 📉️ Successful explicit return disarms Drop while holding the exact ledger authority.
    pub fn close_step(&mut self, maximum_items: usize) -> Result<UiResidentProgress, UiResidentFault> {
        let Some(key) = self.key else { return Ok(UiResidentProgress { complete: true, ..Default::default() }); };
        if maximum_items == 0 { return Ok(Default::default()); }
        let mut ledger = match ledger() { Ok(ledger) => ledger, Err(UiResidentFault::Contended) => return Ok(Default::default()), Err(error) => return Err(error) };
        let step = ledger.release(key, self.limits, self.owner)?;
        self.key = None;
        Ok(step)
    }

    /// 🔁️ One fixed registry position and at most one returned affine owner are consumed per call.
    pub fn drain_one() -> Result<UiResidentProgress, UiResidentFault> {
        let mut ledger = match ledger() { Ok(ledger) => ledger, Err(UiResidentFault::Contended) => return Ok(Default::default()), Err(error) => return Err(error) };
        let index = ledger.cursor;
        let pending = RETURNS[index].load(Ordering::Acquire);
        let owner = if pending & 1 != 0 { 1 } else if pending & 2 != 0 { 2 } else { 0 };
        if owner == 0 { ledger.cursor = (index + 1) % UI_RESIDENT_SLOTS; return Ok(UiResidentProgress { progressed: true, complete: ledger.snapshot.used_slots == 0, ..Default::default() }); }
        let slot = ledger.slots[index];
        let mut step = ledger.release(UiResidentKey { slot: index, epoch: slot.epoch }, slot.limits, owner)?;
        RETURNS[index].fetch_and(!owner, Ordering::AcqRel);
        ledger.cursor = (index + 1) % UI_RESIDENT_SLOTS;
        step.complete = ledger.snapshot.used_slots == 0;
        Ok(step)
    }
}

impl Drop for UiResidentPermit {
    fn drop(&mut self) {
        if let Some(key) = self.key.take() {
            let previous = RETURNS[key.slot].fetch_or(self.owner, Ordering::Release);
            if previous & self.owner != 0 && !std::thread::panicking() { panic!("resident affine permit returned twice"); }
        }
    }
}
//#endregion 🪪️Permit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;
//#endregion 🧪️Tests
