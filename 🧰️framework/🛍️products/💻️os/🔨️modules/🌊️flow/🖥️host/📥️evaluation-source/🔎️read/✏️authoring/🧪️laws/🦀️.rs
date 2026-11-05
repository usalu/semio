//! 🧪️ Original authoring text receipts match physical allocation through success and cancellation.
#[path="../📑️text/🦀️.rs"]
mod text;
use text::FundedText;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,StepContext,OriginalTurnFailure,OperationId,Generation,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retained_clone::RetainedCloneClose;
fn native<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}

fn advance(work:&mut FundedText,source:&str,cx:&mut StepContext<'_>,raw:bool)->Result<bool,OriginalTurnFailure>{
 if !raw{return work.advance(source,cx)}if cx.should_yield()||cx.is_cancelled(){return Ok(work.ready())}let d=work.next_demands(source,cx.retained_grant().maximum_copy_bytes)?;if !cx.retained_work_can_enter(d){return Ok(work.ready())}let turn=work.advance_original(source,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(work.ready())
}
fn take(work:&mut FundedText,output:&mut Option<String>,cx:&mut StepContext<'_>,raw:bool)->Result<bool,OriginalTurnFailure>{
 if !raw{return work.take_into(output,cx)}if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let d=work.take_demands()?;if !cx.retained_work_can_enter(d){return Ok(false)}let turn=work.take_original(cx.retained_grant());cx.consume_fuel(turn.work);match turn.result{Ok(Some((original,p)))=>{assert!(output.is_none());*output=Some(original);cx.consume_retained(p)?;Ok(true)},Ok(None)=>Ok(false),Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx))}
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
    native(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..g},|cx|advance(&mut work,source,cx,raw));
    assert_eq!(caller.receipt(),RetainedCloneProgress::default());
    assert_eq!(work.next_demands(source,4096).unwrap(),before);
    native(&mut caller,&mut decoder,g,|cx|advance(&mut work,source,cx,raw));
    assert_eq!(source.as_ptr(),pointer);
    assert!(index+1<limit,"original text source failed to reach a funded frontier");
   }
   if cancel.is_none(){assert_eq!(output.as_deref(),Some(source))}
   work.begin_close();
   for index in 0..limit{
    if work.terminal_is_empty(){break}
    native(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{let d=work.close_demands(4096)?;close_original_retained_step(cx,d,|grant|work.close_step(grant))?;Ok(())}));
    assert!(index+1<limit);
   }
   assert!(work.terminal_is_empty());drop(work);
   let mut close=RetainedCloneClose::default();
   for index in 0..limit{
    if output.is_none()&&close.is_empty(){break}
    native(&mut caller,&mut decoder,g,|cx|cx.with_original_retirement(|cx|{let d=close.next_owner_demands::<String>(output.is_some(),&None,4096)?;close_original_retained_step(cx,d,|grant|if close.is_empty(){let turn=close.begin_granted(&mut output,grant);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}else{match close.select(4096){Ok(selected)=>selected.step(grant),Err(error)=>semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0}}})?;Ok(())}));
    assert!(index+1<limit);
   }
   while !decoder.terminal_is_empty(){caller.turn(g,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(4096)?;close_original_retained_step(cx,d,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}
   caller.retire(g);
  }
 }
}
}
