//! 🔐️ Detached local receipts require the genuine original recipient in both native directions.
use semio_framework_value_native_decoding::*;
use retirement::controlled::ControlledRetirement;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[path="../../../🎟️work/🧪️tests/eff.rs"]
mod eff;
use semio_framework_test_fixtures::fixture_text;
fn fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../🫴️recipient/🔁️continuation/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn grant(value:&serde_json::Value)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap()as usize}}
struct OriginalCloseWork(u64);
impl OriginalWorkRecipient for OriginalCloseWork{type Context<'turn,'source>=() where 'source:'turn;fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}fn settle_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}fn recover_partial<'reply,'turn,'source>(&mut self,context:&mut (),check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply>where 'source:'turn{check.complete(self.validate_context(context))}fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply>where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=self.0)))}}
macro_rules! paid_close{
 (decode,$receipt:ident,$accepted:expr,$grant:expr,$work:expr,$operation:expr)=>{$receipt.retire::<_,ValueError>($grant,$accepted,$work,$operation)};
 (encode,$receipt:ident,$accepted:expr,$grant:expr,$work:expr,$operation:expr)=>{$receipt.encode($accepted,$operation)};
}
macro_rules! close_turn{
 (decode,$native:ident,$grant:expr,$marker:ident)=>{if $native.requires_original_birth(){observe(||eff::settled($native.close_original_birth_step($grant)))}else{eff::observed($grant,1<<20,&mut $marker,|effect|eff::settled($native.close_retirement_recipient(effect)))}};
 (encode,$native:ident,$grant:expr,$marker:ident)=>{eff::observed($grant,1<<20,&mut $marker,|effect|eff::settled($native.close_retirement_recipient(effect)))};
}
macro_rules! original_local_law{
 ($name:ident,$control:path,$recipient:path,$hop:ident)=>{
  #[test]fn $name(){
   use $control as Control;use $recipient as Recipient;
   let law=fixture();let mut work=native_decoding::OriginalWorkAuthority::new(OriginalCloseWork(law["maximumCloseWork"].as_u64().unwrap()));let mut work_context=();work.with_loan(&mut work_context,|native_work_loan|{let mut accepted=|_|true;let recipient_bytes=std::mem::size_of::<Recipient>();let admitted={let mut native=Control::new(law["maximumNativeBytes"].as_u64().unwrap()as usize,&mut accepted);native.charge(recipient_bytes).unwrap();native.pause().unwrap()};
   let(recipient,heap)=observe(||Box::new(Recipient::new()));assert_eq!(heap,(recipient_bytes,0));let mut recipient=recipient;let mut native=Control::resume(admitted,&mut accepted).unwrap();let pointer=recipient.as_ref()as *const Recipient;
   native.install_retirement_recipient(&mut recipient).unwrap();native.charge(law["initialBytes"].as_u64().unwrap()as usize).unwrap();
   let wrapper=std::mem::size_of::<ControlledRetirement<String>>();
   let(error,heap)=observe(||native.with_retirement_owner::<(),ValueError>(wrapper,|native|{let text=native.copy_text(law["text"].as_str().unwrap()).unwrap();assert_eq!(text.len(),law["textBytes"].as_u64().unwrap()as usize);let owner=Box::new(ControlledRetirement::new(text).unwrap());(Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original local output retained")),Some(owner))}).unwrap_err());assert_eq!(heap,(wrapper+law["textBytes"].as_u64().unwrap()as usize,0));
   assert_eq!(error.kind,ValueRefusalKind::InvalidValue);let before=native.owned_bytes();assert_eq!(before,recipient_bytes+law["initialBytes"].as_u64().unwrap()as usize+wrapper+law["textBytes"].as_u64().unwrap()as usize);
   let(detached,_)=native.pause_with_retirement().unwrap().detach();assert!(recipient.has_owner());assert_eq!(detached.owned_bytes(),before);assert_eq!(recipient.as_ref()as *const Recipient,pointer);
   let mut replacement=Recipient::new();let(error,detached)=match detached.bind(&mut replacement){Err(refused)=>refused,Ok(_)=>panic!("replacement recipient acquired original receipt")};assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);assert!(!replacement.has_owner());assert!(recipient.has_owner());assert_eq!(detached.owned_bytes(),before);
   let mut receipt=detached.bind(&mut recipient).map_err(|(error,_)|error).unwrap();let mut accepted=|_|true;let denied=grant(&law["deniedGrant"]);let closing=grant(&law["closeGrant"]);let step=paid_close!($hop,receipt,&mut accepted,closing,native_work_loan,|native|{let mut marker=None;let(turn,_)=close_turn!($hop,native,denied,marker);assert_eq!(turn.work,0);Ok(turn.result.unwrap())}).unwrap();assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(receipt.owned_bytes(),before);
   let mut released=0;let mut born=0;let mut terminal=false;
   for _ in 0..law["maximumTurns"].as_u64().unwrap(){let done=paid_close!($hop,receipt,&mut accepted,closing,native_work_loan,|native|{let mut marker=None;let(result,heap)=close_turn!($hop,native,closing,marker);let work=result.work;let step=result.result.unwrap();assert!(step.progress()==Default::default()||work>0);assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(closing));released+=step.progress().released_bytes;born+=step.progress().retained_capacity_bytes;Ok(!native.has_retirement_owner())}).unwrap();if done{terminal=true;break}}
   assert!(terminal);let(detached,_)=receipt.detach();assert_eq!(detached.owned_bytes(),before+born,"original admission must retain all cleanup births");assert!(!recipient.has_owner());assert_eq!(recipient.as_ref()as *const Recipient,pointer);assert_eq!(released,wrapper+law["textBytes"].as_u64().unwrap()as usize+born,"payload cleanup must release wrapper={wrapper} and cleanup births={born}");
   let oracle=serde_json::to_string(law["text"].as_str().unwrap()).unwrap();assert_eq!(serde_json::from_str::<String>(&oracle).unwrap().len(),law["textBytes"].as_u64().unwrap()as usize);
   let mut detached=detached;let mut recipient=Some(recipient);let mut canceled=|_|false;for case in law["recipientReleaseRefusals"].as_array().unwrap(){let mut denied=closing;match case["axis"].as_str().unwrap(){"depth"=>denied.maximum_depth=case["value"].as_u64().unwrap()as usize,"release"=>denied.maximum_release_bytes=case["value"].as_u64().unwrap()as usize,_=>unreachable!()}let(result,heap)=observe(||detached.release_recipient_box(&mut recipient,denied,&mut accepted));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind.as_str(),case["kind"].as_str().unwrap());assert_eq!(recipient.as_ref().unwrap().as_ref()as *const Recipient,pointer);assert_eq!(detached.owned_bytes(),before+born);}assert_eq!(detached.release_recipient_box(&mut recipient,denied,&mut accepted).unwrap(),Default::default());assert!(recipient.is_some());let error=detached.release_recipient_box(&mut recipient,closing,&mut canceled).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::Canceled);assert_eq!(recipient.as_ref().unwrap().as_ref()as *const Recipient,pointer);assert_eq!(detached.owned_bytes(),before+born);let(final_release,heap)=observe(||detached.release_recipient_box(&mut recipient,closing,&mut accepted).unwrap());assert_eq!(heap,(0,recipient_bytes));assert!(final_release.fits(closing));assert!(recipient.is_none());assert_eq!(final_release.released_bytes,recipient_bytes);released+=final_release.released_bytes;assert_eq!(released,recipient_bytes+wrapper+law["textBytes"].as_u64().unwrap()as usize+born,"original recipient cleanup must release recipient={recipient_bytes}, wrapper={wrapper} and cleanup births={born}");}).unwrap();
  }
 }
}
original_local_law!(native_local_original_decode_recipient_survives_detachment,native_decoding::NativeDecodeControl,native_decoding::NativeDecodeRetirementRecipient,decode);
original_local_law!(native_local_original_encode_recipient_survives_detachment,native_encoding::NativeEncodeControl,native_encoding::NativeEncodeRetirementRecipient,encode);
macro_rules! refused_work_kind{(decode,$error:ident)=>{$error.operation.as_ref().or($error.context.as_ref()).expect("native refusal retained").kind.as_str()};(encode,$error:ident)=>{$error.kind.as_str()};}
macro_rules! refused_work_law{
 ($name:ident,$control:path,$recipient:path,$hop:ident)=>{
  #[test]fn $name(){
   use $control as Control;use $recipient as Recipient;let law=fixture();let mut accepted=|_|true;let mut recipient=Recipient::new();let mut native=Control::new(law["maximumNativeBytes"].as_u64().unwrap()as usize,&mut accepted);native.install_retirement_recipient(&mut recipient).unwrap();native.charge(law["initialBytes"].as_u64().unwrap()as usize).unwrap();native.begin_stage(law["rejectedWork"]["declared"].as_u64().unwrap()as usize).unwrap();let(detached,_)=native.pause_with_retirement().unwrap().detach();let mut receipt=detached.bind(&mut recipient).map_err(|(error,_)|error).unwrap();let mut accepted=|_|true;let before=receipt.owned_bytes();let error=receipt.$hop(&mut accepted,|native|native.advance(law["rejectedWork"]["attempted"].as_u64().unwrap()as usize)).unwrap_err();assert_eq!(refused_work_kind!($hop,error),law["rejectedWork"]["kind"].as_str().unwrap());assert_eq!(receipt.owned_bytes(),before);receipt.$hop(&mut accepted,|native|native.advance(law["rejectedWork"]["declared"].as_u64().unwrap()as usize)).unwrap();let(detached,_)=receipt.detach();assert_eq!(detached.owned_bytes(),before);assert!(!recipient.has_owner());
  }
 }
}
refused_work_law!(native_local_original_decode_refused_work_preserves_receipt,native_decoding::NativeDecodeControl,native_decoding::NativeDecodeRetirementRecipient,decode);
refused_work_law!(native_local_original_encode_refused_work_preserves_receipt,native_encoding::NativeEncodeControl,native_encoding::NativeEncodeRetirementRecipient,encode);
#[test]
fn original_decode_envelope_keeps_result_and_context_refusal(){
 let law=fixture();for fail in [true,false]{
  let mut recipient=NativeDecodeRetirementRecipient::new();let mut progress=|_|true;let receipt=NativeDecodeControl::new_retained(&mut progress).pause().unwrap();let mut continuation=receipt.with_retirement_recipient(&mut recipient).map_err(|(error,_)|error).unwrap();let mut progress=|_|true;let mut authority=OriginalWorkAuthority::new(OriginalCloseWork(1));
  let value=law["callEnvelope"][if fail{"operation"}else{"output"}].as_str().unwrap().to_owned();let pointer=value.as_ptr();
  let result=authority.with_loan(&mut (),|loan|continuation.decode::<String,ValueError>(&mut progress,|control|control.with_work_recipient(loan,|original|{assert!(original.prepare_work(1)?);if fail{Err(ValueError::new(ValueRefusalKind::InvalidValue,value))}else{Ok(value)}}))).unwrap();
  assert_eq!(continuation.pending_work(),1);assert!(continuation.has_pending_work());
  let failure=result.unwrap_err();if fail{assert_eq!(failure.operation.as_ref().unwrap().message.as_ptr(),pointer);assert!(failure.output.is_none())}else{assert_eq!(failure.output.as_ref().unwrap().as_ptr(),pointer);assert!(failure.operation.is_none())}assert!(failure.context.is_some());
  authority.with_loan(&mut (),|loan|continuation.decode::<(),ValueError>(&mut progress,|control|control.with_work_recipient(loan,|control|control.commit_work(1)))).unwrap().unwrap();assert!(!continuation.has_pending_work());let(_receipt,_recipient)=continuation.detach();assert!(authority.into_recipient().is_ok());
 }
}
