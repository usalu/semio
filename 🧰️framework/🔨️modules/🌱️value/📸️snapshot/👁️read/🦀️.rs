//! 👁️ Original generation leases, immutable reads and bounded return/retirement custody.
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, SnapshotRetirementFactory, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use std::{marker::PhantomData, mem::{size_of, size_of_val}, sync::Arc};
#[path = "../../../../🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/🦀️.rs"]
pub mod snapshot_read_registry_handle;
pub use snapshot_read_registry_handle::SnapshotReadRegistryHandle;
#[path = "../../../../🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🔒️registry/♻️retirement/🦀️.rs"]
pub mod snapshot_read_registry_retirement;
pub use snapshot_read_registry_retirement::{SnapshotReadRegistryAliasRetirement, snapshot_registry_alias_demands, snapshot_registry_alias_work_demand, snapshot_registry_alias_close_step};
pub const SNAPSHOT_READ_LEASE_CAPACITY: usize = 1_024;
const ORIGINAL_FREE_SLOTS:[u16;SNAPSHOT_READ_LEASE_CAPACITY]={let mut slots=[0;SNAPSHOT_READ_LEASE_CAPACITY];let mut index=0;while index<SNAPSHOT_READ_LEASE_CAPACITY{slots[index]=index as u16;index+=1}slots};
/// 📏️ The largest physical release one snapshot registry close step performs: the registry frame or one backing page.
pub const SNAPSHOT_READ_RELEASE_PAGE_BYTES: usize = 4096;
/// 📄 Lease slots per backing page; one page of slots fits one release page on every target.
pub const SNAPSHOT_READ_SLOT_PAGE: usize = 128;
/// 📖️ Generation initialization leaves room for its pointer publication within the same four KiB copy grant.
pub const SNAPSHOT_READ_GENERATION_PAGE: usize = SNAPSHOT_READ_RELEASE_PAGE_BYTES / (2*size_of::<std::sync::atomic::AtomicU64>());

/// ⏱️ One bounded cursor action costs one work unit when it moved any currency; refusals and starved turns cost none.
pub fn bounded_outcome(result: Result<RetainedCloneStep, ValueError>) -> RetainedWorkOutcome {
    let work = result.as_ref().map_or(0, |step| u128::from(step.progress() != RetainedCloneProgress::default()));
    RetainedWorkOutcome { result, work }
}

/// 💰️ Records one pure bounded action on the caller's effect exactly once and adds the work it performed even when it refuses.
pub fn pay_pure_work(effect: &mut RetainedAllocationEffect<'_>, work: &mut u128) -> Result<(), ValueError> {
    match semio_framework_value::retirement::record_retirement_work(effect, 1) {
        Ok(paid) => { *work += paid; Ok(()) }
        Err((error, paid)) => { *work += paid; Err(error) }
    }
}

/// 🧾️ Projects the first original cause of a retirement failure into one value refusal, keeping its physical receipt.
pub fn failure_value_error(failure: OriginalRetirementFailure) -> ValueError {
    let progress = failure.retained_progress();
    let (primary, work, receipt, allocation, child, _) = failure.into_parts();
    primary.or(work).or(receipt).or_else(|| child.map(|child| failure_value_error(*child))).unwrap_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, if allocation.is_some() { "original retirement allocation was refused" } else { "original retirement failed without a cause" })).with_retained_progress(progress)
}

/// 🧾️ Folds one finished close poll into a plain value step for callers whose effect already recorded the work; a pending poll made no physical progress.
pub fn settle_close_poll(poll: OriginalRetirementPoll) -> Result<RetainedCloneStep, ValueError> {
    match poll {
        RetainedWorkPoll::Pending { .. } => Ok(RetainedCloneStep::Progress(Default::default())),
        RetainedWorkPoll::Complete(turn) => turn.result.map_err(failure_value_error),
    }
}

/// ⏳️ Settles a child poll that cannot suspend inside a locked registry action; a pending child is a typed refusal that keeps its work.
pub fn settle_inline_poll<T, E: From<ValueError>>(poll: RetainedWorkPoll<T, E>, what: &'static str) -> RetainedWorkOutcome<T, E> {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn,
        RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, what).into()), work },
    }
}

/// 🪜️ Maps one original work outcome onto the cursor step that carries its exact work.
pub fn outcome_retirement_step(turn: RetainedWorkOutcome) -> semio_framework_value::retirement::RetirementStep {
    use semio_framework_value::retirement::RetirementStep;
    match turn.result {
        Err(error) => RetirementStep::Failure(error.into(), turn.work),
        Ok(RetainedCloneStep::Complete(progress)) if progress == RetainedCloneProgress::default() => RetirementStep::Complete(turn.work),
        Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => RetirementStep::Progress(progress, turn.work),
    }
}

/// 🪜️ Carries a rich original retirement turn into the cursor frontier.
pub fn original_outcome_retirement_step(turn:semio_framework_value::OriginalRetirementOutcome)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep;match turn.result{Err(error)=>RetirementStep::Failure(error,turn.work),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>RetirementStep::Complete(turn.work),Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress,turn.work)}}

pub struct SnapshotReadLeaseSlot {
    pub(crate) generation: u64,
    pub(crate) owner: Arc<dyn std::any::Any + Send + Sync>,
}

type SnapshotReadSlotPage = [std::mem::MaybeUninit<SnapshotReadLeaseSlot>; SNAPSHOT_READ_SLOT_PAGE];
type SnapshotReadGenerationPage = [std::sync::atomic::AtomicU64; SNAPSHOT_READ_GENERATION_PAGE];
const _: () = assert!(size_of::<SnapshotReadSlotPage>() <= SNAPSHOT_READ_RELEASE_PAGE_BYTES && size_of::<SnapshotReadGenerationPage>() <= SNAPSHOT_READ_RELEASE_PAGE_BYTES);
const _: () = assert!(SNAPSHOT_READ_LEASE_CAPACITY % SNAPSHOT_READ_SLOT_PAGE == 0 && SNAPSHOT_READ_LEASE_CAPACITY % SNAPSHOT_READ_GENERATION_PAGE == 0);

/// 📃️ The lease slot table in fixed release pages. Issuing requires every page; closing releases one page per step.
pub struct SnapshotReadSlotPages {
    pages: [Option<Box<SnapshotReadSlotPage>>; SNAPSHOT_READ_LEASE_CAPACITY / SNAPSHOT_READ_SLOT_PAGE],
}

impl SnapshotReadSlotPages {
    fn new() -> Self { Self { pages: std::array::from_fn(|_| Some(Box::new([const { std::mem::MaybeUninit::uninit() }; SNAPSHOT_READ_SLOT_PAGE]))) } }
    /// 🧱️ True while every slot page is resident, the only state in which a lease may be issued.
    pub fn resident(&self) -> bool { self.pages.iter().all(Option::is_some) }
    /// 🪪️ Identity of the first resident slot page, for physical-custody witnesses.
    pub fn backing_identity(&self) -> Option<*const ()> { self.pages.iter().flatten().next().map(|page| page.as_ptr() as *const ()) }
    /// ⏭️ Bytes the next page release frees, or zero once every page is released.
    fn next_release_bytes(&self) -> usize { if self.pages.iter().any(Option::is_some) { size_of::<SnapshotReadSlotPage>() } else { 0 } }
    fn release_page(&mut self) -> usize { self.pages.iter_mut().rev().find(|page| page.is_some()).and_then(Option::take).map_or(0, |page| { drop(page); size_of::<SnapshotReadSlotPage>() }) }
}

