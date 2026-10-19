//! ⏳️ Granted history folding with cooperative decoding and exact cancellation retirement.

use super::*;
use semio_framework_value::{ErasedSnapshotRetirement, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{RetireOwned, admit_owned_retirement, owned_retirement_birth_bytes};
use std::mem::{ManuallyDrop,size_of,size_of_val};
use semio_framework_value::retirement::controlled::{ControlledRetirement, admit_typed_controlled_retirement};
use std::{collections::VecDeque, future::Future, ops::{Deref, DerefMut}, pin::Pin, sync::{Arc, atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering}}, task::{Context, Poll, Wake, Waker}};
use semio_framework_replication_history_index as index;

pub use semio_framework_replication_history_index::{HistoryFoldIndexRead,HistoryFoldIndexInsert,HistoryFoldIndex,HistoryFoldIndexIntoIter,HistoryFoldIndexIter,HistoryFoldSet,HistoryFoldSetIter,HistoryFoldSetIntoIter};
use index::HistoryFoldIndexMatch;

struct FoldRetirementQueue { locked:AtomicBool, values:std::cell::UnsafeCell<VecDeque<Box<dyn ErasedSnapshotRetirement>>> }
unsafe impl Send for FoldRetirementQueue {}
unsafe impl Sync for FoldRetirementQueue {}
impl FoldRetirementQueue {
    fn new()->Self{Self{locked:AtomicBool::new(false),values:std::cell::UnsafeCell::new(VecDeque::with_capacity(128))}}
    fn lock(&self)->FoldRetirementGuard<'_>{while self.locked.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).is_err(){std::hint::spin_loop();}FoldRetirementGuard(self)}
}
struct FoldRetirementGuard<'a>(&'a FoldRetirementQueue);
impl Deref for FoldRetirementGuard<'_>{type Target=VecDeque<Box<dyn ErasedSnapshotRetirement>>;fn deref(&self)->&Self::Target{unsafe{&*self.0.values.get()}}}
impl DerefMut for FoldRetirementGuard<'_>{fn deref_mut(&mut self)->&mut Self::Target{unsafe{&mut*self.0.values.get()}}}
impl Drop for FoldRetirementGuard<'_>{fn drop(&mut self){self.0.locked.store(false,Ordering::Release);}}

#[path = "🏗️birth/🦀️.rs"]
mod original_backing_birth;
pub use original_backing_birth::copy_history_trunk;

struct FoldControlState {
    cancelled: AtomicBool,
    completed: AtomicU64,
    page_bytes: AtomicUsize,
    pending_retirement_birth: AtomicUsize,
    pending_construction_capacity: AtomicUsize,
    pending_retirement_items: AtomicUsize,
    pending_original_copy: AtomicUsize,
    pending_original_depth: AtomicUsize,
    pending_original_release: AtomicUsize,
    retirement_capacity: AtomicUsize,
    retirement_items: AtomicUsize,
    admitted_retirement_items: AtomicUsize,
    admitted_retirement_capacity: AtomicUsize,
    original_copy: AtomicUsize,
    original_depth: AtomicUsize,
    original_release: AtomicUsize,
    admitted_original_copy: AtomicUsize,
    admitted_original_release: AtomicUsize,
    retirements: FoldRetirementQueue,
}

/// ⏱️ The shared pulse and retirement authority of one immutable-source fold.
#[derive(Clone)]
pub struct HistoryFoldControl(Arc<FoldControlState>);

impl HistoryFoldControl {
    /// 🌱️ Creates fixed retirement queue capacity without inspecting any history.
    pub fn new() -> Self { Self(Arc::new(FoldControlState { cancelled: AtomicBool::new(false), completed: AtomicU64::new(0), page_bytes: AtomicUsize::new(4096), pending_retirement_birth:AtomicUsize::new(0),pending_construction_capacity:AtomicUsize::new(0), pending_retirement_items:AtomicUsize::new(0),pending_original_copy:AtomicUsize::new(0),pending_original_depth:AtomicUsize::new(0),pending_original_release:AtomicUsize::new(0), retirement_capacity:AtomicUsize::new(0), retirement_items:AtomicUsize::new(0), admitted_retirement_items:AtomicUsize::new(0), admitted_retirement_capacity:AtomicUsize::new(0), original_copy:AtomicUsize::new(0),original_depth:AtomicUsize::new(0),original_release:AtomicUsize::new(0),admitted_original_copy:AtomicUsize::new(0),admitted_original_release:AtomicUsize::new(0),retirements: FoldRetirementQueue::new() })) }
    /// 📊️ Completed cooperative work units, excluding cancellation retirement.
    pub fn completed(&self) -> u64 { self.0.completed.load(Ordering::Relaxed) }
    fn original_grant(&self)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:self.0.retirement_items.load(Ordering::Relaxed),maximum_copy_bytes:self.0.original_copy.load(Ordering::Relaxed),maximum_capacity_bytes:self.0.retirement_capacity.load(Ordering::Relaxed),maximum_release_bytes:self.0.original_release.load(Ordering::Relaxed),maximum_depth:self.0.original_depth.load(Ordering::Relaxed)}}
    fn accept_original_progress(&self,progress:RetainedCloneProgress){self.0.retirement_items.fetch_sub(progress.copied_items,Ordering::Relaxed);self.0.retirement_capacity.fetch_sub(progress.retained_capacity_bytes,Ordering::Relaxed);self.0.original_copy.fetch_sub(progress.copied_bytes,Ordering::Relaxed);self.0.original_release.fetch_sub(progress.released_bytes,Ordering::Relaxed);self.0.admitted_original_release.fetch_add(progress.released_bytes,Ordering::Relaxed);self.0.admitted_retirement_items.fetch_add(progress.copied_items,Ordering::Relaxed);self.0.admitted_retirement_capacity.fetch_add(progress.retained_capacity_bytes,Ordering::Relaxed);self.0.admitted_original_copy.fetch_add(progress.copied_bytes,Ordering::Relaxed);}
    pub fn adopt_actor(&self,original:HistoryFoldOwned<String>)->SharedActorAdmission {let capacity=semio_framework_value::retirement::arc_custody::shared_retirement_allocation_bytes::<String>();self.0.pending_retirement_birth.fetch_add(capacity,Ordering::Relaxed);self.0.pending_retirement_items.fetch_add(1,Ordering::Relaxed);self.0.pending_original_copy.fetch_add(SharedActorAdmission::COPY_BYTES,Ordering::Relaxed);SharedActorAdmission{original:Some(original),control:self.clone(),capacity}}
    /// 🪪️ Borrows the original shared actor until its exact lease header is admitted.
    pub fn lease_actor<'a>(&self,original:&'a semio_framework_value::SharedUtf8)->SharedActorLease<'a>{self.0.pending_retirement_items.fetch_add(1,Ordering::Relaxed);self.0.pending_original_copy.fetch_add(SharedActorLease::COPY_BYTES,Ordering::Relaxed);SharedActorLease{original,control:self.clone()}}
    /// ♻️ Transfers an owned local to exact retirement when its coroutine scope ends.
    pub fn track<T: RetireOwned>(&self, value: T) -> Result<HistoryFoldAdmission<T>, (ValueError, T)> {
        if !T::controlled_retirement_supported() { return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner, "fold local has no controlled retirement authority"), value)); }
        self.0.pending_retirement_birth.fetch_add(owned_retirement_birth_bytes::<T>(), Ordering::Relaxed);
        self.0.pending_retirement_items.fetch_add(1, Ordering::Relaxed);
        Ok(HistoryFoldAdmission { value: Some(value), control: self.clone() })
    }
    /// ⏭️ One pulse before each bounded work unit; cancellation is checked before publication.
    pub async fn pulse(&self) -> Result<(), semio_framework_replication_error::ProtocolError> { FoldPulse { control: self, yielded: false }.await }
    fn page_bytes(&self) -> usize { self.0.page_bytes.load(Ordering::Relaxed).clamp(1, 4096) }
    fn retire<T: RetireOwned>(&self, value: T) {
        let bytes=owned_retirement_birth_bytes::<T>();
        self.0.retirement_capacity.try_update(Ordering::Relaxed,Ordering::Relaxed,|remaining|remaining.checked_sub(bytes)).expect("fold transfer requires admitted retirement capacity");
        self.0.retirement_items.try_update(Ordering::Relaxed,Ordering::Relaxed,|remaining|remaining.checked_sub(1)).expect("fold transfer requires admitted retirement item");
        let (owner,progress)=admit_owned_retirement(value,RetainedCloneGrant::one_capacity_turn(bytes,1)).unwrap_or_else(|(error,_)|panic!("admitted fold owner refused retirement: {error}"));
        let mut queue=self.0.retirements.lock();assert!(queue.len()<queue.capacity(),"fold retirement queue requires a reserved slot");queue.push_back(owner);
        self.0.admitted_retirement_capacity.fetch_add(progress.retained_capacity_bytes,Ordering::Relaxed);
        self.0.admitted_retirement_items.fetch_add(progress.copied_items,Ordering::Relaxed);
    }
    fn empty(&self) -> bool { self.0.retirements.lock().is_empty() }
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{let queue=self.0.retirements.lock();queue.front().map_or(Ok(RetirementDemand::default()),|owner|semio_framework_value::factory_ticket_demands(owner,copy))}
    fn close_one(&self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let mut queue=self.0.retirements.lock();let Some(owner)=queue.front_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if owner.terminal_is_empty(){let bytes=size_of_val(owner.as_ref());if grant.maximum_release_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"fold ticket release requires admitted depth"));}drop(queue.pop_front());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}));}
        let step=owner.close_step(grant)?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,owner.terminal_is_empty(),"fold queued owner")?;Ok(RetainedCloneStep::Progress(step.progress()))
    }

}

