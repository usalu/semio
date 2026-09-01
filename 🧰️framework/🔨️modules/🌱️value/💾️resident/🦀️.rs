//! 💾️ Canonical composition resident capacity and exact allocation ownership.

//#region 📏️Capacity
pub const RESIDENT_MAXIMUM_COUNT: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentFault {
    Count,
    Capacity,
    Identity,
    Allocation,
    Poisoned,
    Busy,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentResources {
    bytes: u64,
    slots: u64,
    owners: u64,
}

impl ResidentResources {
    pub fn new(bytes: u64, slots: u64, owners: u64) -> Result<Self, ResidentFault> {
        if bytes > RESIDENT_MAXIMUM_COUNT || slots > RESIDENT_MAXIMUM_COUNT || owners > RESIDENT_MAXIMUM_COUNT { return Err(ResidentFault::Count); }
        Ok(Self { bytes, slots, owners })
    }

    pub fn bytes(self) -> u64 { self.bytes }
    pub fn slots(self) -> u64 { self.slots }
    pub fn owners(self) -> u64 { self.owners }

    pub fn checked_add(self, other: Self) -> Result<Self, ResidentFault> {
        Self::new(self.bytes.checked_add(other.bytes).ok_or(ResidentFault::Count)?, self.slots.checked_add(other.slots).ok_or(ResidentFault::Count)?, self.owners.checked_add(other.owners).ok_or(ResidentFault::Count)?)
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, ResidentFault> {
        Self::new(self.bytes.checked_sub(other.bytes).ok_or(ResidentFault::Capacity)?, self.slots.checked_sub(other.slots).ok_or(ResidentFault::Capacity)?, self.owners.checked_sub(other.owners).ok_or(ResidentFault::Capacity)?)
    }

    pub fn fits_within(self, capacity: Self) -> bool {
        self.bytes <= capacity.bytes && self.slots <= capacity.slots && self.owners <= capacity.owners
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentCapacity {
    total: ResidentResources,
    control: ResidentResources,
}

impl ResidentCapacity {
    pub fn new(total: ResidentResources, control: ResidentResources) -> Result<Self, ResidentFault> {
        if !control.fits_within(total) { return Err(ResidentFault::Capacity); }
        Ok(Self { total, control })
    }

    pub fn total(self) -> ResidentResources { self.total }
    pub fn control(self) -> ResidentResources { self.control }
    pub fn data(self) -> ResidentResources {
        ResidentResources { bytes: self.total.bytes - self.control.bytes, slots: self.total.slots - self.control.slots, owners: self.total.owners - self.control.owners }
    }
}
//#endregion 📏️Capacity

//#region 📨️AdmissionVocabulary
use std::{alloc::Layout, any::TypeId, cell::UnsafeCell, marker::PhantomData, mem::size_of, num::NonZeroU64, ops::{Deref, DerefMut}, ptr::NonNull, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentPartition { Data, Control }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentStepKind { Blocked, Pending, Ready, Complete, Rejected }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentGrant { items: u64, bytes: u64 }

impl ResidentGrant {
    pub fn new(items: u64, bytes: u64) -> Result<Self, ResidentFault> {
        ResidentResources::new(bytes, items, 0)?;
        Ok(Self { items, bytes })
    }
    pub fn max_items(self) -> u64 { self.items }
    pub fn max_bytes(self) -> u64 { self.bytes }
    fn admits(self, bytes: u64) -> bool { self.items > 0 && self.bytes >= bytes }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentStep { pub kind: ResidentStepKind, pub items: u64, pub bytes: u64 }

impl ResidentStep {
    fn blocked() -> Self { Self { kind: ResidentStepKind::Blocked, items: 0, bytes: 0 } }
    fn rejected() -> Self { Self { kind: ResidentStepKind::Rejected, items: 0, bytes: 0 } }
    fn done(kind: ResidentStepKind, bytes: u64) -> Self { Self { kind, items: 1, bytes } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentNativeLayout {
    pub root_bytes: u64,
    pub admission_page_bytes: u64,
    pub consumer_page_bytes: u64,
    pub record_page_bytes: u64,
    pub consumer_move_bytes: u64,
    pub shell_move_bytes: u64,
    pub descriptor_move_bytes: u64,
    pub final_root_bytes: u64,
    pub release_slot_bytes: u64,
    pub pending_consumer_bytes: u64,
}
//#endregion 📨️AdmissionVocabulary

//#region 🚦️AllocationFreeAccess
struct ResidentAccess<T> { held: AtomicBool, poisoned: AtomicBool, value: UnsafeCell<T> }
struct ResidentAccessGuard<'a, T> { access: &'a ResidentAccess<T>, thread: PhantomData<*mut ()> }
enum ResidentAccessError { Busy, Poisoned }

unsafe impl<T: Send> Send for ResidentAccess<T> {}
unsafe impl<T: Send> Sync for ResidentAccess<T> {}

impl<T> ResidentAccess<T> {
    const fn new(value: T) -> Self { Self { held: AtomicBool::new(false), poisoned: AtomicBool::new(false), value: UnsafeCell::new(value) } }
    fn try_lock(&self) -> Result<ResidentAccessGuard<'_, T>, ResidentAccessError> {
        if self.held.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() { return Err(ResidentAccessError::Busy); }
        if self.poisoned.load(Ordering::Relaxed) { self.held.store(false, Ordering::Release); return Err(ResidentAccessError::Poisoned); }
        Ok(ResidentAccessGuard { access: self, thread: PhantomData })
    }
}

impl<T> Deref for ResidentAccessGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.access.value.get() } }
}

impl<T> DerefMut for ResidentAccessGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.access.value.get() } }
}

impl<T> Drop for ResidentAccessGuard<'_, T> {
    fn drop(&mut self) {
        if std::thread::panicking() { self.access.poisoned.store(true, Ordering::Relaxed); }
        self.access.held.store(false, Ordering::Release);
    }
}
//#endregion 🚦️AllocationFreeAccess

//#region 🏠️OriginalRoot
pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }

enum ResidentReleaseOrigin { Record, PendingAdmission, Admission, PendingConsumer, Consumer, PrimaryPending { registration: NonZeroU64 }, PrimaryConsumer { registration: NonZeroU64 } }
struct ResidentReleaseAllocation { pointer: NonNull<u8>, layout: Layout }
enum ResidentReleaseStage {
    Destroy { allocation: ResidentReleaseAllocation, destroy_empty: unsafe fn(NonNull<u8>) },
    Free { allocation: ResidentReleaseAllocation },
    Refund { released_layout: Option<Layout> },
    Clear { released_layout: Option<Layout> },
}
struct ResidentRelease {
    origin: ResidentReleaseOrigin,
    partition: ResidentPartition,
    charge: ResidentResources,
    stage: ResidentReleaseStage,
}

impl ResidentReleaseStage {
    fn allocated(pointer: NonNull<u8>, layout: Layout, initialized: bool, destroy_empty: unsafe fn(NonNull<u8>)) -> Self {
        let allocation = ResidentReleaseAllocation { pointer, layout };
        if initialized { Self::Destroy { allocation, destroy_empty } } else { Self::Free { allocation } }
    }
    fn pointerless(&self) -> bool { matches!(self, Self::Refund { .. } | Self::Clear { .. }) }
}

