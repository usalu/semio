//! 🪪️ Consumer cells and admissions: consumer pages, typed consumer handles, ledger admission and consumer-side release.

use std::alloc::Layout;
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::size_of;
use std::num::NonZeroU64;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::{ErasedRecord, LedgerState, RecordNode, ResidentAccessGuard, ResidentFault, ResidentGrant, ResidentLedger, ResidentLedgerRoot, ResidentPartition, ResidentPrimaryAnchor, ResidentPrimaryBacking, ResidentRecord, ResidentRecoveryCursor, ResidentRecoveryMode, ResidentRecoveryPin, ResidentRegistrationStamp, ResidentResources, ResidentStep, ResidentStepKind, destroy_record, record_aliases, record_empty, resident_release_work};

//#region 🪪️ConsumerCell
#[repr(C)]
pub(super) struct ConsumerHeader {
    pub(super) aliases: AtomicUsize,
    pub(super) admissions: AtomicUsize,
    pub(super) recovery_pins: AtomicUsize,
    pub(super) closing: UnsafeCell<bool>,
    pub(super) next: UnsafeCell<Option<ConsumerPage>>,
    type_id: TypeId,
    registration: NonZeroU64,
}

#[repr(C)]
pub(super) struct ConsumerNode<C> {
    header: ConsumerHeader,
    source: UnsafeCell<Option<C>>,
}

pub(super) struct ConsumerPage {
    pub(super) pointer: Option<NonNull<ConsumerHeader>>,
    pub(super) layout: Layout,
    pub(super) partition: ResidentPartition,
    pub(super) charge: ResidentResources,
    pub(super) initialized: bool,
    pub(super) type_id: TypeId,
    pub(super) registration: NonZeroU64,
    initialize: unsafe fn(NonNull<ConsumerHeader>, NonZeroU64),
    pub(super) empty: unsafe fn(NonNull<ConsumerHeader>) -> bool,
    pub(super) destroy_empty: unsafe fn(NonNull<u8>),
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
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let allocated = allocated_bytes.checked_add(self.layout.size() as u64).ok_or(ResidentFault::Count)?;
        let pointer = NonNull::new(unsafe { std::alloc::alloc(self.layout) }.cast::<ConsumerHeader>()).ok_or(ResidentFault::Allocation)?;
        self.pointer = Some(pointer);
        *allocated_bytes = allocated;
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    fn initialize(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let bytes = resident_release_work(&[self.layout.size(), size_of::<bool>()])?;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let pointer = self.pointer.ok_or(ResidentFault::Identity)?;
        unsafe {
            (self.initialize)(pointer, self.registration);
        }
        self.initialized = true;
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }
}

unsafe fn initialize_consumer<C: Send + 'static>(pointer: NonNull<ConsumerHeader>, registration: NonZeroU64) {
    unsafe {
        pointer.cast::<ConsumerNode<C>>().as_ptr().write(ConsumerNode {
            header: ConsumerHeader { aliases: AtomicUsize::new(0), admissions: AtomicUsize::new(0), recovery_pins: AtomicUsize::new(0), closing: UnsafeCell::new(false), next: UnsafeCell::new(None), type_id: TypeId::of::<C>(), registration },
            source: UnsafeCell::new(None),
        });
    }
}

unsafe fn empty_consumer<C>(pointer: NonNull<ConsumerHeader>) -> bool {
    unsafe { (&*pointer.cast::<ConsumerNode<C>>().as_ref().source.get()).is_none() }
}
unsafe fn destroy_consumer<C>(pointer: NonNull<u8>) {
    unsafe {
        pointer.cast::<ConsumerNode<C>>().as_ptr().drop_in_place();
    }
}

pub struct ResidentConsumer<'root, C> {
    root: &'root ResidentLedgerRoot,
    pointer: NonNull<ConsumerNode<C>>,
}
pub struct ResidentConsumerRead<'root, C> {
    _access: ResidentAccessGuard<'root, LedgerState>,
    pointer: NonNull<C>,
}

unsafe impl<C: Send> Send for ResidentConsumer<'_, C> {}
unsafe impl<C: Send> Sync for ResidentConsumer<'_, C> {}

