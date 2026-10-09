//! ⏳️ Granted history folding with cooperative decoding and exact cancellation retirement.

use super::*;
use semio_framework_value::{ErasedSnapshotRetirement, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{RetireOwned, admit_owned_retirement, owned_retirement_birth_bytes};
use std::mem::{ManuallyDrop,size_of,size_of_val};
use semio_framework_value::retirement::controlled::{ControlledRetirement, admit_typed_controlled_retirement};
use std::{collections::VecDeque, future::Future, ops::{Deref, DerefMut}, pin::Pin, sync::{Arc, atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering}}, task::{Context, Poll, Wake, Waker}};

#[path = "🗂️index/🦀️.rs"]
mod index;
pub use index::{HistoryFoldIndex,HistoryFoldIndexIntoIter,HistoryFoldIndexIter,HistoryFoldSet,HistoryFoldSetIter,HistoryFoldSetIntoIter};

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

struct FoldControlState {
    cancelled: AtomicBool,
    completed: AtomicU64,
    page_bytes: AtomicUsize,
    pending_retirement_birth: AtomicUsize,
    pending_retirement_items: AtomicUsize,
    retirement_capacity: AtomicUsize,
    retirement_items: AtomicUsize,
    admitted_retirement_items: AtomicUsize,
    admitted_retirement_capacity: AtomicUsize,
    original_copy: AtomicUsize,
    original_depth: AtomicUsize,
    original_release: AtomicUsize,
    admitted_original_copy: AtomicUsize,
    retirements: FoldRetirementQueue,
}

/// ⏱️ The shared pulse and retirement authority of one immutable-source fold.
#[derive(Clone)]
pub struct HistoryFoldControl(Arc<FoldControlState>);

impl HistoryFoldControl {
    /// 🌱️ Creates fixed retirement queue capacity without inspecting any history.
    pub fn new() -> Self { Self(Arc::new(FoldControlState { cancelled: AtomicBool::new(false), completed: AtomicU64::new(0), page_bytes: AtomicUsize::new(4096), pending_retirement_birth:AtomicUsize::new(0), pending_retirement_items:AtomicUsize::new(0), retirement_capacity:AtomicUsize::new(0), retirement_items:AtomicUsize::new(0), admitted_retirement_items:AtomicUsize::new(0), admitted_retirement_capacity:AtomicUsize::new(0), original_copy:AtomicUsize::new(0),original_depth:AtomicUsize::new(0),original_release:AtomicUsize::new(0),admitted_original_copy:AtomicUsize::new(0),retirements: FoldRetirementQueue::new() })) }
    /// 📊️ Completed cooperative work units, excluding cancellation retirement.
    pub fn completed(&self) -> u64 { self.0.completed.load(Ordering::Relaxed) }
    fn original_grant(&self)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:self.0.retirement_items.load(Ordering::Relaxed),maximum_copy_bytes:self.0.original_copy.load(Ordering::Relaxed),maximum_capacity_bytes:self.0.retirement_capacity.load(Ordering::Relaxed),maximum_release_bytes:self.0.original_release.load(Ordering::Relaxed),maximum_depth:self.0.original_depth.load(Ordering::Relaxed)}}
    fn accept_original_progress(&self,progress:RetainedCloneProgress){self.0.retirement_items.fetch_sub(progress.copied_items,Ordering::Relaxed);self.0.retirement_capacity.fetch_sub(progress.retained_capacity_bytes,Ordering::Relaxed);self.0.original_copy.fetch_sub(progress.copied_bytes,Ordering::Relaxed);self.0.admitted_retirement_items.fetch_add(progress.copied_items,Ordering::Relaxed);self.0.admitted_retirement_capacity.fetch_add(progress.retained_capacity_bytes,Ordering::Relaxed);self.0.admitted_original_copy.fetch_add(progress.copied_bytes,Ordering::Relaxed);}
    pub fn adopt_actor(&self,original:HistoryFoldOwned<String>)->SharedActorAdmission {let capacity=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<String>();self.0.pending_retirement_birth.fetch_add(capacity,Ordering::Relaxed);self.0.pending_retirement_items.fetch_add(1,Ordering::Relaxed);SharedActorAdmission{original:Some(original),control:self.clone(),capacity}}
    /// 🪪️ Borrows the original shared actor until its exact lease header is admitted.
    pub fn lease_actor<'a>(&self,original:&'a semio_framework_value::SharedUtf8)->SharedActorLease<'a>{self.0.pending_retirement_items.fetch_add(1,Ordering::Relaxed);SharedActorLease{original,control:self.clone()}}
    /// ♻️ Transfers an owned local to exact retirement when its coroutine scope ends.
    pub fn track<T: RetireOwned>(&self, value: T) -> Result<HistoryFoldAdmission<T>, (ValueError, T)> {
        if !T::controlled_retirement_supported() { return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner, "fold local has no controlled retirement authority"), value)); }
        self.0.pending_retirement_birth.fetch_add(owned_retirement_birth_bytes::<T>(), Ordering::Relaxed);
        self.0.pending_retirement_items.fetch_add(1, Ordering::Relaxed);
        Ok(HistoryFoldAdmission { value: Some(value), control: self.clone() })
    }
    /// ⏭️ One pulse before each bounded work unit; cancellation is checked before publication.
    pub async fn pulse(&self) -> Result<(), crate::ProtocolError> { FoldPulse { control: self, yielded: false }.await }
    fn page_bytes(&self) -> usize { self.0.page_bytes.load(Ordering::Relaxed).clamp(1, 4096) }
    fn retire<T: RetireOwned>(&self, value: T) {
        let bytes=owned_retirement_birth_bytes::<T>();
        self.0.retirement_capacity.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|remaining|remaining.checked_sub(bytes)).expect("fold transfer requires admitted retirement capacity");
        self.0.retirement_items.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|remaining|remaining.checked_sub(1)).expect("fold transfer requires admitted retirement item");
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
impl Future for SharedActorAdmission {
    type Output=semio_framework_value::SharedUtf8;
    fn poll(mut self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{
        let grant=self.control.original_grant();let copy=size_of::<String>()+size_of::<semio_framework_value::SharedUtf8>();
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<self.capacity{return Poll::Pending;}
        let original=self.original.take().expect("original actor admission retains paid String owner").take();
        let(owner,progress)=semio_framework_value::SharedUtf8::admit(original,grant).unwrap_or_else(|_|unreachable!("original actor admission was preflighted"));
        self.control.accept_original_progress(progress);Poll::Ready(owner)
    }
}
impl Drop for SharedActorAdmission {fn drop(&mut self){self.control.0.pending_retirement_birth.fetch_sub(self.capacity,Ordering::Relaxed);self.control.0.pending_retirement_items.fetch_sub(1,Ordering::Relaxed);}}

/// 🧾️ One original actor alias waits for the caller's independent copy and item authority.
pub struct SharedActorLease<'a>{original:&'a semio_framework_value::SharedUtf8,control:HistoryFoldControl}
impl Future for SharedActorLease<'_>{type Output=semio_framework_value::SharedUtf8;fn poll(self:Pin<&mut Self>,_:&mut Context<'_>)->Poll<Self::Output>{let grant=self.control.original_grant();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<size_of::<semio_framework_value::SharedUtf8>(){return Poll::Pending;}let(owner,progress)=self.original.admit_clone(grant).unwrap_or_else(|_|unreachable!("original shared actor lease preflighted"));self.control.accept_original_progress(progress);Poll::Ready(owner)}}
impl Drop for SharedActorLease<'_>{fn drop(&mut self){self.control.0.pending_retirement_items.fetch_sub(1,Ordering::Relaxed);}}

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
    type Output = Result<(), crate::ProtocolError>;
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
    future: Option<Pin<Box<dyn Future<Output = Result<T, crate::ProtocolError>> + Send + 'a>>>,
    result: Option<Result<T, crate::ProtocolError>>,
    waker: Option<Waker>,
    future_finished: bool,
    control_closed: bool,
    completed_final: u64,
    terminal: bool,
}

