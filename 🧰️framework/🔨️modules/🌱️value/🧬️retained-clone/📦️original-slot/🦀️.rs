//! 📦️ Original native slots admit immutable leases and return the same owner through its captured issuer.
use crate::{ArtifactOwnedValueRetirementFactory,FactoryOwnedSourceAuthority,RetirementDemand,ValueError,ValueRefusalKind,RetainedAllocationEffect,OriginalRetirementFailure,OriginalRetirementCloseLoan,OriginalEffectReply,OriginalEffectReplyComplete,OriginalEffectAction,OriginalEffectInvocationIdentity,OriginalEffectCallbackMode,OriginalWorkBindingIdentity,OriginalSourceOwner,OriginalSourceMode,RetainedOwnedProjectionAdmission,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits};
use std::{mem::{ManuallyDrop,size_of},sync::Arc};
type ReadReply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type ReadComplete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
/// 🧳️ Erases the typed owner while preserving its actual borrowed Source and reply ports.
pub trait RetainedOriginalReadOwner<T>:Send+Sync{
 fn get(&self)->&T;
 fn factory(&self)->&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>;
 fn borrow(&self)->Result<RetainedCloneRef<'_,T>,ValueError>;
 fn retain_demand(&self,output:&RetainedOwnedProjectionAdmission<T>)->Result<RetirementDemand,ValueError>where T:Sync;
 fn retain_work_demand(&self,output:&RetainedOwnedProjectionAdmission<T>)->Result<u64,ValueError>where T:Sync;
 fn retain_into<'r>(&self,output:&mut RetainedOwnedProjectionAdmission<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>where T:Sync;
 fn birth_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn birth_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn advance_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>,recover:bool)->Result<ReadComplete<'r>,ValueError>;
 fn return_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn return_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn return_into<'r>(&mut self,original:&mut Option<T>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>,recover:bool)->Result<ReadComplete<'r>,ValueError>;
 fn requires_original_birth(&self)->bool;
 fn original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll;
 fn requires_original_control(&self)->bool;
 fn failure(&self)->Option<&ValueError>;
 fn failure_demand(&self)->RetirementDemand;
 fn take_failure_into<'r>(&mut self,output:&mut Option<ValueError>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>;
 fn resident_bytes(&self)->usize;
 fn terminal_is_empty(&self)->bool;
}
/// 🪪️ Keeps the actual original authority, projection action and handback in their resident places.
pub struct RetainedOriginalRead<T:Send+Sync+'static>{
 staged:ManuallyDrop<Option<FactoryOwnedSourceAuthority<T>>>,
 owner:OriginalSourceOwner<T,FactoryOwnedSourceAuthority<T>>,
 inner:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>,
 inner_reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,
 returning:bool,
 work:Option<OriginalWorkBindingIdentity>,
 route:Option<(OriginalEffectInvocationIdentity,OriginalSourceMode,usize)>,
}
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl<T:Send+Sync+'static> RetainedOriginalRead<T>{
 const TEMPLATE:ManuallyDrop<Self>=ManuallyDrop::new(Self::vacant());
 const fn vacant()->Self{Self{staged:ManuallyDrop::new(None),owner:OriginalSourceOwner::vacant(),inner:None,inner_reply:ManuallyDrop::new(None),returning:false,work:None,route:None}}
 pub fn initial_demands()->RetirementDemand{OriginalReadBirth::<T>::allocation_demand()}
 fn authenticate(&self,effect:&RetainedAllocationEffect<'_>)->Result<(),ValueError>{if !self.work.is_some_and(|work|work.matches_effect(effect)){return Err(fault("original slot read changed its actual Source or Work"))}Ok(())}
 fn authority(&self)->&FactoryOwnedSourceAuthority<T>{self.staged.as_ref().filter(|owner|!owner.terminal_is_empty()).or_else(||self.owner.original_authority()).expect("original read retains its actual authority")}
 fn quoted(&self,mode:OriginalSourceMode,body:usize)->Result<RetirementDemand,ValueError>{let mut d=self.owner.demand(mode,body)?.unwrap_or_default();if d.depth>0{d.copy_bytes=d.copy_bytes.checked_add(size_of_val(&self.route)*2).ok_or_else(||fault("original slot route extent overflow"))?}Ok(d)}
 fn drive_into<'r>(&mut self,mode:OriginalSourceMode,body:usize,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>,recover:bool)->Result<ReadComplete<'r>,ValueError>{
  self.authenticate(effect)?;let invocation=effect.invocation().ok_or_else(||fault("original slot read lacks its native invocation"))?;
  if invocation.mode()!=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive}{return Err(fault("original slot callback mode changed"))}
  let key=(invocation.identity(),mode,body);if recover&&self.route!=Some(key)||!recover&&self.route.is_some(){return Err(fault("original slot read route changed"))}
  if !recover&&self.inner.is_none()&&self.owner.demand(mode,body)?.is_some(){let quote=self.quoted(mode,body)?;if effect.remaining_work_units()?<self.owner.work_demand(mode,body)?||!crate::retained_clone_grant_admits(effect.grant(),quote){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}}
  let state=self;let mut scope=reply.progress().scope(&mut state.route,key)?;
  if state.owner.projection_drive_required_for(mode)?{
   let frame=state.owner.projection_frame(effect)?;let mut context=frame.context()?;let ready=frame.authority()?.drive(&mut context,effect,&mut state.inner)?;
   if ready{if !state.inner.as_mut().ok_or_else(||fault("original slot projection action absent"))?.transfer_reply(&mut state.inner_reply)?{return Ok(scope.forwarded())}frame.authority()?.finish(&mut state.inner)?;drop(semio_framework_value_retirement::forward_original_value_reply(&mut state.inner_reply,scope.borrow()?)?);}
  }else{
   let done=match(loan,recover){(None,false)=>state.owner.prepare_into(mode,body,effect,scope.borrow()?)?,(None,true)=>state.owner.recover_prepare_into(mode,body,effect,scope.borrow()?)?,(Some(loan),false)=>state.owner.prepare_original_into(mode,body,loan,effect,scope.borrow()?)?,(Some(loan),true)=>state.owner.recover_prepare_original_into(mode,body,loan,effect,scope.borrow()?)?};drop(done);
  }Ok(scope.forwarded())
 }
}
impl<T:Send+Sync+'static> RetainedOriginalReadOwner<T> for RetainedOriginalRead<T>{
 fn get(&self)->&T{self.authority().get()}
 fn factory(&self)->&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>{self.authority().factory()}
 fn borrow(&self)->Result<RetainedCloneRef<'_,T>,ValueError>{if self.returning{return Err(fault("original slot read is returning"))}self.owner.original()}
 fn retain_demand(&self,output:&RetainedOwnedProjectionAdmission<T>)->Result<RetirementDemand,ValueError>{output.demand(self.borrow()?)}
 fn retain_work_demand(&self,output:&RetainedOwnedProjectionAdmission<T>)->Result<u64,ValueError>{output.work_demand(self.borrow()?)}
 fn retain_into<'r>(&self,output:&mut RetainedOwnedProjectionAdmission<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.authenticate(effect)?;output.admit(self.borrow()?,effect,reply)}
 fn birth_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.returning{return Err(fault("returning original read cannot admit another reader"))}if self.owner.terminal_is_empty(){Ok(OriginalSourceOwner::<T,FactoryOwnedSourceAuthority<T>>::capture_demand())}else{self.quoted(OriginalSourceMode::Original,body)}}
 fn birth_work_demand(&self,body:usize)->Result<u64,ValueError>{self.birth_demands(body)?;if self.owner.terminal_is_empty(){Ok(1)}else{self.owner.work_demand(OriginalSourceMode::Original,body)}}
 fn advance_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>,recover:bool)->Result<ReadComplete<'r>,ValueError>{
  self.authenticate(effect)?;if self.returning{return Err(fault("returning original read cannot advance"))}
  if self.owner.terminal_is_empty(){if recover{return Err(fault("native original read capture has no unrecorded callback"))}let mut inner=None;drop(self.owner.capture_into(&mut self.staged,effect,OriginalEffectReply::borrow(&mut inner)?)?);if let Some(Ok(step))=inner.as_mut(){*step=RetainedCloneStep::Progress(step.progress())}return semio_framework_value_retirement::forward_original_value_reply(&mut inner,reply)}
  self.drive_into(OriginalSourceMode::Original,body,None,effect,reply,recover)
 }
 fn return_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.returning{return Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}
  if let Some(staged)=self.staged.as_ref(){return if staged.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryOwnedSourceAuthority<T>>>(),depth:1,..Default::default()})}else{Ok(FactoryOwnedSourceAuthority::<T>::transfer_original_demand())}}
  if self.owner.demand(OriginalSourceMode::Unique,body)?.is_none(){return Ok(self.owner.return_demand())}
  self.quoted(OriginalSourceMode::Unique,body)
 }
 fn return_work_demand(&self,body:usize)->Result<u64,ValueError>{self.return_demands(body)?;if !self.returning||self.staged.is_some()||self.owner.demand(OriginalSourceMode::Unique,body)?.is_none(){Ok(1)}else{self.owner.work_demand(OriginalSourceMode::Unique,body)}}
 fn return_into<'r>(&mut self,original:&mut Option<T>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>,recover:bool)->Result<ReadComplete<'r>,ValueError>{
  self.authenticate(effect)?;let body=effect.grant().maximum_copy_bytes;
  if recover{return self.drive_into(OriginalSourceMode::Unique,body,loan,effect,reply,true)}
  let d=self.return_demands(body)?;let units=self.return_work_demand(body)?;
  if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<units{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !self.returning{if !effect.performed(1)?{return Ok(reply.complete(Err(fault("original read return control exceeded Work").into())))}self.returning=true;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
  if let Some(staged)=self.staged.as_mut(){
   if staged.terminal_is_empty(){if !effect.performed(1)?{return Ok(reply.complete(Err(fault("empty original authority clear exceeded Work").into())))}self.staged.take();return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
   let mut inner=None;drop(staged.transfer_original_into(original,factory,effect,OriginalEffectReply::borrow(&mut inner)?)?);return semio_framework_value_retirement::forward_original_value_reply(&mut inner,reply.progress())
  }
  if self.owner.demand(OriginalSourceMode::Unique,body)?.is_none(){let mut inner=None;drop(self.owner.return_into(&mut self.staged,effect,OriginalEffectReply::borrow(&mut inner)?)?);return semio_framework_value_retirement::forward_original_value_reply(&mut inner,reply.progress())}
  self.drive_into(OriginalSourceMode::Unique,body,loan,effect,reply,false)
 }
 fn requires_original_birth(&self)->bool{self.returning&&self.owner.requires_original_birth()}
 fn original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.owner.original_birth_step(original,grant)}
 fn requires_original_control(&self)->bool{self.returning&&self.owner.requires_original_control()}
 fn failure(&self)->Option<&ValueError>{self.owner.failure()}
 fn failure_demand(&self)->RetirementDemand{self.owner.failure_demand()}
 fn take_failure_into<'r>(&mut self,output:&mut Option<ValueError>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.authenticate(effect)?;self.owner.take_failure_into(output,effect,reply)}
 fn resident_bytes(&self)->usize{size_of::<Self>()}
 fn terminal_is_empty(&self)->bool{self.returning&&self.staged.is_none()&&self.owner.terminal_is_empty()&&self.inner.is_none()&&self.inner_reply.is_none()&&self.route.is_none()}
}
impl<T:Send+Sync+'static> Drop for RetainedOriginalRead<T>{fn drop(&mut self){if std::thread::panicking(){return}assert!(self.terminal_is_empty(),"slot read retains its original authority or action");unsafe{ManuallyDrop::drop(&mut self.staged);ManuallyDrop::drop(&mut self.inner_reply)}}}

/// 🪪️ Identifies the actual original slot and its current authored generation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RetainedOriginalSlotKey{issuance:u64,generation:u64}
/// 🗃️ Holds one native original; writes and transfer fence until every immutable reader returns.
pub struct RetainedOriginalSlot<T>{original:Option<T>,read:Option<Box<dyn RetainedOriginalReadOwner<T>>>,birth:Option<OriginalReadBirth<T>>,issuance:u64,generation:u64}
static ORIGINAL_SLOT_ISSUANCE:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
impl<T> RetainedOriginalSlot<T>{
 pub fn new(original:T)->Self{let mut slot=Self::vacant();slot.original=Some(original);slot}
 /// 🪹️ Admits a fresh sealed resident address without inventing a semantic payload.
 pub fn vacant()->Self{Self{original:None,read:None,birth:None,issuance:ORIGINAL_SLOT_ISSUANCE.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|issuance|issuance.checked_add(1)).expect("original slot issuance exhausted"),generation:0}}
 /// 🪟️ Exclusive mutation borrows are available only while the original slot has no immutable source loan.
 pub fn get_mut_unissued(&mut self)->Option<&mut T>{if self.has_read(){None}else{self.original.as_mut()}}
 pub fn get(&self)->&T{self.original.as_ref().unwrap_or_else(||self.read.as_ref().expect("original slot retains value or source").get())}
 /// 🪹️ A withdrawn resident retains its sealed address without a fabricated payload.
 pub fn is_empty(&self)->bool{self.original.is_none()&&self.read.is_none()&&self.birth.is_none()}
 /// 📏️ Quotes the exact payload and slot-generation moves for an original handoff.
 pub fn transfer_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>()+size_of::<Option<T>>()+size_of::<u64>(),depth:1,..Default::default()}}
 /// 📤️ Withdraws the same original after every reader returns, preserving the resident slot identity.
 pub fn take_original(&mut self,key:RetainedOriginalSlotKey,output:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{self.admit_key(key)?;if self.has_read()||self.original.is_none()||output.is_some(){return Err(fault("original slot withdrawal requires its unique payload and empty recipient"));}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let d=Self::transfer_demand();if !retained_clone_grant_admits(grant,d){return Ok(None)}*output=self.original.take();self.generation=generation;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 /// 📥️ Adopts an actual original into its vacant resident without allocating an address or cloning a payload.
 pub fn install_original(&mut self,key:RetainedOriginalSlotKey,input:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{self.admit_key(key)?;if !self.is_empty()||input.is_none(){return Err(fault("original slot installation requires its vacant resident and original input"));}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let d=Self::transfer_demand();if !retained_clone_grant_admits(grant,d){return Ok(None)}self.original=input.take();self.generation=generation;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 /// 🧷️ Borrows the captured issuer of the resident read or its pending birth.
 pub fn read_issuer(&self)->Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>{self.read.as_ref().map(|read|read.factory()).or_else(||self.birth.as_ref().and_then(|birth|birth.issuer()))}
 pub fn key(&self)->RetainedOriginalSlotKey{RetainedOriginalSlotKey{issuance:self.issuance,generation:self.generation}}
 fn admit_key(&self,key:RetainedOriginalSlotKey)->Result<(),ValueError>{if self.key()==key{Ok(())}else{Err(fault("original slot read belongs to another source generation"))}}
 pub fn has_read(&self)->bool{self.read.is_some()||self.birth.is_some()}
 pub fn try_into_original(mut self)->Result<T,Self>{if self.has_read()||self.original.is_none(){return Err(self);}Ok(self.original.take().unwrap())}
 /// 📏️ Quotes only original field exchange; variable payload allocations keep their native owners.
 pub fn replace_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<T>()+size_of::<Option<T>>()+size_of::<u64>(),depth:1,..Default::default()}}
 /// 🧮️ Quotes one exclusive ownership transform and its caller-owned result without duplicating a payload.
 pub fn transform_demand<R>()->RetirementDemand{RetirementDemand{copy_bytes:3*size_of::<T>()+size_of::<Option<T>>()+2*size_of::<R>()+size_of::<u64>(),depth:1,..Default::default()}}
 /// 🔀️ A pre-admitted infallible move transforms one sealed native row after every immutable reader returns.
 pub fn transform_original<R>(&mut self,key:RetainedOriginalSlotKey,grant:RetainedCloneGrant,transform:impl FnOnce(Option<T>)->(Option<T>,R))->Result<Option<(R,RetainedCloneProgress)>,ValueError>{self.admit_key(key)?;if self.has_read(){return Err(fault("original slot transform requires every reader returned"))}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let demand=Self::transform_demand::<R>();if grant.maximum_items==0||!retained_clone_grant_admits(grant,demand){return Ok(None)}let(next,result)=transform(self.original.take());self.original=next;self.generation=generation;Ok(Some((result,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
 /// 🔁️ Exchanges actual native fields after all readers return and every original currency is admitted.
 pub fn replace(&mut self,original:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<(T,RetainedCloneProgress)>,ValueError>{if self.has_read(){return Err(fault("original slot mutation requires every reader returned"));}let generation=self.generation.checked_add(1).ok_or_else(||fault("original slot generation exhausted"))?;let d=Self::replace_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}let next=original.take().ok_or_else(||fault("original slot replacement absent"))?;let previous=self.original.replace(next).expect("owned original slot value");self.generation=generation;Ok(Some((previous,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn demands(&self,key:RetainedOriginalSlotKey,body:usize)->Result<RetirementDemand,ValueError>where T:Send+Sync+'static{self.admit_key(key)?;if let Some(birth)=self.birth.as_ref(){return birth.next_demand()}self.read.as_ref().map_or(Ok(RetainedOriginalRead::<T>::initial_demands()),|read|read.birth_demands(body))}
 pub fn read_work_demand(&self,key:RetainedOriginalSlotKey,body:usize)->Result<u64,ValueError>where T:Send+Sync+'static{self.demands(key,body)?;self.read.as_ref().map_or(Ok(1),|read|read.birth_work_demand(body))}
 pub fn advance_read_into<'r>(&mut self,key:RetainedOriginalSlotKey,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>where T:Send+Sync+'static{
  self.admit_key(key)?;if let Some(read)=self.read.as_mut(){if !Arc::ptr_eq(read.factory(),factory){return Err(fault("original slot issuer differs from captured authority"))}return read.advance_into(effect.grant().maximum_copy_bytes,effect,reply,false)}
     if let Some(birth)=self.birth.as_mut(){if !birth.matches_factory(factory){return Err(fault("original read birth changed its captured issuer"))}let done=birth.advance_into(&mut self.original,&mut self.read,effect,reply)?;if birth.terminal_is_empty(){self.birth.take();}return Ok(done)}
   if self.original.is_none(){return Err(fault("original slot read payload absent"))}OriginalReadBirth::start_into(&mut self.birth,factory,effect,reply)
 }
 pub fn recover_advance_read_into<'r>(&mut self,key:RetainedOriginalSlotKey,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>where T:Send+Sync+'static{self.admit_key(key)?;let read=self.read.as_mut().ok_or_else(||fault("original read recovery has no resident owner"))?;if !Arc::ptr_eq(read.factory(),factory){return Err(fault("original slot recovery changed issuer"))}read.advance_into(effect.grant().maximum_copy_bytes,effect,reply,true)}
 pub fn borrow(&self,key:RetainedOriginalSlotKey)->Result<RetainedCloneRef<'_,T>,ValueError>{self.admit_key(key)?;self.read.as_ref().ok_or_else(||fault("original slot Source birth is incomplete"))?.borrow()}
 pub fn retain_read_demand(&self,key:RetainedOriginalSlotKey,output:&RetainedOwnedProjectionAdmission<T>)->Result<RetirementDemand,ValueError>where T:Sync{self.admit_key(key)?;self.read.as_ref().ok_or_else(||fault("original slot Source birth is incomplete"))?.retain_demand(output)}
 pub fn retain_read_work_demand(&self,key:RetainedOriginalSlotKey,output:&RetainedOwnedProjectionAdmission<T>)->Result<u64,ValueError>where T:Sync{self.admit_key(key)?;self.read.as_ref().ok_or_else(||fault("original slot Source birth is incomplete"))?.retain_work_demand(output)}
 pub fn retain_read_into<'r>(&self,key:RetainedOriginalSlotKey,output:&mut RetainedOwnedProjectionAdmission<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>where T:Sync{self.admit_key(key)?;self.read.as_ref().ok_or_else(||fault("original slot Source birth is incomplete"))?.retain_into(output,effect,reply)}
 pub fn return_demands(&self,key:RetainedOriginalSlotKey,body:usize)->Result<RetirementDemand,ValueError>{self.admit_key(key)?;if let Some(birth)=self.birth.as_ref(){return Ok(birth.return_demand())}self.read.as_ref().map_or(Ok(Default::default()),|read|if read.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&self.read),release_bytes:read.resident_bytes(),depth:1,..Default::default()})}else{read.return_demands(body)})}
 pub fn return_work_demand(&self,key:RetainedOriginalSlotKey,body:usize)->Result<u64,ValueError>{self.admit_key(key)?;if self.birth.is_some(){return Ok(1)}self.read.as_ref().map_or(Ok(0),|read|if read.terminal_is_empty(){Ok(1)}else{read.return_work_demand(body)})}
 fn return_dispatch<'r>(&mut self,key:RetainedOriginalSlotKey,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>,recover:bool)->Result<ReadComplete<'r>,ValueError>{
  self.admit_key(key)?;if let Some(birth)=self.birth.as_mut(){if recover{return Err(fault("native read birth cancellation has no unrecorded callback"))}let done=birth.return_into(factory,effect,reply)?;if birth.terminal_is_empty(){self.birth.take();}return Ok(done)}let Some(read)=self.read.as_mut()else{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))};
  if read.terminal_is_empty(){if recover{return Err(fault("native read frame release has no unrecorded callback"))}let d=RetirementDemand{copy_bytes:size_of::<Option<Box<dyn RetainedOriginalReadOwner<T>>>>(),release_bytes:read.resident_bytes(),depth:1,..Default::default()};if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(fault("original read frame release exceeded Work").into())))}self.read.take();return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))))}
  read.return_into(&mut self.original,factory,loan,effect,reply.progress(),recover)
 }
 pub fn return_read_into<'r>(&mut self,key:RetainedOriginalSlotKey,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{self.return_dispatch(key,factory,None,effect,reply,false)}
 pub fn recover_return_read_into<'r>(&mut self,key:RetainedOriginalSlotKey,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{self.return_dispatch(key,factory,None,effect,reply,true)}
 pub fn return_read_original_into<'r>(&mut self,key:RetainedOriginalSlotKey,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{self.return_dispatch(key,factory,Some(original),effect,reply,false)}
 pub fn recover_return_read_original_into<'r>(&mut self,key:RetainedOriginalSlotKey,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,original:&mut OriginalRetirementCloseLoan<'_>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{self.return_dispatch(key,factory,Some(original),effect,reply,true)}
 pub fn read_requires_original_birth(&self)->bool{self.read.as_ref().is_some_and(|read|read.requires_original_birth())}
 pub fn read_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.read.as_mut().expect("selected original read birth").original_birth_step(original,grant)}
 pub fn read_requires_original_control(&self)->bool{self.read.as_ref().is_some_and(|read|read.requires_original_control())}
 pub fn read_failure(&self)->Option<&ValueError>{self.read.as_ref().and_then(|read|read.failure())}
 pub fn read_failure_demand(&self)->Result<RetirementDemand,ValueError>{self.read.as_ref().map(|read|read.failure_demand()).ok_or_else(||fault("original read owner absent"))}
 pub fn take_read_failure_into<'r>(&mut self,output:&mut Option<ValueError>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{self.read.as_mut().ok_or_else(||fault("original read owner absent"))?.take_failure_into(output,effect,reply)}

}
impl<T> Drop for RetainedOriginalSlot<T>{fn drop(&mut self){assert!(!self.has_read()||std::thread::panicking(),"original slot dropped with immutable readers");}}

