//! 🧪️ One genuine original caller observes every rich cursor receipt, work event, partial owner and refusal cause.
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
use crate::{NativeSnapshotDecodeOwner,NativeSnapshotFailure};
use std::cell::Cell;
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<T,NativeSnapshotFailure>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{
 let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn_with_decoder(super::fixture_grant(),decoder,|cx|{
  let(outcome,progress)=cx.with_native_receiving::<_,ValueError>(|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,grant);let outcome=run(&mut owner);Ok((outcome,owner.progress()))})?;
  let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture receiver omitted its issued grant"))?;cx.consume_receiving_retained(issued,progress)?;cx.consume_fuel(outcome.work);Ok(outcome)
 }));
 let receipt=caller.receipt();assert_eq!((born,freed),(receipt.retained_capacity_bytes,receipt.released_bytes));
 result.unwrap()
}
fn close<T:semio_framework_value::retirement::RetireOwned>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 let mut close=ControlledRetirement::retain(value);
 for index in 0..65536{if close.terminal_is_empty(){return}let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn_with_decoder(super::fixture_grant(),decoder,|cx|cx.with_original_retirement(|cx|{let d=close.selected_demand(super::fixture_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|grant|{let result=close.step(grant);RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())})));let p=caller.receipt();assert_eq!((born,freed),(p.retained_capacity_bytes,p.released_bytes));result.unwrap();assert!(index<65535);}
}
fn close_decoder(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant){
 for _ in 0..4096{if decoder.terminal_is_empty(){return}caller.turn(g,|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(g.maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}assert!(decoder.terminal_is_empty());
}
#[test]
fn snapshot_primitive_original_loan_precedes_effect_and_preserves_failed_work(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("primitive.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(898),Generation(20),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let effects=Cell::new(0u64);
  let outcome=caller.turn_with_decoder(g,&mut decoder,|cx|{
   let mut captured=None;cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),u64::MAX,|cx|{
   let outcome=cx.with_native_receiving::<_,ValueError>(|native|Ok(crate::native_snapshot_turn(|meter|{
    meter.admit(native)?;effects.set(effects.get()+1);
    if row["fail"].as_bool().unwrap(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original primitive refusal").into())}Ok(())
   })))?;
   let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture receiver omitted its issued grant"))?;cx.consume_receiving_retained(issued,Default::default())?;cx.consume_fuel(outcome.work);captured=Some(outcome);Ok(())
   })?;Ok(captured.unwrap())
  }).unwrap();
  assert_eq!(outcome.work,row["work"].as_u64().unwrap());assert_eq!(effects.get(),row["effects"].as_u64().unwrap());assert_eq!(outcome.result.is_ok(),row["accepted"].as_bool().unwrap());close(outcome.result.err(),&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
 }
}
#[test]
fn snapshot_decode_frame_denied_work_preserves_original_cursor_and_backing(){
 for cursor_entry in [false,true]{
  let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(899),Generation(21),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let calls=Cell::new(0);let mut cursor=Some(());
  let outcome=caller.turn_with_decoder(g,&mut decoder,|cx|{
   let mut captured=None;cx.with_original_work_loan(0,u64::MAX,|cx|{let(outcome,progress)=cx.with_native_receiving::<_,ValueError>(|native|{
    let mut owner=NativeSnapshotDecodeOwner::new(native,g);
    let(outcome,born,freed)=semio_framework_trace_memory_testing::observe_backing(||if cursor_entry{owner.drive_cursor::<(),(),ValueError>(&mut cursor,&mut |_,_,_|{calls.set(calls.get()+1);RetainedWorkOutcome{result:Ok((Some(()),Default::default())),work:0}})}else{owner.receive::<(),(),ValueError>(&mut |_,_,_|{calls.set(calls.get()+1);RetainedWorkOutcome{result:Ok(()),work:0}})});
    assert_eq!((born,freed),(0,0));Ok((outcome,owner.progress()))
   })?;let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original fixture receiver omitted its issued grant"))?;cx.consume_receiving_retained(issued,progress)?;captured=Some(outcome);Ok(())})?;Ok(captured.unwrap())
  }).unwrap();
  assert_eq!((calls.get(),outcome.work),(0,0));assert!(cursor.is_some());assert_eq!(outcome.result.as_ref().unwrap_err().kind(),ValueRefusalKind::WorkLimit);close(outcome.result.err(),&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
 }
}
#[test]
fn snapshot_cursor_rich_outcome_conserves_original_work_causes_and_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("fixture.json")).unwrap();let (_,positive)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let bytes=std::hint::black_box(vec![0u8;73]);std::hint::black_box(&bytes);drop(bytes)});assert_eq!((positive.requested_bytes,positive.released_bytes),(73,73));let mut saw_release_only=false;let frame_bytes=std::mem::size_of::<crate::snapshot::NativeSnapshotReceiving<Vec<u8>,()>>();
 for row in fixture["cases"].as_array().unwrap(){
  let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(897),Generation(19),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();
  let mut cursor=Some(vec![1u8,2,3,4,5,6,7]);let pointer=cursor.as_ref().unwrap().as_ptr();
  let last=row["steps"].as_array().unwrap().last().unwrap();let errors=last["errors"].as_array().unwrap();let error=|name:&str|errors.iter().any(|value|value==name).then(||ValueError::new(ValueRefusalKind::InvalidValue,format!("original {name}")));
  let mut original=(!errors.is_empty()).then(||OriginalReceivingPrimary{retirement_failure:None,work_refusal:None,work_overrun:None,primary:error("primary"),quote_refusal:error("quote"),allocation_refusal:error("allocation"),allocation_failure:None,native_allocation:None,receipt_refusal:None,progress:Default::default()});
  let pointers=original.as_ref().map(|failure|[failure.primary.as_ref(),failure.quote_refusal.as_ref(),failure.allocation_refusal.as_ref()].map(|error|error.map(|error|error.message.as_ptr())));
  let calls=Cell::new(0usize);let probes=Cell::new(0u64);let mut owner_grant=g;if !row["admitted"].as_bool().unwrap(){owner_grant.maximum_items=0}
  let outcome=turn(&mut caller,&mut decoder,owner_grant,|owner|owner.drive_cursor::<Vec<u8>,(),OriginalReceivingPrimary>(&mut cursor,&mut |cursor,native,_|{
   assert_eq!(cursor.as_ptr(),pointer);let index=calls.get();calls.set(index+1);let event=&row["steps"][index];let units=event["work"].as_u64().unwrap();native.prepare_work(units).unwrap();for at in 0..units{std::hint::black_box(cursor[at as usize%cursor.len()]);probes.set(probes.get()+1)}native.commit_work(units).unwrap();
   let progress=RetainedCloneProgress{copied_items:event["items"].as_u64().unwrap()as usize,..Default::default()};
   let result=match event["result"].as_str().unwrap(){"pending"=>Ok((None,progress)),"ready"=>Ok((Some(()),progress)),"failed"=>{let error=original.take().unwrap().with_retained_progress(progress);Err(error)},_=>unreachable!()};RetainedWorkOutcome{result,work:units}
  }));
  assert_eq!(calls.get(),row["expected"]["calls"].as_u64().unwrap()as usize);assert_eq!(probes.get(),row["expected"]["producerWork"].as_u64().unwrap());assert!(outcome.work>=probes.get());
  if !row["admitted"].as_bool().unwrap(){assert_eq!(outcome.work,0);assert_eq!(cursor.as_ref().unwrap().as_ptr(),pointer);}
  if let Some(pointers)=pointers{let failure=outcome.result.as_ref().err().unwrap().receiving_primary().unwrap();assert_eq!([failure.primary.as_ref(),failure.quote_refusal.as_ref(),failure.allocation_refusal.as_ref()].map(|error|error.map(|error|error.message.as_ptr())),pointers);}
  assert_eq!(outcome.result.is_ok(),!row["expected"]["cursorRetained"].as_bool().unwrap());
  close((cursor,original,outcome.result.err()),&mut caller,&mut decoder);
  for index in 0..65536{if decoder.terminal_is_empty(){break}let d=decoder.retirement_demands(g.maximum_copy_bytes).unwrap();let release_only=d.copy_bytes==0&&d.release_bytes>0;let mut selected=g;if release_only{selected.maximum_copy_bytes=0;}let(result,born,freed)=semio_framework_trace_memory_testing::observe_backing(||caller.turn(selected,|cx|cx.with_original_retirement(|cx|{semio_framework_job::close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})));let p=caller.receipt();assert_eq!((born,freed),(p.retained_capacity_bytes,p.released_bytes));result.unwrap();if release_only&&freed==frame_bytes{assert_eq!(p.copied_bytes,0);saw_release_only=true;}assert!(index<65535);}
  assert!(decoder.terminal_is_empty());caller.retire(g);
 }
 assert!(saw_release_only,"original decoder must physically release a terminal child without copy credit");
}

#[test]
fn snapshot_encoder_primitive_borrows_the_original_job_work_loan(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("primitive.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(896),Generation(18),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let effects=Cell::new(0u64);let mut observer=|_|true;let mut encoder=semio_framework_value::NativeEncodeControl::new(0,&mut observer);let fuel_before=caller.fuel_remaining();
  let outcome=caller.turn_with_decoder(g,&mut decoder,|cx|{
   let mut captured=None;cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),u64::MAX,|cx|{
    let outcome=cx.with_native_encoder_work::<_,ValueError>(&mut encoder,|native|Ok(crate::native_snapshot_turn(|meter|{
     meter.admit_encode(native)?;effects.set(effects.get()+1);
     if row["fail"].as_bool().unwrap(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original encoder primitive refusal").into())}Ok(())
    })))?;
    captured=Some(outcome);Ok(())
   })?;Ok(captured.unwrap())
  }).unwrap();
  assert_eq!(outcome.work,row["work"].as_u64().unwrap());assert_eq!(fuel_before-caller.fuel_remaining(),outcome.work);assert_eq!(encoder.actual_work(),outcome.work);assert_eq!(effects.get(),row["effects"].as_u64().unwrap());assert_eq!(outcome.result.is_ok(),row["accepted"].as_bool().unwrap());close(outcome.result.err(),&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
 }
}

#[test]
fn original_job_encoder_keeps_the_actual_append_on_observer_refusal(){
 let f:serde_json::Value=serde_json::from_str(include_str!("encoder-append.json")).unwrap();let g=super::fixture_grant();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(895),Generation(17),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let text=f["text"].as_str().unwrap();let mut output=Vec::new();let mut observer=|event:semio_framework_value::native_encoding::NativeEncodeProgress|event.completed==0;let mut native=semio_framework_value::NativeEncodeControl::new(g.maximum_capacity_bytes,&mut observer);
 let(result,born,released)=semio_framework_trace_memory_testing::observe_backing(||caller.turn_with_decoder(g,&mut decoder,|cx|{
  let mut captured=None;cx.with_original_work_loan(1,u64::MAX,|cx|{
   let outcome=cx.with_native_encoder_work::<_,ValueError>(&mut native,|native|Ok(crate::native_snapshot_turn(|work|work.measured_encode(native,|native|native.append_bytes(&mut output,text.as_bytes())))))?;
   let progress=RetainedCloneProgress{copied_items:1,copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};
   cx.consume_retained(progress)?;captured=Some(outcome);Ok(())
  })?;Ok(captured.unwrap())
 }));
 let outcome=result.unwrap();let receipt=caller.receipt();assert_eq!((born,released),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(receipt.copied_bytes,f["expectedCopiedBytes"].as_u64().unwrap()as usize);assert_eq!(outcome.work,f["expectedWork"].as_u64().unwrap());assert_eq!(native.actual_work(),outcome.work);assert_eq!(std::str::from_utf8(&output).unwrap(),text);let failure=outcome.result.unwrap_err();assert_eq!(failure.kind().as_str(),f["expectedKind"].as_str().unwrap());
 close(failure,&mut caller,&mut decoder);close(output,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
}

#[test]
fn original_job_recipient_gets_frame_before_body_refuses(){
 let f:serde_json::Value=serde_json::from_str(include_str!("frame-submission.json")).unwrap();let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(894),Generation(16),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let calls=Cell::new(0u64);
 let outcome=turn(&mut caller,&mut decoder,g,|owner|owner.receive::<(),(),ValueError>(&mut |_,native,_|{
  calls.set(calls.get()+1);let submitted=native.construction_submitted().unwrap();assert_eq!(submitted.copied_items,f["copiedItems"].as_u64().unwrap() as usize);assert!(submitted.retained_capacity_bytes>0);
  RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"fixture body refused")),work:0}
 }));
 assert_eq!(outcome.work,f["expectedWork"].as_u64().unwrap());assert_eq!(calls.get(),f["callbackCalls"].as_u64().unwrap());let failure=outcome.result.unwrap_err();assert_eq!(failure.kind().as_str(),f["expectedKind"].as_str().unwrap());assert_eq!(caller.receipt().copied_items,f["copiedItems"].as_u64().unwrap() as usize);
 close(failure,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
}

#[test]
fn original_job_recipient_gets_actual_body_before_refusal(){
 let f:serde_json::Value=serde_json::from_str(include_str!("body-submission.json")).unwrap();let g=super::fixture_grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(893),Generation(15),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let text=f["text"].as_str().unwrap();
 let outcome=turn(&mut caller,&mut decoder,g,|owner|owner.receive::<String,(),NativeSnapshotFailure>(&mut |slot,native,body|crate::native_snapshot_turn(|work|{
  let bytes=size_of::<String>();let grant=body.admit_constructed(native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:1,..Default::default()})?;
  work.admit(native)?;*slot=Some(String::new());body.record_constructed_progress(native,grant,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
  work.admit(native)?;body.copy_text_receiving(native,text,slot.as_mut().unwrap(),1)?;
  let submitted=native.construction_submitted()?;assert_eq!(submitted.copied_items,f["copiedItems"].as_u64().unwrap()as usize);assert_eq!(submitted.copied_bytes,bytes+f["textBytes"].as_u64().unwrap()as usize);
  Err(ValueError::literal(ValueRefusalKind::InvalidValue,"fixture body retained after copy").into())
 })));
 assert_eq!(outcome.work,f["work"].as_u64().unwrap());let failure=outcome.result.unwrap_err();assert_eq!(failure.kind().as_str(),f["expectedKind"].as_str().unwrap());assert_eq!(caller.receipt().copied_items,f["copiedItems"].as_u64().unwrap()as usize);
 close(failure,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder,g);caller.retire(g);
}