impl<C> Drop for ResidentConsumer<'_, C> {
    fn drop(&mut self) {
        unsafe {
            self.pointer.as_ref().header.aliases.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

impl<C> Deref for ResidentConsumerRead<'_, C> {
    type Target = C;
    fn deref(&self) -> &C {
        unsafe { self.pointer.as_ref() }
    }
}

impl<'root, C: Send + 'static> ResidentConsumer<'root, C> {
    pub fn install(&self, source: &mut Option<C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(state) = self.root.access()? else {
            return Ok(ResidentStep::blocked());
        };
        let node = unsafe { self.pointer.as_ref() };
        if state.closing || unsafe { *node.header.closing.get() } {
            return Ok(ResidentStep::rejected());
        }
        let target = unsafe { &mut *node.source.get() };
        if source.is_none() || target.is_some() {
            return Ok(ResidentStep::rejected());
        }
        let bytes = size_of::<Option<C>>() as u64;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        *target = source.take();
        Ok(ResidentStep::done(ResidentStepKind::Ready, bytes))
    }

    pub fn read(&self) -> Result<Option<ResidentConsumerRead<'root, C>>, ResidentFault> {
        let Some(access) = self.root.access()? else {
            return Err(ResidentFault::Busy);
        };
        if access.closing || unsafe { *self.pointer.as_ref().header.closing.get() } {
            return Err(ResidentFault::Closed);
        }
        self.read_with_access(access)
    }

    pub fn read_for_close(&self) -> Result<Option<ResidentConsumerRead<'root, C>>, ResidentFault> {
        let Some(access) = self.root.access()? else {
            return Err(ResidentFault::Busy);
        };
        if !access.closing && !unsafe { *self.pointer.as_ref().header.closing.get() } {
            return Err(ResidentFault::Identity);
        }
        self.read_with_access(access)
    }

    fn read_with_access(&self, access: ResidentAccessGuard<'root, LedgerState>) -> Result<Option<ResidentConsumerRead<'root, C>>, ResidentFault> {
        let source = unsafe { &*self.pointer.as_ref().source.get() };
        let Some(value) = source.as_ref() else {
            return Ok(None);
        };
        Ok(Some(ResidentConsumerRead { _access: access, pointer: NonNull::from(value) }))
    }

    pub fn begin_close(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(_state) = self.root.access()? else {
            return Ok(ResidentStep::blocked());
        };
        if !grant.admits(size_of::<bool>() as u64) {
            return Ok(ResidentStep::blocked());
        }
        unsafe {
            *self.pointer.as_ref().header.closing.get() = true;
        }
        Ok(ResidentStep::done(ResidentStepKind::Pending, size_of::<bool>() as u64))
    }

    pub fn handoff_for_close_into(&self, target: &mut Option<C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(state) = self.root.access()? else {
            return Ok(ResidentStep::blocked());
        };
        let node = unsafe { self.pointer.as_ref() };
        if !state.closing && !unsafe { *node.header.closing.get() } {
            return Ok(ResidentStep::rejected());
        }
        let source = unsafe { &mut *node.source.get() };
        if target.is_some() || source.is_none() {
            return Ok(ResidentStep::rejected());
        }
        let bytes = size_of::<Option<C>>() as u64;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        *target = source.take();
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }
}

