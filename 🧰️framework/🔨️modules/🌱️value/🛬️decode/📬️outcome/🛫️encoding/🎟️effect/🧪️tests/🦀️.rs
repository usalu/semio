//! 🧪️ Real original producer, Work and construction owners settle independent errors.
use super::*;
use semio_framework_value_control_contract as core;
use semio_framework_value_receiving::span::effect::*;
struct Work;
impl core::OriginalWorkRecipient for Work{
 type Context<'a,'b>=&'a core::OriginalSourceTurn where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError> where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<core::OriginalSourceTurnIdentity> where 'b:'a{Some(c.identity())}
 fn admits<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalWorkQuote<'r>,_:u64,_:u64)->core::OriginalWorkQuoteComplete<'r> where 'b:'a{q.complete(Ok(true))}
 fn settle_partial<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalWorkPartialCheck<'r>)->core::OriginalWorkPartialCheckComplete<'r> where 'b:'a{q.complete(Ok(()))}
 fn recover_partial<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalWorkPartialCheck<'r>)->core::OriginalWorkPartialCheckComplete<'r> where 'b:'a{q.pending()}
}
struct Physical{progress:RetainedCloneProgress,error:Option<ValueError>}
impl core::OriginalConstructionRecipient for Physical{
 type Context<'a,'b>=&'a core::OriginalSourceTurn where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError> where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<core::OriginalSourceTurnIdentity> where 'b:'a{Some(c.identity())}
 fn submitted<'a,'b>(&self,_:&Self::Context<'a,'b>)->RetainedCloneProgress where 'b:'a{self.progress}
 fn grant<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalConstructionGrant<'r>)->core::OriginalConstructionGrantComplete<'r> where 'b:'a{q.complete(Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:16,maximum_depth:1,..Default::default()}))}
 fn accept<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,q:core::OriginalConstructionAccept<'r>)->core::OriginalConstructionAcceptComplete<'r> where 'b:'a{self.progress=q.progress();q.complete(Err(self.error.take().unwrap()))}
}
struct Producer{error:Option<ValueError>,calls:usize,input:[u8;8],output:[u8;8]}
impl OriginalEffectProducer for Producer{
 type Output=NativeEncodeProducerOutcome;type Failure=ValueError;type Context<'a,'b>=&'a core::OriginalSourceTurn where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError> where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<core::OriginalSourceTurnIdentity> where 'b:'a{Some(c.identity())}
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut core::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeEncodeProducerOutcome,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeEncodeProducerOutcome,ValueError>,ValueError> where 'b:'a{self.calls+=1;assert!(effect.performed(1).unwrap());self.output.copy_from_slice(&self.input);let _reply=reply.complete(Ok(NativeEncodeProducerOutcome{performed:true,progress:RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()},result:Err(self.error.take().unwrap())}));panic!("actual producer interrupted after reply")}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:&mut core::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,NativeEncodeProducerOutcome,ValueError>)->Result<OriginalEffectReplyComplete<'r,NativeEncodeProducerOutcome,ValueError>,ValueError> where 'b:'a{Ok(reply.pending())}
}
#[test]
fn original_work_and_physical_refusals_preserve_the_actual_producer_reply(){
 let issuer=core::OriginalSourceTurnIssuer::new();let turn=issuer.issue().unwrap();let source_error=ValueError::new(ValueRefusalKind::Canceled,"actual producer refusal".to_owned());let source_pointer=source_error.message.as_ptr();let physical_error=ValueError::new(ValueRefusalKind::Canceled,"actual physical refusal".to_owned());let physical_pointer=physical_error.message.as_ptr();let mut producer=OriginalEffectAuthority::new(Producer{error:Some(source_error),calls:0,input:*b"receipt!",output:[0;8]});let mut payer=core::OriginalWorkAuthority::new(Work);let mut physical=core::OriginalConstructionAuthority::new(Physical{progress:Default::default(),error:Some(physical_error)});let mut state=None;let mut action=None;let mut actual=None;let mut receipt=None;let mut reply=None;let demand=RetirementDemand{copy_bytes:8,depth:1,..Default::default()};let progress=RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()};
 payer.with_loan(&mut &turn,|work_loan|physical.with_loan(&mut &turn,|physical_loan|{
  let identity=physical_loan.identity();let mut work_loan=work_loan.reborrow();let mut physical_loan=physical_loan.reborrow();let mut construction=Some(&mut physical_loan);let mut work=core::OriginalWork::empty();work.recipient=Some(&mut work_loan);{let mut recovery=NativeEncodeEffectRecovery::new(&mut work,&mut construction,Some(identity));assert!(recovery.begin(&mut state,demand).unwrap())}
  {let mut turn=core::OriginalWorkTurn::new(&mut work);state.as_ref().unwrap().issue_performed(&mut turn,&mut receipt).unwrap()}
  let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let mut effect=core::RetainedAllocationEffect::new(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8,maximum_depth:1,..Default::default()},1,&mut actual,receipt.as_mut().unwrap());producer.drive(&mut &turn,&mut effect,&mut action).unwrap()}));assert!(interrupted.is_err());{let mut w=core::OriginalWorkTurn::new(&mut work);state.as_ref().unwrap().claim_performed(&mut w,&mut receipt).unwrap();let mut partial=None;assert!(state.as_ref().unwrap().settle_performed_into(&mut w,&mut partial).unwrap());assert_eq!(partial.take().unwrap().delta(),1);state.as_ref().unwrap().issue_performed(&mut w,&mut receipt).unwrap()}
  {let mut effect=core::RetainedAllocationEffect::new(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8,maximum_depth:1,..Default::default()},1,&mut actual,receipt.as_mut().unwrap());assert!(producer.drive(&mut &turn,&mut effect,&mut action).unwrap())}{let mut w=core::OriginalWorkTurn::new(&mut work);state.as_ref().unwrap().claim_performed(&mut w,&mut receipt).unwrap();assert!(action.as_mut().unwrap().transfer_reply(&mut reply).unwrap());producer.finish(&mut action).unwrap();assert!(state.as_mut().unwrap().record_performed_before_post(&mut w,progress,true,None).unwrap())}
  {let mut recovery=NativeEncodeEffectRecovery::new(&mut work,&mut construction,Some(identity));assert!(recovery.settle(&mut state).unwrap())}let mut failure=NativeEncodeEffectFailure::default();assert!(NativeEncodeEffectSettlement::finish_into(&mut state,&mut failure).unwrap());assert_eq!(failure.physical.as_ref().unwrap().message.as_ptr(),physical_pointer);let outcome=reply.take().unwrap().unwrap();assert!(outcome.performed);assert_eq!(outcome.progress,progress);assert_eq!(outcome.result.unwrap_err().message.as_ptr(),source_pointer);
 }).unwrap()).unwrap();let producer=producer.into_producer().ok().unwrap();assert_eq!(producer.calls,1);assert_eq!(producer.output,*b"receipt!");assert!(!payer.has_live_actions());assert!(physical.into_recipient().is_ok())
}
