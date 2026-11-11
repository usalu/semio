//! 🎮️ Admitted typed-owner retirement with a native paged cursor frontier.

use super::{RetireOwned, RetirementCursor, RetirementStep};
use crate::{ValueError, ValueRefusalKind, list::PagedList, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::ManuallyDrop;


thread_local! { static ABANDONING: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
/// 🛑️ True while an explicit abandon releases nested owners whose close protocol can no longer run, so their terminal asserts stand down.
pub(crate) fn abandoning() -> bool { ABANDONING.with(|depth| depth.get() > 0) }

/// 🚦️ Keeps retained-owner exclusion inline and refuses contention without parking or allocating.
pub struct RetainedOwnerGate<T> { occupied:std::sync::atomic::AtomicBool,value:std::cell::UnsafeCell<T> }
unsafe impl<T:Send> Send for RetainedOwnerGate<T> {}
unsafe impl<T:Send> Sync for RetainedOwnerGate<T> {}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct RetainedOwnerGateBusy;
pub struct RetainedOwnerGateGuard<'a,T> {gate:&'a RetainedOwnerGate<T>,exclusive:std::marker::PhantomData<&'a mut T>}
impl<T> RetainedOwnerGate<T> {
    pub const fn new(value:T)->Self {Self{occupied:std::sync::atomic::AtomicBool::new(false),value:std::cell::UnsafeCell::new(value)}}
    pub fn try_lock(&self)->Result<RetainedOwnerGateGuard<'_,T>,RetainedOwnerGateBusy> {
        self.occupied.compare_exchange(false,true,std::sync::atomic::Ordering::Acquire,std::sync::atomic::Ordering::Relaxed).map(|_|RetainedOwnerGateGuard{gate:self,exclusive:std::marker::PhantomData}).map_err(|_|RetainedOwnerGateBusy)
    }
    /// 🪪️ Checks the actual borrowed guard without exposing the protected owner.
    pub fn owns_guard(&self,guard:&RetainedOwnerGateGuard<'_,T>)->bool{std::ptr::eq(self,guard.gate)}
    pub fn get_mut(&mut self)->&mut T {self.value.get_mut()}
    pub fn into_inner(self)->T {self.value.into_inner()}
}
impl<T> std::ops::Deref for RetainedOwnerGateGuard<'_,T> {type Target=T;fn deref(&self)->&T {unsafe{&*self.gate.value.get()}}}
impl<T> std::ops::DerefMut for RetainedOwnerGateGuard<'_,T> {fn deref_mut(&mut self)->&mut T {unsafe{&mut*self.gate.value.get()}}}
impl<T> Drop for RetainedOwnerGateGuard<'_,T> {fn drop(&mut self){self.gate.occupied.store(false,std::sync::atomic::Ordering::Release)}}

/// 🏛️ Erases only the owner type while preserving each independent retirement authority.
pub trait ErasedControlledRetirement: Send {
    /// 📥️ Lends the original caller reply into this actual controlled owner.
    fn step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
    /// 🔁️ Recovers the same original child without replay.
    fn recover_step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
    fn step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;
    fn recover_step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>;

    fn requires_original_birth(&self)->bool;
    fn step_original_birth(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll;
    fn requires_original_control(&self)->bool;
    fn step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll;
    fn step_original(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll;
    fn next_demand(&self, maximum_body_bytes: usize) -> Result<crate::RetirementDemand, ValueError>;
    fn step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll;
    fn step_progress(&self) -> RetainedCloneProgress;
    fn last_turn_work(&self)->u128;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn frame_release_bytes(&self) -> usize;
}

/// 📏️ Borrows the exact frame allocation before original typed ownership transfers.
pub const fn controlled_retirement_birth_bytes<T: RetireOwned>() -> usize { size_of::<ControlledRetirement<T>>() }

/// 🎟️ Admits the exact typed frame before allocation and returns original ownership on refusal.
pub fn admit_controlled_retirement<T:RetireOwned>(value:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<dyn ErasedControlledRetirement>,RetainedCloneProgress)>,ValueError>{let turn=admit_typed_controlled_retirement(value,effect);crate::RetainedWorkOutcome{result:turn.result.map(|owner|owner.map(|(owner,progress)|(owner as Box<dyn ErasedControlledRetirement>,progress))),work:turn.work}}

pub fn controlled_retirement_birth_demand<T:RetireOwned>(value:&T)->Result<crate::RetirementDemand,ValueError>{if !T::controlled_retirement_supported()||!value.retirement_ready(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"typed retirement owner is not ready"))}Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<ControlledRetirement<T>>(),capacity_bytes:size_of::<ControlledRetirement<T>>(),depth:1,..Default::default()})}
mod empty_owner{
 use std::mem::size_of;
 pub trait Sealed:Sized{fn take_empty(&mut self)->Self;fn already_empty(&self)->bool;}
 impl<T> Sealed for Vec<T>{fn take_empty(&mut self)->Self{std::mem::take(self)}fn already_empty(&self)->bool{self.is_empty()&&(size_of::<T>()==0||self.capacity()==0)}}
 impl<T,const N:usize> Sealed for crate::list::PagedList<T,N>{fn take_empty(&mut self)->Self{std::mem::take(self)}fn already_empty(&self)->bool{self.terminal_is_empty()}}
}
/// 🥡️ Defining empty collections replace their original headers without arbitrary default callbacks.
pub trait RetainedEmptyOwner:RetireOwned+empty_owner::Sealed{}
impl<T:RetireOwned> RetainedEmptyOwner for Vec<T>{}
impl<T:RetireOwned,const N:usize> RetainedEmptyOwner for crate::list::PagedList<T,N>{}
fn admit_frame<T:RetireOwned>(demand:crate::RetirementDemand,effect:&mut crate::RetainedAllocationEffect<'_>,take:impl FnOnce()->T)->crate::RetainedWorkOutcome<Option<(Box<ControlledRetirement<T>>,RetainedCloneProgress)>,ValueError>{if effect.maximum_work_units()==0||effect.grant().maximum_items==0||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}if effect.actual().is_some(){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"typed frame allocator marker occupied")),work:0}}
 let work=match super::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return crate::RetainedWorkOutcome{result:Err(error),work}};let layout=std::alloc::Layout::new::<ControlledRetirement<T>>();*effect.actual()=Some(0);let raw=unsafe{std::alloc::alloc(layout)};if !raw.is_null(){*effect.actual()=Some(layout.size())}let Some(pointer)=std::ptr::NonNull::new(raw.cast::<ControlledRetirement<T>>())else{return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"typed retirement frame allocation failed")),work}};
 let owner=unsafe{pointer.as_ptr().write(ControlledRetirement::retain(take()));Box::from_raw(pointer.as_ptr())};crate::RetainedWorkOutcome{result:Ok(Some((owner,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),released_bytes:0}))),work}
}
pub fn admit_typed_controlled_retirement<T:RetireOwned>(value:&mut Option<T>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<ControlledRetirement<T>>,RetainedCloneProgress)>,ValueError>{let Some(original)=value.as_ref()else{return crate::RetainedWorkOutcome{result:Ok(None),work:0}};let demand=match controlled_retirement_birth_demand(original){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};admit_frame(demand,effect,||value.take().expect("admitted typed frame source"))}
/// 👁️ Reads only the defining collection's empty-header witness.
pub fn empty_retirement_is_terminal<T:RetainedEmptyOwner>(value:&T)->bool{empty_owner::Sealed::already_empty(value)}
pub fn admit_empty_controlled_retirement<T:RetainedEmptyOwner>(value:&mut T,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<(Box<ControlledRetirement<T>>,RetainedCloneProgress)>,ValueError>{if empty_owner::Sealed::already_empty(value){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}let mut demand=match controlled_retirement_birth_demand(value){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};demand.copy_bytes=size_of::<T>()+size_of::<ControlledRetirement<T>>();admit_frame(demand,effect,||empty_owner::Sealed::take_empty(value))}

impl<T: RetireOwned> crate::ErasedSnapshotRetirement for ControlledRetirement<T> {
    fn native_header_release(&self)->Option<crate::RetirementHeaderRelease>{self.terminal_is_empty().then(||unsafe{crate::RetirementHeaderRelease::native(self,0,0)})}
 fn close_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_into(body,effect,reply)}
 fn recover_close_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.recover_step_into(body,effect,reply)}
 fn close_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_original_into(original,body,effect,reply)}
 fn recover_close_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.recover_step_original_into(original,body,effect,reply)}
    fn requires_original_birth(&self)->bool{ControlledRetirement::requires_original_birth(self)}
    fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{ControlledRetirement::step_original_birth(self,original,grant)}
    fn requires_original_control(&self)->bool{ControlledRetirement::requires_original_control(self)}
    fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll{let grant=effect.grant();if !self.requires_original_control(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"controlled frontier has no selected original child").into()),work:0})}self.step_with(body,effect,|cursor,effect,_reply,_selected,_work|cursor.close_step_controlled(body,effect,control))}
    fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();if !self.requires_original_control(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"controlled frontier has no selected original child").into()),work:0})}self.step_with(body,effect,|cursor,effect,_reply,_selected,_work|cursor.close_original_step(original,body,effect))}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{ControlledRetirement::next_computational_work(self,body)}
    fn next_demand(&self, body: usize) -> Result<crate::RetirementDemand, ValueError> { ControlledRetirement::next_demand(self, body) }
    fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll {self.step(body,effect)}
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Self::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Self::next_capacity_byte_demand(self, copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Self::next_depth_demand(self) }

    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
}