impl<T:std::fmt::Debug> std::fmt::Debug for RetainedOriginalSlot<T>{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{if self.is_empty(){f.write_str("Vacant")}else{self.get().fmt(f)}}}
impl<T:PartialEq> PartialEq for RetainedOriginalSlot<T>{fn eq(&self,other:&Self)->bool{match(self.is_empty(),other.is_empty()){(true,true)=>true,(false,false)=>self.get()==other.get(),_=>false}}}
impl<T:Eq> Eq for RetainedOriginalSlot<T>{}

#[path="🌱️birth/🦀️.rs"]
mod birth;
use birth::OriginalReadBirth;

#[path="♻️retirement/🦀️.rs"]
mod retirement;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// ⏱️ Records the work a turn reports but its effect never saw, once, so the receipt equals the report.
fn record_owed_work<T,E:From<ValueError>>(effect:&mut crate::RetainedAllocationEffect<'_>,start:u128,mut turn:crate::RetainedWorkOutcome<T,E>)->crate::RetainedWorkOutcome<T,E>{let performed=effect.performed_work().saturating_sub(start);let owed=turn.work.saturating_sub(performed);if owed>0{let refusal=match u64::try_from(owed){Ok(owed)=>match effect.performed(owed){Ok(true)=>None,Ok(false)=>Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original slot work exceeded its authority")),Err(error)=>Some(error)},Err(_)=>Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original slot work exceeds its receipt"))};if let (Some(error),true)=(refusal,turn.result.is_ok()){turn.result=Err(E::from(error))}}turn}