fn resident_release_work(parts: &[usize]) -> Result<u64, ResidentFault> {
    parts.iter().try_fold(0u64, |sum, bytes| sum.checked_add(*bytes as u64).ok_or(ResidentFault::Count))
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ResidentRegistrationStamp { generation: NonZeroU64, type_id: TypeId }

enum ResidentPrimaryBacking { Pending(ConsumerPage), Published(NonNull<ConsumerHeader>), Releasing }
struct ResidentPrimaryAnchor { stamp: ResidentRegistrationStamp, partition: ResidentPartition, backing: ResidentPrimaryBacking }
struct ResidentRecoveryPin { pointer: NonNull<ConsumerHeader>, registration: NonZeroU64 }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentRecoveryMode { Forward, Closing }

struct ResidentRecoveryCursor {
    stamp: ResidentRegistrationStamp,
    mode: ResidentRecoveryMode,
    revoked: bool,
    next: Option<ResidentRecoveryPin>,
    found: Option<ResidentRecoveryPin>,
}

struct LedgerState {
    capacity: ResidentCapacity,
    data: ResidentResources,
    control: ResidentResources,
    allocated_bytes: u64,
    head: Option<AdmissionPage>,
    pending: Option<PendingAdmission>,
    prepared: Option<NonNull<AdmissionNode>>,
    release: Option<ResidentRelease>,
    consumers: Option<ConsumerPage>,
    pending_consumer: Option<ConsumerPage>,
    prepared_consumer: Option<NonNull<ConsumerHeader>>,
    last_consumer_registration: u64,
    primary: Option<ResidentPrimaryAnchor>,
    recovery: Option<ResidentRecoveryCursor>,
    closing: bool,
    closed: bool,
    #[cfg(test)]
    consumer_release_interlock: Option<ConsumerReleaseInterlock>,
}

#[cfg(test)]
struct ConsumerReleaseInterlock { observed: std::sync::mpsc::SyncSender<()>, resume: std::sync::mpsc::Receiver<()> }

pub struct ResidentLedger<'root> { root: &'root ResidentLedgerRoot }

impl ResidentAccess<LedgerState> {
    fn try_lock_close(&self) -> Result<ResidentAccessGuard<'_, LedgerState>, ResidentAccessError> {
        if self.held.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() { return Err(ResidentAccessError::Busy); }
        let guard = ResidentAccessGuard { access: self, thread: PhantomData };
        if self.poisoned.load(Ordering::Relaxed) && !guard.pointerless_close_allowed() && !guard.structurally_empty() {
            return Err(ResidentAccessError::Poisoned);
        }
        Ok(guard)
    }
}