impl std::ops::Index<usize> for SnapshotReadSlotPages {
    type Output = std::mem::MaybeUninit<SnapshotReadLeaseSlot>;
    fn index(&self, index: usize) -> &Self::Output { &self.pages[index / SNAPSHOT_READ_SLOT_PAGE].as_ref().expect("resident lease slot page")[index % SNAPSHOT_READ_SLOT_PAGE] }
}

impl std::ops::IndexMut<usize> for SnapshotReadSlotPages {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.pages[index / SNAPSHOT_READ_SLOT_PAGE].as_mut().expect("resident lease slot page")[index % SNAPSHOT_READ_SLOT_PAGE] }
}

/// 📸️ The lock-free lease generation words in fixed release pages, released only once the registry is exclusively owned.
pub struct SnapshotReadGenerationPages {
    pages: [Option<Box<SnapshotReadGenerationPage>>; SNAPSHOT_READ_LEASE_CAPACITY / SNAPSHOT_READ_GENERATION_PAGE],
}

impl SnapshotReadGenerationPages {
    fn new() -> Self { Self { pages: std::array::from_fn(|_| Some(Box::new(std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0))))) } }
    fn next_release_bytes(&self) -> usize { if self.pages.iter().any(Option::is_some) { size_of::<SnapshotReadGenerationPage>() } else { 0 } }
    fn release_page(&mut self) -> usize { self.pages.iter_mut().rev().find(|page| page.is_some()).and_then(Option::take).map_or(0, |page| { drop(page); size_of::<SnapshotReadGenerationPage>() }) }
}

impl std::ops::Index<usize> for SnapshotReadGenerationPages {
    type Output = std::sync::atomic::AtomicU64;
    fn index(&self, index: usize) -> &Self::Output { &self.pages[index / SNAPSHOT_READ_GENERATION_PAGE].as_ref().expect("resident lease generation page")[index % SNAPSHOT_READ_GENERATION_PAGE] }
}

pub struct SnapshotReadLeaseRegistryState {
    pub(crate) slots: SnapshotReadSlotPages,
    pub(crate) occupied: [u64; SNAPSHOT_READ_LEASE_CAPACITY / 64],
    pub(crate) free: [u16; SNAPSHOT_READ_LEASE_CAPACITY],
    pub(crate) free_read: usize,
    pub(crate) free_len: usize,
    pub(crate) next_generation: u64,
    pub(crate) cleanup_cursor: usize,
}

/// 🧾️ A returned-root admission fails either at the lease registry or with its complete original retirement failure.
#[derive(Debug)]
pub enum SnapshotReadReturnedRefusal {
    Lease(SnapshotReadLeaseRefusal),
    Retirement(OriginalRetirementFailure),
}

impl From<SnapshotReadLeaseRefusal> for SnapshotReadReturnedRefusal {
    fn from(refusal: SnapshotReadLeaseRefusal) -> Self { Self::Lease(refusal) }
}

impl SnapshotReadReturnedRefusal {
    /// 🧳️ Keeps every original cause; lease refusals become a primary failure.
    pub fn into_failure(self) -> OriginalRetirementFailure { match self { Self::Lease(refusal) => refusal.into_value_error().into(), Self::Retirement(failure) => failure } }
}

#[derive(Debug)]
pub enum SnapshotReadLeaseRefusal {
    Busy,
    Poisoned,
    OwnerTypeChanged,
    Admission(ValueError),
}

impl SnapshotReadLeaseRefusal {
    pub fn into_value_error(self) -> ValueError {
        match self {
            Self::Admission(error) => error,
            Self::Busy => ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "snapshot read lease registry is busy"),
            Self::Poisoned => ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease registry is poisoned"),
            Self::OwnerTypeChanged => ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease owner type changed inside its exact registry"),
        }
    }
}

pub struct SnapshotReadLeaseStateLock {
    held: std::sync::atomic::AtomicBool,
    poisoned: std::sync::atomic::AtomicBool,
    state: std::cell::UnsafeCell<SnapshotReadLeaseRegistryState>,
}

/// 🔄️ Acquire/Release guards exclusively borrow the Send registry state and never cross threads.
unsafe impl Sync for SnapshotReadLeaseStateLock where SnapshotReadLeaseRegistryState: Send {}

impl SnapshotReadLeaseStateLock {
    fn new(state: SnapshotReadLeaseRegistryState) -> Self { Self { held: std::sync::atomic::AtomicBool::new(false), poisoned: std::sync::atomic::AtomicBool::new(false), state: std::cell::UnsafeCell::new(state) } }
    pub fn try_lock(&self) -> std::sync::TryLockResult<SnapshotReadLeaseStateGuard<'_>> {
        if self.held.compare_exchange(false, true, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed).is_err() { return Err(std::sync::TryLockError::WouldBlock); }
        let guard = SnapshotReadLeaseStateGuard { lock: self, panicking_on_entry: std::thread::panicking(), not_send: PhantomData };
        if self.poisoned.load(std::sync::atomic::Ordering::Relaxed) { Err(std::sync::TryLockError::Poisoned(std::sync::PoisonError::new(guard))) } else { Ok(guard) }
    }
    fn get_mut(&mut self) -> std::sync::LockResult<&mut SnapshotReadLeaseRegistryState> {
        let state = self.state.get_mut();
        if *self.poisoned.get_mut() { Err(std::sync::PoisonError::new(state)) } else { Ok(state) }
    }
}

pub struct SnapshotReadLeaseStateGuard<'a> { lock: &'a SnapshotReadLeaseStateLock, panicking_on_entry: bool, not_send: PhantomData<*mut ()> }

impl std::ops::Deref for SnapshotReadLeaseStateGuard<'_> {
    type Target = SnapshotReadLeaseRegistryState;
    fn deref(&self) -> &Self::Target { unsafe { &*self.lock.state.get() } }
}

impl std::ops::DerefMut for SnapshotReadLeaseStateGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.lock.state.get() } }
}

