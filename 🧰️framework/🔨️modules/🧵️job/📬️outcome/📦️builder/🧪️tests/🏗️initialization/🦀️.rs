//! 🏗️ Every original native header prefix retains its live backing and exact paid progress.
use semio_framework_job::*;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn clock()->Option<u64>{Some(1000)}
fn observe<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}
#[test]
fn original_empty_payload_header_initialization_preserves_zero_copy_at_every_native_prefix(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let row=&fixture["initialization"];let turns=row["turns"].as_u64().unwrap()as usize;let bindings=row["bindingCount"].as_u64().unwrap()as usize;let per_turn=row["bindingsPerTurn"].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};let stock=RetainedCloneGrant{maximum_items:256,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
 for cut in 0..=turns{
  let(mut caller,birth)=observe(||OriginalJobFixtureCaller::admit(OperationId(99209),Generation(17),stock,clock,1000000,10000));assert_eq!(birth.0,caller.born_bytes());let mut builder=RetainedPayloadBuilder::new(JobPayloadStream::Preview);let identity=builder.native_header_identity();
  for turn in 0..cut{
   for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let before=builder.initialized_bindings();let(answer,heap)=observe(||caller.turn(denied,|cx|Ok(builder.advance_initialization(cx)?)));assert!(!answer.unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(builder.initialized_bindings(),before);}
   let(answer,heap)=observe(||caller.turn(grant,|cx|{let complete=builder.advance_initialization(cx)?;cx.consume_fuel(1);Ok(complete)}));assert_eq!(answer.unwrap(),turn+1==turns);assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!(builder.initialized_bindings(),((turn+1)*per_turn).min(bindings));assert_eq!(builder.native_header_identity(),identity);
  }
  assert_eq!(builder.is_initialized(),cut==turns);if let Some(original)=builder.original(){assert!(original.terminal_is_empty());}assert_eq!(builder.native_header_identity(),identity);
  for close in 0..2{if builder.terminal_is_empty(){break}let(answer,heap)=observe(||caller.turn(grant,|cx|{let done=builder.close_step(cx)?;cx.consume_fuel(1);Ok(done)}));answer.unwrap();assert_eq!(heap,(0,0));assert_eq!(caller.receipt().copied_bytes,0);assert!(close<1);}assert!(builder.terminal_is_empty());assert_eq!(observe(||drop(builder)).1,(0,0));let(released,heap)=observe(||caller.retire(stock));assert_eq!(released,birth.0);assert_eq!(heap,(0,birth.0));eprintln!("[DEBUG] actual Original native header prefix={cut} exactBindings={bindings} perTurn={per_turn} identity={identity:?} copy0capacity0release0 sameStock paidClose={released}");
 }
}
