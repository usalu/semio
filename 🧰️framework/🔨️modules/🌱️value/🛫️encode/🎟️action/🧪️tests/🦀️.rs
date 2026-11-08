//! 🎟️ Encoder action work retains its original loan, actual effect and paid close.
use super::*;
use crate::native_encoding::NativeEncodeProgress;
use std::panic::{catch_unwind,AssertUnwindSafe};
struct Cell(std::sync::atomic::AtomicU64);impl Cell{fn new(value:u64)->Self{Self(std::sync::atomic::AtomicU64::new(value))}fn get(&self)->u64{self.0.load(std::sync::atomic::Ordering::Relaxed)}fn set(&self,value:u64){self.0.store(value,std::sync::atomic::Ordering::Relaxed)}}
struct Loan(u64);
impl OriginalWorkRecipient for Loan{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalWorkQuote<'reply>,observed:u64,additional:u64)->crate::OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|sum|sum<=self.0)))}}
struct Recipient(crate::RetainedCloneGrant);
impl OriginalConstructionRecipient for Recipient{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn submitted<'turn,'source>(&self,_:&())->crate::RetainedCloneProgress where 'source:'turn{Default::default()}fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalConstructionGrant<'a>)->crate::OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(self.0))}fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:crate::OriginalConstructionAccept<'a>)->crate::OriginalConstructionAcceptComplete<'a>where 'source:'turn{receipt.complete(Ok(()))}}
fn close<T:crate::retirement::RetireOwned>(value:T,_grant:crate::RetainedCloneGrant){crate::eff::close(value);}
fn grant8()->crate::RetainedCloneGrant{crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}}
fn wide()->crate::RetainedCloneGrant{crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:4096}}
#[test]
fn original_encoder_append_reports_only_actual_effects(){
 let f:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🧫️fixtures/🎟️action-work.json")).unwrap();
 let grant:crate::RetainedCloneGrant=serde_json::from_value(f["close"].clone()).unwrap();
 for case in f["cases"].as_array().unwrap(){
  let accepted=case["observerAccepted"].as_bool().unwrap();let allocation_accepted=case["allocationAccepted"].as_bool().unwrap();let source=case["source"].as_str().unwrap();let mut output=Vec::new();
  let mut observer=|event:NativeEncodeProgress|accepted||event.completed==0;
  let mut work=OriginalWorkAuthority::new(Loan(case["allowance"].as_u64().unwrap()));let mut native=NativeEncodeControl::new(65536,&mut observer);
  let((result,_),(born,released))=crate::observe_retirement_allocations(||{let mut body=||work.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|{let mut child=|_|true;native.scoped_observer(&mut child,|native|native.append_bytes(&mut output,source.as_bytes()))})).unwrap();if allocation_accepted{(body(),0)}else{semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(body)}});
  let kind=match result{Ok(())=>"success",Err(error)=>{let kind=error.kind.as_str();close(error,grant);kind}};
  assert_eq!(serde_json::json!({"output":std::str::from_utf8(&output).unwrap(),"work":u64::try_from(native.actual_work()).unwrap(),"kind":kind}),case["expected"]);
  assert_eq!((born,released),(output.capacity(),0));
  drop(native);close(output,grant);assert!(work.into_recipient().is_ok());
 }
}
#[test]
fn scoped_encoder_unwind_returns_same_actual_work(){
 let mut observer=|_|true;let mut native=NativeEncodeControl::new(4096,&mut observer);let mut work=OriginalWorkAuthority::new(Loan(1));let mut output=Vec::new();
 let result=catch_unwind(AssertUnwindSafe(||work.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|{
  let mut child=|_|true;native.scoped_observer(&mut child,|native|->Result<(),ValueError>{native.append_bytes(&mut output,b"x")?;panic!("fixture unwind after actual append")})
 }))));
 assert!(result.is_err());assert_eq!(native.actual_work(),1);assert_eq!(output,b"x");
 let receipt=native.pause().unwrap();assert_eq!(receipt.actual_work(),1);let mut observer=|_|true;let native=NativeEncodeControl::resume(receipt,&mut observer).unwrap();assert_eq!(native.actual_work(),1);drop(native);
 close(output,wide());
}

