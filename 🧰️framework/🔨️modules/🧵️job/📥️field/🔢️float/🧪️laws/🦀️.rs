//! 🧪️ The real original caller bounds float-field opportunity, cancellation and deadline gates.
use semio_framework_value_testing_allocator as _;
use super::*;
use crate::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalFailureContext,OperationId,Generation,close_original_effect_step};
use std::sync::atomic::{AtomicU64,Ordering};
static CLOCK:AtomicU64=AtomicU64::new(1000);
fn clock()->Option<u64>{Some(CLOCK.load(Ordering::Relaxed))}
fn native<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=observe_heap(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
fn caller()->OriginalJobFixtureCaller{CLOCK.store(1000,Ordering::Relaxed);OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128*1024*1024,maximum_capacity_bytes:256*1024*1024,maximum_release_bytes:512*1024*1024,maximum_depth:4096},clock,1500,32768)}
fn close(mut work:OriginalFloatFieldWork,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant){
 work.begin_close();for turn in 0..100{if work.terminal_is_empty(){break}native(caller,decoder,g,|cx|cx.with_original_retirement(|cx|{let d=work.close_demands(g.maximum_copy_bytes)?;close_original_effect_step(cx,d,|effect|{semio_framework_value::RetainedWorkPoll::Complete(work.close_step(effect))})?;Ok(())}));assert!(turn+1<100)}assert!(work.terminal_is_empty());drop(work);
 while !decoder.terminal_is_empty(){caller.turn(g,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(g.maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();}
}
#[test]
fn original_float_field_uses_same_200_call_fuel_stock_and_cancel_deadline_authority(){
 let long=format!("1{}5e-4097","0".repeat(4096));let words=["+01.0","-0","NaN","1.2.3",long.as_str()];
 for(items,copy,calls)in[(1,4096,10000),(512,32768,200)]{
  for source in words{
   let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let g=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:4096};native(&mut caller,&mut decoder,g,|_|Ok(()));let mut work=OriginalFloatFieldWork::new();let mut output=None;let pointer=source.as_ptr();
   for turn in 0..calls{let before=(work.test_phase(),work.bytes_read());native(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..g},|cx|work.step(source,cx));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!((work.test_phase(),work.bytes_read()),before);output=native(&mut caller,&mut decoder,g,|cx|work.step(source,cx));assert_eq!(source.as_ptr(),pointer);if output.is_some(){break}assert!(turn+1<calls,"float field exhausted original {calls} calls")}
   assert_eq!(output.map(|v|v.map(f64::to_bits)),Some(source.parse::<f64>().ok().map(f64::to_bits)));close(work,&mut caller,&mut decoder,g);caller.retire(g);
  }
 }
 for phase in 0..=3{for deadline in [false,true]{
  let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:4096};native(&mut caller,&mut decoder,g,|_|Ok(()));let mut work=OriginalFloatFieldWork::new();
  for turn in 0..10000{if work.test_phase()==phase{break}assert!(native(&mut caller,&mut decoder,g,|cx|work.step(&long,cx)).is_none());assert!(turn+1<10000)}
  let before=(work.test_phase(),work.bytes_read());if deadline{CLOCK.store(1500,Ordering::Relaxed)}else{caller.cancel().cancel_now()}
  assert!(native(&mut caller,&mut decoder,g,|cx|work.step(&long,cx)).is_none());assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!((work.test_phase(),work.bytes_read()),before);close(work,&mut caller,&mut decoder,g);caller.retire(g);
 }}
 let source="1.5".to_owned();let other=source.clone();let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:4096};native(&mut caller,&mut decoder,g,|_|Ok(()));let mut work=OriginalFloatFieldWork::new();assert!(native(&mut caller,&mut decoder,g,|cx|work.step(&source,cx)).is_none());let failure=native(&mut caller,&mut decoder,g,|cx|Ok(work.step(&other,cx))).unwrap_err();assert_eq!(failure.primary_value().unwrap().kind,ValueRefusalKind::InvariantViolated);close(work,&mut caller,&mut decoder,g);caller.retire(g);
}
#[test]
fn original_float_word_completes_under_actual_one_fuel_tool_frames(){
 fn constant_clock()->Option<u64>{Some(1000)}
 let source=format!("1{}5e-769","0".repeat(768));let expected=source.parse::<f64>().ok().map(f64::to_bits);let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:128*1024*1024,maximum_capacity_bytes:256*1024*1024,maximum_release_bytes:512*1024*1024,maximum_depth:4096},constant_clock,1500,32768);let mut decoder=OriginalJobDecodeOwner::empty();let g=RetainedCloneGrant{maximum_items:512,maximum_copy_bytes:32768,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:4096};native(&mut caller,&mut decoder,g,|_|Ok(()));let mut work=OriginalFloatFieldWork::new();let mut output=None;
 for _ in 0..200{output=native(&mut caller,&mut decoder,g,|cx|{let deadline=cx.original_absolute_deadline_us();cx.with_original_work_loan(1,deadline,|same|work.step(&source,same))});if output.is_some(){break}}
 close(work,&mut caller,&mut decoder,g);caller.retire(g);assert_eq!(output.map(|v|v.map(f64::to_bits)),Some(expected),"numeric word did not complete under unchanged actual one-fuel tool frames");
}
fn observe_heap<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
