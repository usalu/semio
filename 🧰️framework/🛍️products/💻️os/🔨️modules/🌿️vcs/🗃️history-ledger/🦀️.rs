//! 🗃️ History ledger: the generation-keyed history ledger, its reservations, iterators and trait implementations.
//!
//! Holds items of the module; the root re-exports them.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactHistoryKey {
    pub index: u32,
    pub generation: u32,
}

/// 🧷️ A unique original ledger loan pins directory chunks until explicit funded return.
pub struct ArtifactHistoryDirectoryCursor<'a,T>{ledger:&'a mut ArtifactHistoryLedger<T>,revision:u64,current:*const ArtifactHistoryPageChunk<T>,directory:usize,target:Option<u32>,following:Option<u32>,forward:bool,ready:bool,active:bool}
/// 🚚️ The retained ledger loan prevents directory deallocation while the cursor moves between turns.
unsafe impl<T:Send+Sync> Send for ArtifactHistoryDirectoryCursor<'_,T> where ArtifactHistoryLedger<T>:Send{}
impl<'a,T> ArtifactHistoryDirectoryCursor<'a,T>{
    pub fn copy_bytes()->usize{2*size_of::<Self>()+2*size_of::<usize>()}
    fn progress()->semio_framework_value::RetainedCloneProgress{semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:Self::copy_bytes(),..Default::default()}}
    fn invalid()->semio_framework_value::ValueError{semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original history directory source changed")}
    fn fits(grant:semio_framework_value::RetainedCloneGrant)->bool{grant.maximum_items>0&&grant.maximum_depth>0&&grant.maximum_copy_bytes>=Self::copy_bytes()}
    fn matches(&self)->bool{self.active&&self.ledger.directory_reads>0&&self.ledger.read_count>0}
    /// 🎟️ Pins the genuine directory before its first pointer can survive a caller turn.
    pub fn admit_tail(ledger:&'a mut ArtifactHistoryLedger<T>,revision:u64,grant:semio_framework_value::RetainedCloneGrant,output:&mut Option<Self>)->semio_framework_value::RetainedWorkOutcome<(bool,semio_framework_value::RetainedCloneProgress)>{Self::admit(ledger,revision,grant,output,false)}
    /// 🎬️ Pins the same original directory for chronological traversal.
    pub fn admit_head(ledger:&'a mut ArtifactHistoryLedger<T>,revision:u64,grant:semio_framework_value::RetainedCloneGrant,output:&mut Option<Self>)->semio_framework_value::RetainedWorkOutcome<(bool,semio_framework_value::RetainedCloneProgress)>{Self::admit(ledger,revision,grant,output,true)}
    fn admit(ledger:&'a mut ArtifactHistoryLedger<T>,revision:u64,grant:semio_framework_value::RetainedCloneGrant,output:&mut Option<Self>,forward:bool)->semio_framework_value::RetainedWorkOutcome<(bool,semio_framework_value::RetainedCloneProgress)>{
        use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome};
        if output.is_some(){return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}if ledger.directory_reads!=0||ledger.directory_abandoned||ledger.reservation.is_some()||ledger.group.is_some()||ledger.replacement_count!=0{return RetainedWorkOutcome{result:Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"history directory awaits outstanding publication or loan")),work:0}}if !Self::fits(grant){return RetainedWorkOutcome{result:Ok((false,RetainedCloneProgress::default())),work:0}}let Some(count)=ledger.read_count.checked_add(1)else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}};let Some(directories)=ledger.directory_reads.checked_add(1)else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}};let bounds=ledger.visible_bounds();let target=if forward{bounds.0}else{bounds.1};let current=ledger.pages.as_ref().map_or(std::ptr::null(),|chunk|&**chunk as *const ArtifactHistoryPageChunk<T>);if target.is_some()&&current.is_null(){return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}ledger.read_count=count;ledger.directory_reads=directories;*output=Some(Self{ledger,revision,current,directory:0,target,following:None,forward,ready:target.is_none(),active:true});RetainedWorkOutcome{result:Ok((true,Self::progress())),work:1}
    }
    /// 📍️ One turn follows one pinned chunk or lends the exact visible slot and its following coordinate.
    pub fn advance<'s>(&'s mut self,revision:u64,grant:semio_framework_value::RetainedCloneGrant,fuel:u64)->semio_framework_value::RetainedWorkOutcome<(Option<(ArtifactHistoryKey,&'s T,Option<u32>)>,semio_framework_value::RetainedCloneProgress)>{
        use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome};
        if !self.matches()||self.revision!=revision{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}if self.ready||fuel==0||!Self::fits(grant){return RetainedWorkOutcome{result:Ok((None,RetainedCloneProgress::default())),work:0}}let Some(target)=self.target else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}};let page=target as usize/artifact_history_page_slots::<T>();let wanted=page/ARTIFACT_HISTORY_DIRECTORY_PAGES;if self.current.is_null()||self.directory>wanted{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}let chunk:&'s ArtifactHistoryPageChunk<T>=unsafe{&*self.current};if self.directory<wanted{let Some(next)=chunk.next.as_ref()else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:1}};self.current=&**next;self.directory+=1;return RetainedWorkOutcome{result:Ok((None,Self::progress())),work:1}}if target>=self.ledger.initialized{return RetainedWorkOutcome{result:Err(Self::invalid()),work:1}}let Some(page)=chunk.pages[page%ARTIFACT_HISTORY_DIRECTORY_PAGES].as_ref()else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:1}};let slot=unsafe{page.slots[target as usize%artifact_history_page_slots::<T>()].assume_init_ref()};if slot.visibility.as_ref().is_some_and(|visibility|!visibility.committed()){return RetainedWorkOutcome{result:Err(Self::invalid()),work:1}}let Some(value)=slot.get()else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:1}};let key=ArtifactHistoryKey{index:target,generation:slot.generation};self.following=if self.forward{slot.next}else{slot.previous};self.ready=true;RetainedWorkOutcome{result:Ok((Some((key,value,self.following)),Self::progress())),work:1}
    }
    /// 📖️ Lends the already located original row for one bounded inspection.
    pub fn current<'s>(&'s mut self,revision:u64,grant:semio_framework_value::RetainedCloneGrant,fuel:u64)->semio_framework_value::RetainedWorkOutcome<Option<(ArtifactHistoryKey,&'s T)>>{
        use semio_framework_value::RetainedWorkOutcome;
        if !self.matches()||self.revision!=revision||!self.ready{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}
        if fuel==0||!Self::fits(grant){return RetainedWorkOutcome{result:Ok(None),work:0}}
        let Some(target)=self.target else{return RetainedWorkOutcome{result:Ok(None),work:1}};
        let page=target as usize/artifact_history_page_slots::<T>();
        if self.current.is_null()||page/ARTIFACT_HISTORY_DIRECTORY_PAGES!=self.directory||target>=self.ledger.initialized{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}
        let chunk:&'s ArtifactHistoryPageChunk<T>=unsafe{&*self.current};
        let Some(page)=chunk.pages[page%ARTIFACT_HISTORY_DIRECTORY_PAGES].as_ref()else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:1}};
        let slot=unsafe{page.slots[target as usize%artifact_history_page_slots::<T>()].assume_init_ref()};
        let Some(value)=slot.get()else{return RetainedWorkOutcome{result:Err(Self::invalid()),work:1}};
        RetainedWorkOutcome{result:Ok(Some((ArtifactHistoryKey{index:target,generation:slot.generation},value))),work:1}
    }
    /// ➡️ Reports the pinned traversal direction.
    pub fn is_forward(&self)->bool{self.forward}
    /// 🧭️ Reports whether the pinned traversal has no current row or remaining following row.
    pub fn exhausted(&self)->bool{self.ready&&self.target.is_none()}
    /// 🪢️ Reports whether the current original row has a following row in this direction.
    pub fn has_following(&self)->bool{self.ready&&self.following.is_some()}
    /// 🪜️ Selects the following slot while preserving the same pinned directory loan.
    pub fn rearm_following(&mut self,revision:u64,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<(bool,semio_framework_value::RetainedCloneProgress)>{
        use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome};if !self.matches()||self.revision!=revision||!self.ready{return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}if !Self::fits(grant){return RetainedWorkOutcome{result:Ok((false,RetainedCloneProgress::default())),work:0}}let Some(target)=self.following else{return RetainedWorkOutcome{result:Ok((false,RetainedCloneProgress::default())),work:0}};let wanted=target as usize/artifact_history_page_slots::<T>()/ARTIFACT_HISTORY_DIRECTORY_PAGES;if wanted!=self.directory{self.current=self.ledger.pages.as_ref().map_or(std::ptr::null(),|chunk|&**chunk as *const _);self.directory=0;}self.target=Some(target);self.following=None;self.ready=false;RetainedWorkOutcome{result:Ok((true,Self::progress())),work:1}
    }
    /// ↩️ Returns the exact directory pin to its issuer, even after a revision changed.
    pub fn close(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<(bool,semio_framework_value::RetainedCloneProgress)>{
        use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome};if !self.matches(){return RetainedWorkOutcome{result:Err(Self::invalid()),work:0}}if !Self::fits(grant){return RetainedWorkOutcome{result:Ok((false,RetainedCloneProgress::default())),work:0}}self.ledger.read_count-=1;self.ledger.directory_reads-=1;self.active=false;self.current=std::ptr::null();RetainedWorkOutcome{result:Ok((true,Self::progress())),work:1}
    }
}
impl<T> Drop for ArtifactHistoryDirectoryCursor<'_,T>{fn drop(&mut self){if self.active{self.ledger.directory_abandoned=true;assert!(std::thread::panicking(),"original history directory requires its funded source return")}}}