impl Default for HistoryFoldControl { fn default() -> Self { Self::new() } }

pub struct SharedActorAdmission {original:Option<HistoryFoldOwned<String>>,control:HistoryFoldControl,capacity:usize}
impl SharedActorAdmission{const COPY_BYTES:usize=size_of::<String>()+size_of::<semio_framework_value::SharedUtf8>();}
impl Future for SharedActorAdmission {
    type Output=semio_framework_value::SharedUtf8;
    fn poll(mut self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let grant=self.control.original_grant();let copy=Self::COPY_BYTES;
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<self.capacity{return Poll::Pending;}
        let original=self.original.take().expect("original actor admission retains paid String owner").take();
        let(owner,mut progress)=semio_framework_value::SharedUtf8::admit(original,grant).unwrap_or_else(|_|unreachable!("original actor admission was preflighted"));
        progress.copied_bytes+=copy;self.control.accept_original_progress(progress);Poll::Ready(owner)
    }
}
impl Drop for SharedActorAdmission {fn drop(&mut self){self.control.0.pending_retirement_birth.fetch_sub(self.capacity,Ordering::Relaxed);self.control.0.pending_retirement_items.fetch_sub(1,Ordering::Relaxed);self.control.0.pending_original_copy.fetch_sub(Self::COPY_BYTES,Ordering::Relaxed);}}

/// 🧾️ One original actor alias waits for the caller's independent copy and item authority.
pub struct SharedActorLease<'a>{original:&'a semio_framework_value::SharedUtf8,control:HistoryFoldControl}
impl SharedActorLease<'_>{const COPY_BYTES:usize=size_of::<semio_framework_value::SharedUtf8>();}
impl Future for SharedActorLease<'_>{type Output=semio_framework_value::SharedUtf8;fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{let grant=self.control.original_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<Self::COPY_BYTES{return Poll::Pending;}let(owner,mut progress)=self.original.admit_clone(grant).unwrap_or_else(|_|unreachable!("original shared actor lease preflighted"));progress.copied_bytes+=Self::COPY_BYTES;self.control.accept_original_progress(progress);Poll::Ready(owner)}}
impl Drop for SharedActorLease<'_>{fn drop(&mut self){self.control.0.pending_retirement_items.fetch_sub(1,Ordering::Relaxed);self.control.0.pending_original_copy.fetch_sub(Self::COPY_BYTES,Ordering::Relaxed);}}

/// 🧳️ Coroutine-local owned values enqueue their registered retirement instead of deep-dropping on cancellation.
pub struct HistoryFoldOwned<T: RetireOwned> { owner: Option<Box<ControlledRetirement<T>>>, control: HistoryFoldControl }
impl<T: RetireOwned> HistoryFoldOwned<T> { pub fn take(mut self) -> T { self.owner.as_mut().expect("fold local retains admitted frame").take_original().expect("fold local remains original") } }
impl<T: RetireOwned> Deref for HistoryFoldOwned<T> { type Target = T; fn deref(&self) -> &T { self.owner.as_ref().and_then(|owner| owner.original()).expect("fold local remains original") } }
impl<T: RetireOwned> DerefMut for HistoryFoldOwned<T> { fn deref_mut(&mut self) -> &mut T { self.owner.as_mut().and_then(|owner| owner.original_mut()).expect("fold local remains original") } }
impl<T: RetireOwned> Drop for HistoryFoldOwned<T> { fn drop(&mut self) { if let Some(owner) = self.owner.take() { let mut queue = self.control.0.retirements.lock(); assert!(queue.len() < queue.capacity(), "fold local requires a reserved queue slot"); queue.push_back(owner); } } }

/// 🎟️ Retains an original coroutine local until its real typed frame is admitted.
pub struct HistoryFoldAdmission<T: RetireOwned> { value: Option<T>, control: HistoryFoldControl }
impl<T: RetireOwned> Unpin for HistoryFoldAdmission<T> {}
impl<T: RetireOwned> Future for HistoryFoldAdmission<T> {
    type Output = HistoryFoldOwned<T>;
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let bytes = owned_retirement_birth_bytes::<T>();
        if self.control.0.retirement_items.load(Ordering::Relaxed) == 0 || self.control.0.retirement_capacity.load(Ordering::Relaxed) < bytes { return Poll::Pending; }
        let value = self.value.take().expect("fold admission retains original owner");
        let grant = RetainedCloneGrant::one_capacity_turn(bytes, 1);
        let (owner, progress) = match admit_typed_controlled_retirement(value, grant) { Ok(admitted) => admitted, Err((_, value)) => { self.value = Some(value); return Poll::Pending; } };
        self.control.0.retirement_capacity.fetch_sub(progress.retained_capacity_bytes, Ordering::Relaxed);
        self.control.0.retirement_items.fetch_sub(progress.copied_items, Ordering::Relaxed);
        self.control.0.admitted_retirement_capacity.fetch_add(progress.retained_capacity_bytes, Ordering::Relaxed);
        self.control.0.admitted_retirement_items.fetch_add(progress.copied_items, Ordering::Relaxed);
        self.control.0.pending_retirement_birth.fetch_sub(bytes, Ordering::Relaxed);
        self.control.0.pending_retirement_items.fetch_sub(1, Ordering::Relaxed);
        Poll::Ready(HistoryFoldOwned { owner: Some(owner), control: self.control.clone() })
    }
}
impl<T: RetireOwned> Drop for HistoryFoldAdmission<T> { fn drop(&mut self) { if let Some(value) = self.value.take() { self.control.0.pending_retirement_birth.fetch_sub(owned_retirement_birth_bytes::<T>(), Ordering::Relaxed); self.control.0.pending_retirement_items.fetch_sub(1, Ordering::Relaxed); self.control.retire(value); } } }

struct FoldPulse<'a> { control: &'a HistoryFoldControl, yielded: bool }
impl Future for FoldPulse<'_> {
    type Output = Result<(), semio_framework_replication_error::ProtocolError>;
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.control.0.cancelled.load(Ordering::Acquire) { return Poll::Ready(Err(fold_error("history fold cancelled"))); }
        if !self.yielded { self.yielded = true; return Poll::Pending; }
        self.control.0.completed.fetch_add(1, Ordering::Relaxed);
        Poll::Ready(Ok(()))
    }
}

struct FoldWake;
impl Wake for FoldWake { fn wake(self: Arc<Self>) {} }

/// 📊️ One granted step of a history fold, including the retirement required before handoff.
pub enum HistoryFoldJobStep<T = HistoryFold> { Pending { completed: u64, released_bytes: usize, progress: RetainedCloneProgress }, Ready { value: T, progress: RetainedCloneProgress }, Rejected { progress: RetainedCloneProgress } }

