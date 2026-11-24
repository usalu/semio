//! 🧪️ One genuine original caller observes every rich cursor receipt, work event, partial owner and refusal cause.
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
use crate::{NativeSnapshotDecodeOwner,NativeSnapshotFailure};
use std::cell::Cell;
use super::{LegacyDecode,LegacyEncode,CloseNative,StepOwner};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn receiving<X>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<X,ValueError>)->Result<Option<X>,OriginalTurnFailure>{
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let mut out=None;let mut value=None;let mut inner=(0,0);
 let(ran,heap)=observe(||cx.with_native_receiving(&mut out,|native,_|{let(result,observed)=observe(||run(native));inner=(observed.requested_bytes,observed.released_bytes);value=Some(result?);Ok(())}));let ran=ran?;
 let((),dropped)=observe(||drop(out));
 cx.consume_retained(RetainedCloneProgress{retained_capacity_bytes:heap.requested_bytes-inner.0+dropped.requested_bytes,released_bytes:heap.released_bytes-inner.1+dropped.released_bytes,..Default::default()})?;
 Ok(if ran{Some(value.expect("funded receiving scope ran"))}else{None})
}
fn until_ran<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,mut turn:impl FnMut(&mut StepContext<'_>)->Result<Option<T>,OriginalTurnFailure>)->Result<Option<T>,OriginalTurnFailure>{
 for _ in 0..256{if let Some(value)=caller.refreshed_turn_with_decoder(grant,decoder,&mut turn)?{return Ok(Some(value))}}
 Ok(None)
}
fn step_close<T:semio_framework_value::retirement::RetireOwned>(close:&mut ControlledRetirement<T>,grant:RetainedCloneGrant)->RetainedWorkOutcome<semio_framework_value::RetainedCloneStep,ValueError>{
 use super::StepOwner;
 let result=close.step_with(grant).map_err(|failure|failure.primary().map_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original close failed without a primary cause"),|error|ValueError::new(error.kind,error.message.to_string())));
 RetainedWorkOutcome{result,work:close.last_turn_work()}
}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<T,NativeSnapshotFailure>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{
 let mut run=Some(run);
 for _ in 0..4096{
  let fixture=super::fixture_grant();
  let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.refreshed_turn_with_decoder(fixture,decoder,|cx|{
   let Some((outcome,progress))=receiving(cx,|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,grant);let outcome=(run.take().expect("receiving body runs once"))(&mut owner);Ok((outcome,owner.progress()))})?else{return Ok(None)};
   let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture receiver omitted its issued grant"))?;cx.consume_receiving_retained(issued,progress)?;Ok(Some(outcome))
  }));
  let receipt=caller.receipt();assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));
  if let Some(outcome)=result.unwrap(){return outcome}
 }
 panic!("original receiving scope never opened within its funded turns")
}
fn close<T:semio_framework_value::retirement::RetireOwned>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 let mut close=ControlledRetirement::retain(value);
 let fixture=super::fixture_grant();for index in 0..65536{if close.terminal_is_empty(){return}let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn_with_decoder(fixture,decoder,|cx|cx.with_original_retirement(|cx|{let d=close.selected_demand(fixture.maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|grant|{step_close(&mut close,grant)})?;Ok(())})));let p=caller.receipt();assert_eq!((born,freed),(p.retained_capacity_bytes,p.released_bytes));result.unwrap();assert!(index<65535);}
}
fn close_decoder(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant){
 for _ in 0..4096{if decoder.terminal_is_empty(){return}caller.turn(g,|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(g.maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();}assert!(decoder.terminal_is_empty());
}
#[test]
fn snapshot_primitive_original_loan_precedes_effect_and_preserves_failed_work(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("primitive.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(898),Generation(20),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=caller.admitted_decoder();let effects=Cell::new(0u64);
  let outcome=until_ran(&mut caller,&mut decoder,g,|cx|{
   let mut captured=None;cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),u64::MAX,|cx|{
   let Some(outcome)=receiving(cx,|native|Ok(crate::native_snapshot_turn(|meter|{
    meter.admit(native)?;effects.set(effects.get()+1);
    if row["fail"].as_bool().unwrap(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original primitive refusal").into())}Ok(())
   })))?else{return Ok(())};
   let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture receiver omitted its issued grant"))?;cx.consume_receiving_retained(issued,Default::default())?;captured=Some(outcome);Ok(())
   })?;Ok(captured)
  }).unwrap();
  let Some(outcome)=outcome else{assert_eq!((row["work"].as_u64().unwrap(),row["effects"].as_u64().unwrap(),row["accepted"].as_bool().unwrap()),(0,0,false));assert_eq!(effects.get(),0);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);continue};
  assert_eq!(outcome.work,u128::from(row["work"].as_u64().unwrap()));assert_eq!(effects.get(),row["effects"].as_u64().unwrap());assert_eq!(outcome.result.is_ok(),row["accepted"].as_bool().unwrap());close(outcome.result.err(),&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
 }
}
#[test]
fn snapshot_decode_frame_denied_work_preserves_original_cursor_and_backing(){
 for cursor_entry in [false,true]{
  let g=super::fixture_grant();let calls=Cell::new(0);let mut cursor=Some(());let mut observer=|_|true;let mut native=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(g.maximum_capacity_bytes,&mut observer);
  let mut work=semio_framework_value_testing_admission::finite_work(0,None);let mut physical=semio_framework_value_native_owned::native_decoding::OriginalConstructionAuthority::new(super::Open(g,Default::default()));
  let outcome=work.with_loan(&mut (),|wl|physical.with_loan(&mut (),|cl|native.with_work_recipient(wl,|native|native.with_construction_recipient(cl,|native|{
   let mut owner=NativeSnapshotDecodeOwner::new(native,g);
   let(outcome,born,freed)=semio_framework_trace_memory_testing::observe_backing(||if cursor_entry{owner.drive_legacy::<(),(),ValueError>(&mut cursor,|_,_,_|{calls.set(calls.get()+1);RetainedWorkOutcome{result:Ok((Some(()),Default::default())),work:0}})}else{owner.receive_legacy::<(),(),ValueError>(|_,_,_|{calls.set(calls.get()+1);RetainedWorkOutcome{result:Ok(()),work:0}})});
   assert_eq!((born,freed),(0,0));assert_eq!(owner.progress(),RetainedCloneProgress::default());Ok::<_,ValueError>(outcome)
  })))).unwrap().unwrap().unwrap();
  assert_eq!((calls.get(),outcome.work),(0,0));assert!(cursor.is_some());assert!(!native.has_retirement_owner());assert_eq!(outcome.result.as_ref().unwrap_err().kind(),ValueRefusalKind::WorkLimit);
  if let Some(failure)=outcome.result.err(){let mut close=ControlledRetirement::retain(failure);for _ in 0..4096{if close.terminal_is_empty(){break}close.step_with(g).unwrap();}assert!(close.terminal_is_empty());}
 }
}
#[test]
fn snapshot_cursor_rich_outcome_conserves_original_work_causes_and_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("fixture.json")).unwrap();let (_,positive)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let bytes=std::hint::black_box(vec![0u8;73]);std::hint::black_box(&bytes);drop(bytes)});assert_eq!((positive.requested_bytes,positive.released_bytes),(73,73));let mut saw_release_only=false;
 for row in fixture["cases"].as_array().unwrap(){
  let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(897),Generation(19),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=caller.admitted_decoder();
  let mut cursor=Some(vec![1u8,2,3,4,5,6,7]);let pointer=cursor.as_ref().unwrap().as_ptr();
  let last=row["steps"].as_array().unwrap().last().unwrap();let errors=last["errors"].as_array().unwrap();let error=|name:&str|errors.iter().any(|value|value==name).then(||ValueError::new(ValueRefusalKind::InvalidValue,format!("original {name}")));
  let mut original=(!errors.is_empty()).then(||OriginalReceivingPrimary::from_fields(error("primary"),error("quote"),error("allocation"),None,None,None,None,None,None,Default::default()));
  let pointers=original.as_ref().map(|failure|[failure.primary.as_ref(),failure.quote_refusal.as_ref(),failure.allocation_refusal.as_ref()].map(|error|error.map(|error|error.message.as_ptr())));
  let calls=Cell::new(0usize);let probes=Cell::new(0u64);let mut owner_grant=g;if !row["admitted"].as_bool().unwrap(){owner_grant.maximum_items=0}
  let outcome=turn(&mut caller,&mut decoder,owner_grant,|owner|owner.drive_legacy::<Vec<u8>,(),OriginalReceivingPrimary>(&mut cursor,|cursor,native,_|{
   assert_eq!(cursor.as_ptr(),pointer);let index=calls.get();calls.set(index+1);let event=&row["steps"][index];let units=event["work"].as_u64().unwrap();native.prepare_work(units).unwrap();for at in 0..units{std::hint::black_box(cursor[at as usize%cursor.len()]);probes.set(probes.get()+1)}native.commit_work(u128::from(units)).unwrap();
   let progress=RetainedCloneProgress{copied_items:event["items"].as_u64().unwrap()as usize,..Default::default()};
   let result=match event["result"].as_str().unwrap(){"pending"=>Ok((None,progress)),"ready"=>Ok((Some(()),progress)),"failed"=>{let error={let mut error=original.take().unwrap();error.progress=progress;error};Err(error)},_=>unreachable!()};RetainedWorkOutcome{result,work:u128::from(units)}
  }));
  assert_eq!(calls.get(),row["expected"]["calls"].as_u64().unwrap()as usize);assert_eq!(probes.get(),row["expected"]["producerWork"].as_u64().unwrap());assert!(outcome.work>=u128::from(probes.get()));
  if !row["admitted"].as_bool().unwrap(){assert_eq!(outcome.work,0);assert_eq!(cursor.as_ref().unwrap().as_ptr(),pointer);}
  if let Some(pointers)=pointers{let failure=outcome.result.as_ref().err().unwrap().receiving_primary().unwrap();assert_eq!([failure.primary.as_ref(),failure.quote_refusal.as_ref(),failure.allocation_refusal.as_ref()].map(|error|error.map(|error|error.message.as_ptr())),pointers);}
  assert_eq!(outcome.result.is_ok(),!row["expected"]["cursorRetained"].as_bool().unwrap());
  close((cursor,original,outcome.result.err()),&mut caller,&mut decoder);
  for index in 0..65536{if decoder.terminal_is_empty(){break}let d=decoder.retirement_demands(g.maximum_copy_bytes).unwrap();let release_only=d.release_bytes>0;let mut selected=g;let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(selected,|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})));let p=caller.receipt();assert_eq!((born,freed),(p.retained_capacity_bytes,p.released_bytes));result.unwrap();if release_only&&freed>0{assert!(p.copied_bytes>0);saw_release_only=true;}assert!(index<65535);}
  assert!(decoder.terminal_is_empty());caller.retire(g);
 }
 assert!(saw_release_only,"original decoder must physically release a terminal child under its own copy credit");
}

