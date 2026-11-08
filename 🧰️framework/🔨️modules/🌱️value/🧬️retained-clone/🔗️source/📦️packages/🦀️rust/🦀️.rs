//! 🔗️ Immutable original source/ref/binding/projection custody owns every native alias.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_control_contract as retained_clone;
pub use semio_framework_value_retirement::{retirement_fields_type,retirement_fields_seed,OriginalRetirementOutcome,OriginalRetirementPoll,OriginalRetirementControl,RetainedWorkOutcome,ErasedSnapshotRetirement,retirement,factory_ticket_demands,close_factory_ticket,RetirementSelection,select_factory_ticket,owned_retirement_birth_bytes,admit_owned_retirement,OriginalRetirementFailure,OriginalRetirementCloseLoan,OriginalRetirementClosePort,OriginalFailureStorageBirth,OriginalFailureStoragePool};
pub use semio_framework_value_shared_issuance as shared_issuance;
#[cfg(test)]
pub use semio_framework_value_retirement::{retirement_fields_seed,retirement_fields_type};
use semio_framework_value_retirement::{RetireOwned,retirement::controlled::RetainedOwnerGate};
use std::{mem::ManuallyDrop,sync::atomic::AtomicU64};
static RETAINED_CLONE_SOURCE_IDS:AtomicU64=AtomicU64::new(1);
#[path="../../🦀️.rs"]
mod source_custody;
pub use source_custody::{RetainedCloneSource,RetainedCloneSourceAdmission,StagedSourceAdmission,StagedProjectionSelection,OriginalProjectionContext,OriginalProjectionFrame,OriginalProjectionFramePhase,OriginalProjectionProducer,OriginalProjectionReply,OriginalSourceProjection};
use source_custody::RetainedCloneLeaseOwner;
#[path="../../../🔗️projection/🦀️.rs"]
mod owned_projection;
pub use owned_projection::{RetainedOwnedProjection,RetainedOwnedProjectionAdmission,RetainedNativeFieldProjection,RetainedNativeArrayIndexProjection};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RetainedCloneProjection {
    parent: usize,
    address: usize,
    discriminator: usize,
}

