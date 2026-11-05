#[cfg(test)]
#[global_allocator]
static ORIGINAL_PAGE_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

use semio_framework_plugin_result_page::pending_result_page::*;
use semio_framework_actor::typed_operation_page::{TypedOperationResultToken,TypedOperationResultLane};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_value::{RetainedCloneGrant,RetainedWorkOutcome,RetirementDemand,ValueError};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,StepContext,OriginalTurnFailure,OriginalConstructionDemand,OperationId,Generation,admit_original_construction,close_original_retained_step};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn demand(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(44781),Generation(8),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096},||Some(1),u64::MAX,1<<20)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(g));result.unwrap()
}
fn capture(plan:PublicationPagePlan,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner)->PendingPublicationPage{
 let d=PendingPublicationPage::capture_demands();let g=grant(d);let mut owner=None;
 turn(caller,decoder,g,|cx|{assert!(admit_original_construction(cx,&mut owner,demand(d),|_|{let step=PendingPublicationPage::capture(plan,g);RetainedWorkOutcome{work:step.work,result:step.result.map(|owner|owner.expect("quoted pending page capture"))}})?);Ok(())});owner.unwrap()
}
fn initialize(owner:&mut PendingPublicationPage,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for _ in 0..65536{
  if owner.backing_ready(){return}
  let d=owner.backing_demands(32768).unwrap();let g=grant(d);let mut done=None;
  turn(caller,decoder,g,|cx|{assert!(admit_original_construction(cx,&mut done,demand(d),|_|{let step=owner.advance_backing(g);RetainedWorkOutcome{work:step.work,result:step.result.map(|step|((),step.progress()))}})?);Ok(())});
 }
 panic!("original pending backing did not initialize")
}
fn close(owner:&mut PendingPublicationPage,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 for _ in 0..65536{
  if owner.terminal_is_empty(){return}
  let d=owner.close_demands(32768).unwrap();let g=grant(d);
  turn(caller,decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|owner.close_step(issued))?;Ok(())}));
 }
 panic!("pending page retained its original frontier")
}
fn retire(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096};
 for _ in 0..65536{
  if decoder.terminal_is_empty(){break}
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(g,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(g.maximum_copy_bytes)?;close_original_retained_step(cx,d,|issued|decoder.close_step(issued))?;Ok(())})));
  let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap();
 }
 assert!(decoder.terminal_is_empty());caller.retire(g);
}
#[test]
fn pending_original_page_preserves_accepted_intent_token_and_canonical_bytes_under_original_caller(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/📃️page-admission/🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  if row["expected"]["state"]["frame"].is_null()||row["cancelPayload"].is_object(){continue}
  let value=&row["payload"]["value"];let expected=row["expected"]["state"]["frame"].as_str().unwrap();let token=TypedOperationResultToken{receiver:7,operation:11,generation:3,sequence:2,attempt:1};
  let ui=if row["payload"]["kind"]=="ui"{Some(serde_json::from_value::<UiDirtyScope>(value.clone()).unwrap())}else{None};
  let (intent,source)=match row["payload"]["kind"].as_str().unwrap(){
   "laneReceipt"=>(PublicationPageIntent::LaneReceipt{lane:TypedOperationResultLane::Artifact,generation_before:value["generationBefore"].as_u64().unwrap(),generation_after:value["generationAfter"].as_u64().unwrap()},PublicationPageSource::None),
   "accepted"=>(PublicationPageIntent::Accepted{lane:TypedOperationResultLane::Interaction,count:value[1].as_u64().unwrap()},PublicationPageSource::None),
   "ui"=>(PublicationPageIntent::Ui,PublicationPageSource::Ui(ui.as_ref().unwrap())),
   "download"=>(PublicationPageIntent::Download,PublicationPageSource::Download{filename:value[0].as_str().unwrap(),mime:value[1].as_str().unwrap(),encoding:value[2].as_str(),bytes:value[3].as_u64().unwrap()}),
   "fault"=>(PublicationPageIntent::Raw{lane:TypedOperationResultLane::Fault,reason:PublicationRawReason::Fault},PublicationPageSource::Raw{reason:PublicationRawReason::Fault,bytes:value.as_str().unwrap().as_bytes()}),
   "terminal"=>(PublicationPageIntent::Raw{lane:TypedOperationResultLane::Terminal,reason:PublicationRawReason::Complete},PublicationPageSource::Raw{reason:PublicationRawReason::Complete,bytes:value.as_str().unwrap().as_bytes()}),
   _=>panic!("unknown finite result family"),
  };
  let plan=PublicationPagePlan{token,intent};let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=capture(plan,&mut caller,&mut decoder);initialize(&mut owner,&mut caller,&mut decoder);
  for _ in 0..65536{
   if owner.is_framed(){break}
   let d=owner.write_demands(32768).unwrap();let g=grant(d);let before=owner.written_bytes();
   for denied in[RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{
    turn(&mut caller,&mut decoder,g,|cx|{cx.with_native_receiving(|decode|{let step=owner.write_step(&source,denied,1,decode);assert!(step.result.is_ok());assert_eq!(step.progress,Default::default());assert_eq!(step.work,0);Ok::<(),ValueError>(())})?;Ok(())});assert_eq!(owner.plan(),Some(plan));assert_eq!(owner.written_bytes(),before);
   }
   let mut complete=None;
   turn(&mut caller,&mut decoder,g,|cx|{assert!(admit_original_construction(cx,&mut complete,demand(d),|decode|{
    let step=owner.write_step(&source,g,1,decode);let result=match step.result{Ok(complete)=>Ok((complete,step.progress)),Err(failure)=>Err(failure.primary.or(failure.forwarding).unwrap().with_retained_progress(step.progress))};RetainedWorkOutcome{work:step.work,result}
   })?);Ok(())});assert_eq!(owner.plan(),Some(plan));
  }
  assert!(owner.is_framed());assert_eq!(owner.written_bytes(),expected.len());
  for _ in 0..65536{
   if owner.ready_to_handoff(){break}
   let d=owner.traversal_close_demands(32768).unwrap();let g=grant(d);turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|owner.close_traversal(issued))?;Ok(())}));
  }
  assert!(owner.ready_to_handoff());let d=owner.handoff_demands().unwrap();let g=grant(d);
  let denied=RetainedCloneGrant{maximum_copy_bytes:0,..g};let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.take_page(denied));assert!(step.result.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.plan(),Some(plan));
  let mut page=None;turn(&mut caller,&mut decoder,g,|cx|{assert!(admit_original_construction(cx,&mut page,demand(d),|_|{let step=owner.take_page(g);RetainedWorkOutcome{work:step.work,result:step.result.map(|page|page.expect("quoted original result handoff"))}})?);Ok(())});
  let mut page=page.unwrap();assert_eq!(page.token,token);assert_eq!(page.bytes(),expected.as_bytes());assert!(owner.terminal_is_empty());
  for _ in 0..65536{if page.terminal_is_empty(){break}let d=page.next_close_demand(32768).unwrap();let g=grant(d);turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|RetainedWorkOutcome{result:page.close_step(issued),work:1})?;Ok(())}));}
  assert!(page.terminal_is_empty());retire(caller,&mut decoder);
 }
}
#[test]
fn pending_original_page_cancelled_partial_frontier_retains_plan_until_paid_close(){
 for cut in[0,1,4,8,17]{
  let token=TypedOperationResultToken{receiver:7,operation:11,generation:3,sequence:2,attempt:1};let plan=PublicationPagePlan{token,intent:PublicationPageIntent::Accepted{lane:TypedOperationResultLane::Interaction,count:3}};
  let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=capture(plan,&mut caller,&mut decoder);initialize(&mut owner,&mut caller,&mut decoder);
  for _ in 0..cut{
   if owner.is_framed(){break}
   let d=owner.write_demands(32768).unwrap();let g=grant(d);let mut done=None;
   turn(&mut caller,&mut decoder,g,|cx|{admit_original_construction(cx,&mut done,demand(d),|decode|{let step=owner.write_step(&PublicationPageSource::None,g,1,decode);let result=match step.result{Ok(complete)=>Ok((complete,step.progress)),Err(error)=>Err(error.primary.or(error.forwarding).unwrap().with_retained_progress(step.progress))};RetainedWorkOutcome{work:step.work,result}})?;Ok(())});
  }
  assert_eq!(owner.plan(),Some(plan));caller.cancel().cancel_now();close(&mut owner,&mut caller,&mut decoder);assert!(owner.terminal_is_empty());retire(caller,&mut decoder);
 }
}