#[test]
fn snapshot_encoder_primitive_borrows_the_original_job_work_loan(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("primitive.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(896),Generation(18),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let effects=Cell::new(0u64);let mut observer=|_|true;let mut encoder=semio_framework_value::NativeEncodeControl::new(0,&mut observer);let fuel_before=caller.fuel_remaining();
  let outcome=caller.turn_with_decoder(g,&mut decoder,|cx|{
   let mut captured=None;cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),u64::MAX,|cx|{
    let outcome=cx.with_native_encoder_work::<_,ValueError>(&mut encoder,|native|Ok(crate::native_snapshot_turn(|meter|{
     meter.admit_encode(native)?;effects.set(effects.get()+1);
     if row["fail"].as_bool().unwrap(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original encoder primitive refusal").into())}Ok(())
    })))?;
    captured=Some(outcome);Ok(())
   })?;Ok(captured.unwrap())
  }).unwrap();
  assert_eq!(outcome.work,u128::from(row["work"].as_u64().unwrap()));assert_eq!(u128::from(fuel_before-caller.fuel_remaining()),outcome.work);assert_eq!(encoder.actual_work(),outcome.work);assert_eq!(effects.get(),row["effects"].as_u64().unwrap());assert_eq!(outcome.result.is_ok(),row["accepted"].as_bool().unwrap());close(outcome.result.err(),&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
 }
}