impl ResidentLedgerRoot {
    pub fn new(capacity: ResidentCapacity) -> Self {
        let zero = ResidentResources { bytes: 0, slots: 0, owners: 0 };
        Self { state: ResidentAccess::new(LedgerState { capacity, data: zero, control: zero, allocated_bytes: 0, head: None, pending: None, prepared: None, release: None, consumers: None, pending_consumer: None, prepared_consumer: None, last_consumer_registration: 0, primary: None, recovery: None, closing: false, closed: false, #[cfg(test)] consumer_release_interlock: None }) }
    }

    pub fn ledger(&self) -> ResidentLedger<'_> { ResidentLedger { root: self } }

    pub fn native_layout<C, S>(&self) -> ResidentNativeLayout {
        ResidentNativeLayout { root_bytes: size_of::<Self>() as u64, admission_page_bytes: size_of::<AdmissionNode>() as u64, consumer_page_bytes: size_of::<ConsumerNode<C>>() as u64, record_page_bytes: size_of::<RecordNode<S>>() as u64, consumer_move_bytes: size_of::<Option<C>>() as u64, shell_move_bytes: size_of::<Option<S>>() as u64, descriptor_move_bytes: size_of::<ErasedRecord>().max(size_of::<Option<AdmissionPage>>()).max(size_of::<ConsumerPage>()).max(size_of::<Option<ResidentRelease>>()).max(size_of::<Option<ResidentPrimaryAnchor>>()).max(size_of::<Option<ResidentRecoveryCursor>>()) as u64, final_root_bytes: size_of::<Self>() as u64, release_slot_bytes: size_of::<Option<ResidentRelease>>() as u64, pending_consumer_bytes: size_of::<Option<ConsumerPage>>() as u64 }
    }

    pub fn allocated_bytes(&self) -> Result<u64, ResidentFault> { self.access()?.map(|state| state.allocated_bytes).ok_or(ResidentFault::Busy) }
    pub fn usage(&self, partition: ResidentPartition) -> Result<ResidentResources, ResidentFault> { self.access()?.map(|state| state.used(partition)).ok_or(ResidentFault::Busy) }
    pub fn begin_close(&self) -> Result<bool, ResidentFault> { let Some(mut state) = self.access()? else { return Ok(false); }; state.closing = true; Ok(true) }
    pub fn terminal_is_empty(&self) -> bool { self.state.try_lock_close().map(|state| state.closed && state.structurally_empty()).unwrap_or(false) }

    fn access(&self) -> Result<Option<ResidentAccessGuard<'_, LedgerState>>, ResidentFault> {
        match self.state.try_lock() { Ok(state) => Ok(Some(state)), Err(ResidentAccessError::Busy) => Ok(None), Err(ResidentAccessError::Poisoned) => Err(ResidentFault::Poisoned) }
    }

    pub fn close_step(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let mut state = match self.state.try_lock_close() {
            Ok(state) => state,
            Err(ResidentAccessError::Busy) => return Ok(ResidentStep::blocked()),
            Err(ResidentAccessError::Poisoned) => return Err(ResidentFault::Poisoned),
        };
        if state.closed {
            if !state.structurally_empty() { return Err(ResidentFault::Capacity); }
            return Ok(ResidentStep { kind: ResidentStepKind::Complete, items: 0, bytes: 0 });
        }
        if !state.closing {
            let bytes = size_of::<bool>() as u64;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            state.closing = true;
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        if state.release.is_some() { return state.advance_release(grant); }
        if state.recovery.is_some() { return state.close_recovery(grant); }
        if let Some(pending) = state.pending.as_mut() {
            if let Some(consumer) = pending.consumer.as_ref() {
                if !consumer.is_empty() { return Ok(ResidentStep::blocked()); }
                let bytes = resident_release_work(&[size_of::<Option<ErasedConsumer>>(), size_of::<AtomicUsize>()])?;
                if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
                drop(pending.consumer.take());
                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
            }
            let bytes = resident_release_work(&[size_of::<Option<PendingAdmission>>(), size_of::<Option<ResidentRelease>>()])?;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            let pending = state.pending.take().unwrap();
            let stage = match pending.page {
                Some(page) => ResidentReleaseStage::allocated(page.pointer.cast(), Layout::new::<AdmissionNode>(), page.initialized, destroy_admission),
                None => ResidentReleaseStage::Refund { released_layout: None },
            };
            state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::PendingAdmission, partition: pending.partition, charge: pending.charge, stage });
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        if let Some(page) = state.head.as_ref() {
            let owner = unsafe { page.pointer.as_ref() };
            let node = unsafe { &mut *owner.fields.get() };
            if let Some(record) = node.record.as_ref() {
                if !record.empty()? || record.aliases() != 0 { return Ok(ResidentStep::blocked()); }
                if record.allocated_bytes != record.layout.size() as u64 { return Err(ResidentFault::Identity); }
                let bytes = resident_release_work(&[size_of::<Option<ErasedRecord>>(), size_of::<Option<ResidentRelease>>()])?;
                if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
                let record = node.record.take().unwrap();
                let stage = ResidentReleaseStage::allocated(record.pointer, record.layout, record.initialized, record.destroy_empty);
                state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::Record, partition: record.partition, charge: record.charge, stage });
                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
            }
            if owner.aliases.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
            if let Some(consumer) = node.consumer.as_ref() {
                if !consumer.is_empty() { return Ok(ResidentStep::blocked()); }
                let bytes = resident_release_work(&[size_of::<Option<ErasedConsumer>>(), size_of::<AtomicUsize>()])?;
                if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
                drop(node.consumer.take());
                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
            }
            let clear_prepared = state.prepared == Some(page.pointer);
            let bytes = resident_release_work(&[size_of::<Option<AdmissionPage>>(), size_of::<Option<AdmissionPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<AdmissionNode>>>() } else { 0 }])?;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            let charge = node.charge; let partition = node.partition;
            let page = state.head.take().unwrap();
            let node = unsafe { &mut *page.pointer.as_ref().fields.get() };
            if clear_prepared { state.prepared = None; }
            state.head = node.next.take();
            let stage = ResidentReleaseStage::allocated(page.pointer.cast(), Layout::new::<AdmissionNode>(), page.initialized, destroy_admission);
            state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::Admission, partition, charge, stage });
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        if state.primary.as_ref().is_some_and(|anchor| matches!(&anchor.backing, ResidentPrimaryBacking::Pending(_))) {
            return state.detach_pending_primary(grant);
        }
        if let Some(page) = state.pending_consumer.as_ref() {
            if page.initialized && page.pointer.is_none_or(|pointer| !unsafe { (page.empty)(pointer) }) { return Ok(ResidentStep::blocked()); }
            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>()])?;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            let page = state.pending_consumer.take().unwrap();
            let stage = match page.pointer {
                Some(pointer) => ResidentReleaseStage::allocated(pointer.cast(), page.layout, page.initialized, page.destroy_empty),
                None => ResidentReleaseStage::Refund { released_layout: None },
            };
            state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::PendingConsumer, partition: page.partition, charge: page.charge, stage });
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        if let Some(page) = state.consumers.as_ref() {
            let pointer = page.pointer.unwrap(); let header = unsafe { pointer.as_ref() };
            if !unsafe { *header.closing.get() } {
                if !grant.admits(size_of::<bool>() as u64) { return Ok(ResidentStep::blocked()); }
                unsafe { *header.closing.get() = true; }
                return Ok(ResidentStep::done(ResidentStepKind::Pending, size_of::<bool>() as u64));
            }
            if !unsafe { (page.empty)(pointer) } { return Ok(ResidentStep::blocked()); }
            let primary = state.primary_for_page(page)?;
            #[cfg(test)]
            if let Some(interlock) = state.consumer_release_interlock.take() {
                interlock.observed.try_send(()).map_err(|_| ResidentFault::Identity)?;
                interlock.resume.recv_timeout(std::time::Duration::from_secs(1)).map_err(|_| ResidentFault::Identity)?;
            }
            if header.aliases.load(Ordering::Acquire) != 0 || header.admissions.load(Ordering::Acquire) != 0 || header.recovery_pins.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
            let clear_prepared = state.prepared_consumer == Some(pointer);
            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<ConsumerHeader>>>() } else { 0 }, if primary.is_some() { size_of::<ResidentPrimaryBacking>() } else { 0 }])?;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            if primary.is_some() { state.primary.as_mut().unwrap().backing = ResidentPrimaryBacking::Releasing; }
            let page = state.consumers.take().unwrap();
            state.consumers = unsafe { (&mut *header.next.get()).take() };
            if clear_prepared { state.prepared_consumer = None; }
            let stage = ResidentReleaseStage::allocated(pointer.cast(), page.layout, true, page.destroy_empty);
            let origin = primary.map_or(ResidentReleaseOrigin::Consumer, |registration| ResidentReleaseOrigin::PrimaryConsumer { registration });
            state.release = Some(ResidentRelease { origin, partition: page.partition, charge: page.charge, stage });
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        if !state.structurally_empty() { return Err(ResidentFault::Capacity); }
        let bytes = size_of::<Self>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        state.closed = true;
        Ok(ResidentStep::done(ResidentStepKind::Complete, bytes))
    }
}

impl LedgerState {
    fn primary_for_page(&self, page: &ConsumerPage) -> Result<Option<NonZeroU64>, ResidentFault> {
        let Some(anchor) = self.primary.as_ref() else { return Ok(None); };
        let ResidentPrimaryBacking::Published(pointer) = &anchor.backing else { return Ok(None); };
        let same_pointer = page.pointer == Some(*pointer);
        let same_registration = page.registration == anchor.stamp.generation;
        if same_pointer != same_registration { return Err(ResidentFault::Identity); }
        if !same_pointer { return Ok(None); }
        if page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
        Ok(Some(page.registration))
    }

