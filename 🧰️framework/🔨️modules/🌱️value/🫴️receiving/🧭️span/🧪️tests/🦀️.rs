use super::*;
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
struct Producer{turn:crate::OriginalSourceTurnIdentity,calls:Arc<AtomicUsize>,message:Option<crate::ValueError>}
impl OriginalRecordedProducer for Producer{type Request=OriginalSpanRequest;type Reply=();
 type Context<'turn,'source>=&'turn crate::OriginalSourceTurn where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),crate::ValueError> where 'source:'turn{if c.identity()==self.turn{Ok(())}else{Err(invalid())}}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{Some(c.identity())}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,request:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{assert_eq!(request,OriginalSpanRequest{units:3,owned_bytes:8});self.calls.fetch_add(1,Ordering::Relaxed);let _done=reply.complete(Err(self.message.take().unwrap()));panic!("producer unwound after recording original refusal")}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{Ok(reply.pending())}
}
#[test]
fn span_retains_recorded_original_refusal_and_rejects_foreign_turn(){let issuer=crate::OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let other=issuer.issue().unwrap();let calls=Arc::new(AtomicUsize::new(0));let error=crate::ValueError::new(crate::ValueRefusalKind::Canceled,"original span refusal".to_owned());let pointer=error.message.as_ptr();let mut authority=OriginalRecordedAuthority::new(Producer{turn:turn.identity(),calls:calls.clone(),message:Some(error)});let mut action=None;let mut context=&turn;let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||authority.with_loan(&mut context,|loan|loan.begin(OriginalSpanRequest{units:3,owned_bytes:8},&mut action)).unwrap().unwrap()));assert!(interrupted.is_err());assert_eq!(calls.load(Ordering::Relaxed),1);let mut foreign=&other;assert!(authority.with_loan(&mut foreign,|_|()).is_err());authority.with_loan(&mut context,|loan|{assert!(loan.recover(action.as_mut().unwrap()).unwrap());let error=action.as_mut().unwrap().take_reply().unwrap().unwrap_err();assert_eq!(error.message.as_ptr(),pointer);loan.finish(&mut action).unwrap();drop(error)}).unwrap();assert!(authority.into_port().is_ok());assert_eq!(calls.load(Ordering::Relaxed),1)}
#[test]
fn span_retries_only_when_original_producer_was_never_entered(){
 struct Context<'a>{turn:&'a crate::OriginalSourceTurn,checks:AtomicUsize,panic_at:AtomicUsize}
 struct Before{source:crate::OriginalSourceTurnIdentity,calls:Arc<AtomicUsize>}
 impl OriginalRecordedProducer for Before{type Request=OriginalSpanRequest;type Reply=();
  type Context<'turn,'source>=Context<'turn> where 'source:'turn;
  fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),crate::ValueError> where 'source:'turn{if c.turn.identity()!=self.source{return Err(invalid())}let count=c.checks.fetch_add(1,Ordering::Relaxed)+1;if count==c.panic_at.load(Ordering::Relaxed){panic!("validation interrupted before producer entry")}Ok(())}
  fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{Some(c.turn.identity())}
  fn receive<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,_:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{let _=c;self.calls.fetch_add(1,Ordering::Relaxed);Ok(reply.complete(Ok(())))}
  fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{Ok(reply.pending())}
 }
 let issuer=crate::OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let calls=Arc::new(AtomicUsize::new(0));let mut producer=OriginalRecordedAuthority::new(Before{source:turn.identity(),calls:calls.clone()});let mut context=Context{turn:&turn,checks:AtomicUsize::new(0),panic_at:AtomicUsize::new(3)};let mut action=None;let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||producer.with_send_loan(&mut context,|loan|loan.begin(OriginalSpanRequest{units:3,owned_bytes:8},&mut action)).unwrap().unwrap()));assert!(interrupted.is_err());assert_eq!(calls.load(Ordering::Relaxed),0);assert!(action.is_some());context.panic_at.store(0,Ordering::Relaxed);producer.with_send_loan(&mut context,|loan|{assert!(loan.recover(action.as_mut().unwrap()).unwrap());assert!(action.as_mut().unwrap().take_reply().unwrap().is_ok());loan.finish(&mut action).unwrap()}).unwrap();assert_eq!(calls.load(Ordering::Relaxed),1);assert!(producer.into_port().is_ok());
}
#[test]
fn span_pending_keeps_one_original_producer_and_blocks_transfer(){
 struct Pending{receive:Arc<AtomicUsize>,recover:Arc<AtomicUsize>,error:Option<crate::ValueError>}
 impl OriginalRecordedProducer for Pending{type Request=OriginalSpanRequest;type Reply=();
  type Context<'turn,'source>=() where 'source:'turn;
  fn validate_context<'turn,'source>(&self,_:&())->Result<(),crate::ValueError> where 'source:'turn{Ok(())}
  fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{None}
  fn receive<'reply,'turn,'source>(&mut self,_:&mut (),_:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{self.receive.fetch_add(1,Ordering::Relaxed);Ok(reply.pending())}
  fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{self.recover.fetch_add(1,Ordering::Relaxed);Ok(reply.complete(Err(self.error.take().unwrap())))}
 }
 let receive=Arc::new(AtomicUsize::new(0));let recover=Arc::new(AtomicUsize::new(0));let error=crate::ValueError::new(crate::ValueRefusalKind::Canceled,"pending original refusal".to_owned());let pointer=error.message.as_ptr();let mut authority=OriginalRecordedAuthority::new(Pending{receive:receive.clone(),recover:recover.clone(),error:Some(error)});let mut action=None;authority.with_send_loan(&mut (),|loan|loan.begin(OriginalSpanRequest{units:3,owned_bytes:8},&mut action).unwrap()).unwrap();assert!(authority.has_pending_action());assert!(!crate::retirement::RetireOwned::retirement_ready(action.as_ref().unwrap()));let mut authority=match authority.into_port(){Err(authority)=>authority,Ok(_)=>panic!("pending action transferred its producer")};authority.with_send_loan(&mut (),|loan|{assert!(loan.recover(action.as_mut().unwrap()).unwrap());let refusal=action.as_mut().unwrap().take_reply().unwrap().unwrap_err();assert_eq!(refusal.message.as_ptr(),pointer);loan.finish(&mut action).unwrap();drop(refusal)}).unwrap();assert_eq!((receive.load(Ordering::Relaxed),recover.load(Ordering::Relaxed)),(1,1));assert!(authority.into_port().is_ok());
}
#[test]
fn span_revalidates_borrowed_source_before_action_birth(){
 use std::sync::atomic::AtomicBool;
 struct Source{turn:crate::OriginalSourceTurnIdentity,calls:Arc<AtomicUsize>}
 impl OriginalRecordedProducer for Source{type Request=OriginalSpanRequest;type Reply=();
  type Context<'turn,'source>=(&'turn crate::OriginalSourceTurn,&'turn AtomicBool) where 'source:'turn;
  fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),crate::ValueError> where 'source:'turn{if c.0.identity()!=self.turn||!c.1.load(Ordering::Relaxed){Err(invalid())}else{Ok(())}}
  fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{Some(c.0.identity())}
  fn receive<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{self.calls.fetch_add(1,Ordering::Relaxed);Ok(reply.complete(Ok(())))}
  fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:OriginalSpanRequest,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{Ok(reply.pending())}
 }
 let issuer=crate::OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let valid=AtomicBool::new(true);let calls=Arc::new(AtomicUsize::new(0));let mut authority=OriginalRecordedAuthority::new(Source{turn:turn.identity(),calls:calls.clone()});let mut context=(&turn,&valid);let mut action=None;authority.with_send_loan(&mut context,|loan|{valid.store(false,Ordering::Relaxed);assert!(loan.begin(OriginalSpanRequest{units:3,owned_bytes:8},&mut action).is_err());assert!(action.is_none());valid.store(true,Ordering::Relaxed);loan.begin(OriginalSpanRequest{units:3,owned_bytes:8},&mut action).unwrap();assert!(action.as_mut().unwrap().take_reply().unwrap().is_ok());loan.finish(&mut action).unwrap()}).unwrap();assert_eq!(calls.load(Ordering::Relaxed),1);assert!(authority.into_port().is_ok());
}
#[test]
fn checkpoint_recovers_original_boolean_without_replaying_producer(){
 struct Checkpoint{source:crate::OriginalSourceTurnIdentity,calls:Arc<AtomicUsize>}
 impl OriginalRecordedProducer for Checkpoint{
  type Request=OriginalCheckpointRequest;type Reply=bool;
  type Context<'turn,'source>=&'turn crate::OriginalSourceTurn where 'source:'turn;
  fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),crate::ValueError> where 'source:'turn{if c.identity()==self.source{Ok(())}else{Err(invalid())}}
  fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{Some(c.identity())}
  fn receive<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,request:Self::Request,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{assert_eq!(request,OriginalCheckpointRequest{completed:4,total:8,owned_bytes:16});self.calls.fetch_add(1,Ordering::Relaxed);let _done=reply.complete(Ok(false));panic!("original checkpoint interrupted after decision")}
  fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:Self::Request,reply:OriginalRecordedReply<'reply,Self::Reply>)->Result<OriginalRecordedReplyComplete<'reply,Self::Reply>,crate::ValueError> where 'source:'turn{Ok(reply.pending())}
 }
 let issuer=crate::OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let calls=Arc::new(AtomicUsize::new(0));let mut owner=OriginalRecordedAuthority::new(Checkpoint{source:turn.identity(),calls:calls.clone()});let mut context=&turn;let mut action=None;let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||owner.with_send_loan(&mut context,|loan|loan.begin(OriginalCheckpointRequest{completed:4,total:8,owned_bytes:16},&mut action)).unwrap().unwrap()));assert!(interrupted.is_err());owner.with_send_loan(&mut context,|loan|{assert!(loan.recover(action.as_mut().unwrap()).unwrap());assert!(!action.as_mut().unwrap().take_reply().unwrap().unwrap());loan.finish(&mut action).unwrap()}).unwrap();assert_eq!(calls.load(Ordering::Relaxed),1);assert!(owner.into_port().is_ok());
}