#[test]
fn work_authority_identity_survives_short_loans_and_rejects_a_different_source(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🧫️fixtures/🎟️issuer.json")).unwrap();let allowance=fixture["allowance"].as_u64().unwrap();let mut authority=OriginalWorkAuthority::new(Loan(allowance));let mut other=OriginalWorkAuthority::new(Loan(allowance));let mut callback=|_|true;let mut native=NativeEncodeControl::new(1024,&mut callback);
 let mut identity=|authority:&mut OriginalWorkAuthority<Loan>|authority.with_loan(&mut (),|loan|native.with_work_recipient(loan,|native|Ok::<_,ValueError>(native.work_recipient_identity().unwrap()))).unwrap().unwrap();
 let first=identity(&mut authority);let second=identity(&mut authority);let different=identity(&mut other);assert_eq!(first,second);assert_ne!(first,different);
}

#[test]
fn construction_authority_identity_survives_short_loans(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧪️tests/🧫️fixtures/🎟️issuer.json")).unwrap();assert_eq!(fixture["allowance"],1);let mut authority=OriginalConstructionAuthority::new(Recipient(Default::default()));let mut other=OriginalConstructionAuthority::new(Recipient(Default::default()));let mut callback=|_|true;let mut native=NativeEncodeControl::new(1024,&mut callback);
 let mut identity=|authority:&mut OriginalConstructionAuthority<Recipient>|authority.with_loan(&mut (),|loan|native.with_construction_recipient(loan,|native|Ok::<_,ValueError>(native.construction_recipient_identity().unwrap()))).unwrap().unwrap();
 let first=identity(&mut authority);let second=identity(&mut authority);let different=identity(&mut other);assert_eq!(first,second);assert_ne!(first,different);
}

struct CountedWork<'a>{calls:&'a Cell,panic:bool,admission_panic:bool}
impl OriginalWorkRecipient for CountedWork<'_>{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalWorkQuote<'reply>,_:u64,_:u64)->crate::OriginalWorkQuoteComplete<'reply>where 'source:'turn{if self.admission_panic{self.calls.set(self.calls.get()+1);let reply=quote.complete(Ok(true));std::hint::black_box(&reply);panic!("work admission fixture")}quote.complete(Ok(true))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{if !self.admission_panic{self.calls.set(self.calls.get()+1)}let reply=check.complete(Ok(()));if self.panic{std::hint::black_box(&reply);panic!("original work post")}reply}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.pending()}}
struct CountedPhysical<'a>{calls:&'a Cell,panic:bool,quote_panic:bool,grant:crate::RetainedCloneGrant}
impl OriginalConstructionRecipient for CountedPhysical<'_>{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn submitted<'turn,'source>(&self,_:&())->crate::RetainedCloneProgress where 'source:'turn{Default::default()}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalConstructionGrant<'a>)->crate::OriginalConstructionGrantComplete<'a>where 'source:'turn{if self.quote_panic{self.calls.set(self.calls.get()+1);let reply=quote.complete(Ok(self.grant));std::hint::black_box(&reply);panic!("quote fixture")}quote.complete(Ok(self.grant))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:crate::OriginalConstructionAccept<'a>)->crate::OriginalConstructionAcceptComplete<'a>where 'source:'turn{if !self.quote_panic{self.calls.set(self.calls.get()+1)}let reply=receipt.complete(Ok(()));if self.panic{std::hint::black_box(&reply);panic!("original physical post")}reply}}
macro_rules! effect_turn{($native:ident,$work:ident,$physical:ident,$state:ident,$failure:ident,$local:ident)=>{$physical.with_loan(&mut (),|physical_loan|$work.with_loan(&mut (),|work_loan|$native.with_construction_recipient(physical_loan,|native|native.with_work_recipient(work_loan,|native|Ok::<_,ValueError>(native.effect_allocated_with_work(grant8(),&mut $local,grant8(),None,1,&mut $state.view(),&mut $failure,&mut |progress,effect|{assert!(effect.performed(1)?);*progress=RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()};Ok(())})))))).unwrap().unwrap().unwrap()}}
#[test]
fn original_effect_post_unwind_is_reconciled_without_replaying(){
 let f:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎟️action/🧪️tests/🧫️settlement.json")).unwrap();
 for case in f["cases"].as_array().unwrap(){let work_calls=Cell::new(0);let physical_calls=Cell::new(0);let panic=case["panic"].as_str().unwrap();
  let mut work=OriginalWorkAuthority::new(CountedWork{calls:&work_calls,panic:panic=="work",admission_panic:false});let mut physical=OriginalConstructionAuthority::new(CountedPhysical{calls:&physical_calls,panic:panic=="physical",quote_panic:false,grant:grant8()});
  let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut state=NativeEncodeEffectState::default();let mut failure=None;let mut local=RetainedCloneProgress::default();
  let attempt=catch_unwind(AssertUnwindSafe(||{effect_turn!(native,work,physical,state,failure,local)}));assert!(attempt.is_err());assert!(!crate::retirement::RetireOwned::retirement_ready(&state));
  let turn=effect_turn!(native,work,physical,state,failure,local);assert!(turn.result.unwrap().is_some());
  assert_eq!(work_calls.get(),case["workCalls"].as_u64().unwrap());assert_eq!(physical_calls.get(),case["physicalCalls"].as_u64().unwrap());
  drop(native);state.view().claim_producer_work().unwrap();close(state,wide());if let Some(failure)=failure{close(failure,wide())}assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());
 }
}

