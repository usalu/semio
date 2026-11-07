//! 🫴️ Text Pack fields receive under one original caller; no destination or cancellation cursor is replaced.
use semio_s_artifact_stdio_semio_text_model::schema::snapshot::{SemioTextSnapshot,SemioTextRun,SemioTextMark,SemioTextMarkKind};
use semio_os_kernel::{os_store::OriginalSnapshotRecordReceiving,PackLimits};
use semio_framework_pack::snapshot::ArtifactPack;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,StepContext,OriginalTurnFailure,OriginalTurnPrimary};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetireOwned};
use semio_framework_value_native_owned::{OriginalReceivingPrimary,native_decoding::NativeDecodeControl};
use semio_framework_dsl_record::receiving::OwnedSlot;
use std::cell::Cell;

type Receiver=OriginalSnapshotRecordReceiving<SemioTextSnapshot>;
type Identity=Cell<Option<(u64,Option<u64>)>>;
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128}}
fn phase()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:128}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(87),Generation(4),stock(),||Some(0),u64::MAX,1000000)}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn input()->(Vec<u8>,serde_json::Value){
 let expected=fixture()["snapshot"].clone();
 let runs=expected["runs"].as_array().unwrap().iter().map(|run|SemioTextRun{
  language:run["language"].as_str().unwrap().into(),content:run["content"].as_str().unwrap().into(),
  marks:run["marks"].as_array().unwrap().iter().map(|mark|SemioTextMark{kind:match mark["kind"].as_str().unwrap(){"bold"=>SemioTextMarkKind::Bold,"italic"=>SemioTextMarkKind::Italic,"code"=>SemioTextMarkKind::Code,"link"=>SemioTextMarkKind::Link,_=>unreachable!()},href:mark["href"].as_str().unwrap().into()}).collect()
 }).collect();
 let snapshot=SemioTextSnapshot{schema:expected["schema"].as_str().unwrap().into(),runs};
 (snapshot.encode_pack(),expected)
}
fn receiving(cx:&mut StepContext<'_>,identity:&Identity,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>)->Result<RetainedCloneProgress,OriginalTurnFailure>{
 let grant=cx.retained_grant();let mut work=0;
 let result=cx.with_native_receiving::<_,OriginalReceivingPrimary>(|native|{
  let actual=(native.ledger_identity(),native.retirement_recipient_identity());assert!(actual.1.is_some());
  if let Some(previous)=identity.get(){assert_eq!(actual,previous)}else{identity.set(Some(actual))}
  let turn=run(native,grant);work=turn.work;turn.result
 });
 let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
 let receipt=cx.consume_receiving_retained(grant,progress);let fuel=cx.fuel_remaining();cx.consume_fuel(work);assert_eq!(fuel-cx.fuel_remaining(),work);
 match result{Ok(progress)=>{receipt?;Ok(progress)},Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal:receipt.err(),context_refusal:None,work_refusal:None})}
}
fn measured<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{
 let(original,result)= (caller.stock(),semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,run)));
 let(value,heap)=result;
 let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
 assert_eq!(caller.stock(),original);assert!(caller.cumulative_receipt().fits(stock()));value.unwrap()
}
fn admit(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,identity:&Identity,source:&[u8])->Receiver{
 let mut output=None;
 measured(caller,decoder,phase(),|cx|receiving(cx,identity,|native,grant|{
  let admitted=Receiver::admit(source,"stdio.semio.text",PackLimits::default(),native,grant);
  let result=admitted.map(|candidate|if let Some((owner,progress))=candidate{output=Some(owner);RetainedCloneStep::Progress(progress)}else{RetainedCloneStep::Progress(Default::default())});
  let work=if output.is_some(){1}else{0};
  let posted=native.receive_retained_work(grant,RetainedWorkOutcome{result,work});
  RetainedWorkOutcome{result:posted.result.map(|step|step.progress()),work:posted.work}
 }));
 output.expect("original Text receiver admitted")
}
fn advance(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,identity:&Identity,source:&[u8],owner:&mut Receiver,grant:RetainedCloneGrant)->RetainedCloneProgress{
 measured(caller,decoder,grant,|cx|receiving(cx,identity,|native,grant|owner.step(source,native,4096,grant)))
}
fn close_slot<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,slot:&mut OwnedSlot<T>){
 let mut turns=0;
 while !slot.terminal_is_empty(){
  let demand=slot.close_demands(4096).unwrap();
  if demand.release_bytes>0{let mut denied=phase();denied.maximum_release_bytes=0;let refused=measured(caller,decoder,denied,|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|{let turn=slot.close_step(grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})));assert_eq!(refused.progress(),RetainedCloneProgress::default());assert_eq!(slot.close_demands(4096).unwrap(),demand);}
  measured(caller,decoder,phase(),|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|{
   let turn=slot.close_step(grant);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
  })));
  turns+=1;assert!(turns<100000);
 }
}
fn close_receiver(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,owner:&mut Receiver){
 let mut turns=0;
 while !owner.terminal_is_empty(){
  let demand=owner.close_demands(4096).unwrap();
  measured(caller,decoder,phase(),|cx|cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,demand,|grant|{
   let step=semio_framework_value::retirement::RetirementCursor::close_step(owner,grant);let work=step.work();
   let result=match step{semio_framework_value::retirement::RetirementStep::Progress(progress,_)=>Ok(RetainedCloneStep::Progress(progress)),semio_framework_value::retirement::RetirementStep::Failure(error,_)=>Err(error),_=>panic!("snapshot receiver returns an explicit physical retirement step")};
   RetainedWorkOutcome{result,work}
  })));
  turns+=1;assert!(turns<100000);
 }
}
fn finish(mut caller:OriginalJobFixtureCaller,mut decoder:OriginalJobDecodeOwner){
 let mut turns=0;
 while !decoder.terminal_is_empty(){
  let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(phase(),|cx|cx.with_original_retirement(|cx|{
   let turn=decoder.close_step(cx.retained_grant());let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
   let receipt=cx.consume_retained(progress);cx.consume_fuel(turn.work);
   match turn.result{Ok(_)=>{receipt?;Ok(())},Err(error)=>{let mut failure=OriginalTurnFailure::from(error);failure.receipt_refusal=receipt.err();Err(failure)}}
  })));
  result.unwrap();let receipt=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
  turns+=1;assert!(turns<100000);
 }
 caller.retire(phase());
}
#[test]
fn original_text_pack_receives_all_nested_fields_with_exact_original_receipts(){
 let(source,expected)=input();let pointer=source.as_ptr();let extent=source.len();
 let mut original=caller();let mut decoder=OriginalJobDecodeOwner::empty();let identity=Identity::new(None);
 let mut owner=admit(&mut original,&mut decoder,&identity,&source);let mut turns=0;
 while !owner.is_ready(){advance(&mut original,&mut decoder,&identity,&source,&mut owner,phase());assert_eq!((source.as_ptr(),source.len()),(pointer,extent));turns+=1;assert!(turns<100000);}
 let mut output=OwnedSlot::<SemioTextSnapshot>::default();
 measured(&mut original,&mut decoder,phase(),|cx|receiving(cx,&identity,|native,grant|{
  let result=owner.take_into(&mut output.value,grant).map(RetainedCloneStep::Progress);
  let turn=native.receive_retained_work(grant,RetainedWorkOutcome{result,work:1});RetainedWorkOutcome{result:turn.result.map(|step|step.progress()),work:turn.work}
 }));
 let actual:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(output.value.as_ref().unwrap())).unwrap();
 assert_eq!(actual,expected);assert!(turns>1);
 original.cancel().cancel_now();close_receiver(&mut original,&mut decoder,&mut owner);close_slot(&mut original,&mut decoder,&mut output);finish(original,decoder);
}
#[test]
fn original_text_pack_partial_frontiers_cancel_and_close_under_the_same_stock(){
 let(source,_)=input();
 for prefix in fixture()["cancelAfterTurns"].as_array().unwrap(){
  let mut original=caller();let mut decoder=OriginalJobDecodeOwner::empty();let identity=Identity::new(None);
  let mut owner=admit(&mut original,&mut decoder,&identity,&source);
  for _ in 0..prefix.as_u64().unwrap(){if owner.is_ready(){break}advance(&mut original,&mut decoder,&identity,&source,&mut owner,phase());}
  original.cancel().cancel_now();close_receiver(&mut original,&mut decoder,&mut owner);finish(original,decoder);
 }
}
#[test]
fn original_text_pack_denies_only_unfunded_selected_frontiers_without_progress(){
 let(source,_)=input();let mut original=caller();let mut decoder=OriginalJobDecodeOwner::empty();let identity=Identity::new(None);
 let mut owner=admit(&mut original,&mut decoder,&identity,&source);let mut seen=[false;5];let mut turns=0;
 while !owner.is_ready(){
  let demand=owner.demands(&source,4096).unwrap();
  for (axis,positive) in [true,demand.copy_bytes>0,demand.capacity_bytes>0,demand.release_bytes>0,demand.depth>0].into_iter().enumerate(){
   if !positive{continue}let mut denied=phase();match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=0,2=>denied.maximum_capacity_bytes=0,3=>denied.maximum_release_bytes=0,4=>denied.maximum_depth=0,_=>unreachable!()}
   let receipt=advance(&mut original,&mut decoder,&identity,&source,&mut owner,denied);
   assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!(owner.demands(&source,4096).unwrap(),demand);seen[axis]=true;
  }
  advance(&mut original,&mut decoder,&identity,&source,&mut owner,phase());turns+=1;assert!(turns<100000);
 }
 assert!(seen[0]&&seen[1]&&seen[2]&&seen[4]);
 original.cancel().cancel_now();close_receiver(&mut original,&mut decoder,&mut owner);finish(original,decoder);
}
