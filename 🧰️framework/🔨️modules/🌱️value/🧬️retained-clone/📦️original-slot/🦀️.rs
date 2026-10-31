//! 📦️ Original native slots admit immutable leases and return the same owner through its captured issuer.
use crate::{ArtifactOwnedValueRetirementFactory,FactoryOwnedSourceAuthority,RetirementDemand,ValueError,ValueRefusalKind};
use super::{RetainedCloneSourceAdmission,RetainedCloneSource,RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneSourceTake,RetainedCloneStep,retained_clone_grant_admits};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
pub trait RetainedOriginalReadOwner<T>:Send+Sync {
 fn get(&self)->&T;
 fn matches_factory(&self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->bool;
 fn factory(&self)->&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>;
 fn borrow(&self)->Result<RetainedCloneRef<'_,T>,ValueError>;
 fn retain_original(&self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(RetainedOwnedProjection<T>,RetainedCloneProgress),ValueError>where T:Sync;
 fn birth_demands(&self)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,ValueError>;
 fn return_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn return_original(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedOriginalReadReturn<T>,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
pub enum RetainedOriginalReadReturn<T> {Pending(RetainedCloneProgress),Ready(T,Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,RetainedCloneProgress)}
pub struct RetainedOriginalRead<T:Send+Sync+'static> {
 staged:ManuallyDrop<Option<FactoryOwnedSourceAuthority<T>>>,
 admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<T,FactoryOwnedSourceAuthority<T>>>>,
 source:ManuallyDrop<Option<RetainedCloneSource<T,FactoryOwnedSourceAuthority<T>>>>,
 returning:bool,
}
fn fault(message:&'static str)->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn frame_copy<T:Send+Sync+'static>()->usize {2*size_of::<FactoryOwnedSourceAuthority<T>>()+size_of::<Box<dyn RetainedOriginalReadOwner<T>>>()}
impl<T:Send+Sync+'static> RetainedOriginalRead<T> {
 pub fn new(original:T,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->Self{Self{staged:ManuallyDrop::new(Some(FactoryOwnedSourceAuthority::new(original,factory))),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),returning:false}}
 pub fn initial_demands()->RetirementDemand {RetirementDemand{copy_bytes:frame_copy::<T>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 fn source_demands()->RetirementDemand {RetirementDemand{copy_bytes:2*size_of::<FactoryOwnedSourceAuthority<T>>()+RetainedCloneSource::<T,FactoryOwnedSourceAuthority<T>>::borrowed_constructor_copy_bytes::<FactoryOwnedSourceAuthority<T>>(),capacity_bytes:RetainedCloneSource::<T,FactoryOwnedSourceAuthority<T>>::borrowed_constructor_capacity_bytes::<FactoryOwnedSourceAuthority<T>>(),depth:1,..Default::default()}}
}
impl<T:Send+Sync+'static> RetainedOriginalReadOwner<T> for RetainedOriginalRead<T> {
 fn get(&self)->&T {if let Some(staged)=self.staged.as_ref(){staged.get()}else if let Some(admission)=self.admission.as_ref(){admission.original_authority().expect("original admission retains authority").get()}else{self.source.as_ref().expect("original slot source remains present").try_authority().expect("original slot authority remains readable during return").get()}}
 fn matches_factory(&self,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>)->bool {let actual=if let Some(staged)=self.staged.as_ref(){staged.factory()}else if let Some(admission)=self.admission.as_ref(){admission.original_authority().expect("original admission retains authority").factory()}else{self.source.as_ref().expect("original read source").try_authority().expect("original authority").factory()};Arc::ptr_eq(actual,factory)}
 fn factory(&self)->&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>{if let Some(staged)=self.staged.as_ref(){staged.factory()}else if let Some(admission)=self.admission.as_ref(){admission.original_authority().expect("original admission retains authority").factory()}else{self.source.as_ref().expect("original read source").try_authority().expect("original authority").factory()}}
 fn borrow(&self)->Result<RetainedCloneRef<'_,T>,ValueError> {if self.returning{return Err(fault("original slot source is returning"));}let source=self.source.as_ref().ok_or_else(||fault("original slot source birth is incomplete"))?;Ok(source.borrow())}
 fn retain_original(&self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(RetainedOwnedProjection<T>,RetainedCloneProgress),ValueError>{let mut work=0;let result=(||{if self.returning{return Err(fault("original slot source is returning"));}{let turn=self.source.as_ref().ok_or_else(||fault("original slot source birth is incomplete"))?.retain_original(grant);work=turn.work;turn.result}})();crate::RetainedWorkOutcome{result,work}}
 fn birth_demands(&self)->Result<RetirementDemand,ValueError> {if self.returning{return Err(fault("returning original slot source cannot birth another read"))}if self.source.is_some(){return Ok(Default::default())}if let Some(admission)=self.admission.as_ref(){return admission.next_demand()}Ok(RetirementDemand{depth:1,..Default::default()})}
 fn advance(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,ValueError>{
  let demand=match self.birth_demands(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if self.source.is_some(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}if !retained_clone_grant_admits(grant,demand){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.admission.is_none(){let Some(staged)=self.staged.take()else{return crate::RetainedWorkOutcome{result:Err(fault("slot source lost its original staged authority")),work:0}};*self.admission=Some(RetainedCloneSource::admission_borrowed(staged,FactoryOwnedSourceAuthority::get));return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
  let admission=self.admission.as_mut().unwrap();if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,grant);if admission.terminal_is_empty(){self.admission.take();}return turn}let turn=admission.advance(grant);crate::RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}
 }
 fn return_demands(&self,body:usize)->Result<RetirementDemand,ValueError> {
  if !self.returning{return Ok(RetirementDemand{depth:1,..Default::default()});}
  if let Some(admission)=self.admission.as_ref(){return admission.next_return_demand()}
  if self.staged.is_some(){return Ok(RetirementDemand{copy_bytes:FactoryOwnedSourceAuthority::<T>::transfer_copy_byte_demand(),release_bytes:size_of::<Self>(),depth:1,..Default::default()});}
  self.source.as_ref().ok_or_else(||fault("slot return lost its exact source"))?.next_take_demand(body)
 }
 fn return_original(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedOriginalReadReturn<T>,ValueError> {let mut work=0;let result=(||{
  let demand=self.return_demands(grant.maximum_copy_bytes)?;
  if !retained_clone_grant_admits(grant,demand){return Ok(RetainedOriginalReadReturn::Pending(Default::default()));}
  if !self.returning{self.returning=true;work=1;return Ok(RetainedOriginalReadReturn::Pending(RetainedCloneProgress{copied_items:1,..Default::default()}));}
  if let Some(admission)=self.admission.as_mut(){if admission.phase()=="ready"{let turn=admission.take_output(&mut self.source,grant);if admission.terminal_is_empty(){self.admission.take();}work=turn.work;return turn.result.map(|step|RetainedOriginalReadReturn::Pending(step.progress()));}let mut returned=None;let turn=admission.return_inputs(&mut returned,grant);if let Some((staged,()))=returned{*self.staged=Some(staged);}if admission.terminal_is_empty(){self.admission.take();}work=turn.work;return turn.result.map(|step|RetainedOriginalReadReturn::Pending(step.progress()));}

  if let Some(staged)=self.staged.take(){work=1;return match staged.take_original(grant){Ok((original,factory,mut receipt))=>{receipt.released_bytes=size_of::<Self>();Ok(RetainedOriginalReadReturn::Ready(original,factory,receipt))},Err((error,staged))=>{*self.staged=Some(staged);Err(error)}};}
  let source=self.source.as_mut().ok_or_else(||fault("slot return source disappeared"))?;
  match {let turn=source.take_authority(grant);work=turn.work;turn.result}?{
   RetainedCloneSourceTake::Pending(receipt)=>Ok(RetainedOriginalReadReturn::Pending(receipt)),
   RetainedCloneSourceTake::Ready(staged,receipt)=>{*self.staged=Some(staged);*self.source=None;Ok(RetainedOriginalReadReturn::Pending(receipt))}
  }
 })();crate::RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool {self.staged.is_none()&&self.admission.is_none()&&self.source.is_none()}
}
impl<T:Send+Sync+'static> Drop for RetainedOriginalRead<T> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"slot source reached Drop before original owner and issuer returned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.staged);ManuallyDrop::drop(&mut self.admission);ManuallyDrop::drop(&mut self.source);}}}}


/// 🪪️ Identifies the actual original slot and its current authored generation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RetainedOriginalSlotKey{issuance:u64,generation:u64}
/// 🗃️ Holds one native original; writes and transfer fence until every immutable reader returns.
pub struct RetainedOriginalSlot<T>{original:Option<T>,read:Option<Box<dyn RetainedOriginalReadOwner<T>>>,issuance:u64,generation:u64}
static ORIGINAL_SLOT_ISSUANCE:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
impl<T> RetainedOriginalSlot<T>{
 pub fn new(original:T)->Self{let mut slot=Self::vacant();slot.original=Some(original);slot}
 /// 🪹️ Admits a fresh sealed resident address without inventing a semantic payload.
 pub fn vacant()->Self{Self{original:None,read:None,issuance:ORIGINAL_SLOT_ISSUANCE.fetch_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|issuance|issuance.checked_add(1)).expect("original slot issuance exhausted"),generation:0}}
 /// 🪟️ Exclusive mutation borrows are available only while the original slot has no immutable source loan.
 pub fn get_mut_unissued(&mut self)->Option<&mut T>{if self.read.is_some(){None}else{self.original.as_mut()}}
 pub fn get(&self)->&T{self.original.as_ref().unwrap_or_else(||self.read.as_ref().expect("original slot retains value or source").get())}
 /// 🪹️ A withdrawn resident retains its sealed address without a fabricated payload.
 pub fn is_empty(&self)->bool{self.original.is_none()&&self.read.is_none()}
 /// 📏️ Quotes the exact payload and slot-generation moves for an original handoff.
 pub fn transfer_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>()+size_of::<Option<T>>()+size_of::<u64>(),depth:1,..Default::default()}}
 /// 📤️ Withdraws the same original after every reader returns, preserving the resident slot identity.
 pub fn take_original(&mut self,key:RetainedOriginalSlotKey,output:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{self.admit_key(key)?;if self.read.is_some()||self.original.is_none()||output.is_some(){return Err(fault("original slot withdrawal requires its unique payload and empty recipient"));}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let d=Self::transfer_demand();if !retained_clone_grant_admits(grant,d){return Ok(None)}*output=self.original.take();self.generation=generation;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 /// 📥️ Adopts an actual original into its vacant resident without allocating an address or cloning a payload.
 pub fn install_original(&mut self,key:RetainedOriginalSlotKey,input:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{self.admit_key(key)?;if !self.is_empty()||input.is_none(){return Err(fault("original slot installation requires its vacant resident and original input"));}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let d=Self::transfer_demand();if !retained_clone_grant_admits(grant,d){return Ok(None)}self.original=input.take();self.generation=generation;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn key(&self)->RetainedOriginalSlotKey{RetainedOriginalSlotKey{issuance:self.issuance,generation:self.generation}}
 fn admit_key(&self,key:RetainedOriginalSlotKey)->Result<(),ValueError>{if self.key()==key{Ok(())}else{Err(fault("original slot read belongs to another source generation"))}}
 pub fn has_read(&self)->bool{self.read.is_some()}
 pub fn try_into_original(mut self)->Result<T,Self>{if self.read.is_some()||self.original.is_none(){return Err(self);}Ok(self.original.take().unwrap())}
 /// 📏️ Quotes only original field exchange; variable payload allocations keep their native owners.
 pub fn replace_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>()+size_of::<Option<T>>()+size_of::<u64>(),depth:1,..Default::default()}}
 /// 🧮️ Quotes one exclusive ownership transform and its caller-owned result without duplicating a payload.
 pub fn transform_demand<R>()->RetirementDemand{RetirementDemand{copy_bytes:3*size_of::<T>()+size_of::<Option<T>>()+2*size_of::<R>()+size_of::<u64>(),depth:1,..Default::default()}}
 /// 🔀️ A pre-admitted infallible move transforms one sealed native row after every immutable reader returns.
 pub fn transform_original<R>(&mut self,key:RetainedOriginalSlotKey,grant:RetainedCloneGrant,transform:impl FnOnce(Option<T>)->(Option<T>,R))->Result<Option<(R,RetainedCloneProgress)>,ValueError>{self.admit_key(key)?;if self.read.is_some(){return Err(fault("original slot transform requires every reader returned"))}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let demand=Self::transform_demand::<R>();if grant.maximum_items==0||!retained_clone_grant_admits(grant,demand){return Ok(None)}let(next,result)=transform(self.original.take());self.original=next;self.generation=generation;Ok(Some((result,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 /// 🔁️ Exchanges actual native fields after all readers return and every original currency is admitted.
 pub fn replace(&mut self,original:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{if self.read.is_some(){return Err(fault("original slot mutation requires every reader returned"));}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let d=Self::replace_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}let next=original.take().ok_or_else(||fault("original slot replacement absent"))?;let previous=self.original.replace(next).expect("owned original slot value");self.generation=generation;Ok(Some((previous,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn demands(&self,key:RetainedOriginalSlotKey)->Result<RetirementDemand,ValueError>where T:Send+Sync+'static{self.admit_key(key)?;self.read.as_ref().map_or(Ok(RetainedOriginalRead::<T>::initial_demands()),|read|read.birth_demands())}
 pub fn advance_read(&mut self,key:RetainedOriginalSlotKey,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,ValueError>where T:Send+Sync+'static{let mut work=0;let result=(||{
  let d=self.demands(key)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(read)=self.read.as_mut(){if !read.matches_factory(factory){return Err(fault("original slot issuer differs from captured authority"));}let turn=read.advance(grant);work=turn.work;return turn.result;}
  if self.original.is_none(){return Err(fault("original slot value absent"));}let layout=std::alloc::Layout::new::<RetainedOriginalRead<T>>();work=1;let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<RetainedOriginalRead<T>>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original read frame allocation failed"));};let original=self.original.take().unwrap();self.read=Some(unsafe{pointer.as_ptr().write(RetainedOriginalRead::new(original,Arc::clone(factory)));Box::from_raw(pointer.as_ptr())});Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))
 })();crate::RetainedWorkOutcome{result,work}}
 pub fn borrow(&self,key:RetainedOriginalSlotKey)->Result<RetainedCloneRef<'_,T>,ValueError>{self.admit_key(key)?;self.read.as_ref().ok_or_else(||fault("original slot source birth not admitted"))?.borrow()}
 /// 🎟️ Retains the exact admitted original source after validating its current slot key.
 pub fn retain_read(&self,key:RetainedOriginalSlotKey,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(RetainedOwnedProjection<T>,RetainedCloneProgress),ValueError>where T:Sync{let mut work=0;let result=(||{self.admit_key(key)?;{let turn=self.read.as_ref().ok_or_else(||fault("original slot source birth not admitted"))?.retain_original(grant);work=turn.work;turn.result}})();crate::RetainedWorkOutcome{result,work}}
 pub fn return_demands(&self,key:RetainedOriginalSlotKey,body:usize)->Result<RetirementDemand,ValueError>{self.admit_key(key)?;self.read.as_ref().map_or(Ok(Default::default()),|read|read.return_demands(body))}
 pub fn return_read(&mut self,key:RetainedOriginalSlotKey,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,ValueError>{let mut work=0;let result=(||{self.admit_key(key)?;if factory.is_some(){return Err(fault("original slot returned issuer output already occupied"));}let Some(read)=self.read.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};match {let turn=read.return_original(grant);work=turn.work;turn.result}?{RetainedOriginalReadReturn::Pending(p)=>Ok(RetainedCloneStep::Progress(p)),RetainedOriginalReadReturn::Ready(original,issuer,p)=>{if !read.terminal_is_empty(){return Err(fault("original slot return omitted terminal custody"));}self.original=Some(original);*factory=Some(issuer);self.read=None;Ok(RetainedCloneStep::Complete(p))}}})();crate::RetainedWorkOutcome{result,work}}
}
impl<T> Drop for RetainedOriginalSlot<T>{fn drop(&mut self){assert!(self.read.is_none()||std::thread::panicking(),"original slot dropped with immutable readers");}}

impl<T:std::fmt::Debug> std::fmt::Debug for RetainedOriginalSlot<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{if self.is_empty(){f.write_str("Vacant")}else{self.get().fmt(f)}}}
impl<T:PartialEq> PartialEq for RetainedOriginalSlot<T>{fn eq(&self,other:&Self)->bool{match(self.is_empty(),other.is_empty()){(true,true)=>true,(false,false)=>self.get()==other.get(),_=>false}}}
impl<T:Eq> Eq for RetainedOriginalSlot<T>{}

#[path="♻️retirement/🦀️.rs"]
mod retirement;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