    fn detach_pending_primary(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let anchor = self.primary.as_mut().ok_or(ResidentFault::Identity)?;
        let ResidentPrimaryBacking::Pending(page) = &anchor.backing else { return Ok(ResidentStep::rejected()); };
        if page.registration != anchor.stamp.generation || page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
        let bytes = resident_release_work(&[size_of::<ResidentPrimaryBacking>(), size_of::<Option<ResidentRelease>>()])?;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        if page.initialized && page.pointer.is_none_or(|pointer| !unsafe { (page.empty)(pointer) }) { return Ok(ResidentStep::blocked()); }
        let ResidentPrimaryBacking::Pending(page) = std::mem::replace(&mut anchor.backing, ResidentPrimaryBacking::Releasing) else { unreachable!() };
        let stage = match page.pointer {
            Some(pointer) => ResidentReleaseStage::allocated(pointer.cast(), page.layout, page.initialized, page.destroy_empty),
            None => ResidentReleaseStage::Refund { released_layout: None },
        };
        self.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::PrimaryPending { registration: page.registration }, partition: page.partition, charge: page.charge, stage });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    fn close_recovery(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let cursor = self.recovery.as_mut().ok_or(ResidentFault::Identity)?;
        if !cursor.revoked {
            let bytes = size_of::<bool>() as u64;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            cursor.revoked = true;
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        let slot = if cursor.next.is_some() { &mut cursor.next } else { &mut cursor.found };
        if let Some(pin) = slot.as_ref() {
            let bytes = resident_release_work(&[size_of::<Option<ResidentRecoveryPin>>(), size_of::<AtomicUsize>()])?;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            #[cfg(test)]
            tests::observe_primary_recovery_pointer_load(pin.registration.get());
            let header = unsafe { pin.pointer.as_ref() };
            let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
            header.recovery_pins.store(remaining, Ordering::Release);
            *slot = None;
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        let bytes = size_of::<Option<ResidentRecoveryCursor>>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        self.recovery = None;
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    fn primary_recovery_target<C: Send + 'static>(&self, mode: ResidentRecoveryMode) -> Result<Option<(ResidentRegistrationStamp, NonNull<ConsumerHeader>)>, ResidentFault> {
        if self.closed || self.closing != (mode == ResidentRecoveryMode::Closing) { return Err(ResidentFault::Closed); }
        let Some(anchor) = self.primary.as_ref() else { return Ok(None); };
        if anchor.stamp.type_id != TypeId::of::<C>() { return Err(ResidentFault::Identity); }
        match &anchor.backing { ResidentPrimaryBacking::Published(pointer) => Ok(Some((anchor.stamp, *pointer))), _ => Ok(None) }
    }

    fn pointerless_close_allowed(&self) -> bool {
        self.release.as_ref().is_some_and(|release| release.stage.pointerless()) && self.recovery.is_none() && self.primary.as_ref().is_none_or(|anchor| matches!(&anchor.backing, ResidentPrimaryBacking::Releasing))
    }

    fn structurally_empty(&self) -> bool {
        #[cfg(test)]
        if self.consumer_release_interlock.is_some() { return false; }
        let zero = ResidentResources { bytes: 0, slots: 0, owners: 0 };
        self.head.is_none() && self.pending.is_none() && self.prepared.is_none() && self.release.is_none() && self.consumers.is_none() && self.pending_consumer.is_none() && self.prepared_consumer.is_none() && self.primary.is_none() && self.recovery.is_none() && self.allocated_bytes == 0 && self.data == zero && self.control == zero
    }

    fn advance_release(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let release = self.release.as_mut().ok_or(ResidentFault::Identity)?;
        let slot = size_of::<Option<ResidentRelease>>();
        let primary = match &release.origin { ResidentReleaseOrigin::PrimaryPending { registration } | ResidentReleaseOrigin::PrimaryConsumer { registration } => Some(*registration), _ => None };
        if let Some(registration) = primary {
            if self.primary.as_ref().is_none_or(|anchor| anchor.stamp.generation != registration || anchor.partition != release.partition || !matches!(&anchor.backing, ResidentPrimaryBacking::Releasing)) { return Err(ResidentFault::Identity); }
        }
        match &release.stage {
            ResidentReleaseStage::Destroy { allocation, destroy_empty } => {
                let bytes = resident_release_work(&[allocation.layout.size(), slot])?;
                if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
                let pointer = allocation.pointer; let layout = allocation.layout; let destroy_empty = *destroy_empty;
                unsafe { destroy_empty(pointer); }
                release.stage = ResidentReleaseStage::Free { allocation: ResidentReleaseAllocation { pointer, layout } };
                #[cfg(test)]
                tests::observe_release_destroy_returned();
                Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
            }
            ResidentReleaseStage::Free { allocation } => {
                let bytes = resident_release_work(&[allocation.layout.size(), slot, size_of::<u64>()])?;
                if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
                let remaining = self.allocated_bytes.checked_sub(allocation.layout.size() as u64).ok_or(ResidentFault::Capacity)?;
                let pointer = allocation.pointer; let layout = allocation.layout;
                unsafe { std::alloc::dealloc(pointer.as_ptr(), layout); }
                release.stage = ResidentReleaseStage::Refund { released_layout: Some(layout) };
                self.allocated_bytes = remaining;
                Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
            }
            ResidentReleaseStage::Refund { released_layout } => {
                let bytes = resident_release_work(&[slot, size_of::<ResidentResources>()])?;
                if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
                let used = match release.partition { ResidentPartition::Data => self.data, ResidentPartition::Control => self.control };
                let next = used.checked_sub(release.charge)?;
                let released_layout = *released_layout; let partition = release.partition;
                match partition { ResidentPartition::Data => self.data = next, ResidentPartition::Control => self.control = next }
                release.stage = ResidentReleaseStage::Clear { released_layout };
                Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
            }
            ResidentReleaseStage::Clear { .. } => {
                let bytes = resident_release_work(&[slot, if primary.is_some() { size_of::<Option<ResidentPrimaryAnchor>>() } else { 0 }])?;
                if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
                if primary.is_some() { self.primary = None; }
                self.release = None;
                Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
            }
        }
    }

    fn used(&self, partition: ResidentPartition) -> ResidentResources { match partition { ResidentPartition::Data => self.data, ResidentPartition::Control => self.control } }
    fn reserve(&mut self, partition: ResidentPartition, charge: ResidentResources) -> Result<(), ResidentFault> {
        let next = self.used(partition).checked_add(charge)?;
        let capacity = match partition { ResidentPartition::Data => self.capacity.data(), ResidentPartition::Control => self.capacity.control() };
        if !next.fits_within(capacity) { return Err(ResidentFault::Capacity); }
        match partition { ResidentPartition::Data => self.data = next, ResidentPartition::Control => self.control = next }
        Ok(())
    }
    fn release(&mut self, partition: ResidentPartition, charge: ResidentResources) -> Result<(), ResidentFault> {
        let next = self.used(partition).checked_sub(charge)?;
        match partition { ResidentPartition::Data => self.data = next, ResidentPartition::Control => self.control = next }
        Ok(())
    }
}
//#endregion 🏠️OriginalRoot

//#region 🪪️ConsumerCell
#[repr(C)]
struct ConsumerHeader {
    aliases: AtomicUsize,
    admissions: AtomicUsize,
    recovery_pins: AtomicUsize,
    closing: UnsafeCell<bool>,
    next: UnsafeCell<Option<ConsumerPage>>,
    type_id: TypeId,
    registration: NonZeroU64,
}

#[repr(C)]
struct ConsumerNode<C> { header: ConsumerHeader, source: UnsafeCell<Option<C>> }

struct ConsumerPage {
    pointer: Option<NonNull<ConsumerHeader>>,
    layout: Layout,
    partition: ResidentPartition,
    charge: ResidentResources,
    initialized: bool,
    type_id: TypeId,
    registration: NonZeroU64,
    initialize: unsafe fn(NonNull<ConsumerHeader>, NonZeroU64),
    empty: unsafe fn(NonNull<ConsumerHeader>) -> bool,
    destroy_empty: unsafe fn(NonNull<u8>),
}

/// 🧷️ Typed constructors bind Send sources and exact layouts; pointers remain in the original gated root.
unsafe impl Send for ConsumerPage {}

impl ConsumerPage {
    fn reserved<C: Send + 'static>(partition: ResidentPartition, registration: NonZeroU64) -> Result<Self, ResidentFault> {
        let layout = Layout::new::<ConsumerNode<C>>();
        let charge = ResidentResources::new(layout.size() as u64, 1, 1)?;
        Ok(Self { pointer: None, layout, partition, charge, initialized: false, type_id: TypeId::of::<C>(), registration, initialize: initialize_consumer::<C>, empty: empty_consumer::<C>, destroy_empty: destroy_consumer::<C> })
    }

    fn allocate(&mut self, allocated_bytes: &mut u64, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let bytes = resident_release_work(&[self.layout.size(), size_of::<Option<NonNull<ConsumerHeader>>>(), size_of::<u64>()])?;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        let allocated = allocated_bytes.checked_add(self.layout.size() as u64).ok_or(ResidentFault::Count)?;
        let pointer = NonNull::new(unsafe { std::alloc::alloc(self.layout) }.cast::<ConsumerHeader>()).ok_or(ResidentFault::Allocation)?;
        self.pointer = Some(pointer);
        *allocated_bytes = allocated;
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    fn initialize(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let bytes = resident_release_work(&[self.layout.size(), size_of::<bool>()])?;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        let pointer = self.pointer.ok_or(ResidentFault::Identity)?;
        unsafe { (self.initialize)(pointer, self.registration); }
        self.initialized = true;
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }
}

unsafe fn initialize_consumer<C: Send + 'static>(pointer: NonNull<ConsumerHeader>, registration: NonZeroU64) {
    unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().write(ConsumerNode { header: ConsumerHeader { aliases: AtomicUsize::new(0), admissions: AtomicUsize::new(0), recovery_pins: AtomicUsize::new(0), closing: UnsafeCell::new(false), next: UnsafeCell::new(None), type_id: TypeId::of::<C>(), registration }, source: UnsafeCell::new(None) }); }
}