pub struct ControlledRetirement<T: RetireOwned> {
    value: ManuallyDrop<Option<T>>,
    cursors: ManuallyDrop<PagedList<Box<dyn RetirementCursor>, {usize::MAX}>>,
    pending_cursor:ManuallyDrop<Option<Box<dyn RetirementCursor>>>,
    step_progress: RetainedCloneProgress,
    last_work:u128,
    child_reply:ManuallyDrop<Option<Result<RetirementStep,crate::OriginalRetirementFailure>>>,
    close_invocation:Option<semio_framework_value_control_contract::OriginalEffectInvocationIdentity>,
    close_body:Option<usize>,
    close_original:Option<(crate::NativeAllocationIdentity,Option<u64>)>,
    child_work:Option<(crate::OriginalWorkActionBoundary,u64)>,
    close_selected:Option<(crate::RetirementDemand,u64)>,
    pending_child_grant:Option<RetainedCloneGrant>,
}

impl<T: RetireOwned> ControlledRetirement<T> {
    /// 🆕️ Retains the input inline without constructing a retirement scaffold.
    pub fn new(value: T) -> Result<Self, (ValueError, T)> {
        if !T::controlled_retirement_supported() { return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner, "typed owner has no controlled retirement authority"), value)); }
        Ok(Self::retain(value))
    }

    /// 🫴️ Keeps an unfinished original inline so unsupported retirement can refuse without losing custody.
    pub fn retain(value:T)->Self {Self{value:ManuallyDrop::new(Some(value)),cursors:ManuallyDrop::new(PagedList::with_page_items(8)),pending_cursor:ManuallyDrop::new(None),step_progress:RetainedCloneProgress::default(),last_work:0,child_reply:ManuallyDrop::new(None),close_invocation:None,close_body:None,close_original:None,child_work:None,close_selected:None,pending_child_grant:None}}

    /// 👁️ Borrows the original typed owner before retirement cursor construction.
    pub fn original(&self) -> Option<&T> { self.value.as_ref() }
    /// 🔒️ Quotes whether the original can transfer before any retirement scaffold is retained.
    pub fn original_is_untouched(&self) -> bool { self.value.is_some() && self.pending_cursor.is_none() && self.cursors.terminal_is_empty() }
    /// ✍️ Mutates the original typed owner before retirement cursor construction.
    pub fn original_mut(&mut self) -> Option<&mut T> { self.value.as_mut() }
    /// 🎁️ Transfers an untouched original while retaining its empty admitted frame.
    pub fn take_original(&mut self) -> Option<T> { if self.pending_cursor.is_none()&&self.cursors.terminal_is_empty() { self.value.take() } else { None } }

    /// 🪑️ Reports whether the next cursor push already owns its reserved slot.
    pub fn has_reserved_slot(&self) -> bool { self.cursors.has_reserved_slot() }
    pub fn terminal_is_empty(&self) -> bool { self.value.is_none() && self.pending_cursor.is_none() && self.cursors.terminal_is_empty() }
    /// 🛑️ Releases a close that can no longer progress: the memory is dropped, never leaked, and the unbalanced retirement is the caller's typed refusal to report.
    pub fn abandon(mut self) {
        ABANDONING.with(|depth| depth.set(depth.get() + 1));
        let released = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { ManuallyDrop::drop(&mut self.value); ManuallyDrop::drop(&mut self.pending_cursor); ManuallyDrop::drop(&mut self.cursors); }));
        ABANDONING.with(|depth| depth.set(depth.get() - 1));
        std::mem::forget(self);
        drop(released);
    }

    fn cursor_reserve_demand(&self)->Result<crate::RetirementDemand,ValueError>{Ok(crate::RetirementDemand{copy_bytes:self.cursors.next_reserve_copy_byte_demand().map_err(ValueError::from)?,capacity_bytes:self.cursors.next_allocation_bytes().map_err(ValueError::from)?,depth:self.cursors.next_reserve_depth_demand().map_err(ValueError::from)?,..Default::default()})}

    /// 🔂️ Visits only the original active child once and preserves one actual body hint.
    pub fn next_demand(&self, body:usize)->Result<crate::RetirementDemand,ValueError>{
        use crate::RetirementDemand;
        if self.pending_child_grant.is_some(){return self.close_selected.map(|selected|selected.0).ok_or_else(||ValueError::invariant("controlled recovery lost its original quote"))}
        if self.value.as_ref().is_some_and(|value|!value.retirement_ready()){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original owner requires recovery before retirement"))}
        if self.value.is_some()&&!T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"typed owner has no controlled retirement authority"))}
        if self.terminal_is_empty(){return Ok(Default::default());}
        if self.pending_cursor.is_some(){return Ok(if self.cursors.has_reserved_slot(){RetirementDemand{copy_bytes:2*size_of::<Option<Box<dyn RetirementCursor>>>()+size_of::<usize>(),depth:1,..Default::default()}}else{self.cursor_reserve_demand()?})}
        if let Some(value)=self.value.as_ref(){if !self.cursors.has_reserved_slot(){return Ok(self.cursor_reserve_demand()?)}return super::retirement_cursor_birth_demand(value);}
        if self.cursors.is_empty(){return Ok(RetirementDemand{copy_bytes:self.cursors.next_release_copy_byte_demand().map_err(ValueError::from)?,release_bytes:self.cursors.next_release_allocation_bytes().map_err(ValueError::from)?,depth:self.cursors.next_release_depth_demand().map_err(ValueError::from)?,..Default::default()});}
        let cursor=self.cursors.get(self.cursors.len()-1).unwrap();
        if cursor.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:2*size_of::<usize>()+size_of::<Box<dyn RetirementCursor>>(),release_bytes:cursor.terminal_release_bytes().ok_or_else(||refusal("retirement owner has no controlled terminal release authority"))?,depth:1,..Default::default()});}
        let mut d=cursor.next_demand(body)?;d.depth=d.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retirement frontier depth overflow"))?;
        if !self.cursors.has_reserved_slot(){return Ok(self.cursor_reserve_demand()?);}Ok(d)
    }

    /// ⛽️ Quotes the selected native retirement operation or its original child.
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{
        self.next_demand(body)?;if self.pending_child_grant.is_some(){return self.close_selected.map(|selected|selected.1).ok_or_else(||ValueError::invariant("controlled recovery lost its Work quote"))}if self.terminal_is_empty(){return Ok(0)}
        if self.pending_cursor.is_some()||self.value.is_some()||self.cursors.is_empty()||!self.cursors.has_reserved_slot(){return Ok(1)}
        let cursor=self.cursors.get(self.cursors.len()-1).ok_or_else(||ValueError::invariant("retirement cursor absent"))?;
        if cursor.terminal_is_empty(){Ok(1)}else{cursor.next_computational_work(body)}
    }

    /// 🧩️ Borrows all currencies from the same selected source, structural birth, payload or physical release event.
    pub fn selected_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{self.next_demand(body)}

    /// 🧷️ Loans the original typed frontier while its complete selected quote is admitted or refused.
    pub fn select(&mut self,body:usize)->Result<crate::RetirementSelection<'_,Self>,ValueError>{crate::select_retirement_cursor(self,body)}

    /// 🧮️ Borrows the active payload work without charging future constructor movement as input copying.
    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {Ok(self.next_demand(self.close_body.unwrap_or(0))?.copy_bytes)}

    /// ⏭️ Exposes the next complete constructor allocation before the caller grants it.
    pub fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> {Ok(self.next_demand(maximum_body_bytes)?.capacity_bytes)}

    /// 📦️ Exposes the next complete physical release before the caller grants it.
    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {Ok(self.next_demand(self.close_body.unwrap_or(0))?.release_bytes)}

    /// 🪆️ Borrows the per-turn nesting depth of the top cursor; the explicit paged frontier is heap-iterative and never counts as call depth.
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> {Ok(self.next_demand(self.close_body.unwrap_or(0))?.depth)}

    pub fn step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{self.step_with(body,effect,|cursor,effect,reply,_selected,_work|{let before=effect.performed_boundary();match borrowed_child_step(cursor,body,effect,reply,None,false){Ok(Some(step))=>step,Ok(None)=>RetirementStep::Pending(effect.performed_since(before).expect("same controlled receipt")),Err(error)=>RetirementStep::Failure(error.into(),effect.performed_since(before).expect("same controlled receipt"))}})}
    fn requires_original_control(&self)->bool{if self.close_invocation.is_some(){return self.close_original.is_some()}self.value.is_none()&&self.cursors.has_reserved_slot()&&self.cursors.get(self.cursors.len().saturating_sub(1)).is_some_and(|cursor|!cursor.terminal_is_empty()&&cursor.requires_original_control())}
    fn step_with(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut dyn RetirementCursor,&mut crate::RetainedAllocationEffect<'_>,&mut Option<Result<RetirementStep,crate::OriginalRetirementFailure>>,&mut Option<RetainedCloneGrant>,&mut Option<(crate::OriginalWorkActionBoundary,u64)>)->RetirementStep)->crate::OriginalRetirementPoll{
        self.step_progress=Default::default();self.last_work=0;
        let result=(||{
          if self.value.as_ref().is_some_and(|value|!value.retirement_ready()){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original owner requires recovery before retirement").into())}
          if self.value.is_some()&&!T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"typed owner has no controlled retirement authority").into())}
          if self.next_computational_work(body)?>effect.maximum_work_units(){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
          self.step_original(body,effect,invoke)
        })();
        self.finish_step_result(result)
    }



    /// 📬️ Drives one defining child producer while its actual reply stays in this frontier.
    pub fn step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_reply(None,body,effect,reply,false)}
    /// 🔁️ Recovers the same original action and any reply recorded before unwind.
    pub fn recover_step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_reply(None,body,effect,reply,true)}
    pub fn step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_reply(Some(original),body,effect,reply,false)}
    pub fn recover_step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.step_reply(Some(original),body,effect,reply,true)}

    fn step_reply<'reply>(&mut self,mut original:Option<&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
        if let Some(original)=original.as_deref_mut(){if !original.original_work().binding_identity()?.matches_effect(effect){return Err(ValueError::invariant("controlled original loan changed its actual Work action"))}}
        let body=if recover{self.close_body.ok_or_else(||ValueError::invariant("controlled close recovery lost its selected body"))?}else{body};
        let selected=if recover{None}else{Some((self.next_demand(body)?,self.next_computational_work(body)?))};
        let original_binding=original.as_ref().map(|loan|(loan.ledger_identity(),loan.retirement_recipient_identity()));
        let invocation=effect.invocation().ok_or_else(||ValueError::invariant("controlled close requires its original producer invocation"))?.identity();
        if recover{if self.close_invocation!=Some(invocation)||self.close_original!=original_binding{return Err(ValueError::invariant("controlled close recovery changed its original invocation"))}}else{if self.close_invocation.is_some(){return Err(ValueError::invariant("controlled close already retains an invocation"))}self.close_invocation=Some(invocation);self.close_body=Some(body);self.close_original=original_binding;self.close_selected=selected}
        let boundary=effect.performed_boundary();
        let turn=if let Some(grant)=self.pending_child_grant{
            let index=self.cursors.len().checked_sub(1).ok_or_else(||ValueError::invariant("recovering controlled child lost its resident cursor"))?;
            let (start,maximum)=self.child_work.ok_or_else(||ValueError::invariant("controlled child lost its selected Work boundary"))?;let spent=effect.performed_since_action(start)?;let remaining=u128::from(maximum).checked_sub(spent).ok_or_else(||ValueError::invariant("controlled child exceeded its selected Work"))?;let remaining=u64::try_from(remaining).map_err(|_|ValueError::invariant("controlled child Work does not fit"))?;let quote=self.close_selected.ok_or_else(||ValueError::invariant("controlled recovery lost its original quote"))?.0;if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),quote)||remaining>effect.remaining_work_units()?{self.last_work=0;return Ok(reply.pending())}let mut child=effect.reborrow(grant.restricted_by(effect.grant()),remaining)?;
            let step=borrowed_child_step(self.cursors.get_mut(index).expect("same resident controlled child").as_mut(),body,&mut child,&mut self.child_reply,original.as_deref_mut(),true)?;
            match step{None=>crate::RetainedWorkPoll::Pending{work:effect.performed_since(boundary)?},Some(step)=>{self.pending_child_grant=None;self.child_work=None;let result=self.process_child_step(index,step,effect.grant(),grant,grant.maximum_capacity_bytes);self.finish_step_result(result)}}
        }else{
            let mut protocol_failure=None;
            let turn=self.step_with(body,effect,|cursor,effect,slot,selected,selected_work|{
                *selected=Some(effect.grant());*selected_work=Some((effect.action_boundary(),effect.maximum_work_units()));let boundary=effect.performed_boundary();
                match borrowed_child_step(cursor,body,effect,slot,original.as_deref_mut(),false){Ok(Some(step))=>{*selected=None;*selected_work=None;step},Ok(None)=>RetirementStep::Pending(effect.performed_since(boundary).expect("same child receipt boundary")),Err(error)=>{protocol_failure=Some(error);RetirementStep::Pending(effect.performed_since(boundary).expect("same child receipt boundary"))}}
            });
            if let Some(error)=protocol_failure{return Err(error)}turn
        };
        let work=effect.performed_since(boundary)?;self.last_work=work;
        match turn{crate::RetainedWorkPoll::Pending{..}=>Ok(reply.pending()),crate::RetainedWorkPoll::Complete(turn)=>{self.close_invocation=None;self.close_body=None;self.close_original=None;self.close_selected=None;Ok(reply.complete(turn.result))}}
    }

    /// ⏲️ Actual work of the most recent invoked turn, including genuine failed actions.
    pub fn last_turn_work(&self)->u128{self.last_work}

    /// 🧾️ The physical receipt of the last original turn, including a retained allocation on failure.
    pub fn step_progress(&self) -> RetainedCloneProgress { self.step_progress }

    /// 🎰️ Reserves one frontier cursor slot through the caller's reservation strategy, recording the physical receipt of the turn.
    pub fn reserve_cursor_slot(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reserve:impl FnOnce(&mut PagedList<Box<dyn RetirementCursor>,{usize::MAX}>,&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::list::PagedListReservationOutcome,crate::list::PagedListReservationFailure>)->Result<Option<RetainedCloneStep>,ValueError>{
        self.last_work=0;self.step_progress=Default::default();let before=effect.performed_work();let turn=reserve(&mut self.cursors,effect);self.last_work=effect.performed_work().checked_sub(before).ok_or_else(||ValueError::invariant("original frontier work moved backwards"))?;match turn.result{
              Ok(crate::list::PagedListReservationOutcome::Denied)=>{if turn.work!=self.last_work{return Err(ValueError::invariant("frontier denial work differs from its original receipt"))}Ok(Some(RetainedCloneStep::Progress(Default::default())))},
              Ok(crate::list::PagedListReservationOutcome::Ready(step))=>{if turn.work!=self.last_work{return Err(ValueError::invariant("frontier work differs from its original receipt"))}self.step_progress=RetainedCloneProgress{copied_items:usize::from(step.progressed),copied_bytes:step.placed_bytes,retained_capacity_bytes:step.allocated_bytes,..Default::default()};if u128::from(step.work)!=self.last_work{return Err(ValueError::invariant("frontier reservation work differs from its original receipt").with_retained_progress(self.step_progress))}Ok(Some(RetainedCloneStep::Progress(self.step_progress)))},
              Err(crate::list::PagedListReservationFailure::Allocation{error,copied_bytes})=>{self.step_progress=RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0||copied_bytes!=0),copied_bytes,retained_capacity_bytes:error.allocated_bytes,..Default::default()};Err(ValueError::from(error.refusal()).with_retained_progress(self.step_progress))},
              Err(crate::list::PagedListReservationFailure::Work(error))=>Err(error),
        }
    }

    fn step_original(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut dyn RetirementCursor,&mut crate::RetainedAllocationEffect<'_>,&mut Option<Result<RetirementStep,crate::OriginalRetirementFailure>>,&mut Option<RetainedCloneGrant>,&mut Option<(crate::OriginalWorkActionBoundary,u64)>)->RetirementStep)->Result<Option<RetainedCloneStep>,crate::OriginalRetirementFailure>{
        let grant=effect.grant();
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(Some(RetainedCloneStep::Complete(empty))); }
        if grant.maximum_items == 0 { return Ok(Some(RetainedCloneStep::Progress(empty))); }
        let demand=self.next_demand(body)?;
        if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(empty)))}
        if self.pending_cursor.is_some(){if !self.cursors.has_reserved_slot(){return self.reserve_cursor_slot(effect,PagedList::reserve_one).map_err(Into::into)}if grant.maximum_copy_bytes<demand.copy_bytes{return Ok(Some(RetainedCloneStep::Progress(empty)))}self.last_work=match super::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>{self.last_work=work;return Err(error.into())}};let cursor=self.pending_cursor.take().expect("resident completed cursor");self.cursors.push_reserved(cursor).map_err(|_|refusal("retirement pending cursor lost its reserved publication slot"))?;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
        if self.value.is_none() && self.cursors.is_empty() {
            let before=effect.performed_work();let turn=self.cursors.release_empty_page(effect);self.last_work=effect.performed_work().checked_sub(before).ok_or_else(||ValueError::invariant("original frontier release work moved backwards"))?;let reported=turn.work;let step=match turn.result{Ok(crate::list::PagedListReleaseOutcome::Denied)=>{if reported!=self.last_work{return Err(ValueError::invariant("frontier release denial work differs from its original receipt").into())}return Ok(Some(RetainedCloneStep::Progress(Default::default())))},Ok(crate::list::PagedListReleaseOutcome::Ready(step))=>{if reported!=self.last_work{return Err(ValueError::invariant("frontier release work differs from its original receipt").into())}step},Err(crate::list::PagedListReleaseFailure::Work(error))=>return Err(error.into()),Err(crate::list::PagedListReleaseFailure::Release(error))=>return Err(ValueError::from(error).into())};if u128::from(step.work)!=self.last_work{return Err(ValueError::invariant("frontier release work differs from its original receipt").into())}return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),copied_bytes:step.placed_bytes,retained_capacity_bytes:0,released_bytes:step.released_allocation_bytes})));
        }
        if self.value.is_none() && self.cursors.get(self.cursors.len() - 1).is_some_and(|cursor| cursor.terminal_is_empty()) {
            let bytes = self.cursors.get(self.cursors.len() - 1).and_then(|cursor| cursor.terminal_release_bytes()).ok_or_else(|| refusal("retirement terminal scaffold has no controlled release authority"))?;
            if bytes > grant.maximum_release_bytes||demand.copy_bytes>grant.maximum_copy_bytes { return Ok(Some(RetainedCloneStep::Progress(empty))); }
            self.last_work=match super::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>{self.last_work=work;return Err(error.into())}};
            drop(self.cursors.pop());
            return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: 0, released_bytes: bytes })));
        }
        if !self.cursors.has_reserved_slot() {
            let demand = self.cursors.next_allocation_bytes().map_err(ValueError::from)?;
            if demand > grant.maximum_capacity_bytes { return Ok(Some(RetainedCloneStep::Progress(empty))); }
            return self.reserve_cursor_slot(effect,PagedList::reserve_one).map_err(Into::into);
        }
        if self.value.is_some(){let turn=super::admit_retirement_cursor(&mut self.value,effect);self.last_work=turn.work;return match turn.result{Ok(Some((cursor,progress)))=>{*self.pending_cursor=Some(cursor);Ok(Some(RetainedCloneStep::Progress(progress)))},Ok(None)=>Ok(Some(RetainedCloneStep::Progress(empty))),Err(error)=>Err(error.into())}}
        let index = self.cursors.len() - 1;
        let minimum_work_bytes = demand.copy_bytes;
        let work = minimum_work_bytes != 0;
        let body_bytes = grant.maximum_copy_bytes;
        let birth = demand.capacity_bytes;
        if work && body_bytes < minimum_work_bytes && (birth==0 || !self.cursors.get(index).unwrap().allows_admitted_narrow_work()) { return Ok(Some(RetainedCloneStep::Progress(empty))); }
        if birth > grant.maximum_capacity_bytes { return Ok(Some(RetainedCloneStep::Progress(empty))); }
        let cursor_grant = RetainedCloneGrant { maximum_capacity_bytes: birth, maximum_depth: grant.maximum_depth - 1, ..grant };
        let cursor=self.cursors.get_mut(index).ok_or_else(||ValueError::invariant("get_mut target absent"))?;let maximum_work=cursor.next_computational_work(body)?;let mut child=effect.reborrow(cursor_grant,maximum_work)?;let step=invoke(cursor.as_mut(),&mut child,&mut self.child_reply,&mut self.pending_child_grant,&mut self.child_work);
        self.process_child_step(index,step,grant,cursor_grant,birth)
    }
    fn process_child_step(&mut self,index:usize,step:RetirementStep,grant:RetainedCloneGrant,cursor_grant:RetainedCloneGrant,birth:usize)->Result<Option<RetainedCloneStep>,crate::OriginalRetirementFailure>{
        self.last_work=step.work();
        let (processed, bytes, progressed) = match step {
            RetirementStep::Pending(_) => return Ok(None),
            RetirementStep::Progress(progress,_) => {
                self.step_progress = progress;
                if !progress.fits(cursor_grant) || (progress.copied_items!=0 && progress.retained_capacity_bytes!=birth) { return Err(refusal("typed retirement receipt exceeded or changed its declared full grant").into()); }
                return Ok(Some(RetainedCloneStep::Progress(progress)));
            }
            RetirementStep::Child(child) => { self.cursors.push_reserved(child).map_err(|_| refusal("retirement child lost its admitted frontier slot"))?; (0, 0, true) }
            RetirementStep::ChildProgress(child,progress,_)=>{self.step_progress=progress;self.cursors.push_reserved(child).map_err(|_|refusal("original child lost its admitted frontier slot"))?;if !progress.fits(cursor_grant)||progress.copied_items!=1||progress.retained_capacity_bytes!=birth{return Err(refusal("original child receipt differs from its admitted physical transfer").into());}return Ok(Some(RetainedCloneStep::Progress(progress)));}
            RetirementStep::Advanced => (0, 0, true),
            RetirementStep::Failure(error,_) => return Err(error),
            RetirementStep::Bytes(bytes) if bytes <= grant.maximum_release_bytes => (0, bytes, true),
            RetirementStep::Bytes(_) => return Err(refusal("retirement body exceeded its admitted release grant").into()),
            RetirementStep::ProcessedBytes(bytes) if bytes <= grant.maximum_copy_bytes => (bytes, 0, true),
            RetirementStep::ProcessedBytes(_) => return Err(refusal("retirement body exceeded its admitted work grant").into()),
            RetirementStep::Complete(_) if self.cursors.get(index).ok_or_else(||ValueError::invariant("get target absent"))?.terminal_is_empty() => (0, 0, true),
            RetirementStep::Complete(_) => return Err(refusal("retirement body has no terminal-empty witness").into()),
            RetirementStep::BudgetExhausted => (0, 0, false),
        };
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(progressed), copied_bytes: processed, retained_capacity_bytes: if progressed { birth } else { 0 }, released_bytes: bytes })))
    }
    /// 👶️ Selects only the live frontier cursor that already owns the original birth.
    pub fn requires_original_birth(&self)->bool{self.close_invocation.is_none()&&self.value.is_none()&&self.pending_cursor.is_none()&&self.cursors.has_reserved_slot()&&self.cursors.len().checked_sub(1).and_then(|index|self.cursors.get(index)).is_some_and(|cursor|!cursor.terminal_is_empty()&&cursor.requires_original_birth())}
    /// 🧵️ Drives the selected birth directly through the original loan before any ordinary effect.
    pub fn step_original_birth(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{
      self.step_progress=Default::default();self.last_work=0;let result=(||{if !self.requires_original_birth(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"controlled frontier has no selected original birth").into())}if grant.maximum_items==0{return if original.failure_storage_birth().terminal_is_empty(){Ok(Some(RetainedCloneStep::Progress(Default::default())))}else{Ok(None)}}let demand=self.next_demand(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"controlled birth exceeds its admitted depth").into())}if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes{return if original.failure_storage_birth().terminal_is_empty(){Ok(Some(RetainedCloneStep::Progress(Default::default())))}else{Ok(None)}}let index=self.cursors.len()-1;let child_grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:grant.maximum_depth-1,..grant};let step=self.cursors.get_mut(index).ok_or_else(||ValueError::invariant("original birth cursor absent"))?.close_original_birth_step(original,child_grant);self.process_child_step(index,step,grant,child_grant,demand.capacity_bytes)})();self.finish_step_result(result)
    }
    fn finish_step_result(&mut self,result:Result<Option<RetainedCloneStep>,crate::OriginalRetirementFailure>)->crate::OriginalRetirementPoll{match result{Ok(None)=>crate::RetainedWorkPoll::Pending{work:self.last_work},Ok(Some(step))=>{self.step_progress=step.progress();crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(step),work:self.last_work})},Err(error)=>{if error.retained_progress()!=Default::default(){self.step_progress=error.retained_progress()}crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.with_retained_progress(self.step_progress)),work:self.last_work})}}}

}


