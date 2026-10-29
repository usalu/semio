//! 🪶️ The actual strong parent lends one native allocation witness while its Weak alias returns under the same caller.
use super::{StepContext,OriginalTurnFailure,OriginalRetainedLoanIdentity,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::{any::TypeId,cell::Cell,sync::{Arc,Weak,atomic::{AtomicU64,Ordering}}};
const NATIVE_WEAK_SCOPE_SLOTS:usize=128;
static NEXT_NATIVE_WEAK_SCOPE:AtomicU64=AtomicU64::new(1);
thread_local!{static NATIVE_WEAK_SCOPES:[Cell<u64>;NATIVE_WEAK_SCOPE_SLOTS]=const{[const{Cell::new(0)};NATIVE_WEAK_SCOPE_SLOTS]};}

#[derive(Clone,Copy)]
pub(super) struct OriginalNativeWeakParent{allocation:usize,source_type:TypeId,slot:usize,ticket:u64,identity:OriginalRetainedLoanIdentity,cancel:usize,epoch:Option<u64>}
impl OriginalNativeWeakParent{fn active(&self,cx:&StepContext<'_>)->bool{NATIVE_WEAK_SCOPES.with(|slots|slots[self.slot].get()==self.ticket)&&self.identity.matches(cx)&&self.cancel==cx.original_cancel_token()as*const _ as usize&&self.epoch==cx.source_epoch}}
struct OriginalNativeWeakLoan<'context>{target:*mut StepContext<'context>,previous:Option<OriginalNativeWeakParent>,identity:OriginalRetainedLoanIdentity,cancel:usize,epoch:Option<u64>,slot:usize,ticket:u64}
impl OriginalNativeWeakLoan<'_>{fn matches(&self)->bool{unsafe{let actual=&*self.target;self.identity.matches(actual)&&self.cancel==actual.original_cancel_token()as*const _ as usize&&self.epoch==actual.source_epoch}}}
impl Drop for OriginalNativeWeakLoan<'_>{fn drop(&mut self){NATIVE_WEAK_SCOPES.with(|slots|{assert_eq!(slots[self.slot].replace(0),self.ticket,"original scoped Weak ticket has one return")});let restore=if self.matches(){self.previous}else{None};unsafe{(*self.target).native_weak_parent=restore}}}
impl StepContext<'_>{
    /// 🧲️ Borrows a still-live typed native parent without a new alias, source identity, budget, clock or recipient.
    pub fn with_original_native_weak_parent<T:'static,R:Copy>(&mut self,parent:&Arc<T>,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        self.original_source_parent()?;
        let ticket=NEXT_NATIVE_WEAK_SCOPE.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original scoped Weak receiver epochs are exhausted"))?;
        let slot=NATIVE_WEAK_SCOPES.with(|slots|slots.iter().position(|slot|{if slot.get()!=0{return false}slot.set(ticket);true})).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original scoped Weak receiver slots are busy"))?;
        let restore=OriginalNativeWeakLoan{target:std::ptr::addr_of_mut!(*self),previous:self.native_weak_parent,identity:OriginalRetainedLoanIdentity::capture(self),cancel:self.original_cancel_token()as*const _ as usize,epoch:self.source_epoch,slot,ticket};
        self.native_weak_parent=Some(OriginalNativeWeakParent{allocation:Arc::as_ptr(parent)as usize,source_type:TypeId::of::<T>(),slot,ticket,identity:OriginalRetainedLoanIdentity::capture(self),cancel:self.original_cancel_token()as*const _ as usize,epoch:self.source_epoch});
        let result=run(self);if restore.matches(){return result}
        let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Weak return callback replaced its original initiating context");
        Err(match result{Ok(_)=>OriginalTurnFailure::from(refusal()),Err(mut original)=>{if original.context_refusal.is_none(){original.context_refusal=Some(refusal())}original}})
    }
    /// 🔎️ Lends the original still-live producer source without upgrading its Weak into another strong alias.
    pub fn with_original_native_weak_source<T:'static,R>(&mut self,original:&Weak<T>,run:impl FnOnce(&T,&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
        let parent=self.native_weak_parent.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native Weak has no scoped strong parent witness"))?;
        if !parent.active(self)||parent.source_type!=TypeId::of::<T>()||parent.allocation!=original.as_ptr()as usize{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native Weak requires its exact typed allocation parent").into())}
        let source=unsafe{&*(parent.allocation as*const T)};run(source,self)
    }
    /// 🧾️ Returns one matching native Weak alias while its actual strong parent keeps the allocation live.
    pub fn return_original_native_weak<T:'static>(&mut self,original:&mut Option<Weak<T>>)->Result<Option<RetainedCloneProgress>,OriginalTurnFailure>{
        let Some(source)=original.as_ref()else{return Ok(Some(Default::default()))};
        let parent=self.native_weak_parent.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native Weak has no scoped strong parent witness"))?;
        if !parent.active(self)||parent.source_type!=TypeId::of::<T>()||parent.allocation!=source.as_ptr()as usize{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native Weak requires its exact typed allocation parent").into())}
        let grant=self.retained_grant();if self.retirement_should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.consume_retained(progress)?;self.consume_fuel(1);drop(original.take());Ok(Some(progress))
    }
}