#[test]
fn original_job_encoder_keeps_the_actual_append_on_observer_refusal(){
 let f:serde_json::Value=serde_json::from_str(include_str!("encoder-append.json")).unwrap();let g=super::fixture_grant();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(895),Generation(17),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let text=f["text"].as_str().unwrap();let mut output=Vec::new();let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|event.completed==0;let mut native=semio_framework_value::NativeEncodeControl::new(g.maximum_capacity_bytes,&mut observer);
 let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||caller.turn_with_decoder(g,&mut decoder,|cx|{
  let mut captured=None;cx.with_original_work_loan(1,u64::MAX,|cx|{
   let outcome=cx.with_native_encoder_work::<_,ValueError>(&mut native,|native|Ok(crate::native_snapshot_turn(|work|work.measured_encode(native,|native|native.append_bytes(&mut output,text.as_bytes())))))?;
   let progress=RetainedCloneProgress{copied_items:1,copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};
   cx.consume_retained(progress)?;captured=Some(outcome);Ok(())
  })?;Ok(captured.unwrap())
 }));
 let outcome=result.unwrap();let receipt=caller.receipt();assert_eq!((born,released),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(receipt.copied_bytes,f["expectedCopiedBytes"].as_u64().unwrap()as usize);assert_eq!(outcome.work,u128::from(f["expectedWork"].as_u64().unwrap()));assert_eq!(native.actual_work(),outcome.work);assert_eq!(std::str::from_utf8(&output).unwrap(),text);let failure=outcome.result.unwrap_err();assert_eq!(failure.kind().as_str(),f["expectedKind"].as_str().unwrap());
 close(failure,&mut caller,&mut decoder);close(output,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
}

#[test]
fn original_job_recipient_gets_frame_before_body_refuses(){
 let f:serde_json::Value=serde_json::from_str(include_str!("frame-submission.json")).unwrap();let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(894),Generation(16),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=caller.admitted_decoder();let calls=Cell::new(0u64);
 let outcome=turn(&mut caller,&mut decoder,g,|owner|owner.receive_legacy::<(),(),ValueError>(|_,native,_|{
  calls.set(calls.get()+1);let submitted=native.construction_submitted().unwrap();assert_eq!(submitted.copied_items,f["copiedItems"].as_u64().unwrap() as usize);assert!(submitted.retained_capacity_bytes>0);
  RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"fixture body refused")),work:0}
 }));
 assert_eq!(outcome.work,u128::from(f["expectedWork"].as_u64().unwrap()));assert_eq!(calls.get(),f["callbackCalls"].as_u64().unwrap());let failure=outcome.result.unwrap_err();assert_eq!(failure.kind().as_str(),f["expectedKind"].as_str().unwrap());assert_eq!(caller.business_receipt().copied_items,f["copiedItems"].as_u64().unwrap() as usize);
 close(failure,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
}

