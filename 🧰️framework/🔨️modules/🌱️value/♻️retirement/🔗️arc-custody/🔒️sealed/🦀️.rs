//! 🔒️ Shared immutable values expose strong handles and retain their original issuer.
pub const ORIGINAL_TRANSFER_WORK:u64=1;
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{ErasedSnapshotRetirement,RetireOwned,retirement,factory_ticket_demands,close_factory_ticket};
use retirement::{RetirementCursor,RetirementStep};
use std::{mem::ManuallyDrop,sync::atomic::{AtomicPtr,AtomicU8,AtomicU64,Ordering}};
use retirement::arc_custody::original::{SharedOriginal,SharedOriginalStorage,SharedOriginalRelease,SharedOriginalBirth};

static NEXT_ISSUANCE:AtomicU64=AtomicU64::new(1);

#[cfg(test)]
#[path="🚚️frontier/🧪️tests/🦀️.rs"]
mod frontier_tests;
/// 🎟️ The original producer supplies admission and demand functions without an erased allocation.
enum OriginalDemand{Native,Shared}
enum OriginalSlot<'a,T:Send+Sync+'static>{Native(&'a mut Option<T>),Shared(&'a mut Option<SharedOriginal<T>>)}
pub struct SharedIssuer<T:Send+Sync+'static>{pub transfer_copy_bytes:fn(&T)->usize,ready:fn(&T)->bool,demand:fn(&T,OriginalDemand)->Result<Option<RetirementDemand>,ValueError>,admit:fn(OriginalSlot<'_,T>,&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>>}
impl<T:Send+Sync+'static> Copy for SharedIssuer<T>{}
impl<T:Send+Sync+'static> Clone for SharedIssuer<T>{fn clone(&self)->Self{*self}}
impl<T:RetireOwned+Sync> SharedIssuer<T>{
 pub fn owned()->Self{Self{transfer_copy_bytes:|_|size_of::<T>(),ready:RetireOwned::retirement_ready,demand:|value,mode|match mode{OriginalDemand::Native=>native_payload_demand(value).map(Some),OriginalDemand::Shared=>{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no controlled shared issuer"))}value.shared_original_demands()}},admit:|slot,effect|match slot{OriginalSlot::Shared(slot)=>T::shared_original_retirement(slot,effect),OriginalSlot::Native(slot)=>semio_framework_value_retirement::admit_retirement_cursor(slot,effect)}}}

}

fn native_payload_demand<T:RetireOwned>(value:&T)->Result<RetirementDemand,ValueError>{semio_framework_value_retirement::retirement_cursor_birth_demand(value)}


/// ❄️ An exclusive cold payload can release its entire original custody without allocation.
///
/// Implementations must make the exclusivity check pure and release unique payloads without allocation, refusal or panic.
pub trait ColdRetire:RetireOwned+Sync{
 fn cold_exclusive(&self)->bool;
 fn cold_release(self);
}

impl ColdRetire for u64{fn cold_exclusive(&self)->bool{true}fn cold_release(self){}}

/// 📤️ Releases one already verified exclusive original under its exact handback grant.
///
/// The caller must keep the original initialized and physically exclusive until this function returns.
pub fn cold_take_exclusive<T:RetireOwned+Sync>(value:SealedShared<T>)->T{
 let demand=match value.original_transfer_demand(){Ok(demand)=>demand,Err(_)=>unreachable!("verified original remains initialized")};
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
 match value.try_unwrap(grant).result{Ok((value,_))=>value,Err(_)=>unreachable!("verified original remains physically exclusive")}
}

/// 🔐️ The private Arc has no raw, Weak, mutable or consuming escape hatch.
pub struct SealedShared<T:Send+Sync+'static>{source:ManuallyDrop<Option<SharedOriginalStorage<T>>>,issuer:SharedIssuer<T>,issuance:u64}
impl<T:std::fmt::Debug+Send+Sync+'static> std::fmt::Debug for SealedShared<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{self.get().fmt(f)}}
impl<T:PartialEq+Send+Sync+'static> PartialEq for SealedShared<T>{fn eq(&self,other:&Self)->bool{self.get()==other.get()}}
impl<T:Eq+Send+Sync+'static> Eq for SealedShared<T>{}

/// 🟢️ Retains one real empty shared backing until funded publication or cancellation.
pub struct SealedSharedBirth<T:Send+Sync+'static>{issuance:u64,storage:SharedOriginalBirth<T>}
/// 📤️ A current item/depth grant permits one allocation-free sealed publication.
pub struct SealedSharedPublication<'a,T:Send+Sync+'static>{issuance:u64,storage:retirement::arc_custody::original::SharedOriginalPublication<'a,T>}
impl<T:Send+Sync+'static> SealedSharedPublication<'_,T>{pub fn commit(self,value:T,issuer:SharedIssuer<T>)->RetainedWorkOutcome<(SealedShared<T>,RetainedCloneProgress)>{let turn=self.storage.commit(value);RetainedWorkOutcome{result:turn.result.map(|(source,p)|(SealedShared{source:ManuallyDrop::new(Some(source)),issuer,issuance:self.issuance},p)),work:turn.work}}}
impl<T:Send+Sync+'static> SealedSharedBirth<T>{
 pub fn publication(&mut self,grant:RetainedCloneGrant)->Result<Option<SealedSharedPublication<'_,T>>,ValueError>{let issuance=self.issuance;self.storage.publication(grant).map(|p|p.map(|storage|SealedSharedPublication{issuance,storage}))}
 pub const fn publication_work_demand()->u64{retirement::arc_custody::original::SharedOriginalPublication::<T>::work_demand()}
 pub fn next_close_demand(&self)->RetirementDemand{self.storage.next_close_demand()}
 pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{self.storage.close_step(effect.grant())}
 pub fn terminal_is_empty(&self)->bool{self.storage.terminal_is_empty()}
}
/// 📥️ One admitted original alias has one fixed issuer return slot without heap backing.
pub struct SharedAliasReturn<T:Send+Sync+'static>{source:AtomicPtr<()>,state:AtomicU8,identity:usize,issuer:SharedIssuer<T>,issuance:u64}
impl<T:Send+Sync+'static> SharedAliasReturn<T>{
 pub fn drain(&self,grant:RetainedCloneGrant)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{
  if self.state.load(Ordering::Acquire)!=2{return Ok(None);}
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<SealedShared<T>>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original returned alias transfer is not funded"));}
  let pointer=self.source.swap(std::ptr::null_mut(),Ordering::AcqRel);if pointer.is_null(){return Ok(None);}
  let handle=SealedShared{source:ManuallyDrop::new(Some(unsafe{SharedOriginalStorage::from_raw(pointer.cast_const())})),issuer:self.issuer,issuance:self.issuance};
  self.state.store(3,Ordering::Release);
  Ok(Some((handle,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<SealedShared<T>>(),..Default::default()})))
 }
 pub fn retire_unused(&self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"unused original slot retirement is not funded"));}self.state.compare_exchange(0,3,Ordering::AcqRel,Ordering::Acquire).map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"original slot has already entered handback"))?;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}
 pub fn terminal_is_empty(&self)->bool{self.state.load(Ordering::Acquire)==3}
}
impl<T:Send+Sync+'static> Drop for SharedAliasReturn<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"issuer slot retains an original returned alias");}}
impl<T:Send+Sync+'static> SealedShared<T>{
 pub fn birth_bytes()->usize{SharedOriginalStorage::<T>::allocation_bytes()}
 pub fn admission_demand()->Result<RetirementDemand,ValueError>{SharedOriginalStorage::<T>::value_admission_demand()}
 pub fn prepare_birth_demand()->Result<RetirementDemand,ValueError>{SharedOriginalStorage::<T>::prepare_birth_demand()}
 pub fn publication_demand()->RetirementDemand{SharedOriginalBirth::<T>::publication_demand()}
 /// ⛽️ Quotes the issuance action and the whole prepared birth.
 pub const fn birth_work_demand()->u64{1+Self::prepare_birth_work_demand()}
 pub const fn source_admission_work_demand()->u64{SharedOriginalStorage::<T>::source_admission_work_demand()}
 pub const fn prepare_birth_work_demand()->u64{SharedOriginalStorage::<T>::prepare_birth_work_demand()}
 pub const fn duplicate_alias_work_demand()->u64{SharedOriginalStorage::<T>::duplicate_work_demand()}
 pub fn prepare_birth(grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<(SealedSharedBirth<T>,RetainedCloneProgress)>{
  if actual.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("sealed birth requires an empty actual marker")),work:0}}
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<Self::birth_bytes(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed issuance requires genuine header birth authority")),work:0}}
  let issuance=match NEXT_ISSUANCE.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)){Ok(issuance)=>issuance,Err(_)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed original issuance space exhausted")),work:1}};
  let turn=SharedOriginalStorage::prepare_birth(grant,actual);RetainedWorkOutcome{result:turn.result.map(|(storage,p)|(SealedSharedBirth{issuance,storage},p)),work:1+turn.work}
 }
 pub fn admit(value:T,issuer:SharedIssuer<T>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<(Self,RetainedCloneProgress),(ValueError,T)>{
  if actual.is_some(){return RetainedWorkOutcome{result:Err((ValueError::invariant("sealed admission requires an empty actual marker"),value)),work:0}}
  let refusal=if grant.maximum_items==0{Some(ValueRefusalKind::WorkLimit)}else if grant.maximum_depth==0{Some(ValueRefusalKind::DepthLimit)}else if grant.maximum_capacity_bytes<Self::birth_bytes(){Some(ValueRefusalKind::OwnershipLimit)}else{None};
  if let Some(kind)=refusal{return RetainedWorkOutcome{result:Err((ValueError::literal(kind,"sealed shared source birth is not funded"),value)),work:0}}
  let issuance=match NEXT_ISSUANCE.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)){Ok(issuance)=>issuance,Err(_)=>return RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed original issuance space exhausted"),value)),work:1}};
  let turn=SharedOriginalStorage::admit(value,grant,actual);
  RetainedWorkOutcome{result:turn.result.map(|(source,progress)|(Self{source:ManuallyDrop::new(Some(source)),issuer,issuance},progress)),work:1+turn.work}
 }

 /// ⛽️ Borrows the actual original storage alias work before duplication.
 pub fn duplicate_work_demand(&self)->Result<u64,ValueError>{self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"sealed source has already transferred"))?;Ok(SharedOriginalStorage::<T>::duplicate_work_demand())}
 pub fn duplicate_copy_bytes()->usize{SharedOriginalStorage::<T>::duplicate_demand().copy_bytes+size_of::<Self>()}
 pub fn try_duplicate(&self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{
  let Some(source)=self.source.as_ref()else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"sealed source has already transferred")),work:0}};let grant=effect.grant();let copy=Self::duplicate_copy_bytes();
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||effect.maximum_work_units()<1{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed alias handoff is not funded")),work:0}}
  let before=effect.performed_work();if let Err((error,work))=retirement::record_retirement_work(effect,SharedOriginalStorage::<T>::duplicate_work_demand()){return RetainedWorkOutcome{result:Err(error),work}}
  let turn=source.duplicate(grant);RetainedWorkOutcome{result:turn.result.map(|source|(Self{source:ManuallyDrop::new(Some(source)),issuer:self.issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})),work:effect.performed_work()-before}
 }

 /// 🪢️ Returns one paid alias while the matching original strong parent remains borrowed and live.
 /// 📏️ Quotes the actual live alias-header clear before its original return.
 pub fn borrowed_alias_return_demand(&self)->RetirementDemand{if self.source.is_none(){Default::default()}else{RetirementDemand{copy_bytes:size_of::<Option<SharedOriginalStorage<T>>>(),depth:1,..Default::default()}}}
 pub fn return_borrowed_alias(&mut self,witness:&Self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{let mut work=0;let result=(||{
  let Some(original)=self.source.as_ref()else{return Ok(Default::default())};
  let Some(parent)=witness.source.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original sealed parent has already transferred"))};
  if self.issuance!=witness.issuance||original.identity()!=parent.identity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed strong parent does not witness the original sealed alias"))}
  let demand=self.borrowed_alias_return_demand();if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(Default::default())}
  work=1;let Some(source)=self.source.take()else{unreachable!("borrowed original presence checked")};match source.retire(){SharedOriginalRelease::Alias=>{},SharedOriginalRelease::Original(_)=>unreachable!("live borrowed parent retains the original")};Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
 })();RetainedWorkOutcome{result,work}}
 pub fn terminal_is_empty(&self)->bool{self.source.is_none()}
 /// 🕊️ Observes the genuine strong source after all issued aliases have returned.
 pub fn original_aliases_are_returned(&self)->bool{self.source.as_ref().is_none_or(|source|source.aliases()==1)}
 /// 🔍️ A live original is exclusive only when logical and physical strong counts both equal one.
 pub fn original_is_exclusive(&self)->bool{self.source.as_ref().is_some_and(|source|source.physically_unique())}
 pub fn return_slot(&self,grant:RetainedCloneGrant)->Result<(SharedAliasReturn<T>,RetainedCloneProgress),ValueError>{
  if self.source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"transferred alias cannot issue a return slot"));}
  if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<SharedAliasReturn<T>>()||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixed original return slot is not funded"));}
  Ok((SharedAliasReturn{source:AtomicPtr::new(std::ptr::null_mut()),state:AtomicU8::new(0),identity:self.identity(),issuer:self.issuer,issuance:self.issuance},RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<SharedAliasReturn<T>>(),..Default::default()}))
 }
 pub fn return_alias(mut self,slot:&SharedAliasReturn<T>)->Result<(),Self>{
  if self.identity()!=slot.identity||self.issuance!=slot.issuance||slot.state.compare_exchange(0,1,Ordering::AcqRel,Ordering::Acquire).is_err(){return Err(self);}
  let pointer=self.source.take().unwrap().into_raw().cast_mut();
  slot.source.store(pointer,Ordering::Release);slot.state.store(2,Ordering::Release);
  Ok(())
 }
 /// 📤️ Transfers the same original value after one funded atomic strong-only unwrap.
 pub fn original_transfer_demand(&self)->Result<RetirementDemand,ValueError>{let source=self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original sealed value already transferred"))?;Ok(RetirementDemand{copy_bytes:(self.issuer.transfer_copy_bytes)(source.get()),release_bytes:Self::birth_bytes(),depth:1,..Default::default()})}
/// 🧮️ The transfer quote and the actual unique-owner primitive share one declared work unit.
 pub fn original_transfer_work_demand(&self)->Result<u64,ValueError>{self.original_transfer_demand()?;Ok(ORIGINAL_TRANSFER_WORK)}
 pub fn try_unwrap(mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<(T,RetainedCloneProgress),(ValueError,Self)>{let mut work=0;let result=(||{
  let demand=match self.original_transfer_demand(){Ok(demand)=>demand,Err(error)=>return Err((error,self))};
  if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original sealed value transfer is not funded"),self));}
  let Some(source)=self.source.take()else{return Err((ValueError::literal(ValueRefusalKind::InvalidValue,"original sealed value already transferred"),self));};
  work=ORIGINAL_TRANSFER_WORK;match source.try_owner(){Ok(mut owner)=>{let value=owner.take_value().expect("unique initialized original handback");Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))},Err(source)=>{*self.source=Some(source);Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original sealed value still has live projections"),self))}}
 })();RetainedWorkOutcome{result,work:u128::from(work)}}
 pub fn identity(&self)->usize{self.source.as_ref().map_or(0,|source|source.identity())}
 pub fn get(&self)->&T{self.source.as_ref().expect("sealed shared source has transferred").get()}
}
/// 🔗️ Cloning adds one counted alias without allocating or failing; a transferred handle clones to another transferred handle.
impl<T:Send+Sync+'static> Clone for SealedShared<T>{fn clone(&self)->Self{Self{source:ManuallyDrop::new(self.source.as_ref().map(SharedOriginalStorage::alias)),issuer:self.issuer,issuance:self.issuance}}}
impl<T:Send+Sync+'static> std::ops::Deref for SealedShared<T>{type Target=T;fn deref(&self)->&T{self.get()}}
/// 🕊️ Dropping a non-last alias only decrements the shared count; the last owner must retire explicitly, which is a debug-time law and leaks (never panics) in production.
impl<T:Send+Sync+'static> Drop for SealedShared<T>{fn drop(&mut self){if let Some(source)=self.source.take(){if let SharedOriginalRelease::Original(owner)=source.retire(){debug_assert!(std::thread::panicking(),"sealed source requires original granted handback");owner.abandon();}}unsafe{ManuallyDrop::drop(&mut self.source);}}}

#[doc(hidden)]
pub struct Cursor<T:Send+Sync+'static>{handle:SealedShared<T>,pending:ManuallyDrop<Option<T>>,original:Option<SharedOriginal<T>>}
impl<T:Send+Sync+'static> Cursor<T>{
 fn original_demands(&self,original:&T)->Result<RetirementDemand,ValueError>{if let Some(demand)=(self.handle.issuer.demand)(original,OriginalDemand::Shared)?{return Ok(demand);}let copy_bytes=size_of::<Option<SharedOriginalStorage<T>>>().checked_add(size_of::<Option<T>>().checked_mul(2).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shared payload transfer footprint overflow"))?).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original shared payload transfer footprint overflow"))?;Ok(RetirementDemand{copy_bytes,release_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()})}
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.handle.source.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
  if let Some(original)=self.original.as_ref(){return self.original_demands(original.get())}
  if let Some(value)=self.pending.as_ref(){return(self.handle.issuer.demand)(value,OriginalDemand::Native)?.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native payload has no issuer"))}
  let _=body;Ok(Default::default())
 }
 fn step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();
  let d=match self.demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetirementStep::Failure(e.into(),0)};
  self.step_selected(effect,d)
 }
 fn step_selected(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,demand:RetirementDemand)->RetirementStep{let grant=effect.grant();
  if !retained_clone_grant_admits(grant,demand){return RetirementStep::BudgetExhausted}
  if self.handle.source.is_some(){
   let work=match retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};
   let Some(source)=self.handle.source.take()else{return RetirementStep::Failure(ValueError::invariant("sealed alias present").into(),work)};
   match source.retire(){SharedOriginalRelease::Alias=>{},SharedOriginalRelease::Original(original)=>self.original=Some(original)}
   return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},work)
  }
  if let Some(original)=self.original.as_ref(){
   let specialized=match(self.handle.issuer.demand)(original.get(),OriginalDemand::Shared){Ok(d)=>d.is_some(),Err(e)=>return RetirementStep::Failure(e.into(),0)};
   if specialized{return self.admit_cursor(effect,true)}
   let work=match retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};
   let Some(value)=self.original.as_mut().unwrap().take_value()else{return RetirementStep::Progress(Default::default(),work)};
   self.original=None;*self.pending=Some(value);
   return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()},work)
  }
  if self.pending.is_some(){return self.admit_cursor(effect,false)}
  RetirementStep::Complete(0)
 }
 fn admit_cursor(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,shared:bool)->RetirementStep{
  let slot=if shared{OriginalSlot::Shared(&mut self.original)}else{OriginalSlot::Native(&mut self.pending)};
  let turn=(self.handle.issuer.admit)(slot,effect);
  match turn.result{
   Err(error)=>RetirementStep::Failure(error.into(),turn.work),
   Ok(None)=>RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated,"original issuer omitted its owned cursor").into(),turn.work),
   Ok(Some((owner,p)))=>RetirementStep::ChildProgress(owner,p,turn.work)
  }
 }
}