/// 🧭️ Pages the original ledger backwards using its live generation keys and caller revision.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ArtifactHistoryReverseCursor {
    source: Option<(u64,usize,u64,usize,Option<u32>)>,
    next: Option<ArtifactHistoryKey>,
    remaining: usize,
    complete: bool,
}

impl ArtifactHistoryReverseCursor {
    pub fn new() -> Self { Self::default() }
    pub fn copy_bytes() -> usize { 2 * size_of::<Self>() + 2 * size_of::<ArtifactHistoryKey>() }
    pub fn complete(&self) -> bool { self.complete }
    /// 🪪️ Refuses another original ledger or revision before a pending turn can be retried.
    pub fn validate_source<T>(&self, ledger: &ArtifactHistoryLedger<T>, revision: u64) -> Result<(),semio_framework_value::ValueError> {
        let identity = (ledger.retained_issuance(),ledger.authority(),revision,ledger.len(),ledger.visible_bounds().1);
        if self.source.is_some_and(|source| source != identity) { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original history traversal source changed")); }
        Ok(())
    }
    /// 🤲️ Lends one actual entry only after the caller grants its fixed traversal and one work unit.
    pub fn advance<'a,T>(&mut self, ledger: &'a ArtifactHistoryLedger<T>, revision: u64, grant: semio_framework_value::RetainedCloneGrant, fuel: u64) -> semio_framework_value::RetainedWorkOutcome<(Option<(ArtifactHistoryKey,&'a T)>,semio_framework_value::RetainedCloneProgress)> {
        use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind};
        let identity = (ledger.retained_issuance(),ledger.authority(),revision,ledger.len(),ledger.visible_bounds().1);
        let invalid = || ValueError::literal(ValueRefusalKind::InvariantViolated,"original history traversal source changed");
        if let Err(error) = self.validate_source(ledger,revision) { return RetainedWorkOutcome { result: Err(error), work: 0 }; }
        let max_chunks=usize::try_from(ledger.page_count).unwrap_or(usize::MAX).saturating_sub(1)/ARTIFACT_HISTORY_DIRECTORY_PAGES+1;let quote=1u64.saturating_add((max_chunks as u64).saturating_mul(6));
        if self.complete || fuel < quote || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < Self::copy_bytes() { return RetainedWorkOutcome { result: Ok((None,RetainedCloneProgress::default())), work: 0 }; }
        let key = if self.source.is_none() { ledger.last_key() } else { self.next };
        let remaining = if self.source.is_none() { identity.3 } else { self.remaining };
        let initial=1u64.saturating_add(if self.source.is_none(){key.map_or(0,|key|ledger.retained_key_lookup_work(key))}else{0});
        let entry = match key { Some(key) => match ledger.get_key(key) { Some(value) => Some((key,value)), None => return RetainedWorkOutcome { result: Err(invalid()), work: u128::from(initial.saturating_add(ledger.retained_key_lookup_work(key))) } }, None if remaining == 0 => None, None => return RetainedWorkOutcome { result: Err(invalid()), work: u128::from(initial) } };
        let next = key.and_then(|key| ledger.previous_key(key));
        let after = remaining.saturating_sub(usize::from(entry.is_some()));
        let chunks=|key:ArtifactHistoryKey|1u64+(key.index as usize/artifact_history_page_slots::<T>()/ARTIFACT_HISTORY_DIRECTORY_PAGES)as u64;let actual=1u64.saturating_add(if self.source.is_none(){key.map_or(0,chunks)}else{0}).saturating_add(key.map_or(0,|key|chunks(key).saturating_mul(3))).saturating_add(next.map_or(0,|key|chunks(key).saturating_mul(2)));
        if (after == 0) != next.is_none() { return RetainedWorkOutcome { result: Err(invalid()), work: u128::from(actual) }; }
        self.source = Some(identity); self.next = next; self.remaining = after; self.complete = after == 0;
        RetainedWorkOutcome { result: Ok((entry,RetainedCloneProgress { copied_items: 1, copied_bytes: Self::copy_bytes(), ..Default::default() })), work: u128::from(actual) }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactHistoryReservation {
    authority: usize,
    pub(super) index: u32,
    pub(super) generation: u32,
}

#[path="../🔗️read/🦀️.rs"]
pub mod history_read;
pub use history_read::ArtifactHistoryReadKey;

pub(super) struct ArtifactHistorySlot<T> {
    pub(super) generation: u32,
    pub(super) visibility: Option<std::sync::Arc<ArtifactGroupVisibility>>,
    previous: Option<u32>,
    pub(super) next: Option<u32>,
    free_next: Option<u32>,
    original: semio_framework_value_original_slot::RetainedOriginalSlot<T>,
    replacement: Option<Box<semio_framework_value::RetainedPublicationRoot<T>>>,
    read_previous:Option<u32>,
    read_next:Option<u32>,
}

impl<T> ArtifactHistorySlot<T> {
    fn get(&self)->Option<&T>{self.replacement.as_deref().map(|root|&**root).or_else(||(!self.original.is_empty()).then(||self.original.get()))}
    fn is_occupied(&self)->bool{!self.original.is_empty()||self.replacement.is_some()}
    fn take_original(&mut self)->Option<T>{if self.original.has_read(){return None}std::mem::replace(&mut self.original,semio_framework_value_original_slot::RetainedOriginalSlot::vacant()).try_into_original().ok()}
}

pub(super) struct ArtifactHistorySlotPage<T> {
    pub(super) slots: Box<[std::mem::MaybeUninit<ArtifactHistorySlot<T>>]>,
}

pub(super) struct ArtifactHistoryPageChunk<T> {
    pub(super) pages: [Option<Box<ArtifactHistorySlotPage<T>>>; ARTIFACT_HISTORY_DIRECTORY_PAGES],
    pub(super) count: u16,
    pub(super) next: Option<Box<ArtifactHistoryPageChunk<T>>>,
}

impl<T> ArtifactHistoryPageChunk<T> {
    pub(super) fn blank() -> Self {
        Self { pages: std::array::from_fn(|_| None), count: 0, next: None }
    }
}

fn artifact_history_page_slots<T>()->usize{(ARTIFACT_HISTORY_RELEASE_PAGE_BYTES/4/size_of::<ArtifactHistorySlot<T>>().max(1)).clamp(1,ARTIFACT_HISTORY_PAGE_SLOTS)}

fn artifact_history_slot_page<T>() -> Box<[std::mem::MaybeUninit<ArtifactHistorySlot<T>>]> {
    let bytes = size_of::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>().saturating_mul(artifact_history_page_slots::<T>());
    assert!(bytes <= ARTIFACT_HISTORY_CONTIGUOUS_CEILING, "one history ledger page must stay under the 64 KiB contiguous ceiling");
    let mut slots = Vec::new();
    slots.try_reserve_exact(artifact_history_page_slots::<T>()).expect("history ledger page allocation");
    slots.resize_with(artifact_history_page_slots::<T>(), std::mem::MaybeUninit::uninit);
    slots.into_boxed_slice()
}

fn artifact_history_descend<'a, T>(chunk: &'a mut ArtifactHistoryPageChunk<T>, remaining: usize) -> &'a mut ArtifactHistoryPageChunk<T> {
    if remaining == 0 {
        return chunk;
    }
    if chunk.next.is_none() {
        chunk.next = Some(Box::new(ArtifactHistoryPageChunk::blank()));
    }
    artifact_history_descend(chunk.next.as_mut().expect("history ledger chunk link"), remaining - 1)
}