impl Drop for SnapshotReadLeaseStateGuard<'_> {
    fn drop(&mut self) {
        if !self.panicking_on_entry && std::thread::panicking() { self.lock.poisoned.store(true, std::sync::atomic::Ordering::Relaxed); }
        self.lock.held.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub struct SnapshotReadLeaseRegistry {
    pub(crate) state: SnapshotReadLeaseStateLock,
    pub(crate) returned: std::sync::atomic::AtomicUsize,
    pub(crate) lease_generations: SnapshotReadGenerationPages,
    authority_sequence: std::sync::atomic::AtomicU64,
    authority_generation: std::sync::atomic::AtomicU64,
    authority_revision: [std::sync::atomic::AtomicU64; 4],
}

impl SnapshotReadLeaseRegistry {
    /// 🪴️ Begins original registry custody without allocating any resident backing page.
    fn vacant() -> Self {
        let free=ORIGINAL_FREE_SLOTS;
        Self { state: SnapshotReadLeaseStateLock::new(SnapshotReadLeaseRegistryState {
            slots: SnapshotReadSlotPages { pages: std::array::from_fn(|_| None) },
            occupied: [0; SNAPSHOT_READ_LEASE_CAPACITY / 64], free, free_read: 0, free_len: SNAPSHOT_READ_LEASE_CAPACITY, next_generation: 0, cleanup_cursor: 0 }),
            returned: std::sync::atomic::AtomicUsize::new(0),
            lease_generations: SnapshotReadGenerationPages { pages: std::array::from_fn(|_| None) },
            authority_sequence: std::sync::atomic::AtomicU64::new(0), authority_generation: std::sync::atomic::AtomicU64::new(0),
            authority_revision: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)) }
    }

    /// 🧮️ Quotes one exact absent lease-slot or generation page before original birth.
    fn birth_page_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let state = self.state.try_lock().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "unpublished snapshot registry is busy"))?;
        let (capacity,copy) = if state.slots.pages.iter().any(Option::is_none) { (size_of::<SnapshotReadSlotPage>(),size_of::<Option<Box<SnapshotReadSlotPage>>>()) }
            else if self.lease_generations.pages.iter().any(Option::is_none) { (size_of::<SnapshotReadGenerationPage>(),size_of::<SnapshotReadGenerationPage>()+size_of::<Option<Box<SnapshotReadGenerationPage>>>()) } else { (0,0) };
        Ok(semio_framework_value::RetirementDemand { copy_bytes:copy, capacity_bytes: capacity, depth: usize::from(capacity != 0), ..Default::default() })
    }

    /// 🌿️ Installs only the page already paid by its original registry construction control.
    fn admit_birth_page(&mut self) -> Result<(), ValueError> {
        let state = self.state.get_mut().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "unpublished snapshot registry is poisoned"))?;
        if let Some(page) = state.slots.pages.iter_mut().find(|page| page.is_none()) {
            *page = Some(Box::new([const { std::mem::MaybeUninit::uninit() }; SNAPSHOT_READ_SLOT_PAGE]));
        } else if let Some(page) = self.lease_generations.pages.iter_mut().find(|page| page.is_none()) {
            *page = Some(Box::new([const{std::sync::atomic::AtomicU64::new(0)};SNAPSHOT_READ_GENERATION_PAGE]));
        }
        Ok(())
    }

    pub fn returned_count(&self) -> usize { self.returned.load(std::sync::atomic::Ordering::Acquire) }
    /// 🔒️ Observes the registry state lock so a caller can prove returned reads wait for it without blocking.
    pub fn try_lock_state(&self) -> std::sync::TryLockResult<SnapshotReadLeaseStateGuard<'_>> { self.state.try_lock() }

    pub fn new() -> Self {
        let free=ORIGINAL_FREE_SLOTS;
        Self {
            state: SnapshotReadLeaseStateLock::new(SnapshotReadLeaseRegistryState {
                slots: SnapshotReadSlotPages::new(),
                occupied: [0; SNAPSHOT_READ_LEASE_CAPACITY / 64],
                free,
                free_read: 0,
                free_len: SNAPSHOT_READ_LEASE_CAPACITY,
                next_generation: 0,
                cleanup_cursor: 0,
            }),
            returned: std::sync::atomic::AtomicUsize::new(0),
            lease_generations: SnapshotReadGenerationPages::new(),
            authority_sequence: std::sync::atomic::AtomicU64::new(0),
            authority_generation: std::sync::atomic::AtomicU64::new(0),
            authority_revision: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub fn publish_authority(&self, generation: u64, revision: [u8; 32]) -> bool {
        let sequence = self.authority_sequence.load(std::sync::atomic::Ordering::Acquire);
        let Some(terminal_sequence) = sequence.checked_add(2) else { return false };
        if sequence & 1 != 0 || self.authority_sequence.compare_exchange(sequence, sequence + 1, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return false;
        }
        self.authority_generation.store(generation, std::sync::atomic::Ordering::Relaxed);
        for (word, bytes) in self.authority_revision.iter().zip(revision.as_chunks::<8>().0.iter()) {
            word.store(u64::from_le_bytes(*bytes), std::sync::atomic::Ordering::Relaxed);
        }
        self.authority_sequence.store(terminal_sequence, std::sync::atomic::Ordering::Release);
        true
    }

    pub fn authority_matches(&self, generation: u64, revision: [u8; 32]) -> bool {
        let first = self.authority_sequence.load(std::sync::atomic::Ordering::Acquire);
        if first == 0 || first & 1 != 0 || self.authority_generation.load(std::sync::atomic::Ordering::Relaxed) != generation {
            return false;
        }
        for (word, bytes) in self.authority_revision.iter().zip(revision.as_chunks::<8>().0.iter()) {
            if word.load(std::sync::atomic::Ordering::Relaxed) != u64::from_le_bytes(*bytes) {
                return false;
            }
        }
        first == self.authority_sequence.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn try_take(&self, index: u16, generation: u64) -> Result<Arc<dyn std::any::Any + Send + Sync>, ValueError> {
        let Ok(mut state) = self.state.try_lock() else {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "snapshot read lease registry is busy"));
        };
        let index = index as usize;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease is no longer registered"));
        }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != generation {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease generation is stale"));
        }
        let slot = unsafe { state.slots[index].assume_init_read() };
        state.occupied[index / 64] &= !mask;
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        if self.lease_generations[index].swap(0, std::sync::atomic::Ordering::AcqRel) & (1u64 << 63) != 0 {
            self.returned.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
        Ok(slot.owner)
    }

    pub fn contains(&self, index: u16, generation: u64) -> bool {
        let Ok(state) = self.state.try_lock() else {
            return true;
        };
        let index = index as usize;
        let mask = 1 << (index % 64);
        state.occupied[index / 64] & mask != 0 && unsafe { state.slots[index].assume_init_ref() }.generation == generation
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.returned.load(std::sync::atomic::Ordering::Acquire) == 0 && self.state.try_lock().is_ok_and(|state| state.free_len == SNAPSHOT_READ_LEASE_CAPACITY)
    }

    pub fn has_returned(&self) -> bool {
        self.returned.load(std::sync::atomic::Ordering::Acquire) != 0
    }

    /// 🪫️ Quotes the next slot page an empty shared registry releases; generation pages wait for exclusive ownership.
    pub fn empty_backing_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let state = self.state.try_lock().map_err(|reason| match reason { std::sync::TryLockError::WouldBlock => SnapshotReadLeaseRefusal::Busy.into_value_error(), std::sync::TryLockError::Poisoned(_) => SnapshotReadLeaseRefusal::Poisoned.into_value_error() })?;
        let release_bytes = state.slots.next_release_bytes();
        Ok(semio_framework_value::RetirementDemand { release_bytes, depth: usize::from(release_bytes != 0), ..Default::default() })
    }
    /// ♻️ Releases one slot page of an empty shared registry; the last page completes the shared backing.
    pub fn close_empty_backing_step(&self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
        let grant=effect.grant();let mut work=0;let result=(||{
            let mut state=match self.state.try_lock(){Ok(state)=>state,Err(std::sync::TryLockError::WouldBlock)=>return Ok(RetainedCloneStep::Progress(Default::default())),Err(std::sync::TryLockError::Poisoned(_))=>return Err(SnapshotReadLeaseRefusal::Poisoned.into_value_error())};
            if state.free_len!=SNAPSHOT_READ_LEASE_CAPACITY||self.returned.load(std::sync::atomic::Ordering::Acquire)!=0{return Ok(RetainedCloneStep::Progress(Default::default()))}
            let bytes=state.slots.next_release_bytes();if bytes==0{return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0||effect.maximum_work_units()==0||grant.maximum_release_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"registry backing requires admitted depth"))}
            pay_pure_work(effect,&mut work)?;let progress=RetainedCloneProgress{copied_items:1,released_bytes:state.slots.release_page(),..Default::default()};Ok(if state.slots.next_release_bytes()==0{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
        })();RetainedWorkOutcome{result,work}
    }
    /// ⏪️ Quotes the next page an exclusively owned empty registry releases: slot pages, then generation pages.
    pub fn owned_backing_demands(&self)->Result<semio_framework_value::RetirementDemand,ValueError>{let shared=self.empty_backing_demands()?;if shared.release_bytes!=0{return Ok(shared)}let release_bytes=self.lease_generations.next_release_bytes();Ok(semio_framework_value::RetirementDemand{release_bytes,depth:usize::from(release_bytes!=0),..Default::default()})}
    /// ⛽️ Quotes the selected resident slot or generation page release before any effect.
    pub fn owned_backing_work_demand(&self)->Result<u64,ValueError>{let state=self.state.try_lock().map_err(|reason|match reason{std::sync::TryLockError::WouldBlock=>SnapshotReadLeaseRefusal::Busy.into_value_error(),std::sync::TryLockError::Poisoned(_)=>SnapshotReadLeaseRefusal::Poisoned.into_value_error()})?;Ok(u64::from(state.slots.pages.iter().any(Option::is_some)||self.lease_generations.pages.iter().any(Option::is_some)))}
    /// 📪️ Releases one selected original registry page and reports its actual work.
    pub fn close_owned_backing_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
        let grant=effect.grant();let shared=match self.empty_backing_demands(){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if shared.release_bytes!=0{let turn=self.close_empty_backing_step(effect);return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}
        let bytes=self.lease_generations.next_release_bytes();if bytes==0{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if grant.maximum_items==0||effect.maximum_work_units()==0||grant.maximum_release_bytes<bytes{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if grant.maximum_depth==0{return RetainedWorkOutcome{result:Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"registry backing requires admitted depth")),work:0}}
        let mut work=0;if let Err(error)=pay_pure_work(effect,&mut work){return RetainedWorkOutcome{result:Err(error),work}}let progress=RetainedCloneProgress{copied_items:1,released_bytes:self.lease_generations.release_page(),..Default::default()};RetainedWorkOutcome{result:Ok(if self.lease_generations.next_release_bytes()==0{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}),work}
    }

    pub fn occupied_count(&self) -> usize {
        self.state.try_lock().map_or(SNAPSHOT_READ_LEASE_CAPACITY, |state| SNAPSHOT_READ_LEASE_CAPACITY - state.free_len)
    }

    /// 🪶 Releases a lease whose root is still aliased outside this registry, on the returning [Design notes](README.md#source-notes-331).
    fn try_release_aliased(&self, index: u16, generation: u64) -> bool {
        let Ok(mut state) = self.state.try_lock() else { return false };
        let index = index as usize;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 {
            return false;
        }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != generation || self.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) & (1u64 << 63) != 0 || Arc::strong_count(&slot.owner) == 1 {
            return false;
        }
        drop(self.free_slot_locked(&mut state, index));
        true
    }

    /// 🆓️ Frees one occupied slot under the held registry lock and hands its owner to the caller to drop after unlocking.
    fn free_slot_locked(&self, state: &mut SnapshotReadLeaseStateGuard<'_>, index: usize) -> SnapshotReadLeaseSlot {
        let slot = unsafe { state.slots[index].assume_init_read() };
        state.occupied[index / 64] &= !(1 << (index % 64));
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        self.lease_generations[index].store(0, std::sync::atomic::Ordering::Release);
        slot
    }

    pub fn returned_admission_demands<T: Send + Sync + 'static>(&self, demand: impl FnOnce(&Arc<T>) -> semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, SnapshotReadLeaseRefusal> {
        let state = match self.state.try_lock() { Ok(state) => state, Err(std::sync::TryLockError::WouldBlock) => return Err(SnapshotReadLeaseRefusal::Busy), Err(std::sync::TryLockError::Poisoned(_)) => return Err(SnapshotReadLeaseRefusal::Poisoned) };
        let start = state.cleanup_cursor;
        let index = (0..=state.occupied.len()).find_map(|offset| { let word_index = (start / 64 + offset) % state.occupied.len(); let mut occupied = state.occupied[word_index]; if offset == 0 { occupied &= u64::MAX << (start % 64); } else if offset == state.occupied.len() { occupied &= (1u64 << (start % 64)) - 1; } (occupied != 0).then(|| word_index * 64 + occupied.trailing_zeros() as usize) });
        let Some(index) = index else { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); };
        if self.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) & (1u64 << 63) == 0 { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        let owner = Arc::clone(&slot.owner).downcast::<T>().map_err(|_| SnapshotReadLeaseRefusal::OwnerTypeChanged)?;
        Ok(demand(&owner))
    }

    pub fn try_admit_one_returned<T: Send + Sync + 'static, R>(&self, effect: &mut RetainedAllocationEffect<'_>, admission: impl FnOnce(&mut Option<Arc<T>>, &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<Option<(R, RetainedCloneProgress)>, OriginalRetirementFailure>) -> RetainedWorkOutcome<(Option<R>, RetainedCloneProgress), SnapshotReadReturnedRefusal> {
        let before = effect.performed_work();
        let mut result = self.admit_one_returned(effect, admission);
        let moved = result.as_ref().is_ok_and(|(_, progress)| *progress != RetainedCloneProgress::default());
        let mut work = effect.performed_work().saturating_sub(before);
        if moved && work == 0 { if let Err(error) = pay_pure_work(effect, &mut work) { result = Err(SnapshotReadLeaseRefusal::Admission(error).into()); } }
        RetainedWorkOutcome { result, work }
    }

    fn admit_one_returned<T: Send + Sync + 'static, R>(&self, effect: &mut RetainedAllocationEffect<'_>, admission: impl FnOnce(&mut Option<Arc<T>>, &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<Option<(R, RetainedCloneProgress)>, OriginalRetirementFailure>) -> Result<(Option<R>, RetainedCloneProgress), SnapshotReadReturnedRefusal> {
        let grant = effect.grant();
        if grant.maximum_items == 0 || !matches!(effect.remaining_work_units(), Ok(1..)) { return Ok((None, Default::default())); }
        if grant.maximum_depth == 0 { return Err(SnapshotReadLeaseRefusal::Admission(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned read admission requires depth")).into()); }
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(std::sync::TryLockError::WouldBlock) => return Err(SnapshotReadLeaseRefusal::Busy.into()),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(SnapshotReadLeaseRefusal::Poisoned.into()),
        };
        let start = state.cleanup_cursor;
        let Some(index) = (0..=state.occupied.len()).find_map(|offset| {
            let word_index = (start / 64 + offset) % state.occupied.len();
            let mut occupied = state.occupied[word_index];
            if offset == 0 {
                occupied &= u64::MAX << (start % 64);
            } else if offset == state.occupied.len() {
                occupied &= (1u64 << (start % 64)) - 1;
            }
            (occupied != 0).then(|| word_index * 64 + occupied.trailing_zeros() as usize)
        }) else {
            return Ok((None, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        };
        state.cleanup_cursor = (index + 1) % SNAPSHOT_READ_LEASE_CAPACITY;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 {
            return Ok((None, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if self.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) & (1u64 << 63) == 0 {
            return Ok((None, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if !slot.owner.is::<T>() {
            return Err(SnapshotReadLeaseRefusal::OwnerTypeChanged.into());
        }
        let slot = unsafe { state.slots[index].assume_init_read() };
        let generation = slot.generation;
        let Ok(original) = slot.owner.downcast::<T>() else { return Err(SnapshotReadLeaseRefusal::OwnerTypeChanged.into()); };
        let mut original = Some(original);
        let turn = admission(&mut original, &mut *effect);
        let (result, progress) = match (turn.result, original) {
            (Ok(Some(admitted)), None) => admitted,
            (Ok(Some(_)), Some(original)) => { state.slots[index].write(SnapshotReadLeaseSlot { generation, owner: original }); return Err(SnapshotReadLeaseRefusal::Admission(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned read admission published without consuming its original")).into()); },
            (Ok(None), Some(original)) => { state.slots[index].write(SnapshotReadLeaseSlot { generation, owner: original }); return Ok((None, Default::default())); },
            (Ok(None), None) => return Err(SnapshotReadLeaseRefusal::Admission(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned read admission consumed its original without publishing")).into()),
            (Err(error), Some(original)) => { state.slots[index].write(SnapshotReadLeaseSlot { generation, owner: original }); return Err(SnapshotReadReturnedRefusal::Retirement(error)); },
            (Err(error), None) => return Err(SnapshotReadReturnedRefusal::Retirement(error)),
        };
        state.occupied[index / 64] &= !mask;
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        self.returned.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        self.lease_generations[index].store(0, std::sync::atomic::Ordering::Release);
        Ok((Some(result), progress))
    }
}

impl Drop for SnapshotReadLeaseRegistry {
    fn drop(&mut self) {
        let state = self.state.get_mut().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(std::thread::panicking() || state.free_len == SNAPSHOT_READ_LEASE_CAPACITY, "snapshot read lease registry reached Drop before every exact owner was returned and retired: {} of {SNAPSHOT_READ_LEASE_CAPACITY}", state.free_len);
        assert!(std::thread::panicking() || self.returned.load(std::sync::atomic::Ordering::Acquire) == 0, "snapshot read lease registry reached Drop with returned owners outside the bounded retirement pump");
    }
}

pub struct SnapshotReadLeaseRegistryOwner(Option<SnapshotReadRegistryAliasRetirement>);

impl SnapshotReadLeaseRegistryOwner {
    /// 🧳️ Receives the exact admitted registry handle without constructing another registry.
    pub fn from_original(registry:SnapshotReadRegistryHandle)->Self{Self(Some(SnapshotReadRegistryAliasRetirement::new(registry)))}
    pub fn returned_count(&self) -> usize { self.0.as_ref().map_or(0, |owner| owner.returned_count()) }
    pub fn has_returned(&self)->bool{self.0.as_ref().is_some_and(|owner|SnapshotReadLeaseRegistry::has_returned(owner))}

    pub fn new() -> Self { Self(Some(SnapshotReadRegistryAliasRetirement::new(SnapshotReadRegistryHandle::new()))) }
    pub fn terminal_is_empty(&self) -> bool { self.0.as_ref().is_none_or(|owner| SnapshotReadLeaseRegistry::terminal_is_empty(owner)) }
    pub fn closed(&self) -> bool { self.0.is_none() }
    pub fn occupied_count(&self) -> usize { self.0.as_ref().map_or(0, |owner| owner.occupied_count()) }
    pub fn empty_backing_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.0.as_ref().map_or(Ok(Default::default()), |owner| owner.empty_backing_demands()) }
    pub fn close_empty_backing_step(&self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{self.0.as_ref().map_or(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0},|owner|owner.close_empty_backing_step(effect))}
    /// 📚️ Quotes the next release of the registry's own custody: its Arc frame, then, once sole, each generation page.
    pub fn custody_byte_demand(&self) -> Result<usize, ValueError> { self.0.as_ref().map_or(Ok(0), |owner| owner.custody_byte_demand()) }
    /// 🪜️ Releases the registry's own custody one frame or page at a time once it is the sole terminal owner.
    pub fn close_custody_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
        if self.0.as_ref().is_some_and(|owner| owner.strong_count() != 1 || !SnapshotReadLeaseRegistry::terminal_is_empty(owner)) { return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 }); }
        snapshot_registry_alias_close_step(&mut self.0, effect)
    }
}

pub fn snapshot_registry_frame_bytes() -> Result<usize, ValueError> {
    std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<SnapshotReadLeaseRegistry>()).map(|(layout, _)| layout.pad_to_align().size()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "snapshot registry Arc layout overflow"))
}

impl std::ops::Deref for SnapshotReadLeaseRegistryOwner {
    type Target = SnapshotReadRegistryHandle;
    fn deref(&self) -> &Self::Target { self.0.as_ref().expect("snapshot registry is unavailable after exact owned close").alias_handle() }
}

pub struct SnapshotReadLease {
    pub(crate) registry: SnapshotReadRegistryHandle,
    pub(crate) index: u16,
    pub(crate) generation: u64,
}

impl SnapshotReadLease {
    pub fn return_now(&mut self) -> bool {
        let mark = &self.registry.lease_generations[self.index as usize];
        if mark.load(std::sync::atomic::Ordering::Acquire) != self.generation { return false; }
        if self.registry.try_release_aliased(self.index, self.generation) { return true; }
        self.registry.returned.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        if mark.compare_exchange(self.generation, self.generation | (1u64 << 63), std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            self.registry.returned.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            return false;
        }
        true
    }
}

pub fn return_snapshot_read<T: ?Sized>(owner: &mut Option<Arc<T>>, lease: &mut Option<SnapshotReadLease>, after_alias_release: impl FnOnce()) -> bool {
    let Some(mut lease) = lease.take() else {
        assert!(owner.is_none(), "snapshot read alias requires its exact unreturned registry guard");
        return false;
    };
    drop(owner.take());
    after_alias_release();
    lease.return_now()
}

/// 🔐️ Returns only the exact original read while its registry slot protects the payload alias.
pub fn try_return_original_snapshot_read<T: ?Sized>(owner: &mut Option<Arc<T>>, lease: &mut Option<SnapshotReadLease>) -> Result<SnapshotReadReturn, ()> {
    let fields = lease.as_ref().ok_or(())?;
    let original_owner = owner.as_ref().ok_or(())?;
    let state = fields.registry.state.try_lock().map_err(|_| ())?;
    let index = fields.index as usize;
    if index >= SNAPSHOT_READ_LEASE_CAPACITY || !state.slots.resident() || state.occupied[index / 64] & (1 << (index % 64)) == 0 { return Err(()); }
    let slot = unsafe { state.slots[index].assume_init_ref() };
    if slot.generation != fields.generation || Arc::as_ptr(&slot.owner) as *const () != Arc::as_ptr(original_owner) as *const () { return Err(()); }
    fields.registry.lease_generations[index].compare_exchange(fields.generation, fields.generation | (1u64 << 63), std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).map_err(|_| ())?;
    fields.registry.returned.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    drop(owner.take());
    drop(state);
    let original = lease.take().expect("validated original read lease");
    Ok(SnapshotReadReturn { registry: original.registry, index: original.index, generation: original.generation })
}

pub struct SnapshotRead<T: ?Sized> {
    pub(crate) owner: Option<Arc<T>>,
    pub(crate) lease: Option<SnapshotReadLease>,
}

/// 🧾 Exact returned-read ticket retained by a mounted owner until store maintenance reclaims it.
pub struct SnapshotReadReturn {
    pub(crate) registry: SnapshotReadRegistryHandle,
    pub(crate) index: u16,
    pub(crate) generation: u64,
}

impl SnapshotReadReturn {
    pub fn terminal_is_empty(&self) -> bool {
        !self.registry.contains(self.index, self.generation)
    }
}

impl<T: ?Sized> SnapshotRead<T> {
    pub fn release_original_alias(&mut self) -> bool { self.owner.take().is_some() }
    pub fn return_original_lease(&mut self) -> Option<SnapshotReadRegistryAliasRetirement> { let mut lease = self.lease.take()?; lease.return_now(); Some(SnapshotReadRegistryAliasRetirement::new(lease.registry)) }

    pub fn belongs_to(&self, registry: &SnapshotReadRegistryHandle) -> bool { self.lease.as_ref().is_some_and(|lease| lease.registry.ptr_eq(registry)) }
    pub fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.lease.is_none() }
    pub fn lease_identity(&self) -> Option<(usize, u16, u64)> { self.lease.as_ref().map(|lease| (lease.registry.identity(), lease.index, lease.generation)) }

    /// 📏️ Quotes the captured read and the registry's original shared-root header.
    pub fn capture_copy_byte_demand() -> usize { size_of::<Self>() + size_of::<Arc<T>>() }

    pub fn new(owner: Arc<T>, lease: SnapshotReadLease) -> Self {
        Self { owner: Some(owner), lease: Some(lease) }
    }

    /// 🗿️ Borrows the captured immutable value without exposing its owner.
    pub fn get(&self) -> &T {
        self.owner.as_deref().expect("live snapshot read owner is present")
    }

    /// 🏛️ Revalidates this lease against the store-owned event authority without reading
    /// a session-local registry. A concurrent publication produces an odd or changed sequence and
    /// therefore fails closed.
    pub fn commit_authority_matches(&self, generation: u64, revision: [u8; 32]) -> bool {
        self.lease.as_ref().is_some_and(|lease| lease.registry.authority_matches(generation, revision))
    }

    /// 🧬️ Stages this original captured lease as source authority without replacing its registry.
    pub fn source_admission(self) -> Result<semio_framework_value::retained_clone::RetainedCloneSourceAdmission<T, Self>, (ValueError, Self)>
    where
        T: Sized + semio_framework_value::retirement::RetireOwned + Sync,
    {
        if self.owner.is_none() { return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read source requires its captured root"), self)); }
        Ok(semio_framework_value::retained_clone::RetainedCloneSource::admission_borrowed(self))
    }

    /// 🧹 Returns this lease to its exact registry while the registry retains the snapshot
    /// root for its own one-owner maintenance cursor.
    pub fn return_to_registry(mut self) -> bool {
        return_snapshot_read(&mut self.owner, &mut self.lease, || {})
    }

    /// 🧳️ Rejected return preserves the original read rather than consuming its registry custody.
    pub fn try_return_to_registry_witness(mut self)->Result<SnapshotReadReturn,Self>{
        match try_return_original_snapshot_read(&mut self.owner, &mut self.lease) { Ok(witness) => Ok(witness), Err(()) => Err(self) }
    }
    /// 📋️ Returns this lease and hands the caller an exact generation witness for store retirement.
    pub fn return_to_registry_witness(mut self) -> Option<SnapshotReadReturn> {
        let lease = self.lease.as_ref()?;
        let witness = SnapshotReadReturn { registry: lease.registry.clone(), index: lease.index, generation: lease.generation };
        if !return_snapshot_read(&mut self.owner, &mut self.lease, || {}) {
            return None;
        }
        Some(witness)
    }
}

impl<T: semio_framework_value::retirement::RetireOwned + Sync + 'static> semio_framework_value::retained_clone::OriginalSourceProjection<T> for SnapshotRead<T> {
    fn projection_work_demand(&self) -> Result<u64, ValueError> { Ok(1) }
    fn project_source<'reply, 'source>(&'source self, effect: &mut RetainedAllocationEffect<'_>, reply: semio_framework_value::retained_clone::OriginalProjectionReply<'reply, 'source, T>) -> Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'reply, std::ptr::NonNull<T>, ValueError>, ValueError> {
        if !effect.performed(1)? { return Ok(reply.complete(Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "snapshot read projection exceeded its original work")))); }
        Ok(reply.complete(Ok(self.get())))
    }
    fn recover_source_projection<'reply, 'source>(&'source self, _: &mut RetainedAllocationEffect<'_>, reply: semio_framework_value::retained_clone::OriginalProjectionReply<'reply, 'source, T>) -> Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'reply, std::ptr::NonNull<T>, ValueError>, ValueError> { Ok(reply.pending()) }
}

impl<T: Send + Sync + 'static> SnapshotRead<T> {
    /// 🪞️ Moves the original captured root and lease into erased custody without cloning or allocating.
    pub fn into_erased(mut self) -> ErasedSnapshotRead {
        ErasedSnapshotRead { owner: self.owner.take().map(|owner| owner as Arc<dyn std::any::Any + Send + Sync>), lease: self.lease.take() }
    }
}

impl<T: ?Sized> Drop for SnapshotRead<T> {
    fn drop(&mut self) {
        let _ = return_snapshot_read(&mut self.owner, &mut self.lease, || {});
    }
}

impl<T: ?Sized> std::ops::Deref for SnapshotRead<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

/// 🧰 Type-erased snapshot capability used only until a composition view selects its
/// repository-owned concrete artifact type.
pub struct ErasedSnapshotRead {
    pub(crate) owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
    pub(crate) lease: Option<SnapshotReadLease>,
}

impl ErasedSnapshotRead {
    pub fn belongs_to(&self, registry: &SnapshotReadRegistryHandle) -> bool { self.lease.as_ref().is_some_and(|lease| lease.registry.ptr_eq(registry)) }
    pub fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.lease.is_none() }
    pub fn lease_identity(&self) -> Option<(usize, u16, u64)> { self.lease.as_ref().map(|lease| (lease.registry.identity(), lease.index, lease.generation)) }

    /// 🐙️ Revalidates the original erased lease against its issuer's current event authority.
    pub fn commit_authority_matches(&self,generation:u64,revision:[u8;32])->bool{
        self.owner.is_some()&&self.lease.as_ref().is_some_and(|lease|lease.registry.authority_matches(generation,revision))
    }
    /// ⚛️ Generation leases use the registry's fixed inline atomics and allocate no per-read backing.
    pub const LEASE_ALLOCATION_BYTES: usize = 0;

    /// 📥️ Quotes the original private read, fixed registry slot and exact return metadata.
    pub fn prepared_return_demands() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { depth: 2, ..Default::default() }
    }

    /// 🎟️ Issues a fixed registry slot without allocating or copying snapshot payload.
    pub fn prepared_read_demands() -> semio_framework_value::RetirementDemand { semio_framework_value::RetirementDemand { depth: 2, ..Default::default() } }

    pub fn new<T: Send + Sync + 'static>(owner: Arc<T>, lease: SnapshotReadLease) -> Self {
        Self { owner: Some(owner), lease: Some(lease) }
    }

    /// 💎️ Acquires another genuine lease over this exact captured root and original issuer without allocating.
    pub fn try_duplicate(&self) -> Result<Self, ValueError> {
        let owner = self.owner.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original erased read has no captured root"))?;
        let lease = self.lease.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original erased read has no captured lease"))?;
        let mut state = lease.registry.state.try_lock().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "original read issuer is busy"))?;
        let index = lease.index as usize;
        if state.occupied[index / 64] & (1 << (index % 64)) == 0 || lease.registry.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) != lease.generation { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original read generation is unavailable")); }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != lease.generation || !Arc::ptr_eq(&slot.owner, owner) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original read issuer root changed")); }
        if state.free_len == 0 || !state.slots.resident() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original read issuer has no vacant slot")); }
        let generation = state.next_generation.checked_add(1).filter(|generation| *generation < 1u64 << 63).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original read issuer generation exhausted"))?;
        let next = state.free[state.free_read] as usize;
        state.free_read = (state.free_read + 1) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free_len -= 1;
        state.next_generation = generation;
        state.slots[next].write(SnapshotReadLeaseSlot { generation, owner: Arc::clone(owner) });
        lease.registry.lease_generations[next].store(generation, std::sync::atomic::Ordering::Release);
        state.occupied[next / 64] |= 1 << (next % 64);
        Ok(Self { owner: Some(Arc::clone(owner)), lease: Some(SnapshotReadLease { registry: lease.registry.clone(), index: next as u16, generation }) })
    }
    pub fn typed<T: Send + Sync + 'static>(&self) -> Option<SnapshotReadRef<'_, T>> {
        self.owner.as_deref()?.downcast_ref::<T>().map(|owner| SnapshotReadRef { owner })
    }

    /// ⌨️ Moves this original erased root and lease into typed custody; mismatch returns the exact original read.
    pub fn try_into_typed<T: Send + Sync + 'static>(mut self) -> Result<SnapshotRead<T>, Self> {
        if self.lease.is_none() { return Err(self); }
        let Some(owner) = self.owner.take() else { return Err(self); };
        match owner.downcast::<T>() {
            Ok(owner) => Ok(SnapshotRead { owner: Some(owner), lease: self.lease.take() }),
            Err(owner) => { self.owner = Some(owner); Err(self) }
        }
    }
    /// 🎯️ Borrows one exact concrete owner without exposing or cloning its retained `Arc`.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.owner.as_deref()?.downcast_ref::<T>()
    }

    pub fn admit_retirement<T: Send + Sync + 'static>(&mut self, registry: &SnapshotReadRegistryHandle, factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<T>>, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<Option<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)>, OriginalRetirementFailure> {
        let refuse = |error: ValueError| RetainedWorkOutcome { result: Err(error.into()), work: 0 };
        let Some(lease) = self.lease.as_ref().filter(|lease| lease.registry.ptr_eq(registry)) else { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its original registry")); };
        if self.owner.as_deref().is_none_or(|owner| !owner.is::<T>()) { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its original concrete owner")); }
        let Ok(mut state) = registry.state.try_lock() else { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "read admission registry is busy")); };
        let index = lease.index as usize;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 || registry.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) != lease.generation { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission generation is stale")); }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != lease.generation || self.owner.as_ref().is_none_or(|owner| !Arc::ptr_eq(&slot.owner, owner)) { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission must preserve its exact original slot")); }
        let Some(taken) = self.owner.take() else { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its original concrete owner")); };
        let typed = match taken.downcast::<T>() { Ok(typed) => typed, Err(taken) => { self.owner = Some(taken); return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its original concrete owner")); } };
        let mut original = Some(typed);
        let mut alias = Some(Arc::clone(factory));
        let mut output = None;
        let turn = settle_inline_poll(ArcRootRetirement::admit_original(&mut original, &mut alias, &mut output, effect), "read admission left its producer reply pending");
        drop(alias);
        if let Some(original) = original.take() { self.owner = Some(original); }
        let progress = match turn.result {
            Ok(step) => step.progress(),
            Err(error) => return RetainedWorkOutcome { result: Err(error.into()), work: turn.work },
        };
        let Some(admitted) = output else { return RetainedWorkOutcome { result: Ok(None), work: turn.work }; };
        let slot = unsafe { state.slots[index].assume_init_read() };
        state.occupied[index / 64] &= !mask;
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        registry.lease_generations[index].store(0, std::sync::atomic::Ordering::Release);
        drop(slot);
        self.lease.take();
        RetainedWorkOutcome { result: Ok(Some((admitted, progress))), work: turn.work }
    }

    pub fn into_typed<T: Send + Sync + 'static>(mut self, registry: &SnapshotReadRegistryHandle) -> Result<Arc<T>, Self> {
        if self.lease.as_ref().is_none_or(|lease| !lease.registry.ptr_eq(registry)) || self.owner.as_deref().is_none_or(|owner| !owner.is::<T>()) {
            return Err(self);
        }
        let lease = self.lease.take().expect("validated snapshot read lease is present");
        let owner = self.owner.take().expect("validated snapshot read owner is present").downcast::<T>().expect("validated snapshot read type");
        let guard = match registry.try_take(lease.index, lease.generation) {
            Ok(guard) => guard,
            Err(_) => {
                self.owner = Some(owner);
                self.lease = Some(lease);
                return Err(self);
            }
        };
        drop(guard);
        Ok(owner)
    }
}