pub struct RetainedCloneRef<'a,T:?Sized>{value:&'a T,lease:&'a shared_issuance::SealedShared<RetainedCloneLeaseOwner>,projection:RetainedCloneProjection}
impl<'a,T:?Sized> Copy for RetainedCloneRef<'a,T>{}
impl<'a,T:?Sized> Clone for RetainedCloneRef<'a,T>{fn clone(&self)->Self{*self}}
impl<'a,T:?Sized> RetainedCloneRef<'a,T>{
 pub fn get(self)->&'a T{self.value}
 /// 🪪️ Observes the already issued sealed source without duplicating a lease or granting authority.
 pub fn source_identity(self)->u64{self.lease.get().id}
 /// 🪪️ Observes fixed issued-source and projection coordinates without retaining or authorizing access.
 pub fn reference_identity(self)->(u64,usize,usize,usize){(self.lease.get().id,self.projection.parent,self.projection.address,self.projection.discriminator)}
 /// 📏️ Quotes one projected reference and its three source coordinates without accessing the original value.
 pub fn projection_demand<U:?Sized+'a>()->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<RetainedCloneRef<'a,U>>(),depth:1,..Default::default()}}
 /// ⛽️ Composes a concrete accessor's bounded work with five reference and coordinate writes.
 pub fn projection_work_demand(accessor_work:u64)->Result<u64,ValueError>{accessor_work.checked_add(5).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"projected source work overflow"))}
 /// 🪡️ Invokes a bounded, nonallocating accessor only after the original work and physical grant admit its projected reference. The accessor must report its actual work and have no retained side effects. A borrowed projection copies no owned item.
 pub fn project_paid<U:?Sized,F>(self,discriminator:usize,accessor_work:u64,maximum_work_units:u64,grant:RetainedCloneGrant,project:F)->RetainedWorkOutcome<Option<(RetainedCloneRef<'a,U>,RetainedCloneProgress)>>where F:FnOnce(&'a T)->RetainedWorkOutcome<&'a U>{
  let quoted=match Self::projection_work_demand(accessor_work){Ok(v)=>v,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let demand=Self::projection_demand::<U>();if maximum_work_units<quoted||!retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(None),work:0}}
  let turn=project(self.value);let work=turn.work;if work>u128::from(accessor_work){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"projected accessor exceeded its work quote")),work}}let value=match turn.result{Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{result:Err(error),work}};let result=RetainedCloneRef{value,lease:self.lease,projection:RetainedCloneProjection{parent:self.projection.address,address:value as*const U as*const()as usize,discriminator}};RetainedWorkOutcome{result:Ok(Some((result,RetainedCloneProgress{copied_items:0,copied_bytes:demand.copy_bytes,..Default::default()}))),work:work+5}
 }
 pub fn project<U:?Sized,F>(self,discriminator:usize,project:F)->RetainedCloneRef<'a,U>where F:for<'source>FnOnce(&'source T)->&'source U{let value=project(self.value);RetainedCloneRef{value,lease:self.lease,projection:RetainedCloneProjection{parent:self.projection.address,address:value as*const U as*const()as usize,discriminator}}}
 /// 🪤️ Projects through a fallible accessor so an absent original child refuses instead of aborting.
 pub fn try_project<U:?Sized,E,F>(self,discriminator:usize,project:F)->Result<RetainedCloneRef<'a,U>,E>where F:for<'source>FnOnce(&'source T)->Result<&'source U,E>{let value=project(self.value)?;Ok(RetainedCloneRef{value,lease:self.lease,projection:RetainedCloneProjection{parent:self.projection.address,address:value as*const U as*const()as usize,discriminator}})}
 pub fn binding_copy_bytes(self)->usize{RetainedCloneBinding::alias_copy_bytes()}
 /// ⛽️ Quotes fixed binding-state and four coordinate checks without reading the original source.
 pub fn check_binding_work(binding:&Option<RetainedCloneBinding>)->u64{if binding.is_some(){5}else{1}}
 /// 🪪️ Checks one issued original binding under its caller work and physical grant.
 pub fn check_binding_paid(self,binding:&Option<RetainedCloneBinding>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<bool>>{
  let before=effect.performed_work();let result=(||->Result<Option<bool>,ValueError>{let quoted=Self::check_binding_work(binding);if effect.remaining_work_units()?<quoted||!retained_clone_grant_admits(effect.grant(),RetirementDemand{depth:1,..Default::default()}){return Ok(None)}let charge=|effect:&mut RetainedAllocationEffect<'_>|->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"source binding check exceeded its original work"))}};charge(effect)?;let Some(expected)=binding.as_ref()else{return Ok(Some(false))};let Some(lease)=expected.lease.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"projected source binding already closing"))};let mismatch=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original clone source or projected path changed");charge(effect)?;if lease.get().id!=self.lease.get().id{return Err(mismatch())}charge(effect)?;if expected.projection.parent!=self.projection.parent{return Err(mismatch())}charge(effect)?;if expected.projection.address!=self.projection.address{return Err(mismatch())}charge(effect)?;if expected.projection.discriminator!=self.projection.discriminator{return Err(mismatch())}Ok(Some(true))})();RetainedWorkOutcome{result,work:effect.performed_work()-before}
 }
  /// ✅️ Checks the original issued source and exact projected path without granting a new alias.
 pub fn check_binding(self,binding:&Option<RetainedCloneBinding>)->Result<bool,ValueError>{self.binding_demand(binding).map(|demand|demand.is_none())}
 /// 🔒️ Validates one original lease and projected path without exposing private custody.
 pub fn matches_binding(self,binding:&RetainedCloneBinding)->bool{binding.lease.as_ref().is_some_and(|lease|lease.get().id==self.lease.get().id)&&binding.projection==self.projection}
 pub fn binding_demand(self,binding:&Option<RetainedCloneBinding>)->Result<Option<RetirementDemand>,ValueError>{if let Some(expected)=binding{if expected.lease.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"projected source binding already closing"))?.get().id!=self.lease.get().id||expected.projection!=self.projection{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original clone source or projected path changed"));}Ok(None)}else{Ok(Some(RetirementDemand{copy_bytes:self.binding_copy_bytes(),depth:1,..Default::default()}))}}
 /// ⛽️ Validates the original binding before borrowing its defining alias increment quote.
 pub fn binding_work_demand(self,binding:&Option<RetainedCloneBinding>)->Result<u64,ValueError>{if self.binding_demand(binding)?.is_none(){Ok(0)}else{self.lease.duplicate_work_demand()}}
 pub fn bind(self,binding:&mut Option<RetainedCloneBinding>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>>{let grant=effect.grant();let mut work=0;let result=(||{
  if let Some(expected)=binding{if expected.lease.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"projected source binding already closing"))?.get().id!=self.lease.get().id||expected.projection!=self.projection{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original clone source or projected path changed"));}return Ok(None);}
  if effect.remaining_work_units()?<self.binding_work_demand(binding)?||grant.maximum_items==0||grant.maximum_copy_bytes<self.binding_copy_bytes()||grant.maximum_depth==0{return Ok(Some(Default::default()));}
  let mut alias=None;let turn=self.lease.duplicate_into(&mut alias,effect);work=turn.work;let mut progress=turn.result?.progress();let Some(alias)=alias else{return Ok(Some(progress))};*binding=Some(RetainedCloneBinding::new(alias,self.projection));progress.copied_bytes=self.binding_copy_bytes();Ok(Some(progress))
 })();RetainedWorkOutcome{result,work}}
}
fn close_query_busy()->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,"original close query already holds exclusive cursor custody")}
pub struct RetainedCloneBinding{lease:ManuallyDrop<Option<shared_issuance::SealedShared<RetainedCloneLeaseOwner>>>,close:ManuallyDrop<Option<RetainedOwnerGate<retirement::controlled::ControlledRetirement<shared_issuance::SealedShared<RetainedCloneLeaseOwner>>>>>,projection:RetainedCloneProjection}
impl RetainedCloneBinding{
 pub fn close_into<'reply>(binding:&mut Option<Self>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Self::close_borrowed(binding,None,body,effect,reply,false)}
 pub fn recover_close_into<'reply>(binding:&mut Option<Self>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Self::close_borrowed(binding,None,body,effect,reply,true)}
 pub fn close_original_into<'reply>(binding:&mut Option<Self>,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Self::close_borrowed(binding,Some(original),body,effect,reply,false)}
 pub fn recover_close_original_into<'reply>(binding:&mut Option<Self>,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{Self::close_borrowed(binding,Some(original),body,effect,reply,true)}
 fn close_borrowed<'reply>(binding:&mut Option<Self>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if effect.invocation().is_none(){return Err(ValueError::invariant("binding close requires its original producer invocation"))}
  if recover{
   let owner=binding.as_mut().and_then(|owner|owner.close.as_mut()).ok_or_else(||ValueError::invariant("binding recovery lost its original close owner"))?.get_mut();
   return match original{Some(original)=>owner.recover_step_original_into(original,body,effect,reply),None=>owner.recover_step_into(body,effect,reply)}
  }
  if let Some(owner)=binding.as_mut().and_then(|owner|owner.close.as_mut()){
   let owner=owner.get_mut();if !owner.terminal_is_empty(){return match original{Some(original)=>owner.step_original_into(original,body,effect,reply),None=>owner.step_into(body,effect,reply)}}
  }
  match Self::close_one(binding,body,effect){RetainedWorkPoll::Complete(turn)=>Ok(reply.complete(turn.result)),RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("native binding header transition cannot suspend"))}
 }


 pub fn bound_close_available(binding:&Option<Self>)->bool{binding.as_ref().is_some_and(|owner|owner.close.is_none()&&owner.lease.as_ref().is_some_and(|lease|!lease.terminal_is_empty()))}
 pub fn bound_close_demand<T:?Sized>(binding:&Option<Self>,source:RetainedCloneRef<'_,T>,body:usize)->Result<RetirementDemand,ValueError>{let Some(owner)=binding.as_ref()else{return Ok(Default::default())};if owner.close.is_some(){return Self::close_demand(binding,body)}source.binding_demand(binding)?;let lease=owner.lease.as_ref().ok_or_else(||ValueError::invariant("bound close lost its lease"))?;let mut demand=lease.borrowed_alias_return_demand();demand.copy_bytes=demand.copy_bytes.checked_add(std::mem::size_of_val(&owner.lease)).and_then(|n|n.checked_add(std::mem::size_of::<Option<Self>>())).ok_or_else(||ValueError::invariant("bound alias copy overflow"))?;Ok(demand)}
 pub fn bound_close_work_demand<T:?Sized>(binding:&Option<Self>,_source:RetainedCloneRef<'_,T>,body:usize)->Result<u64,ValueError>{Self::close_work_demand(binding,body)}
 pub fn close_bound_into<'r,T:?Sized>(binding:&mut Option<Self>,source:RetainedCloneRef<'_,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
  if effect.invocation().is_none(){return Err(ValueError::invariant("bound close requires its actual invocation"))}if binding.as_ref().is_some_and(|owner|owner.close.is_some()){return Self::close_into(binding,body,effect,reply)}let demand=Self::bound_close_demand(binding,source,effect.grant().maximum_copy_bytes)?;let Some(owner)=binding.as_mut()else{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))};if !retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let turn=owner.lease.as_mut().unwrap().return_borrowed_alias(source.lease,effect);let result=match turn.result{Err(error)=>Err(error.into()),Ok(progress)if progress.copied_items==0=>Ok(RetainedCloneStep::Progress(progress)),Ok(progress)=>{*binding.as_mut().unwrap().lease=None;*binding=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..progress}))}};Ok(reply.complete(result))
 }
 pub fn alias_copy_bytes()->usize{shared_issuance::SealedShared::<RetainedCloneLeaseOwner>::duplicate_copy_bytes()+std::mem::size_of::<Self>()+std::mem::size_of::<Option<Self>>()}
 /// ⛽️ Quotes the actual sealed alias duplication and two binding-state writes.
 pub const fn alias_work_demand()->u64{shared_issuance::SealedShared::<RetainedCloneLeaseOwner>::DUPLICATE_WORK}
 fn new(lease:shared_issuance::SealedShared<RetainedCloneLeaseOwner>,projection:RetainedCloneProjection)->Self{Self{lease:ManuallyDrop::new(Some(lease)),close:ManuallyDrop::new(None),projection}}
 pub fn close_one(binding:&mut Option<Self>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
  if binding.is_none(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}let grant=effect.grant();let demand=match Self::close_demand(binding,body){Ok(demand)=>demand,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};let work=match Self::close_work_demand(binding,body){Ok(work)=>work,Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if effect.maximum_work_units()<work||!retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  let Some(owner)=binding.as_mut()else{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})};if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if owner.close.is_none()&&owner.lease.is_some(){match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original binding close exceeded its work authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}let lease=owner.lease.take().expect("original lease checked");*owner.close=Some(RetainedOwnerGate::new(retirement::controlled::ControlledRetirement::new(lease).unwrap_or_else(|_|unreachable!("sealed projection retains its typed issuer"))));return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1})}
  let close=match owner.close.as_mut(){Some(v)=>v,None=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("close absent").into()),work:0})}.get_mut();if close.terminal_is_empty(){match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original binding close exceeded its work authority").into()),work:1}),Err(error)=>return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:1})}*owner.close=None;*binding=None;return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1})}close.step(body,effect)
 }
 /// ⛽️ Quotes the actual original binding closure action.
 pub fn close_work_demand(binding:&Option<Self>,body:usize)->Result<u64,ValueError>{binding.as_ref().map_or(Ok(0),|owner|owner.close.as_ref().map_or(Ok(1),|gate|{let owner=gate.try_lock().map_err(|_|close_query_busy())?;if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}))}
 pub fn copy_demand(binding:&Option<Self>)->Result<usize,ValueError>{Self::close_demand(binding,0).map(|demand|demand.copy_bytes)}
 pub fn capacity_demand(binding:&Option<Self>,body:usize)->Result<usize,ValueError>{binding.as_ref().and_then(|owner|owner.close.as_ref()).map_or(Ok(0),|owner|owner.try_lock().map_err(|_|close_query_busy())?.next_capacity_byte_demand(body))}
 pub fn release_demand(binding:&Option<Self>)->Result<usize,ValueError>{binding.as_ref().and_then(|owner|owner.close.as_ref()).map_or(Ok(0),|owner|owner.try_lock().map_err(|_|close_query_busy())?.next_release_byte_demand())}
 pub fn depth_demand(binding:&Option<Self>)->Result<usize,ValueError>{binding.as_ref().map_or(Ok(0),|owner|owner.close.as_ref().map_or(Ok(1),|owner|{let owner=owner.try_lock().map_err(|_|close_query_busy())?;if owner.terminal_is_empty(){Ok(1)}else{owner.next_depth_demand()}}))}
 pub fn close_demand(binding:&Option<Self>,body:usize)->Result<RetirementDemand,ValueError>{binding.as_ref().map_or(Ok(Default::default()),|owner|owner.close.as_ref().map_or(Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<shared_issuance::SealedShared<RetainedCloneLeaseOwner>>>()+std::mem::size_of::<Option<RetainedOwnerGate<retirement::controlled::ControlledRetirement<shared_issuance::SealedShared<RetainedCloneLeaseOwner>>>>>(),depth:1,..Default::default()}),|gate|{let current=gate.try_lock().map_err(|_|close_query_busy())?;if current.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<RetainedOwnerGate<retirement::controlled::ControlledRetirement<shared_issuance::SealedShared<RetainedCloneLeaseOwner>>>>>()+std::mem::size_of::<Option<Self>>(),depth:1,..Default::default()})}else{current.selected_demand(body)}}))}
 /// 👶️ Preserves a selected child Birth; busy custody conservatively selects its original route.
 pub fn requires_original_birth(binding:&Option<Self>)->bool{binding.as_ref().and_then(|owner|owner.close.as_ref()).is_some_and(|gate|gate.try_lock().map_or(true,|owner|owner.requires_original_birth()))}
 /// 🪪️ Preserves the selected child original-control route without inventing a loan.
 pub fn requires_original_control(binding:&Option<Self>)->bool{binding.as_ref().and_then(|owner|owner.close.as_ref()).is_some_and(|gate|gate.try_lock().map_or(true,|owner|ErasedSnapshotRetirement::requires_original_control(&*owner)))}
 /// 👶️ Advances a real selected Birth before any general receiving effect.
 pub fn close_original_birth_step(binding:&mut Option<Self>,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  match binding.as_mut().and_then(|owner|owner.close.as_mut()){Some(gate)=>gate.get_mut().step_original_birth(original,grant),None=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original binding has no selected child birth").into()),work:0})}
 }
 /// 🧷️ Lends the unchanged original loan and effect directly to the actual selected child.
 pub fn close_original_step(binding:&mut Option<Self>,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  if !Self::requires_original_control(binding){return Self::close_one(binding,body,effect)}
  let owner=binding.as_mut().and_then(|owner|owner.close.as_mut()).expect("selected original binding child").get_mut();
  ErasedSnapshotRetirement::close_original_step(owner,original,body,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.lease.is_none()&&self.close.is_none()}
}
impl Drop for RetainedCloneBinding{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"projected original source requires genuine alias closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.lease);ManuallyDrop::drop(&mut self.close);}}}}

pub struct RetainedCloneBorrowAuthority<A:RetireOwned+Sync>{source:RetainedCloneSource<A>}
impl<A:RetireOwned+Sync> RetainedCloneBorrowAuthority<A>{
 pub fn constructor_capacity_bytes()->usize{RetainedCloneSource::<A>::owned_constructor_capacity_bytes::<()>()}
 pub fn constructor_copy_bytes()->usize{RetainedCloneSource::<A>::constructor_copy_bytes()}
 pub fn admission(authority:A)->RetainedCloneBorrowAuthorityAdmission<A>{RetainedCloneBorrowAuthorityAdmission{source:RetainedCloneSource::admission_owned(authority,())}}
 pub fn borrow<T:?Sized,F>(&self,discriminator:usize,project:F)->RetainedCloneRef<'_,T>where F:for<'source>FnOnce(&'source A)->&'source T{self.source.borrow().project(discriminator,project)}
 /// 🧾️ Borrows the one actual source close quote with every independent physical currency.
 /// ⛽️ Quotes the same original source owner's selected computational work.
 pub fn next_close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.source.next_close_work_demand(body)}
 pub fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.source.next_close_demand(body)}
 pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.source.next_close_copy_byte_demand()}
 pub fn next_close_capacity_byte_demand(&self,work:usize)->Result<usize,ValueError>{self.source.next_close_capacity_byte_demand(work)}
 pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.source.next_close_release_byte_demand()}
 pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.source.next_close_depth_demand()}
 pub fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let grant=effect.grant();self.source.close_step(body,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}
 pub fn next_take_work_demand(&self,body:usize)->Result<u64,ValueError>{self.source.next_take_work_demand(body)}
 pub fn next_take_copy_byte_demand(&self)->Result<usize,ValueError>{self.source.next_take_copy_byte_demand()}
 pub fn next_take_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.source.next_take_capacity_byte_demand(body)}
 pub fn next_take_release_byte_demand(&self)->Result<usize,ValueError>{self.source.next_take_release_byte_demand()}
 pub fn next_take_depth_demand(&self)->Result<usize,ValueError>{self.source.next_take_depth_demand()}
 /// 🎯️ Quotes the one actual original-authority return with every independent physical currency.
 pub fn next_take_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.source.next_take_demand(body)}
 pub fn take_authority_into<'r>(&mut self,output:&mut Option<A>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.source.take_authority_into(output,body,effect,reply)}
 pub fn recover_take_authority_into<'r>(&mut self,output:&mut Option<A>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.source.recover_take_authority_into(output,body,effect,reply)}
 pub fn requires_take_original_birth(&self)->bool{self.source.requires_take_original_birth()}
 pub fn take_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.source.take_original_birth_step(original,grant)}
 pub fn requires_take_original_control(&self)->bool{self.source.requires_take_original_control()}
 pub fn take_authority_original_into<'r>(&mut self,output:&mut Option<A>,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.source.take_authority_original_into(output,original,body,effect,reply)}
 pub fn recover_take_authority_original_into<'r>(&mut self,output:&mut Option<A>,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.source.recover_take_authority_original_into(output,original,body,effect,reply)}
 pub fn try_authority(&self)->Result<&A,ValueError>{self.source.try_authority()}
 pub fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
}
#[cfg(test)]
use semio_framework_value_testing_allocator as allocation_testing;
#[cfg(test)]
pub(crate) use allocation_testing::observe_retirement_allocations;
#[cfg(test)]
#[path="../../🧪️tests/🔐️custody/🦀️.rs"]
mod original_sealed_custody_tests;