fn artifact_history_chunk<'a, T>(chunk: &'a ArtifactHistoryPageChunk<T>, remaining: usize) -> &'a ArtifactHistoryPageChunk<T> {
    if remaining == 0 { chunk } else { artifact_history_chunk(chunk.next.as_ref().expect("history ledger chunk link"), remaining - 1) }
}

/// 📚️ Paged generation-keyed history authority. Live entries form one stable linked order;
/// removed slots are tombstoned and reused only after their generation advances. A full page
/// opens another fixed page instead of refusing the entry. `seek` remembers the `(position, slot)` of the last entry a
/// positional read or a search reached, so reads that walk the order forward (applied order, replay order, a retained
/// initializer's cursor) cost O(1) amortized instead of O(position); every structural change forgets it.
static NEXT_HISTORY_READ_ISSUANCE:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);

pub struct ArtifactHistoryLedger<T> {
    pub(super) pages: std::mem::ManuallyDrop<Option<Box<ArtifactHistoryPageChunk<T>>>>,
    page_count: u32,
    read_issuance:u64,
    read_count:usize,
    directory_reads:usize,
    directory_abandoned:bool,
    replacement_count:usize,
    replacement_decision:Option<usize>,
    read_head:Option<u32>,
    initialized: u32,
    pub(super) head: Option<u32>,
    tail: Option<u32>,
    free_head: Option<u32>,
    pub(super) reservation: Option<ArtifactHistoryReservation>,
    group: Option<ArtifactHistoryGroupSuffix>,
    pub(super) len: usize,
    seek: std::sync::atomic::AtomicU64,
}

/// 🧭️ No remembered seek position.
const ARTIFACT_HISTORY_NO_SEEK: u64 = u64::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactHistoryReservationFault {
    Busy,
    Capacity,
    GenerationExhausted,
    GroupUnavailable,
}

