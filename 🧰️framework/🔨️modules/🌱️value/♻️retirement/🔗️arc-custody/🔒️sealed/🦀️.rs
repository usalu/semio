//! 🔒️ Shared immutable values expose strong handles and retain their original issuer.
pub const ORIGINAL_TRANSFER_WORK:u64=1;
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{OriginalRetirementFailure,OriginalRetirementCloseLoan,ErasedSnapshotRetirement,RetireOwned,retirement,factory_ticket_demands,close_factory_ticket};
use retirement::{RetirementCursor,RetirementStep};
use std::{mem::ManuallyDrop,sync::atomic::{AtomicPtr,AtomicU8,AtomicU64,Ordering}};
pub use retirement::arc_custody::original::SharedOriginal;
use retirement::arc_custody::original::{SharedOriginalStorage,SharedOriginalRelease,SharedOriginalBirth};

static NEXT_ISSUANCE:AtomicU64=AtomicU64::new(1);

#[cfg(test)]
#[path="🚚️frontier/🧪️tests/🦀️.rs"]
mod frontier_tests;
/// 🎟️ The original producer supplies admission and demand functions without an erased allocation.
enum OriginalDemand{Native,Shared}
enum OriginalSlot<'a,T:Send+Sync+'static>{Native(&'a mut Option<T>),Shared(&'a mut Option<SharedOriginal<T>>)}
pub struct SharedIssuer<T:Send+Sync+'static>{native_admission:Option<retirement::NativeSharedRetirementAdmission<T>>,pub transfer_copy_bytes:fn(&T)->usize,ready:fn(&T)->bool,work:fn(&T,OriginalDemand)->Result<Option<u64>,ValueError>,demand:fn(&T,OriginalDemand)->Result<Option<RetirementDemand>,ValueError>,admit:fn(OriginalSlot<'_,T>,&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<(Box<dyn RetirementCursor>,RetainedCloneProgress)>>}
impl<T:Send+Sync+'static> Copy for SharedIssuer<T>{}
impl<T:Send+Sync+'static> Clone for SharedIssuer<T>{fn clone(&self)->Self{*self}}
impl<T:RetireOwned+Sync> SharedIssuer<T>{
 pub fn owned()->Self{Self{native_admission:T::native_shared_original_admission(),transfer_copy_bytes:|_|size_of::<T>(),ready:RetireOwned::retirement_ready,work:|value,mode|match mode{OriginalDemand::Native=>Ok(Some(1)),OriginalDemand::Shared=>value.shared_original_work_demand()},demand:|value,mode|match mode{OriginalDemand::Native=>native_payload_demand(value).map(Some),OriginalDemand::Shared=>{if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"sealed original has no controlled shared issuer"))}value.shared_original_demands()}},admit:|slot,effect|match slot{OriginalSlot::Shared(slot)=>T::shared_original_retirement(slot,effect),OriginalSlot::Native(slot)=>semio_framework_value_retirement::admit_retirement_cursor(slot,effect)}}}

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
 let mut value=value;let source=value.source.take().expect("verified original remains initialized");let mut owner=match source.try_owner(){Ok(owner)=>owner,Err(_)=>unreachable!("verified original remains physically exclusive")};owner.take_value().expect("verified original payload remains exclusive")
}

/// 🔐️ The private Arc has no raw, Weak, mutable or consuming escape hatch.
pub struct SealedShared<T:Send+Sync+'static>{source:ManuallyDrop<Option<SharedOriginalStorage<T>>>,issuer:SharedIssuer<T>,issuance:u64}
impl<T:std::fmt::Debug+Send+Sync+'static> std::fmt::Debug for SealedShared<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{self.get().fmt(f)}}
impl<T:PartialEq+Send+Sync+'static> PartialEq for SealedShared<T>{fn eq(&self,other:&Self)->bool{self.get()==other.get()}}
impl<T:Eq+Send+Sync+'static> Eq for SealedShared<T>{}