#[cfg(test)]
#[path="../../🧪️tests/🎟️header/🦀️.rs"]
mod original_zero_header_tests;

#[cfg(test)]
#[path="../../🧪️tests/🔐️query/🦀️.rs"]
mod shared_query_tests;

#[cfg(test)]#[path="../../🧪️tests/📌️retain/🦀️.rs"]mod retained_source_ref_tests;

#[cfg(test)]
#[path="../../🎟️work/🧪️tests/🦀️.rs"]
mod actual_source_work_tests;

#[path="../../../♻️close/🦀️.rs"]
mod clone_close;
pub use clone_close::{RetainedCloneClose,RetainedEmptyOwner,RetainedCloseSelection,close_retained_binding};

/// 🎟️ A borrowed authority's staged birth preserves the same original source admission.
pub struct RetainedCloneBorrowAuthorityAdmission<A:RetireOwned+Sync>{source:RetainedCloneSourceAdmission<A,A>}
impl<A:RetireOwned+Sync> RetainedCloneBorrowAuthorityAdmission<A>{
 pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{self.source.next_demand()}
 /// ⛽️ Borrows the actual current source handback work quote.
 pub fn return_work_demand(&self)->Result<u64,ValueError>{self.source.return_work_demand()}
 pub fn next_return_demand(&self)->Result<RetirementDemand,ValueError>{self.source.next_return_demand()}
 /// 🪙️ Writes actual allocator extent into the caller's retained marker before publishing the outcome.
 pub fn advance_work_demand(&self)->Result<u64,ValueError>{self.source.advance_work_demand()}
 pub fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{self.source.advance(effect)}
 pub fn take_output_demand(&self)->Result<RetirementDemand,ValueError>{let mut demand=self.source.take_output_demand();if self.source.take_output_work_demand()==0{return Ok(demand)}demand.copy_bytes=demand.copy_bytes.checked_add(std::mem::size_of::<Option<RetainedCloneBorrowAuthority<A>>>()).ok_or_else(||ValueError::invariant("borrow authority publication copy overflow"))?;Ok(demand)}
 pub fn take_output_work_demand(&self)->u64{self.source.take_output_work_demand()}
 pub fn take_output(&mut self,output:&mut Option<RetainedCloneBorrowAuthority<A>>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if output.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("original borrow authority output is occupied")),work:0}}let demand=match self.take_output_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let mut source=None;let turn=self.source.take_output(&mut source,effect);if let Some(source)=source{*output=Some(RetainedCloneBorrowAuthority{source})}RetainedWorkOutcome{result:turn.result.map(|step|if output.is_some(){let mut progress=step.progress();progress.copied_bytes=demand.copy_bytes;RetainedCloneStep::Complete(progress)}else{step}),work:turn.work}
 }

 pub fn return_inputs(&mut self,output:&mut Option<(A,())>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{self.source.return_inputs(output,effect)}
 pub fn terminal_is_empty(&self)->bool{self.source.terminal_is_empty()}
}

#[cfg(test)]
#[path="../../🧪️law-bank/🦀️.rs"]
mod source_law_bank;

#[path="../../👁️owner/🦀️.rs"]mod source_owner;
pub use source_owner::{OriginalSourceOwner,OriginalSourceMode};