impl<T> Default for ArtifactHistoryLedger<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArtifactHistoryLedger<T> {
    /// 📏️ Quotes the ledger-owned pin return left by an unwound directory cursor.
    pub fn abandoned_directory_demand(&self)->Option<semio_framework_value::RetirementDemand>{self.directory_abandoned.then_some(semio_framework_value::RetirementDemand{copy_bytes:2*size_of::<(bool,usize,usize)>(),depth:1,..Default::default()})}
    /// ↩️ Returns an abandoned pin only under the original ledger and admitted currencies.
    pub fn recover_abandoned_directory(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};let Some(demand)=self.abandoned_directory_demand()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if self.directory_reads!=1||self.read_count==0{return RetainedWorkOutcome{result:Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"abandoned directory pin lost its original ledger count")),work:0}}self.directory_reads-=1;self.read_count-=1;self.directory_abandoned=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}
    /// 📏️ Counts linked directory chunks touched by one exact indexed slot lookup.
    pub fn retained_key_lookup_work(&self,key:ArtifactHistoryKey)->u64{1u64+(key.index as usize/artifact_history_page_slots::<T>()/ARTIFACT_HISTORY_DIRECTORY_PAGES)as u64}
    /// 📐️ Quotes an indexed lookup from the cached page count without walking the directory.
    pub fn retained_lookup_ceiling(&self)->u64{1u64+u64::from(self.page_count.saturating_sub(1))/(ARTIFACT_HISTORY_DIRECTORY_PAGES as u64)}
    /// 📐️ Computes actual fixed page cardinality from the typed slot footprint and native physical page policy.
    pub fn page_slot_count()->usize{artifact_history_page_slots::<T>()}

    /// 🪹️ Creates an inline ledger frame without native slot or directory backing.
    pub fn empty() -> Self { Self { pages: std::mem::ManuallyDrop::new(None), page_count: 0, read_count:0, directory_reads:0, directory_abandoned:false, replacement_count:0, replacement_decision:None, read_head:None, read_issuance:NEXT_HISTORY_READ_ISSUANCE.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|value|value.checked_add(1)).expect("original history read issuance exhausted"), initialized: 0, head: None, tail: None, free_head: None, reservation: None, group: None, len: 0, seek: std::sync::atomic::AtomicU64::new(ARTIFACT_HISTORY_NO_SEEK) } }

    /// 🧮 Quotes the exact slot backing, original page and directory before birth.
    pub fn initial_page_birth_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.page_count != 0 { return Ok(Default::default()); }
        let capacity_bytes = size_of::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>().checked_mul(artifact_history_page_slots::<T>())
            .and_then(|bytes| bytes.checked_add(size_of::<ArtifactHistorySlotPage<T>>()))
            .and_then(|bytes| bytes.checked_add(size_of::<ArtifactHistoryPageChunk<T>>()))
            .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger original page extent overflowed"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }
    /// 🧱 True after the original first page remains installed.
    pub fn initial_page_is_admitted(&self) -> bool { self.page_count != 0 }
    /// 🎟️ Installs one genuine history page under the supplied independent currencies.
    pub fn admit_initial_page(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, ValueError> {
        if self.directory_reads!=0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"history directory is borrowed"));}
        if self.page_count != 0 { return Ok(Default::default()); }
        let demand = self.initial_page_birth_demand()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history ledger birth requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        self.open_page().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger original page allocation refused"))?;
        let progress = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original history ledger first page")?;
        Ok(progress)
    }

    pub fn new() -> Self {
        let mut ledger = Self::empty();
        ledger.open_page().expect("history ledger first page");
        ledger
    }

    pub(super) fn open_page(&mut self) -> Result<(), ()> {
        if self.page_count == u32::MAX {
            return Err(());
        }
        let page = Box::new(ArtifactHistorySlotPage { slots: artifact_history_slot_page() });
        let page_index = self.page_count as usize;
        if self.pages.is_none() {
            *self.pages = Some(Box::new(ArtifactHistoryPageChunk::blank()));
        }
        {
            let chunk = artifact_history_descend(self.pages.as_mut().expect("history ledger chunk"), page_index / ARTIFACT_HISTORY_DIRECTORY_PAGES);
            let offset = page_index % ARTIFACT_HISTORY_DIRECTORY_PAGES;
            chunk.pages[offset] = Some(page);
            chunk.count = chunk.count.saturating_add(1);
        }
        self.page_count += 1;
        Ok(())
    }

    fn slot_page(&self, page: usize) -> &ArtifactHistorySlotPage<T> {
        let head = self.pages.as_ref().expect("history ledger owns its first page");
        artifact_history_chunk(head, page / ARTIFACT_HISTORY_DIRECTORY_PAGES).pages[page % ARTIFACT_HISTORY_DIRECTORY_PAGES].as_ref().expect("history ledger page")
    }

    fn slot_page_mut(&mut self, page: usize) -> &mut ArtifactHistorySlotPage<T> {
        let head = self.pages.as_mut().expect("history ledger owns its first page");
        artifact_history_descend(head, page / ARTIFACT_HISTORY_DIRECTORY_PAGES).pages[page % ARTIFACT_HISTORY_DIRECTORY_PAGES].as_mut().expect("history ledger page")
    }

    fn slot(&self, index: u32) -> &ArtifactHistorySlot<T> {
        let index = index as usize;
        unsafe { self.slot_page(index / artifact_history_page_slots::<T>()).slots[index % artifact_history_page_slots::<T>()].assume_init_ref() }
    }

    fn slot_mut(&mut self, index: u32) -> &mut ArtifactHistorySlot<T> {
        let index = index as usize;
        unsafe { self.slot_page_mut(index / artifact_history_page_slots::<T>()).slots[index % artifact_history_page_slots::<T>()].assume_init_mut() }
    }

    pub fn authority(&self) -> usize {
        self.pages.as_ref().map(|chunk| chunk.as_ref() as *const ArtifactHistoryPageChunk<T> as usize).unwrap_or(0)
    }

    fn allocated_slots(&self) -> usize {
        self.page_count as usize * artifact_history_page_slots::<T>()
    }

    /// 📜️ Reports backing required by the next unoccupied history reservation.
    pub fn next_reservation_allocation_bytes(&self) -> usize {
        if self.reservation.is_some() || self.free_head.is_some() || (self.initialized as usize) < self.allocated_slots() { return 0; }
        size_of::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>().saturating_mul(artifact_history_page_slots::<T>())
            .saturating_add(size_of::<ArtifactHistorySlotPage<T>>())
            .saturating_add(if self.page_count as usize % ARTIFACT_HISTORY_DIRECTORY_PAGES == 0 { size_of::<ArtifactHistoryPageChunk<T>>() } else { 0 })
    }

    /// 📏️ Prices exactly one original row header and the native backing needed by its real next slot.
    pub fn owned_row_admission_demands(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:size_of::<T>()*2+size_of::<ArtifactHistoryReservation>()*2,capacity_bytes:self.next_reservation_allocation_bytes(),depth:1,..Default::default()}}
    /// 📥️ Moves one original deque row into this ledger without cloning payloads or reallocating its source backing.
    /// 📥️ The actual retained row slot stays the donor until its unreserved ledger admits complete backing and metadata.
    pub fn admit_owned_row(&mut self,source:&mut Option<T>,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<semio_framework_value::RetainedCloneProgress>{
        use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome,ValueRefusalKind};
        if self.directory_reads!=0||self.group.is_some()||self.reservation.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native row admission requires its unreserved original ledger")),work:0}}
        if source.is_none(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
        let demand=self.owned_row_admission_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
        match self.try_push(source.take().unwrap()){
            Ok(_)=>RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}),work:1},
            Err(row)=>{*source=Some(row);let p=RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<T>(),..Default::default()};RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native original row admission exhausted its ledger").with_retained_progress(p)),work:1}}
        }
    }
    pub fn admit_deque_front(&mut self,source:&mut std::collections::VecDeque<T>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,ValueError>{if self.directory_reads!=0||self.group.is_some()||self.reservation.is_some(){return Err(ValueError::literal(protocol::value::ValueRefusalKind::InvariantViolated,"native row admission requires its exact unreserved original ledger"))}if source.is_empty(){return Ok(Default::default())}let d=self.owned_row_admission_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}let row=source.pop_front().unwrap();if let Err(row)=self.try_push(row){source.push_front(row);return Err(ValueError::literal(protocol::value::ValueRefusalKind::OwnershipLimit,"native original row admission exhausted its ledger"))}Ok(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})}

    pub fn reserve_one(&mut self) -> Result<ArtifactHistoryReservation, ArtifactHistoryReservationFault> {
        if self.directory_reads!=0 || self.group.is_some() || self.replacement_count!=0 {
            return Err(ArtifactHistoryReservationFault::GroupUnavailable);
        }
        self.reserve_slot()
    }

    fn reserve_slot(&mut self) -> Result<ArtifactHistoryReservation, ArtifactHistoryReservationFault> {
        if self.reservation.is_some() {
            return Err(ArtifactHistoryReservationFault::Busy);
        }
        let (index, generation) = if let Some(index) = self.free_head {
            let generation = self.slot(index).generation.checked_add(1).ok_or(ArtifactHistoryReservationFault::GenerationExhausted)?;
            (index, generation)
        } else {
            if self.initialized as usize == self.allocated_slots() && self.open_page().is_err() {
                return Err(ArtifactHistoryReservationFault::Capacity);
            }
            (self.initialized, 1)
        };
        let reservation = ArtifactHistoryReservation { authority: self.authority(), index, generation };
        self.reservation = Some(ArtifactHistoryReservation { authority: reservation.authority, index, generation });
        Ok(reservation)
    }

    pub fn cancel_reservation(&mut self, reservation: ArtifactHistoryReservation) -> Result<(), ArtifactHistoryReservation> {
        if self.directory_reads!=0 || self.reservation.as_ref() != Some(&reservation) {
            return Err(reservation);
        }
        self.reservation = None;
        Ok(())
    }

    pub fn insert_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        if self.directory_reads!=0 || self.group.is_some() {
            return Err((reservation, value));
        }
        let key = self.insert_owned_slot(reservation, value, self.tail)?;
        if self.head.is_none() {
            self.head = Some(key.index);
        }
        self.tail = Some(key.index);
        self.len += 1;
        Ok(key)
    }

    fn insert_owned_slot(&mut self, reservation: ArtifactHistoryReservation, value: T, previous: Option<u32>) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        if reservation.authority != self.authority() || self.reservation.as_ref() != Some(&reservation) {
            return Err((reservation, value));
        }
        let index = reservation.index;
        let generation = reservation.generation;
        self.reservation = None;
        self.forget_seek();
        if Some(index) == self.free_head {
            let free_next = self.slot(index).free_next;
            self.free_head = free_next;
            let slot = self.slot_mut(index);
            slot.generation = generation;
            slot.visibility = None;
            slot.previous = previous;
            slot.next = None;
            slot.free_next = None;
            slot.original = semio_framework_value_original_slot::RetainedOriginalSlot::new(value);
        } else if index == self.initialized && (index as usize) < self.allocated_slots() {
            let offset = index as usize;
            self.slot_page_mut(offset / artifact_history_page_slots::<T>()).slots[offset % artifact_history_page_slots::<T>()].write(ArtifactHistorySlot { generation, visibility: None, previous, next: None, free_next: None, original: semio_framework_value_original_slot::RetainedOriginalSlot::new(value), replacement:None, read_previous:None, read_next:None });
            self.initialized += 1;
        } else {
            return Err((reservation, value));
        }
        if let Some(tail) = previous {
            self.slot_mut(tail).next = Some(index);
        }
        Ok(ArtifactHistoryKey { index, generation })
    }

    /// 🪢️ Binds an already reserved slot to its exact still-private visibility owner.
    pub fn bind_group_reservation(&mut self, reservation: &ArtifactHistoryReservation, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<(), ()> {
        if self.directory_reads!=0 || self.reservation.as_ref() != Some(reservation) || !visibility.pending() || self.group.as_ref().is_some_and(|group| !std::sync::Arc::ptr_eq(&group.visibility, visibility)) { return Err(()); }
        if self.group.is_none() { self.group = Some(ArtifactHistoryGroupSuffix { visibility: std::sync::Arc::clone(visibility), head: None, tail: None, len: 0 }); }
        Ok(())
    }

    /// 🏷️ Prices the next exact native reservation, its typed backing and shared original decision header.
    pub fn group_reservation_demands(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:size_of::<ArtifactHistoryReservation>()*2+size_of::<ArtifactHistoryGroupSuffix>(),capacity_bytes:self.next_reservation_allocation_bytes(),depth:1,..Default::default()}}
    /// 🎭️ Stages one real reservation only after every independent original currency is admitted.
    pub fn reserve_group_controlled(&mut self,visibility:&std::sync::Arc<ArtifactGroupVisibility>,slot:&mut Option<ArtifactHistoryReservation>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,ValueError>{if slot.is_some()||!visibility.pending()||self.group.as_ref().is_some_and(|group|!std::sync::Arc::ptr_eq(&group.visibility,visibility)){return Err(ValueError::literal(protocol::value::ValueRefusalKind::InvariantViolated,"original group reservation changed its source or recipient"))}let d=self.group_reservation_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}*slot=Some(self.reserve_group_one(visibility).map_err(|_|ValueError::literal(protocol::value::ValueRefusalKind::OwnershipLimit,"original group history reservation refused"))?);Ok(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})}

    pub fn reserve_group_one(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<ArtifactHistoryReservation, ArtifactHistoryReservationFault> {
        if self.directory_reads!=0 || !visibility.pending() || self.group.as_ref().is_some_and(|group| !std::sync::Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(ArtifactHistoryReservationFault::GroupUnavailable);
        }
        let reservation = self.reserve_slot()?;
        if self.group.is_none() {
            self.group = Some(ArtifactHistoryGroupSuffix { visibility: std::sync::Arc::clone(visibility), head: None, tail: None, len: 0 });
        }
        Ok(reservation)
    }

    pub fn stage_group_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        if self.directory_reads!=0{return Err((reservation,value));}
        let Some(group) = self.group.as_ref().filter(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility) && visibility.pending()) else {
            return Err((reservation, value));
        };
        let previous = group.tail.or(self.tail);
        let key = self.insert_owned_slot(reservation, value, previous)?;
        self.slot_mut(key.index).visibility = Some(std::sync::Arc::clone(visibility));
        let group = self.group.as_mut().expect("validated group suffix remains owned");
        if group.head.is_none() {
            group.head = Some(key.index);
        }
        group.tail = Some(key.index);
        group.len += 1;
        Ok(key)
    }

    pub fn adopt_group(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<(), ()> {
        if self.directory_reads!=0 || self.reservation.is_some() || !visibility.committed() || !self.group.as_ref().is_some_and(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        let group = self.group.take().expect("validated group suffix remains owned");
        self.forget_seek();
        self.head = self.head.or(group.head);
        self.tail = group.tail.or(self.tail);
        self.len += group.len;
        Ok(())
    }

    pub fn abort_group_one(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<Option<T>, ()> {
        if self.directory_reads!=0 || self.reservation.is_some() || visibility.pending() || visibility.committed() {
            return Err(());
        }
        let group = self.group.as_ref().filter(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility)).ok_or(())?;
        let Some(index) = group.tail else {
            self.group = None;
            return Ok(None);
        };
        self.forget_seek();
        let previous = self.slot(index).previous;
        if let Some(previous) = previous {
            self.slot_mut(previous).next = None;
        }
        let free_head = self.free_head;
        let value = {
            let slot = self.slot_mut(index);
            slot.previous = None;
            slot.next = None;
            slot.free_next = free_head;
            slot.take_original().expect("aborted group slot retains its exact entry owner")
        };
        self.free_head = Some(index);
        let group = self.group.as_mut().expect("validated aborted group remains owned");
        group.len -= 1;
        group.tail = if group.len == 0 { None } else { previous };
        if group.len == 0 {
            group.head = None;
        }
        Ok(Some(value))
    }

    fn visible_bounds(&self) -> (Option<u32>, Option<u32>, usize) {
        match self.group.as_ref().filter(|group| group.visibility.committed()) {
            Some(group) => (self.head.or(group.head), group.tail.or(self.tail), self.len + group.len),
            None => (self.head, self.tail, self.len),
        }
    }

    pub fn group_visibility(&self) -> Option<&ArtifactGroupVisibility> {
        self.group.as_ref().map(|group| group.visibility.as_ref())
    }

    pub fn read_group(&self, decision: Option<&ArtifactGroupReadDecision<'_>>) -> Result<ArtifactHistoryIter<'_, T>, ()> {
        let (front, back, remaining) = match self.group.as_ref() {
            Some(group) if decision.ok_or(())?.committed_for(&group.visibility)? => (self.head.or(group.head), group.tail.or(self.tail), self.len + group.len),
            _ => (self.head, self.tail, self.len),
        };
        Ok(ArtifactHistoryIter { ledger: self, front, back, remaining })
    }

    pub fn try_push(&mut self, value: T) -> Result<ArtifactHistoryKey, T> {
        let reservation = match self.reserve_one() {
            Ok(reservation) => reservation,
            Err(_) => return Err(value),
        };
        self.insert_reserved(reservation, value).map_err(|(_, value)| value)
    }

    /// 📎️ Inserts before an exact generation key without moving any existing payload or changing its identity.
    pub fn try_insert_before(&mut self,next:ArtifactHistoryKey,value:T)->Result<ArtifactHistoryKey,T>{if self.directory_reads!=0||self.group.is_some()||self.get_key(next).is_none(){return Err(value)}let previous=self.slot(next.index).previous;let reservation=match self.reserve_one(){Ok(reservation)=>reservation,Err(_)=>return Err(value)};let key=match self.insert_owned_slot(reservation,value,previous){Ok(key)=>key,Err((reservation,value))=>{self.cancel_reservation(reservation).ok();return Err(value)}};self.slot_mut(key.index).next=Some(next.index);self.slot_mut(next.index).previous=Some(key.index);if previous.is_none(){self.head=Some(key.index)}self.len+=1;self.forget_seek();Ok(key)}

    pub fn try_from_preflighted(values: Vec<T>) -> Result<Self, Vec<T>> {
        let mut ledger = Self::new();
        let mut pending = values.into_iter();
        while let Some(value) = pending.next() {
            if let Err(value) = ledger.try_push(value) {
                let mut rejected = Vec::new();
                while let Some(established) = ledger.pop() {
                    rejected.push(established);
                }
                rejected.reverse();
                rejected.push(value);
                rejected.extend(pending);
                return Err(rejected);
            }
        }
        Ok(ledger)
    }

    /// 🪃️ Quotes one exact original unlink before a publisher changes any companion roots.
    pub fn removal_demands(&self,key:ArtifactHistoryKey)->Result<semio_framework_value::RetirementDemand,ValueError>{if self.directory_reads!=0||self.reservation.is_some()||self.group.is_some()||self.replacement_count!=0||key.index>=self.initialized{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history removal requires its settled original generation"))}let slot=self.slot(key.index);if slot.generation!=key.generation||slot.original.is_empty()||slot.original.has_read(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history removal requires its unborrowed original row"))}Ok(semio_framework_value::RetirementDemand{copy_bytes:2*size_of::<T>()+12*size_of::<usize>(),depth:1,..Default::default()})}

    pub fn remove_key(&mut self, key: ArtifactHistoryKey) -> Result<T, ArtifactHistoryKey> {
        if self.directory_reads!=0 || self.reservation.is_some() || self.group.is_some() || self.replacement_count!=0 {
            return Err(key);
        }
        if key.index >= self.initialized {
            return Err(key);
        }
        let slot = self.slot(key.index);
        if slot.generation != key.generation || slot.original.is_empty() || slot.original.has_read() {
            return Err(key);
        }
        self.forget_seek();
        let previous = slot.previous;
        let next = slot.next;
        if let Some(previous) = previous {
            self.slot_mut(previous).next = next;
        } else {
            self.head = next;
        }
        if let Some(next) = next {
            self.slot_mut(next).previous = previous;
        } else {
            self.tail = previous;
        }
        let free_head = self.free_head;
        let value = {
            let slot = self.slot_mut(key.index);
            slot.previous = None;
            slot.next = None;
            slot.free_next = free_head;
            slot.take_original().expect("validated history slot retains its exact owner")
        };
        self.free_head = Some(key.index);
        self.len -= 1;
        Ok(value)
    }

    /// ✂️ Removes and answers every entry `remove` selects, in ledger order — how a ledger follows a fold whose facts shrank
    /// (a retracted transition). Refused, removing nothing, while a reservation or a group decision is open.
    pub fn extract_if(&mut self, mut remove: impl FnMut(&T) -> bool) -> Result<Vec<T>, ArtifactHistoryReservationFault> {
        if self.directory_reads!=0 || self.group.is_some() {
            return Err(ArtifactHistoryReservationFault::GroupUnavailable);
        }
        if self.reservation.is_some() {
            return Err(ArtifactHistoryReservationFault::Busy);
        }
        let mut keys = Vec::new();
        let mut cursor = self.head;
        while let Some(index) = cursor {
            let slot = self.slot(index);
            if slot.get().is_some_and(&mut remove) {
                if slot.original.has_read(){return Err(ArtifactHistoryReservationFault::Busy);}
                keys.push(ArtifactHistoryKey { index, generation: slot.generation });
            }
            cursor = slot.next;
        }
        Ok(keys.into_iter().filter_map(|key| self.remove_key(key).ok()).collect())
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.tail?;
        let generation = self.slot(index).generation;
        self.remove_key(ArtifactHistoryKey { index, generation }).ok()
    }

    pub fn first(&self) -> Option<&T> {
        self.visible_bounds().0.and_then(|index| self.slot(index).get())
    }

    pub fn last(&self) -> Option<&T> {
        self.visible_bounds().1.and_then(|index| self.slot(index).get())
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.directory_reads!=0 || self.group.is_some() || self.replacement_count!=0 {
            return None;
        }
        let index = self.tail?;
        self.slot_mut(index).original.get_mut_unissued()
    }

    /// 🎯️ The entry at `position` of the visible order — O(1) amortized when reads walk forward (see `seek`).
    pub fn get(&self, position: usize) -> Option<&T> {
        let index = self.seek_slot(position)?;
        self.slot(index).get()
    }

    /// 🗝️ Reads an exact live slot generation while honoring its staged group decision.
    pub fn get_key(&self, key: ArtifactHistoryKey) -> Option<&T> {
        if key.index >= self.initialized { return None; }
        let slot = self.slot(key.index);
        if slot.generation != key.generation || slot.visibility.as_ref().is_some_and(|visibility| !visibility.committed()) { return None; }
        slot.get()
    }

    /// 🔚️ Borrows the visible tail generation without walking positional history.
    pub fn last_key(&self)->Option<ArtifactHistoryKey>{let index=self.visible_bounds().1?;Some(ArtifactHistoryKey{index,generation:self.slot(index).generation})}

    /// ⏮️ Borrows one preceding visible link from the unchanged original slot generation.
    pub fn previous_key(&self,key:ArtifactHistoryKey)->Option<ArtifactHistoryKey>{self.get_key(key)?;let index=self.slot(key.index).previous?;let previous=ArtifactHistoryKey{index,generation:self.slot(index).generation};self.get_key(previous)?;Some(previous)}

    /// 🪪️ Identifies the genuine original ledger independently of moves of its containing Store.
    pub fn retained_issuance(&self) -> u64 { self.read_issuance }

    /// 🤲️ Mutably borrows one exact original generation only after every loan and staged history group has returned.
    pub fn get_key_mut(&mut self, key: ArtifactHistoryKey) -> Option<&mut T> {
        if self.directory_reads!=0 || self.group.is_some() || self.replacement_count!=0 || key.index >= self.initialized { return None; }
        let slot = self.slot_mut(key.index);
        if slot.generation != key.generation || slot.visibility.as_ref().is_some_and(|visibility| !visibility.committed()) || slot.original.has_read() { return None; }
        slot.original.get_mut_unissued()
    }

    /// 🧢️ Captures the exact generation of the visible entry at `position`.
    pub fn key_at(&self, position: usize) -> Option<ArtifactHistoryKey> {
        let index = self.seek_slot(position)?;
        Some(ArtifactHistoryKey { index, generation: self.slot(index).generation })
    }

    pub fn get_mut(&mut self, position: usize) -> Option<&mut T> {
        assert!(self.group.is_none(), "mutable history access requires its staged group to be adopted or aborted");
        if self.directory_reads!=0||self.replacement_count!=0{return None}
        let index = self.seek_slot(position)?;
        self.slot_mut(index).original.get_mut_unissued()
    }

    /// 🔎️ The first visible entry `found` accepts, searched from the last entry a read reached onward and then from the
    /// front — O(1) amortized for lookups that follow the order (an id per applied edit, a replay's next edit).
    pub fn find_near(&self, found: impl FnMut(&T) -> bool) -> Option<&T> {
        self.find_near_position(found).map(|(_, value)| value)
    }

    /// 🔦️ [`Self::find_near`] answering the entry's position in the visible order too.
    pub fn find_near_position(&self, found: impl FnMut(&T) -> bool) -> Option<(usize, &T)> {
        self.find_near_entry(found).map(|(position, _, value)| (position, value))
    }

    /// 🌀️ [`Self::find_near`] with the matched generation key.
    pub fn find_near_key(&self, found: impl FnMut(&T) -> bool) -> Option<(ArtifactHistoryKey, &T)> {
        self.find_near_entry(found).map(|(_, key, value)| (key, value))
    }

    fn find_near_entry(&self, mut found: impl FnMut(&T) -> bool) -> Option<(usize, ArtifactHistoryKey, &T)> {
        let (front, _, remaining) = self.visible_bounds();
        let (start_position, start) = match self.remembered_seek() {
            Some((position, index)) if position < remaining => (position, Some(index)),
            _ => (0, front),
        };
        let mut cursor = (start_position, start);
        for _ in 0..remaining {
            let (position, Some(index)) = cursor else { break };
            let slot = self.slot(index);
            if slot.get().is_some_and(&mut found) {
                self.remember_seek(position, index);
                return slot.get().map(|value| (position, ArtifactHistoryKey { index, generation: slot.generation }, value));
            }
            cursor = match slot.next {
                Some(next) if position + 1 < remaining => (position + 1, Some(next)),
                _ => (0, front),
            };
        }
        None
    }

    /// 🌁️ The slot at `position` of the visible order, walking forward from the remembered seek when it lies at or
    /// before `position`, else from the front; remembers where it landed.
    fn seek_slot(&self, position: usize) -> Option<u32> {
        let (front, _, remaining) = self.visible_bounds();
        if position >= remaining {
            return None;
        }
        let (mut at, mut index) = match self.remembered_seek() {
            Some((known, index)) if known <= position => (known, index),
            _ => (0, front?),
        };
        while at < position {
            index = self.slot(index).next?;
            at += 1;
        }
        self.remember_seek(position, index);
        Some(index)
    }

    /// 📛️ The remembered `(position, slot)`, only while it still names a live entry and no staged group shifts the order.
    fn remembered_seek(&self) -> Option<(usize, u32)> {
        let packed = self.seek.load(std::sync::atomic::Ordering::Relaxed);
        if packed == ARTIFACT_HISTORY_NO_SEEK || self.group.is_some() {
            return None;
        }
        let (position, index) = ((packed >> 32) as usize, packed as u32);
        (index < self.initialized && self.slot(index).is_occupied()).then_some((position, index))
    }

    fn remember_seek(&self, position: usize, index: u32) {
        if let Ok(position) = u32::try_from(position) {
            self.seek.store((u64::from(position) << 32) | u64::from(index), std::sync::atomic::Ordering::Relaxed);
        }
    }

    fn forget_seek(&self) {
        self.seek.store(ARTIFACT_HISTORY_NO_SEEK, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn len(&self) -> usize {
        self.visible_bounds().2
    }

    /// 🪑️ How many more entries the ledger admits one reservation at a time — the preflight of an atomic batch of inserts.
    pub fn vacancies(&self) -> usize {
        if self.group.is_some() || self.reservation.is_some() || self.replacement_count!=0 {
            return 0;
        }
        let mut vacancies = 0;
        let mut free = self.free_head;
        while let Some(index) = free {
            let slot = self.slot(index);
            if slot.generation == u32::MAX {
                return vacancies;
            }
            vacancies += 1;
            free = slot.free_next;
        }
        let unused = self.allocated_slots().saturating_sub(self.initialized as usize);
        let next_page = if unused == 0 && self.page_count < u32::MAX { artifact_history_page_slots::<T>() } else { 0 };
        vacancies + unused + next_page
    }

    pub fn has_capacity(&self) -> bool {
        self.replacement_count==0 && self.group.is_none() && self.reservation.is_none() && (self.free_head.is_some_and(|index| self.slot(index).generation != u32::MAX) || (self.initialized as usize) < self.allocated_slots() || self.page_count < u32::MAX)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> ArtifactHistoryIter<'_, T> {
        let (front, back, remaining) = self.visible_bounds();
        ArtifactHistoryIter { ledger: self, front, back, remaining }
    }

    pub fn iter_mut(&mut self) -> ArtifactHistoryIterMut<'_, T> {
        assert!(self.group.is_none(), "mutable history iteration requires its staged group to be adopted or aborted");
        assert!(self.replacement_count==0,"mutable history iteration requires every original replacement to settle");
        assert!(self.read_count==0, "mutable history iteration requires every original retained read to return");
        ArtifactHistoryIterMut { ledger: self, front: self.head, back: self.tail, remaining: self.len, marker: std::marker::PhantomData }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.replacement_count == 0 && self.read_count == 0 && self.len == 0 && self.head.is_none() && self.tail.is_none() && self.reservation.is_none() && self.group.is_none()
    }

    /// 🪵️ Observes the original native backing after logical entry ownership has left.
    pub fn backing_is_empty(&self) -> bool { self.pages.is_none() && self.page_count == 0 }

    /// 📃️ Observes initialized original headers still retaining their slot visibility leases.
    pub fn retained_terminal_slot_count(&self) -> usize { self.initialized as usize }

    /// 👁️ Borrows the exact next slot visibility without copying or transferring its Arc.
    pub fn next_terminal_slot_visibility(&self) -> Result<Option<&std::sync::Arc<ArtifactGroupVisibility>>, ValueError> {
        if !self.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger still retains logical entry owners")); }
        if self.initialized == 0 { return Ok(None); }
        let slot = self.slot(self.initialized - 1);
        if slot.is_occupied() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "terminal history header still contains its original value")); }
        Ok(slot.visibility.as_ref())
    }

    /// 🫙️ Transfers at most one original empty slot's visibility after item and depth admission.
    pub fn retire_terminal_slot(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Option<std::sync::Arc<ArtifactGroupVisibility>>, semio_framework_value::retained_clone::RetainedCloneProgress), ValueError> {
        self.next_terminal_slot_visibility()?;
        if grant.maximum_items == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "history slot retirement requires one admitted item")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history slot retirement requires original owner depth")); }
        if self.initialized == 0 { return Ok((None, Default::default())); }
        let index = self.initialized - 1;
        let visibility = self.slot_mut(index).visibility.take();
        self.initialized = index;
        self.free_head = None;
        self.forget_seek();
        Ok((visibility, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    /// 🍕️ Declares the actual last slot slice, page shell, and separately owned empty directory.
    pub fn next_empty_page_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.terminal_is_empty() || self.initialized != 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history page retains original entry or visibility headers")); }
        if self.backing_is_empty() { return Ok(0); }
        let index = self.page_count.checked_sub(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history directory has no corresponding page"))? as usize;
        let mut chunk = self.pages.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page lost its original directory"))?;
        for _ in 0..index / ARTIFACT_HISTORY_DIRECTORY_PAGES { chunk = chunk.next.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page directory chain is incomplete"))?; }
        let offset = index % ARTIFACT_HISTORY_DIRECTORY_PAGES;
        if chunk.next.is_some() || usize::from(chunk.count) != offset + 1 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page directory differs from its actual last allocation")); }
        let page = chunk.pages[offset].as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page slot backing is absent"))?;
        let slots = std::alloc::Layout::array::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>(page.slots.len()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history slot backing layout overflow"))?.size();
        slots.checked_add(size_of::<ArtifactHistorySlotPage<T>>()).and_then(|bytes| bytes.checked_add(if offset == 0 { size_of::<ArtifactHistoryPageChunk<T>>() } else { 0 })).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history page whole release extent overflow"))
    }

    /// ♻️ Releases one real native page only after its visibility owners and full release grant.
    pub fn release_empty_page(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        let demand = self.next_empty_page_release_byte_demand()?;
        if self.backing_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history page release requires original owner depth")); }
        let index = self.page_count as usize - 1;
        let directory = index / ARTIFACT_HISTORY_DIRECTORY_PAGES;
        let offset = index % ARTIFACT_HISTORY_DIRECTORY_PAGES;
        let mut chunk = self.pages.as_mut().unwrap();
        for _ in 0..directory { chunk = chunk.next.as_mut().unwrap(); }
        drop(chunk.pages[offset].take());
        chunk.count -= 1;
        if offset == 0 {
            if directory == 0 { drop(self.pages.take()); }
            else {
                let mut previous = self.pages.as_mut().unwrap();
                for _ in 0..directory - 1 { previous = previous.next.as_mut().unwrap(); }
                drop(previous.next.take());
            }
        }
        self.page_count -= 1;
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand, ..empty };
        Ok(if self.backing_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

}

impl<T> Drop for ArtifactHistoryLedger<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "artifact history ledger reached Drop before every exact entry owner was retired");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.pages) };
    }
}