unsafe fn empty_consumer<C>(pointer: NonNull<ConsumerHeader>) -> bool { unsafe { (&*pointer.cast::<ConsumerNode<C>>().as_ref().source.get()).is_none() } }
unsafe fn destroy_consumer<C>(pointer: NonNull<u8>) { unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().drop_in_place(); } }

pub struct ResidentConsumer<'root, C> { root: &'root ResidentLedgerRoot, pointer: NonNull<ConsumerNode<C>> }
pub struct ResidentConsumerRead<'root, C> { _access: ResidentAccessGuard<'root, LedgerState>, pointer: NonNull<C> }

unsafe impl<C: Send> Send for ResidentConsumer<'_, C> {}
unsafe impl<C: Send> Sync for ResidentConsumer<'_, C> {}

impl<C> Drop for ResidentConsumer<'_, C> {
    fn drop(&mut self) { unsafe { self.pointer.as_ref().header.aliases.fetch_sub(1, Ordering::AcqRel); } }
}

impl<C> Deref for ResidentConsumerRead<'_, C> {
    type Target = C;
    fn deref(&self) -> &C { unsafe { self.pointer.as_ref() } }
}

impl<'root, C: Send + 'static> ResidentConsumer<'root, C> {
    pub fn install(&self, source: &mut Option<C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        let node = unsafe { self.pointer.as_ref() };
        if state.closing || unsafe { *node.header.closing.get() } { return Ok(ResidentStep::rejected()); }
        let target = unsafe { &mut *node.source.get() };
        if source.is_none() || target.is_some() { return Ok(ResidentStep::rejected()); }
        let bytes = size_of::<Option<C>>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        *target = source.take();
        Ok(ResidentStep::done(ResidentStepKind::Ready, bytes))
    }

    pub fn read(&self) -> Result<Option<ResidentConsumerRead<'root, C>>, ResidentFault> {
        let Some(access) = self.root.access()? else { return Err(ResidentFault::Busy); };
        if access.closing || unsafe { *self.pointer.as_ref().header.closing.get() } { return Err(ResidentFault::Closed); }
        self.read_with_access(access)
    }

    pub fn read_for_close(&self) -> Result<Option<ResidentConsumerRead<'root, C>>, ResidentFault> {
        let Some(access) = self.root.access()? else { return Err(ResidentFault::Busy); };
        if !access.closing && !unsafe { *self.pointer.as_ref().header.closing.get() } { return Err(ResidentFault::Identity); }
        self.read_with_access(access)
    }

    fn read_with_access(&self, access: ResidentAccessGuard<'root, LedgerState>) -> Result<Option<ResidentConsumerRead<'root, C>>, ResidentFault> {
        let source = unsafe { &*self.pointer.as_ref().source.get() };
        let Some(value) = source.as_ref() else { return Ok(None); };
        Ok(Some(ResidentConsumerRead { _access: access, pointer: NonNull::from(value) }))
    }

    pub fn begin_close(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(_state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        if !grant.admits(size_of::<bool>() as u64) { return Ok(ResidentStep::blocked()); }
        unsafe { *self.pointer.as_ref().header.closing.get() = true; }
        Ok(ResidentStep::done(ResidentStepKind::Pending, size_of::<bool>() as u64))
    }

    pub fn handoff_for_close_into(&self, target: &mut Option<C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        let node = unsafe { self.pointer.as_ref() };
        if !state.closing && !unsafe { *node.header.closing.get() } { return Ok(ResidentStep::rejected()); }
        let source = unsafe { &mut *node.source.get() };
        if target.is_some() || source.is_none() { return Ok(ResidentStep::rejected()); }
        let bytes = size_of::<Option<C>>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        *target = source.take();
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }
}

impl ResidentLedgerRoot {
    pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
        let state = &mut *state;
        if state.closing || state.closed { return Ok(ResidentStep::rejected()); }
        if let Some(page) = state.pending_consumer.as_mut() {
            if page.type_id != TypeId::of::<C>() || page.partition != partition { return Ok(ResidentStep::rejected()); }
            if page.pointer.is_none() { return page.allocate(&mut state.allocated_bytes, grant); }
            if !page.initialized { return page.initialize(grant); }
            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            let page = state.pending_consumer.take().unwrap();
            let pointer = page.pointer.unwrap();
            unsafe { *pointer.as_ref().next.get() = state.consumers.take(); }
            state.prepared_consumer = Some(pointer);
            state.consumers = Some(page);
            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
        }
        let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<u64>(), size_of::<ResidentResources>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
        let page = ConsumerPage::reserved::<C>(partition, registration)?;
        if state.reserve(partition, page.charge).is_err() { return Ok(ResidentStep::blocked()); }
        state.last_consumer_registration = registration.get();
        state.pending_consumer = Some(page);
        state.prepared_consumer = None;
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn reserve_primary_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
        if state.closing || state.closed || state.primary.is_some() { return Ok(ResidentStep::rejected()); }
        let bytes = resident_release_work(&[size_of::<Option<ResidentPrimaryAnchor>>(), size_of::<u64>(), size_of::<ResidentResources>()])?;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
        let page = ConsumerPage::reserved::<C>(partition, registration)?;
        if state.reserve(partition, page.charge).is_err() { return Ok(ResidentStep::blocked()); }
        state.last_consumer_registration = registration.get();
        state.primary = Some(ResidentPrimaryAnchor { stamp: ResidentRegistrationStamp { generation: registration, type_id: TypeId::of::<C>() }, partition, backing: ResidentPrimaryBacking::Pending(page) });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn prepare_primary_consumer<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
        let state = &mut *state;
        if state.closing || state.closed { return Ok(ResidentStep::rejected()); }
        let Some(anchor) = state.primary.as_mut() else { return Ok(ResidentStep::rejected()); };
        if anchor.stamp.type_id != TypeId::of::<C>() { return Err(ResidentFault::Identity); }
        let ResidentPrimaryBacking::Pending(page) = &mut anchor.backing else { return Ok(ResidentStep::rejected()); };
        if page.registration != anchor.stamp.generation || page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
        if page.pointer.is_none() { return page.allocate(&mut state.allocated_bytes, grant); }
        if !page.initialized { return page.initialize(grant); }
        let bytes = resident_release_work(&[size_of::<ResidentPrimaryBacking>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>()])?;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        let pointer = page.pointer.unwrap();
        let ResidentPrimaryBacking::Pending(page) = std::mem::replace(&mut anchor.backing, ResidentPrimaryBacking::Published(pointer)) else { unreachable!() };
        unsafe { *pointer.as_ref().next.get() = state.consumers.take(); }
        state.consumers = Some(page);
        Ok(ResidentStep::done(ResidentStepKind::Ready, bytes))
    }

    pub fn begin_primary_recovery<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
        let Some((stamp, _)) = state.primary_recovery_target::<C>(mode)? else { return Ok(ResidentStep::rejected()); };
        if state.recovery.is_some() { return Ok(ResidentStep::rejected()); }
        let bytes = resident_release_work(&[size_of::<Option<ResidentRecoveryCursor>>(), size_of::<AtomicUsize>()])?;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        let page = state.consumers.as_ref().ok_or(ResidentFault::Identity)?;
        if !page.initialized { return Err(ResidentFault::Identity); }
        let pointer = page.pointer.ok_or(ResidentFault::Identity)?;
        #[cfg(test)]
        tests::observe_primary_recovery_pointer_load(page.registration.get());
        let header = unsafe { pointer.as_ref() };
        let count = header.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
        let registration = page.registration;
        header.recovery_pins.store(count, Ordering::Release);
        state.recovery = Some(ResidentRecoveryCursor { stamp, mode, revoked: false, next: Some(ResidentRecoveryPin { pointer, registration }), found: None });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn advance_primary_recovery<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
        if state.closed { return Err(ResidentFault::Closed); }
        let Some(cursor) = state.recovery.as_ref() else { return Ok(ResidentStep::rejected()); };
        if cursor.revoked { return Err(ResidentFault::Closed); }
        let Some((stamp, original)) = state.primary_recovery_target::<C>(cursor.mode)? else { return Ok(ResidentStep::rejected()); };
        if cursor.stamp != stamp || cursor.found.is_some() { return Ok(ResidentStep::rejected()); }
        let Some(pin) = cursor.next.as_ref() else { return Ok(ResidentStep::rejected()); };
        let matching = pin.registration == stamp.generation;
        if matching && pin.pointer != original { return Err(ResidentFault::Identity); }
        let bytes = if matching {
            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
        } else {
            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
        };
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        #[cfg(test)]
        tests::observe_primary_recovery_pointer_load(pin.registration.get());
        let header = unsafe { pin.pointer.as_ref() };
        if header.registration != pin.registration { return Err(ResidentFault::Identity); }
        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
        if matching {
            if header.type_id != stamp.type_id || (cursor.mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() }) { return Err(ResidentFault::Identity); }
            let cursor = state.recovery.as_mut().unwrap();
            cursor.found = cursor.next.take();
            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
        }
        let successor = unsafe { (&*header.next.get()).as_ref() }.ok_or(ResidentFault::Identity)?;
        let pointer = successor.pointer.ok_or(ResidentFault::Identity)?;
        let registration = successor.registration;
        if !successor.initialized || pointer == pin.pointer || registration == pin.registration { return Err(ResidentFault::Identity); }
        #[cfg(test)]
        tests::observe_primary_recovery_pointer_load(registration.get());
        let next = unsafe { pointer.as_ref() };
        let count = next.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
        next.recovery_pins.store(count, Ordering::Release);
        header.recovery_pins.store(remaining, Ordering::Release);
        state.recovery.as_mut().unwrap().next = Some(ResidentRecoveryPin { pointer, registration });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn capture_primary_consumer<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<(ResidentStep, Option<ResidentConsumer<'_, C>>), ResidentFault> {
        let Some(mut state) = self.access()? else { return Ok((ResidentStep::blocked(), None)); };
        let Some((stamp, original)) = state.primary_recovery_target::<C>(mode)? else { return Ok((ResidentStep::rejected(), None)); };
        let Some(cursor) = state.recovery.as_ref() else { return Ok((ResidentStep::rejected(), None)); };
        if cursor.revoked { return Err(ResidentFault::Closed); }
        if cursor.mode != mode || cursor.stamp != stamp || cursor.next.is_some() { return Ok((ResidentStep::rejected(), None)); }
        let Some(pin) = cursor.found.as_ref() else { return Ok((ResidentStep::rejected(), None)); };
        if pin.registration != stamp.generation || pin.pointer != original { return Err(ResidentFault::Identity); }
        let bytes = resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryCursor>>(), size_of::<ResidentConsumer<'_, C>>()])?;
        if !grant.admits(bytes) { return Ok((ResidentStep::blocked(), None)); }
        #[cfg(test)]
        tests::observe_primary_recovery_pointer_load(pin.registration.get());
        let header = unsafe { pin.pointer.as_ref() };
        if header.registration != stamp.generation || header.type_id != stamp.type_id { return Err(ResidentFault::Identity); }
        if mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() } { return Err(ResidentFault::Closed); }
        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
        let aliases = header.aliases.load(Ordering::Acquire);
        let count = aliases.checked_add(1).ok_or(ResidentFault::Count)?;
        header.aliases.compare_exchange(aliases, count, Ordering::AcqRel, Ordering::Acquire).map_err(|_| ResidentFault::Busy)?;
        header.recovery_pins.store(remaining, Ordering::Release);
        state.recovery = None;
        Ok((ResidentStep::done(ResidentStepKind::Ready, bytes), Some(ResidentConsumer { root: self, pointer: original.cast() })))
    }

    pub fn begin_primary_consumer_close(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
        if state.closed || state.primary.is_none() { return Ok(ResidentStep::rejected()); }
        let revoke = state.recovery.as_ref().is_some_and(|cursor| !cursor.revoked);
        let bytes = resident_release_work(&[if state.closing { 0 } else { size_of::<bool>() }, if revoke { size_of::<bool>() } else { 0 }])?;
        if bytes == 0 { return Ok(ResidentStep { kind: ResidentStepKind::Pending, items: 0, bytes: 0 }); }
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        if !state.closing { state.closing = true; }
        if revoke { state.recovery.as_mut().unwrap().revoked = true; }
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn prepared_consumer<C: Send + 'static>(&self) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
        self.consumer_access(false)
    }

    pub fn recover_consumer_for_close<C: Send + 'static>(&self) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
        self.consumer_access(true)
    }

    fn consumer_access<C: Send + 'static>(&self, closing: bool) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
        let Some(state) = self.access()? else { return Ok(None); };
        let Some(pointer) = state.prepared_consumer else { return Ok(None); };
        let header = unsafe { pointer.as_ref() };
        if header.type_id != TypeId::of::<C>() { return Err(ResidentFault::Identity); }
        if (state.closing || unsafe { *header.closing.get() }) != closing { return Err(ResidentFault::Closed); }
        header.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
        Ok(Some(ResidentConsumer { root: self, pointer: pointer.cast() }))
    }
}

