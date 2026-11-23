//! 🧪️ Snapshot callback and unwind custody survives typed retirement and same-original recovery.
use semio_framework_io_control::{NativeSnapshotEncodeOwner,NativeSnapshotEncodeSettlement};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,ValueError,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value::native_encoding::*;
use std::panic::{catch_unwind,AssertUnwindSafe};
struct Work{remaining:u64,quote:u64}
impl OriginalWorkRecipient for Work{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{assert_eq!(additional,self.quote);quote.complete(Ok(observed.checked_add(additional).is_some_and(|total|total<=self.remaining)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
 fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}
}
struct Physical{remaining:RetainedCloneGrant,submitted:RetainedCloneProgress,interrupt:bool}
impl OriginalConstructionRecipient for Physical{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
 fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(self.remaining))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let progress=receipt.progress();assert!(progress.fits(receipt.grant())&&progress.fits(self.remaining));self.remaining.maximum_items-=progress.copied_items;self.remaining.maximum_copy_bytes-=progress.copied_bytes;let total=self.submitted.checked_add(progress);if let Ok(total)=total{self.submitted=total}let done=receipt.complete(total.map(|_|()));if std::mem::take(&mut self.interrupt){panic!("actual original physical post interrupted")}done}
}
fn close<T:RetireOwned>(value:T){
 let grant=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 let mut owner=ControlledRetirement::retain(value);let mut bank=semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::BankWallet::new(grant,1<<40));
 for _ in 0..4096{if owner.terminal_is_empty(){break}let turn=bank.run_outcome(grant,1<<20,|effect|{let mut turn=None;semio_framework_value_control_contract::with_original_invocation(None,effect,&mut |effect|turn=Some(match owner.step(65536,effect){semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{..}=>panic!("settlement close stayed pending")})).unwrap();turn.unwrap()});turn.result.unwrap();bank.take_admitted();}
 assert!(owner.terminal_is_empty());
}
#[test]
fn snapshot_typed_settlement_preserves_original_callbacks_and_actual_unwind(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🌱️value/🛫️encode/♻️retirement/🧪️tests/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap().iter().filter(|row|row["kind"]=="settlement"){
  let quote=row["selectedWork"].as_u64().unwrap();let actual=row["actualWork"].as_u64().unwrap();let unwinds=row["id"]=="settlement-physical-pending";
  let effect_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_depth:1,..Default::default()};let progress=RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()};
  let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut slot=None;let mut output=[0u8;3];
  let mut construction=OriginalConstructionAuthority::new(Physical{remaining:effect_grant,submitted:Default::default(),interrupt:unwinds});let mut work=OriginalWorkAuthority::new(Work{remaining:quote,quote});
  construction.with_loan(&mut (),|construction_loan|work.with_loan(&mut (),|work_loan|native.with_construction_recipient(construction_loan,|native|native.with_work_recipient(work_loan,|native|{
   let mut owner=NativeSnapshotEncodeOwner::new(native,effect_grant);
   assert!(owner.begin_original_settlement(&mut slot,effect_grant,quote)?);
   assert_eq!(actual,output.len() as u64);for byte in &mut output{*byte=1;}
   assert!(owner.record_original_effect(&mut slot,progress,true,Some(actual))?);
   if unwinds{let payload=catch_unwind(||std::panic::panic_any(42usize)).unwrap_err();assert!(slot.as_mut().unwrap().retain_operation_unwind(payload).is_ok());}
   let first=catch_unwind(AssertUnwindSafe(||owner.settle_original_effect(&mut slot)));
   assert_eq!(first.is_err(),unwinds);
   if unwinds{assert!(!slot.as_ref().unwrap().retirement_ready());assert!(owner.settle_original_effect(&mut slot)?);}else{assert!(first.unwrap()?);}
   assert_eq!(owner.progress(),progress);
   if unwinds{
    let receipt=NativeSnapshotEncodeSettlement::take_receipt(&mut slot)?;close(receipt);
    let resumed=catch_unwind(AssertUnwindSafe(||NativeSnapshotEncodeSettlement::resume_operation_unwind(&mut slot)));let payload=resumed.unwrap_err().downcast::<usize>().unwrap();assert_eq!(*payload,42);assert!(slot.is_none());
   }else{assert!(slot.as_ref().unwrap().retirement_ready());}
   Ok::<(),ValueError>(())
  })))).unwrap().unwrap().unwrap();
  let submitted=construction.inspect(|physical|physical.submitted);
  assert_eq!(output,[1;3]);assert_eq!(native.actual_work(),u128::from(actual));assert_eq!(quote-actual,row["unspentWork"].as_u64().unwrap());assert_eq!(submitted.copied_items,1);assert_eq!(submitted.copied_bytes,3);
  if let Some(original)=slot.take(){close(original);}
 }
}
