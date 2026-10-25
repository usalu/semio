//! 🏗️ Each actual scalar prefix and final header remains under the same finite original source.
use semio_framework_job::*;
use semio_framework_value::RetirementDemand;
#[cfg(test)] use semio_framework_job_payload::payload_builder::RetainedPayloadBuilder;
#[cfg(test)] use semio_framework_job_payload::payload_pages_and_ledger::JobPayloadStream;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn clock()->Option<u64>{Some(1000)}
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);assert!(!heap.overflowed);(value,(heap.requested_bytes,heap.released_bytes))}
fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
#[test]
fn original_empty_payload_header_initialization_preserves_every_scalar_and_final_header_prefix(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let row=&fixture["initialization"];let turns=row["turns"].as_u64().unwrap()as usize;let bindings=row["bindingCount"].as_u64().unwrap()as usize;let stock=RetainedCloneGrant{maximum_items:10_000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 for cut in 0..=turns{
  let(mut caller,birth)=observe(||OriginalJobFixtureCaller::admit(OperationId(99209),Generation(17),stock,clock,1000000,10000));assert_eq!(birth.0,caller.born_bytes());
  let mut storage=std::mem::MaybeUninit::<RetainedPayloadBuilder>::uninit();let metadata=RetainedPayloadBuilder::empty_carrier_copy_bytes();
  caller.turn(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:metadata,..stock},|cx|{cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:metadata,..Default::default()})?;unsafe{RetainedPayloadBuilder::initialize_empty_at(storage.as_mut_ptr(),JobPayloadStream::Preview)}cx.consume_fuel(1);Ok(())}).unwrap();
  let builder=unsafe{storage.assume_init_mut()};let identity=builder.native_header_identity();
  for turn in 0..cut{
   let demand=builder.initialization_demand();assert!(demand.copy_bytes>0&&demand.copy_bytes<=row["policy"]["copy"].as_u64().unwrap()as usize);
   let grant=exact(demand);
   for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
    let before=builder.initialized_bindings();let(answer,heap)=observe(||caller.turn(denied,|cx|Ok(builder.advance_initialization(cx)?)));assert!(!answer.unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(builder.initialized_bindings(),before);
   }
   let(answer,heap)=observe(||caller.turn(grant,|cx|{let complete=builder.advance_initialization(cx)?;cx.consume_fuel(1);Ok(complete)}));assert_eq!(answer.unwrap(),turn+1==turns);assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});assert_eq!(builder.initialized_bindings(),(turn+1).min(bindings));assert_eq!(builder.native_header_identity(),identity);
  }
  assert_eq!(builder.is_initialized(),cut==turns);assert_eq!(builder.native_header_identity(),identity);if cut==bindings{assert!(!builder.is_initialized());}
  if !builder.is_initialized(){
   let before=builder.initialized_bindings();let demand=builder.initialization_demand();
   let(answer,heap)=observe(||caller.turn(exact(demand),|cx|cx.with_original_work_loan(0,u64::MAX,|cx|Ok(builder.advance_initialization(cx)?))));assert!(!answer.unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(builder.initialized_bindings(),before);
   caller.cancel().cancel_now();let(answer,heap)=observe(||caller.turn(exact(demand),|cx|Ok(builder.advance_initialization(cx)?)));assert!(!answer.unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(builder.initialized_bindings(),before);
  }
  while !builder.terminal_is_empty(){let demand=builder.retirement_demands().unwrap();let(answer,heap)=observe(||caller.turn(exact(demand),|cx|close_original_retained_step(cx,demand,|grant|builder.close_step_granted(grant))));let receipt=answer.unwrap().progress();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(receipt.copied_bytes,demand.copy_bytes);}
  assert_eq!(observe(||unsafe{storage.assume_init_drop()}).1,(0,0));let(released,heap)=observe(||caller.retire(stock));assert_eq!(released,birth.0);assert_eq!(heap,(0,birth.0));
 }
}
