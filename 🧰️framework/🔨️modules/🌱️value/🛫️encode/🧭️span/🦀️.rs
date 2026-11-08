//! 🧭️ Native Source progress delegates to the actual original recorded-action owner.
use super::*;
pub use crate::native_receiving::span::progress::{OriginalSourceCheckpointState,OriginalSourceProgressState};
impl NativeEncodeControl<'_>{
 /// 🧭️ Starts a Source stage without invoking an unrecorded progress callback.
 pub fn begin_source_stage(&mut self,source:crate::OriginalSourceTurnWitness<'_>,total:usize,slot:&Option<OriginalSourceProgressState>)->Result<(),ValueError>{let invalid=crate::native_receiving::invariant;if self.source_turn!=Some(source.identity()){return Err(invalid())}let allocation=self.scoped_allocation.as_deref_mut().ok_or_else(invalid)?;if allocation.source_turn()!=Some(source.identity()){return Err(invalid())}allocation.validate_context()?;crate::native_receiving::span::progress::begin_source_progress(source,self.ledger.as_ref().ok_or_else(invalid)?,&mut self.stage,&mut self.completed,&mut self.total,&mut self.started,self.recorded_span.as_deref_mut().ok_or_else(invalid)?,self.recorded_checkpoint.as_deref_mut().ok_or_else(invalid)?,slot.as_ref(),total)}
 /// 📍️ Runs the original checkpoint alone without synthesizing a Source span.
 pub fn checkpoint_source(&mut self,source:crate::OriginalSourceTurnWitness<'_>,slot:&mut Option<OriginalSourceCheckpointState>)->Result<Option<Result<bool,ValueError>>,ValueError>{let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Source checkpoint changed its original authority");let turn=source.identity();if self.source_turn!=Some(turn){return Err(invalid())}let allocation=self.scoped_allocation.as_deref_mut().ok_or_else(invalid)?;let work=self.action.recipient.as_deref_mut().ok_or_else(invalid)?;let physical=self.construction.as_deref_mut().ok_or_else(invalid)?;crate::native_receiving::span::progress::validate_source_authorities(source,allocation,work,physical)?;let span=self.recorded_span.as_deref_mut().ok_or_else(invalid)?;if span.source_turn()!=Some(turn)||span.has_pending_action(){return Err(invalid())}span.validate_context()?;let ledger=self.ledger.as_mut().ok_or_else(invalid)?;let checkpoint=self.recorded_checkpoint.as_deref_mut().ok_or_else(invalid)?;crate::native_receiving::span::progress::source_checkpoint(source,ledger,self.stage,self.completed,self.total,&mut self.started,checkpoint,slot)}
 /// 📍️ Advances the same recorded Source progress with every original authority validated.
 pub fn advance_source(&mut self,source:crate::OriginalSourceTurnWitness<'_>,units:usize,slot:&mut Option<OriginalSourceProgressState>)->Result<Option<Result<bool,ValueError>>,ValueError>{
  let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Source progress changed its original authority");let turn=source.identity();if self.source_turn!=Some(turn){return Err(invalid())}
  let allocation=self.scoped_allocation.as_deref_mut().ok_or_else(invalid)?;let work=self.action.recipient.as_deref_mut().ok_or_else(invalid)?;let physical=self.construction.as_deref_mut().ok_or_else(invalid)?;crate::native_receiving::span::progress::validate_source_authorities(source,allocation,work,physical)?;
  let ledger=self.ledger.as_mut().ok_or_else(invalid)?;let span=self.recorded_span.as_deref_mut().ok_or_else(invalid)?;let checkpoint=self.recorded_checkpoint.as_deref_mut().ok_or_else(invalid)?;
  crate::native_receiving::span::progress::advance_source_progress(source,ledger,self.stage,&mut self.completed,self.total,&mut self.started,span,checkpoint,units,slot)
 }
}

#[cfg(test)]
mod tests{
 use super::*;
 use crate::native_receiving::span::*;
 use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
 trait Bound{fn source(&self)->crate::OriginalSourceTurnIdentity;}
 impl Bound for Source{fn source(&self)->crate::OriginalSourceTurnIdentity{self.0}}
 impl Bound for Check{fn source(&self)->crate::OriginalSourceTurnIdentity{self.source}}
 impl Bound for Span{fn source(&self)->crate::OriginalSourceTurnIdentity{self.source}}
 macro_rules! binding{()=>{type Context<'a,'b>=&'a crate::OriginalSourceTurn where 'b:'a;fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError> where 'b:'a{if c.identity()==self.source(){Ok(())}else{Err(crate::native_receiving::invariant())}}fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<crate::OriginalSourceTurnIdentity> where 'b:'a{Some(c.identity())}}}
 struct Source(crate::OriginalSourceTurnIdentity);
 impl crate::NativeAllocationAuthorityPort for Source{
  binding!();
  fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:crate::NativeAllocationEvent,reply:crate::NativeAllocationReply<'r>)->Result<crate::NativeAllocationReplyComplete<'r>,ValueError> where 'b:'a{Ok(reply.complete(Ok(())))}
  fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:crate::NativeAllocationEvent,reply:crate::NativeAllocationReply<'r>)->Result<crate::NativeAllocationReplyComplete<'r>,ValueError> where 'b:'a{Ok(reply.pending())}
 }
 impl crate::OriginalWorkRecipient for Source{
  binding!();
  fn settle_partial<'r,'a,'b>(&mut self,context:&mut Self::Context<'a,'b>,check:crate::OriginalWorkPartialCheck<'r>)->crate::OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(crate::OriginalWorkRecipient::validate_context(self,context))}
  fn recover_partial<'r,'a,'b>(&mut self,context:&mut Self::Context<'a,'b>,check:crate::OriginalWorkPartialCheck<'r>)->crate::OriginalWorkPartialCheckComplete<'r> where 'b:'a{check.complete(crate::OriginalWorkRecipient::validate_context(self,context))}
  fn admits<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,reply:crate::OriginalWorkQuote<'r>,_:u64,_:u64)->crate::OriginalWorkQuoteComplete<'r> where 'b:'a{reply.complete(Ok(true))}
 }
 impl crate::OriginalConstructionRecipient for Source{
  binding!();
  fn submitted<'a,'b>(&self,_:&Self::Context<'a,'b>)->crate::RetainedCloneProgress where 'b:'a{Default::default()}
  fn grant<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,reply:crate::OriginalConstructionGrant<'r>)->crate::OriginalConstructionGrantComplete<'r> where 'b:'a{reply.complete(Ok(Default::default()))}
  fn accept<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,reply:crate::OriginalConstructionAccept<'r>)->crate::OriginalConstructionAcceptComplete<'r> where 'b:'a{reply.complete(Ok(()))}
 }
 struct Check{source:crate::OriginalSourceTurnIdentity,calls:Arc<AtomicUsize>}
 impl OriginalRecordedProducer for Check{
  type Request=OriginalCheckpointRequest;type Reply=bool;binding!();
  fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,bool>)->Result<OriginalRecordedReplyComplete<'r,bool>,ValueError> where 'b:'a{self.calls.fetch_add(1,Ordering::Relaxed);let _done=reply.complete(Ok(false));panic!("actual checkpoint decision interrupted")}
  fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,bool>)->Result<OriginalRecordedReplyComplete<'r,bool>,ValueError> where 'b:'a{Ok(reply.pending())}
 }
 struct Span{source:crate::OriginalSourceTurnIdentity,calls:Arc<AtomicUsize>}
 impl OriginalRecordedProducer for Span{
  type Request=OriginalSpanRequest;type Reply=();binding!();
  fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,()>)->Result<OriginalRecordedReplyComplete<'r,()>,ValueError> where 'b:'a{self.calls.fetch_add(1,Ordering::Relaxed);Ok(reply.complete(Ok(())))}
  fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,()>)->Result<OriginalRecordedReplyComplete<'r,()>,ValueError> where 'b:'a{Ok(reply.pending())}
 }
 #[test]
 fn source_progress_recovers_actual_cancel_before_span(){let issuer=crate::OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let calls=Arc::new(AtomicUsize::new(0));let spans=Arc::new(AtomicUsize::new(0));let mut allocation=crate::NativeAllocationAuthority::new(Source(turn.identity()));let mut work=crate::OriginalWorkAuthority::new(Source(turn.identity()));let mut physical=crate::OriginalConstructionAuthority::new(Source(turn.identity()));let mut checkpoint=OriginalRecordedAuthority::new(Check{source:turn.identity(),calls:calls.clone()});let mut span=OriginalRecordedAuthority::new(Span{source:turn.identity(),calls:spans.clone()});let(mut a,mut w,mut p,mut c,mut s)=(&turn,&turn,&turn,&turn,&turn);allocation.with_send_loan(&mut a,|allocation|work.with_loan(&mut w,|work|physical.with_loan(&mut p,|physical|span.with_send_loan(&mut s,|span|checkpoint.with_send_loan(&mut c,|checkpoint|{let mut control=NativeEncodeControl::new_recorded_source(8,2,turn.witness().identity(),allocation,span,checkpoint).unwrap();let mut progress=None;control.with_work_recipient(work,|control|control.with_construction_recipient(physical,|control|{control.begin_source_stage(turn.witness(),1,&progress)?;let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.advance_source(turn.witness(),1,&mut progress)));assert!(interrupted.is_err());assert!(progress.is_some());assert_eq!(control.advance_source(turn.witness(),1,&mut progress)?.unwrap().unwrap(),false);assert!(progress.is_none());Ok::<(),ValueError>(())})).unwrap();}).unwrap()).unwrap()).unwrap()).unwrap()).unwrap();assert_eq!((calls.load(Ordering::Relaxed),spans.load(Ordering::Relaxed)),(1,0));assert!(allocation.into_port().is_ok());assert!(span.into_port().is_ok());assert!(checkpoint.into_port().is_ok());}
}
