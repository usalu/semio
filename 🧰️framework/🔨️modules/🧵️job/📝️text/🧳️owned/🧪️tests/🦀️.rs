//! 🧪️ Original authoring text receipts match physical allocation through success and cancellation.
use super::FundedText;
#[global_allocator]
static ALLOCATOR:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,StepContext,OriginalTurnFailure,OperationId,Generation,close_original_effect_step,receive_original_retained_work};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value::retained_clone::RetainedCloneClose;
fn native<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}

fn advance(work:&mut FundedText,source:&str,cx:&mut StepContext<'_>,raw:bool)->Result<bool,OriginalTurnFailure>{
 if !raw{return work.advance(source,cx)}if cx.should_yield()||cx.is_cancelled(){return Ok(work.ready())}let fuel=work.next_work_demand(source,cx.retained_grant().maximum_copy_bytes)?;if cx.fuel_remaining()<fuel{return Ok(work.ready())}let d=work.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(work.ready())}let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|work.advance_original(source,effect))?else{return Ok(work.ready())};assert!(turn.work<=u128::from(fuel));assert_eq!(turn.work,performed);receive_original_retained_work(cx,turn)?;Ok(work.ready())
}
fn take(work:&mut FundedText,output:&mut Option<String>,cx:&mut StepContext<'_>,raw:bool)->Result<bool,OriginalTurnFailure>{
 if !raw{return work.take_into(output,cx)}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let fuel=work.take_work_demand()?;if cx.fuel_remaining()<fuel{return Ok(false)}let d=work.take_demands()?;if !cx.retained_work_can_enter(d){return Ok(false)}let grant=cx.retained_grant();let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|work.take_original(effect))?else{return Ok(false)};assert!(turn.work<=u128::from(fuel));assert_eq!(turn.work,performed);
 let mut moved=false;let settled=RetainedWorkOutcome{result:turn.result.map(|pair|match pair{Some((original,p))=>{assert!(output.is_none());*output=Some(original);moved=true;RetainedCloneStep::Complete(p)},None=>RetainedCloneStep::Progress(Default::default())}),work:turn.work};receive_original_retained_work(cx,settled)?;Ok(moved)
}
fn denied_advance(work:&mut FundedText,source:&str,cx:&mut StepContext<'_>,denied:u64)->Result<(),OriginalTurnFailure>{
 cx.with_original_work_loan(denied,cx.deadline_us(),|cx|{let grant=cx.retained_grant();if let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|work.advance_original(source,effect))?{assert_eq!((turn.work,performed),(0,0));assert_eq!(turn.result?.progress(),RetainedCloneProgress::default());}Ok(())})
}

#[test]
fn original_authoring_text_work_quotes_match_neutral_utf8_chunks(){
 let law:serde_json::Value=serde_json::from_str(include_str!("📁️text-work.json")).unwrap();
 let limit=law["maximumEvents"].as_u64().unwrap()as usize;
 for raw in [false,true]{for row in law["cases"].as_array().unwrap(){
  let source=row["source"].as_str().unwrap();let payload=row["payloadBytes"].as_u64().unwrap()as usize;
  let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128};
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(983),Generation(68),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<26,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:128},semio_framework_job::default_now_us,u64::MAX,1<<20);
  let mut decoder=OriginalJobDecodeOwner::empty();native(&mut caller,&mut decoder,g,|_|Ok(()));
  let mut work=FundedText::new();let mut output=None;native(&mut caller,&mut decoder,g,|cx|advance(&mut work,source,cx,raw));
  let chunk=RetainedCloneGrant{maximum_copy_bytes:payload+std::mem::size_of::<usize>(),..g};
  for expected in row["chunks"].as_array().unwrap(){
   assert!(!work.ready());let d=work.next_demands(source,chunk.maximum_copy_bytes).unwrap();assert_eq!(d.copy_bytes-std::mem::size_of::<usize>(),expected.as_u64().unwrap()as usize);
   let fuel=work.next_work_demand(source,chunk.maximum_copy_bytes).unwrap();
   for denied in [0,fuel-1]{native(&mut caller,&mut decoder,chunk,|cx|cx.with_original_work_loan(denied,cx.deadline_us(),|cx|advance(&mut work,source,cx,raw)));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(work.next_demands(source,chunk.maximum_copy_bytes).unwrap(),d);}
   native(&mut caller,&mut decoder,chunk,|cx|cx.with_original_work_loan(fuel,cx.deadline_us(),|cx|advance(&mut work,source,cx,raw)));
   assert_eq!(caller.receipt().copied_bytes,d.copy_bytes);
  }
  assert!(work.ready());
  native(&mut caller,&mut decoder,g,|cx|cx.with_original_work_loan(0,cx.deadline_us(),|cx|take(&mut work,&mut output,cx,raw)));assert!(output.is_none());assert!(work.ready());assert_eq!(caller.receipt(),RetainedCloneProgress::default());
  native(&mut caller,&mut decoder,g,|cx|cx.with_original_work_loan(work.take_work_demand()?,cx.deadline_us(),|cx|take(&mut work,&mut output,cx,raw)));assert_eq!(output.as_deref(),Some(source));
  close_text(&mut work,&mut output,&mut caller,&mut decoder,g,limit);drop(work);caller.retire(g);
 }}
}