fn borrowed_child_step(cursor:&mut dyn RetirementCursor,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,slot:&mut Option<Result<RetirementStep,crate::OriginalRetirementFailure>>,original:Option<&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>>,recover:bool)->Result<Option<RetirementStep>,ValueError>{
 if slot.is_none(){
  let token=semio_framework_value_control_contract::OriginalEffectReply::borrow(slot)?;
  let maximum=effect.remaining_work_units()?;let complete=match original{Some(original)=>original.with_work_limit(maximum,|original|if recover{cursor.recover_close_original_into(original,body,effect,token)}else{cursor.close_original_into(original,body,effect,token)})?,None=>if recover{cursor.recover_close_into(body,effect,token)}else{cursor.close_into(body,effect,token)}}?;drop(complete);
 }
 Ok(slot.take().map(|result|match result{Ok(step)=>step,Err(error)=>RetirementStep::Failure(error,0)}))
}

impl<T: RetireOwned> Drop for ControlledRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "controlled retirement abandoned before terminal-empty");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.value); ManuallyDrop::drop(&mut self.pending_cursor);ManuallyDrop::drop(&mut self.cursors);ManuallyDrop::drop(&mut self.child_reply); } }
    }
}

impl<T: RetireOwned> ErasedControlledRetirement for ControlledRetirement<T> {
 fn step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{ControlledRetirement::step_into(self,body,effect,reply)}
 fn recover_step_into<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{ControlledRetirement::recover_step_into(self,body,effect,reply)}
 fn step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{ControlledRetirement::step_original_into(self,original,body,effect,reply)}
 fn recover_step_original_into<'reply>(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'reply,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{ControlledRetirement::recover_step_original_into(self,original,body,effect,reply)}