impl ResidentLedgerRoot {
    pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else {
            return Ok(ResidentStep::blocked());
        };
        let state = &mut *state;
        if state.closing || state.closed {
            return Ok(ResidentStep::rejected());
        }
        if let Some(page) = state.pending_consumer.as_mut() {
            if page.type_id != TypeId::of::<C>() || page.partition != partition {
                return Ok(ResidentStep::rejected());
            }
            if page.pointer.is_none() {
                return page.allocate(&mut state.allocated_bytes, grant);
            }
            if !page.initialized {
                return page.initialize(grant);
            }
            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
            if !grant.admits(bytes) {
                return Ok(ResidentStep::blocked());
            }
            let page = state.pending_consumer.take().unwrap();
            let pointer = page.pointer.unwrap();
            unsafe {
                *pointer.as_ref().next.get() = state.consumers.take();
            }
            state.prepared_consumer = Some(pointer);
            state.consumers = Some(page);
            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
        }
        let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<u64>(), size_of::<ResidentResources>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
        let page = ConsumerPage::reserved::<C>(partition, registration)?;
        if state.reserve(partition, page.charge).is_err() {
            return Ok(ResidentStep::blocked());
        }
        state.last_consumer_registration = registration.get();
        state.pending_consumer = Some(page);
        state.prepared_consumer = None;
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn reserve_primary_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else {
            return Ok(ResidentStep::blocked());
        };
        if state.closing || state.closed || state.primary.is_some() {
            return Ok(ResidentStep::rejected());
        }
        let bytes = resident_release_work(&[size_of::<Option<ResidentPrimaryAnchor>>(), size_of::<u64>(), size_of::<ResidentResources>()])?;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
        let page = ConsumerPage::reserved::<C>(partition, registration)?;
        if state.reserve(partition, page.charge).is_err() {
            return Ok(ResidentStep::blocked());
        }
        state.last_consumer_registration = registration.get();
        state.primary = Some(ResidentPrimaryAnchor { stamp: ResidentRegistrationStamp { generation: registration, type_id: TypeId::of::<C>() }, partition, backing: ResidentPrimaryBacking::Pending(page) });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn prepare_primary_consumer<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else {
            return Ok(ResidentStep::blocked());
        };
        let state = &mut *state;
        if state.closing || state.closed {
            return Ok(ResidentStep::rejected());
        }
        let Some(anchor) = state.primary.as_mut() else {
            return Ok(ResidentStep::rejected());
        };
        if anchor.stamp.type_id != TypeId::of::<C>() {
            return Err(ResidentFault::Identity);
        }
        let ResidentPrimaryBacking::Pending(page) = &mut anchor.backing else {
            return Ok(ResidentStep::rejected());
        };
        if page.registration != anchor.stamp.generation || page.type_id != anchor.stamp.type_id || page.partition != anchor.partition {
            return Err(ResidentFault::Identity);
        }
        if page.pointer.is_none() {
            return page.allocate(&mut state.allocated_bytes, grant);
        }
        if !page.initialized {
            return page.initialize(grant);
        }
        let bytes = resident_release_work(&[size_of::<ResidentPrimaryBacking>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>()])?;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let pointer = page.pointer.unwrap();
        let ResidentPrimaryBacking::Pending(page) = std::mem::replace(&mut anchor.backing, ResidentPrimaryBacking::Published(pointer)) else { unreachable!() };
        unsafe {
            *pointer.as_ref().next.get() = state.consumers.take();
        }
        state.consumers = Some(page);
        Ok(ResidentStep::done(ResidentStepKind::Ready, bytes))
    }

    pub fn begin_primary_recovery<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else {
            return Ok(ResidentStep::blocked());
        };
        let Some((stamp, _)) = state.primary_recovery_target::<C>(mode)? else {
            return Ok(ResidentStep::rejected());
        };
        if state.recovery.is_some() {
            return Ok(ResidentStep::rejected());
        }
        let bytes = resident_release_work(&[size_of::<Option<ResidentRecoveryCursor>>(), size_of::<AtomicUsize>()])?;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let page = state.consumers.as_ref().ok_or(ResidentFault::Identity)?;
        if !page.initialized {
            return Err(ResidentFault::Identity);
        }
        let pointer = page.pointer.ok_or(ResidentFault::Identity)?;
        let header = unsafe { pointer.as_ref() };
        let count = header.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
        let registration = page.registration;
        header.recovery_pins.store(count, Ordering::Release);
        state.recovery = Some(ResidentRecoveryCursor { stamp, mode, revoked: false, next: Some(ResidentRecoveryPin { pointer, registration }), found: None });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn advance_primary_recovery<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else {
            return Ok(ResidentStep::blocked());
        };
        if state.closed {
            return Err(ResidentFault::Closed);
        }
        let Some(cursor) = state.recovery.as_ref() else {
            return Ok(ResidentStep::rejected());
        };
        if cursor.revoked {
            return Err(ResidentFault::Closed);
        }
        let Some((stamp, original)) = state.primary_recovery_target::<C>(cursor.mode)? else {
            return Ok(ResidentStep::rejected());
        };
        if cursor.stamp != stamp || cursor.found.is_some() {
            return Ok(ResidentStep::rejected());
        }
        let Some(pin) = cursor.next.as_ref() else {
            return Ok(ResidentStep::rejected());
        };
        let matching = pin.registration == stamp.generation;
        if matching && pin.pointer != original {
            return Err(ResidentFault::Identity);
        }
        let bytes = if matching {
            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
        } else {
            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
        };
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let header = unsafe { pin.pointer.as_ref() };
        if header.registration != pin.registration {
            return Err(ResidentFault::Identity);
        }
        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
        if matching {
            if header.type_id != stamp.type_id || (cursor.mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() }) {
                return Err(ResidentFault::Identity);
            }
            let cursor = state.recovery.as_mut().unwrap();
            cursor.found = cursor.next.take();
            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
        }
        let successor = unsafe { (&*header.next.get()).as_ref() }.ok_or(ResidentFault::Identity)?;
        let pointer = successor.pointer.ok_or(ResidentFault::Identity)?;
        let registration = successor.registration;
        if !successor.initialized || pointer == pin.pointer || registration == pin.registration {
            return Err(ResidentFault::Identity);
        }
        let next = unsafe { pointer.as_ref() };
        let count = next.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
        next.recovery_pins.store(count, Ordering::Release);
        header.recovery_pins.store(remaining, Ordering::Release);
        state.recovery.as_mut().unwrap().next = Some(ResidentRecoveryPin { pointer, registration });
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn capture_primary_consumer<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<(ResidentStep, Option<ResidentConsumer<'_, C>>), ResidentFault> {
        let Some(mut state) = self.access()? else {
            return Ok((ResidentStep::blocked(), None));
        };
        let Some((stamp, original)) = state.primary_recovery_target::<C>(mode)? else {
            return Ok((ResidentStep::rejected(), None));
        };
        let Some(cursor) = state.recovery.as_ref() else {
            return Ok((ResidentStep::rejected(), None));
        };
        if cursor.revoked {
            return Err(ResidentFault::Closed);
        }
        if cursor.mode != mode || cursor.stamp != stamp || cursor.next.is_some() {
            return Ok((ResidentStep::rejected(), None));
        }
        let Some(pin) = cursor.found.as_ref() else {
            return Ok((ResidentStep::rejected(), None));
        };
        if pin.registration != stamp.generation || pin.pointer != original {
            return Err(ResidentFault::Identity);
        }
        let bytes = resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryCursor>>(), size_of::<ResidentConsumer<'_, C>>()])?;
        if !grant.admits(bytes) {
            return Ok((ResidentStep::blocked(), None));
        }
        let header = unsafe { pin.pointer.as_ref() };
        if header.registration != stamp.generation || header.type_id != stamp.type_id {
            return Err(ResidentFault::Identity);
        }
        if mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() } {
            return Err(ResidentFault::Closed);
        }
        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
        let aliases = header.aliases.load(Ordering::Acquire);
        let count = aliases.checked_add(1).ok_or(ResidentFault::Count)?;
        header.aliases.compare_exchange(aliases, count, Ordering::AcqRel, Ordering::Acquire).map_err(|_| ResidentFault::Busy)?;
        header.recovery_pins.store(remaining, Ordering::Release);
        state.recovery = None;
        Ok((ResidentStep::done(ResidentStepKind::Ready, bytes), Some(ResidentConsumer { root: self, pointer: original.cast() })))
    }

    pub fn begin_primary_consumer_close(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        let Some(mut state) = self.access()? else {
            return Ok(ResidentStep::blocked());
        };
        if state.closed || state.primary.is_none() {
            return Ok(ResidentStep::rejected());
        }
        let revoke = state.recovery.as_ref().is_some_and(|cursor| !cursor.revoked);
        let bytes = resident_release_work(&[if state.closing { 0 } else { size_of::<bool>() }, if revoke { size_of::<bool>() } else { 0 }])?;
        if bytes == 0 {
            return Ok(ResidentStep { kind: ResidentStepKind::Pending, items: 0, bytes: 0 });
        }
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        if !state.closing {
            state.closing = true;
        }
        if revoke {
            state.recovery.as_mut().unwrap().revoked = true;
        }
        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
    }

    pub fn prepared_consumer<C: Send + 'static>(&self) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
        self.consumer_access(false)
    }

    pub fn recover_consumer_for_close<C: Send + 'static>(&self) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
        self.consumer_access(true)
    }

    fn consumer_access<C: Send + 'static>(&self, closing: bool) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
        let Some(state) = self.access()? else {
            return Ok(None);
        };
        let Some(pointer) = state.prepared_consumer else {
            return Ok(None);
        };
        let header = unsafe { pointer.as_ref() };
        if header.type_id != TypeId::of::<C>() {
            return Err(ResidentFault::Identity);
        }
        if (state.closing || unsafe { *header.closing.get() }) != closing {
            return Err(ResidentFault::Closed);
        }
        header.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
        Ok(Some(ResidentConsumer { root: self, pointer: pointer.cast() }))
    }
}