struct ErasedConsumer { pointer: NonNull<ConsumerHeader>, empty: unsafe fn(NonNull<ConsumerHeader>) -> bool }

impl ErasedConsumer {
    fn new<C: Send + 'static>(source: &ResidentConsumer<'_, C>) -> Result<Self, ResidentFault> {
        unsafe { source.pointer.as_ref().header.admissions.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
        Ok(Self { pointer: source.pointer.cast(), empty: empty_consumer::<C> })
    }
    fn matches<C>(&self, source: &ResidentConsumer<'_, C>) -> bool { self.pointer == source.pointer.cast() }
    fn is_empty(&self) -> bool { unsafe { (self.empty)(self.pointer) } }
}

impl Drop for ErasedConsumer {
    fn drop(&mut self) { unsafe { self.pointer.as_ref().admissions.fetch_sub(1, Ordering::AcqRel); } }
}

struct AdmissionNode {
    aliases: AtomicUsize,
    fields: UnsafeCell<AdmissionFields>,
}

struct AdmissionPage { pointer: NonNull<AdmissionNode>, initialized: bool }

unsafe fn destroy_admission(pointer: NonNull<u8>) { unsafe { pointer.cast::<AdmissionNode>().as_ptr().drop_in_place(); } }

struct AdmissionFields {
    next: Option<AdmissionPage>,
    consumer: Option<ErasedConsumer>,
    record: Option<ErasedRecord>,
    partition: ResidentPartition,
    charge: ResidentResources,
    claimed: bool,
}

struct PendingAdmission { page: Option<AdmissionPage>, consumer: Option<ErasedConsumer>, partition: ResidentPartition, charge: ResidentResources }

/// 🪢️ Send-typed nodes stay in this gated root/list/Release; cursor pointers have counted same-root pins and captures acquire an alias before releasing a pin.
unsafe impl Send for LedgerState {}

pub struct ResidentAdmission<'root, C> { root: &'root ResidentLedgerRoot, node: NonNull<AdmissionNode>, marker: PhantomData<fn() -> C> }

