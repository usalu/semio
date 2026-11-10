//! 🧪️ Shared fixtures of the value-edit test targets: a fixture original effect and a reply-turn driver over the public API.
#![allow(dead_code)]
pub mod effect{
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedCloneGrant,RetainedWorkOutcome,ValueError};
use semio_framework_value_retirement::RetainedWorkPoll;
use semio_framework_value_retained_clone::retained_clone::{OriginalSourceTurnIdentity,OriginalWork,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,OriginalWorkSink};
struct Producer{maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original effect over the caller's allocation marker.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().unwrap();assert_eq!(partial.total(),performed);assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}output}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ Fixtures drive every retirement turn to completion, so a suspended poll is a fixture fault.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// 🪢️ Runs one operation inside one actual original effect over a fresh allocation marker.
pub fn run<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{with_effect(grant,WORK,&mut None,operation)}
/// 🧾️ A turn settles to its outcome whether the owner returned it directly or as a poll that must not stay pending.
pub trait Settle{type Settled;fn settle(self)->Self::Settled;}
impl<T,E> Settle for RetainedWorkPoll<T,E>{type Settled=RetainedWorkOutcome<T,E>;fn settle(self)->Self::Settled{settled(self)}}
impl<T,E> Settle for RetainedWorkOutcome<T,E>{type Settled=Self;fn settle(self)->Self::Settled{self}}
/// 🏁️ Runs one retirement turn to completion inside one actual original effect.
pub fn settle<R:Settle>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R::Settled{run(grant,operation).settle()}
/// 📏️ Observes only the operation's heap, never the fixture authority around it.
pub fn observed<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,(usize,usize)){with_effect(grant,WORK,&mut None,|effect|semio_framework_value_testing_allocator::observe_retirement_allocations(||operation(effect)))}
/// ⏱️ Fixture work ceiling every observed turn may record.
pub const WORK:u64=1<<20;
/// 🎯️ Grants one item and exactly the selected action's quote under the copy body that quote was priced with.
pub fn quoted(body:usize,quote:impl Fn(usize)->semio_framework_value_retirement::RetirementDemand)->RetainedCloneGrant{let mut copy=body;for _ in 0..8{let demand=quote(copy);let want=body.max(demand.copy_bytes);if want==copy{return RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}copy=want}panic!("retirement quote did not settle on one copy body")}

}
pub mod driver{
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneStep,ValueError,OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value_edit::{OriginalRetirementFailure,indexed_edit::CloseReply};
pub type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
pub type Done<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
use semio_framework_value_control_contract::{OriginalEffectAuthority,OriginalEffectProducer,OriginalSourceTurnIdentity};

pub(crate) type Call<'a,X>=&'a mut dyn for<'r,'e> FnMut(&mut X,bool,&mut RetainedAllocationEffect<'e>,Reply<'r>)->Result<Done<'r>,ValueError>;
/// 🧳️ Holds the owner under test as the resident producer of one original effect authority.
pub(crate) struct Probe<X>(pub X);
impl<X:Send+'static> OriginalEffectProducer for Probe<X>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'turn,'source>=Call<'turn,X> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&Call<'turn,X>)->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&Call<'turn,X>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,call:&mut Call<'turn,X>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Done<'reply>,ValueError>where 'source:'turn{call(&mut self.0,false,effect,reply)}
 fn recover<'reply,'turn,'source>(&mut self,call:&mut Call<'turn,X>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'reply>)->Result<Done<'reply>,ValueError>where 'source:'turn{call(&mut self.0,true,effect,reply)}
}
/// 🏁️ Runs one fresh turn to its recorded reply and observes only the operation's heap.
pub(crate) fn turn<X:Send+'static>(owner:X,grant:RetainedCloneGrant,mut call:impl for<'r,'e> FnMut(&mut X,bool,&mut RetainedAllocationEffect<'e>,Reply<'r>)->Result<Done<'r>,ValueError>)->(X,Result<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize),u128){
 let mut authority=OriginalEffectAuthority::new(Probe(owner));let mut action=None;let mut context:Call<'_,X>=&mut call;
 let((ready,work),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||super::effect::run(grant,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut context,effect,&mut action);(ready,effect.performed_work()-before)}));
 assert!(ready.unwrap(),"fixture turn stayed pending");let mut answer=None;assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(&mut action).unwrap();
 let Probe(owner)=authority.into_producer().ok().expect("fixture authority retained a pending action");(owner,answer.unwrap(),heap,work)
}
/// 🏁️ One fresh close turn of any reply-shaped close owner; the body is the issued copy grant.
pub(crate) fn close_turn<X:CloseReply+Send+'static>(owner:&mut X,spare:X,issued:RetainedCloneGrant)->(Result<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize),u128){turn_mut(owner,spare,issued,|x,recover,effect,reply|{let body=effect.grant().maximum_copy_bytes;x.close_reply(None,body,effect,reply,recover)})}
/// 🪢️ Runs one fresh turn over an owner held by the caller, parking a spare never-used owner meanwhile and forgetting it afterwards, since an unclosed spare would refuse to drop.
pub(crate) fn turn_mut<X:Send+'static>(owner:&mut X,spare:X,grant:RetainedCloneGrant,call:impl for<'r,'e> FnMut(&mut X,bool,&mut RetainedAllocationEffect<'e>,Reply<'r>)->Result<Done<'r>,ValueError>)->(Result<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize),u128){
 let taken=std::mem::replace(owner,spare);let(back,answer,heap,work)=turn(taken,grant,call);std::mem::forget(std::mem::replace(owner,back));(answer,heap,work)
}

