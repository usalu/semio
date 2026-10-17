//! 🧪️ The real registered invocation lends the original context and keeps pending caller bytes intact.
use semio_framework_plugin_extension::extension_runtime_lift::ExtensionBundle;
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::{ExtensionResourceOwner, MetadataOwner};
use semio_framework::Fault;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
use std::sync::atomic::{AtomicUsize, Ordering};

struct OriginalResourceCloseProbe{pub failure:Option<semio_framework_job::OriginalTurnFailure>,pub closed:bool}
impl ExtensionResourceOwner for OriginalResourceCloseProbe{
 fn invoke(&mut self,_:&str,_:&[u8],_:&mut semio_framework_job::StepContext<'_>)->Result<ExtensionInvokeStep,Fault>{unreachable!("resource close probe has no invocation")}
 fn begin_close(&mut self){}
 fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
  let progress=RetainedCloneProgress{copied_items:1,..Default::default()};if cx.should_yield()||!progress.fits(cx.retained_grant())||cx.retained_grant().maximum_depth==0{return Ok(PluginLifecycleStep::Progress(Default::default()))}
  cx.consume_retained(progress)?;cx.consume_fuel(1);self.closed=true;Err(self.failure.take().expect("same original failure remains owned until actual source close"))
 }
 fn terminal_is_empty(&self)->bool{self.closed&&self.failure.is_none()}
 fn retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
}

/// 🛂️ Actual bundle delegation returns three original causes after one paid source donation and complete physical disposal.
#[test]
fn original_extension_resource_close_preserves_three_causes_same_caller_and_single_debit(){
 use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,OriginalTurnFailure,close_original_retained_step};
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 let rows:serde_json::Value=serde_json::from_str(include_str!("../../../⚛️reactor/🎟️context/🌉️call/⚖️failure-debit/🧪️tests/🧫️fixtures/🚪️resources.json")).unwrap();let row=&rows[1];let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
 for copy in [1,3,64]{
  let(mut caller,caller_birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()),Generation(policy["generation"].as_u64().unwrap()),stock,||Some(1000),policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap()));
  let((mut bundle,pointers),initial)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let primary=ValueError::new(ValueRefusalKind::InvalidValue,row["causes"][0].as_str().unwrap()).with_retained_progress(RetainedCloneProgress{copied_items:1,..Default::default()});let receipt=ValueError::new(ValueRefusalKind::OwnershipLimit,row["causes"][1].as_str().unwrap());let context=ValueError::new(ValueRefusalKind::InvariantViolated,row["causes"][2].as_str().unwrap());let pointers=(primary.message.as_ptr(),receipt.message.as_ptr(),context.message.as_ptr());
   let failure=OriginalTurnFailure{primary:primary.into(),receipt_refusal:Some(receipt),context_refusal:Some(context)};
   (ExtensionBundle::new("original-resource","Original Resource","1").resource_owner(OriginalResourceCloseProbe{failure:Some(failure),closed:false}),pointers)
  });let(mut born,mut freed)=(initial.requested_bytes,initial.released_bytes);
  caller.turn(stock,|cx|{
   let identity=(cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.clock_source()as usize,cx.deadline_us());let loan=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,..stock};
   for denied in [RetainedCloneGrant{maximum_items:0,..loan},RetainedCloneGrant{maximum_depth:0,..loan}]{let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|bundle.close_step(same)));assert_eq!(step?.progress(),Some(Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);}
   let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|bundle.close_step(same)));let failure=result.unwrap_err();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(fuel-cx.fuel_remaining(),1);assert_eq!(cx.retained_progress(),before.checked_add(RetainedCloneProgress{copied_items:1,..Default::default()})?);assert_eq!((failure.primary.message().as_ptr(),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr()),pointers);
   let(mut failure,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(failure).unwrap_or_else(|_|panic!("complete native failure declares retirement")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
   for turn in 0..policy["maximumTurns"].as_u64().unwrap(){if failure.terminal_is_empty(){break}let demand=failure.selected_demand(copy)?;let before=cx.retained_progress();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(RetainedCloneGrant{maximum_copy_bytes:copy,..loan},|same|close_original_retained_step(same,demand,|grant|failure.step(grant))));let progress=step?.progress();assert_eq!(cx.retained_progress(),before.checked_add(progress)?);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<policy["maximumTurns"].as_u64().unwrap());}
   assert!(failure.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(failure));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   for turn in 0..policy["maximumTurns"].as_u64().unwrap(){if bundle.terminal_is_empty(){break}let before=cx.retained_progress();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|bundle.close_step(same)));let progress=step?.progress().unwrap_or_default();assert_eq!(cx.retained_progress(),before.checked_add(progress)?);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn+1<policy["maximumTurns"].as_u64().unwrap());}
   assert!(bundle.terminal_is_empty());assert_eq!(born,freed);assert_eq!((cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.clock_source()as usize,cx.deadline_us()),identity);Ok(())
  }).unwrap();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bundle));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(_,close)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.retire(stock));assert_eq!(caller_birth.requested_bytes+close.requested_bytes,caller_birth.released_bytes+close.released_bytes);
 }
}