impl<T:Send+Sync+'static> RetireOwned for SealedShared<T>{fn retirement_ready(&self)->bool{self.source.as_ref().is_none_or(|source|(self.issuer.ready)(source.get()))}type RetirementCursor=Cursor<T>;fn retirement_seed(self)->Self::RetirementCursor{Cursor{handle:self,pending:ManuallyDrop::new(None),original:None}}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Cursor<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+Sync+'static> RetirementCursor for Cursor<T>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.demands(body)?;if self.handle.source.is_some(){return Ok(1)}if self.original.is_none()&&self.pending.is_none(){return Ok(0)}Ok(1)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();if self.terminal_is_empty(){return RetirementStep::Complete(0)}self.step(&mut effect.nested(grant))}
 fn terminal_is_empty(&self)->bool{self.handle.source.is_none()&&self.original.is_none()&&self.pending.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{false}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Drop for Cursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"sealed cursor retains its original payload and issuer");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.pending);}}}}

#[cfg(test)]
#[path="../../🧪️tests/⏱️effect/🦀️.rs"]
mod effect;
/// 🎟️ Funds exactly the complete admission quote of one value, including its inline placement copy.
#[cfg(test)]
fn admission_grant<T:Send+Sync+'static>()->RetainedCloneGrant{let d=SealedShared::<T>::admission_demand().unwrap();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;

#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod actual_work_tests;