impl<'a, T: RetireOwned> HistoryFoldJob<'a, T> {
    /// 🌱️ Captures immutable owners without scanning their contents.
    pub fn new<F: Future<Output = Result<T, crate::ProtocolError>> + Send + 'a>(runner: impl FnOnce(HistoryFoldControl) -> F) -> Self {
        let control = HistoryFoldControl::new();
        let future = Box::pin(runner(control.clone()));
        Self { control:ManuallyDrop::new(control), future: Some(future), result: None, waker: Some(Waker::from(Arc::new(FoldWake))), future_finished:false,control_closed:false,completed_final:0,terminal: false }
    }
    /// 📊️ Completed semantic and decoding work units.
    pub fn completed(&self) -> u64 { if self.control_closed{self.completed_final}else{self.control.completed()} }
    /// ⏭️ Preserves independent work, copy, allocation, release and depth grants.
    pub fn step(&mut self,grant:RetainedCloneGrant,should_yield:&mut impl FnMut()->bool)->Result<HistoryFoldJobStep<T>,ValueError>{
        let pending=|completed,progress:RetainedCloneProgress|HistoryFoldJobStep::Pending{completed,released_bytes:progress.released_bytes,progress};
        if grant.maximum_items==0||should_yield(){return Ok(pending(self.completed(),Default::default()));}
        if self.control_closed && matches!(self.result, Some(Err(_))) { return Ok(HistoryFoldJobStep::Rejected { progress: Default::default() }); }
        if !self.control_closed&&self.control.0.cancelled.load(Ordering::Acquire){let step=self.close_step(grant)?;return Ok(pending(self.completed(),step.progress()));}
        if self.future_finished||self.future.is_none()||!self.control.empty(){let step=self.cleanup_one(grant,false)?;let progress=step.progress();if self.control_closed{match self.result.as_ref(){Some(Ok(_))=>{let Some(Ok(value))=self.result.take()else{unreachable!()};self.terminal=true;return Ok(HistoryFoldJobStep::Ready { value, progress });},Some(Err(_))=>return Ok(HistoryFoldJobStep::Rejected { progress }),None=>{}}}return Ok(pending(self.completed(),progress));}
        if grant.maximum_copy_bytes==0{return Ok(pending(self.completed(),Default::default()));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"fold work requires admitted depth"));}
        let birth=self.control.0.pending_retirement_birth.load(Ordering::Relaxed);if grant.maximum_capacity_bytes<birth||grant.maximum_items<self.control.0.pending_retirement_items.load(Ordering::Relaxed){return Ok(pending(self.completed(),Default::default()));}
        self.control.0.page_bytes.store(grant.maximum_copy_bytes.clamp(1,4096),Ordering::Relaxed);
        self.control.0.original_copy.store(grant.maximum_copy_bytes,Ordering::Relaxed);self.control.0.original_depth.store(grant.maximum_depth,Ordering::Relaxed);self.control.0.original_release.store(grant.maximum_release_bytes,Ordering::Relaxed);self.control.0.admitted_original_copy.store(0,Ordering::Relaxed);
        self.control.0.retirement_capacity.store(grant.maximum_capacity_bytes,Ordering::Relaxed);self.control.0.admitted_retirement_capacity.store(0,Ordering::Relaxed);self.control.0.retirement_items.store(grant.maximum_items,Ordering::Relaxed);self.control.0.admitted_retirement_items.store(0,Ordering::Relaxed);
        let mut items=0;while items<grant.maximum_items{if should_yield(){break;}self.control.0.retirement_items.store(grant.maximum_items-items,Ordering::Relaxed);let before=self.control.0.admitted_retirement_items.load(Ordering::Relaxed);let polled=self.future.as_mut().unwrap().as_mut().poll(&mut Context::from_waker(self.waker.as_ref().unwrap()));items+=(self.control.0.admitted_retirement_items.load(Ordering::Relaxed)-before).max(1);if let Poll::Ready(result)=polled{self.result=Some(result);self.future_finished=true;break;}if !self.control.empty(){break;}}
        let progress=RetainedCloneProgress{copied_items:items,copied_bytes:self.control.0.admitted_original_copy.load(Ordering::Relaxed),retained_capacity_bytes:self.control.0.admitted_retirement_capacity.load(Ordering::Relaxed),..Default::default()};semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"fold cooperative work")?;Ok(pending(self.completed(),progress))
    }
    /// 🧮️ Publishes every waiting original frame item before atomic coroutine destruction.
    pub fn next_local_admission_item_demand(&self)->usize{if self.control_closed{0}else{self.control.0.pending_retirement_items.load(Ordering::Relaxed)}}
    /// 📏️ Explicit cold admission preserves all independent currencies.
    pub fn next_step_grant(&self,maximum_items:usize,logical_copy:usize)->Result<RetainedCloneGrant,ValueError>{
        Ok(RetainedCloneGrant{maximum_items,maximum_copy_bytes:if !self.control_closed&&!self.future_finished&&self.control.empty(){logical_copy}else{self.next_copy_byte_demand()?.max(logical_copy)},maximum_capacity_bytes:self.next_capacity_byte_demand(logical_copy)?,maximum_release_bytes:self.next_release_byte_demand()?,maximum_depth:self.next_depth_demand()?.max(1)})
    }
    fn control_frame_bytes(&self)->usize{let layout=std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<FoldControlState>()).unwrap().0.pad_to_align();if Arc::strong_count(&self.control.0)==1{layout.size()}else{0}}
    fn cleanup_one(&mut self,grant:RetainedCloneGrant,cancelled:bool)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.control_closed {
            if self.result.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
            let Some(Err(error)) = self.result.as_mut() else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "closed fold retained an unhanded successful result")); };
            let step = crate::close_protocol_cause_one(error, grant)?;
            if matches!(step, RetainedCloneStep::Complete(_)) { drop(self.result.take()); }
            return Ok(step);
        }
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"fold closure requires admitted depth"));}
        let birth=self.next_capacity_byte_demand(grant.maximum_copy_bytes)?;let release=self.next_release_byte_demand()?;if grant.maximum_capacity_bytes<birth||grant.maximum_release_bytes<release{return Ok(RetainedCloneStep::Progress(empty));}
        if self.control.empty()&&self.future.is_some()&&grant.maximum_items<self.control.0.pending_retirement_items.load(Ordering::Relaxed){return Ok(RetainedCloneStep::Progress(empty));}
        if !self.control.empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};return self.control.close_one(child);}
        if self.future.is_some(){self.control.0.retirement_capacity.store(grant.maximum_capacity_bytes,Ordering::Relaxed);self.control.0.admitted_retirement_capacity.store(0,Ordering::Relaxed);self.control.0.retirement_items.store(grant.maximum_items,Ordering::Relaxed);self.control.0.admitted_retirement_items.store(0,Ordering::Relaxed);drop(self.future.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:self.control.0.admitted_retirement_items.load(Ordering::Relaxed).max(1),retained_capacity_bytes:self.control.0.admitted_retirement_capacity.load(Ordering::Relaxed),released_bytes:release,..empty}));}
        if cancelled&&self.result.is_some(){let Some(result)=self.result.take()else{unreachable!()};match result{Ok(value)=>{match admit_owned_retirement(value,grant){Ok((owner,progress))=>{let mut queue=self.control.0.retirements.lock();assert!(queue.len()<queue.capacity(),"fold result retirement requires reserved slot");queue.push_back(owner);return Ok(RetainedCloneStep::Progress(progress));},Err((error,value))=>{self.result=Some(Ok(value));return Err(error);}}},Err(error)=>{self.result=Some(Err(error));let Some(Err(error))=self.result.as_mut()else{unreachable!()};let step=crate::close_protocol_cause_one(error,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){drop(self.result.take());}return Ok(RetainedCloneStep::Progress(step.progress()));}}}
        let mut queue=self.control.0.retirements.lock();if queue.capacity()!=0{drop(std::mem::take(&mut*queue));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..empty}));}drop(queue);
        if self.waker.is_some(){drop(self.waker.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..empty}));}
        self.completed_final=self.control.completed();unsafe{ManuallyDrop::drop(&mut self.control)};self.control_closed=true;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:release,..empty}))
    }
    /// 🧊️ Runs the same cursor to completion for explicit cold callers; interactive owners grant individual turns.
    pub fn finish_cold(mut self) -> Result<T, crate::ProtocolError> {
        loop {
            match self.step(self.next_step_grant(64,4096).expect("registered fold demands"), &mut || false).expect("registered history owners must retire within their grants") {
                HistoryFoldJobStep::Pending { .. } => {},
                HistoryFoldJobStep::Ready { value, .. } => return Ok(value),
                HistoryFoldJobStep::Rejected { .. } => return Err(self.take_rejection().expect("cold fold rejection retains its original error")),
            }
        }
    }

    /// 🔎️ Borrows the original rejected result without copying or disposing it.
    pub fn rejection(&self) -> Option<&crate::ProtocolError> { match self.result.as_ref() { Some(Err(error)) => Some(error), _ => None } }

    /// 📤️ Transfers the original closed result into an explicit cold caller.
    pub fn take_rejection(&mut self) -> Option<crate::ProtocolError> {
        if !self.control_closed || !matches!(self.result, Some(Err(_))) { return None; }
        let Some(Err(error)) = self.result.take() else { unreachable!() };
        self.terminal = true;
        Some(error)
    }

    /// 🛑️ Cancels without executing further fold work; tracked locals transfer to exact retirement.
    pub fn request_cancel(&mut self){if !self.control_closed{self.control.0.cancelled.store(true,Ordering::Release);}}

}