pub(super) struct ErasedConsumer {
    pointer: NonNull<ConsumerHeader>,
    empty: unsafe fn(NonNull<ConsumerHeader>) -> bool,
}

impl ErasedConsumer {
    fn new<C: Send + 'static>(source: &ResidentConsumer<'_, C>) -> Result<Self, ResidentFault> {
        unsafe {
            source.pointer.as_ref().header.admissions.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
        }
        Ok(Self { pointer: source.pointer.cast(), empty: empty_consumer::<C> })
    }
    fn matches<C>(&self, source: &ResidentConsumer<'_, C>) -> bool {
        self.pointer == source.pointer.cast()
    }
    pub(super) fn is_empty(&self) -> bool {
        unsafe { (self.empty)(self.pointer) }
    }
}

impl Drop for ErasedConsumer {
    fn drop(&mut self) {
        unsafe {
            self.pointer.as_ref().admissions.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

pub(super) struct AdmissionNode {
    pub(super) aliases: AtomicUsize,
    pub(super) fields: UnsafeCell<AdmissionFields>,
}

pub(super) struct AdmissionPage {
    pub(super) pointer: NonNull<AdmissionNode>,
    pub(super) initialized: bool,
}

pub(super) unsafe fn destroy_admission(pointer: NonNull<u8>) {
    unsafe {
        pointer.cast::<AdmissionNode>().as_ptr().drop_in_place();
    }
}

pub(super) struct AdmissionFields {
    pub(super) next: Option<AdmissionPage>,
    pub(super) consumer: Option<ErasedConsumer>,
    pub(super) record: Option<ErasedRecord>,
    pub(super) partition: ResidentPartition,
    pub(super) charge: ResidentResources,
    claimed: bool,
}

pub(super) struct PendingAdmission {
    pub(super) page: Option<AdmissionPage>,
    pub(super) consumer: Option<ErasedConsumer>,
    pub(super) partition: ResidentPartition,
    pub(super) charge: ResidentResources,
}

/// 🪢️ Send-typed nodes stay in this gated root/list/Release; cursor pointers have counted same-root pins and captures acquire an alias before releasing a pin.
unsafe impl Send for LedgerState {}

pub struct ResidentAdmission<'root, C> {
    root: &'root ResidentLedgerRoot,
    node: NonNull<AdmissionNode>,
    marker: PhantomData<fn() -> C>,
}

impl<'root> ResidentLedger<'root> {
    pub fn prepare_admission<C: Send + 'static>(&self, consumer: &ResidentConsumer<'_, C>, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, consumer.root) {
            return Ok(ResidentStep::rejected());
        }
        let Some(mut state) = self.root.access()? else {
            return Ok(ResidentStep::blocked());
        };
        if state.closing || state.closed || unsafe { *consumer.pointer.as_ref().header.closing.get() } {
            return Ok(ResidentStep::rejected());
        }
        if state.prepared.is_some() {
            return Ok(ResidentStep::blocked());
        }
        let bytes = size_of::<AdmissionNode>() as u64;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        if let Some(pending) = state.pending.as_mut() {
            if pending.partition != partition || !pending.consumer.as_ref().is_some_and(|held| held.matches(consumer)) {
                return Ok(ResidentStep::blocked());
            }
            if pending.page.is_none() {
                let allocated = state.allocated_bytes.checked_add(bytes).ok_or(ResidentFault::Count)?;
                let pointer = NonNull::new(unsafe { std::alloc::alloc(Layout::new::<AdmissionNode>()) }.cast::<AdmissionNode>()).ok_or(ResidentFault::Allocation)?;
                state.pending.as_mut().unwrap().page = Some(AdmissionPage { pointer, initialized: false });
                state.allocated_bytes = allocated;
                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
            }
            let mut pending = state.pending.take().unwrap();
            let mut page = pending.page.take().unwrap();
            unsafe {
                page.pointer
                    .as_ptr()
                    .write(AdmissionNode { aliases: AtomicUsize::new(0), fields: UnsafeCell::new(AdmissionFields { next: state.head.take(), consumer: pending.consumer.take(), record: None, partition, charge: pending.charge, claimed: false }) });
            }
            page.initialized = true;
            state.prepared = Some(page.pointer);
            state.head = Some(page);
            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
        }
        let charge = ResidentResources { bytes, slots: 1, owners: 1 };
        if state.reserve(partition, charge).is_err() {
            return Ok(ResidentStep::blocked());
        }
        let held = match ErasedConsumer::new(consumer) {
            Ok(held) => held,
            Err(error) => {
                state.release(partition, charge)?;
                return Err(error);
            }
        };
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
        if !std::ptr::eq(self.root, consumer.root) {
            return Ok(None);
        }
        let Some(state) = self.root.access()? else {
            return Ok(None);
        };
        if (state.closing || unsafe { *consumer.pointer.as_ref().header.closing.get() }) != closing {
            return Err(ResidentFault::Closed);
        }
        let Some(pointer) = state.prepared else {
            return Ok(None);
        };
        let node = unsafe { pointer.as_ref() };
        let fields = unsafe { &*node.fields.get() };
        if fields.claimed || !fields.consumer.as_ref().is_some_and(|held| held.matches(consumer)) {
            return Ok(None);
        }
        node.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
        Ok(Some(ResidentAdmission { root: self.root, node: pointer, marker: PhantomData }))
    }

