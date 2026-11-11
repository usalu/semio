//! 📇️ One original key admission publishes immutable sorted ranks for repeated sparse planning.
use super::{NamedEditPlan,NamedEditSource,NamedEditLane,NamedEditStep,NamedEditFault,NamedRankedSource,invalid};
use crate::{IndexedEditShape,RetirementDemand,ValueError,ValueRefusalKind,RetainedWorkOutcome,OriginalRetirementCloseLoan,OriginalRetirementFailure,indexed_edit::{CloseKey,CloseReply,Reply,Done,invocation_identity,loan_key},retirement::{RetireOwned,RetirementCursor,RetirementStep},shared_issuance::{SealedShared,SharedIssuer},retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
/// 📬️ Runs one native child reply to completion inside the caller's own action and reports the work it performed; a reply left pending is an invariant violation.
pub(crate) fn settle_reply<T,E:From<ValueError>>(effect:&mut crate::RetainedAllocationEffect<'_>,call:impl for<'r,'e,'a> FnOnce(&'e mut crate::RetainedAllocationEffect<'a>,OriginalEffectReply<'r,T,E>)->Result<OriginalEffectReplyComplete<'r,T,E>,ValueError>)->RetainedWorkOutcome<T,E>{
 let before=effect.performed_work();let mut slot=None;
 let settled=(||->Result<Result<T,E>,ValueError>{let done=call(&mut *effect,OriginalEffectReply::borrow(&mut slot)?)?;drop(done);slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original child left its producer reply pending inside one action"))})();
 let work=effect.performed_work().saturating_sub(before);RetainedWorkOutcome{result:match settled{Ok(result)=>result,Err(error)=>Err(error.into())},work}
}
/// 🪢️ Generates the four reply-shaped close entries of a named cursor over fresh or recovered invocations with ordinary or original-loan control.
macro_rules! named_close{([$($generic:tt)*] $type:ty,$dispatch:ident)=>{
 impl<$($generic)*> $type{
  pub fn close_into<'r>(&mut self,body:usize,effect:&mut $crate::RetainedAllocationEffect<'_>,reply:$crate::indexed_edit::Reply<'r>)->Result<$crate::indexed_edit::Done<'r>,$crate::ValueError>{self.$dispatch(None,body,effect,reply,false)}
  pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut $crate::RetainedAllocationEffect<'_>,reply:$crate::indexed_edit::Reply<'r>)->Result<$crate::indexed_edit::Done<'r>,$crate::ValueError>{self.$dispatch(None,body,effect,reply,true)}
  pub fn close_original_into<'r>(&mut self,original:&mut $crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut $crate::RetainedAllocationEffect<'_>,reply:$crate::indexed_edit::Reply<'r>)->Result<$crate::indexed_edit::Done<'r>,$crate::ValueError>{if self.requires_original_birth(){return Err($crate::ValueError::invariant("named close requires its selected original birth"))}if !self.requires_original_control(){return self.$dispatch(None,body,effect,reply,false)}self.$dispatch(Some(original),body,effect,reply,false)}
  pub fn recover_close_original_into<'r>(&mut self,original:&mut $crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut $crate::RetainedAllocationEffect<'_>,reply:$crate::indexed_edit::Reply<'r>)->Result<$crate::indexed_edit::Done<'r>,$crate::ValueError>{if self.close_route.is_some_and(|(_,_,_,loan)|loan.is_none()){return self.$dispatch(None,body,effect,reply,true)}self.$dispatch(Some(original),body,effect,reply,true)}
 }
};}
/// 🧳️ A named owner exposes its fixed close quotes and one selected child close to the shared retirement relay.
#[doc(hidden)]
pub trait NamedOwner:Send+'static{
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn work(&self,body:usize)->Result<u64,ValueError>;
 fn close<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>;
 fn original_birth(&self)->bool;
 fn original_control(&self)->bool;
 fn birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome;
 fn empty(&self)->bool;
}
/// ♻️ The shared retirement relay retains the original child reply and Work frontier across suspension.
#[doc(hidden)]
pub struct NamedRetirement<C:NamedOwner>{cursor:ManuallyDrop<C>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,route:Option<(CloseKey,u128)>}
impl<C:NamedOwner> NamedRetirement<C>{
 pub fn seed(cursor:C)->Self{Self{cursor:ManuallyDrop::new(cursor),reply:ManuallyDrop::new(None),route:None}}
 fn relay<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{
  let identity=invocation_identity(effect,recover)?;let loan=match original.as_deref_mut(){Some(loan)=>Some(loan_key(loan,effect)?),None=>None};let key=(identity,0,body,loan);let lost=||ValueError::invariant("named retirement lost its retained reply");
  if recover{if !self.route.is_some_and(|(selected,_)|selected==key){return Err(lost())}}else{if self.route.is_some()||self.reply.is_some(){return Err(lost())}self.route=Some((key,effect.performed_work()))}
  if self.reply.is_none(){let done=self.cursor.close(original,body,effect,OriginalEffectReply::borrow(&mut self.reply)?,recover)?;drop(done)}
  let Some(result)=self.reply.take()else{return Ok(reply.pending())};let(_,start)=self.route.take().ok_or_else(lost)?;let work=effect.performed_work().checked_sub(start).ok_or_else(lost)?;
  Ok(reply.complete(result.map(|step|RetirementStep::Progress(step.progress(),work))))
 }
}
impl<C:NamedOwner> RetirementCursor for NamedRetirement<C>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.route.is_some_and(|((_,_,selected,_),_)|selected!=body){return Err(ValueError::invariant("named retirement changed its selected body"))}self.cursor.demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_demand(body)?;self.cursor.work(body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.cursor.original_birth()}
 fn requires_original_control(&self)->bool{self.route.is_some_and(|((_,_,_,loan),_)|loan.is_some())||self.cursor.original_control()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{let turn=self.cursor.birth_step(original,grant);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}
 fn terminal_is_empty(&self)->bool{self.route.is_none()&&self.reply.is_none()&&self.cursor.empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<C:NamedOwner> Drop for NamedRetirement<C>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cursor);ManuallyDrop::drop(&mut self.reply)}}else if !std::thread::panicking(){panic!("named retirement retains its original owner")}}}