impl<'root> ResidentLedger<'root> {
    pub fn prepare_admission<C: Send + 'static>(&self, consumer: &ResidentConsumer<'_, C>, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, consumer.root) { return Ok(ResidentStep::rejected()); }
        let Some(mut state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        if state.closing || state.closed || unsafe { *consumer.pointer.as_ref().header.closing.get() } { return Ok(ResidentStep::rejected()); }
        if state.prepared.is_some() { return Ok(ResidentStep::blocked()); }
        let bytes = size_of::<AdmissionNode>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        if let Some(pending) = state.pending.as_mut() {
            if pending.partition != partition || !pending.consumer.as_ref().is_some_and(|held| held.matches(consumer)) { return Ok(ResidentStep::blocked()); }
            if pending.page.is_none() {
                let allocated = state.allocated_bytes.checked_add(bytes).ok_or(ResidentFault::Count)?;
                let pointer = NonNull::new(unsafe { std::alloc::alloc(Layout::new::<AdmissionNode>()) }.cast::<AdmissionNode>()).ok_or(ResidentFault::Allocation)?;
                state.pending.as_mut().unwrap().page = Some(AdmissionPage { pointer, initialized: false });
                state.allocated_bytes = allocated;
                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
            }
            let mut pending = state.pending.take().unwrap();
            let mut page = pending.page.take().unwrap();
            unsafe { page.pointer.as_ptr().write(AdmissionNode { aliases: AtomicUsize::new(0), fields: UnsafeCell::new(AdmissionFields { next: state.head.take(), consumer: pending.consumer.take(), record: None, partition, charge: pending.charge, claimed: false }) }); }
            page.initialized = true;
            state.prepared = Some(page.pointer);
            state.head = Some(page);
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        let charge = ResidentResources { bytes, slots: 1, owners: 1 };
        if state.reserve(partition, charge).is_err() { return Ok(ResidentStep::blocked()); }
        let held = match ErasedConsumer::new(consumer) { Ok(held) => held, Err(error) => { state.release(partition, charge)?; return Err(error); } };
        state.pending = Some(PendingAdmission { page: None, consumer: Some(held), partition, charge });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn prepared_admission<C: Send + 'static>(&self, consumer: &ResidentConsumer<'_, C>) -> Result<Option<ResidentAdmission<'root, C>>, ResidentFault> {
        self.admission_access(consumer, false)
    }

    pub fn recover_admission_for_close<C: Send + 'static>(&self, consumer: &ResidentConsumer<'_, C>) -> Result<Option<ResidentAdmission<'root, C>>, ResidentFault> {
        self.admission_access(consumer, true)
    }

    fn admission_access<C: Send + 'static>(&self, consumer: &ResidentConsumer<'_, C>, closing: bool) -> Result<Option<ResidentAdmission<'root, C>>, ResidentFault> {
        if !std::ptr::eq(self.root, consumer.root) { return Ok(None); }
        let Some(state) = self.root.access()? else { return Ok(None); };
        if (state.closing || unsafe { *consumer.pointer.as_ref().header.closing.get() }) != closing { return Err(ResidentFault::Closed); }
        let Some(pointer) = state.prepared else { return Ok(None); };
        let node = unsafe { pointer.as_ref() };
        let fields = unsafe { &*node.fields.get() };
        if fields.claimed || !fields.consumer.as_ref().is_some_and(|held| held.matches(consumer)) { return Ok(None); }
        node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
        Ok(Some(ResidentAdmission { root: self.root, node: pointer, marker: PhantomData }))
    }

    pub fn claim_admission<C: Send + 'static>(&self, consumer: &ResidentConsumer<'_, C>, cell: &ResidentAdmission<'_, C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, cell.root) || !std::ptr::eq(self.root, consumer.root) { return Ok(ResidentStep::rejected()); }
        let Some(mut state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        if state.closing || unsafe { *consumer.pointer.as_ref().header.closing.get() } { return Ok(ResidentStep::rejected()); }
        let node = unsafe { &mut *cell.node.as_ref().fields.get() };
        if node.claimed || !node.consumer.as_ref().is_some_and(|held| held.matches(consumer)) { return Ok(ResidentStep::rejected()); }
        if !grant.admits(size_of::<bool>() as u64) { return Ok(ResidentStep::blocked()); }
        node.claimed = true;
        state.prepared = None;
        Ok(ResidentStep::done(ResidentStepKind::Ready, size_of::<bool>() as u64))
    }

    pub fn reserve_record<C: Send + 'static, S: Send + 'static>(&self, cell: &ResidentAdmission<'_, C>, envelope: ResidentResources, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, cell.root) { return Ok(ResidentStep::rejected()); }
        let Some(mut state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        if state.closing { return Ok(ResidentStep::rejected()); }
        let node = unsafe { &mut *cell.node.as_ref().fields.get() };
        if node.consumer.as_ref().is_none_or(|consumer| unsafe { *consumer.pointer.as_ref().closing.get() }) { return Ok(ResidentStep::rejected()); }
        if !node.claimed { return Ok(ResidentStep::rejected()); }
        if let Some(record) = node.record.as_mut() {
            if record.type_id != TypeId::of::<S>() || record.initialized { return Ok(ResidentStep::rejected()); }
            let bytes = size_of::<RecordNode<S>>() as u64;
            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
            unsafe { record.pointer.cast::<RecordNode<S>>().as_ptr().write(RecordNode { source: UnsafeCell::new(None), aliases: AtomicUsize::new(0) }); }
            record.initialized = true;
            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
        }
        let bytes = size_of::<RecordNode<S>>() as u64;
        let work = bytes.max(size_of::<ErasedRecord>() as u64);
        if !grant.admits(work) { return Ok(ResidentStep::blocked()); }
        let charge = envelope.checked_add(ResidentResources { bytes, slots: 1, owners: 1 })?;
        let allocated = state.allocated_bytes.checked_add(bytes).ok_or(ResidentFault::Count)?;
        if state.reserve(node.partition, charge).is_err() { return Ok(ResidentStep::blocked()); }
        let layout = Layout::new::<RecordNode<S>>();
        let Some(pointer) = NonNull::new(unsafe { std::alloc::alloc(layout) }) else { state.release(node.partition, charge)?; return Err(ResidentFault::Allocation); };
        node.record = Some(ErasedRecord { pointer, layout, allocated_bytes: bytes, charge, partition: node.partition, initialized: false, type_id: TypeId::of::<S>(), empty: record_empty::<S>, aliases: record_aliases::<S>, destroy_empty: destroy_record::<S> });
        state.allocated_bytes = allocated;
        Ok(ResidentStep::done(ResidentStepKind::Pending, work))
    }
}

