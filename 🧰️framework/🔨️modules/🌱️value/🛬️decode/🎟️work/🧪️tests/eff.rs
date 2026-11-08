//! 🎟️ Shared original Work effect, allocation port and typed close for the decoder laws.
use crate::*;
pub struct Producer{pub maximum:u64}
impl OriginalWorkRecipient for Producer{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.maximum)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
/// 🪢️ Runs one operation inside one actual original Work effect over the caller's allocation marker.
pub fn with_effect<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let mut authority=OriginalWorkAuthority::new(Producer{maximum});
 let result=authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;assert_eq!(work.prepare_for_action(maximum,&mut action).unwrap(),Some(true));let mut receipt=None;work.issue_performed_for_action(action,&mut receipt).unwrap();let output={let mut effect=RetainedAllocationEffect::new(grant,maximum,actual,receipt.as_mut().unwrap());operation(&mut effect)};let performed=OriginalWorkSink::observed_work(receipt.as_ref().unwrap());work.claim_performed_for_action(action,&mut receipt).unwrap();if performed==0{work.cancel_for_action(&mut action).unwrap()}else{let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let partial=partial.take().unwrap();assert_eq!(partial.total(),performed);assert!(work.commit_for_action(performed,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap()}output}).unwrap();
 assert!(!authority.has_live_actions());result
}
/// 🏁️ Fixtures drive every retirement turn to completion.
pub fn settled<T,E>(poll:RetainedWorkPoll<T,E>)->RetainedWorkOutcome<T,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("original fixture turn stayed pending")}}
/// 🔎️ Runs one operation inside an original effect and reports its exact physical allocations.
pub fn observed<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,(usize,usize)){with_effect(grant,maximum,actual,|effect|semio_framework_value_testing_allocator::observe_retirement_allocations(||operation(effect)))}
/// 🧾️ Generous per-turn stock for closing fixture-owned results.
pub fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}}
/// ♻️ Closes one typed owner through controlled retirement turns inside original effects.
pub fn close<T:retirement::RetireOwned>(value:T){
 let mut owner=retirement::controlled::ControlledRetirement::new(value).map_err(|(error,_)|error).expect("fixture owner supports typed retirement");
 for _ in 0..4096{if owner.terminal_is_empty(){return}let mut actual=None;let turn=settled(with_effect(stock(),1<<20,&mut actual,|effect|owner.step(effect)));turn.result.expect("fixture close step refused");}
 panic!("fixture close exhausted its turns")
}
/// 🛂️ Adapts one recording closure to the original allocation authority port.
pub struct Port<F>(pub F);
impl<F:FnMut(NativeAllocationEvent)->Result<(),ValueError>> NativeAllocationAuthorityPort for Port<F>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{Ok(reply.complete((self.0)(event).map_err(Into::into)))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