/// 🧬️ Rebuilds a fresh, independent ledger from every currently-visible entry (mirrors `Serialize`'s
/// own `read_group`-scoped view) — a pending, uncommitted group's staged entries clone away exactly
/// as they already serialize away, since a clone is just another reader. `try_from_preflighted` can
/// only fail when the page address space is exhausted, which an already-admitted source ledger never does.
impl<T: Clone> Clone for ArtifactHistoryLedger<T> {
    fn clone(&self) -> Self {
        Self::try_from_preflighted(self.iter().cloned().collect()).unwrap_or_else(|_| unreachable!("a source ledger's own entries never exhaust its page address space"))
    }
}

/// 📼️ `ToValue`/`FromValue` for `ArtifactHistoryLedger<T>` — serde's own `Serialize`/`Deserialize`
/// were dropped from this type (see the `Change` struct's docstring above); this is now the only
/// codec. Same admission path (`try_from_preflighted`) every other ledger construction uses.
impl<T: ToValue> ToValue for ArtifactHistoryLedger<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}

impl<T: FromValue> FromValue for ArtifactHistoryLedger<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let values = Vec::<T>::from_value(value)?;
        Self::try_from_preflighted(values).map_err(|rejected| ValueError::new(protocol::value::ValueRefusalKind::OwnershipLimit,format!("a history ledger within its page address space ({} entries rejected)", rejected.len())))
    }
}