impl<C> Drop for ResidentAdmission<'_, C> {
    fn drop(&mut self) { unsafe { self.node.as_ref().aliases.fetch_sub(1, Ordering::AcqRel); } }
}

impl<'root, C: Send + 'static> ResidentAdmission<'root, C> {
    pub fn handoff_consumer_into(&self, consumer: &ResidentConsumer<'_, C>, target: &mut Option<C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, consumer.root) { return Ok(ResidentStep::rejected()); }
        let Some(_state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        let node = unsafe { &*self.node.as_ref().fields.get() };
        if target.is_some() || !node.consumer.as_ref().is_some_and(|held| held.matches(consumer)) { return Ok(ResidentStep::rejected()); }
        let bytes = size_of::<Option<C>>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        let source = unsafe { &mut *consumer.pointer.as_ref().source.get() };
        if source.is_none() { return Ok(ResidentStep::rejected()); }
        *target = source.take();
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn record<S: Send + 'static>(&self) -> Result<Option<ResidentRecord<'root, S>>, ResidentFault> {
        self.record_access(false)
    }

    pub fn recover_record_for_close<S: Send + 'static>(&self) -> Result<Option<ResidentRecord<'root, S>>, ResidentFault> {
        self.record_access(true)
    }

    fn record_access<S: Send + 'static>(&self, closing: bool) -> Result<Option<ResidentRecord<'root, S>>, ResidentFault> {
        let Some(state) = self.root.access()? else { return Ok(None); };
        let node = unsafe { &*self.node.as_ref().fields.get() };
        let Some(consumer) = node.consumer.as_ref() else { return Ok(None); };
        if (state.closing || unsafe { *consumer.pointer.as_ref().closing.get() }) != closing { return Err(ResidentFault::Closed); }
        let Some(record) = node.record.as_ref() else { return Ok(None); };
        if record.type_id != TypeId::of::<S>() { return Err(ResidentFault::Identity); }
        if !record.initialized { return Ok(None); }
        let pointer = record.pointer.cast::<RecordNode<S>>();
        unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
        Ok(Some(ResidentRecord { root: self.root, pointer, consumer: consumer.pointer }))
    }
}
//#endregion 🪪️ConsumerCell

//#region 📦️TypedRecord
struct RecordNode<S> { source: UnsafeCell<Option<S>>, aliases: AtomicUsize }

struct ErasedRecord {
    pointer: NonNull<u8>,
    layout: Layout,
    allocated_bytes: u64,
    charge: ResidentResources,
    partition: ResidentPartition,
    initialized: bool,
    type_id: TypeId,
    empty: unsafe fn(NonNull<u8>) -> bool,
    aliases: unsafe fn(NonNull<u8>) -> usize,
    destroy_empty: unsafe fn(NonNull<u8>),
}

unsafe impl Send for ErasedRecord {}

impl ErasedRecord {
    fn empty(&self) -> Result<bool, ResidentFault> { Ok(!self.initialized || unsafe { (self.empty)(self.pointer) }) }
    fn aliases(&self) -> usize { if self.initialized { unsafe { (self.aliases)(self.pointer) } } else { 0 } }
}

unsafe fn record_empty<S>(pointer: NonNull<u8>) -> bool { unsafe { (&*pointer.cast::<RecordNode<S>>().as_ref().source.get()).is_none() } }
unsafe fn record_aliases<S>(pointer: NonNull<u8>) -> usize { unsafe { pointer.cast::<RecordNode<S>>().as_ref().aliases.load(Ordering::Acquire) } }
unsafe fn destroy_record<S>(pointer: NonNull<u8>) { unsafe { pointer.cast::<RecordNode<S>>().as_ptr().drop_in_place(); } }

pub struct ResidentRecord<'root, S> { root: &'root ResidentLedgerRoot, pointer: NonNull<RecordNode<S>>, consumer: NonNull<ConsumerHeader> }

impl<S> Drop for ResidentRecord<'_, S> {
    fn drop(&mut self) { unsafe { self.pointer.as_ref().aliases.fetch_sub(1, Ordering::AcqRel); } }
}

impl<S: Send + 'static> ResidentRecord<'_, S> {
    pub fn install(&self, source: &mut Option<S>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        if state.closing || unsafe { *self.consumer.as_ref().closing.get() } { return Ok(ResidentStep::rejected()); }
        let slot = unsafe { &mut *self.pointer.as_ref().source.get() };
        if slot.is_some() || source.is_none() { return Ok(ResidentStep::rejected()); }
        let bytes = size_of::<Option<S>>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        *slot = source.take();
        Ok(ResidentStep::done(ResidentStepKind::Ready, bytes))
    }

    pub fn handoff_into(&self, target: &mut Option<S>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(_state) = self.root.access()? else { return Ok(ResidentStep::blocked()); };
        let slot = unsafe { &mut *self.pointer.as_ref().source.get() };
        if target.is_some() || slot.is_none() { return Ok(ResidentStep::rejected()); }
        let bytes = size_of::<Option<S>>() as u64;
        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
        *target = slot.take();
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }
}
//#endregion 📦️TypedRecord

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