    pub fn claim_admission<C: Send + 'static>(&self, consumer: &ResidentConsumer<'_, C>, cell: &ResidentAdmission<'_, C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, cell.root) || !std::ptr::eq(self.root, consumer.root) {
            return Ok(ResidentStep::rejected());
        }
        let Some(mut state) = self.root.access()? else {
            return Ok(ResidentStep::blocked());
        };
        if state.closing || unsafe { *consumer.pointer.as_ref().header.closing.get() } {
            return Ok(ResidentStep::rejected());
        }
        let node = unsafe { &mut *cell.node.as_ref().fields.get() };
        if node.claimed || !node.consumer.as_ref().is_some_and(|held| held.matches(consumer)) {
            return Ok(ResidentStep::rejected());
        }
        if !grant.admits(size_of::<bool>() as u64) {
            return Ok(ResidentStep::blocked());
        }
        node.claimed = true;
        state.prepared = None;
        Ok(ResidentStep::done(ResidentStepKind::Ready, size_of::<bool>() as u64))
    }

    pub fn reserve_record<C: Send + 'static, S: Send + 'static>(&self, cell: &ResidentAdmission<'_, C>, envelope: ResidentResources, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, cell.root) {
            return Ok(ResidentStep::rejected());
        }
        let Some(mut state) = self.root.access()? else {
            return Ok(ResidentStep::blocked());
        };
        if state.closing {
            return Ok(ResidentStep::rejected());
        }
        let node = unsafe { &mut *cell.node.as_ref().fields.get() };
        if node.consumer.as_ref().is_none_or(|consumer| unsafe { *consumer.pointer.as_ref().closing.get() }) {
            return Ok(ResidentStep::rejected());
        }
        if !node.claimed {
            return Ok(ResidentStep::rejected());
        }
        if let Some(record) = node.record.as_mut() {
            if record.type_id != TypeId::of::<S>() || record.initialized {
                return Ok(ResidentStep::rejected());
            }
            let bytes = size_of::<RecordNode<S>>() as u64;
            if !grant.admits(bytes) {
                return Ok(ResidentStep::blocked());
            }
            unsafe {
                record.pointer.cast::<RecordNode<S>>().as_ptr().write(RecordNode { source: UnsafeCell::new(None), aliases: AtomicUsize::new(0) });
            }
            record.initialized = true;
            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
        }
        let bytes = size_of::<RecordNode<S>>() as u64;
        let work = bytes.max(size_of::<ErasedRecord>() as u64);
        if !grant.admits(work) {
            return Ok(ResidentStep::blocked());
        }
        let charge = envelope.checked_add(ResidentResources { bytes, slots: 1, owners: 1 })?;
        let allocated = state.allocated_bytes.checked_add(bytes).ok_or(ResidentFault::Count)?;
        if state.reserve(node.partition, charge).is_err() {
            return Ok(ResidentStep::blocked());
        }
        let layout = Layout::new::<RecordNode<S>>();
        let Some(pointer) = NonNull::new(unsafe { std::alloc::alloc(layout) }) else {
            state.release(node.partition, charge)?;
            return Err(ResidentFault::Allocation);
        };
        node.record =
            Some(ErasedRecord { pointer, layout, allocated_bytes: bytes, charge, partition: node.partition, initialized: false, type_id: TypeId::of::<S>(), empty: record_empty::<S>, aliases: record_aliases::<S>, destroy_empty: destroy_record::<S> });
        state.allocated_bytes = allocated;
        Ok(ResidentStep::done(ResidentStepKind::Pending, work))
    }
}

