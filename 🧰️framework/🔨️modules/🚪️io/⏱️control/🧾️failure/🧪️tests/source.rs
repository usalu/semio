//! 🧭️ One actual recorded Source turn binds every original authority of a scoped encoder.
use semio_framework_value::native_encoding::*;
use semio_framework_value_control_contract::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
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
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,reply:OriginalRecordedReply<'r,bool>)->Result<OriginalRecordedReplyComplete<'r,bool>,ValueError> where 'b:'a{Ok(reply.complete(Ok(true)))}
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
 let result=allocation.with_send_loan(&mut a,|allocation|span.with_send_loan(&mut s,|span|checkpoint.with_send_loan(&mut c,|checkpoint|{let mut control=NativeEncodeControl::new_recorded_source(maximum,depth,turn.witness(),allocation,span,checkpoint).unwrap();operation(&mut control)}).unwrap()).unwrap()).unwrap();
 assert!(allocation.into_port().is_ok());assert!(span.into_port().is_ok());assert!(checkpoint.into_port().is_ok());result
}
/// 🧭️ Runs one operation against a recorded Source control whose Work and construction authorities share that Source turn.
pub fn with_source_authorities<R>(maximum:usize,depth:usize,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->R)->R{
 let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let mut allocation=NativeAllocationAuthority::new(Source(turn.identity()));let mut work=OriginalWorkAuthority::new(Source(turn.identity()));let mut physical=OriginalConstructionAuthority::new(Source(turn.identity()));let mut span=OriginalRecordedAuthority::new(Span(turn.identity()));let mut checkpoint=OriginalRecordedAuthority::new(Check(turn.identity()));let(mut a,mut w,mut p,mut s,mut c)=(&turn,&turn,&turn,&turn,&turn);
 let result=allocation.with_send_loan(&mut a,|allocation|work.with_loan(&mut w,|work|physical.with_loan(&mut p,|physical|span.with_send_loan(&mut s,|span|checkpoint.with_send_loan(&mut c,|checkpoint|{let mut control=NativeEncodeControl::new_recorded_source(maximum,depth,turn.witness(),allocation,span,checkpoint).unwrap();control.with_work_recipient(work,|control|control.with_construction_recipient(physical,|control|Ok::<_,ValueError>(operation(control)))).unwrap()}).unwrap()).unwrap()).unwrap()).unwrap()).unwrap();
 assert!(allocation.into_port().is_ok());assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());assert!(span.into_port().is_ok());assert!(checkpoint.into_port().is_ok());result
}