    fn requires_original_birth(&self)->bool{ControlledRetirement::requires_original_birth(self)}
    fn step_original_birth(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{ControlledRetirement::step_original_birth(self,original,grant)}
    fn requires_original_control(&self)->bool{ControlledRetirement::requires_original_control(self)}
    fn step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll{let grant=effect.grant();crate::ErasedSnapshotRetirement::close_step_controlled(self,body,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"),control)}
    fn step_original(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();crate::ErasedSnapshotRetirement::close_original_step(self,original,body,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
    fn next_demand(&self, body: usize) -> Result<crate::RetirementDemand, ValueError> { ControlledRetirement::next_demand(self, body) }
    fn step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{ControlledRetirement::step(self,body,effect)}
    fn step_progress(&self) -> RetainedCloneProgress { ControlledRetirement::step_progress(self) }
    fn last_turn_work(&self)->u128{ControlledRetirement::last_turn_work(self)}
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { ControlledRetirement::next_capacity_byte_demand(self, maximum_body_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_release_byte_demand(self) }
    fn terminal_is_empty(&self) -> bool { ControlledRetirement::terminal_is_empty(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_depth_demand(self) }
    fn frame_release_bytes(&self) -> usize { size_of::<Self>() }
}

fn refusal(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }

impl<T: RetireOwned> RetireOwned for ControlledRetirement<T> {
    fn retirement_ready(&self)->bool{self.value.as_ref().is_none_or(RetireOwned::retirement_ready)}
    type RetirementCursor=Self;
    fn retirement_seed(self)->Self::RetirementCursor{self}
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
    fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}

impl<T: RetireOwned> RetirementCursor for ControlledRetirement<T> {
    native_retirement_reply!();
    fn requires_original_control(&self)->bool{ControlledRetirement::requires_original_control(self)}
    fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(crate::ErasedSnapshotRetirement::close_step_controlled(self,body,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"),control))}
    fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(crate::ErasedSnapshotRetirement::close_original_step(self,original,body,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect")))}
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{ControlledRetirement::next_computational_work(self,body)}
    fn next_demand(&self, body: usize) -> Result<crate::RetirementDemand, ValueError> { ControlledRetirement::next_demand(self, body) }
    
    fn terminal_is_empty(&self) -> bool { ControlledRetirement::terminal_is_empty(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_depth_demand(self) }
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
    fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError> {Ok(self.selected_demand(body)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_close_byte_demand(&self) -> Option<usize> { self.next_release_byte_demand().ok() }
    fn next_birth_bytes(&self, maximum_bytes: usize) -> Option<usize> { self.next_capacity_byte_demand(maximum_bytes).ok() }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}
impl<T: RetireOwned> ControlledRetirement<T> {
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{super::erased_cursor_turn(self.step(body,effect))}
}





