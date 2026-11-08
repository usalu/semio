//! 🎟️ Shared original Work effect, allocation port and typed close for the encoder laws.
use crate::*;
struct Producer{maximum:u64}
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
/// 🔎️ Names the one cause that a native call failure retains.
pub fn cause<T>(failure:semio_framework_value_call::NativeCallFailure<T,ValueError>)->ValueError{failure.operation.or(failure.context).expect("native call failure retains its cause")}
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
/// 🔎️ Runs one operation inside an original effect and reports its exact physical allocations.
pub fn observed<R>(grant:RetainedCloneGrant,maximum:u64,actual:&mut Option<usize>,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->(R,(usize,usize)){with_effect(grant,maximum,actual,|effect|semio_framework_value_testing_allocator::observe_retirement_allocations(||operation(effect)))}
macro_rules! binding{()=>{type Context<'a,'b>=&'a OriginalSourceTurn where 'b:'a;fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError> where 'b:'a{if c.identity()==self.0{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"foreign parent Source turn"))}}fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity> where 'b:'a{Some(c.identity())}}}
/// 🧭️ One actual Source turn that admits every allocation, work and construction authority.
pub struct Source(pub OriginalSourceTurnIdentity);
impl NativeAllocationAuthorityPort for Source{
 binding!();
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:NativeAllocationEvent,reply:NativeAllocationReply<'r>)->Result<NativeAllocationReplyComplete<'r>,ValueError> where 'b:'a{Ok(reply.complete(Ok(())))}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:NativeAllocationEvent,reply:NativeAllocationReply<'r>)->Result<NativeAllocationReplyComplete<'r>,ValueError> where 'b:'a{Ok(reply.pending())}
}
impl OriginalWorkRecipient for Source{
 binding!();
 fn settle_partial<'r,'a,'b>(&mut self,context:&mut Self::Context<'a,'b>,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(OriginalWorkRecipient::validate_context(self,context))}
 fn recover_partial<'r,'a,'b>(&mut self,context:&mut Self::Context<'a,'b>,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(OriginalWorkRecipient::validate_context(self,context))}
 fn admits<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,reply:OriginalWorkQuote<'r>,_:u64,_:u64)->OriginalWorkQuoteComplete<'r> where 'b:'a{reply.complete(Ok(true))}
}
impl OriginalConstructionRecipient for Source{
 binding!();
 fn submitted<'a,'b>(&self,_:&Self::Context<'a,'b>)->RetainedCloneProgress where 'b:'a{Default::default()}
 fn grant<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,reply:OriginalConstructionGrant<'r>)->OriginalConstructionGrantComplete<'r> where 'b:'a{reply.complete(Ok(Default::default()))}
 fn accept<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,reply:OriginalConstructionAccept<'r>)->OriginalConstructionAcceptComplete<'r> where 'b:'a{reply.complete(Ok(()))}
}
/// 📍️ Recorded checkpoint producer that never asks to stop.
pub struct Check(pub OriginalSourceTurnIdentity);
impl OriginalRecordedProducer for Check{
 type Request=OriginalCheckpointRequest;type Reply=bool;binding!();
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,bool>)->Result<OriginalRecordedReplyComplete<'r,bool>,ValueError> where 'b:'a{Ok(reply.complete(Ok(false)))}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,bool>)->Result<OriginalRecordedReplyComplete<'r,bool>,ValueError> where 'b:'a{Ok(reply.pending())}
}
/// 🧭️ Recorded span producer that accepts every span.
pub struct Span(pub OriginalSourceTurnIdentity);
impl OriginalRecordedProducer for Span{
 type Request=OriginalSpanRequest;type Reply=();binding!();
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,()>)->Result<OriginalRecordedReplyComplete<'r,()>,ValueError> where 'b:'a{Ok(reply.complete(Ok(())))}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,()>)->Result<OriginalRecordedReplyComplete<'r,()>,ValueError> where 'b:'a{Ok(reply.pending())}
}
/// 🧭️ Runs one operation against a control bound to every actual recorded Source authority.
pub fn with_recorded_source<R>(maximum:usize,depth:usize,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->R)->R{
 let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let mut allocation=NativeAllocationAuthority::new(Source(turn.identity()));let mut span=OriginalRecordedAuthority::new(Span(turn.identity()));let mut checkpoint=OriginalRecordedAuthority::new(Check(turn.identity()));let(mut a,mut s,mut c)=(&turn,&turn,&turn);
 let result=allocation.with_send_loan(&mut a,|allocation|span.with_send_loan(&mut s,|span|checkpoint.with_send_loan(&mut c,|checkpoint|{let mut control=NativeEncodeControl::new_recorded_source(maximum,depth,turn.identity(),allocation,span,checkpoint).unwrap();operation(&mut control)}).unwrap()).unwrap()).unwrap();
 assert!(allocation.into_port().is_ok());assert!(span.into_port().is_ok());assert!(checkpoint.into_port().is_ok());result
}
/// 🔎️ Names the one cause that an allocation failure retains.
pub fn allocation_cause(mut failure:NativeAllocationFailure)->ValueError{
 let refusals=[failure.forwarded_admission.take(),failure.scoped_admission.take(),failure.forwarded_quote.take(),failure.scoped_quote.take(),failure.forwarded_post.take(),failure.scoped_post.take(),failure.forwarded_cancel.take(),failure.scoped_cancel.take()];
 failure.primary.take().or(failure.admission.take()).or(failure.extent.take()).or_else(||refusals.into_iter().flatten().find_map(|refusal|refusal.into_owned().ok())).expect("allocation failure retains its cause")
}
/// 📦️ Reserves exact octet backing through the quoted original allocation effect.
pub fn reserve_bytes(control:&mut NativeEncodeControl<'_>,bytes:usize)->Result<Vec<u8>,ValueError>{let mut output=Vec::new();let mut actual=None;let mut progress=None;assert!(control.allocate_vec_into(bytes,&mut output,&mut actual,&mut progress).map_err(allocation_cause)?);Ok(output)}
/// 🔤️ Reserves exact UTF-8 backing through the quoted original allocation effect.
pub fn reserve_text(control:&mut NativeEncodeControl<'_>,bytes:usize)->Result<String,ValueError>{let mut output=String::new();let mut actual=None;let mut progress=None;assert!(control.reserve_text_into(bytes,&mut output,&mut actual,&mut progress).map_err(allocation_cause)?);Ok(output)}