/// 📌️ Keeps an owner resident inside one original effect authority and one effect bank across turns, so address-bound cursors never move and every turn shares one original Work binding.
pub struct Rig<X:Send+'static>{accumulated:(usize,usize),action:Option<semio_framework_value_control_contract::OriginalEffectAction<RetainedCloneStep,OriginalRetirementFailure>>,authority:Option<OriginalEffectAuthority<Probe<X>>>,bank:semio_framework_value_testing_effect_bank::EffectBank}
impl<X:Send+'static> Rig<X>{
 pub fn new(owner:X)->Self{let stock=RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:256};Self{accumulated:(0,0),action:None,authority:Some(OriginalEffectAuthority::new(Probe(owner))),bank:semio_framework_value_testing_effect_bank::EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::new(stock,1<<30))}}
 /// 🏁️ Runs one fresh turn on the resident owner and posts its receipt to the bank.
 pub fn turn(&mut self,grant:RetainedCloneGrant,mut call:impl for<'r,'e> FnMut(&mut X,bool,&mut RetainedAllocationEffect<'e>,Reply<'r>)->Result<Done<'r>,ValueError>)->(Result<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize),u128){
  let authority=self.authority.as_mut().unwrap();let bank=&mut self.bank;let mut action=None;let mut context:Call<'_,X>=&mut call;
  let((poll,_),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,1<<20,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut context,effect,&mut action);let work=effect.performed_work()-before;match ready{Ok(true)=>semio_framework_value_retirement::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work}),Ok(false)=>semio_framework_value_retirement::RetainedWorkPoll::Pending{work},Err(error)=>semio_framework_value_retirement::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:Err(error),work})}}));
  let semio_framework_value_retirement::RetainedWorkPoll::Complete(turn)=poll else{panic!("fixture turn stayed pending")};turn.result.unwrap();
  let mut answer=None;assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(&mut action).unwrap();let answer=answer.unwrap();
  let progress=match &answer{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,grant,heap);(answer,heap,turn.work)
 }
 /// 🔁️ Runs one turn of a possibly pending invocation: fresh when none is open, otherwise its recovery; yields the reply only once it is recorded.
 pub fn step(&mut self,grant:RetainedCloneGrant,mut call:impl for<'r,'e> FnMut(&mut X,bool,&mut RetainedAllocationEffect<'e>,Reply<'r>)->Result<Done<'r>,ValueError>)->Option<(Result<RetainedCloneStep,OriginalRetirementFailure>,u128)>{
  let authority=self.authority.as_mut().unwrap();let bank=&mut self.bank;let action=&mut self.action;let mut context:Call<'_,X>=&mut call;
  let((poll,_),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,1<<20,|effect|{let before=effect.performed_work();let ready=authority.drive(&mut context,effect,action);let work=effect.performed_work()-before;match ready{Ok(true)=>semio_framework_value_retirement::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work}),Ok(false)=>semio_framework_value_retirement::RetainedWorkPoll::Pending{work},Err(error)=>semio_framework_value_retirement::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:Err(error),work})}}));
  self.accumulated.0+=heap.0;self.accumulated.1+=heap.1;
  match poll{
   semio_framework_value_retirement::RetainedWorkPoll::Pending{..}=>None,
   semio_framework_value_retirement::RetainedWorkPoll::Complete(turn)=>{turn.result.unwrap();let mut answer=None;assert!(action.as_mut().unwrap().transfer_reply(&mut answer).unwrap());authority.finish(action).unwrap();let answer=answer.unwrap();let progress=match &answer{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert_ne!(bank.take_admitted(),Some(false));bank.post(progress,grant,std::mem::take(&mut self.accumulated));Some((answer,turn.work))}
  }
 }
 /// 🚫️ Runs one fresh turn that must be refused by its producer and returns that refusal.
 pub fn refused(mut self,grant:RetainedCloneGrant,mut call:impl for<'r,'e> FnMut(&mut X,bool,&mut RetainedAllocationEffect<'e>,Reply<'r>)->Result<Done<'r>,ValueError>)->ValueError{
  let authority=self.authority.as_mut().unwrap();let bank=&mut self.bank;let mut action=None;let mut context:Call<'_,X>=&mut call;
  let((poll,_),_)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(grant,1<<20,|effect|{let ready=authority.drive(&mut context,effect,&mut action);semio_framework_value_retirement::RetainedWorkPoll::Complete(semio_framework_value_control_contract::RetainedWorkOutcome{result:ready,work:0})}));
  let semio_framework_value_retirement::RetainedWorkPoll::Complete(turn)=poll else{panic!("fixture refusal stayed pending")};let error=turn.result.expect_err("producer must refuse");bank.take_admitted();std::mem::forget(action);std::mem::forget(self);error
 }
 /// 🎛️ Applies one control change such as beginning close without moving the owner.
 pub fn control(&mut self,change:impl FnOnce(&mut X)){let mut change=Some(change);self.turn(Default::default(),|owner,_,_,reply|{(change.take().unwrap())(owner);Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}).0.unwrap();}
}
impl<X:Send+'static> std::ops::Deref for Rig<X>{type Target=X;fn deref(&self)->&X{&self.authority.as_ref().unwrap().producer().0}}
impl<X:Send+'static> Drop for Rig<X>{fn drop(&mut self){if let Some(authority)=self.authority.take(){match authority.into_producer(){Ok(owner)=>drop(owner),Err(authority)=>{std::mem::forget(authority);}}}}}
}