#[test]
fn pending_original_page_oversized_json_seals_its_original_failure_and_paid_frontier(){
 let token=TypedOperationResultToken{receiver:7,operation:11,generation:3,sequence:2,attempt:1};
 let plan=PublicationPagePlan{token,intent:PublicationPageIntent::Download};let filename="x".repeat(8192);
 let source=PublicationPageSource::Download{filename:&filename,mime:"text/plain",encoding:None,bytes:0};
 let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=capture(plan,&mut caller,&mut decoder);initialize(&mut owner,&mut caller,&mut decoder);
 let mut failure=None;
 for _ in 0..65536{
  let mut d=owner.write_demands(32768).unwrap();let frame=2*std::mem::size_of::<Result<bool,PublicationPageWriteFailure>>();d.copy_bytes+=frame;let g=grant(d);let mut output=None;
  turn(&mut caller,&mut decoder,g,|cx|{
   assert!(admit_original_construction(cx,&mut output,demand(d),|decode|{
    let step=owner.write_step(&source,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-frame,..g},1,decode);
    let mut progress=step.progress;progress.copied_bytes+=frame;
    RetainedWorkOutcome{work:step.work,result:Ok((step.result,progress))}
   })?);Ok(())
  });
  match output.unwrap(){Ok(_)=>{},Err(error)=>{failure=Some(error);break}}
 }
 assert_eq!(owner.written_bytes(),4096);assert_eq!(owner.plan(),Some(plan));assert!(owner.write_refused());assert!(!owner.ready_to_handoff());
 let mut original=failure.expect("oversized immutable source must preserve its framing refusal");assert!(original.primary.is_some()||original.forwarding.is_some());
 close(&mut owner,&mut caller,&mut decoder);
 type ErrorClose=semio_framework_value::retirement::controlled::ControlledRetirement<PublicationPageWriteFailure>;
 let mut original=Some(original);let mut retirement=None;
 let d=RetirementDemand{copy_bytes:2*std::mem::size_of::<Option<PublicationPageWriteFailure>>()+2*std::mem::size_of::<Option<ErrorClose>>(),depth:1,..Default::default()};let g=grant(d);
 turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{
  close_original_retained_step(cx,d,|_|{retirement=Some(ErrorClose::retain(original.take().unwrap()));RetainedWorkOutcome{work:1,result:Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}})?;Ok(())
 }));
 let mut retirement=retirement.unwrap();
 for _ in 0..65536{if retirement.terminal_is_empty(){break}let d=retirement.next_demand(32768).unwrap();let g=grant(d);turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|{let result=retirement.step(issued);RetainedWorkOutcome{work:retirement.last_turn_work(),result}})?;Ok(())}));}
 assert!(retirement.terminal_is_empty());retire(caller,&mut decoder);
}