impl<C> Drop for ResidentAdmission<'_, C> {
    fn drop(&mut self) {
        unsafe {
            self.node.as_ref().aliases.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

impl<'root, C: Send + 'static> ResidentAdmission<'root, C> {
    pub fn handoff_consumer_into(&self, consumer: &ResidentConsumer<'_, C>, target: &mut Option<C>, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
        if !std::ptr::eq(self.root, consumer.root) {
            return Ok(ResidentStep::rejected());
        }
        let Some(_state) = self.root.access()? else {
            return Ok(ResidentStep::blocked());
        };
        let node = unsafe { &*self.node.as_ref().fields.get() };
        if target.is_some() || !node.consumer.as_ref().is_some_and(|held| held.matches(consumer)) {
            return Ok(ResidentStep::rejected());
        }
        let bytes = size_of::<Option<C>>() as u64;
        if !grant.admits(bytes) {
            return Ok(ResidentStep::blocked());
        }
        let source = unsafe { &mut *consumer.pointer.as_ref().source.get() };
        if source.is_none() {
            return Ok(ResidentStep::rejected());
        }
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
        let Some(state) = self.root.access()? else {
            return Ok(None);
        };
        let node = unsafe { &*self.node.as_ref().fields.get() };
        let Some(consumer) = node.consumer.as_ref() else {
            return Ok(None);
        };
        if (state.closing || unsafe { *consumer.pointer.as_ref().closing.get() }) != closing {
            return Err(ResidentFault::Closed);
        }
        let Some(record) = node.record.as_ref() else {
            return Ok(None);
        };
        if record.type_id != TypeId::of::<S>() {
            return Err(ResidentFault::Identity);
        }
        if !record.initialized {
            return Ok(None);
        }
        let pointer = record.pointer.cast::<RecordNode<S>>();
        unsafe {
            pointer.as_ref().aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
        }
        Ok(Some(ResidentRecord { root: self.root, pointer, consumer: consumer.pointer }))
    }
}
//#endregion 🪪️ConsumerCell