/// 🧮️ A pinned fold state machine; its immutable input is captured by the caller-owned runner.
pub struct HistoryFoldJob<'a, T: RetireOwned = HistoryFold> {
    control: ManuallyDrop<HistoryFoldControl>,
    native_binding: Option<(u64,Option<u64>)>,
    original_output: Option<ControlledRetirement<T>>,
    future: Option<Pin<Box<dyn Future<Output = Result<T, semio_framework_replication_error::ProtocolError>> + Send + 'a>>>,
    result: Option<Result<T, semio_framework_replication_error::ProtocolError>>,
    waker: Option<Waker>,
    future_finished: bool,
    control_closed: bool,
    completed_final: u64,
    terminal: bool,
}

impl<'a, T: RetireOwned> HistoryFoldJob<'a, T> {
    /// 🌱️ Captures immutable owners without scanning their contents.
    pub fn new<F: Future<Output = Result<T, semio_framework_replication_error::ProtocolError>> + Send + 'a>(runner: impl FnOnce(HistoryFoldControl) -> F) -> Self {
        let control = HistoryFoldControl::new();
        let future = Box::pin(runner(control.clone()));
        Self { control:ManuallyDrop::new(control), native_binding:None, original_output:None, future: Some(future), result: None, waker: Some(Waker::from(Arc::new(FoldWake))), future_finished:false,control_closed:false,completed_final:0,terminal: false }
    }
    /// 📊️ Completed semantic and decoding work units.
    pub fn completed(&self) -> u64 { if self.control_closed{self.completed_final}else{self.control.completed()} }
    /// ⏭️ Preserves independent work, copy, allocation, release and depth grants.
    pub fn step(&mut self,grant:RetainedCloneGrant,body:usize,should_yield:&mut impl FnMut()->bool)->Result<HistoryFoldJobStep<T>,ValueError>{
        if self.native_binding.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original history fold requires its receiving source"));}
        self.step_granted(grant,body,should_yield)
    }
    fn step_granted(&mut self,grant:RetainedCloneGrant,body:usize,should_yield:&mut impl FnMut()->bool)->Result<HistoryFoldJobStep<T>,ValueError>{
        let pending=|completed,progress:RetainedCloneProgress|HistoryFoldJobStep::Pending{completed,released_bytes:progress.released_bytes,progress};
        if grant.maximum_items==0||should_yield(){return Ok(pending(self.completed(),Default::default()));}
        if self.control_closed && matches!(self.result, Some(Err(_))) { return Ok(HistoryFoldJobStep::Rejected { progress: Default::default() }); }
        if !self.control_closed&&self.control.0.cancelled.load(Ordering::Acquire){let step=self.close_step(grant)?;return Ok(pending(self.completed(),step.progress()));}
        if self.future_finished||self.future.is_none()||!self.control.empty(){let step=self.cleanup_one(grant,false)?;let progress=step.progress();if self.control_closed{match self.result.as_ref(){Some(Ok(_))=>{let Some(Ok(value))=self.result.take()else{unreachable!()};self.terminal=true;return Ok(HistoryFoldJobStep::Ready { value, progress });},Some(Err(_))=>return Ok(HistoryFoldJobStep::Rejected { progress }),None=>{}}}return Ok(pending(self.completed(),progress));}
        if grant.maximum_copy_bytes==0{return Ok(pending(self.completed(),Default::default()));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"fold work requires admitted depth"));}
        let birth=self.control.0.pending_retirement_birth.load(Ordering::Relaxed);if grant.maximum_capacity_bytes<birth||grant.maximum_items<self.control.0.pending_retirement_items.load(Ordering::Relaxed){return Ok(pending(self.completed(),Default::default()));}
        self.control.0.page_bytes.store(body.clamp(1,4096),Ordering::Relaxed);
        self.control.0.original_copy.store(grant.maximum_copy_bytes,Ordering::Relaxed);self.control.0.original_depth.store(grant.maximum_depth,Ordering::Relaxed);self.control.0.original_release.store(grant.maximum_release_bytes,Ordering::Relaxed);self.control.0.admitted_original_copy.store(0,Ordering::Relaxed);self.control.0.admitted_original_release.store(0,Ordering::Relaxed);
        self.control.0.retirement_capacity.store(grant.maximum_capacity_bytes,Ordering::Relaxed);self.control.0.admitted_retirement_capacity.store(0,Ordering::Relaxed);self.control.0.retirement_items.store(grant.maximum_items,Ordering::Relaxed);self.control.0.admitted_retirement_items.store(0,Ordering::Relaxed);
        let mut items=0;while items<grant.maximum_items{if should_yield(){break;}self.control.0.retirement_items.store(grant.maximum_items-items,Ordering::Relaxed);let before=self.control.0.admitted_retirement_items.load(Ordering::Relaxed);let polled=self.future.as_mut().unwrap().as_mut().poll(&mut Context::from_waker(self.waker.as_ref().unwrap()));items+=(self.control.0.admitted_retirement_items.load(Ordering::Relaxed)-before).max(1);if let Poll::Ready(result)=polled{self.result=Some(result);self.future_finished=true;break;}if !self.control.empty(){break;}}
        let progress=RetainedCloneProgress{copied_items:items,copied_bytes:self.control.0.admitted_original_copy.load(Ordering::Relaxed),retained_capacity_bytes:self.control.0.admitted_retirement_capacity.load(Ordering::Relaxed),released_bytes:self.control.0.admitted_original_release.load(Ordering::Relaxed)};semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"fold cooperative work")?;Ok(pending(self.completed(),progress))
    }
    /// 🧮️ Publishes every waiting original frame item before atomic coroutine destruction.
    pub fn next_local_admission_item_demand(&self)->usize{if self.control_closed{0}else{self.control.0.pending_retirement_items.load(Ordering::Relaxed)}}
    /// 📏️ Explicit cold admission preserves all independent currencies.
    pub fn next_step_grant(&self,maximum_items:usize,logical_copy:usize)->Result<RetainedCloneGrant,ValueError>{
        let demand=self.next_demand(logical_copy)?;
        Ok(RetainedCloneGrant{maximum_items,maximum_copy_bytes:(if !self.control_closed&&!self.future_finished&&self.control.empty(){logical_copy}else{demand.copy_bytes.max(logical_copy)}).max(if self.control_closed||self.control.0.cancelled.load(Ordering::Acquire){0}else{self.control.0.pending_original_copy.load(Ordering::Relaxed)}),maximum_capacity_bytes:demand.capacity_bytes.checked_add(if self.control_closed||self.future_finished||self.control.0.cancelled.load(Ordering::Acquire)||!self.control.empty(){0}else{self.control.0.pending_construction_capacity.load(Ordering::Relaxed)}).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fold construction capacity overflow"))?,maximum_release_bytes:demand.release_bytes.max(if self.control_closed||self.control.0.cancelled.load(Ordering::Acquire){0}else{self.control.0.pending_original_release.load(Ordering::Relaxed)}),maximum_depth:demand.depth.max(1).max(if self.control_closed||self.control.0.cancelled.load(Ordering::Acquire){0}else{self.control.0.pending_original_depth.load(Ordering::Relaxed)})})
    }
    fn control_frame_bytes(&self)->usize{let layout=std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<FoldControlState>()).unwrap().0.pad_to_align();if Arc::strong_count(&self.control.0)==1{layout.size()}else{0}}
    fn cleanup_one(&mut self,grant:RetainedCloneGrant,cancelled:bool)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.control_closed {
            if self.result.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
            let Some(Err(error)) = self.result.as_mut() else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "closed fold retained an unhanded successful result")); };
            let step = semio_framework_replication_error::close_protocol_cause_one(error, grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) { drop(self.result.take()); }
            return Ok(step);
        }
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"fold closure requires admitted depth"));}
        let demand=self.next_demand(grant.maximum_copy_bytes)?;let release=demand.release_bytes;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<release||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        if self.control.empty()&&self.future.is_some()&&grant.maximum_items<self.control.0.pending_retirement_items.load(Ordering::Relaxed){return Ok(RetainedCloneStep::Progress(empty));}
        if !self.control.empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};return self.control.close_one(child);}
        if self.future.is_some(){self.control.0.retirement_capacity.store(grant.maximum_capacity_bytes,Ordering::Relaxed);self.control.0.admitted_retirement_capacity.store(0,Ordering::Relaxed);self.control.0.retirement_items.store(grant.maximum_items,Ordering::Relaxed);self.control.0.admitted_retirement_items.store(0,Ordering::Relaxed);drop(self.future.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:self.control.0.admitted_retirement_items.load(Ordering::Relaxed).max(1),retained_capacity_bytes:self.control.0.admitted_retirement_capacity.load(Ordering::Relaxed),released_bytes:release,..empty}));}
        if cancelled&&self.result.is_some(){let Some(result)=self.result.take()else{unreachable!()};match result{Ok(value)=>{match admit_owned_retirement(value,grant){Ok((owner,progress))=>{let mut queue=self.control.0.retirements.lock();assert!(queue.len()<queue.capacity(),"fold result retirement requires reserved slot");queue.push_back(owner);return Ok(RetainedCloneStep::Progress(progress));},Err((error,value))=>{self.result=Some(Ok(value));return Err(error);}}},Err(error)=>{self.result=Some(Err(error));let Some(Err(error))=self.result.as_mut()else{unreachable!()};let step=semio_framework_replication_error::close_protocol_cause_one(error,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){drop(self.result.take());}return Ok(RetainedCloneStep::Progress(step.progress()));}}}
        let mut queue=self.control.0.retirements.lock();if queue.capacity()!=0{drop(std::mem::take(&mut*queue));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..empty}));}drop(queue);
        if self.waker.is_some(){drop(self.waker.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..empty}));}
        self.completed_final=self.control.completed();unsafe{ManuallyDrop::drop(&mut self.control)};self.control_closed=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..empty}))
    }
    /// 🧊️ Runs the same cursor to completion for explicit cold callers; interactive owners grant individual turns.
    pub fn finish_cold(mut self) -> Result<T, semio_framework_replication_error::ProtocolError> {
        loop {
            match self.step(self.next_step_grant(64,4096).expect("registered fold demands"),4096, &mut || false).expect("registered history owners must retire within their grants") {
                HistoryFoldJobStep::Pending { .. } => {},
                HistoryFoldJobStep::Ready { value, .. } => return Ok(value),
                HistoryFoldJobStep::Rejected { .. } => return Err(self.take_rejection().expect("cold fold rejection retains its original error")),
            }
        }
    }

    /// 🔎️ Borrows the original rejected result without copying or disposing it.
    pub fn rejection(&self) -> Option<&semio_framework_replication_error::ProtocolError> { match self.result.as_ref() { Some(Err(error)) => Some(error), _ => None } }

    /// 📤️ Transfers the original closed result into an explicit cold caller.
    pub fn take_rejection(&mut self) -> Option<semio_framework_replication_error::ProtocolError> {
        if !self.control_closed || !matches!(self.result, Some(Err(_))) { return None; }
        let Some(Err(error)) = self.result.take() else { unreachable!() };
        self.terminal = true;
        Some(error)
    }

    /// 🛑️ Cancels without executing further fold work; tracked locals transfer to exact retirement.
    pub fn request_cancel(&mut self){if !self.control_closed{self.control.0.cancelled.store(true,Ordering::Release);}}

}

