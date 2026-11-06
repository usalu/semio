//! 🧪️ Complete lifecycle source ownership and original-context laws use the neutral corpus.
use super::*;
use semio_framework::{FaultCode,FaultOrigin};
use semio_framework_job::{Generation,OperationId,StepBudget,root_cancel_token};
use semio_framework_value::{ValueRefusalKind,retirement::controlled::ControlledRetirement};
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128}}
fn source(kind:&str,corpus:&serde_json::Value,progress:RetainedCloneProgress)->PluginLifecycleFailure{
 if kind=="fault"{let mut fault=Fault::new(FaultOrigin::Plugin,FaultCode::new(corpus["fault"]["code"].as_str().unwrap()),corpus["fault"]["message"].as_str().unwrap()).with_retained_progress(progress);fault.causes.push(semio_framework_diagnostic::FaultCause{message:corpus["fault"]["cause"].as_str().unwrap().into(),code:None,retained_progress:None});fault.into()}else{ValueError::new(ValueRefusalKind::UnsupportedOwner,corpus["value"]["message"].as_str().unwrap()).with_retained_progress(progress).into()}
}
fn pointer(source:&PluginLifecycleFailure)->*const u8{match &source.primary{PluginLifecyclePrimary::Fault(fault)=>fault.message.as_ptr(),PluginLifecyclePrimary::Value(error)=>error.message.as_ptr(),PluginLifecyclePrimary::Domain(original)=>original.message().as_ptr(),PluginLifecyclePrimary::Receiving(original)=>original.message().as_ptr(),PluginLifecyclePrimary::Retirement(original)=>original.primary().or(original.work_refusal()).or(original.receipt_refusal()).expect("retirement cause").message.as_ptr(),PluginLifecyclePrimary::RetirementReceiving(original)=>original.receiving_primary.message.as_ptr()}}
fn close(source:PluginLifecycleFailure,copy:usize)->(usize,usize){
 let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(source).unwrap_or_else(|_|panic!("lifecycle failure retirement unsupported")));let(mut born,mut freed)=(heap.requested_bytes,heap.released_bytes);let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..grant()};
 for turn in 0..100000{if owner.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant));let progress=step.unwrap().progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(turn<99999)}assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));(born,freed)
}

/// 🪢️ Both original error kinds preserve their exact body, causes and separately owned receipt refusal.
#[test]
fn original_lifecycle_whole_primary_and_raw_failure_debit_once(){
 let corpus=corpus();for kind in corpus["primaryTypes"].as_array().unwrap(){for copy in [1,3,64]{let((source,source_pointer),initial)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let source=source(kind.as_str().unwrap(),&corpus,RetainedCloneProgress{copied_items:1,copied_bytes:4,..Default::default()});let pointer=pointer(&source);(source,pointer)});let mut receipt=Default::default();let mut sequence=0;let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(2,12345,grant()),root_cancel_token(),||Some(1),&mut sequence,&mut receipt);
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original_lifecycle_step(&mut cx,grant(),RetirementDemand::default(),|_,_|Err(source)));let failure=result.unwrap_err();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pointer(&failure),source_pointer);assert_eq!(failure.retained_progress(),cx.retained_progress());assert_eq!(cx.retained_progress().copied_bytes,4);assert_eq!(cx.fuel_remaining(),1);let refusal=failure.receipt_refusal.as_ref().unwrap();assert_eq!(refusal.kind,ValueRefusalKind::InvariantViolated);assert_eq!(refusal.retained_progress(),failure.retained_progress());if let PluginLifecyclePrimary::Fault(fault)=&failure.primary{assert_eq!(fault.code.0,corpus["fault"]["code"].as_str().unwrap());assert_eq!(fault.causes[0].message,corpus["fault"]["cause"].as_str().unwrap())}let(born,freed)=close(failure,copy);assert_eq!(initial.requested_bytes+born,initial.released_bytes+freed);
 }}
}

