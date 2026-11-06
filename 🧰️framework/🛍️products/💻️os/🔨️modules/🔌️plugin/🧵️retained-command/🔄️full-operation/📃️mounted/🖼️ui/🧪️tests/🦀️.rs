#[cfg(test)]
#[global_allocator]
static MOUNTED_UI_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

use semio_framework_plugin_retained_command::typed_operation::MountedUiCopy;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetainedWorkOutcome,RetireOwned};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext,close_original_retained_step};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096}}
fn g(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(grant,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(p.fits(grant));result.unwrap()}
fn retire<T:RetireOwned+'static>(input:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){let mut pending=Some(input);let mut active:Option<ControlledRetirement<T>>=None;for _ in 0..100000{if pending.is_none()&&active.is_none(){return}let d=if let Some(owner)=active.as_ref(){if owner.terminal_is_empty(){RetirementDemand{copy_bytes:size_of_val(&active),depth:1,..Default::default()}}else{owner.next_demand(2048).unwrap()}}else{RetirementDemand{copy_bytes:2*size_of_val(&pending)+size_of_val(&active),depth:1,..Default::default()}};turn(caller,decoder,g(d),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|grant|{if let Some(owner)=active.as_mut(){if !owner.terminal_is_empty(){let result=owner.step(grant);return RetainedWorkOutcome{work:owner.last_turn_work(),result}}active=None;}else{active=Some(ControlledRetirement::retain(pending.take().unwrap()));}RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}})?;Ok(())}));}panic!("scope fixture original retirement must terminate")}
fn close_decoder(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for _ in 0..100000{if decoder.terminal_is_empty(){return}let d=decoder.retirement_demands(2048).unwrap();caller.turn(g(d),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();}panic!("original decoder must close")}
#[test]
fn mounted_ui_copy_pages_both_vectors_and_flags_under_one_original_stock(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/📃️page-admission/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap().iter().filter(|row|row["payload"]["kind"]=="ui"){
  let source:UiDirtyScope=serde_json::from_value(row["payload"]["value"].clone()).unwrap();let expected=serde_json::to_value(&source).unwrap();
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(44801),Generation(1),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();
  for _ in 0..2{let mut copy=MountedUiCopy::new();
   for _ in 0..10000{if copy.ready(){break}let d=copy.demands(&source,128).unwrap();let grant=g(d);
    for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then(||RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant}),(d.capacity_bytes>0).then(||RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..grant}),(d.release_bytes>0).then(||RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..grant}),(d.depth>0).then(||RetainedCloneGrant{maximum_depth:d.depth-1,..grant})].into_iter().flatten(){let(turn,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||copy.advance(&source,denied));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    turn(&mut caller,&mut decoder,grant,|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|grant|copy.advance(&source,grant))?;Ok(())}));
   }assert!(copy.ready());let d=MountedUiCopy::transfer_demands();let mut output=None;turn(&mut caller,&mut decoder,g(d),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|_|{output=Some(copy.take());RetainedWorkOutcome{work:1,result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}})?;Ok(())}));
   assert!(copy.terminal_is_empty());let output=output.unwrap();assert_eq!(serde_json::to_value(&output).unwrap(),expected);retire(output,&mut caller,&mut decoder);
  }retire(source,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder);caller.retire(stock());
 }
}
#[test]
fn mounted_ui_copy_cancelled_partial_owner_has_a_paid_close(){
 let source=UiDirtyScope::Partial{window_bodies:vec!["ä".repeat(4000)],panel_bodies:vec!["Details".into()],utilities:true,tools:false,engagements:true,measures:false,labels:true};let mut caller=OriginalJobFixtureCaller::admit(OperationId(44802),Generation(1),stock(),||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let mut copy=MountedUiCopy::new();
 for _ in 0..5{let d=copy.demands(&source,128).unwrap();turn(&mut caller,&mut decoder,g(d),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|grant|copy.advance(&source,grant))?;Ok(())}));}
 caller.cancel().cancel_now();for _ in 0..100000{if copy.terminal_is_empty(){break}let d=copy.close_demands(128).unwrap();turn(&mut caller,&mut decoder,g(d),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|grant|copy.close_step(grant))?;Ok(())}));}assert!(copy.terminal_is_empty());retire(source,&mut caller,&mut decoder);close_decoder(&mut caller,&mut decoder);caller.retire(stock());
}