impl Drop for ErasedSnapshotRead {
    fn drop(&mut self) {
        let _ = return_snapshot_read(&mut self.owner, &mut self.lease, || {});
    }
}

pub struct SnapshotReadRef<'a, T: ?Sized> {
    owner: &'a T,
}

impl<T: ?Sized> std::ops::Deref for SnapshotReadRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}





pub fn artifact_retirement_owned_birth_demands<T: semio_framework_value::retirement::RetireOwned>(pending: &Option<T>) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    if pending.is_none() { return Ok(Default::default()); }
    if !T::controlled_retirement_supported() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "typed retirement has no controlled owner authority")); }
    Ok(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), depth: 2, ..Default::default() })
}

pub fn artifact_retirement_admit_owned<T: semio_framework_value::retirement::RetireOwned>(pending: &mut Option<T>, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome {
    let grant = effect.grant();
    let mut work = 0;
    let result = (|| {
        let empty = RetainedCloneProgress::default();
        if active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "typed admission requires an empty original child slot")); }
        if pending.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || effect.maximum_work_units() == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = artifact_retirement_owned_birth_demands(pending)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "typed child birth exceeds admitted parent depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let turn = semio_framework_value::retirement::admit_owned_retirement(pending, &mut effect.nested(child));
        work += turn.work;
        match turn.result? {
            Some((owner, progress)) => {
                *active = Some(owner);
                semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "typed child birth")?;
                if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "typed child birth receipt differs from its exact current frame quote")); }
                Ok(RetainedCloneStep::Progress(progress))
            }
            None => Ok(RetainedCloneStep::Progress(empty)),
        }
    })();
    RetainedWorkOutcome { result, work }
}

