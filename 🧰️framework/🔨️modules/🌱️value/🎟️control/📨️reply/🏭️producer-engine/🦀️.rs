//! 🏭️ Defines the actual borrowed producer and action engine for its receiving contract.
#[macro_export]
macro_rules! original_effect_producer_engine {()=>{
use crate::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,OriginalWorkActionIdentity,RetainedAllocationEffect,ValueError,ValueRefusalKind};
use std::sync::atomic::{AtomicU64,Ordering};
static IDS:AtomicU64=AtomicU64::new(1);
fn next()->u64{IDS.try_update(Ordering::Relaxed,Ordering::Relaxed,|n|n.checked_add(1)).expect("original effect identity exhausted")}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect producer or receipt changed")}
/// 🏭️ Supplies an actual resident producer and separately borrowed original context.
pub trait OriginalEffectProducer{
 type Output;
 type Failure;
 type Context<'turn,'source> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn;
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn;
 fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'reply,Self::Output,Self::Failure>,ValueError> where 'source:'turn;
 fn recover<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'reply,Self::Output,Self::Failure>,ValueError> where 'source:'turn;
}


/// 🫴️ Lends only the genuine sealed Work capability of an actual before-effect controller.
pub trait OriginalEffectBirthContext{
 fn original_work(&mut self)->crate::OriginalWorkBorrow<'_>;
}
impl OriginalEffectBirthContext for crate::OriginalWorkBorrow<'_>{fn original_work(&mut self)->crate::OriginalWorkBorrow<'_>{self.reborrow()}}
/// 👶️ Drives a resident child birth before any normal producer effect.
pub trait OriginalEffectBirthProducer:OriginalEffectProducer<Output=crate::RetainedCloneStep>{
 type Original<'loan>:OriginalEffectBirthContext;
 fn requires_original_birth(&self)->bool;
 fn close_original_birth<'turn,'source,'loan>(&mut self,context:&mut Self::Context<'turn,'source>,original:&mut Self::Original<'loan>,grant:crate::RetainedCloneGrant)->crate::RetainedWorkPoll<crate::RetainedCloneStep,Self::Failure> where 'source:'turn;
}
impl<P:OriginalEffectBirthProducer> OriginalEffectAuthority<P>{
 /// 🧷️ Retains the actual producer and its same validated original Work binding through Pending.
 pub fn drive_original_birth<'turn,'source,'loan>(&mut self,context:&mut P::Context<'turn,'source>,original:&mut P::Original<'loan>,grant:crate::RetainedCloneGrant)->Result<crate::RetainedWorkPoll<crate::RetainedCloneStep,P::Failure>,ValueError> where 'source:'turn{
  if self.active.is_some(){return Err(ValueError::invariant("original birth cannot overlap an active normal producer action"))}
  self.producer.validate_context(context)?;let source=self.producer.source_turn(context);let binding=original.original_work().binding_identity()?;
  if source.is_some()&&source!=binding.source_turn(){return Err(ValueError::invariant("original birth producer and Work Source differ"))}
  if self.birth.is_some_and(|original|original!=binding){return Err(ValueError::invariant("original birth changed its actual Work binding"))}
  if self.birth.is_none()&&!self.producer.requires_original_birth(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original producer has no selected child birth"))}
  self.producer.validate_context(context)?;self.birth=Some(binding);let result=self.producer.close_original_birth(context,original,grant);if matches!(&result,crate::RetainedWorkPoll::Complete(_)){self.birth=None}Ok(result)
 }
}
/// 🚦️ Reports whether the original producer must start, recover, or transfer its recorded reply.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalEffectCallbackMode{Receive,Recover,Recorded}
/// 🪪️ Readonly identity of one actually issued producer invocation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalEffectInvocationIdentity{authority:u64,action:u64,source:Option<OriginalSourceTurnIdentity>,work:OriginalWorkActionIdentity}
/// 🫴️ Borrows an existing live action; it grants no fresh work or physical currency.
pub struct OriginalEffectInvocation<'a>{identity:&'a OriginalEffectInvocationIdentity,mode:OriginalEffectCallbackMode}
impl<'a> OriginalEffectInvocation<'a>{
 pub fn identity(&self)->OriginalEffectInvocationIdentity{*self.identity}
 pub fn mode(&self)->OriginalEffectCallbackMode{self.mode}
 pub fn source_turn(&self)->Option<OriginalSourceTurnIdentity>{self.identity.source}
 pub fn work_action(&self)->OriginalWorkActionIdentity{self.identity.work}
 pub(crate) fn new(identity:&'a OriginalEffectInvocationIdentity,mode:OriginalEffectCallbackMode)->Self{Self{identity,mode}}
 pub(crate) fn reborrow(&self)->OriginalEffectInvocation<'_>{OriginalEffectInvocation{identity:self.identity,mode:self.mode}}
}
/// 🪪️ Retains evidence lent by the original construction loan before its marker is mutably borrowed.
pub struct OriginalConstructionEffectProof{identity:crate::OriginalConstructionActionIdentity,issuer:crate::OriginalConstructionIssuerIdentity,demand:crate::RetirementDemand,grant:crate::RetainedCloneGrant,work:crate::OriginalWorkBindingIdentity,source:Option<OriginalSourceTurnIdentity>}
impl OriginalConstructionEffectProof{
 pub fn identity(&self)->crate::OriginalConstructionActionIdentity{self.identity}
 pub fn demand(&self)->crate::RetirementDemand{self.demand}
 pub fn work_binding(&self)->crate::OriginalWorkBindingIdentity{self.work}
 pub fn source_turn(&self)->Option<OriginalSourceTurnIdentity>{self.source}
 /// 🛂️ Validates the actual effect issued by the same physical action and Work owner.
 pub fn matches_effect(&self,effect:&RetainedAllocationEffect<'_>)->bool{effect.construction_action()==Some(self.identity)&&effect.grant()==self.grant&&effect.work_source_turn()==self.source&&self.work.matches_effect(effect)}
 /// 🛂️ Revalidates the same live original issuer and Source producer.
 pub fn matches_loan(&self,loan:&crate::OriginalConstructionLoan<'_>)->Result<bool,ValueError>{loan.validate_context()?;Ok(loan.identity()==self.issuer&&loan.source_turn()==self.source&&loan.has_live_actions())}
}
impl crate::OriginalConstructionLoan<'_>{
 /// 🪢️ Lends opaque evidence from the actual granted action before its marker is borrowed.
 pub fn effect_proof(&self,action:&crate::OriginalConstructionAction)->Result<OriginalConstructionEffectProof,ValueError>{let witness=self.witness(action)?;if witness.phase()!=crate::OriginalConstructionPhase::Granted{return Err(invalid())}let grant=witness.grant().ok_or_else(invalid)?;Ok(OriginalConstructionEffectProof{identity:witness.identity(),issuer:self.identity(),demand:witness.demand(),grant,work:witness.work_binding(),source:witness.source_turn()})}
}
/// 🧳️ Holds one original producer reply and its same Source and work action.
pub struct OriginalEffectAction<T,E>{invocation:OriginalEffectInvocationIdentity,binding:crate::OriginalWorkBindingIdentity,reply:Option<Result<T,E>>,forward_progress:crate::RetainedCloneProgress,success_copy:Option<(OriginalWorkActionIdentity,Option<crate::OriginalConstructionActionIdentity>)>,settled:bool,finished:bool}
impl<T,E> OriginalEffectAction<T,E>{
 /// 👁️ Observes the original recorded reply while its action retains ownership.
 pub fn recorded_reply(&self)->Option<&Result<T,E>>{self.reply.as_ref()}
 /// 📏️ Observes the same action resident forwarding metadata.
 pub fn forward_progress(&self)->crate::RetainedCloneProgress{self.forward_progress}
 /// 📤️ Transfers the exact producer reply directly into its empty resident slot.
 pub fn transfer_reply(&mut self,output:&mut Option<Result<T,E>>)->Result<bool,ValueError>{if output.is_some(){return Err(invalid())}if !self.settled{return Ok(false)}let Some(reply)=self.reply.take()else{return Err(invalid())};*output=Some(reply);Ok(true)}
}
impl<T,E> Drop for OriginalEffectAction<T,E>{fn drop(&mut self){assert!(std::thread::panicking()||self.finished,"original effect producer action remains live")}}
/// 🧾️ Carries the actual accepted Copy Post and finished Work provenance for one success action.
pub struct OriginalEffectSuccessProof{authority:u64,action:u64,work:OriginalWorkActionIdentity,physical:crate::OriginalConstructionActionIdentity,binding:crate::OriginalWorkBindingIdentity}
/// 🏠️ Owns the actual producer and prevents replacement while an action is live.
pub struct OriginalEffectAuthority<P:OriginalEffectProducer>{producer:std::mem::ManuallyDrop<P>,identity:Option<u64>,active:Option<u64>,birth:Option<crate::OriginalWorkBindingIdentity>}
impl<P:OriginalEffectProducer> OriginalEffectAuthority<P>{
 /// 🌱️ Creates an authority over this actual producer.
 pub fn new(producer:P)->Self{Self{producer:std::mem::ManuallyDrop::new(producer),identity:None,active:None,birth:None}}
 /// 👁️ Observes the actual resident producer without lending its authority.
 pub fn inspect<R>(&self,query:impl FnOnce(&P)->R)->R{query(&self.producer)}
 /// 👁️ Borrows the actual original producer while preserving exclusive action custody.
 pub fn producer(&self)->&P{&self.producer}
 /// ⏳️ Reports whether the original producer action still has custody.
 pub fn has_pending_action(&self)->bool{self.active.is_some()||self.birth.is_some()}
 /// 🫴️ Returns the actual producer only after its original action finishes.
 pub fn into_producer(self)->Result<P,Self>{if self.active.is_some()||self.birth.is_some(){return Err(self)}let mut owner=std::mem::ManuallyDrop::new(self);Ok(unsafe{std::mem::ManuallyDrop::take(&mut owner.producer)})}
 /// 📏️ Quotes the actual complete authority and producer slot transfer.
 pub fn producer_transfer_demand()->Result<crate::RetirementDemand,ValueError>{let copy_bytes=std::mem::size_of::<Option<Self>>().checked_add(std::mem::size_of::<Option<P>>()).ok_or_else(||ValueError::invariant("original producer transfer extent overflow"))?;Ok(crate::RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 /// 📤️ Transfers an idle actual producer directly into its paid caller-owned slot.
 pub fn transfer_producer<'turn,'source>(slot:&mut Option<Self>,context:&P::Context<'turn,'source>,output:&mut Option<P>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<crate::RetainedCloneProgress>,ValueError> where 'source:'turn{use crate::{RetainedCloneProgress,RetainedWorkOutcome};if output.is_some(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}let Some(owner)=slot.as_ref()else{return RetainedWorkOutcome{result:Ok(None),work:0}};if owner.active.is_some()||owner.birth.is_some(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}if let Err(error)=owner.producer.validate_context(context){return RetainedWorkOutcome{result:Err(error),work:0}}let source=owner.producer.source_turn(context);if source.is_some()&&source!=effect.work_source_turn(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}let demand=match Self::producer_transfer_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if remaining==0||!crate::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original producer transfer exceeded its Work grant")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}}let owner=slot.take().expect("original producer slot checked");match owner.into_producer(){Ok(producer)=>{*output=Some(producer);RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}},Err(owner)=>{*slot=Some(owner);RetainedWorkOutcome{result:Err(invalid()),work:1}}}}
 /// 🔎️ Validates the actual producer, Source turn and Work action before entry.
 pub fn callback_mode<'turn,'source>(&self,context:&P::Context<'turn,'source>,effect:&RetainedAllocationEffect<'_>,action:Option<&OriginalEffectAction<P::Output,P::Failure>>)->Result<OriginalEffectCallbackMode,ValueError> where 'source:'turn{self.producer.validate_context(context)?;let source=self.producer.source_turn(context);if source.is_some()&&source!=effect.work_source_turn(){return Err(invalid())}match action{None if self.active.is_none()=>Ok(OriginalEffectCallbackMode::Receive),Some(action) if Some(action.invocation.authority)==self.identity&&self.active==Some(action.invocation.action)&&action.invocation.source==source&&action.invocation.work==effect.work_identity().action_identity()&&!action.finished=>Ok(if action.settled||action.reply.is_some(){OriginalEffectCallbackMode::Recorded}else{OriginalEffectCallbackMode::Recover}),_=>Err(invalid())}}
 /// 🪢️ Starts or recovers only the matching producer and actual work action.
 pub fn drive<'turn,'source>(&mut self,context:&mut P::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,slot:&mut Option<OriginalEffectAction<P::Output,P::Failure>>)->Result<bool,ValueError> where 'source:'turn{
  if self.birth.is_some(){return Err(invalid())}self.producer.validate_context(context)?;let source=self.producer.source_turn(context);if source.is_some()&&source!=effect.work_source_turn(){return Err(invalid())}let work=effect.work_identity().action_identity();let fresh=slot.is_none();if fresh{if self.active.is_some(){return Err(invalid())}let authority=*self.identity.get_or_insert_with(next);let identity=next();self.active=Some(identity);*slot=Some(OriginalEffectAction{invocation:OriginalEffectInvocationIdentity{authority,action:identity,source,work},binding:crate::OriginalWorkBindingIdentity::from_effect(effect),reply:None,forward_progress:Default::default(),success_copy:None,settled:false,finished:false})}let action=slot.as_mut().expect("original effect action installed");if Some(action.invocation.authority)!=self.identity||self.active!=Some(action.invocation.action)||action.invocation.source!=source||action.invocation.work!=work||action.finished{return Err(invalid())}if action.settled{return Ok(true)}if action.reply.is_some(){action.settled=true;return Ok(true)}let mode=if fresh{OriginalEffectCallbackMode::Receive}else{OriginalEffectCallbackMode::Recover};{let mut invoked=effect.with_invocation(&action.invocation,mode,&mut action.forward_progress)?;let reply=OriginalEffectReply::borrow(&mut action.reply)?;let complete=if fresh{self.producer.receive(context,&mut invoked,reply)}else{self.producer.recover(context,&mut invoked,reply)}?;drop(complete);}if action.reply.is_some(){action.settled=true;Ok(true)}else{Ok(false)}
 }

 /// 🧾️ Recovers the same pending producer using new actual payment after its exact original Work proof returned.
 pub fn recover_with_payment<'turn,'source>(&mut self,context:&mut P::Context<'turn,'source>,original:&crate::OriginalWorkActionProof,effect:&mut RetainedAllocationEffect<'_>,slot:&mut Option<OriginalEffectAction<P::Output,P::Failure>>)->Result<bool,ValueError> where 'source:'turn{
  self.producer.validate_context(context)?;let source=self.producer.source_turn(context);let action=slot.as_mut().ok_or_else(invalid)?;if self.birth.is_some()||Some(action.invocation.authority)!=self.identity||self.active!=Some(action.invocation.action)||action.invocation.source!=source||source!=effect.work_source_turn()||action.finished||!original.is_finished()||original.action_identity()!=action.invocation.work||effect.work_identity().action_identity()==action.invocation.work||!action.binding.matches_effect(effect){return Err(invalid())}if action.settled||action.reply.is_some(){action.settled=true;return Ok(true)}{let mut invoked=effect.with_invocation(&action.invocation,OriginalEffectCallbackMode::Recover,&mut action.forward_progress)?;let reply=OriginalEffectReply::borrow(&mut action.reply)?;let complete=self.producer.recover(context,&mut invoked,reply)?;drop(complete)}if action.reply.is_some(){action.settled=true;Ok(true)}else{Ok(false)}
 }
 /// 📏️ Quotes the live Copy success and its resident destination metadata.
 pub fn copy_success_demand()->Result<crate::RetirementDemand,ValueError> where P::Output:Copy{let bytes=std::mem::size_of::<Option<P::Output>>().checked_mul(2).and_then(|n|n.checked_add(std::mem::size_of::<P::Output>())).and_then(|n|n.checked_add(std::mem::size_of::<usize>()+std::mem::size_of::<Option<(OriginalWorkActionIdentity,Option<crate::OriginalConstructionActionIdentity>)>>())).ok_or_else(invalid)?;Ok(crate::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()})}
 /// 📤️ Pays to transfer only a recorded Copy success, retaining any original failure.
 pub fn copy_success_into<'turn,'source>(&self,context:&P::Context<'turn,'source>,action:&mut OriginalEffectAction<P::Output,P::Failure>,output:&mut Option<P::Output>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<crate::RetainedCloneProgress>,ValueError> where 'source:'turn,P::Output:Copy{self.copy_success_into_mode(context,action,output,effect,false)}
 /// 🔁️ Pays to replace the previously observed Copy success in its same resident slot.
 pub fn replace_copy_success_into<'turn,'source>(&self,context:&P::Context<'turn,'source>,action:&mut OriginalEffectAction<P::Output,P::Failure>,output:&mut Option<P::Output>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<crate::RetainedCloneProgress>,ValueError> where 'source:'turn,P::Output:Copy{self.copy_success_into_mode(context,action,output,effect,true)}
 fn copy_success_into_mode<'turn,'source>(&self,context:&P::Context<'turn,'source>,action:&mut OriginalEffectAction<P::Output,P::Failure>,output:&mut Option<P::Output>,effect:&mut RetainedAllocationEffect<'_>,replace:bool)->crate::RetainedWorkOutcome<Option<crate::RetainedCloneProgress>,ValueError> where 'source:'turn,P::Output:Copy{use crate::{RetainedCloneProgress,RetainedWorkOutcome};let rejected=||RetainedWorkOutcome{result:Err(invalid()),work:0};if (!replace&&output.is_some())||Some(action.invocation.authority)!=self.identity||self.active!=Some(action.invocation.action)||!action.settled||action.finished||effect.actual().is_some()||!action.binding.matches_effect(effect){return rejected()}if let Err(error)=self.producer.validate_context(context){return RetainedWorkOutcome{result:Err(error),work:0}}if self.producer.source_turn(context)!=action.invocation.source{return rejected()}let value=match action.reply.as_ref(){Some(Ok(value))=>*value,_=>return rejected()};let demand=match Self::copy_success_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if remaining==0||!crate::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"recorded success transfer exceeded its Work grant")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}}*output=Some(value);drop(action.reply.take());action.success_copy=Some((effect.work_identity().action_identity(),effect.construction_action()));RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}
 /// 🛂️ Authenticates the exact published Copy Post and finished original Work before clearance.
 pub fn success_proof<'turn,'source>(&self,context:&P::Context<'turn,'source>,action:&OriginalEffectAction<P::Output,P::Failure>,loan:&crate::OriginalConstructionLoan<'_>,completion:&crate::OriginalConstructionCompletion,work:&crate::OriginalWorkBorrow<'_>,proof:&crate::OriginalWorkActionProof)->Result<OriginalEffectSuccessProof,ValueError> where 'source:'turn,P::Output:Copy{self.producer.validate_context(context)?;let Some((copy_work,Some(physical)))=action.success_copy else{return Err(invalid())};if Some(action.invocation.authority)!=self.identity||self.active!=Some(action.invocation.action)||!action.settled||action.finished||action.reply.is_some()||self.producer.source_turn(context)!=action.invocation.source||!completion.is_published()||completion.status()!=Some(crate::OriginalConstructionAcceptStatus::Accepted)||completion.action_identity()!=Some(physical)||completion.work_binding()!=Some(action.binding)||!loan.validates_completion(completion)?||!proof.is_finished()||proof.action_identity()!=copy_work||work.binding_identity()?!=action.binding||!work.validates_finished(proof,proof.actual_work())?{return Err(invalid())}Ok(OriginalEffectSuccessProof{authority:action.invocation.authority,action:action.invocation.action,work:copy_work,physical,binding:action.binding})}
 /// 📐️ Quotes only the cleared success action, authority and actual reply slot.
 pub fn finish_success_demand<F>()->Result<crate::RetirementDemand,ValueError> where P::Output:Copy{let copy_bytes=std::mem::size_of::<Option<OriginalEffectAction<P::Output,P::Failure>>>().checked_add(std::mem::size_of::<Option<u64>>()).and_then(|n|n.checked_add(std::mem::size_of::<Option<Result<crate::RetainedCloneStep,F>>>())).ok_or_else(invalid)?;Ok(crate::RetirementDemand{copy_bytes,depth:1,..Default::default()})}
 /// 🧾️ Pays and records clearance of the exact action after its Copy success was transferred.
 pub fn finish_success_into<'reply,'turn,'source,F:From<ValueError>>(&mut self,context:&P::Context<'turn,'source>,slot:&mut Option<OriginalEffectAction<P::Output,P::Failure>>,proof:&OriginalEffectSuccessProof,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,crate::RetainedCloneStep,F>)->Result<OriginalEffectReplyComplete<'reply,crate::RetainedCloneStep,F>,ValueError> where 'source:'turn,P::Output:Copy{let Some(action)=slot.as_ref()else{return Ok(reply.complete(Err(invalid().into())))};if Some(action.invocation.authority)!=self.identity||self.active!=Some(action.invocation.action)||!action.settled||action.finished||action.success_copy!=Some((proof.work,Some(proof.physical)))||proof.authority!=action.invocation.authority||proof.action!=action.invocation.action||proof.binding!=action.binding||action.reply.is_some()||effect.actual().is_some()||!action.binding.matches_effect(effect){return Ok(reply.complete(Err(invalid().into())))}if let Err(error)=self.producer.validate_context(context){return Ok(reply.complete(Err(error.into())))}if self.producer.source_turn(context)!=action.invocation.source{return Ok(reply.complete(Err(invalid().into())))}let demand=Self::finish_success_demand::<F>()?;if !crate::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original success action clear exceeded Work").into())))}let mut action=slot.take().expect("original success action retained");action.finished=true;self.active=None;drop(action);Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
 /// 📏️ Quotes the original action, destination reply and authority metadata.
 pub fn transfer_reply_demand()->Result<crate::RetirementDemand,ValueError>{let bytes=std::mem::size_of::<Option<OriginalEffectAction<P::Output,P::Failure>>>().checked_add(std::mem::size_of::<Option<Result<P::Output,P::Failure>>>()).and_then(|n|n.checked_add(std::mem::size_of::<Option<u64>>())).ok_or_else(invalid)?;Ok(crate::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()})}
 /// 📦️ Pays to move the complete recorded reply and finish its exact resident action.
 pub fn transfer_reply_into<'turn,'source>(&mut self,context:&P::Context<'turn,'source>,slot:&mut Option<OriginalEffectAction<P::Output,P::Failure>>,output:&mut Option<Result<P::Output,P::Failure>>,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<crate::RetainedCloneProgress>,ValueError> where 'source:'turn{use crate::{RetainedCloneProgress,RetainedWorkOutcome};let rejected=||RetainedWorkOutcome{result:Err(invalid()),work:0};if output.is_some(){return rejected()}let Some(action)=slot.as_ref()else{return rejected()};if Some(action.invocation.authority)!=self.identity||self.active!=Some(action.invocation.action)||!action.settled||action.finished||action.reply.is_none()||effect.actual().is_some()||!action.binding.matches_effect(effect){return rejected()}if let Err(error)=self.producer.validate_context(context){return RetainedWorkOutcome{result:Err(error),work:0}}if self.producer.source_turn(context)!=action.invocation.source{return rejected()}let demand=match Self::transfer_reply_demand(){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if remaining==0||!crate::retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original reply transfer exceeded its Work grant")),work:1},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}}let mut action=slot.take().expect("original action checked");*output=action.reply.take();action.finished=true;self.active=None;drop(action);RetainedWorkOutcome{result:Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}
 /// 🏁️ Releases the same action after its actual reply has transferred.
 pub fn finish(&mut self,slot:&mut Option<OriginalEffectAction<P::Output,P::Failure>>)->Result<(),ValueError>{let action=slot.as_mut().ok_or_else(invalid)?;if Some(action.invocation.authority)!=self.identity||self.active!=Some(action.invocation.action)||!action.settled||action.reply.is_some(){return Err(invalid())}action.finished=true;self.active=None;drop(slot.take());Ok(())}
}
impl<P:OriginalEffectProducer> Drop for OriginalEffectAuthority<P>{fn drop(&mut self){assert!(std::thread::panicking(),"original effect producer requires transfer or paid retirement")}}

};}