/// 🟢️ Retains one real empty shared backing until funded publication or cancellation.
pub struct SealedSharedBirth<T:Send+Sync+'static>{issuance:u64,storage:SharedOriginalBirth<T>}
impl<T:Send+Sync+'static> SealedSharedBirth<T>{
 pub const fn publication_work_demand()->u64{SharedOriginalBirth::<T>::publication_work_demand()}
 pub fn publication_demand()->RetirementDemand{let mut demand=SharedOriginalBirth::<T>::publication_demand();demand.copy_bytes+=size_of::<SealedShared<T>>()+size_of::<Option<SealedShared<T>>>();demand}
 pub fn publish(&mut self,input:&mut Option<T>,issuer:SharedIssuer<T>,output:&mut Option<SealedShared<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let result=(||{if output.is_some(){return Err(ValueError::invariant("sealed publication output is occupied"))}if !retained_clone_grant_admits(effect.grant(),Self::publication_demand())||effect.remaining_work_units()?<Self::publication_work_demand(){return Ok(None)}Ok(Some(()))})();
  match result{Err(error)=>RetainedWorkOutcome{result:Err(error),work:0},Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Ok(Some(()))=>{let mut source=None;let turn=self.storage.publish(input,&mut source,effect);if let Some(source)=source{*output=Some(SealedShared{source:ManuallyDrop::new(Some(source)),issuer,issuance:self.issuance})}RetainedWorkOutcome{result:turn.result.map(|step|if output.is_some(){let mut progress=step.progress();progress.copied_bytes=Self::publication_demand().copy_bytes;RetainedCloneStep::Complete(progress)}else{step}),work:turn.work}}}
 }
 pub fn next_close_demand(&self)->RetirementDemand{self.storage.next_close_demand()}
 pub fn next_close_work_demand(&self)->u64{self.storage.close_work_demand()}
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{self.storage.close_step(body,effect)}
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
 pub const fn birth_work_demand()->u64{1+Self::prepare_birth_work_demand()}
 pub const fn duplicate_alias_work_demand()->u64{SharedOriginalStorage::<T>::DUPLICATE_WORK}
 pub fn source_admission_demand()->Result<RetirementDemand,ValueError>{let mut demand=SharedOriginalStorage::<T>::source_admission_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Self>()+size_of::<Option<Self>>()+size_of::<u64>()).ok_or_else(||ValueError::invariant("sealed source admission copy overflow"))?;Ok(demand)}
 pub const fn source_admission_work_demand()->u64{1+SharedOriginalStorage::<T>::source_admission_work_demand()}
 pub fn prepare_birth_demand()->Result<RetirementDemand,ValueError>{let mut demand=SharedOriginalStorage::<T>::prepare_birth_demand()?;demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<SealedSharedBirth<T>>()+size_of::<Option<SealedSharedBirth<T>>>()+size_of::<u64>()).ok_or_else(||ValueError::invariant("sealed empty birth copy overflow"))?;Ok(demand)}
 pub const fn prepare_birth_work_demand()->u64{1+SharedOriginalStorage::<T>::prepare_birth_work_demand()}
 /// 📥️ Keeps original input and sealed output resident through the actual native allocation.
 pub fn admit_from(input:&mut Option<T>,issuer:SharedIssuer<T>,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{if output.is_some()||effect.actual().is_some(){return Err(ValueError::invariant("sealed source recipient or marker occupied"))}if input.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=Self::source_admission_demand()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<Self::source_admission_work_demand(){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"sealed issuance exceeded original work"))}let issuance=NEXT_ISSUANCE.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed issuance space exhausted"))?;let mut source=None;let turn=SharedOriginalStorage::admit_from(input,&mut source,effect);let step=turn.result?;if let Some(source)=source{*output=Some(Self{source:ManuallyDrop::new(Some(source)),issuer,issuance});let mut progress=step.progress();progress.copied_bytes=demand.copy_bytes;Ok(RetainedCloneStep::Complete(progress))}else{Ok(step)}})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same sealed source receipt")}
 }
 /// 👶️ Publishes an actual empty sealed backing into its original caller slot.
 pub fn prepare_birth(output:&mut Option<SealedSharedBirth<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  let before=effect.performed_boundary();let result=(||{if output.is_some()||effect.actual().is_some(){return Err(ValueError::invariant("sealed empty birth output or marker occupied"))}let demand=Self::prepare_birth_demand()?;if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<Self::prepare_birth_work_demand(){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"sealed issuance exceeded original work"))}let issuance=NEXT_ISSUANCE.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed issuance space exhausted"))?;let mut storage=None;let turn=SharedOriginalStorage::prepare_birth(&mut storage,effect);let step=turn.result?;if let Some(storage)=storage{*output=Some(SealedSharedBirth{issuance,storage});let mut progress=step.progress();progress.copied_bytes=demand.copy_bytes;Ok(RetainedCloneStep::Complete(progress))}else{Ok(step)}})();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same sealed empty birth receipt")}
 }

 /// ⛽️ Borrows the actual original storage alias work before duplication.
 pub fn try_duplicate(&self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<(Self,RetainedCloneProgress)>{
  if self.source.is_none(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"sealed source has already transferred")),work:0}}let grant=effect.grant();let copy=Self::duplicate_copy_bytes();
  if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||effect.maximum_work_units()<1{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed alias handoff is not funded")),work:0}}
  let mut output=None;let turn=self.duplicate_into(&mut output,effect);RetainedWorkOutcome{result:turn.result.and_then(|step|output.take().map(|source|(source,step.progress())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed alias handoff is not funded"))),work:turn.work}
 }
 pub fn duplicate_work_demand(&self)->Result<u64,ValueError>{self.source.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"sealed source has already transferred"))?.duplicate_work_demand()}
 pub const DUPLICATE_WORK:u64=SharedOriginalStorage::<T>::DUPLICATE_WORK;
 pub const fn duplicate_copy_bytes()->usize{SharedOriginalStorage::<T>::duplicate_copy_bytes()+size_of::<Option<Self>>()}
 pub fn duplicate_into(&self,output:&mut Option<Self>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("sealed duplicate output is occupied")),work:0}}
  let Some(source)=self.source.as_ref()else{return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"sealed source has already transferred")),work:0}};let copy=Self::duplicate_copy_bytes();if !retained_clone_grant_admits(effect.grant(),RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  let mut duplicate=None;let turn=source.duplicate_into(&mut duplicate,effect);if let Some(source)=duplicate{*output=Some(Self{source:ManuallyDrop::new(Some(source)),issuer:self.issuer,issuance:self.issuance})}RetainedWorkOutcome{result:turn.result.map(|step|if output.is_some(){let mut progress=step.progress();progress.copied_bytes=copy;RetainedCloneStep::Complete(progress)}else{step}),work:turn.work}
 }

 /// 🪢️ Returns one paid alias while the matching original strong parent remains borrowed and live.
 /// 📏️ Quotes the actual live alias-header clear before its original return.
 pub fn borrowed_alias_return_demand(&self)->RetirementDemand{if self.source.is_none(){Default::default()}else{RetirementDemand{copy_bytes:size_of::<Option<SharedOriginalStorage<T>>>()+2*size_of::<std::sync::atomic::AtomicUsize>(),depth:1,..Default::default()}}}
 pub fn borrowed_alias_return_work_demand(&self)->u64{u64::from(self.source.is_some())}
 pub fn return_borrowed_alias(&mut self,witness:&Self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>{let grant=effect.grant();let before=effect.performed_boundary();let result=(||{
  let Some(original)=self.source.as_ref()else{return Ok(Default::default())};
  let Some(parent)=witness.source.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original sealed parent has already transferred"))};
  if self.issuance!=witness.issuance||original.identity()!=parent.identity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"borrowed strong parent does not witness the original sealed alias"))}
  let demand=self.borrowed_alias_return_demand();if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(Default::default())}
  if effect.remaining_work_units()?<1{return Ok(Default::default())}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original borrowed alias return exceeded work"))}let Some(source)=self.source.take()else{unreachable!("borrowed original presence checked")};match source.retire(){SharedOriginalRelease::Alias=>{},SharedOriginalRelease::Original(_)=>unreachable!("live borrowed parent retains the original")};Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
 })();RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same borrowed alias return receipt")}}
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
 /// 📏️ Quotes the original alias election and both caller-owned slots.
 pub fn original_election_demand(&self)->Result<RetirementDemand,ValueError>{if self.source.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original sealed value already transferred"))}let copy_bytes=std::mem::size_of::<Option<Self>>().checked_add(std::mem::size_of::<Option<SharedOriginalStorage<T>>>()).and_then(|n|n.checked_add(std::mem::size_of::<Option<SharedOriginal<T>>>() )).and_then(|n|n.checked_add(std::mem::size_of::<std::sync::atomic::AtomicUsize>())).ok_or_else(||ValueError::invariant("original election copy extent overflow"))?;Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 /// ⛽️ Quotes the actual original strong-owner election.
 pub fn original_election_work_demand(&self)->Result<u64,ValueError>{self.original_election_demand()?;Ok(ORIGINAL_TRANSFER_WORK)}
 /// 🗳️ Elects the same original owner directly into a caller-owned slot.
 pub fn try_owner_into(original:&mut Option<Self>,output:&mut Option<SharedOriginal<T>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("original owner output is occupied")),work:0}}let Some(sealed)=original.as_mut()else{return RetainedWorkOutcome{result:Ok(None),work:0}};let demand=match sealed.original_election_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if remaining<ORIGINAL_TRANSFER_WORK||!retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(ORIGINAL_TRANSFER_WORK){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original election exceeded its Work quote")),work:u128::from(ORIGINAL_TRANSFER_WORK)},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let source=sealed.source.take().expect("funded sealed original retains its source");match source.try_owner(){Ok(owner)=>{*output=Some(owner);drop(original.take());RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:u128::from(ORIGINAL_TRANSFER_WORK)}},Err(source)=>{*sealed.source=Some(source);RetainedWorkOutcome{result:Ok(None),work:u128::from(ORIGINAL_TRANSFER_WORK)}}}
 }
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

 fn original_demands(&self,original:&SharedOriginal<T>)->Result<RetirementDemand,ValueError>{if original.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<SharedOriginal<T>>>(),depth:1,..Default::default()})}if let Some(demand)=(self.handle.issuer.demand)(original.get(),OriginalDemand::Shared)?{return Ok(demand)}original.take_value_demand()}
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.handle.source.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<SharedOriginalStorage<T>>>()+size_of::<Option<SharedOriginal<T>>>()+2*size_of::<usize>(),depth:1,..Default::default()})}
  if let Some(original)=self.original.as_ref(){return self.original_demands(original)}
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
   match effect.remaining_work_units(){Ok(0)=>return RetirementStep::BudgetExhausted,Err(error)=>return RetirementStep::Failure(error.into(),0),Ok(_)=>{}}
   match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetirementStep::Progress(Default::default(),1),Err(error)=>return RetirementStep::Failure(error.into(),1)}
   let source=self.handle.source.take().expect("selected original shared source remains resident");
   match source.retire(){SharedOriginalRelease::Alias=>{},SharedOriginalRelease::Original(original)=>self.original=Some(original)}
   return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1)
  }

  if let Some(original)=self.original.as_ref(){
   if original.terminal_is_empty(){
    match effect.remaining_work_units(){Ok(0)=>return RetirementStep::BudgetExhausted,Err(error)=>return RetirementStep::Failure(error.into(),0),Ok(_)=>{}}
    match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetirementStep::Progress(Default::default(),1),Err(error)=>return RetirementStep::Failure(error.into(),1)}
    self.original=None;return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1)
   }
   let specialized=match(self.handle.issuer.demand)(original.get(),OriginalDemand::Shared){Ok(d)=>d.is_some(),Err(e)=>return RetirementStep::Failure(e.into(),0)};
   if specialized{return self.admit_cursor(effect,true)}
   let turn=self.original.as_mut().expect("selected resident original").take_value_step(&mut self.pending,effect);
   return match turn.result{Ok(Some(progress))=>RetirementStep::Progress(progress,turn.work),Ok(None)=>RetirementStep::Progress(Default::default(),turn.work),Err(error)=>RetirementStep::Failure(error.into(),turn.work)}
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
 fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{
  if effect.invocation().is_none(){return Err(ValueError::invariant("sealed retirement requires its original producer invocation"))}
  if let Some(original)=self.original.as_ref(){if !original.terminal_is_empty()&&(self.handle.issuer.demand)(original.get(),OriginalDemand::Shared)?.is_some(){let admission=self.handle.issuer.native_admission.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"in-place shared retirement has no native admission capability"))?;return admission.admit_into(&mut self.original,effect,reply)}}
  let step=self.step(effect);Ok(match step{RetirementStep::Failure(error,_)=>reply.complete(Err(error)),step=>reply.complete(Ok(step))})
 }

 fn recover_close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetirementStep,OriginalRetirementFailure>,ValueError>{if effect.invocation().is_none(){return Err(ValueError::invariant("sealed native recovery requires its original producer invocation"))}if self.original.as_ref().is_some_and(|original|!original.terminal_is_empty()){if let Some(admission)=self.handle.issuer.native_admission{return admission.admit_into(&mut self.original,effect,reply)}}Ok(reply.pending())}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.demands(body)?;if self.handle.source.is_some(){return Ok(1)}if let Some(original)=self.original.as_ref(){if original.terminal_is_empty(){return Ok(1)}if(self.handle.issuer.demand)(original.get(),OriginalDemand::Shared)?.is_some(){return(self.handle.issuer.work)(original.get(),OriginalDemand::Shared)?.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original shared issuer has no work quote"))}return original.take_value_work_demand()}if let Some(value)=self.pending.as_ref(){return(self.handle.issuer.work)(value,OriginalDemand::Native)?.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original native issuer has no work quote"))}Ok(0)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 
 fn terminal_is_empty(&self)->bool{self.handle.source.is_none()&&self.original.is_none()&&self.pending.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{false}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demands(body).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Send+Sync+'static> Cursor<T>{
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();if self.terminal_is_empty(){return RetirementStep::Complete(0)}self.step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
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