impl<T:RetireOwned> ErasedSnapshotRetirement for HistoryFoldJob<'_,T>{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.control_closed{return self.result.as_ref().map_or(Ok(Default::default()),|result|match result{Err(error)=>crate::protocol_cause_retirement_demand(error),Ok(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result"))});}
        if !self.control.empty(){let mut demand=self.control.demands(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"fold nested retirement depth overflow"))?;return Ok(demand);}
        if let Some(future)=self.future.as_ref(){return Ok(RetirementDemand{capacity_bytes:self.control.0.pending_retirement_birth.load(Ordering::Relaxed),release_bytes:size_of_val(future.as_ref().get_ref()),depth:1,..Default::default()});}
        if self.control.0.cancelled.load(Ordering::Acquire){if let Some(result)=self.result.as_ref(){return match result{Ok(_)=>Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()}),Err(error)=>crate::protocol_cause_retirement_demand(error)};}}
        let queue=self.control.0.retirements.lock();let release_bytes=if queue.capacity()!=0{queue.capacity()*size_of::<Box<dyn ErasedSnapshotRetirement>>()}else if self.waker.is_some(){size_of::<[usize;2]>()}else{self.control_frame_bytes()};Ok(RetirementDemand{release_bytes,depth:1,..Default::default()})
    }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.control_closed{self.result.as_ref().map_or(Ok(0),|result|match result{Err(error)=>semio_framework_replication_error::protocol_cause_retirement_demand(error).map(|demand|demand.copy_bytes),Ok(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result"))})}else if self.control.empty()&&self.future.is_none()&&self.control.0.cancelled.load(Ordering::Acquire){match self.result.as_ref(){Some(Err(error))=>semio_framework_replication_error::protocol_cause_retirement_demand(error).map(|demand|demand.copy_bytes),_=>Ok(0)}}else{self.control.demands(0).map(|demand|demand.copy_bytes)}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if self.control_closed{return self.result.as_ref().map_or(Ok(0),|result|match result{Err(error)=>semio_framework_replication_error::protocol_cause_retirement_demand(error).map(|demand|demand.capacity_bytes),Ok(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result"))});}if !self.control.empty(){return Ok(self.control.demands(copy)?.capacity_bytes);}if self.future.is_some(){return Ok(self.control.0.pending_retirement_birth.load(Ordering::Relaxed));}Ok(if self.control.0.cancelled.load(Ordering::Acquire)&&matches!(self.result,Some(Ok(_))){owned_retirement_birth_bytes::<T>()}else{0})}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.control_closed{return self.result.as_ref().map_or(Ok(0), |result| match result { Err(error) => semio_framework_replication_error::protocol_cause_retirement_demand(error).map(|demand|demand.release_bytes), Ok(_) => Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result")) });}if !self.control.empty(){return Ok(self.control.demands(0)?.release_bytes);}if let Some(future)=self.future.as_ref(){return Ok(size_of_val(future.as_ref().get_ref()));}if self.control.0.cancelled.load(Ordering::Acquire){if let Some(result)=self.result.as_ref(){return match result{Ok(_)=>Ok(0),Err(error)=>semio_framework_replication_error::protocol_cause_retirement_demand(error).map(|demand|demand.release_bytes)};}}let queue=self.control.0.retirements.lock();if queue.capacity()!=0{return Ok(queue.capacity()*size_of::<Box<dyn ErasedSnapshotRetirement>>());}drop(queue);if self.waker.is_some(){return Ok(size_of::<[usize;2]>());}Ok(self.control_frame_bytes())}
    fn next_depth_demand(&self)->Result<usize,ValueError>{if self.control_closed{return self.result.as_ref().map_or(Ok(0),|result|match result{Err(error)=>semio_framework_replication_error::protocol_cause_retirement_demand(error).map(|demand|demand.depth),Ok(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result"))});}let depth=if self.control.empty(){0}else{self.control.demands(0)?.depth};depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"fold nested retirement depth overflow"))}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.request_cancel();let step=self.cleanup_one(grant,true)?;if self.control_closed&&matches!(step,RetainedCloneStep::Complete(_)){self.terminal=true;}semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"fold job closure")}
    fn terminal_is_empty(&self)->bool{self.terminal&&self.original_output.is_none()&&self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none()}
}
impl<T: RetireOwned> Drop for HistoryFoldJob<'_, T> { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "history fold job released before exact handoff or retirement"); } }

/// 📝️ Copies verified UTF-8 in fixed pages, yielding before each page allocation or write.
pub async fn copy_history_text(text: &str, control: &HistoryFoldControl) -> Result<String, semio_framework_replication_error::ProtocolError> { copy_history_text_parts(&[text], control).await }

/// 🪪️ Constructs the canonical generated operation identity from borrowed edit text and stack-only decimal digits.
pub async fn copy_history_operation_identity(edit:&str,index:u64,control:&HistoryFoldControl)->Result<String,semio_framework_replication_error::ProtocolError>{
    let mut digits=[0;20];let mut value=index;let mut start=20;
    loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}
    copy_history_text_parts(&[edit,"#",std::str::from_utf8(&digits[start..]).expect("operation index is ASCII")],control).await
}

/// ⚠️ Builds the canonical history-fold refusal text in admitted original storage.
pub async fn history_fold_refusal_controlled(detail:&str,control:&HistoryFoldControl)->Result<semio_framework_replication_error::ProtocolError,semio_framework_replication_error::ProtocolError>{
    original_fold_refusal("history fold",0,&[FoldErrorPart::Text(detail)],control).await
}

enum FoldErrorPart<'a>{Text(&'a str),Unsigned(u64)}

async fn original_fold_refusal(what:&'static str,offset:usize,parts:&[FoldErrorPart<'_>],control:&HistoryFoldControl)->Result<semio_framework_replication_error::ProtocolError,semio_framework_replication_error::ProtocolError>{
    let mut length=0usize;
    for part in parts{
        let count=match part{FoldErrorPart::Text(text)=>text.len(),FoldErrorPart::Unsigned(value)=>{let mut value=*value;let mut count=1;while value>=10{count+=1;value/=10;}count}};
        length=length.checked_add(count).ok_or(semio_framework_replication_error::ProtocolError::LimitExceeded("history refusal text"))?;
    }
    let mut detail=control.text(length).await?;
    for part in parts{
        let mut digits=[0;20];let text=match part{
            FoldErrorPart::Text(text)=>*text,
            FoldErrorPart::Unsigned(value)=>{let mut value=*value;let mut start=20;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}std::str::from_utf8(&digits[start..]).expect("decimal digits are valid UTF8")}
        };
        let mut position=0;control.copy_text_into(&mut detail,text.as_bytes(),&mut position,text.len()).await?;
    }
    Ok(semio_framework_replication_error::ProtocolError::Malformed{what,offset:offset as u64,detail:detail.take()})
}

async fn original_fold_offset(value:Option<usize>,offset:usize,detail:&str,control:&HistoryFoldControl)->Result<usize,semio_framework_replication_error::ProtocolError>{
    match value{Some(value)=>Ok(value),None=>Err(original_fold_refusal("history transition",offset,&[FoldErrorPart::Text(detail)],control).await?)}
}

async fn original_fold_required(value:Option<&usize>,parts:&[FoldErrorPart<'_>],control:&HistoryFoldControl)->Result<usize,semio_framework_replication_error::ProtocolError>{
    match value{Some(value)=>Ok(*value),None=>Err(original_fold_refusal("history fold",0,parts,control).await?)}
}

pub async fn copy_history_text_parts(parts: &[&str], control: &HistoryFoldControl) -> Result<String, semio_framework_replication_error::ProtocolError> {
    let length=parts.iter().try_fold(0usize,|length,part|length.checked_add(part.len())).ok_or(semio_framework_replication_error::ProtocolError::LimitExceeded("history text"))?;
    let mut copied=control.text(length).await?;
    for part in parts{let mut position=0;control.copy_text_into(&mut copied,part.as_bytes(),&mut position,part.len()).await?;}
    Ok(copied.take())
}
async fn read_fold_string(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<String, semio_framework_replication_error::ProtocolError> {
    let count=control.read_varint(bytes,pos).await?;
    let length=original_fold_offset(usize::try_from(count).ok(),*pos,"string length exceeds address space",control).await?;
    let end=original_fold_offset(pos.checked_add(length).filter(|end|*end<=bytes.len()),*pos,"truncated string",control).await?;
    let mut text=control.text(length).await?;
    control.copy_text_into(&mut text,bytes,pos,end).await?;
    Ok(text.take())
}

async fn read_fold_hlc(bytes:&[u8],position:&mut usize,control:&HistoryFoldControl)->Result<semio_framework_replication_ids::HybridLogicalTimestamp,semio_framework_replication_error::ProtocolError>{
    let actor=control.read_varint(bytes,position).await?;
    let physical_ms=control.read_varint(bytes,position).await?;
    let logical=control.read_varint(bytes,position).await?;
    Ok(semio_framework_replication_ids::HybridLogicalTimestamp{actor,physical_ms,logical})
}
async fn read_fold_optional(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Option<String>, semio_framework_replication_error::ProtocolError> {
    control.pulse().await?;
    match control.read_byte(bytes, pos).await? { 0 => Ok(None), 1 => read_fold_string(bytes, pos, control).await.map(Some), other => Err(original_fold_refusal("history transition",*pos,&[FoldErrorPart::Text("invalid option tag "),FoldErrorPart::Unsigned(other as u64)],control).await?) }
}

async fn read_fold_ids(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Vec<MutationId>, semio_framework_replication_error::ProtocolError> {
    control.pulse().await?;
    let count = control.read_varint(bytes, pos).await?;
    if count > bytes.len().saturating_sub(*pos) as u64 { return Err(original_fold_refusal("history transition",*pos,&[FoldErrorPart::Text("id count exceeds payload")],control).await?); }
    let mut ids = control.vector(count as usize).await?;
    for _ in 0..count { ids.push(MutationId(read_fold_string(bytes, pos, control).await?)); }
    Ok(ids.take())
}

async fn read_fold_payload(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> {
    control.pulse().await?;
    let length = control.read_varint(bytes, pos).await?;
    if length > SUPERSEDE_PAYLOAD_MAX_BYTES as u64 { return Err(original_fold_refusal("history transition",*pos,&[FoldErrorPart::Text("supersede payload exceeds "),FoldErrorPart::Unsigned(SUPERSEDE_PAYLOAD_MAX_BYTES as u64),FoldErrorPart::Text(" bytes")],control).await?); }
    let end = original_fold_offset(pos.checked_add(length as usize).filter(|end| *end <= bytes.len()),*pos,"truncated replacement payload",control).await?;
    let mut payload = control.vector(length as usize).await?;
    control.copy_bytes_into(&mut payload,bytes,pos,end).await?;
    Ok(payload.take())
}

/// 🔀️ Decodes every transition field and repeated member cooperatively; malformed and cancelled partial owners retire.
pub async fn decode_history_transition_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<HistoryTransition, semio_framework_replication_error::ProtocolError> {
    let mut pos = 0;
    control.pulse().await?;
    let transition = match control.read_varint(bytes, &mut pos).await? {
        0 => HistoryTransition::Revert { mutation_ids: read_fold_ids(bytes, &mut pos, control).await? },
        1 => HistoryTransition::Reinstate { mutation_ids: read_fold_ids(bytes, &mut pos, control).await? },
        2 => {
            let checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let parent_id = control.track(read_fold_optional(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let change_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let mutation_ids = control.track(read_fold_ids(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let description = control.track(read_fold_optional(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let saved_at = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let count = control.read_varint(bytes, &mut pos).await?;
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("author count exceeds payload")],control).await?); }
            let mut authors = control.vector(count as usize).await?;
            for _ in 0..count {
                let id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                let name = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                let avatar = read_fold_optional(bytes, &mut pos, control).await?;
                authors.push(TransitionAuthor { id: id.take(), name: name.take(), avatar });
            }
            let message = control.track(read_fold_optional(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let timestamp = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let line_id = read_fold_optional(bytes, &mut pos, control).await?;
            HistoryTransition::Commit(TransitionCheckpoint { checkpoint_id: checkpoint_id.take(), parent_id: parent_id.take(), change_id: change_id.take(), mutation_ids: mutation_ids.take(), description: description.take(), saved_at: saved_at.take(), authors: authors.take(), message: message.take(), timestamp: timestamp.take(), line_id })
        }
        3 => {
            let alternative_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let name = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let checkpoint_id = read_fold_string(bytes, &mut pos, control).await?;
            HistoryTransition::Branch { alternative_id: alternative_id.take(), name: name.take(), checkpoint_id }
        }
        4 => { let checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; let alternative_id = read_fold_optional(bytes, &mut pos, control).await?; HistoryTransition::Checkout { checkpoint_id: checkpoint_id.take(), alternative_id } }
        5 => {
            let checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let pinned_checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let count = control.read_varint(bytes, &mut pos).await?;
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("pin count exceeds payload")],control).await?); }
            let mut pins = control.vector(count as usize).await?;
            for _ in 0..count { let child_uri = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; let checkpoint_id = read_fold_string(bytes, &mut pos, control).await?; pins.push(TransitionPin { child_uri: child_uri.take(), checkpoint_id }); }
            HistoryTransition::Repin { checkpoint_id: checkpoint_id.take(), pinned_checkpoint_id: pinned_checkpoint_id.take(), pins: pins.take() }
        }
        6 => {
            let scope = control.track(read_fold_optional(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            if scope.as_ref().is_some_and(|scope| scope.len() > SUPERSEDE_SCOPE_MAX_BYTES) { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("supersede scope exceeds "),FoldErrorPart::Unsigned(SUPERSEDE_SCOPE_MAX_BYTES as u64),FoldErrorPart::Text(" bytes")],control).await?); }
            let count = control.read_varint(bytes, &mut pos).await?;
            if count == 0 { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("supersede names no input")],control).await?); }
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("input count exceeds payload")],control).await?); }
            let mut inputs = control.vector(count as usize).await?;
            let mut seen = control.track(HistoryFoldSet::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            for _ in 0..count {
                let target = control.track(MutationId(read_fold_string(bytes, &mut pos, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                if !seen.insert_original_text(copy_history_text(&target.0, control).await?, control).await? { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("supersede repeats target "),FoldErrorPart::Text(&target.0)],control).await?); }
                let replacement = match control.read_byte(bytes, &mut pos).await? {
                    0 => { let schema = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; let payload = read_fold_payload(bytes, &mut pos, control).await?; InputReplacement::Input { schema: schema.take(), payload } }
                    1 => InputReplacement::Withdrawn,
                    tag => return Err(original_fold_refusal("history transition",pos-1,&[FoldErrorPart::Text("invalid replacement tag "),FoldErrorPart::Unsigned(tag as u64)],control).await?),
                };
                inputs.push(SupersededInput { target: target.take(), replacement });
            }
            HistoryTransition::Supersede(TransitionSupersede { scope: scope.take(), inputs: inputs.take() })
        }
        tag => return Err(original_fold_refusal("history transition",0,&[FoldErrorPart::Text("unknown transition tag "),FoldErrorPart::Unsigned(tag as u64)],control).await?),
    };
    let transition = control.track(transition).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if pos != bytes.len() { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("trailing bytes")],control).await?); }
    Ok(transition.take())
}

