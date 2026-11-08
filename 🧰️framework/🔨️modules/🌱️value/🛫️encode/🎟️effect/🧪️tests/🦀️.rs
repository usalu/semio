//! 🧪️ Finite lower recipient loans retain independent effects and refusals.
use super::*;
use crate::retirement::RetireOwned;
use std::panic::{catch_unwind,AssertUnwindSafe};
fn error(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::Canceled,message)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:128,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:16}}
struct Work{posted:u64}
impl OriginalWorkRecipient for Work{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalWorkQuote<'reply>,_:u64,_:u64)->crate::OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(true))}fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{self.posted+=u64::try_from(check.delta()).unwrap();check.complete(Err(error("work post")))}fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.pending()}}
struct Physical{submitted:RetainedCloneProgress}
impl OriginalConstructionRecipient for Physical{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalConstructionGrant<'a>)->crate::OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(grant()))}fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:crate::OriginalConstructionAccept<'a>)->crate::OriginalConstructionAcceptComplete<'a>where 'source:'turn{match self.submitted.checked_add(receipt.progress()){Ok(total)=>{self.submitted=total;receipt.complete(Err(error("physical post")))},Err(refusal)=>receipt.complete(Err(refusal))}}}
fn close<T:RetireOwned>(value:T){crate::eff::close(value);}
/// 🧪️ A denied quote and a quoted closure that never performs both retire their construction action without posting work.
#[test]
fn denied_and_unperformed_effects_do_not_post_work(){
 for denied in [true,false]{let mut observer=|_|true;let mut native=NativeEncodeControl::new(8,&mut observer);let mut work=OriginalWorkAuthority::new(Work{posted:0});let mut physical=OriginalConstructionAuthority::new(Physical{submitted:Default::default()});let mut local=RetainedCloneProgress::default();let mut state=NativeEncodeEffectState::default();let mut called=false;let mut failure=None;let demand=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let original=if denied{RetainedCloneGrant{maximum_items:0,..grant()}}else{grant()};
  physical.with_loan(&mut (),|physical_loan|work.with_loan(&mut (),|work_loan|native.with_construction_recipient(physical_loan,|native|native.with_work_recipient(work_loan,|native|{let turn=native.effect_allocated(original,&mut local,demand,None,&mut state.view(),&mut failure,&mut |_,_|{called=true;Ok(())});assert_eq!(turn.work,0);if let Some(refused)=turn.result.err(){close(refused)}Ok::<(),ValueError>(())})))).unwrap().unwrap().unwrap();
  drop(native);assert_eq!(called,!denied);state.view().claim_producer_work().unwrap();assert_eq!(work.inspect(|payer|payer.posted),0);assert_eq!(physical.inspect(|recipient|recipient.submitted),RetainedCloneProgress::default());assert_eq!(local,RetainedCloneProgress::default());close(state);assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());}
}

#[test]
fn zero_backing_does_not_invent_an_allocation_event(){
 let mut observer=|_|true;let mut native=NativeEncodeControl::new(0,&mut observer);let mut local=RetainedCloneProgress::default();let mut state=NativeEncodeEffectState::default();let mut unwind=None;let turn=native.effect_allocated(grant(),&mut local,RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},Some(0),&mut state.view(),&mut unwind,&mut |_,_|panic!("zero backing must not execute"));assert_eq!(turn.work,0);assert_eq!(local,RetainedCloneProgress::default());assert!(unwind.is_none());state.view().claim_producer_work().unwrap();close(turn.result.err().unwrap());close(state);
}

