//! 🧪️ Settles reply-shaped cursor turns and admits fixture sources under the caller's finite stock.
use semio_framework_value_control_contract::{OriginalEffectAction,OriginalEffectAuthority,OriginalEffectProducer,OriginalSourceTurnIdentity,OriginalEffectReply,OriginalEffectReplyComplete,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_retained_clone::retained_clone::{RetainedCloneRef,RetainedCloneSource};
use semio_framework_value_retirement::retirement::RetireOwned;
use super::{PersistentList,PersistentListRead};
struct Turn<F,E>{op:Option<F>,work:u128,failure:std::marker::PhantomData<fn()->E>}
fn refused()->ValueError{ValueError::invariant("fixture turn has no original callback to receive or recover")}
impl<E,F:for<'r,'e,'x> FnOnce(&'x mut RetainedAllocationEffect<'e>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>> OriginalEffectProducer for Turn<F,E>{
 type Output=RetainedCloneStep;type Failure=E;type Context<'a,'b>=() where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&()) ->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,_:&mut (),effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>where 'b:'a{let before=effect.performed_work();let answer=(self.op.take().ok_or_else(refused)?)(effect,reply);self.work=effect.performed_work().saturating_sub(before);answer}
 fn recover<'r,'a,'b>(&mut self,_:&mut (),_:&mut RetainedAllocationEffect<'_>,_:OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>where 'b:'a{Err(refused())}
}
/// 📬️ Runs one reply-shaped turn inside its genuine original producer invocation and records its performed work.
pub fn poll<E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,op:impl for<'r,'e,'x> FnOnce(&'x mut RetainedAllocationEffect<'e>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkPoll<RetainedCloneStep,E>{
 let mut authority=OriginalEffectAuthority::new(Turn{op:Some(op),work:0,failure:std::marker::PhantomData});let mut action=None;
 let driven=authority.drive(&mut (),effect,&mut action);
 let settled=driven.and_then(|ready|{if !ready{return Ok(None)}let mut answer=None;action.as_mut().ok_or_else(refused)?.transfer_reply(&mut answer)?;authority.finish(&mut action)?;Ok(answer)});
 let work=match authority.into_producer(){Ok(producer)=>producer.work,Err(pending)=>{std::mem::forget(pending);0}};
 if action.is_some(){std::mem::forget(action)}
 match settled{Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),Ok(Some(result))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),Ok(None)=>RetainedWorkPoll::Pending{work}}
}
/// 🧵️ Settles one reply-shaped turn under an explicit grant.
pub fn run<E:From<ValueError>>(grant:RetainedCloneGrant,op:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkOutcome<RetainedCloneStep,E>{crate::effect::settle(grant,|effect|poll(effect,op))}
/// 📤️ Settles one publication turn and pairs the published output with its physical progress.
pub fn take<T,E:From<ValueError>>(grant:RetainedCloneGrant,op:impl for<'r> FnOnce(&mut Option<T>,&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>,E>{
 let mut output=None;let turn=crate::effect::settle(grant,|effect|poll(effect,|effect,reply|op(&mut output,effect,reply)));
 RetainedWorkOutcome{result:turn.result.map(|step|output.take().map(|value|(value,step.progress()))),work:turn.work}
}
/// 🚪️ Settles one paid close-start action.
pub fn begin(grant:RetainedCloneGrant,op:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->RetainedWorkOutcome{crate::effect::settle(grant,|effect|RetainedWorkPoll::Complete(op(effect)))}
/// 🔎️ Settles one read turn and pairs the step with the row it published.
pub fn read<'a,T:RetireOwned+Send+Sync+'static>(cursor:&mut PersistentListRead<T>,source:RetainedCloneRef<'a,PersistentList<T>>,grant:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep>,Option<(usize,RetainedCloneRef<'a,T>)>){
 let mut row=None;let turn=crate::effect::settle(grant,|effect|poll(effect,|effect,reply|cursor.advance_into(source,&mut row,effect,reply)));(turn,row)
}
/// 🎟️ Admits one fixture source from a fresh bank seated with the caller's remaining stock and Work.
pub fn admit_source<T:RetireOwned+Sync+'static>(value:T,bank:RetainedCloneGrant,fuel:u64)->(RetainedCloneSource<T>,RetainedCloneProgress,u128){
 let mut effects=semio_framework_value_testing_effect_bank::EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::new(bank,fuel));
 let turn=semio_framework_value_clone_source_testing::admit_owned(&mut effects,value,(),RetainedCloneGrant{maximum_items:1,..bank});let work=turn.work;
 match turn.result{Ok((source,progress))=>(source,progress,work),Err(recovery)=>panic!("{}",recovery.error())}
}
type Resume<'a,E>=dyn for<'r,'e,'x> FnMut(&'x mut RetainedAllocationEffect<'e>,OriginalEffectReply<'r,RetainedCloneStep,E>,bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>+'a;
struct Turns<E>{source:OriginalSourceTurnIdentity,failure:std::marker::PhantomData<fn()->E>}
impl<E:'static> OriginalEffectProducer for Turns<E>{
 type Output=RetainedCloneStep;type Failure=E;type Context<'a,'b>=&'a mut Resume<'a,E> where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&&'a mut Resume<'a,E>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&&'a mut Resume<'a,E>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(self.source)}
 fn receive<'r,'a,'b>(&mut self,context:&mut &'a mut Resume<'a,E>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>where 'b:'a{context(effect,reply,false)}
 fn recover<'r,'a,'b>(&mut self,context:&mut &'a mut Resume<'a,E>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>where 'b:'a{context(effect,reply,true)}
}
/// 🔁️ Keeps one original producer authority and its pending action alive across the turns of a shared finite Source bank.
pub struct Driver<E:'static>{authority:Option<OriginalEffectAuthority<Turns<E>>>,action:Option<OriginalEffectAction<RetainedCloneStep,E>>}
impl<E:From<ValueError>+'static> Default for Driver<E>{fn default()->Self{Self{authority:None,action:None}}}
impl<E:From<ValueError>+'static> Driver<E>{
 /// ⏳️ Reports an action that began on an earlier turn and awaits its recovery turn.
 pub fn pending(&self)->bool{self.action.is_some()}
 /// 🧵️ Runs one turn; the callback receives true when it must recover the pending action instead of starting one.
 pub fn turn(&mut self,bank:&mut semio_framework_value_testing_effect_bank::EffectBank,grant:RetainedCloneGrant,maximum:u64,mut op:impl for<'r,'e,'x> FnMut(&'x mut RetainedAllocationEffect<'e>,OriginalEffectReply<'r,RetainedCloneStep,E>,bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkOutcome<RetainedCloneStep,E>{
  let source=bank.source_witness().identity();
  let authority=self.authority.get_or_insert_with(||OriginalEffectAuthority::new(Turns{source,failure:std::marker::PhantomData}));let action=&mut self.action;
  let(settled,_)=bank.run(grant,maximum,|effect|{
   let before=effect.performed_work();let mut context:&mut Resume<'_,E>=&mut op;
   let driven=authority.drive(&mut context,effect,action);let work=effect.performed_work().saturating_sub(before);
   let settled=driven.and_then(|ready|{if !ready{return Ok(None)}let mut answer=None;action.as_mut().ok_or_else(refused)?.transfer_reply(&mut answer)?;authority.finish(action)?;Ok(answer)});
   match settled{Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),Ok(Some(result))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),Ok(None)=>RetainedWorkPoll::Pending{work}}
  });
  let _=bank.take_admitted();
  match settled{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}
 }
}
impl<E:'static> Drop for Driver<E>{fn drop(&mut self){if let Some(authority)=self.authority.take(){match authority.into_producer(){Ok(producer)=>drop(producer),Err(pending)=>std::mem::forget(pending)}}if self.action.is_some(){std::mem::forget(self.action.take())}}}