pub fn artifact_retirement_shared_lease_birth_demands<T: semio_framework_value::retirement::RetireOwned + Sync>(pending: &Option<Arc<T>>) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    if pending.is_none() { return Ok(Default::default()); }
    if !T::controlled_retirement_supported() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "shared lease has no controlled owner authority")); }
    let mut demand = retained_read_retirement::arc_lease_admission_demand::<T>(); demand.depth += 1; Ok(demand)
}

pub fn artifact_retirement_admit_shared_lease<T: semio_framework_value::retirement::RetireOwned + Sync>(pending: &mut Option<Arc<T>>, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome {
    let grant = effect.grant();
    let mut work = 0;
    let result = (|| {
        let empty = RetainedCloneProgress::default();
        if active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "lease admission requires an empty original child slot")); }
        if pending.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || effect.maximum_work_units() == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = artifact_retirement_shared_lease_birth_demands(pending)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "lease birth exceeds admitted parent depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let turn = retained_read_retirement::admit_arc_lease(pending, &mut effect.nested(child));
        work += turn.work;
        match turn.result.map_err(failure_value_error)? {
            Some((owner, progress)) => { *active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
            None => Ok(RetainedCloneStep::Progress(empty)),
        }
    })();
    RetainedWorkOutcome { result, work }
}