#[test]
fn borrowed_status_keeps_original_settlement_refusals_in_place(){
 let mut observer=|_|true;let mut native=NativeEncodeControl::new(0,&mut observer);let mut work=OriginalWorkAuthority::new(Work{posted:0});let mut physical=OriginalConstructionAuthority::new(Physical{submitted:Default::default()});let mut state=NativeEncodeEffectState::default();let mut failure=None;let mut local=RetainedCloneProgress::default();let mut value=0u8;failure=Some(Default::default());let mut refused_value=None;assert_eq!(state.settlement_status(),NativeEncodeEffectSettlementStatus::Absent);
 let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1,maximum_depth:1,..Default::default()};
 physical.with_loan(&mut (),|physical_loan|work.with_loan(&mut (),|work_loan|native.with_construction_recipient(physical_loan,|native|native.with_work_recipient(work_loan,|native|{native.begin_effect_settlement(&mut state.settlement,RetirementDemand{copy_bytes:1,depth:1,..Default::default()})?;assert_eq!(state.view().settlement_status(),NativeEncodeEffectSettlementStatus::Pending);value=1;
  let turn=native.effect_allocated_with_work(grant(),&mut local,demand,None,1,&mut state.view(),&mut failure,&mut |progress,effect|{assert!(effect.performed(1)?);*progress=RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()};Ok(())});refused_value=Some(turn.result.err().expect("both independent post refusals are retained"));Ok::<(),ValueError>(())})))).unwrap().unwrap().unwrap();
 drop(native);let refused_value=refused_value.unwrap();assert_eq!(value,1);assert_eq!(state.settlement_status(),NativeEncodeEffectSettlementStatus::Absent);assert!(refused_value.work.is_some());assert!(refused_value.physical.is_some());assert!(refused_value.invariant.is_none());state.view().claim_producer_work().unwrap();close(refused_value);close(state);assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());
}

#[test]
fn native_encode_depth_restores_the_original_scope_after_unwind(){
 let mut observer=|_|true;let mut native=NativeEncodeControl::new(0,&mut observer);let panic=catch_unwind(AssertUnwindSafe(||native.scoped_depth(1,|_|->Result<(),ValueError>{panic!("fixture nested unwind")})));assert!(panic.is_err());let mut entered=false;native.scoped_depth(1,|_|{entered=true;Ok::<(),ValueError>(())}).unwrap();assert!(entered);
}

struct Accepting{posted:u64}
impl OriginalWorkRecipient for Accepting{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{None}fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:crate::OriginalWorkQuote<'reply>,_:u64,_:u64)->crate::OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(true))}fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{self.posted+=u64::try_from(check.delta()).unwrap();check.complete(Ok(()))}fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut (),check:crate::OriginalWorkPartialCheck<'reply>)->crate::OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.pending()}}
/// 🧪️ The shared four-driver corpus: the encode control settles every case exactly as the job provider and the authoring wallet do.
#[test]
fn the_shared_corpus_settles_identically_through_the_encode_control(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/🪙️effect-provider/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let maximum=case["maximumWork"].as_u64().unwrap();let performed=u64::try_from(case["performed"].as_u64().unwrap()).unwrap();let steps:Vec<u64>=case["performs"].as_array().unwrap().iter().map(|step|step.as_u64().unwrap()).collect();
  let mut observer=|_|true;let mut native=NativeEncodeControl::new(8,&mut observer);let mut work=OriginalWorkAuthority::new(Accepting{posted:0});let mut physical=OriginalConstructionAuthority::new(Physical{submitted:Default::default()});let mut admitted=None;
  physical.with_loan(&mut (),|physical_loan|work.with_loan(&mut (),|work_loan|native.with_construction_recipient(physical_loan,|native|native.with_work_recipient(work_loan,|native|{admitted=native.with_effect(grant(),maximum,|_,effect|steps.iter().all(|step|effect.performed(*step).unwrap()))?;Ok::<(),ValueError>(())})))).unwrap().unwrap().unwrap();
  drop(native);assert_eq!(admitted,Some(true),"{}",case["name"]);assert_eq!(work.inspect(|payer|payer.posted),performed,"{}",case["name"]);assert!(performed<=maximum);assert_eq!(physical.inspect(|recipient|recipient.submitted),RetainedCloneProgress::default());
  assert!(work.into_recipient().is_ok());assert!(physical.into_recipient().is_ok());
 }
}