#[test]
fn original_job_recipient_gets_actual_body_before_refusal(){
 let f:serde_json::Value=serde_json::from_str(include_str!("body-submission.json")).unwrap();let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(893),Generation(15),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=caller.admitted_decoder();let text=f["text"].as_str().unwrap();
 let outcome=turn(&mut caller,&mut decoder,g,|owner|owner.receive_legacy::<String,(),NativeSnapshotFailure>(|slot,native,body|crate::native_snapshot_turn(|work|{
  let bytes=size_of::<String>();let grant=body.admit_constructed(native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
  work.admit(native)?;*slot=Some(String::new());body.record_constructed_progress(native,grant,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
  work.admit(native)?;body.copy_text_receiving(native,text,slot.as_mut().unwrap(),1)?;
  let submitted=native.construction_submitted()?;assert_eq!(submitted.copied_items,f["copiedItems"].as_u64().unwrap()as usize);assert_eq!(submitted.copied_bytes,bytes+f["textBytes"].as_u64().unwrap()as usize);
  Err(ValueError::literal(ValueRefusalKind::InvalidValue,"fixture body retained after copy").into())
 })));
 assert_eq!(outcome.work,u128::from(f["work"].as_u64().unwrap()));let failure=outcome.result.unwrap_err();assert_eq!(failure.kind().as_str(),f["expectedKind"].as_str().unwrap());assert_eq!(caller.business_receipt().copied_items,f["copiedItems"].as_u64().unwrap()as usize);
 close(failure,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
}