/// 🚦️ Every denied currency, work grant and original deadline preserves the actual source with zero System effects.
#[test]
fn original_lifecycle_denials_preserve_original_source_and_upper_loan(){
 let corpus=corpus();let(source,initial)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source("fault",&corpus,Default::default()));let source_pointer=pointer(&source);let mut held=Some(source);let mut sequence=0;let demand=RetirementDemand{copy_bytes:3,capacity_bytes:7,release_bytes:11,depth:2};let full=grant();
 for axis in corpus["denials"].as_array().unwrap(){let axis=axis.as_str().unwrap();let mut loan=full;match axis{"items"=>loan.maximum_items=0,"copy"=>loan.maximum_copy_bytes=2,"capacity"=>loan.maximum_capacity_bytes=6,"release"=>loan.maximum_release_bytes=10,"depth"=>loan.maximum_depth=1,_=>{}}let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(if axis=="work"{0}else{1},if axis=="deadline"{0}else{12345},full),root_cancel_token(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original_lifecycle_step(&mut cx,loan,demand,|_,_|{drop(held.take());panic!("denied lifecycle body ran")}));assert_eq!(result.unwrap().progress(),Some(Default::default()));assert_eq!(cx.retained_progress(),Default::default());assert_eq!(pointer(held.as_ref().unwrap()),source_pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
 let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(1,12345,full),root_cancel_token(),||Some(1),&mut sequence,&mut receipt);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original_lifecycle_step(&mut cx,full,Default::default(),|_,actual|{assert_eq!(actual.maximum_items,1);Ok((PluginLifecycleStep::Progress(Default::default()),false))}));assert_eq!(step.unwrap().progress(),Some(Default::default()));assert_eq!(cx.fuel_remaining(),0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(born,freed)=close(held.take().unwrap(),3);assert_eq!(initial.requested_bytes+born,initial.released_bytes+freed);
}

/// 🤝️ A nested producer receives the original context and its physical effects remain posted exactly once.
#[test]
fn original_lifecycle_adopts_nested_context_receipt_without_second_debit(){
 let mut sequence=0;for failed in [false,true]{let progress=RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()};let source=source("value",&corpus(),progress);let source_pointer=pointer(&source);let mut source=Some(source);let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(2,12345,grant()),root_cancel_token(),||Some(1),&mut sequence,&mut receipt);
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original_lifecycle_step(&mut cx,grant(),Default::default(),|original,loan|{assert_eq!((original.operation().0,original.generation().0),(17,9));assert_eq!(loan.maximum_items,1);original.consume_fuel(1);original.consume_retained(progress).unwrap();if failed{Err(source.take().unwrap())}else{Ok((PluginLifecycleStep::Progress(progress),false))}}));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),progress);assert_eq!(cx.fuel_remaining(),1);if failed{let failure=result.unwrap_err();assert_eq!(pointer(&failure),source_pointer);assert!(failure.receipt_refusal.is_none());close(failure,3);}else{assert_eq!(result.unwrap().progress(),Some(progress));close(source.take().unwrap(),3);}
 }
}

/// 🔄️ Job and lifecycle boundaries transfer both complete original owners without replacing their storage.
#[test]
fn original_lifecycle_job_pair_transfer_preserves_both_owned_pointers(){
 let corpus=corpus();for kind in ["fault","value"]{
  let mut failure=source(kind,&corpus,Default::default());
  failure.receipt_refusal=Some(ValueError::new(ValueRefusalKind::InvariantViolated,corpus["independentRefusals"]["receipt"].as_str().unwrap()));
  failure.context_refusal=Some(ValueError::new(ValueRefusalKind::InvariantViolated,corpus["independentRefusals"]["context"].as_str().unwrap()));
  failure.work_refusal=Some(ValueError::new(ValueRefusalKind::WorkLimit,corpus["independentRefusals"]["work"].as_str().unwrap()));
  let pointers=(pointer(&failure),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr(),failure.work_refusal.as_ref().unwrap().message.as_ptr());
  let(failure,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let job:semio_framework_job::OriginalTurnFailure=failure.into();PluginLifecycleFailure::from(job)});
  assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  assert_eq!((pointer(&failure),failure.receipt_refusal.as_ref().unwrap().message.as_ptr(),failure.context_refusal.as_ref().unwrap().message.as_ptr(),failure.work_refusal.as_ref().unwrap().message.as_ptr()),pointers);
  assert_eq!(failure.work_refusal.as_ref().unwrap().message,corpus["independentRefusals"]["work"].as_str().unwrap());close(failure,3);
 }
}