#[test]
fn original_effect_recovery_rejects_other_authority_before_callbacks(){
 let calls=Cell::new(0);let wrong_calls=Cell::new(0);let work_calls=Cell::new(0);
 let mut physical=OriginalConstructionAuthority::new(CountedPhysical{calls:&calls,panic:true,quote_panic:false,grant:grant8()});let mut wrong=OriginalConstructionAuthority::new(CountedPhysical{calls:&wrong_calls,panic:false,quote_panic:false,grant:grant8()});let mut work=OriginalWorkAuthority::new(CountedWork{calls:&work_calls,panic:false,admission_panic:false});
 let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut state=NativeEncodeEffectState::default();let mut failure=None;let mut local=RetainedCloneProgress::default();
 assert!(catch_unwind(AssertUnwindSafe(||{effect_turn!(native,work,physical,state,failure,local)})).is_err());assert_eq!(calls.get(),1);
 let refused=catch_unwind(AssertUnwindSafe(||{effect_turn!(native,work,wrong,state,failure,local)}));assert!(refused.is_err()||refused.unwrap().result.is_err());assert_eq!(wrong_calls.get(),0);assert_eq!(calls.get(),1);if failure.is_none(){failure=Some(Default::default())}
 let turn=effect_turn!(native,work,physical,state,failure,local);assert!(turn.result.unwrap().is_some());assert_eq!(calls.get(),1);
 drop(native);state.view().claim_producer_work().unwrap();close(state,wide());if let Some(failure)=failure{close(failure,wide())}
}

#[test]
fn original_effect_quote_and_work_admission_unwind_never_replay(){
 let f:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎟️action/🧪️tests/🧫️settlement.json")).unwrap();
 for case in f["admissionCases"].as_array().unwrap(){let which=case.as_str().unwrap();let q=Cell::new(0);let w=Cell::new(0);
  let mut physical=OriginalConstructionAuthority::new(CountedPhysical{calls:&q,panic:false,quote_panic:which=="quote",grant:grant8()});let mut work=OriginalWorkAuthority::new(CountedWork{calls:&w,panic:false,admission_panic:which=="work"});
  let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut state=NativeEncodeEffectState::default();let mut failure=None;let mut local=RetainedCloneProgress::default();
  assert!(catch_unwind(AssertUnwindSafe(||{effect_turn!(native,work,physical,state,failure,local)})).is_err());
  let turn=effect_turn!(native,work,physical,state,failure,local);assert!(turn.result.unwrap().is_some());assert_eq!(q.get(),1);assert_eq!(w.get(),1);
  drop(native);state.view().claim_producer_work().unwrap();close(state,wide());if let Some(failure)=failure{close(failure,wide())}
 }
}

