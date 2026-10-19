//! 🔬️ The actual original cursor preserves its source on work/deadline denial and reports physical effects once.
use semio_framework_job::{close_original_granted_step,close_original_nested_step,close_original_retained_step,close_with_self_funded_context,SelfFundedCloseCaller,InteractiveJobCloseStep,StepContext,RetainedCloneGrant};
use semio_framework_job::{Generation,OperationId,StepBudget,root_cancel_token};
use semio_framework_value::retirement::controlled::ControlledRetirement;

#[global_allocator]
static ORIGINAL_CLOSE_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

/// ⏱️ Original cleanup denial does not execute a callback or replace its source with a new wallet.
#[test]
fn original_close_context_denied_work_and_deadline_preserve_owned_source(){
 let source=vec![7u8;1799];let pointer=source.as_ptr();let mut owner=ControlledRetirement::new(source).unwrap_or_else(|_|panic!("original source retirement unsupported"));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};let mut sequence=0;let cancel=root_cancel_token();
 for budget in [StepBudget::new(0,12345,grant),StepBudget::new(1,0,grant)]{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(71),Generation(3),budget,cancel.clone(),||Some(1),&mut sequence,&mut receipt);let demand=owner.selected_demand(grant.maximum_copy_bytes).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_original_retained_step(&mut cx,demand,|grant|owner.step(grant)));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!(cx.retained_progress(),Default::default());assert_eq!(owner.original().unwrap().as_ptr(),pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
 let mut freed=0;let mut born=0;cancel.cancel_now();for turn in 0..100000{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(71),Generation(3),StepBudget::new(1,12345,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);assert!(cx.is_cancelled());let demand=owner.selected_demand(grant.maximum_copy_bytes).unwrap();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close_original_retained_step(&mut cx,demand,|grant|owner.step(grant)));let step=result.unwrap();assert_eq!(step.progress(),cx.retained_progress());assert!(cx.retained_progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(cx.retained_progress().retained_capacity_bytes,cx.retained_progress().released_bytes));assert_eq!(cx.fuel_remaining(),0);born+=heap.requested_bytes;freed+=heap.released_bytes;if owner.terminal_is_empty(){break}assert!(turn<99999,"same original source close did not complete");}assert_eq!(1799+born,freed);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

/// 🎫️ A granted close body sees the caller's remaining grant, its progress is debited once and its refusal keeps the whole error with its receipt.
#[test]
fn granted_close_body_is_debited_once_and_its_refusal_keeps_the_whole_error(){
 use semio_framework_value::retained_clone::RetainedCloneProgress;use semio_framework_value::{ValueError,ValueRefusalKind};
 let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};let mut sequence=0;let cancel=root_cancel_token();let mut receipt=Default::default();
 let mut cx=StepContext::new(OperationId(71),Generation(3),StepBudget::new(3,12345,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
 let progress=RetainedCloneProgress{copied_items:1,released_bytes:8,..Default::default()};let mut seen=0;
 let step=close_original_granted_step(&mut cx,|granted|{seen=granted.maximum_items;Ok(InteractiveJobCloseStep::Pending{progress})}).unwrap();
 assert_eq!((step,seen,cx.retained_progress(),cx.fuel_remaining()),(InteractiveJobCloseStep::Pending{progress},4,progress,2));
 let refused=RetainedCloneProgress{copied_items:1,..Default::default()};let error=ValueError::literal(ValueRefusalKind::InvariantViolated,"granted close refused").with_retained_progress(refused);
 let failure=close_original_granted_step(&mut cx,|granted|{seen=granted.maximum_items;Err(error)}).unwrap_err();
 assert_eq!((seen,failure.primary_value().map(|error|error.kind),failure.receipt_refusal.is_none(),cx.fuel_remaining()),(3,Some(ValueRefusalKind::InvariantViolated),true,1));assert_eq!(cx.retained_progress().copied_items,2);
 let blocked=close_original_granted_step(&mut cx,|_|Ok(InteractiveJobCloseStep::Blocked)).unwrap();assert_eq!((blocked,cx.fuel_remaining()),(InteractiveJobCloseStep::Blocked,0));
 let mut ran=false;let yielded=close_original_granted_step(&mut cx,|_|{ran=true;Ok(InteractiveJobCloseStep::Blocked)}).unwrap();
 assert_eq!((yielded,ran,cx.retained_progress().copied_items),(InteractiveJobCloseStep::Pending{progress:Default::default()},false,2));
}

/// 🪆️ A nested close body owes only the part of its reported step that its nested callers did not already post, and may not report less than they posted.
#[test]
fn nested_close_body_debits_only_the_remainder_its_nested_callers_did_not_post(){
 use semio_framework_value::retained_clone::RetainedCloneProgress;use semio_framework_value::ValueRefusalKind;
 let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};let mut sequence=0;let cancel=root_cancel_token();let mut receipt=Default::default();
 let mut cx=StepContext::new(OperationId(71),Generation(3),StepBudget::new(3,12345,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);
 let reported=RetainedCloneProgress{copied_items:2,released_bytes:16,..Default::default()};let posted=RetainedCloneProgress{copied_items:1,..Default::default()};
 let step=close_original_nested_step(&mut cx,|nested,_|{nested.consume_retained(posted)?;Ok(InteractiveJobCloseStep::Pending{progress:reported})}).unwrap();
 assert_eq!((step,cx.retained_progress(),cx.fuel_remaining()),(InteractiveJobCloseStep::Pending{progress:reported},reported,2));
 let failure=close_original_nested_step(&mut cx,|nested,_|{nested.consume_retained(posted)?;Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}).unwrap_err();
 assert_eq!(failure.primary_value().map(|error|error.kind),Some(ValueRefusalKind::InvariantViolated));
}

/// 🧪️ A self-funded close context lends exactly its grant, carries the body's value back and keeps the body's failure whole.
#[test]
fn self_funded_close_context_lends_exactly_its_grant_and_returns_the_body_outcome(){
 use semio_framework_value::{ValueError,ValueRefusalKind};
 let grant=RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:3,maximum_capacity_bytes:5,maximum_release_bytes:7,maximum_depth:11};
 let value=close_with_self_funded_context(grant,|cx|{let lent=cx.retained_grant();assert_eq!((lent.maximum_items,lent.maximum_copy_bytes,lent.maximum_capacity_bytes,lent.maximum_release_bytes,lent.maximum_depth),(2,3,5,7,11));assert!(!cx.should_yield());Ok(41+1)}).unwrap();assert_eq!(value,42);
 let mut caller=SelfFundedCloseCaller::new();let (first,second)=(caller.turn(grant,|cx|Ok(cx.retained_grant().maximum_depth)).unwrap(),caller.turn(grant,|cx|Ok(cx.retained_grant().maximum_depth)).unwrap());assert_eq!((first,second),(11,11));
 let failure=close_with_self_funded_context(grant,|_|Err::<(),_>(ValueError::literal(ValueRefusalKind::InvariantViolated,"self funded body refused").into())).unwrap_err();assert_eq!(failure.primary_value().map(|error|error.kind),Some(ValueRefusalKind::InvariantViolated));
}