fn close_text(work:&mut FundedText,output:&mut Option<String>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,limit:usize){
 work.begin_close();
 for index in 0..limit{
  if work.terminal_is_empty(){break}
  native(caller,decoder,g,|cx|cx.with_original_retirement(|cx|{let d=work.close_demands(4096)?;close_original_effect_step(cx,d,|effect|work.close_step(effect))?;Ok(())}));
  assert!(index+1<limit);
 }
 assert!(work.terminal_is_empty());
 let mut close=RetainedCloneClose::default();
 for index in 0..limit{
  if output.is_none()&&close.is_empty(){break}
  native(caller,decoder,g,|cx|cx.with_original_retirement(|cx|{let d=close.next_owner_demands::<String>(output.is_some(),&None,4096)?;close_original_effect_step(cx,d,|effect|if close.is_empty(){let turn=close.begin_granted(output,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::<semio_framework_value::OriginalRetirementFailure>::into),work:turn.work})}else{close.step_granted(effect)})?;Ok(())}));
  assert!(index+1<limit);
 }
 assert!(output.is_none()&&close.is_empty());
 for index in 0..limit{
  if decoder.terminal_is_empty(){return}
  caller.turn(g,|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();
  assert!(index+1<limit);
 }
 assert!(decoder.terminal_is_empty());
}

#[test]
fn original_authoring_text_keeps_exact_source_receipts_and_cancel_custody(){

 let law:serde_json::Value=serde_json::from_str(include_str!("📁️text.json")).unwrap();
 let limit=law["maximumEvents"].as_u64().unwrap()as usize;
 for raw in [false,true]{for source in law["samples"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()){
  for cancel in law["cancelAfter"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap()as usize)).chain([None]){
   let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128};
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(982),Generation(67),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<26,maximum_capacity_bytes:1<<26,maximum_release_bytes:1<<26,maximum_depth:128},semio_framework_job::default_now_us,u64::MAX,1<<20);
   let mut decoder=OriginalJobDecodeOwner::empty();
   native(&mut caller,&mut decoder,g,|_|Ok(()));
   let mut work=FundedText::new();let mut output=None;let pointer=source.as_ptr();
   for index in 0..limit{
    if cancel==Some(index){caller.cancel().cancel_now();let before=work.ready();native(&mut caller,&mut decoder,g,|cx|advance(&mut work,source,cx,raw));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(work.ready(),before);break}
    if work.ready(){native(&mut caller,&mut decoder,g,|cx|take(&mut work,&mut output,cx,raw));break}
    let before=work.next_demands(source,4096).unwrap();
    let fuel=work.next_work_demand(source,4096).unwrap();assert!(fuel>0);
    for denied in [0,fuel-1]{native(&mut caller,&mut decoder,g,|cx|denied_advance(&mut work,source,cx,denied));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(work.next_demands(source,4096).unwrap(),before);}
    for denied in [0,fuel-1]{native(&mut caller,&mut decoder,g,|cx|cx.with_original_work_loan(denied,cx.deadline_us(),|cx|advance(&mut work,source,cx,raw)));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(work.next_demands(source,4096).unwrap(),before);assert_eq!(work.next_work_demand(source,4096).unwrap(),fuel);}
    native(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..g},|cx|advance(&mut work,source,cx,raw));
    assert_eq!(caller.receipt(),RetainedCloneProgress::default());
    assert_eq!(work.next_demands(source,4096).unwrap(),before);
    native(&mut caller,&mut decoder,g,|cx|cx.with_original_work_loan(fuel,cx.deadline_us(),|cx|advance(&mut work,source,cx,raw)));
    assert_eq!(source.as_ptr(),pointer);
    assert!(index+1<limit,"original text source failed to reach a funded frontier");
   }
   if cancel.is_none(){assert_eq!(output.as_deref(),Some(source))}
   close_text(&mut work,&mut output,&mut caller,&mut decoder,g,limit);drop(work);caller.retire(g);
  }
 }
}
}