/// 🪡️ An actual issued decoder scope retains the producer error when its enclosing grant is refused.
#[test]
fn original_receiving_refusal_keeps_the_raw_semantic_owner(){
 let stock=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:65536,..grant()};
 for kind in ["fault","value"]{
  let original=source(kind,&corpus(),Default::default());let original_pointer=pointer(&original);let mut held=Some(original.primary);let mut paired=None;
  let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(OperationId(97533),Generation(18),stock,close_clock,100,10000);let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();
  caller.turn_with_decoder(stock,&mut decoder,|cx|{
   let issued=cx.retained_grant();
   let result=cx.with_native_receiving::<(),PluginLifecyclePrimary>(|_|Err(held.take().unwrap()));
   let mut invalid=issued;invalid.maximum_depth+=1;
   let receipt=cx.consume_receiving_retained(invalid,Default::default());
   assert!(receipt.is_err());
   let(failure,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||result.unwrap_err().with_receiving_receipt(receipt));
   assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pointer(&failure),original_pointer);assert!(failure.receipt_refusal.is_some());assert!(failure.context_refusal.is_none()&&failure.work_refusal.is_none());assert_eq!(cx.retained_progress(),Default::default());paired=Some(failure);Ok(())
  }).unwrap();
  while !decoder.terminal_is_empty(){caller.turn(stock,|cx|{semio_framework_job::close_original_retained_step(cx,decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?,|grant|decoder.close_step(grant))?;Ok(())}).unwrap();}
  let mut retirement=semio_framework_value::retained_clone::RetainedCloneClose::default();
  while paired.is_some()||!retirement.is_empty(){
   caller.turn(stock,|cx|{
    let demand=retirement.next_owner_demands::<PluginLifecycleFailure>(paired.is_some(),&None,cx.retained_grant().maximum_copy_bytes)?;
    semio_framework_job::close_original_retained_step(cx,demand,|grant|{
     if !retirement.is_empty(){return retirement.step_granted(grant)}
     let turn=retirement.begin_granted(&mut paired,grant);semio_framework_value::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|step|step.unwrap_or(semio_framework_value::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)}
    })?;Ok(())
   }).unwrap();
  }
  caller.retire(stock);
 }
}

thread_local!{static CLOSE_CLOCK:std::cell::Cell<u64>=const{std::cell::Cell::new(1)};}
fn close_clock()->Option<u64>{Some(CLOSE_CLOCK.get())}
#[test]
fn original_lifecycle_expired_deadline_needs_actual_paid_retirement_permission(){
    CLOSE_CLOCK.set(1);let stock=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:65536,..grant()};let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(OperationId(97532),Generation(18),stock,close_clock,100,10000);CLOSE_CLOCK.set(101);let demand=RetirementDemand{copy_bytes:3,depth:1,..Default::default()};let progress=RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()};
    caller.turn(stock,|cx|{let before=cx.retained_progress();let ordinary=original_lifecycle_step(cx,stock,demand,|_,_|panic!("expired semantic lifecycle ran")).map_err(semio_framework_job::OriginalTurnFailure::from)?;assert_eq!(ordinary.progress(),Some(Default::default()));assert_eq!(cx.retained_progress(),before);cx.with_original_retirement(|cx|{let paid=original_lifecycle_step(cx,stock,demand,|_,_|Ok((PluginLifecycleStep::Complete(progress),true))).map_err(semio_framework_job::OriginalTurnFailure::from)?;assert_eq!(paid.progress(),Some(progress));Ok(())})}).unwrap();caller.retire(stock);CLOSE_CLOCK.set(1);
}