struct OriginalInvocationProbe{pub pointer:usize,pub operation:u64,pub generation:u64,pub calls:AtomicUsize}
impl ExtensionResourceOwner for OriginalInvocationProbe{
 fn invoke(&mut self,_:&str,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<ExtensionInvokeStep,Fault>{
  assert_eq!(request.as_ptr()as usize,self.pointer);assert_eq!(cx.operation().0,self.operation);assert_eq!(cx.generation().0,self.generation);
  let grant=cx.retained_grant();if cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(ExtensionInvokeStep{payload:None,retained_progress:Default::default(),refusal:None})}
  let retained_progress=RetainedCloneProgress{copied_items:1,..Default::default()};cx.consume_retained(retained_progress).map_err(|error|semio_framework_diagnostic::FaultFrom::to_fault(&error))?;cx.consume_fuel(1);
  let complete=self.calls.fetch_add(1,Ordering::SeqCst)!=0;Ok(ExtensionInvokeStep{payload:complete.then(Vec::new),retained_progress,refusal:None})
 }
 fn begin_close(&mut self){}
 fn close_step(&mut self,_:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{Ok(PluginLifecycleStep::Complete(Default::default()))}
 fn terminal_is_empty(&self)->bool{true}
 fn retirement_demands(&self,_:usize)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
}

/// 🤝️ Normal registered handlers receive the same caller identity, source and independently debited receipt.
#[test]
fn original_extension_invoke_lends_caller_context_and_retains_pending_source(){
 let mut original_caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=original_caller.context();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📨️invoke/🔣️.json")).unwrap();
 let request=fixture["text"].as_str().unwrap().repeat(fixture["repeat"].as_u64().unwrap()as usize).into_bytes();let pointer=request.as_ptr();assert_eq!(request.len(),fixture["utf8Bytes"].as_u64().unwrap()as usize);
 let operation=cx.operation().0;let generation=cx.generation().0;
 let mut bundle=ExtensionBundle::new("original-invoke","Original Invoke","1").resource_owner(OriginalInvocationProbe{pointer:pointer as usize,operation,generation,calls:AtomicUsize::new(0)}).owned_handler("probe");
 let policy=&fixture["caller"]["grant"];let grant=RetainedCloneGrant{maximum_items:policy["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap()as usize};
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|{let reply=bundle.invoke("probe",&request,same)?;assert!(reply.payload.is_none());assert_eq!(reply.retained_progress,Default::default());Ok(())}));result.unwrap();assert_eq!(cx.retained_progress(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(request.as_ptr(),pointer);}
 for complete in [false,true]{let before=cx.retained_progress();let fuel=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(grant,|same|{let reply=bundle.invoke("probe",&request,same)?;assert_eq!(reply.payload.is_some(),complete);assert_eq!(reply.retained_progress,RetainedCloneProgress{copied_items:1,..Default::default()});Ok(())}));result.unwrap();assert_eq!(cx.retained_progress(),before.checked_add(RetainedCloneProgress{copied_items:1,..Default::default()}).unwrap());assert_eq!(fuel-cx.fuel_remaining(),1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(request.as_ptr(),pointer);}
 bundle.dispose_cold(&mut cx).unwrap();let fuel=cx.fuel_remaining();drop(cx);original_caller.retire(fuel).unwrap();
}

/// 🛡️ Admission preserves the exact refused payload and typed cause without spending another wallet.
#[test]
fn original_extension_invoke_refusal_preserves_owned_payload_and_actual_receipt(){
 let payload=b"original refused output".to_vec();let pointer=payload.as_ptr();
 let answer=ExtensionInvokeStep{payload:Some(payload),retained_progress:RetainedCloneProgress{copied_items:1,..Default::default()},refusal:None};
 let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||answer.admit(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},Default::default(),Default::default()));
 assert_eq!(answer.payload.as_ref().unwrap().as_ptr(),pointer);let cause=answer.refusal.as_ref().unwrap();assert_eq!(cause.primary_value().unwrap().kind,ValueRefusalKind::InvariantViolated);assert_eq!(cause.primary.retained_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

/// 🧳️ A second admission retains both original causes and the same output until paid full retirement.
#[test]
fn original_extension_admit_keeps_whole_failure_pair_and_paid_output_close(){
 use semio_framework_value::retirement::{controlled::ControlledRetirement,RetireOwned};
 use semio_framework_job::OriginalTurnFailure;
 for copy in [1,3,64]{
  let primary=ValueError::new(ValueRefusalKind::InvalidValue,"original primary 源");let secondary=ValueError::new(ValueRefusalKind::OwnershipLimit,"original receipt 源");let pointers=(primary.message.as_ptr(),secondary.message.as_ptr());let payload="original retained output 源".as_bytes().to_vec();let pointer=payload.as_ptr();
  let owned=primary.message.len()+secondary.message.len()+payload.capacity();let step=ExtensionInvokeStep{payload:Some(payload),retained_progress:RetainedCloneProgress{copied_items:2,..Default::default()},refusal:Some(OriginalTurnFailure{primary:primary.into(),receipt_refusal:Some(secondary),context_refusal:None})};
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||step.admit(Default::default(),RetainedCloneProgress{copied_items:9,..Default::default()},Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(step.payload.as_ref().unwrap().as_ptr(),pointer);let refusal=step.refusal.as_ref().unwrap();assert_eq!(refusal.primary.message().as_ptr(),pointers.0);assert_eq!(refusal.receipt_refusal.as_ref().unwrap().message.as_ptr(),pointers.1);
  assert!(ExtensionInvokeStep::controlled_retirement_supported());let(mut close,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(step).unwrap_or_else(|_|panic!("original invocation declares typed retirement")));let mut born=owned+heap.requested_bytes;let mut freed=heap.released_bytes;
  for turn in 0..100000{if close.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(denied,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close.step(RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(denied.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close.step(grant));let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<99999,"original invocation pair closure stalled");}
  assert_eq!(born,freed);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(close));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
}
