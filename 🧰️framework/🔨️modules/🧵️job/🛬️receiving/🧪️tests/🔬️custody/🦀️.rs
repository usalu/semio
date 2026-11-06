use semio_framework_test_fixtures::fixture_text;
use super::*;
struct Work{remaining:u64,committed:u64}
impl Work{fn receive_actual(&mut self,actual:u64)->Result<(),ValueError>{self.remaining=self.remaining.checked_sub(actual).ok_or_else(||ValueError::invariant("fixture actual work overdraw"))?;self.committed=self.committed.checked_add(actual).ok_or_else(||ValueError::invariant("fixture actual work overflow"))?;Ok(())}}
impl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.remaining.checked_sub(observed).ok_or_else(||ValueError::invariant("observed fixture work exceeds its finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(additional<=self.available_work(observed)?)}}
fn admit(owner:&mut OriginalDecodeState,grant:RetainedCloneGrant){assert!(owner.admit_work_issuer(grant,1).result.unwrap());}
struct Construction{grant:RetainedCloneGrant,spent:RetainedCloneProgress}
impl NativeDecodeConstructionRecipient for Construction{fn submitted(&self)->RetainedCloneProgress{self.spent}fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{if !semio_framework_value::retained_clone::retained_clone_grant_admits(self.grant,d){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original fixture construction denied"));}Ok(self.grant)}fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(g));self.spent=self.spent.checked_add(p)?;Ok(())}}
#[test]
fn original_job_receiving_preserves_ledger_recipient_and_cumulative_capacity(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for case in corpus["cases"].as_array().unwrap(){let mut owner=OriginalDecodeState::empty();let mut identity=None;let mut recipient=None;let cancelled=case["cancelled"].as_bool().unwrap();let mut callback=|_:NativeDecodeProgress|!cancelled;
  for capacity in case["turnCapacity"].as_array().unwrap(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:capacity.as_u64().unwrap()as usize,maximum_release_bytes:4096,maximum_depth:16};let mut construction=Construction{grant,spent:Default::default()};admit(&mut owner,grant);let mut work=Work{remaining:1000000,committed:0};let mut observed=0;let result=owner.receive(grant,&mut callback,&mut construction,&mut work,&mut observed,|control|{if let Some(original)=identity{assert_eq!(control.ledger_identity(),original);}else{identity=Some(control.ledger_identity());}if let Some(original)=recipient{assert_eq!(control.retirement_recipient_identity(),Some(original));}else{recipient=control.retirement_recipient_identity();}control.copy_text(case["text"].as_str().unwrap())});work.receive_actual(observed).unwrap();if cancelled{let error=result.err().expect("cancelled decode refuses");assert_eq!(error.context.as_ref().or(error.operation.as_ref()).unwrap().kind,ValueRefusalKind::Canceled);assert!(error.output.is_none());}else{let actual=result.unwrap();assert_eq!(serde_json::to_value(&actual).unwrap(),case["text"]);let bytes=actual.capacity();let close=RetainedCloneGrant{maximum_release_bytes:bytes,..grant};semio_framework_value::advance_retirement_turn(RetirementDemand{release_bytes:bytes,depth:1,..Default::default()},close,|_|{drop(actual);Ok::<_,ValueError>((RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}),true))}).unwrap();}}
  assert_eq!(owner.owned_bytes(),case["expectedOwned"].as_u64().unwrap()as usize);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:16};while !owner.terminal_is_empty(){assert!(owner.close_step(grant).result.unwrap().progress().fits(grant));}assert!(owner.terminal_is_empty());
 }
}

#[test]
fn original_job_receiving_restores_continuation_and_construction_issuer(){
 use semio_framework_value_native_owned::native_decoding::NativeReceivingEffect;
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/🎟️construction.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
 let mut owner=OriginalDecodeState::empty();admit(&mut owner,grant);let mut callback=|_|true;let mut construction=Construction{grant,spent:Default::default()};let mut identity=None;let mut recipient=None;
 let mut initial_work=Work{remaining:1000000,committed:0};let mut observed=0;let panic=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||owner.receive(grant,&mut callback,&mut construction,&mut initial_work,&mut observed,|control|->Result<(),ValueError>{identity=Some(control.ledger_identity());recipient=control.retirement_recipient_identity();panic!("original Job operation unwound")})));
 initial_work.receive_actual(observed).unwrap();assert!(panic.is_err());assert!(owner.receiving_identity().is_some());assert_eq!(owner.owned_bytes(),0);
 let mut work=Work{remaining:fixture["expectedWork"].as_u64().unwrap(),committed:0};
 let mut frame=NativeReceivingEffect::<ValueError>::default();
 owner.receive(grant,&mut callback,&mut construction,&mut work,&mut observed,|control|{
  assert_eq!(Some(control.ledger_identity()),identity);assert_eq!(control.retirement_recipient_identity(),recipient);
  frame.quote(control,RetirementDemand{copy_bytes:1,depth:1,..Default::default()})?;
  assert!(control.prepare_work(fixture["expectedWork"].as_u64().unwrap())?);
  frame.effect(control,|actual,_|{*actual=Some(0);semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})),work:fixture["expectedWork"].as_u64().unwrap()}})?;
  frame.settle_original_work(control,fixture["expectedWork"].as_u64().unwrap())?;Ok::<_,ValueError>(())
 }).unwrap();work.receive_actual(observed).unwrap();
 assert_eq!(work.committed,fixture["expectedWork"].as_u64().unwrap());
 let turn=owner.receive(grant,&mut callback,&mut construction,&mut work,&mut observed,|control|{assert_eq!(Some(control.ledger_identity()),identity);frame.receipt(control)?;frame.take(control)}).unwrap();work.receive_actual(observed).unwrap();
 assert_eq!(turn.work,fixture["expectedWork"].as_u64().unwrap());assert!(turn.result.is_ok());assert_eq!(serde_json::to_value(construction.spent).unwrap(),fixture["receipt"]);assert_eq!(serde_json::to_value(turn.result.as_ref().ok().unwrap().progress()).unwrap(),fixture["receipt"]);
 while !owner.terminal_is_empty(){owner.close_step(grant).result.unwrap();}
}
