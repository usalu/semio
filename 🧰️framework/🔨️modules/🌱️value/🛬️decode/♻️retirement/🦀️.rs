//! ♻️ Original retirement loans retain decoder custody after new semantic work is cancelled.
use super::{NativeDecodeControl,NativeDecodeProgress,allocation};
use crate::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneStep,retained_clone_grant_admits}};
/// 🪢️ Exposes only original semantic admission during a prepared work action.
pub trait NativePreparedDecodeControl{
 fn checkpoint(&mut self)->Result<(),ValueError>;
 fn admit_turn_capacity(&mut self,bytes:usize)->Result<(),ValueError>;
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>;
}
impl NativePreparedDecodeControl for NativeDecodeControl<'_>{
 fn checkpoint(&mut self)->Result<(),ValueError>{NativeDecodeControl::checkpoint(self)}
 fn admit_turn_capacity(&mut self,bytes:usize)->Result<(),ValueError>{NativeDecodeControl::admit_turn_capacity(self,bytes)}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{NativeDecodeControl::charge(self,bytes)}
}
/// 🫴️ A borrowed original ledger permits only the paid capacity of one already-owned retirement hop.
pub struct NativeRetirementDecodeControl<'operation,'decode>{original:&'operation mut NativeDecodeControl<'decode>,capacity:usize,spent:usize,previous_maximum:usize,failure_reservation:Option<super::NativeDecodeAllocation>}
fn invalid(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::literal(kind,message)}
impl<'operation,'decode> NativeRetirementDecodeControl<'operation,'decode>{
 fn begin(original:&'operation mut NativeDecodeControl<'decode>,grant:RetainedCloneGrant)->Result<Self,ValueError>{if grant.maximum_items==0||grant.maximum_depth==0{return Err(invalid(ValueRefusalKind::WorkLimit,"original retirement requires its paid caller turn"));}if original.retirement.is_none(){return Err(invalid(ValueRefusalKind::UnsupportedOwner,"original retirement requires its exact original recipient"));}let previous_maximum=original.maximum_bytes;original.admit_turn_capacity(grant.maximum_capacity_bytes)?;Ok(Self{original,capacity:grant.maximum_capacity_bytes,spent:0,previous_maximum,failure_reservation:None})}
 /// 🪪️ Observes the unchanged original decoder issuance.
 pub fn ledger_identity(&self)->u64{self.original.ledger_identity()}
 /// 🧷️ Observes the exact original recipient retained by this decoder continuation.
 pub fn retirement_recipient_identity(&self)->Option<u64>{self.original.retirement_recipient_identity()}
 /// 📊️ Reads cumulative original allocation without issuing another capacity.
 pub fn owned_bytes(&self)->usize{self.original.owned_bytes}
 /// 🎟️ Admits one real cleanup birth against this original hop before reporting the original cancellation observer.
 pub fn charge(&mut self,bytes:usize)->Result<(),ValueError>{let spent=self.spent.checked_add(bytes).filter(|spent|*spent<=self.capacity).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original retirement exceeds its paid hop capacity"))?;let next=self.original.owned_bytes.checked_add(bytes).filter(|next|*next<=self.original.maximum_bytes).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original retirement exceeds its cumulative ownership ceiling"))?;if matches!(&self.original.allocation,allocation::Binding::Forwarded(_)){return Err(invalid(ValueRefusalKind::UnsupportedOwner,"forwarded retirement allocation requires actual receipt admission"))}(self.original.callback)(NativeDecodeProgress{completed:self.original.completed,total:self.original.total,owned_bytes:self.original.owned_bytes});self.spent=spent;self.original.owned_bytes=next;Ok(())}
 fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{
  if self.failure_reservation.is_some(){return Err(invalid(ValueRefusalKind::InvariantViolated,"original failure storage quote is already pending"))}
  let available=self.capacity.saturating_sub(self.spent).min(self.original.maximum_bytes.saturating_sub(self.original.owned_bytes));let quoted=RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.min(available),..grant};if !retained_clone_grant_admits(quoted,demand){return Err(invalid(ValueRefusalKind::OwnershipLimit,"original failure storage exceeds its retirement grant"))}
  let request=super::NativeDecodeAllocation{bytes:demand.capacity_bytes,owned_bytes:self.original.owned_bytes,next_owned_bytes:self.original.owned_bytes+demand.capacity_bytes,maximum_bytes:self.original.maximum_bytes};if let allocation::Binding::Forwarded(port)=&mut self.original.allocation{port(allocation::NativeDecodeAllocationEvent::Quote(request))?}self.failure_reservation=Some(request);Ok(quoted)
 }
 fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:crate::RetainedCloneProgress)->Result<(),ValueError>{
  let request=self.failure_reservation.take().ok_or_else(||invalid(ValueRefusalKind::InvariantViolated,"original failure storage receipt has no quote"))?;if !progress.fits(grant)||progress.retained_capacity_bytes>request.bytes{return Err(invalid(ValueRefusalKind::InvariantViolated,"original failure storage receipt exceeds its quote"))}
  let bytes=progress.retained_capacity_bytes;let spent=self.spent.checked_add(bytes).filter(|spent|*spent<=self.capacity).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original failure storage exceeds its paid capacity"))?;let next=request.owned_bytes.checked_add(bytes).filter(|next|*next<=self.original.maximum_bytes).ok_or_else(||invalid(ValueRefusalKind::OwnershipLimit,"original failure storage exceeds its ownership ceiling"))?;
  self.spent=spent;self.original.owned_bytes=next;let posted=match &mut self.original.allocation{allocation::Binding::Forwarded(port)=>port(allocation::NativeDecodeAllocationEvent::Post{request,actual:bytes}),allocation::Binding::Local=>Ok(())};(self.original.callback)(NativeDecodeProgress{completed:self.original.completed,total:self.original.total,owned_bytes:next});posted
 }
 fn cancel_failure_storage(&mut self)->Result<(),ValueError>{let Some(request)=self.failure_reservation.take()else{return Ok(())};match &mut self.original.allocation{allocation::Binding::Forwarded(port)=>port(allocation::NativeDecodeAllocationEvent::Cancel(request)),allocation::Binding::Local=>Ok(())}}
 fn failure_storage_pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool{&mut self.original.retirement.as_deref_mut().expect("original retirement recipient remains installed").failure_pool}
 /// 📨️ Forwards an accepted result through this same paid retirement ledger and its original observer.
 pub fn with_encoding_receiver<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(usize,&mut dyn FnMut(crate::native_encoding::NativeEncodeProgress)->bool,&mut crate::native_encoding::NativeEncodeAllocationPort<'_>)->Result<T,E>)->Result<T,E>{
  let before=self.original.owned_bytes;let scope=RetirementEncodingSpend{loan:self,before};scope.loan.original.with_encoding_receiver(operation)
 }
 /// 👓️ Reports already-born ownership in this exact original recipient.
 pub fn has_retirement_owner(&self)->bool{self.original.has_retirement_owner()}
 /// 🧺️ Observes spare failure custody separately from a pending returned owner.
 pub fn has_retirement_failure_storage(&self)->bool{self.original.has_retirement_failure_storage()}
 /// 📏️ Quotes this original recipient once under the paid cleanup body span.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{let recipient=self.original.retirement.as_ref().ok_or_else(||invalid(ValueRefusalKind::UnsupportedOwner,"original retirement has no recipient"))?;if !recipient.reserved&&recipient.owner.is_some()&&recipient.failure_pool.is_empty(){return Ok(RetirementDemand{capacity_bytes:semio_framework_value_control_contract::OriginalFailureStorage::birth_bytes(),depth:1,..Default::default()})}recipient.next_demand(body)}
 /// 🚮️ Charges and closes the original returned owner after every independent caller currency admits it.
 pub fn close_retirement_recipient(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{
  use semio_framework_value_retirement::{OriginalRetirementFailure as Failure,settle_original_retirement_work as settle};
  let d=match self.retirement_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error.into()),work:0}};
  if !retained_clone_grant_admits(grant,d){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
  if self.original.retirement.as_ref().is_some_and(|recipient|!recipient.reserved&&recipient.owner.is_some()&&recipient.failure_pool.is_empty()){let funding:&mut dyn semio_framework_value_retirement::OriginalRetirementControl=self;return semio_framework_value_retirement::refill_original_failure_storage(funding,grant)}
  if match self.original.retirement.as_ref(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0}}.terminal_is_empty(){return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
  let only_spares=self.original.retirement.as_ref().is_some_and(|recipient|!recipient.reserved&&recipient.owner.is_none()&&!recipient.failure_pool.is_empty());
  if only_spares{match self.original.prepare_work(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return crate::RetainedWorkOutcome{result:Err(Failure::from_work_refusal(error)),work:0}}let turn=self.original.retirement.as_mut().expect("original recipient remains installed").close_spare_step(grant);let result=self.original.commit_work(turn.work);return settle(turn,result)}
  let reserved=match self.original.retirement.as_ref(){Some(v)=>v.reserved,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0}};
  if reserved{match self.original.prepare_work(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Err(error)=>return crate::RetainedWorkOutcome{result:Err(Failure::from_work_refusal(error)),work:0}}
   let turn=match self.original.retirement.as_mut(){Some(v)=>v.close_reserved_step(grant),None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0}};let result=self.original.commit_work(turn.work);return settle(turn,result)
  }
  let mut owner=match self.original.retirement.as_mut(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0}}.owner.take();
  let turn=semio_framework_value_retirement::close_factory_ticket_controlled(&mut owner,grant,self);
  *match self.original.retirement.as_mut(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("retirement absent").into()),work:0}}.owner=owner;turn
 }

}
struct RetirementEncodingSpend<'loan,'operation,'decode>{loan:&'loan mut NativeRetirementDecodeControl<'operation,'decode>,before:usize}
impl Drop for RetirementEncodingSpend<'_,'_,'_>{fn drop(&mut self){self.loan.spent+=self.loan.original.owned_bytes-self.before;}}
impl Drop for NativeRetirementDecodeControl<'_, '_>{fn drop(&mut self){if !self.original.receiving{self.original.maximum_bytes=self.previous_maximum;}}}
impl semio_framework_value_retirement::OriginalRetirementControl for NativeRetirementDecodeControl<'_, '_>{
 fn prepare_work(&mut self,additional:u64)->Result<bool,ValueError>{self.original.prepare_work(additional)}
 fn commit_work(&mut self,actual:u64)->Result<(),ValueError>{self.original.commit_work(actual)}
 fn cancel_prepared_work(&mut self){self.original.cancel_prepared_work()}
 fn ledger_identity(&self)->u64{self.original.ledger_identity()}
 fn retirement_recipient_identity(&self)->Option<u64>{self.original.retirement_recipient_identity()}
 fn charge(&mut self,bytes:usize)->Result<(),ValueError>{NativeRetirementDecodeControl::charge(self,bytes)}
 fn quote_failure_storage(&mut self,demand:RetirementDemand,grant:RetainedCloneGrant)->Result<RetainedCloneGrant,ValueError>{NativeRetirementDecodeControl::quote_failure_storage(self,demand,grant)}
 fn post_failure_storage(&mut self,grant:RetainedCloneGrant,progress:crate::RetainedCloneProgress)->Result<(),ValueError>{NativeRetirementDecodeControl::post_failure_storage(self,grant,progress)}
 fn cancel_failure_storage(&mut self)->Result<(),ValueError>{NativeRetirementDecodeControl::cancel_failure_storage(self)}
 fn failure_storage_pool(&mut self)->&mut semio_framework_value_control_contract::OriginalFailureStoragePool{NativeRetirementDecodeControl::failure_storage_pool(self)}
}
impl<'decode> NativeDecodeControl<'decode>{
 /// 🛟️ Loans only retirement capacity while keeping this operation's original ledger, observer and recipient.
 pub fn with_retirement<T,E:From<ValueError>>(&mut self,grant:RetainedCloneGrant,operation:impl FnOnce(&mut NativeRetirementDecodeControl<'_, 'decode>)->Result<T,E>)->Result<T,E>{let mut loan=NativeRetirementDecodeControl::begin(self,grant).map_err(E::from)?;operation(&mut loan)}
 /// 🧺️ Funds a collision node on its own original turn before a semantic child can run.
 pub fn prepare_failure_storage(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{
  match self.with_retirement::<_,ValueError>(grant,|original|{let funding:&mut dyn semio_framework_value_retirement::OriginalRetirementControl=original;Ok(semio_framework_value_retirement::refill_original_failure_storage(funding,grant))}){Ok(turn)=>turn,Err(error)=>crate::RetainedWorkOutcome{result:Err(error.into()),work:0}}
 }
 /// 🧾️ Advances only with prepaid collision custody and settles the actual typed child work on this original source.
 pub fn run_prepared_work<T:Copy>(&mut self,maximum_work:u64,operation:impl FnOnce(&mut dyn NativePreparedDecodeControl)->crate::RetainedWorkOutcome<T,semio_framework_value_retirement::OriginalRetirementFailure>,effects:impl FnOnce(&T)->crate::RetainedCloneProgress)->Option<crate::RetainedWorkOutcome<T,semio_framework_value_retirement::OriginalRetirementFailure>>{
  if !self.has_retirement_failure_storage(){return None}
  match self.prepare_work(maximum_work){Ok(true)=>{},Ok(false)=>return None,Err(error)=>return Some(crate::RetainedWorkOutcome{result:Err(semio_framework_value_retirement::OriginalRetirementFailure::from_work_refusal(error)),work:0})}
  let Some(recipient)=self.retirement.as_deref_mut()else{self.cancel_prepared_work();return None};let recipient=std::ptr::from_mut(recipient);
  let mut storage=unsafe{(&mut *recipient).failure_pool.take()};
  if storage.is_none(){self.cancel_prepared_work();return None}
  let attempted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||semio_framework_value_control_contract::run_with_failure_storage_transaction(&mut storage,||{let turn=operation(self);let settlement=self.commit_work(turn.work);(turn,settlement)},effects)));
  if let Some(storage)=storage{unsafe{(&mut *recipient).failure_pool.put(storage)}}
  match attempted{Ok(Some(turn))=>Some(turn),Ok(None)=>{self.cancel_prepared_work();None},Err(payload)=>std::panic::resume_unwind(payload)}
 }
}

#[path="🔒️close/🦀️.rs"]
mod close;
pub use close::NativeDecodeCloseControl;

