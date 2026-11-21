use super::*;
use semio_framework_actor::RetainedTurnInput;
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;
fn binary(body:&[u8],maximum_pages:u8)->Vec<u8>{let mut bytes=b"\0asm\x01\0\0\0".to_vec();for(id,data)in[(1,vec![1,0x60,1,0x7f,1,0x7f]),(3,vec![1,0]),(5,vec![1,1,1,maximum_pages]),(7,vec![1,3,b'r',b'u',b'n',0,0]),(10,{let mut code=vec![1,body.len()as u8];code.extend_from_slice(body);code}),(11,vec![1,0,0x41,0,0x0b,6,b'a',b'b',b'c',b'd',b'e',b'f'])]{bytes.extend([id,data.len()as u8]);bytes.extend(data)}bytes}
fn clock()->Option<u64>{Some(1000)}
/// 💾️ Actual growth and bounded overlapping copies agree with Wasmtime and preserve per-turn physical receipts.
#[test]
fn original_memory_growth_and_bulk_copy_match_independent_vm_and_heap(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){let body:Vec<u8>=row["body"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect();let bytes=binary(&body,row["maximumPages"].as_u64().unwrap_or(2)as u8);let input=row["args"][0].as_i64().unwrap()as i32;let engine=wasmtime::Engine::default();let module=wasmtime::Module::new(&engine,&bytes).unwrap();let mut store=wasmtime::Store::new(&engine,());let oracle=wasmtime::Instance::new(&mut store,&module,&[]).unwrap();let expected=oracle.get_typed_func::<i32,i32>(&mut store,"run").unwrap().call(&mut store,input).unwrap();assert_eq!(i64::from(expected),row["expected"].as_i64().unwrap());super::super::control::tests::run_original_program(&bytes,input,expected,row["name"].as_str().unwrap());}}

#[test]
fn retained_linear_growth_reuses_paid_spare_without_exposing_partial_pages(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let row=fixture["cases"].as_array().unwrap().iter().find(|row|row["name"]=="repeated growth preserves original bytes and zeros the funded extension").unwrap();
 let body=row["body"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();
 let page=WASM_PAGE_BYTES;
 let module=Arc::new(CoreModule::parse(&binary(&body,row["maximumPages"].as_u64().unwrap()as u8)).unwrap());
 let stock=RetainedCloneGrant{maximum_items:10_000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64};
 for cancelled in [false,true]{
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(194),Generation(3),stock,clock,1_000_000,1_000_000);
  let mut epoch=1;
  let mut owner=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap().unwrap();
  for _ in 0..100{
   if owner.resources_ready(){break}
   epoch+=1;
   caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.advance(&module,cx)}).unwrap();
  }
  assert!(owner.resources_ready());
  epoch+=1;
  assert!(caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.begin_function_step(&module,0,&[Value::I32(1)],cx)}).unwrap());
  let scalar=|caller:&mut OriginalJobFixtureCaller,owner:&mut CoreInstanceOpening,epoch:&mut u64,grant:RetainedCloneGrant|{
   *epoch+=1;
   let(call,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|{
    cx.bind_original_source_epoch(*epoch)?;
    let input=RetainedTurnInput{grant:cx.retained_grant(),..RetainedTurnInput::borrow_original(cx)?};
    owner.advance_scalar(&module,OriginalGuestExecutionBudget{input,fuel:1,deadline_us:cx.deadline_us()},cx)
   }).unwrap());
   let receipt=caller.receipt();
   assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
   (call.result.unwrap(),call.execution.spent_fuel.unwrap(),receipt)
  };
  assert_eq!(scalar(&mut caller,&mut owner,&mut epoch,stock).1,1);
  let blocked=RetainedCloneGrant{maximum_capacity_bytes:page,..stock};
  let(status,used,receipt)=scalar(&mut caller,&mut owner,&mut epoch,blocked);
  assert_eq!((status,used,receipt),(CoreOriginalInvocationStatus::Pending,0,RetainedCloneProgress::default()));
  assert!(owner.memory_work.is_none());
  let mut admitted=0;
  let mut partial=false;
  for _ in 0..100{
   let(visible,capacity)={let memory=&owner.core.as_ref().unwrap().memories[0];(memory.bytes.len(),memory.bytes.capacity())};
   let in_place=matches!(owner.memory_work,Some(OriginalMemoryWork::Extend{..}));
   let replacing=matches!(owner.memory_work,Some(OriginalMemoryWork::Grow{..}));
   let grant=if in_place&&!partial{RetainedCloneGrant{maximum_copy_bytes:1024,..stock}}else{stock};
   let had_plan=owner.memory_work.is_some();
   let(status,_,receipt)=scalar(&mut caller,&mut owner,&mut epoch,grant);
   if !had_plan{
    if let Some(OriginalMemoryWork::Grow{new,length,..})=owner.memory_work.as_ref(){
     admitted+=1;
     assert_eq!(*length,visible+page);
     assert_eq!(new.capacity(),admitted*2*page);
     assert_eq!(receipt.retained_capacity_bytes,new.capacity());
    }
   }
   if replacing&&owner.memory_work.is_none(){assert_eq!(receipt.released_bytes,capacity);}
   if in_place{
    assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(0,0));
    if !partial{
     partial=true;
     assert_eq!(visible,3*page);
     assert_eq!(capacity,4*page);
     assert_eq!(owner.core.as_ref().unwrap().memories[0].bytes.len(),visible);
     assert!(matches!(owner.memory_work,Some(OriginalMemoryWork::Extend{initialized,..}) if initialized>0&&initialized<page));
     if cancelled{
      caller.cancel().cancel_now();
      let(status,used,receipt)=scalar(&mut caller,&mut owner,&mut epoch,stock);
      assert_eq!((status,used,receipt),(CoreOriginalInvocationStatus::Cancelled,0,RetainedCloneProgress::default()));
      assert_eq!(owner.core.as_ref().unwrap().memories[0].bytes.len(),visible);
      break
     }
    }
   }
   if status==CoreOriginalInvocationStatus::Complete{break}
  }
  assert_eq!(admitted,2);
  assert!(partial);
  if !cancelled{assert_eq!(owner.invocation_values(),Some(&[Value::I32(100)][..]));}
  caller.cancel().cancel_now();
  for _ in 0..100{
   if owner.terminal_is_empty(){break}
   epoch+=1;
   let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|{
    cx.bind_original_source_epoch(epoch)?;
    owner.close_step(&module,cx)
   }).unwrap());
   assert_eq!(heap.released_bytes,caller.receipt().released_bytes);
  }
  assert!(owner.terminal_is_empty());
  caller.retire(stock);
 }
}

#[test]
fn spare_linear_pages_respect_the_guest_and_physical_ceiling(){
 let page=WASM_PAGE_BYTES;
 let mut memory=MemoryState{bytes:Vec::with_capacity(2*page),maximum_pages:Some(3),memory64:false};
 assert_eq!(growth_capacity(&memory,3*page,4*page),3*page);
 memory.maximum_pages=Some(5);
 assert_eq!(growth_capacity(&memory,3*page,3*page+1),3*page);
 assert_eq!(growth_capacity(&memory,3*page,5*page),4*page);
}