pub struct ArtifactHistoryIter<'a, T> {
    ledger: &'a ArtifactHistoryLedger<T>,
    front: Option<u32>,
    back: Option<u32>,
    pub(super) remaining: usize,
}

impl<T> Clone for ArtifactHistoryIter<'_, T> {
    fn clone(&self) -> Self {
        Self { ledger: self.ledger, front: self.front, back: self.back, remaining: self.remaining }
    }
}

/// 💧️ `ToValue` for `ArtifactHistoryIter` — serde's own `Serialize` was dropped from this type
/// (see the `Change` struct's docstring above). This is the exact shape `ArtifactVcsRead` (below)
/// needs for each of its four group-visibility-consistent fields.
impl<T: ToValue> ToValue for ArtifactHistoryIter<'_, T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.clone().map(ToValue::to_value).collect())
    }
}

impl<'a, T> Iterator for ArtifactHistoryIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.front?;
        let slot = self.ledger.slot(index);
        self.front = slot.next;
        self.remaining -= 1;
        slot.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for ArtifactHistoryIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.back?;
        let slot = self.ledger.slot(index);
        self.back = slot.previous;
        self.remaining -= 1;
        slot.get()
    }
}

impl<T> ExactSizeIterator for ArtifactHistoryIter<'_, T> {}