#[path="♻️transfer/🦀️.rs"]mod transfer;
pub use transfer::KeyPreservingPersistentSet;
#[path="🌳️root/🦀️.rs"]mod root;
pub use root::{NamedPersistentRoot,NamedPersistentRootPreparation,NamedRecordKey,NamedPersistentRootReceiver};
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
/// 🔐️ Private immutable coordinates retain their genuine original key root and sorted directory issuer.
pub struct AdmittedNamedKeyIndex{original:ManuallyDrop<Option<RetainedCloneBinding>>,directory:ManuallyDrop<Option<SealedShared<Vec<usize>>>>,retirement:RetainedCloneClose,len:usize,projection:usize,closing:bool,close_route:Option<CloseKey>}
impl AdmittedNamedKeyIndex{
 pub(super) fn new(original:Option<RetainedCloneBinding>,directory:Option<SealedShared<Vec<usize>>>,len:usize,projection:usize)->Self{Self{original:ManuallyDrop::new(original),directory:ManuallyDrop::new(directory),retirement:Default::default(),len,projection,closing:false,close_route:None}}
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
 pub fn ready(&self)->bool{!self.closing&&self.original.is_some()&&self.directory.is_some()}
 pub(super) fn order(&self)->&[usize]{self.directory.as_ref().expect("admitted original named key directory").get()}
 /// 🧪️ Test targets read the sealed sorted directory through the testkit feature only.
 #[cfg(feature="testkit")] #[doc(hidden)] pub fn testkit_order(&self)->&[usize]{self.order()}
 pub(super) fn same_original(&self,other:&Self)->bool{self.ready()&&other.ready()&&self.len==other.len&&self.projection==other.projection&&self.directory.as_ref().unwrap().identity()==other.directory.as_ref().unwrap().identity()}
 /// 🪪️ Equal content does not authorize a different original root or different key projection selector.
 pub fn check_source(&self,source:&impl NamedRankedSource)->Result<(),ValueError>{if !self.ready()||self.len!=source.len()||self.projection!=source.key_projection()||!source.original().check_binding(&self.original)?{return Err(invalid());}Ok(())}
 /// 🎟️ Reuse duplicates two fixed original authorities and performs no base scan or allocation.
 pub fn project_demand(&self,source:&impl NamedRankedSource)->Result<RetirementDemand,ValueError>{self.check_source(source)?;Ok(fixed(size_of::<Self>()*2+source.original().binding_copy_bytes()+size_of::<SealedShared<Vec<usize>>>()))}
 pub fn project(&self,source:&impl NamedRankedSource,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let grant=effect.grant();let mut work=0;let result=(||{
  let d=self.project_demand(source)?;
  if let Some(owner)=output.as_ref(){if owner.closing||owner.len!=self.len||owner.projection!=self.projection||!source.original().check_binding(&owner.original)?||match(owner.directory.as_ref(),self.directory.as_ref()){(Some(owned),Some(own))=>owned.identity()!=own.identity(),(Some(_),None)=>true,_=>false}{return Err(invalid())}if owner.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}}
  if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if output.is_none(){*output=Some(Self::new(None,None,self.len,self.projection));work+=1;}
  let owner=output.as_mut().ok_or_else(||ValueError::invariant("output absent"))?;
  if owner.original.is_none(){let _=take_work!(work,source.original().bind(&mut owner.original,effect)).ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(paid(d)))}
  let(directory,_)=take_work!(work,self.directory.as_ref().ok_or_else(||ValueError::invariant("directory absent"))?.try_duplicate(effect));*owner.directory=Some(directory);
  Ok(RetainedCloneStep::Complete(paid(d)))
 })();crate::finish(effect,work,result)}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if self.directory.is_some()||!self.retirement.is_empty(){return self.retirement.next_owner_demands::<SealedShared<Vec<usize>>>(self.directory.is_some(),&None,body);}RetainedCloneBinding::close_demand(&self.original,body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if self.directory.is_some()||!self.retirement.is_empty(){return self.retirement.next_owner_work::<SealedShared<Vec<usize>>>(self.directory.is_some(),&None,body)}RetainedCloneBinding::close_work_demand(&self.original,body)}
 fn close_reply_index<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if !self.closing{return Err(invalid())}
  if !recover&&self.close_route.is_none()&&self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let identity=invocation_identity(effect,recover)?;let body=if recover{self.close_route.map_or(body,|key|key.2)}else{body};let loan=match original.as_deref_mut(){Some(loan)=>Some(loan_key(loan,effect)?),None=>None};
  if recover{
   let key=self.close_route.ok_or_else(invalid)?;if key.0!=identity||key.3!=loan{return Err(ValueError::invariant("named key index close recovery changed its invocation, body or loan"))}
   let mut scope=reply.progress().scope(&mut self.close_route,key)?;let inner=scope.borrow()?;
   let done=if key.1==1{self.retirement.close_reply(original,body,effect,inner,true)}else{RetainedCloneBinding::recover_close_into(&mut self.original,body,effect,inner)}?;drop(done);return Ok(scope.forwarded());
  }
  if self.close_route.is_some(){return Err(ValueError::invariant("named key index close requires recovery"))}
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let grant=effect.grant();let demand=self.close_demand(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !self.retirement.is_empty(){let mut scope=reply.progress().scope(&mut self.close_route,(identity,1,body,loan))?;let inner=scope.borrow()?;let done=self.retirement.close_reply(original,body,effect,inner,false)?;drop(done);return Ok(scope.forwarded())}
  if self.directory.is_some(){let turn=self.retirement.begin_granted(&mut self.directory,effect);return Ok(reply.complete(turn.result.map(|step|RetainedCloneStep::Progress(step.map_or_else(Default::default,RetainedCloneStep::progress))).map_err(Into::into)))}
  let mut scope=reply.progress().scope(&mut self.close_route,(identity,2,body,loan))?;let inner=scope.borrow()?;let done=RetainedCloneBinding::close_into(&mut self.original,body,effect,inner)?;drop(done);Ok(scope.forwarded())
 }
 pub fn requires_original_birth(&self)->bool{self.retirement.requires_original_birth()}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{crate::Settle::settled(self.retirement.step_original_birth(original,grant))}
 pub fn requires_original_control(&self)->bool{self.retirement.requires_original_control()}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.directory.is_none()&&self.retirement.is_empty()&&self.close_route.is_none()}
}
impl Drop for AdmittedNamedKeyIndex{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"immutable named key index requires original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.directory);}}}}
named_close!{[] AdmittedNamedKeyIndex,close_reply_index}
impl CloseReply for AdmittedNamedKeyIndex{fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.close_reply_index(original,body,effect,reply,recover)}}
impl NamedOwner for AdmittedNamedKeyIndex{
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing{self.close_demand(body)}else{Ok(fixed(0))}}
 fn work(&self,body:usize)->Result<u64,ValueError>{if self.closing{self.close_work_demand(body)}else{Ok(1)}}
 fn close<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if !self.closing{if recover{return Err(invalid())}if effect.remaining_work_units()?==0||!retained_clone_grant_admits(effect.grant(),fixed(0)){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"named key index close start exceeded its original work").into())))}self.begin_close();return Ok(reply.complete(Ok(RetainedCloneStep::Progress(paid(fixed(0))))))}
  match(original,recover){(None,false)=>self.close_into(body,effect,reply),(None,true)=>self.recover_close_into(body,effect,reply),(Some(loan),false)=>self.close_original_into(loan,body,effect,reply),(Some(loan),true)=>self.recover_close_original_into(loan,body,effect,reply)}
 }
 fn original_birth(&self)->bool{self.closing&&self.requires_original_birth()}
 fn original_control(&self)->bool{self.closing&&self.requires_original_control()}
 fn birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{self.close_original_birth_step(original,grant)}
 fn empty(&self)->bool{self.closing&&self.terminal_is_empty()}
}
impl RetireOwned for AdmittedNamedKeyIndex{type RetirementCursor=NamedRetirement<Self>;fn retirement_seed(self)->Self::RetirementCursor{NamedRetirement::seed(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<NamedRetirement<Self>>())}fn controlled_retirement_supported()->bool{true}}
struct AdmissionSource<'a,S:NamedRankedSource>{source:&'a S}
impl<S:NamedRankedSource> NamedEditSource for AdmissionSource<'_,S>{
 fn shape(&self)->IndexedEditShape{IndexedEditShape{base_len:self.source.len(),removed:0,modified:0,added:0}}
 fn key_demand(&self,lane:NamedEditLane,ordinal:usize)->Result<RetirementDemand,ValueError>{if lane!=NamedEditLane::Base{return Err(invalid());}self.source.rank_demand(ordinal)}
 fn observe_key(&self,lane:NamedEditLane,ordinal:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(&str,RetainedCloneProgress)>>{let mut work=0;let result=(||{if lane!=NamedEditLane::Base{return Err(invalid());}Ok(take_work!(work,self.source.observe_key(ordinal,grant)).map(|(key,p)|(key.get(),p)))})();crate::RetainedWorkOutcome{result,work}}
 fn added_index(&self,_:usize)->crate::RetainedWorkOutcome<usize>{crate::RetainedWorkOutcome{result:Err(invalid()),work:0}}
}
/// 📥️ Admission reuses the genuine named planner and closes it before sealing its sorted original directory.
pub struct NamedKeyIndexPreparation{original:ManuallyDrop<Option<RetainedCloneBinding>>,order:ManuallyDrop<Option<Vec<usize>>>,directory:ManuallyDrop<Option<SealedShared<Vec<usize>>>>,plan:Option<NamedEditPlan>,retirement:RetainedCloneClose,len:usize,projection:usize,phase:u8,fault:Option<NamedEditFault>,closing:bool,close_route:Option<CloseKey>}
impl NamedKeyIndexPreparation{
 pub fn constructor_demand()->RetirementDemand{fixed(size_of::<Self>()*2)}
 pub fn new(len:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<Self>>{let mut work=0;let result=(||{if !retained_clone_grant_admits(grant,Self::constructor_demand()){return Ok(None);}work=1;Ok(Some(Self{original:ManuallyDrop::new(None),order:ManuallyDrop::new(None),directory:ManuallyDrop::new(None),plan:None,retirement:Default::default(),len,projection:0,phase:0,fault:None,closing:false,close_route:None}))})();crate::RetainedWorkOutcome{result,work}}
 fn validate(&self,source:&impl NamedRankedSource)->Result<(),ValueError>{if self.closing||self.phase>6||self.fault.is_some()||source.len()!=self.len{return Err(invalid());}if self.original.is_some()&&(self.projection!=source.key_projection()||!source.original().check_binding(&self.original)?){return Err(invalid());}Ok(())}
 pub fn demand(&self,source:&impl NamedRankedSource,body:usize)->Result<RetirementDemand,ValueError>{self.validate(source)?;match self.phase{0=>Ok(fixed(size_of::<Self>()+source.original().binding_copy_bytes())),1=>Ok(fixed(size_of::<NamedEditPlan>()*2)),2=>self.plan.as_ref().ok_or_else(invalid)?.demand(&AdmissionSource{source},body),3=>self.plan.as_ref().ok_or_else(invalid)?.base_take_demand(),4=>if self.plan.as_ref().ok_or_else(invalid)?.terminal_is_empty(){Ok(fixed(size_of::<NamedEditPlan>()*2))}else{self.plan.as_ref().ok_or_else(invalid)?.close_demand(body)},5=>{let mut d=SealedShared::<Vec<usize>>::source_admission_demand()?;d.copy_bytes=d.copy_bytes.checked_add(size_of::<Vec<usize>>()*3+size_of::<SealedShared<Vec<usize>>>()*2).ok_or_else(invalid)?;d.depth=d.depth.max(1);Ok(d)},6=>Ok(Default::default()),_=>Err(invalid())}}
 pub fn advance(&mut self,source:&impl NamedRankedSource,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<NamedEditStep,crate::OriginalRetirementFailure>{let grant=effect.grant();let mut work=0;let result=(||->Result<NamedEditStep,crate::OriginalRetirementFailure>{let d=self.demand(source,grant.maximum_copy_bytes)?;if self.phase==6{return Ok(NamedEditStep::Admitted(Default::default()))}if !retained_clone_grant_admits(grant,d){return Ok(NamedEditStep::Progress(Default::default()));}let mut p=paid(d);match self.phase{
  0=>{take_work!(work,source.original().bind(&mut self.original,effect)).ok_or_else(invalid)?;self.projection=source.key_projection();self.phase=1;work+=1;},
  1=>{work+=1;self.plan=Some(NamedEditPlan::new(AdmissionSource{source}.shape()));self.phase=2;},
  2=>{let step=take_work!(work,self.plan.as_mut().ok_or_else(||ValueError::invariant("plan absent"))?.advance(&AdmissionSource{source},effect));p=step.progress();match step{NamedEditStep::Admitted(_)=>{self.phase=3;work+=1;},NamedEditStep::Refused(fault,_)=>{self.fault=Some(fault);work+=1;return Ok(NamedEditStep::Refused(fault,p));},_=>{}}},
  3=>{*self.order=Some(take_work!(work,self.plan.as_mut().ok_or_else(||ValueError::invariant("plan absent"))?.take_base_order(grant)).ok_or_else(invalid)?.0);self.plan.as_mut().ok_or_else(||ValueError::invariant("plan absent"))?.begin_close();self.phase=4;work+=1;},
  4=>{if self.plan.as_ref().ok_or_else(||ValueError::invariant("plan absent"))?.terminal_is_empty(){self.plan=None;self.phase=5;work+=1;}else{p=take_work!(work,settle_reply(effect,|child,reply|self.plan.as_mut().ok_or_else(||ValueError::invariant("plan absent"))?.close_into(grant.maximum_copy_bytes,child,reply.progress()))).progress();}},
  5=>{let rows=self.order.take().ok_or_else(invalid)?;work+=1;let mut input=Some(rows);let mut output=None;let turn=SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect);work+=turn.work;match(turn.result,output.take()){(Ok(step),Some(directory))=>{*self.directory=Some(directory);p.retained_capacity_bytes=step.progress().retained_capacity_bytes;self.phase=6;},(outcome,_)=>{*self.order=input.take();work+=1;return Err(outcome.err().unwrap_or_else(||ValueError::invariant("directory birth is not funded")).into());}}},
  6=>return Ok(NamedEditStep::Admitted(Default::default())),_=>return Err(invalid().into()),
 }Ok(if self.phase==6{NamedEditStep::Admitted(p)}else{NamedEditStep::Progress(p)})})();crate::finish(effect,work,result)}
 pub fn output_ready(&self)->bool{!self.closing&&self.phase==6&&self.fault.is_none()&&self.directory.is_some()}
 pub fn fault(&self)->Option<NamedEditFault>{self.fault}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(size_of::<Self>()+size_of::<AdmittedNamedKeyIndex>()*2))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(AdmittedNamedKeyIndex,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}work=1;let index=AdmittedNamedKeyIndex::new(self.original.take(),self.directory.take(),self.len,self.projection);self.phase=7;Ok(Some((index,paid(d))))})();crate::RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(plan)=self.plan.as_mut(){plan.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if let Some(plan)=self.plan.as_ref(){return if plan.terminal_is_empty(){Ok(fixed(size_of::<NamedEditPlan>()*2))}else{plan.close_demand(body)};}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Vec<usize>>(false,&None,body);}if self.order.is_some(){return self.retirement.next_owner_demands::<Vec<usize>>(true,&None,body);}if self.directory.is_some(){return self.retirement.next_owner_demands::<SealedShared<Vec<usize>>>(true,&None,body);}RetainedCloneBinding::close_demand(&self.original,body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if let Some(plan)=self.plan.as_ref(){return if plan.terminal_is_empty(){Ok(1)}else{plan.close_work_demand(body)}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<Vec<usize>>(false,&None,body)}if self.order.is_some(){return self.retirement.next_owner_work::<Vec<usize>>(true,&None,body)}if self.directory.is_some(){return self.retirement.next_owner_work::<SealedShared<Vec<usize>>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.original,body)}
 fn close_reply_preparation<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  if !self.closing{return Err(invalid())}
  if !recover&&self.close_route.is_none()&&self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let identity=invocation_identity(effect,recover)?;let body=if recover{self.close_route.map_or(body,|key|key.2)}else{body};let loan=match original.as_deref_mut(){Some(loan)=>Some(loan_key(loan,effect)?),None=>None};
  if recover{
   let key=self.close_route.ok_or_else(invalid)?;if key.0!=identity||key.3!=loan{return Err(ValueError::invariant("named key preparation close recovery changed its invocation, body or loan"))}
   let mut scope=reply.progress().scope(&mut self.close_route,key)?;let inner=scope.borrow()?;
   let done=match key.1{1=>self.plan.as_mut().ok_or_else(invalid)?.close_reply(original,body,effect,inner,true),2=>self.retirement.close_reply(original,body,effect,inner,true),_=>RetainedCloneBinding::recover_close_into(&mut self.original,body,effect,inner)}?;drop(done);return Ok(scope.forwarded());
  }
  if self.close_route.is_some(){return Err(ValueError::invariant("named key preparation close requires recovery"))}
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let grant=effect.grant();let demand=self.close_demand(body)?;if !retained_clone_grant_admits(grant,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if let Some(plan)=self.plan.as_mut(){
   if plan.terminal_is_empty(){if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"named key preparation plan release exceeded its original work").into())))}self.plan=None;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(paid(demand)))))}
   let mut scope=reply.progress().scope(&mut self.close_route,(identity,1,body,loan))?;let inner=scope.borrow()?;let done=plan.close_reply(original,body,effect,inner,false)?;drop(done);return Ok(scope.forwarded());
  }
  if !self.retirement.is_empty(){let mut scope=reply.progress().scope(&mut self.close_route,(identity,2,body,loan))?;let inner=scope.borrow()?;let done=self.retirement.close_reply(original,body,effect,inner,false)?;drop(done);return Ok(scope.forwarded())}
  if self.order.is_some(){let turn=self.retirement.begin_granted(&mut self.order,effect);return Ok(reply.complete(turn.result.map(|step|RetainedCloneStep::Progress(step.map_or_else(Default::default,RetainedCloneStep::progress))).map_err(Into::into)))}
  if self.directory.is_some(){let turn=self.retirement.begin_granted(&mut self.directory,effect);return Ok(reply.complete(turn.result.map(|step|RetainedCloneStep::Progress(step.map_or_else(Default::default,RetainedCloneStep::progress))).map_err(Into::into)))}
  let mut scope=reply.progress().scope(&mut self.close_route,(identity,3,body,loan))?;let inner=scope.borrow()?;let done=RetainedCloneBinding::close_into(&mut self.original,body,effect,inner)?;drop(done);Ok(scope.forwarded())
 }
 pub fn requires_original_birth(&self)->bool{if let Some(plan)=self.plan.as_ref(){return !plan.terminal_is_empty()&&plan.requires_original_birth()}self.retirement.requires_original_birth()}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementOutcome{if let Some(plan)=self.plan.as_mut(){return plan.close_original_birth_step(original,grant)}crate::Settle::settled(self.retirement.step_original_birth(original,grant))}
 pub fn requires_original_control(&self)->bool{if let Some(plan)=self.plan.as_ref(){return !plan.terminal_is_empty()&&plan.requires_original_control()}self.retirement.requires_original_control()}
 pub fn terminal_is_empty(&self)->bool{self.plan.is_none()&&self.original.is_none()&&self.order.is_none()&&self.directory.is_none()&&self.retirement.is_empty()&&self.close_route.is_none()}
}
impl Drop for NamedKeyIndexPreparation{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"named key admission requires original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.order);ManuallyDrop::drop(&mut self.directory);}}}}
named_close!{[] NamedKeyIndexPreparation,close_reply_preparation}
impl CloseReply for NamedKeyIndexPreparation{fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{self.close_reply_preparation(original,body,effect,reply,recover)}}
/// 🧭️ The same public named planner receives either original Vec ranks or immutable page ranks.
pub struct IndexedNamedSource<'a,S:NamedRankedSource,E:NamedEditSource>{source:&'a S,edits:&'a E,index:&'a AdmittedNamedKeyIndex}
impl<'a,S:NamedRankedSource,E:NamedEditSource> IndexedNamedSource<'a,S,E>{pub fn new(source:&'a S,edits:&'a E,index:&'a AdmittedNamedKeyIndex)->Result<Self,ValueError>{index.check_source(source)?;if edits.shape().base_len!=source.len(){return Err(invalid());}Ok(Self{source,edits,index})}}
impl<S:NamedRankedSource,E:NamedEditSource> NamedEditSource for IndexedNamedSource<'_,S,E>{
 fn shape(&self)->IndexedEditShape{self.edits.shape()}
 fn key_demand(&self,lane:NamedEditLane,ordinal:usize)->Result<RetirementDemand,ValueError>{self.index.check_source(self.source)?;if lane==NamedEditLane::Base{self.source.rank_demand(ordinal)}else{self.edits.key_demand(lane,ordinal)}}
 fn observe_key(&self,lane:NamedEditLane,ordinal:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(&str,RetainedCloneProgress)>>{let mut work=0;let result=(||{self.index.check_source(self.source)?;let observation=if lane==NamedEditLane::Base{take_work!(work,self.source.observe_key(ordinal,grant)).map(|(key,p)|(key.get(),p))}else{take_work!(work,self.edits.observe_key(lane,ordinal,grant))};Ok(observation)})();crate::RetainedWorkOutcome{result,work}}
 fn added_index(&self,ordinal:usize)->crate::RetainedWorkOutcome<usize>{self.edits.added_index(ordinal)}
 fn admitted_base(&self)->Option<&AdmittedNamedKeyIndex>{Some(self.index)}
}