#[test]
fn original_effect_short_quote_keeps_actual_grant_until_same_owner_releases(){
 let f:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎟️action/🧪️tests/🧫️settlement.json")).unwrap();let short=crate::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:f["shortQuoteBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 let calls=Cell::new(0);let mut physical=OriginalConstructionAuthority::new(CountedPhysical{calls:&calls,panic:false,quote_panic:false,grant:short});let mut work=OriginalWorkAuthority::new(Loan(1));let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut state=None;
 physical.with_loan(&mut (),|physical_loan|work.with_loan(&mut (),|work_loan|native.with_construction_recipient(physical_loan,|native|native.with_work_recipient(work_loan,|native|{assert!(!matches!(native.begin_effect_settlement(&mut state,crate::RetirementDemand{copy_bytes:8,depth:1,..Default::default()}),Ok(true)));assert_eq!(state.as_ref().unwrap().quoted_grant(),Some(short));assert!(!matches!(native.resume_effect_admission(&mut state),Ok(true)));assert_eq!(native.actual_work(),0);let _=native.release_refused_effect(&mut state)?;Ok::<_,ValueError>(())})))).unwrap().unwrap().unwrap();assert!(state.is_none());assert_eq!(calls.get(),0);
}

#[test]
fn original_effect_retains_both_owned_post_refusals(){
 struct Work(Option<ValueError>);
 impl OriginalWorkRecipient for Work{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalWorkQuote<'reply>,_:u64,_:u64)->crate::OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(true))}fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(Err(self.0.take().unwrap()))}fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.pending()}}
 struct Physical{grant:crate::RetainedCloneGrant,error:Option<ValueError>}
 impl OriginalConstructionRecipient for Physical{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn submitted<'turn,'source>(&self,_:&())->crate::RetainedCloneProgress where 'source:'turn{Default::default()}fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalConstructionGrant<'a>)->crate::OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(self.grant))}fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:crate::OriginalConstructionAccept<'a>)->crate::OriginalConstructionAcceptComplete<'a>where 'source:'turn{receipt.complete(Err(self.error.take().unwrap()))}}
 let f:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🎟️action/🧪️tests/🧫️settlement.json")).unwrap();let work_error=ValueError::new(ValueRefusalKind::WorkLimit,f["postRefusals"]["work"].as_str().unwrap().to_owned());let physical_error=ValueError::new(ValueRefusalKind::OwnershipLimit,f["postRefusals"]["physical"].as_str().unwrap().to_owned());let wp=work_error.message.as_ptr();let pp=physical_error.message.as_ptr();
 let mut work=OriginalWorkAuthority::new(Work(Some(work_error)));let mut physical=OriginalConstructionAuthority::new(Physical{grant:grant8(),error:Some(physical_error)});let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut state=NativeEncodeEffectState::default();let mut failure=None;let mut local=RetainedCloneProgress::default();
 let turn=effect_turn!(native,work,physical,state,failure,local);let refusal=turn.result.err().expect("both independent post refusals are retained");
 assert_eq!(native.actual_work(),1);assert_eq!(refusal.work.as_ref().unwrap().message.as_ptr(),wp);assert_eq!(refusal.physical.as_ref().unwrap().message.as_ptr(),pp);assert!(refusal.invariant.is_none());
 drop(native);state.view().claim_producer_work().unwrap();close(refusal,wide());close(state,wide());
}