pub struct ArtifactHistoryIterMut<'a, T> {
    ledger: *mut ArtifactHistoryLedger<T>,
    front: Option<u32>,
    back: Option<u32>,
    pub(super) remaining: usize,
    marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for ArtifactHistoryIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.front?;
        let slot = unsafe { (*self.ledger).slot_mut(index) };
        self.front = slot.next;
        self.remaining -= 1;
        slot.original.get_mut_unissued().map(|value| unsafe { &mut *(value as *mut T) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for ArtifactHistoryIterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.back?;
        let slot = unsafe { (*self.ledger).slot_mut(index) };
        self.back = slot.previous;
        self.remaining -= 1;
        slot.original.get_mut_unissued().map(|value| unsafe { &mut *(value as *mut T) })
    }
}

impl<T> ExactSizeIterator for ArtifactHistoryIterMut<'_, T> {}

impl<'a, T> IntoIterator for &'a ArtifactHistoryLedger<T> {
    type Item = &'a T;
    type IntoIter = ArtifactHistoryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ArtifactHistoryLedger<T> {
    type Item = &'a mut T;
    type IntoIter = ArtifactHistoryIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> std::ops::Index<usize> for ArtifactHistoryLedger<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("artifact history index outside the live deterministic order")
    }
}

impl<T> std::ops::IndexMut<usize> for ArtifactHistoryLedger<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("artifact history index outside the live deterministic order")
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ArtifactHistoryLedger<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for ArtifactHistoryLedger<T> {
    fn eq(&self, other: &Self) -> bool {
        let left = self.iter();
        let right = other.iter();
        left.len() == right.len() && left.zip(right).all(|(left, right)| left == right)
    }
}