type ReturnedAdmission = RetainedWorkOutcome<Option<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)>, OriginalRetirementFailure>;

fn admit_returned_root<P: Send + Sync + 'static>(registry: &SnapshotReadLeaseRegistry, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, effect: &mut RetainedAllocationEffect<'_>, admission: impl FnOnce(&mut Option<Arc<P>>, &mut RetainedAllocationEffect<'_>) -> ReturnedAdmission) -> semio_framework_value::OriginalRetirementOutcome {
    let grant = effect.grant();
    let turn = registry.try_admit_one_returned::<P, _>(effect, admission);
    let result = match turn.result {
        Ok((owner, receipt)) => { *active = owner; if !receipt.fits(grant) { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned read admission exceeded its original grant").into()) } else { Ok(RetainedCloneStep::Progress(receipt)) } },
        Err(SnapshotReadReturnedRefusal::Lease(SnapshotReadLeaseRefusal::Busy)) => Ok(RetainedCloneStep::Progress(Default::default())),
        Err(reason) => Err(reason.into_failure()),
    };
    RetainedWorkOutcome { result, work: turn.work }
}

pub fn advance_returned_snapshot_read<P: Send + Sync + 'static>(
    registry: &SnapshotReadLeaseRegistry,
    active: &mut Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    effect: &mut RetainedAllocationEffect<'_>,
) -> OriginalRetirementPoll {
    let done = |result: Result<RetainedCloneStep, OriginalRetirementFailure>| RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: 0 });
    if effect.grant().maximum_items == 0 { return done(Ok(RetainedCloneStep::Progress(Default::default()))); }
    if active.is_some() { return artifact_retirement_box_close_step(active, effect.grant().maximum_copy_bytes, effect); }
    if !registry.has_returned() { return done(Ok(RetainedCloneStep::Complete(Default::default()))); }
    RetainedWorkPoll::Complete(admit_returned_root::<P>(registry, active, effect, |root, effect| {
        let mut alias = Some(Arc::clone(factory));
        let mut output = None;
        let turn = settle_inline_poll(ArcRootRetirement::admit_original(root, &mut alias, &mut output, effect), "returned read admission left its producer reply pending");
        drop(alias);
        RetainedWorkOutcome { result: turn.result.map(|step| output.take().map(|owner| (owner, step.progress()))).map_err(Into::into), work: turn.work }
    }))
}