async fn owned_positions(ids: &[MutationId], owners: &HistoryFoldIndex<String, usize>, control: &HistoryFoldControl) -> Result<Vec<usize>, semio_framework_replication_error::ProtocolError> {
    let mut positions = control.vector(ids.len()).await?;
    let mut seen = control.track(HistoryFoldSet::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for id in ids { control.pulse().await?; let position = original_fold_required(control.get_text(owners, &id.0).await?,&[FoldErrorPart::Text("transition references unknown operation "),FoldErrorPart::Text(&id.0)],control).await?; if seen.insert_original_copy(position, control).await? { positions.push(position); } }
    Ok(positions.take())
}

/// 🧮️ The shared semantic fold, cooperatively ordered and indexed without whole-history collect, sort or retain.
pub async fn fold_history_for_controlled(document_id: &ArtifactId, edits: &[FoldEdit], transitions: &[semio_framework_replication_causal::MutationEnvelope], excluded: &HistoryFoldIndex<String, ()>, head: &ViewerHead, control: &HistoryFoldControl) -> Result<HistoryFold, semio_framework_replication_error::ProtocolError> {
    let mut owners = control.track(HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut identities = control.track(HistoryFoldSet::<String>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut events = control.track(HistoryFoldIndex::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for (index, edit) in edits.iter().enumerate() {
        let id = control.track(copy_history_text(&edit.id, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        if !identities.insert_original_text(copy_history_text(&id, control).await?, control).await? { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("history repeats an edit")],control).await?); }
        for mutation in &edit.mutation_ids {
            if control.get_text(&owners, &mutation.0).await?.is_some() { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("history repeats operation "),FoldErrorPart::Text(&mutation.0)],control).await?); }
            control.insert_text(&mut owners, copy_history_text(&mutation.0, control).await?, index).await?;
        }
        control.insert_ordered_text(&mut events, (edit.timestamp.cmp_key(), id.take()), (false, index)).await?;
    }
    for (index, event) in transitions.iter().enumerate() {
        let id = control.track(copy_history_text(&event.mutation_id.0, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        if !identities.insert_original_text(copy_history_text(&id, control).await?, control).await? { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("history repeats transition "),FoldErrorPart::Text(&id.as_str())],control).await?); }
        if event.diff.schema.0 != HISTORY_TRANSITION_SCHEMA { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text(&id.as_str()),FoldErrorPart::Text(" is not a history transition")],control).await?); }
        control.insert_ordered_text(&mut events, (event.timestamp.cmp_key(), id.take()), (true, index)).await?;
    }
    control.pulse().await?;
    let applied = control.vector(edits.len()).await?;
    let redo_output = control.vector(edits.len()).await?;
    let refused = control.vector(transitions.len()).await?;
    let changes = control.vector(transitions.len()).await?;
    let checkpoints = control.vector(transitions.len()).await?;
    let alternatives = control.vector(transitions.len().checked_add(1).ok_or(semio_framework_replication_error::ProtocolError::LimitExceeded("history alternatives"))?).await?;
    let trunk = control.track(copy_history_trunk(document_id, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut fold = control.track(HistoryFold { trunk: trunk.take(), applied: applied.take(), redo: redo_output.take(), refused: refused.take(), changes: changes.take(), checkpoints: checkpoints.take(), alternatives: alternatives.take(), ..HistoryFold::default() }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut trunk_chain = control.vector::<String>(transitions.len()).await?;
    let mut active = control.track(HistoryFoldSet::<usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut redo = control.track(HistoryFoldIndex::<u64, usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut redo_lookup = control.track(HistoryFoldIndex::<usize, u64>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut redo_actors = control.track(HistoryFoldIndex::<Option<semio_framework_value::SharedUtf8>, HistoryFoldSet<u64>>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut renamed = control.track(HistoryFoldIndex::<String, String>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut checkpoint_lookup = control.track(HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut change_lookup = control.track(HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut alternative_lookup = control.track(HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut edit_lookup = control.track(HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut serial = 0;
    for (index, edit) in edits.iter().enumerate() { control.insert_text(&mut edit_lookup, copy_history_text(&edit.id, control).await?, index).await?; }
    loop {
        let event=control.pop_first(&mut events).await?;
        let Some((key,(structural,index)))=event.take()else{break};
        let _key = control.track(key).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        control.pulse().await?;
        if !structural {
            let edit = &edits[index];
            if control.get_text(excluded, &edit.id).await?.is_some() { continue; }
            active.insert_original_copy(index, control).await?;
            loop {
                let Some(orders)=control.get_actor_mut(&mut redo_actors,&edit.actor).await?else{break};
                let Some(position)=orders.pop_first_original_copy(control).await?else{break};
                control.pulse().await?;
                if let Some(index)=control.remove_copy(&mut redo,&position).await?{control.remove_copy(&mut redo_lookup,&index).await?;}
            }
            continue;
        }
        let event = &transitions[index];
        let mut transition = control.track(decode_history_transition_controlled(&event.diff.payload, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        let reinstate = matches!(&*transition, HistoryTransition::Reinstate { .. });
        match &mut *transition {
            HistoryTransition::Revert { mutation_ids } | HistoryTransition::Reinstate { mutation_ids } => {
                let positions = control.track(owned_positions(mutation_ids, &owners, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                let mut foreign = false;
                for position in positions.iter() { control.pulse().await?; foreign |= edits[*position].actor.as_deref().is_some_and(|actor| actor != event.actor.0.as_ref()); }
                if foreign { fold.refused.push(copy_history_text(&event.mutation_id.0, control).await?); continue; }
                for position in positions.iter().copied() {
                    control.pulse().await?;
                    if reinstate {
                        if let Some(order)=control.remove_copy(&mut redo_lookup,&position).await?{
                            control.remove_copy(&mut redo,&order).await?;
                            if let Some(orders)=control.get_actor_mut(&mut redo_actors,&edits[position].actor).await?{orders.remove_original_copy(&order,control).await?;}
                            active.insert_original_copy(position,control).await?;
                        }
                    } else if active.remove_original_copy(&position,control).await? {
                        control.insert_copy(&mut redo, serial, position).await?;
                        control.insert_copy(&mut redo_lookup, position, serial).await?;
                        let actor = match &edits[position].actor { Some(actor) => Some(control.lease_actor(actor).await), None => None };
                        control.get_or_insert_actor(&mut redo_actors, actor, HistoryFoldSet::new()).await?;
                        control.get_actor_mut(&mut redo_actors,&edits[position].actor).await?.expect("original actor row was just admitted").insert_original_copy(serial, control).await?;
                        serial += 1;
                    }
                }
            }
            HistoryTransition::Commit(checkpoint) => {
                let parent = match &checkpoint.parent_id { Some(parent) => Some(original_fold_required(control.get_text(&checkpoint_lookup, parent).await?,&[FoldErrorPart::Text("checkpoint "),FoldErrorPart::Text(&checkpoint.checkpoint_id),FoldErrorPart::Text(" names unknown parent "),FoldErrorPart::Text(&parent)],control).await?), None => None };
                let count = parent.map_or(0, |parent| fold.checkpoints[parent].change_ids.len());
                let mut change_ids = control.vector(count+1).await?;
                if let Some(parent) = parent { for change in &fold.checkpoints[parent].change_ids { change_ids.push(copy_history_text(change, control).await?); } }
                change_ids.push(copy_history_text(&checkpoint.change_id, control).await?);
                let positions = control.track(owned_positions(&checkpoint.mutation_ids, &owners, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                let mut edit_ids = control.vector(positions.len()).await?;
                for position in positions.iter() { edit_ids.push(copy_history_text(&edits[*position].id, control).await?); }
                control.get_or_insert_text(&mut change_lookup, copy_history_text(&checkpoint.change_id, control).await?, fold.changes.len()).await?;
                control.get_or_insert_text(&mut checkpoint_lookup, copy_history_text(&checkpoint.checkpoint_id, control).await?, fold.checkpoints.len()).await?;
                let checkpoint_id = control.track(copy_history_text(&checkpoint.checkpoint_id, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                let line = checkpoint.line_id.as_ref().filter(|line| *line != &fold.trunk);
                match line {
                    Some(line) => { let index = original_fold_required(control.get_text(&alternative_lookup, line).await?,&[FoldErrorPart::Text("commit "),FoldErrorPart::Text(&checkpoint.checkpoint_id),FoldErrorPart::Text(" names unknown alternative "),FoldErrorPart::Text(&line)],control).await?; control.push_vec(&mut fold.alternatives[index].checkpoint_ids, copy_history_text(&checkpoint_id, control).await?).await?; }
                    None => trunk_chain.push(copy_history_text(&checkpoint_id, control).await?),
                }
                fold.changes.push(FoldChange { id: std::mem::take(&mut checkpoint.change_id), edit_ids: edit_ids.take(), description: checkpoint.description.take(), saved_at: std::mem::take(&mut checkpoint.saved_at) });
                fold.checkpoints.push(FoldCheckpoint { id: std::mem::take(&mut checkpoint.checkpoint_id), change_ids: change_ids.take(), parent_id: checkpoint.parent_id.take(), authors: std::mem::take(&mut checkpoint.authors), message: checkpoint.message.take(), timestamp: std::mem::take(&mut checkpoint.timestamp), pins: Vec::new() });
            }
            HistoryTransition::Branch { alternative_id, name, checkpoint_id } => {
                if *alternative_id == fold.trunk { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("branch claims the trunk alternative "),FoldErrorPart::Text(&alternative_id)],control).await?); }
                if control.get_text(&checkpoint_lookup, checkpoint_id).await?.is_none() { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("branch names unknown checkpoint "),FoldErrorPart::Text(&checkpoint_id)],control).await?); }
                control.get_or_insert_text(&mut alternative_lookup, copy_history_text(alternative_id, control).await?, fold.alternatives.len()).await?;
                let mut checkpoint_ids = control.vector(1).await?;
                checkpoint_ids.push(std::mem::take(checkpoint_id));
                fold.alternatives.push(FoldAlternative { id: std::mem::take(alternative_id), name: std::mem::take(name), checkpoint_ids: checkpoint_ids.take() });
            }
            HistoryTransition::Checkout { checkpoint_id, .. } => { if control.get_text(&checkpoint_lookup, checkpoint_id).await?.is_none() { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("checkout names unknown checkpoint "),FoldErrorPart::Text(&checkpoint_id)],control).await?); } }
            HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins } => {
                let index = original_fold_required(control.get_text(&checkpoint_lookup, checkpoint_id).await?,&[FoldErrorPart::Text("repin names unknown checkpoint "),FoldErrorPart::Text(&checkpoint_id)],control).await?;
                control.track(std::mem::replace(&mut fold.checkpoints[index].id, copy_history_text(pinned_checkpoint_id, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                control.track(std::mem::replace(&mut fold.checkpoints[index].pins, std::mem::take(pins))).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                control.remove_text(&mut checkpoint_lookup,checkpoint_id).await?;
                control.insert_text(&mut checkpoint_lookup, copy_history_text(pinned_checkpoint_id, control).await?, index).await?;
                for alternative in &mut fold.alternatives { for id in &mut alternative.checkpoint_ids { control.pulse().await?; if *id == *checkpoint_id { control.track(std::mem::replace(id, copy_history_text(pinned_checkpoint_id, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; } } }
                for id in trunk_chain.iter_mut() { control.pulse().await?; if *id == *checkpoint_id { control.track(std::mem::replace(id, copy_history_text(pinned_checkpoint_id, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; } }
                control.insert_text(&mut renamed, std::mem::take(checkpoint_id), std::mem::take(pinned_checkpoint_id)).await?;
            }
            HistoryTransition::Supersede(supersede) => {
                for input in &supersede.inputs { control.pulse().await?; if control.get_text(&owners, &input.target.0).await?.is_none() { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("transition references unknown operation "),FoldErrorPart::Text(&input.target.0)],control).await?); } }
                if supersede.scope.as_ref().is_none_or(|scope| *scope == head.line_id) {
                    for input in &mut supersede.inputs {
                        control.pulse().await?;
                        let actor = control.track(control.lease_actor(&event.actor.0).await).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                        let transition_id = control.track(copy_history_text(&event.mutation_id.0, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                        let scope = match &supersede.scope { Some(scope) => Some(copy_history_text(scope, control).await?), None => None };
                        let target = MutationId(std::mem::take(&mut input.target.0));
                        let replacement = std::mem::replace(&mut input.replacement, InputReplacement::Withdrawn);
                        control.insert_mutation(&mut fold.supersessions, target, EffectiveSupersession { transition_id: transition_id.take(), actor: actor.take(), timestamp: event.timestamp, scope, replacement }).await?;
                    }
                }
            }
        }
    }
    let on_trunk = head.line_id == fold.trunk;
    let source = if on_trunk { &*trunk_chain } else { let index = original_fold_required(control.get_text(&alternative_lookup, &head.line_id).await?,&[FoldErrorPart::Text("viewer head names unknown alternative "),FoldErrorPart::Text(&head.line_id)],control).await?; &fold.alternatives[index].checkpoint_ids };
    let mut chain = control.vector(source.len()).await?;
    for id in source { chain.push(copy_history_text(id, control).await?); }
    let mut viewed = control.track(match &head.checkpoint_id { Some(id) => Some(copy_history_text(id, control).await?), None => None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if let Some(id) = viewed.as_mut() {
        let mut guard = 0;
        while let Some(next) = control.get_text(&renamed, id).await? { control.pulse().await?; control.track(std::mem::replace(id, copy_history_text(next, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; guard += 1; if guard == 64 { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("repin cycle")],control).await?); } }
        let mut known = false;
        for candidate in chain.iter() { control.pulse().await?; known |= candidate == id; }
        if !known { return Err(original_fold_refusal("history fold",0,&[FoldErrorPart::Text("viewer head names unknown checkpoint "),FoldErrorPart::Text(&id)],control).await?); }
    }
    let at_tip = viewed.is_none();
    let checkpoint = match viewed.take() { Some(id) => Some(id), None => match chain.last() { Some(id) => Some(copy_history_text(id, control).await?), None => None } };
    let checkpoint = control.track(checkpoint).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut visible = control.track(HistoryFoldSet::<usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if let Some(id) = checkpoint.as_ref() {
        let position = original_fold_required(control.get_text(&checkpoint_lookup, id).await?,&[FoldErrorPart::Text("viewer head names unknown checkpoint "),FoldErrorPart::Text(&id)],control).await?;
        for change_id in &fold.checkpoints[position].change_ids {
            control.pulse().await?;
            let change = original_fold_required(control.get_text(&change_lookup, change_id).await?,&[FoldErrorPart::Text("checkpoint "),FoldErrorPart::Text(&id),FoldErrorPart::Text(" names unknown change "),FoldErrorPart::Text(&change_id)],control).await?;
            for edit_id in &fold.changes[change].edit_ids { control.pulse().await?; if let Some(position) = control.get_text(&edit_lookup, edit_id).await? { if active.contains_original_copy(position, control).await? { visible.insert_original_copy(*position, control).await?; } } }
        }
    }
    if at_tip {
        let mut committed = control.track(HistoryFoldSet::<usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        for change in &fold.changes { for edit_id in &change.edit_ids { control.pulse().await?; if let Some(position) = control.get_text(&edit_lookup, edit_id).await? { committed.insert_original_copy(*position, control).await?; } } }
        for (position, edit) in edits.iter().enumerate() { control.pulse().await?; if !active.contains_original_copy(&position, control).await? || committed.contains_original_copy(&position, control).await? { continue; } let on_line = match &edit.line { Some(line) if line != &fold.trunk => line == &head.line_id, _ => on_trunk }; if on_line { visible.insert_original_copy(position, control).await?; } }
    }
    let mut ordered = control.track(HistoryFoldIndex::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for position in visible.iter().copied() { let edit = &edits[position]; control.insert_ordered_text(&mut ordered, (edit.timestamp.cmp_key(), copy_history_text(&edit.id, control).await?), position).await?; }
    loop {let row=control.pop_first(&mut ordered).await?;let Some((key,position))=row.take()else{break};control.track(key).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;fold.applied.push(copy_history_text(&edits[position].id,control).await?);}
    for position in redo.values() { fold.redo.push(copy_history_text(&edits[*position].id, control).await?); }
    if !trunk_chain.is_empty() { let trunk = copy_history_text(&fold.trunk, control).await?; fold.alternatives.push(FoldAlternative { id: trunk, name: String::new(), checkpoint_ids: trunk_chain.take() }); for index in (1..fold.alternatives.len()).rev() { control.pulse().await?; fold.alternatives.swap(index, index-1); } }
    fold.checkpoint = checkpoint.take();
    fold.alternative = if on_trunk { None } else { Some(copy_history_text(&head.line_id, control).await?) };
    Ok(fold.take())
}

/// 📄️ Copies immutable protocol bytes through the same granted decoder pages.
pub async fn copy_history_bytes(bytes: &[u8], control: &HistoryFoldControl) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> {
    let mut copied = control.vector(bytes.len()).await?;
    let mut at = 0;
    control.pulse().await?;
    control.copy_bytes_into(&mut copied,bytes,&mut at,bytes.len()).await?;
    Ok(copied.take())
}

/// 🪪️ Validates an opaque quarantined envelope while copying only its operation identity.
pub async fn history_envelope_id_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<MutationId, semio_framework_replication_error::ProtocolError> {
    let mut pos = 0;
    let identity = control.track(MutationId(read_fold_string(bytes, &mut pos, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let count = control.read_varint(bytes, &mut pos).await?;
    if count > bytes.len().saturating_sub(pos) as u64 { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("dependency count exceeds envelope")],control).await?); }
    for _ in 0..count { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    match control.read_varint(bytes, &mut pos).await? { 0 => {}, 1 => { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }, flag => return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("observed flag "),FoldErrorPart::Unsigned(flag as u64)],control).await?) }
    let count = control.read_varint(bytes, &mut pos).await?;
    if count > bytes.len().saturating_sub(pos) as u64 { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("target count exceeds envelope")],control).await?); }
    for _ in 0..count { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    for _ in 0..2 {
        control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        control.pulse().await?;
        let length = control.read_varint(bytes, &mut pos).await?;
        pos = original_fold_offset(usize::try_from(length).ok().and_then(|length| pos.checked_add(length)).filter(|end| *end <= bytes.len()),pos,"truncated envelope payload",control).await?;
    }
    control.pulse().await?;
    read_fold_hlc(bytes, &mut pos, control).await?;
    let flags = control.read_varint(bytes, &mut pos).await?;
    if flags > 0b111 { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("invalid envelope flags")],control).await?); }
    if flags & 1 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    if flags & 2 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    if flags & 4 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    if pos != bytes.len() { return Err(original_fold_refusal("history transition",pos,&[FoldErrorPart::Text("quarantined envelope has trailing bytes")],control).await?); }
    Ok(identity.take())
}

async fn read_fold_blob(bytes: &[u8], position: &mut usize, control: &HistoryFoldControl) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> {
    control.pulse().await?;
    let length = original_fold_offset(usize::try_from(control.read_varint(bytes, position).await?).ok(),*position,"envelope payload length exceeds address space",control).await?;
    let end = original_fold_offset(position.checked_add(length).filter(|end| *end <= bytes.len()),*position,"truncated envelope payload",control).await?;
    let copied = copy_history_bytes(&bytes[*position..end], control).await?;
    *position = end;
    Ok(copied)
}

/// 📦️ Decodes quarantined envelopes in byte pages, preserving every opaque payload and exact header fact.
pub async fn decode_history_envelope_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<semio_framework_replication_causal::MutationEnvelope, semio_framework_replication_error::ProtocolError> {
    let mut position = 0;
    let mutation_id = control.track(MutationId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let document_id = control.track(ArtifactId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let original_actor=control.track(read_fold_string(bytes,&mut position,control).await?).unwrap_or_else(|_|unreachable!("fold String declares tracked original ownership")).await;
    let shared_actor=control.adopt_actor(original_actor).await;
    let actor=control.track(ActorId(shared_actor)).unwrap_or_else(|_|unreachable!("fold actor declares shared original ownership")).await;
    let dependencies = control.track(read_fold_ids(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.pulse().await?;
    let observed = control.track(match control.read_varint(bytes, &mut position).await? { 0 => None, 1 => Some(MutationId(read_fold_string(bytes, &mut position, control).await?)), other => return Err(original_fold_refusal("history transition",position,&[FoldErrorPart::Text("invalid observed flag "),FoldErrorPart::Unsigned(other as u64)],control).await?) }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let count = control.read_varint(bytes, &mut position).await?;
    if count > bytes.len().saturating_sub(position) as u64 { return Err(original_fold_refusal("history transition",position,&[FoldErrorPart::Text("target count exceeds envelope")],control).await?); }
    let mut target = control.vector(count as usize).await?;
    for _ in 0..count { target.push(read_fold_string(bytes, &mut position, control).await?); }
    let diff_schema = control.track(SchemaId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let diff_payload = control.track(read_fold_blob(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let inverse_schema = control.track(SchemaId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let inverse_payload = control.track(read_fold_blob(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.pulse().await?;
    let timestamp = read_fold_hlc(bytes, &mut position, control).await?;
    let flags = control.read_varint(bytes, &mut position).await?;
    if flags > 0b111 { return Err(original_fold_refusal("history transition",position,&[FoldErrorPart::Text("invalid envelope flags")],control).await?); }
    let transaction = control.track(if flags & 1 != 0 {
        let id = control.track(read_fold_string(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        let tool = read_fold_string(bytes, &mut position, control).await?;
        Some(semio_framework_replication_edit::TransactionRef { id: id.take(), tool })
    } else { None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let verb = control.track(if flags & 2 != 0 { Some(read_fold_string(bytes, &mut position, control).await?) } else { None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let line = control.track(if flags & 4 != 0 { Some(read_fold_string(bytes, &mut position, control).await?) } else { None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if position != bytes.len() { return Err(original_fold_refusal("history transition",position,&[FoldErrorPart::Text("quarantined envelope has trailing bytes")],control).await?); }
    Ok(semio_framework_replication_causal::MutationEnvelope { mutation_id: mutation_id.take(), document_id: document_id.take(), actor: actor.take(), dependencies: dependencies.take(), observed: observed.take(), target: target.take(), diff: semio_framework_replication_causal::ArtifactDiff { schema: diff_schema.take(), payload: diff_payload.take() }, inverse: semio_framework_replication_causal::InverseMutation { schema: inverse_schema.take(), payload: inverse_payload.take() }, timestamp, transaction: transaction.take(), verb: verb.take(), line: line.take() })
}

#[cfg(test)]
include!("📏️retirement/🧪️tests/🦀️.rs");
