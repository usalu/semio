//! 🏭️ One actual shared constructor producer owns native input, backing, output and close frontier.
use super::SharedCloseSelection;
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::{OriginalRetirementCloseLoan,OriginalRetirementFailure};
use semio_framework_value_clone_source::RetainedCloneClose;
use semio_framework_value_retirement::retirement::RetireOwned;
use semio_framework_value_shared_issuance::{SealedShared,SealedSharedBirth,SharedIssuer};
use semio_framework_value_allocation_return::original::{SharedOriginalBirth,SharedOriginalStorage};
use std::mem::{ManuallyDrop,size_of};
type Step=RetainedCloneStep;
type Failure=OriginalRetirementFailure;
type Reply<'a>=OriginalEffectReply<'a,Step,Failure>;
type Complete<'a>=OriginalEffectReplyComplete<'a,Step,Failure>;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(super) enum Kind{Sealed,Issued}
#[derive(Clone,Copy,PartialEq,Eq)]
pub(super) enum OperationKind{Capture,Advance,Take,Reclaim,Close(SharedCloseSelection),Birth}
pub(super) enum Operation<'turn,'source,T:RetireOwned+Sync>where 'source:'turn{
 Capture(&'turn mut Option<T>,Kind),Advance,TakeSealed(&'turn mut Option<SealedShared<T>>),TakeIssued(&'turn mut Option<SharedOriginalStorage<T>>),Reclaim(&'turn mut Option<T>),Close(usize,Option<&'turn mut OriginalRetirementCloseLoan<'source>>),Birth
}
pub(super) struct Context<'turn,'source,T:RetireOwned+Sync>where 'source:'turn{pub source:Option<OriginalSourceTurnIdentity>,pub work:OriginalWorkBindingIdentity,pub operation:Operation<'turn,'source,T>}
pub(super) struct Producer<T:RetireOwned+Sync>{pub input:ManuallyDrop<Option<T>>,pub sealed:ManuallyDrop<Option<SealedShared<T>>>,pub issued:ManuallyDrop<Option<SharedOriginalStorage<T>>>,sealed_birth:ManuallyDrop<Option<SealedSharedBirth<T>>>,issued_birth:ManuallyDrop<Option<SharedOriginalBirth<T>>>,pub kind:Option<Kind>,source:Option<OriginalSourceTurnIdentity>,work:Option<OriginalWorkBindingIdentity>,retirement:RetainedCloneClose}
fn invalid()->ValueError{ValueError::invariant("shared constructor changed its original owner or operation")}
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn progress(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes,..Default::default()}}
fn funded(effect:&mut RetainedAllocationEffect<'_>,d:RetirementDemand,work:u64)->Result<bool,ValueError>{if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<work{return Ok(false)}if !effect.performed(work)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared constructor exceeded its original Work"))}Ok(true)}
impl<T:RetireOwned+Sync> Producer<T>{
 pub fn new()->Self{Self{input:ManuallyDrop::new(None),sealed:ManuallyDrop::new(None),issued:ManuallyDrop::new(None),sealed_birth:ManuallyDrop::new(None),issued_birth:ManuallyDrop::new(None),kind:None,source:None,work:None,retirement:Default::default()}}
 pub fn empty(&self)->bool{self.kind.is_none()&&self.input.is_none()&&self.sealed.is_none()&&self.issued.is_none()&&self.sealed_birth.is_none()&&self.issued_birth.is_none()&&self.retirement.is_empty()}
 pub fn output_ready(&self)->bool{(self.sealed.is_some()||self.issued.is_some())&&self.sealed_birth.is_none()&&self.issued_birth.is_none()&&self.retirement.is_empty()}
 pub fn has_birth(&self)->bool{self.sealed_birth.is_some()||self.issued_birth.is_some()}
 fn control_bytes()->usize{size_of::<Option<Kind>>()+size_of::<Option<OriginalSourceTurnIdentity>>()+size_of::<Option<OriginalWorkBindingIdentity>>()}
 pub fn capture_demand()->Result<RetirementDemand,ValueError>{let bytes=size_of::<Option<T>>().checked_mul(2).and_then(|n|n.checked_add(Self::control_bytes())).ok_or_else(invalid)?;Ok(fixed(bytes))}
 pub fn take_demand<U>()->Result<RetirementDemand,ValueError>{let bytes=size_of::<Option<U>>().checked_mul(2).and_then(|n|n.checked_add(Self::control_bytes())).ok_or_else(invalid)?;Ok(fixed(bytes))}
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{match self.kind{
  Some(Kind::Sealed)=>if let Some(birth)=self.sealed_birth.as_ref(){if birth.terminal_is_empty(){Ok(fixed(size_of::<Option<SealedSharedBirth<T>>>()))}else{Ok(SealedSharedBirth::<T>::publication_demand())}}else if self.sealed.is_some(){Ok(Default::default())}else{SealedShared::<T>::prepare_birth_demand()},
  Some(Kind::Issued)=>if let Some(birth)=self.issued_birth.as_ref(){if birth.terminal_is_empty(){Ok(fixed(size_of::<Option<SharedOriginalBirth<T>>>()))}else{Ok(SharedOriginalBirth::<T>::publication_demand())}}else if self.issued.is_some(){Ok(Default::default())}else{SharedOriginalStorage::<T>::prepare_birth_demand()},
  None=>Err(invalid())
 }}
 pub fn work_demand(&self)->Result<u64,ValueError>{match self.kind{
  Some(Kind::Sealed)=>Ok(if let Some(birth)=self.sealed_birth.as_ref(){if birth.terminal_is_empty(){1}else{SealedSharedBirth::<T>::publication_work_demand()}}else if self.sealed.is_some(){0}else{SealedShared::<T>::prepare_birth_work_demand()}),
  Some(Kind::Issued)=>Ok(if let Some(birth)=self.issued_birth.as_ref(){if birth.terminal_is_empty(){1}else{SharedOriginalBirth::<T>::publication_work_demand()}}else if self.issued.is_some(){0}else{SharedOriginalStorage::<T>::prepare_birth_work_demand()}),None=>Err(invalid())
 }}
 fn capture(&mut self,input:&mut Option<T>,kind:Kind,source:Option<OriginalSourceTurnIdentity>,effect:&mut RetainedAllocationEffect<'_>)->Result<Step,ValueError>{if !self.empty()||input.is_none(){return Err(invalid())}let d=Self::capture_demand()?;if !funded(effect,d,1)?{return Ok(Step::Progress(Default::default()))}*self.input=input.take();self.kind=Some(kind);self.source=source;self.work=Some(OriginalWorkBindingIdentity::from_effect(effect));Ok(Step::Complete(progress(d)))}
 fn transfer<U>(source:&mut Option<U>,output:&mut Option<U>,effect:&mut RetainedAllocationEffect<'_>)->Result<Step,ValueError>{if source.is_none()||output.is_some(){return Err(invalid())}let d=Self::take_demand::<U>()?;if !funded(effect,d,1)?{return Ok(Step::Progress(Default::default()))}*output=source.take();Ok(Step::Complete(progress(d)))}
 fn transferred(&mut self,step:Step)->Step{if matches!(step,Step::Complete(_)){self.kind=None}step}
 fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Step,ValueError>{
  if self.output_ready(){return Ok(Step::Complete(Default::default()))}if self.input.is_none()&&!self.has_birth(){return Err(invalid())}
  let step=match self.kind{
   Some(Kind::Sealed)=>if let Some(birth)=self.sealed_birth.as_mut(){if birth.terminal_is_empty(){let d=fixed(size_of::<Option<SealedSharedBirth<T>>>());if !funded(effect,d,1)?{return Ok(Step::Progress(Default::default()))}self.sealed_birth.take();Step::Complete(progress(d))}else{birth.publish(&mut self.input,SharedIssuer::owned(),&mut self.sealed,effect).result?}}else{SealedShared::<T>::prepare_birth(&mut self.sealed_birth,effect).result?},
   Some(Kind::Issued)=>if let Some(birth)=self.issued_birth.as_mut(){if birth.terminal_is_empty(){let d=fixed(size_of::<Option<SharedOriginalBirth<T>>>());if !funded(effect,d,1)?{return Ok(Step::Progress(Default::default()))}self.issued_birth.take();Step::Complete(progress(d))}else{birth.publish(&mut self.input,&mut self.issued,effect).result?}}else{SharedOriginalStorage::<T>::prepare_birth(&mut self.issued_birth,effect).result?},None=>return Err(invalid())
  };Ok(if self.output_ready(){Step::Complete(step.progress())}else{Step::Progress(step.progress())})
 }
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(birth)=self.sealed_birth.as_ref(){return Ok(if birth.terminal_is_empty(){fixed(size_of::<Option<SealedSharedBirth<T>>>())}else{birth.next_close_demand()})}
  if let Some(birth)=self.issued_birth.as_ref(){return Ok(if birth.terminal_is_empty(){fixed(size_of::<Option<SharedOriginalBirth<T>>>())}else{birth.next_close_demand()})}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}
  if self.input.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}
  if self.sealed.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body)}
  if self.issued.is_some(){return self.retirement.next_owner_demands::<SharedOriginalStorage<T>>(true,&None,body)}
  Ok(if self.kind.is_some(){fixed(Self::control_bytes())}else{Default::default()})
 }
 pub fn close_work(&self,body:usize)->Result<u64,ValueError>{
  if let Some(birth)=self.sealed_birth.as_ref(){return Ok(if birth.terminal_is_empty(){1}else{birth.next_close_work_demand()})}
  if let Some(birth)=self.issued_birth.as_ref(){return Ok(if birth.terminal_is_empty(){1}else{birth.close_work_demand()})}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}
  if self.input.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}
  if self.sealed.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}
  if self.issued.is_some(){return self.retirement.next_owner_work::<SharedOriginalStorage<T>>(true,&None,body)}Ok(u64::from(self.kind.is_some()))
 }
 pub fn requires_control(&self)->bool{self.sealed_birth.is_none()&&self.issued_birth.is_none()&&self.retirement.requires_original_control()}
 fn close<'reply>(&mut self,body:usize,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>,recover:bool)->Result<Complete<'reply>,ValueError>{
  if self.requires_original_birth(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared child Birth must precede its general close"))}
  if !self.retirement.is_empty(){return match(original,recover){(Some(original),true)=>self.retirement.recover_step_original_into(original,body,effect,reply),(Some(original),false)=>self.retirement.step_original_into(original,body,effect,reply),(None,true)=>self.retirement.recover_step_into(body,effect,reply),(None,false)=>self.retirement.step_into(body,effect,reply)}}
  if recover{return Ok(reply.pending())}
  let result=(||{
   if let Some(birth)=self.sealed_birth.as_mut(){if birth.terminal_is_empty(){let d=fixed(size_of::<Option<SealedSharedBirth<T>>>());if !funded(effect,d,1)?{return Ok(Step::Progress(Default::default()))}self.sealed_birth.take();return Ok(Step::Progress(progress(d)))}return birth.close_step(body,effect).result}
   if let Some(birth)=self.issued_birth.as_mut(){if birth.terminal_is_empty(){let d=fixed(size_of::<Option<SharedOriginalBirth<T>>>());if !funded(effect,d,1)?{return Ok(Step::Progress(Default::default()))}self.issued_birth.take();return Ok(Step::Progress(progress(d)))}return birth.close_step(body,effect).result}
   let turn=if self.input.is_some(){Some(self.retirement.begin_granted(&mut self.input,effect))}else if self.sealed.is_some(){Some(self.retirement.begin_granted(&mut self.sealed,effect))}else if self.issued.is_some(){Some(self.retirement.begin_granted(&mut self.issued,effect))}else{None};
   if let Some(turn)=turn{return turn.result.map(|step|step.unwrap_or(Step::Progress(Default::default())))}
   if self.kind.is_some(){let d=fixed(Self::control_bytes());if !funded(effect,d,1)?{return Ok(Step::Progress(Default::default()))}self.kind=None;self.source=None;self.work=None;return Ok(Step::Progress(progress(d)))}
   Ok(Step::Complete(Default::default()))
  })();Ok(reply.complete(result.map_err(Into::into)))
 }
}
impl<T:RetireOwned+Sync> OriginalEffectProducer for Producer<T>{
 type Output=Step;type Failure=Failure;type Context<'turn,'source>=Context<'turn,'source,T> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{if self.source.is_some()&&self.source!=c.source||self.work.is_some_and(|work|work!=c.work){return Err(invalid())}Ok(())}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{c.source}
 fn receive<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>where 'source:'turn{
  if !c.work.matches_effect(effect){return Err(invalid())}if let Operation::Close(body,original)=&mut c.operation{return self.close(*body,original.as_deref_mut(),effect,reply,false)}
  let result=match &mut c.operation{
   Operation::Capture(input,kind)=>self.capture(input,*kind,c.source,effect),
   Operation::Advance=>self.advance(effect),
   Operation::TakeSealed(output)=>{if !self.output_ready()||self.kind!=Some(Kind::Sealed){Err(invalid())}else{Self::transfer(&mut self.sealed,output,effect).map(|step|self.transferred(step))}},
   Operation::TakeIssued(output)=>{if !self.output_ready()||self.kind!=Some(Kind::Issued){Err(invalid())}else{Self::transfer(&mut self.issued,output,effect).map(|step|self.transferred(step))}},
   Operation::Reclaim(output)=>{if self.has_birth()||self.output_ready()||!self.retirement.is_empty(){Err(invalid())}else{Self::transfer(&mut self.input,output,effect).map(|step|self.transferred(step))}},
   _=>Err(invalid())
  };Ok(reply.complete(result.map_err(Into::into)))
 }
 fn recover<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Complete<'reply>,ValueError>where 'source:'turn{if !c.work.matches_effect(effect){return Err(invalid())}if let Operation::Close(body,original)=&mut c.operation{return self.close(*body,original.as_deref_mut(),effect,reply,true)}Ok(reply.pending())}
}
impl<T:RetireOwned+Sync> OriginalEffectBirthProducer for Producer<T>{
 type Original<'loan>=OriginalRetirementCloseLoan<'loan>;
 fn requires_original_birth(&self)->bool{self.sealed_birth.is_none()&&self.issued_birth.is_none()&&self.retirement.requires_original_birth()}
 fn close_original_birth<'turn,'source,'loan>(&mut self,_:&mut Self::Context<'turn,'source>,original:&mut Self::Original<'loan>,grant:RetainedCloneGrant)->RetainedWorkPoll<Step,Failure>where 'source:'turn{self.retirement.step_original_birth(original,grant)}
}
impl<T:RetireOwned+Sync> Drop for Producer<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"shared producer retains original custody")}}