pub fn advance_returned_snapshot_read_root<P: Send + Sync + 'static>(
    registry: &SnapshotReadLeaseRegistry,
    active: &mut Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: &Arc<dyn SnapshotRetirementFactory<P>>,
    effect: &mut RetainedAllocationEffect<'_>,
) -> OriginalRetirementPoll {
    let done = |result: Result<RetainedCloneStep, OriginalRetirementFailure>| RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: 0 });
    if effect.grant().maximum_items == 0 { return done(Ok(RetainedCloneStep::Progress(Default::default()))); }
    if active.is_some() { return artifact_retirement_box_close_step(active, effect.grant().maximum_copy_bytes, effect); }
    if !registry.has_returned() { return done(Ok(RetainedCloneStep::Complete(Default::default()))); }
    RetainedWorkPoll::Complete(admit_returned_root::<P>(registry, active, effect, |root, effect| {
        let mut alias = Some(Arc::clone(factory));
        let mut output = None;
        let turn = settle_inline_poll(ArcRootRetirement::admit_snapshot(root, &mut alias, &mut output, effect), "returned read admission left its producer reply pending");
        drop(alias);
        RetainedWorkOutcome { result: turn.result.map(|step| output.take().map(|owner| (owner, step.progress()))).map_err(Into::into), work: turn.work }
    }))
}