impl<T:RetireOwned> ErasedSnapshotRetirement for HistoryFoldJob<'_,T>{
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if self.control_closed{self.result.as_ref().map_or(Ok(0),|result|match result{Err(error)=>crate::protocol_cause_retirement_demand(error).map(|demand|demand.copy_bytes),Ok(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result"))})}else if self.control.empty()&&self.future.is_none()&&self.control.0.cancelled.load(Ordering::Acquire){match self.result.as_ref(){Some(Err(error))=>crate::protocol_cause_retirement_demand(error).map(|demand|demand.copy_bytes),_=>Ok(0)}}else{self.control.demands(0).map(|demand|demand.copy_bytes)}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if self.control_closed{return self.result.as_ref().map_or(Ok(0),|result|match result{Err(error)=>crate::protocol_cause_retirement_demand(error).map(|demand|demand.capacity_bytes),Ok(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result"))});}if !self.control.empty(){return Ok(self.control.demands(copy)?.capacity_bytes);}if self.future.is_some(){return Ok(self.control.0.pending_retirement_birth.load(Ordering::Relaxed));}Ok(if self.control.0.cancelled.load(Ordering::Acquire)&&matches!(self.result,Some(Ok(_))){owned_retirement_birth_bytes::<T>()}else{0})}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if self.control_closed{return self.result.as_ref().map_or(Ok(0), |result| match result { Err(error) => crate::protocol_cause_retirement_demand(error).map(|demand|demand.release_bytes), Ok(_) => Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result")) });}if !self.control.empty(){return Ok(self.control.demands(0)?.release_bytes);}if let Some(future)=self.future.as_ref(){return Ok(size_of_val(future.as_ref().get_ref()));}if self.control.0.cancelled.load(Ordering::Acquire){if let Some(result)=self.result.as_ref(){return match result{Ok(_)=>Ok(0),Err(error)=>crate::protocol_cause_retirement_demand(error).map(|demand|demand.release_bytes)};}}let queue=self.control.0.retirements.lock();if queue.capacity()!=0{return Ok(queue.capacity()*size_of::<Box<dyn ErasedSnapshotRetirement>>());}drop(queue);if self.waker.is_some(){return Ok(size_of::<[usize;2]>());}Ok(self.control_frame_bytes())}
    fn next_depth_demand(&self)->Result<usize,ValueError>{if self.control_closed{return self.result.as_ref().map_or(Ok(0),|result|match result{Err(error)=>crate::protocol_cause_retirement_demand(error).map(|demand|demand.depth),Ok(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closed fold retained successful result"))});}let depth=if self.control.empty(){0}else{self.control.demands(0)?.depth};depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"fold nested retirement depth overflow"))}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.request_cancel();let step=self.cleanup_one(grant,true)?;if self.control_closed&&matches!(step,RetainedCloneStep::Complete(_)){self.terminal=true;}semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"fold job closure")}
    fn terminal_is_empty(&self)->bool{self.terminal&&self.control_closed&&self.future.is_none()&&self.result.is_none()&&self.waker.is_none()}
}
impl<T: RetireOwned> Drop for HistoryFoldJob<'_, T> { fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "history fold job released before exact handoff or retirement"); } }

/// 📝️ Copies verified UTF-8 in fixed pages, yielding before each page allocation or write.
pub async fn copy_history_text(text: &str, control: &HistoryFoldControl) -> Result<String, crate::ProtocolError> { copy_history_text_parts(&[text], control).await }

pub async fn copy_history_text_parts(parts: &[&str], control: &HistoryFoldControl) -> Result<String, crate::ProtocolError> {
    let mut copied = control.track(String::with_capacity(parts.iter().map(|part| part.len()).sum())).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;

    let mut scalar = [0;4];
    let mut width = 0;
    let mut needed = 0;
    control.pulse().await?;
    for text in parts {
    let mut at = 0;
    while at < text.len() {
        control.pulse().await?;
        let end = (at + control.page_bytes()).min(text.len());
        for &byte in &text.as_bytes()[at..end] {
            if width == 0 { needed = match byte { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, _ => 4 }; }
            scalar[width] = byte; width += 1;
            if width == needed { copied.push_str(std::str::from_utf8(&scalar[..width]).expect("source text preserves complete UTF-8")); width = 0; }
        }
        at = end;
    }
    }
    Ok(copied.take())
}

async fn read_fold_string(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<String, crate::ProtocolError> {
    control.pulse().await?;
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    let length = usize::try_from(count).map_err(|_| malformed(*pos, "string length exceeds address space"))?;
    let end = pos.checked_add(length).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(*pos, "truncated string"))?;
    let mut text = control.track(String::with_capacity(length)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut scalar = [0; 4];
    let mut width = 0;
    let mut needed = 0;
    while *pos < end {
        control.pulse().await?;
        let limit = (*pos + control.page_bytes()).min(end);
        while *pos < limit {
            let byte = bytes[*pos];
            if width == 0 { needed = match byte { 0..=0x7f => 1, 0xc2..=0xdf => 2, 0xe0..=0xef => 3, 0xf0..=0xf4 => 4, _ => return Err(malformed(*pos, "invalid UTF-8")) }; }
            scalar[width] = byte;
            width += 1;
            *pos += 1;
            if width == needed { let value = std::str::from_utf8(&scalar[..width]).map_err(|_| malformed(*pos-width, "invalid UTF-8"))?; text.push_str(value); width = 0; }
        }
    }
    if width != 0 { return Err(malformed(*pos, "truncated UTF-8")); }
    Ok(text.take())
}

async fn read_fold_optional(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Option<String>, crate::ProtocolError> {
    control.pulse().await?;
    match read_u8(bytes, pos)? { 0 => Ok(None), 1 => read_fold_string(bytes, pos, control).await.map(Some), other => Err(malformed(*pos, format!("invalid option tag {other}"))) }
}

async fn read_fold_ids(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Vec<MutationId>, crate::ProtocolError> {
    control.pulse().await?;
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    if count > bytes.len().saturating_sub(*pos) as u64 { return Err(malformed(*pos, "id count exceeds payload")); }
    let mut ids = control.track(Vec::with_capacity(count as usize)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for _ in 0..count { ids.push(MutationId(read_fold_string(bytes, pos, control).await?)); }
    Ok(ids.take())
}

async fn read_fold_payload(bytes: &[u8], pos: &mut usize, control: &HistoryFoldControl) -> Result<Vec<u8>, crate::ProtocolError> {
    control.pulse().await?;
    let length = crate::wire::read_varint_u64(bytes, pos)?;
    if length > SUPERSEDE_PAYLOAD_MAX_BYTES as u64 { return Err(malformed(*pos, format!("supersede payload exceeds {SUPERSEDE_PAYLOAD_MAX_BYTES} bytes"))); }
    let end = pos.checked_add(length as usize).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(*pos, "truncated replacement payload"))?;
    let mut payload = control.track(Vec::with_capacity(length as usize)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    while *pos < end { control.pulse().await?; let limit = (*pos+control.page_bytes()).min(end); payload.extend_from_slice(&bytes[*pos..limit]); *pos = limit; }
    Ok(payload.take())
}

/// 🔀️ Decodes every transition field and repeated member cooperatively; malformed and cancelled partial owners retire.
pub async fn decode_history_transition_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<HistoryTransition, crate::ProtocolError> {
    let mut pos = 0;
    control.pulse().await?;
    let transition = match crate::wire::read_varint_u64(bytes, &mut pos)? {
        0 => HistoryTransition::Revert { mutation_ids: read_fold_ids(bytes, &mut pos, control).await? },
        1 => HistoryTransition::Reinstate { mutation_ids: read_fold_ids(bytes, &mut pos, control).await? },
        2 => {
            let checkpoint_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let parent_id = control.track(read_fold_optional(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let change_id = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let mutation_ids = control.track(read_fold_ids(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let description = control.track(read_fold_optional(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let saved_at = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "author count exceeds payload")); }
            let mut authors = control.track(Vec::with_capacity(count as usize)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
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
            let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "pin count exceeds payload")); }
            let mut pins = control.track(Vec::with_capacity(count as usize)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            for _ in 0..count { let child_uri = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; let checkpoint_id = read_fold_string(bytes, &mut pos, control).await?; pins.push(TransitionPin { child_uri: child_uri.take(), checkpoint_id }); }
            HistoryTransition::Repin { checkpoint_id: checkpoint_id.take(), pinned_checkpoint_id: pinned_checkpoint_id.take(), pins: pins.take() }
        }
        6 => {
            let scope = control.track(read_fold_optional(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            if scope.as_ref().is_some_and(|scope| scope.len() > SUPERSEDE_SCOPE_MAX_BYTES) { return Err(malformed(pos, format!("supersede scope exceeds {SUPERSEDE_SCOPE_MAX_BYTES} bytes"))); }
            let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if count == 0 { return Err(malformed(pos, "supersede names no input")); }
            if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "input count exceeds payload")); }
            let mut inputs = control.track(Vec::with_capacity(count as usize)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            let mut seen = control.track(HistoryFoldSet::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
            for _ in 0..count {
                let target = control.track(MutationId(read_fold_string(bytes, &mut pos, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                if !seen.insert(copy_history_text(&target.0, control).await?) { return Err(malformed(pos, format!("supersede repeats target {}", target.0))); }
                let replacement = match read_u8(bytes, &mut pos)? {
                    0 => { let schema = control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; let payload = read_fold_payload(bytes, &mut pos, control).await?; InputReplacement::Input { schema: schema.take(), payload } }
                    1 => InputReplacement::Withdrawn,
                    tag => return Err(malformed(pos-1, format!("invalid replacement tag {tag}"))),
                };
                inputs.push(SupersededInput { target: target.take(), replacement });
            }
            HistoryTransition::Supersede(TransitionSupersede { scope: scope.take(), inputs: inputs.take() })
        }
        tag => return Err(malformed(0, format!("unknown transition tag {tag}"))),
    };
    let transition = control.track(transition).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if pos != bytes.len() { return Err(malformed(pos, "trailing bytes")); }
    Ok(transition.take())
}

async fn owned_positions(ids: &[MutationId], owners: &HistoryFoldIndex<String, usize>, control: &HistoryFoldControl) -> Result<Vec<usize>, crate::ProtocolError> {
    let mut positions = control.track(Vec::with_capacity(ids.len())).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut seen = control.track(HistoryFoldSet::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for id in ids { control.pulse().await?; let position = *owners.get(&id.0).ok_or_else(|| fold_error(format!("transition references unknown operation {}", id.0)))?; if seen.insert(position) { positions.push(position); } }
    Ok(positions.take())
}

/// 🧮️ The shared semantic fold, cooperatively ordered and indexed without whole-history collect, sort or retain.
pub async fn fold_history_for_controlled(document_id: &ArtifactId, edits: &[FoldEdit], transitions: &[super::super::MutationEnvelope], excluded: &(impl Fn(&str) -> bool + Sync), head: &ViewerHead, control: &HistoryFoldControl) -> Result<HistoryFold, crate::ProtocolError> {
    let mut owners = control.track(HistoryFoldIndex::<String, usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut identities = control.track(HistoryFoldSet::<String>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut events = control.track(HistoryFoldIndex::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for (index, edit) in edits.iter().enumerate() {
        let id = control.track(copy_history_text(&edit.id, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        if !identities.insert(copy_history_text(&id, control).await?) { return Err(fold_error("history repeats an edit")); }
        for mutation in &edit.mutation_ids { owners.insert(copy_history_text(&mutation.0, control).await?, index); }
        events.insert((edit.timestamp.cmp_key(), id.take()), (false, index));
    }
    for (index, event) in transitions.iter().enumerate() {
        let id = control.track(copy_history_text(&event.mutation_id.0, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        if !identities.insert(copy_history_text(&id, control).await?) { return Err(fold_error(format!("history repeats transition {}", id.as_str()))); }
        if event.diff.schema.0 != HISTORY_TRANSITION_SCHEMA { return Err(fold_error(format!("{} is not a history transition", id.as_str()))); }
        events.insert((event.timestamp.cmp_key(), id.take()), (true, index));
    }
    control.pulse().await?;
    let mut fold = control.track(HistoryFold { trunk: trunk_alternative_id(document_id), applied: Vec::with_capacity(edits.len()), redo: Vec::with_capacity(edits.len()), refused: Vec::with_capacity(transitions.len()), changes: Vec::with_capacity(transitions.len()), checkpoints: Vec::with_capacity(transitions.len()), alternatives: Vec::with_capacity(transitions.len()+1), ..HistoryFold::default() }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut trunk_chain = control.track(Vec::<String>::with_capacity(transitions.len())).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
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
    for (index, edit) in edits.iter().enumerate() { edit_lookup.insert(copy_history_text(&edit.id, control).await?, index); }
    while let Some((key, (structural, index))) = events.pop_first() {
        let key = control.track(key).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        control.pulse().await?;
        if !structural {
            let edit = &edits[index];
            if excluded(&edit.id) { continue; }
            active.insert(index);
            while let Some(position) = redo_actors.get_mut(&edit.actor).and_then(HistoryFoldSet::pop_first) { control.pulse().await?; if let Some(index) = redo.remove(&position) { redo_lookup.remove(&index); } }
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
                        if let Some(order) = redo_lookup.remove(&position) { redo.remove(&order); if let Some(orders) = redo_actors.get_mut(&edits[position].actor) { orders.remove(&order); } active.insert(position); }
                    } else if active.remove(&position) {
                        redo.insert(serial, position);
                        redo_lookup.insert(position, serial);
                        let actor = match &edits[position].actor { Some(actor) => Some(control.lease_actor(actor).await), None => None };
                        redo_actors.get_or_insert(actor, HistoryFoldSet::new()).insert(serial);
                        serial += 1;
                    }
                }
            }
            HistoryTransition::Commit(checkpoint) => {
                let parent = match &checkpoint.parent_id { Some(parent) => Some(*checkpoint_lookup.get(parent).ok_or_else(|| fold_error(format!("checkpoint {} names unknown parent {parent}", checkpoint.checkpoint_id)))?), None => None };
                let count = parent.map_or(0, |parent| fold.checkpoints[parent].change_ids.len());
                let mut change_ids = control.track(Vec::with_capacity(count+1)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                if let Some(parent) = parent { for change in &fold.checkpoints[parent].change_ids { change_ids.push(copy_history_text(change, control).await?); } }
                change_ids.push(copy_history_text(&checkpoint.change_id, control).await?);
                let positions = control.track(owned_positions(&checkpoint.mutation_ids, &owners, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                let mut edit_ids = control.track(Vec::with_capacity(positions.len())).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                for position in positions.iter() { edit_ids.push(copy_history_text(&edits[*position].id, control).await?); }
                change_lookup.get_or_insert(copy_history_text(&checkpoint.change_id, control).await?, fold.changes.len());
                checkpoint_lookup.get_or_insert(copy_history_text(&checkpoint.checkpoint_id, control).await?, fold.checkpoints.len());
                let checkpoint_id = control.track(copy_history_text(&checkpoint.checkpoint_id, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                let line = checkpoint.line_id.as_ref().filter(|line| *line != &fold.trunk);
                match line {
                    Some(line) => { let index = *alternative_lookup.get(line).ok_or_else(|| fold_error(format!("commit {} names unknown alternative {line}", checkpoint.checkpoint_id)))?; fold.alternatives[index].checkpoint_ids.push(copy_history_text(&checkpoint_id, control).await?); }
                    None => trunk_chain.push(copy_history_text(&checkpoint_id, control).await?),
                }
                fold.changes.push(FoldChange { id: std::mem::take(&mut checkpoint.change_id), edit_ids: edit_ids.take(), description: checkpoint.description.take(), saved_at: std::mem::take(&mut checkpoint.saved_at) });
                fold.checkpoints.push(FoldCheckpoint { id: std::mem::take(&mut checkpoint.checkpoint_id), change_ids: change_ids.take(), parent_id: checkpoint.parent_id.take(), authors: std::mem::take(&mut checkpoint.authors), message: checkpoint.message.take(), timestamp: std::mem::take(&mut checkpoint.timestamp), pins: Vec::new() });
            }
            HistoryTransition::Branch { alternative_id, name, checkpoint_id } => {
                if *alternative_id == fold.trunk { return Err(fold_error(format!("branch claims the trunk alternative {alternative_id}"))); }
                if !checkpoint_lookup.contains_key(checkpoint_id) { return Err(fold_error(format!("branch names unknown checkpoint {checkpoint_id}"))); }
                alternative_lookup.get_or_insert(copy_history_text(alternative_id, control).await?, fold.alternatives.len());
                fold.alternatives.push(FoldAlternative { id: std::mem::take(alternative_id), name: std::mem::take(name), checkpoint_ids: vec![std::mem::take(checkpoint_id)] });
            }
            HistoryTransition::Checkout { checkpoint_id, .. } => { if !checkpoint_lookup.contains_key(checkpoint_id) { return Err(fold_error(format!("checkout names unknown checkpoint {checkpoint_id}"))); } }
            HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins } => {
                let index = *checkpoint_lookup.get(checkpoint_id).ok_or_else(|| fold_error(format!("repin names unknown checkpoint {checkpoint_id}")))?;
                control.track(std::mem::replace(&mut fold.checkpoints[index].id, copy_history_text(pinned_checkpoint_id, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                control.track(std::mem::replace(&mut fold.checkpoints[index].pins, std::mem::take(pins))).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                checkpoint_lookup.remove(checkpoint_id);
                checkpoint_lookup.insert(copy_history_text(pinned_checkpoint_id, control).await?, index);
                for alternative in &mut fold.alternatives { for id in &mut alternative.checkpoint_ids { control.pulse().await?; if *id == *checkpoint_id { control.track(std::mem::replace(id, copy_history_text(pinned_checkpoint_id, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; } } }
                for id in trunk_chain.iter_mut() { control.pulse().await?; if *id == *checkpoint_id { control.track(std::mem::replace(id, copy_history_text(pinned_checkpoint_id, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; } }
                if let Some(previous) = renamed.insert(std::mem::take(checkpoint_id), std::mem::take(pinned_checkpoint_id)) { control.track(previous).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
            }
            HistoryTransition::Supersede(supersede) => {
                for input in &supersede.inputs { control.pulse().await?; if !owners.contains_key(&input.target.0) { return Err(fold_error(format!("transition references unknown operation {}", input.target.0))); } }
                if supersede.scope.as_ref().is_none_or(|scope| *scope == head.line_id) {
                    for input in &mut supersede.inputs {
                        control.pulse().await?;
                        let actor = control.track(control.lease_actor(&event.actor.0).await).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                        let transition_id = control.track(copy_history_text(&event.mutation_id.0, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
                        let scope = match &supersede.scope { Some(scope) => Some(copy_history_text(scope, control).await?), None => None };
                        let target = MutationId(std::mem::take(&mut input.target.0));
                        let replacement = std::mem::replace(&mut input.replacement, InputReplacement::Withdrawn);
                        if let Some(previous) = fold.supersessions.insert(target, EffectiveSupersession { transition_id: transition_id.take(), actor: actor.take(), timestamp: event.timestamp, scope, replacement }) { control.track(previous).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
                    }
                }
            }
        }
    }
    let on_trunk = head.line_id == fold.trunk;
    let mut chain = control.track(Vec::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let source = if on_trunk { &*trunk_chain } else { let index = *alternative_lookup.get(&head.line_id).ok_or_else(|| fold_error(format!("viewer head names unknown alternative {}", head.line_id)))?; &fold.alternatives[index].checkpoint_ids };
    chain.reserve_exact(source.len());
    for id in source { chain.push(copy_history_text(id, control).await?); }
    let mut viewed = control.track(match &head.checkpoint_id { Some(id) => Some(copy_history_text(id, control).await?), None => None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if let Some(id) = viewed.as_mut() {
        let mut guard = 0;
        while let Some(next) = renamed.get(id) { control.pulse().await?; control.track(std::mem::replace(id, copy_history_text(next, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; guard += 1; if guard == 64 { return Err(fold_error("repin cycle")); } }
        let mut known = false;
        for candidate in chain.iter() { control.pulse().await?; known |= candidate == id; }
        if !known { return Err(fold_error(format!("viewer head names unknown checkpoint {id}"))); }
    }
    let at_tip = viewed.is_none();
    let checkpoint = match viewed.take() { Some(id) => Some(id), None => match chain.last() { Some(id) => Some(copy_history_text(id, control).await?), None => None } };
    let checkpoint = control.track(checkpoint).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut visible = control.track(HistoryFoldSet::<usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if let Some(id) = checkpoint.as_ref() {
        let position = *checkpoint_lookup.get(id).ok_or_else(|| fold_error(format!("viewer head names unknown checkpoint {id}")))?;
        for change_id in &fold.checkpoints[position].change_ids {
            control.pulse().await?;
            let change = *change_lookup.get(change_id).ok_or_else(|| fold_error(format!("checkpoint {id} names unknown change {change_id}")))?;
            for edit_id in &fold.changes[change].edit_ids { control.pulse().await?; if let Some(position) = edit_lookup.get(edit_id) { if active.contains(position) { visible.insert(*position); } } }
        }
    }
    if at_tip {
        let mut committed = control.track(HistoryFoldSet::<usize>::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        for change in &fold.changes { for edit_id in &change.edit_ids { control.pulse().await?; if let Some(position) = edit_lookup.get(edit_id) { committed.insert(*position); } } }
        for (position, edit) in edits.iter().enumerate() { control.pulse().await?; if !active.contains(&position) || committed.contains(&position) { continue; } let on_line = match &edit.line { Some(line) if line != &fold.trunk => line == &head.line_id, _ => on_trunk }; if on_line { visible.insert(position); } }
    }
    let mut ordered = control.track(HistoryFoldIndex::new()).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for position in visible.iter().copied() { let edit = &edits[position]; ordered.insert((edit.timestamp.cmp_key(), copy_history_text(&edit.id, control).await?), position); }
    while let Some((key, position)) = ordered.pop_first() { control.track(key).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; fold.applied.push(copy_history_text(&edits[position].id, control).await?); }
    for position in redo.values() { fold.redo.push(copy_history_text(&edits[*position].id, control).await?); }
    if !trunk_chain.is_empty() { let trunk = copy_history_text(&fold.trunk, control).await?; fold.alternatives.push(FoldAlternative { id: trunk, name: String::new(), checkpoint_ids: trunk_chain.take() }); for index in (1..fold.alternatives.len()).rev() { control.pulse().await?; fold.alternatives.swap(index, index-1); } }
    fold.checkpoint = checkpoint.take();
    fold.alternative = if on_trunk { None } else { Some(copy_history_text(&head.line_id, control).await?) };
    Ok(fold.take())
}

/// 📄️ Copies immutable protocol bytes through the same granted decoder pages.
pub async fn copy_history_bytes(bytes: &[u8], control: &HistoryFoldControl) -> Result<Vec<u8>, crate::ProtocolError> {
    let mut copied = control.track(Vec::with_capacity(bytes.len())).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let mut at = 0;
    control.pulse().await?;
    while at < bytes.len() { control.pulse().await?; let end = (at+control.page_bytes()).min(bytes.len()); copied.extend_from_slice(&bytes[at..end]); at = end; }
    Ok(copied.take())
}

/// 🪪️ Validates an opaque quarantined envelope while copying only its operation identity.
pub async fn history_envelope_id_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<MutationId, crate::ProtocolError> {
    let mut pos = 0;
    let identity = control.track(MutationId(read_fold_string(bytes, &mut pos, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "dependency count exceeds envelope")); }
    for _ in 0..count { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    match crate::wire::read_varint_u64(bytes, &mut pos)? { 0 => {}, 1 => { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }, flag => return Err(malformed(pos, format!("observed flag {flag}"))) }
    let count = crate::wire::read_varint_u64(bytes, &mut pos)?;
    if count > bytes.len().saturating_sub(pos) as u64 { return Err(malformed(pos, "target count exceeds envelope")); }
    for _ in 0..count { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    for _ in 0..2 {
        control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        control.pulse().await?;
        let length = crate::wire::read_varint_u64(bytes, &mut pos)?;
        pos = usize::try_from(length).ok().and_then(|length| pos.checked_add(length)).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(pos, "truncated envelope payload"))?;
    }
    control.pulse().await?;
    super::super::decode_hlc(bytes, &mut pos)?;
    let flags = crate::wire::read_varint_u64(bytes, &mut pos)?;
    if flags > 0b111 { return Err(malformed(pos, "invalid envelope flags")); }
    if flags & 1 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    if flags & 2 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    if flags & 4 != 0 { control.track(read_fold_string(bytes, &mut pos, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await; }
    if pos != bytes.len() { return Err(malformed(pos, "quarantined envelope has trailing bytes")); }
    Ok(identity.take())
}

async fn read_fold_blob(bytes: &[u8], position: &mut usize, control: &HistoryFoldControl) -> Result<Vec<u8>, crate::ProtocolError> {
    control.pulse().await?;
    let length = usize::try_from(crate::wire::read_varint_u64(bytes, position)?).map_err(|_| malformed(*position, "envelope payload length exceeds address space"))?;
    let end = position.checked_add(length).filter(|end| *end <= bytes.len()).ok_or_else(|| malformed(*position, "truncated envelope payload"))?;
    let copied = copy_history_bytes(&bytes[*position..end], control).await?;
    *position = end;
    Ok(copied)
}

/// 📦️ Decodes quarantined envelopes in byte pages, preserving every opaque payload and exact header fact.
pub async fn decode_history_envelope_controlled(bytes: &[u8], control: &HistoryFoldControl) -> Result<super::super::MutationEnvelope, crate::ProtocolError> {
    let mut position = 0;
    let mutation_id = control.track(MutationId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let document_id = control.track(ArtifactId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let original_actor=control.track(read_fold_string(bytes,&mut position,control).await?).unwrap_or_else(|_|unreachable!("fold String declares tracked original ownership")).await;
    let shared_actor=control.adopt_actor(original_actor).await;
    let actor=control.track(ActorId(shared_actor)).unwrap_or_else(|_|unreachable!("fold actor declares shared original ownership")).await;
    let dependencies = control.track(read_fold_ids(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.pulse().await?;
    let observed = control.track(match crate::wire::read_varint_u64(bytes, &mut position)? { 0 => None, 1 => Some(MutationId(read_fold_string(bytes, &mut position, control).await?)), other => return Err(malformed(position, format!("invalid observed flag {other}"))) }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let count = crate::wire::read_varint_u64(bytes, &mut position)?;
    if count > bytes.len().saturating_sub(position) as u64 { return Err(malformed(position, "target count exceeds envelope")); }
    let mut target = control.track(Vec::with_capacity(count as usize)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    for _ in 0..count { target.push(read_fold_string(bytes, &mut position, control).await?); }
    let diff_schema = control.track(SchemaId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let diff_payload = control.track(read_fold_blob(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let inverse_schema = control.track(SchemaId(read_fold_string(bytes, &mut position, control).await?)).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let inverse_payload = control.track(read_fold_blob(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    control.pulse().await?;
    let timestamp = super::super::decode_hlc(bytes, &mut position)?;
    let flags = crate::wire::read_varint_u64(bytes, &mut position)?;
    if flags > 0b111 { return Err(malformed(position, "invalid envelope flags")); }
    let transaction = control.track(if flags & 1 != 0 {
        let id = control.track(read_fold_string(bytes, &mut position, control).await?).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
        let tool = read_fold_string(bytes, &mut position, control).await?;
        Some(crate::TransactionRef { id: id.take(), tool })
    } else { None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let verb = control.track(if flags & 2 != 0 { Some(read_fold_string(bytes, &mut position, control).await?) } else { None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    let line = control.track(if flags & 4 != 0 { Some(read_fold_string(bytes, &mut position, control).await?) } else { None }).unwrap_or_else(|_| unreachable!("fold schema declares tracked ownership")).await;
    if position != bytes.len() { return Err(malformed(position, "quarantined envelope has trailing bytes")); }
    Ok(super::super::MutationEnvelope { mutation_id: mutation_id.take(), document_id: document_id.take(), actor: actor.take(), dependencies: dependencies.take(), observed: observed.take(), target: target.take(), diff: super::super::ArtifactDiff { schema: diff_schema.take(), payload: diff_payload.take() }, inverse: super::super::InverseMutation { schema: inverse_schema.take(), payload: inverse_payload.take() }, timestamp, transaction: transaction.take(), verb: verb.take(), line: line.take() })
}

#[cfg(test)]
include!("📏️retirement/🧪️tests/🦀️.rs");
