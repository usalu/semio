//! 🔬️ The actual original cursor preserves its source on work/deadline denial and reports physical effects once.
use semio_framework_job::{close_original_retained_step,StepContext,RetainedCloneGrant};
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