#[path = "../../../../🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/♻️retirement/🦀️.rs"]
pub mod retained_read_retirement;
pub use semio_framework_value::factory::shared::root::{box_demands as artifact_retirement_box_demands, box_depth as artifact_retirement_box_depth, box_close_step as artifact_retirement_box_close_step};
pub use retained_read_retirement::{admit_arc_lease, arc_lease_admission_demand, arc_lease_birth_bytes};
use semio_framework_value::factory::ArcRootRetirement;
#[path = "../../../../🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧾️return/🦀️.rs"]
pub mod retained_read_return_retirement;
#[path = "../../../../🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🪞️projection/🦀️.rs"]
pub mod snapshot_read_projection;
pub use snapshot_read_projection::SnapshotReadProjection;

#[cfg(test)]
extern crate self as semio_framework_value_snapshot_read;
#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️testing/👥️presence-retirement/🦀️.rs"]
pub mod presence_test_retirement;

#[cfg(test)]
#[path="🧪️tests/👥️presence/🦀️.rs"]
mod presence_read_tests;

#[cfg(test)]
#[path="../../../../🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️snapshot-read-retirement/🦀️.rs"]
mod returned_retirement_tests;

#[cfg(test)]
#[path="🧪️tests/🔒️registry-lock/🦀️.rs"]
mod registry_lock_tests;

#[path="🪜️staging/🦀️.rs"]
pub mod source_staging;
pub use source_staging::SnapshotReadSourceStaging;

#[path="🔗️source/🦀️.rs"]
pub mod captured_snapshot_source;
pub use captured_snapshot_source::CapturedSnapshotSource;

#[path="👥️duplicate/🦀️.rs"]
mod original_duplicate;
#[path="📋️field-clone/🦀️.rs"]
pub mod field_clone;
pub use field_clone::SnapshotReadFieldClone;