#[test]
fn pending_original_page_delivers_accepted_receipt_after_original_cancellation(){
 for cut in[0,1,4]{
  let token=TypedOperationResultToken{receiver:7,operation:11,generation:8,sequence:2,attempt:1};let plan=PublicationPagePlan{token,intent:PublicationPageIntent::LaneReceipt{lane:TypedOperationResultLane::Artifact,generation_before:7,generation_after:8}};
  let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=capture(plan,&mut caller,&mut decoder);initialize(&mut owner,&mut caller,&mut decoder);
  for _ in 0..cut{
   if owner.is_framed(){break}let d=owner.write_demands(32768).unwrap();let g=grant(d);let mut output=None;
   turn(&mut caller,&mut decoder,g,|cx|{assert!(admit_original_construction(cx,&mut output,demand(d),|decode|{let step=owner.write_step(&PublicationPageSource::None,g,1,decode);let done=step.result.unwrap_or_else(|_|panic!("accepted original framing refused before cancellation"));RetainedWorkOutcome{work:step.work,result:Ok((done,step.progress))}})?);Ok(())});
  }
  caller.cancel().cancel_now();let decoder_identity=decoder.receiving_identity();assert!(decoder_identity.is_some());
  for _ in 0..65536{
   if owner.is_framed(){break}let d=owner.write_demands(32768).unwrap();let g=grant(d);
   turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{
    assert!(cx.is_cancelled());assert!(cx.retained_work_can_enter(d));let mut actual=None;
    cx.with_native_retirement(|original|{actual=Some(owner.write_retirement_step(&PublicationPageSource::None,g,1,original));Ok(())})?;
    let step=actual.unwrap();assert!(step.result.is_ok());assert!(step.work<=1);cx.consume_retained(step.progress)?;cx.consume_fuel(step.work);Ok(())
   }));assert_eq!(owner.plan(),Some(plan));assert_eq!(decoder.receiving_identity(),decoder_identity);
  }
  assert!(owner.is_framed());assert!(!owner.write_refused());
  for _ in 0..65536{if owner.ready_to_handoff(){break}let d=owner.traversal_close_demands(32768).unwrap();let g=grant(d);turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|owner.close_traversal(issued))?;Ok(())}));}
  assert!(owner.ready_to_handoff());let mut d=owner.handoff_demands().unwrap();let parent_copy=std::mem::size_of::<Option<semio_framework_actor::typed_operation_page::TypedOperationResultPage>>();d.copy_bytes+=parent_copy;let g=grant(d);let mut output=None;
  turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{
   close_original_retained_step(cx,d,|issued|{let step=owner.take_page(RetainedCloneGrant{maximum_copy_bytes:issued.maximum_copy_bytes-parent_copy,..issued});RetainedWorkOutcome{work:step.work,result:step.result.map(|page|{let(page,mut progress)=page.unwrap();output=Some(page);progress.copied_bytes+=parent_copy;semio_framework_value::RetainedCloneStep::Progress(progress)})}})?;Ok(())
  }));
  let mut page=output.unwrap();assert_eq!(page.token,token);let expected=serde_json::json!({"generationBefore":7,"generationAfter":8});assert_eq!(serde_json::from_slice::<serde_json::Value>(page.bytes()).unwrap(),expected);assert!(owner.terminal_is_empty());assert_eq!(decoder.receiving_identity(),decoder_identity);
  for _ in 0..65536{if page.terminal_is_empty(){break}let d=page.next_close_demand(32768).unwrap();let g=grant(d);turn(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|issued|RetainedWorkOutcome{result:page.close_step(issued),work:1})?;Ok(())}));}
  assert!(page.terminal_is_empty());retire(caller,&mut decoder);
 }
}