impl<T: Eq> Eq for ArtifactHistoryLedger<T> {}

#[path="♻️retirement/🦀️.rs"]
mod intrinsic_retirement;

impl protocol::HistoryTransitionEvents for ArtifactHistoryLedger<protocol::MutationEnvelope>{fn event_count(&self)->usize{self.len()}fn event_at(&self,index:usize)->Option<&protocol::MutationEnvelope>{self.get(index)}}

#[path="⏪️selection/🦀️.rs"]
mod edit_selection;
pub use edit_selection::OriginalHistoryEditSelection;


#[path="replacements.rs"]
mod replacements;

#[cfg(test)]
mod original_row_admission_laws {
 use super::*;
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
 #[test]
 fn original_option_row_denials_preserve_the_donor_and_paid_success_closes_every_backing() {
  let law:serde_json::Value=serde_json::from_str(include_str!("row.json")).unwrap();
  for row in law["cases"].as_array().unwrap() {
   let mut ledger=ArtifactHistoryLedger::<i32>::empty();let demand=ledger.owned_row_admission_demands();let mut donor=Some(42);
   let mut grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:row["depth"].as_u64().unwrap()as usize,..Default::default()};
   if row["copy"].as_u64()<row["requiredCopy"].as_u64(){grant.maximum_copy_bytes=demand.copy_bytes-1}
   if row["capacity"].as_u64()<row["requiredCapacity"].as_u64(){grant.maximum_capacity_bytes=demand.capacity_bytes-1}
   let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ledger.admit_owned_row(&mut donor,grant));
   let progress=turn.result.unwrap();assert!(progress.fits(grant));let admitted=row["admitted"].as_bool().unwrap();assert_eq!(donor.is_none(),admitted);assert_eq!(turn.work,u64::from(admitted));
   assert_eq!(serde_json::to_value(ledger.iter().copied().collect::<Vec<_>>()).unwrap(),if admitted{serde_json::json!([42])}else{serde_json::json!([])});
   assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,0));
   if !admitted{assert_eq!(progress,RetainedCloneProgress::default());assert_eq!(donor,Some(42));}
   while ledger.pop().is_some(){}
   while ledger.retained_terminal_slot_count()>0{ledger.retire_terminal_slot(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).unwrap();}
   while !ledger.backing_is_empty(){let bytes=ledger.next_empty_page_release_byte_demand().unwrap();ledger.release_empty_page(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()}).unwrap();}
   assert!(ledger.terminal_is_empty());
  }
 }
}