#[test]
fn original_lifecycle_retirement_roundtrip_preserves_nested_owners(){
 let primary=ValueError::new(ValueRefusalKind::InvalidValue,"outer original");
 let child=ValueError::new(ValueRefusalKind::WorkLimit,"nested original");
 let child_pointer=child.message.as_ptr();
 let nested=semio_framework_value::OriginalRetirementFailure::from_owned_parts(Some(primary),None,None,Some(Box::new(semio_framework_value::OriginalRetirementFailure::from_primary(child))),None);
 let outer_pointer=nested.primary().unwrap().message.as_ptr();
 let lifecycle:PluginLifecycleFailure=nested.into();
 let(job,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::OriginalTurnFailure::from(lifecycle));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let(lifecycle,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||PluginLifecycleFailure::from(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let PluginLifecyclePrimary::Retirement(original)=&lifecycle.primary else{panic!("retirement owner was flattened")};
 assert_eq!(original.primary().unwrap().message.as_ptr(),outer_pointer);assert_eq!(original.child().unwrap().primary().unwrap().message.as_ptr(),child_pointer);
 close(lifecycle,3);
}

/// 🧮️ A denied original owner turn consumes no work; admitted turns consume their actual selected work.
#[test]
fn original_quoted_typed_owner_denial_and_paid_work(){
 let stock=RetainedCloneGrant{maximum_items:32,maximum_copy_bytes:65536,..grant()};
 let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(OperationId(97534),Generation(19),stock,close_clock,100,10000);
 let cases=corpus();let denied_case=&cases["typedOwnerWork"][0];let paid_case=&cases["typedOwnerWork"][1];
 let mut source=Some(String::from("original material"));let pointer=source.as_ref().unwrap().as_ptr();let mut active=None;
 let denied=RetainedCloneGrant{maximum_items:0,..stock};
 caller.turn(denied,|cx|cx.with_original_retirement(|cx|{let before=cx.fuel_remaining();let demand=crate::bounded_ownership::plugin_typed_owner_demand(&source,&active,stock.maximum_copy_bytes)?;let work=crate::bounded_ownership::plugin_typed_owner_work_demand(&source,&active,stock.maximum_copy_bytes)?;let step=semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|crate::bounded_ownership::plugin_typed_owner_original(&mut source,&mut active,grant))?;assert_eq!(step.progress(),Default::default());assert_eq!(before-cx.fuel_remaining(),denied_case["fuelDebit"].as_u64().unwrap());assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);Ok(())})).unwrap();
 let mut observed_paid=false;
 for _ in 0..100000{if source.is_none()&&active.is_none(){break}caller.turn(stock,|cx|cx.with_original_retirement(|cx|{let before=cx.fuel_remaining();let birth=source.is_some()&&active.is_none();let demand=crate::bounded_ownership::plugin_typed_owner_demand(&source,&active,stock.maximum_copy_bytes)?;let work=crate::bounded_ownership::plugin_typed_owner_work_demand(&source,&active,stock.maximum_copy_bytes)?;semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|crate::bounded_ownership::plugin_typed_owner_original(&mut source,&mut active,grant))?;if birth{assert_eq!(before-cx.fuel_remaining(),paid_case["fuelDebit"].as_u64().unwrap())}observed_paid|=cx.fuel_remaining()<before;Ok(())})).unwrap();}
 assert!(source.is_none()&&active.is_none());assert!(observed_paid);caller.retire(stock);
}
