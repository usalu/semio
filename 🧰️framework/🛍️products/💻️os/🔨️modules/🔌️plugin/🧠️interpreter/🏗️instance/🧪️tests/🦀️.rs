//! 🏗️ The genuine fixture issuer opens actual Core resources under physical grants and compares them with Wasmtime.
use super::*;
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::RetainedCloneGrant;
use std::sync::atomic::{AtomicU64,Ordering};
static NOW:AtomicU64=AtomicU64::new(1000);
fn clock()->Option<u64>{Some(NOW.load(Ordering::Acquire))}
fn binary()->Vec<u8>{let mut bytes=b"\0asm\x01\0\0\0".to_vec();for(id,section)in [(4,vec![1,0x70,0,3]),(5,vec![1,0,1]),(6,vec![1,0x7f,0,0x41,7,0x0b]),(7,vec![3,6,b'm',b'e',b'm',b'o',b'r',b'y',2,0,5,b't',b'a',b'b',b'l',b'e',1,0,5,b'v',b'a',b'l',b'u',b'e',3,0]),(11,vec![1,0,0x41,5,0x0b,2,b'2',b'0'])]{bytes.extend([id,section.len()as u8]);bytes.extend(section)}bytes}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64}}
fn oracle(bytes:&[u8])->(wasmtime::Store<()>,wasmtime::Instance){let engine=wasmtime::Engine::default();let module=wasmtime::Module::new(&engine,bytes).unwrap();let mut store=wasmtime::Store::new(&engine,());let instance=wasmtime::Instance::new(&mut store,&module,&[]).unwrap();(store,instance)}
fn observed<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
fn scalar_input(cx:&StepContext<'_>)->Result<semio_framework_actor::RetainedTurnInput,ValueError>{
 use semio_framework_actor::RetainedTurnInput;
 Ok(RetainedTurnInput{grant:cx.retained_grant(),..RetainedTurnInput::borrow_original(cx)?})
}
fn observed_turn<T>(caller:&mut OriginalJobFixtureCaller,epoch:&mut u64,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<T,semio_framework_job::OriginalTurnFailure>)->(T,(usize,usize)){*epoch+=1;observed(||caller.turn(grant,|cx|{cx.bind_original_source_epoch(*epoch)?;run(cx)}).unwrap())}

#[test]
fn inline_globals_require_the_actual_funded_backing_return(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/globals-close.json")).unwrap();
 let count=usize::try_from(fixture["globals"].as_u64().unwrap()).unwrap();
 let value=u8::try_from(fixture["value"].as_u64().unwrap()).unwrap();
 let mut bytes=b"\0asm\x01\0\0\0".to_vec();
 bytes.extend([6,u8::try_from(1+5*count).unwrap(),u8::try_from(count).unwrap()]);
 for _ in 0..count{bytes.extend([0x7f,0,0x41,value,0x0b]);}
 bytes.extend([7,9,1,5,b'v',b'a',b'l',b'u',b'e',3,0]);
 let (mut store,instance)=oracle(&bytes);
 let expected=instance.get_global(&mut store,"value").unwrap().get(&mut store).i32().unwrap();
 let module=Arc::new(CoreModule::parse(&bytes).unwrap());let stock=grant();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(796),Generation(1),stock,clock,100_000_000,1_000_000);
 let mut epoch=0;
 let(owner,_)=observed_turn(&mut caller,&mut epoch,stock,|cx|CoreInstanceOpening::begin(&module,cx));
 let mut owner=owner.unwrap();
 for _ in 0..256{
  if observed_turn(&mut caller,&mut epoch,stock,|cx|owner.advance(&module,cx)).0{break}
 }
 let core=owner.resources().unwrap();
 assert_eq!(core.globals.len(),count);
 assert!(core.globals.iter().all(|global|global.value==Value::I32(expected)));
 let backing=bytes_for::<GlobalState>(core.globals.capacity()).unwrap();
 let items=usize::try_from(fixture["returnItems"].as_u64().unwrap()).unwrap();
 caller.cancel().cancel_now();
 let denied=RetainedCloneGrant{maximum_items:items,maximum_release_bytes:backing-1,..stock};
 assert_eq!(observed_turn(&mut caller,&mut epoch,denied,|cx|owner.close_step(&module,cx)),(false,(0,0)));
 assert_eq!(caller.receipt(),Default::default());
 assert_eq!(owner.core.as_ref().unwrap().globals.len(),count);
 let funded=RetainedCloneGrant{maximum_release_bytes:backing,..denied};
 assert_eq!(observed_turn(&mut caller,&mut epoch,funded,|cx|owner.close_step(&module,cx)),(false,(0,backing)));
 assert_eq!(caller.receipt().copied_items,items);
 assert_eq!(caller.receipt().released_bytes,backing);
 assert_eq!(owner.core.as_ref().unwrap().globals.capacity(),0);
 for _ in 0..32{
  if observed_turn(&mut caller,&mut epoch,stock,|cx|owner.close_step(&module,cx)).0{break}
 }
 assert!(owner.terminal_is_empty());
 caller.retire(stock);
}

#[test]
fn prepared_original_core_opening_uses_exact_image_and_private_paid_storage(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/prepared-instance.json")).unwrap();
 let mut bytes=b"\0asm\x01\0\0\0".to_vec();
 for row in fixture["sections"].as_array().unwrap(){
  let payload=row[1].as_array().unwrap();
  bytes.extend([row[0].as_u64().unwrap()as u8,payload.len()as u8]);
  bytes.extend(payload.iter().map(|byte|byte.as_u64().unwrap()as u8));
 }
 let module=Arc::new(CoreModule::parse(&bytes).unwrap());
 assert!(PreparedCoreInstance::seal(CoreInstance::instantiate(Arc::clone(&module)).unwrap()).is_err());
 let prepare=||{
  let mut instance=CoreInstance::instantiate(Arc::clone(&module)).unwrap();
  assert!(matches!(instance.step(500,StepControl::default()),CoreStepOutcome::Complete{..}));
  PreparedCoreInstance::seal(instance).unwrap()
 };
 let image=prepare();
 let other=prepare();
 let wrong_module=Arc::new(CoreModule::parse(&bytes).unwrap());
 let stock=grant();
 let slice=RetainedCloneGrant{maximum_copy_bytes:1024,maximum_capacity_bytes:WASM_PAGE_BYTES,..stock};
 let offset=fixture["offset"].as_u64().unwrap()as usize;
 let expected=serde_json::from_value::<Vec<u8>>(fixture["bytes"].clone()).unwrap();
 for (index,cancel) in [false,false,true].into_iter().enumerate(){
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(867+index as u64),Generation(5),stock,clock,100_000,100_000);
  let mut epoch=0;
  let (denied,heap)=observed_turn(&mut caller,&mut epoch,slice,|cx|Ok(CoreInstanceOpening::begin_prepared(&wrong_module,&image,cx).is_err()));
  assert!(denied&&heap==(0,0));
  let (owner,heap)=observed_turn(&mut caller,&mut epoch,slice,|cx|CoreInstanceOpening::begin_prepared(&module,&image,cx));
  assert_eq!(heap,(0,0));
  let mut owner=owner.unwrap();
  let (denied,heap)=observed_turn(&mut caller,&mut epoch,slice,|cx|Ok(owner.advance_prepared(&module,&other,cx).is_err()));
  assert!(denied&&heap==(0,0));
  let mut born=0;
  let mut denied=false;
  for _ in 0..200{
   if owner.resources_ready(){break}
   let (ready,heap)=observed_turn(&mut caller,&mut epoch,slice,|cx|owner.advance_prepared(&module,&image,cx));
   assert_eq!(heap,(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));
   born+=heap.0;
   if owner.phase==Phase::Memories(0)&&owner.core.as_ref().unwrap().memories.first().is_some_and(|memory|memory.bytes.capacity()==0)&&!denied{
    let too_small=RetainedCloneGrant{maximum_capacity_bytes:WASM_PAGE_BYTES-1,..slice};
    let (ready,heap)=observed_turn(&mut caller,&mut epoch,too_small,|cx|owner.advance_prepared(&module,&image,cx));
    assert!(!ready&&owner.resources().is_none()&&heap==(0,0));
    assert_eq!(caller.receipt(),RetainedCloneProgress::default());
    denied=true;
   }
   if cancel&&owner.core.as_ref().unwrap().memories.first().is_some_and(|memory|!memory.bytes.is_empty()){
    assert!(!ready&&owner.resources().is_none());
    caller.cancel().cancel_now();
    let (ready,heap)=observed_turn(&mut caller,&mut epoch,slice,|cx|owner.advance_prepared(&module,&image,cx));
    assert!(!ready&&heap==(0,0));
    break
   }
  }
  assert!(denied);
  if !cancel{
   let core=owner.resources().unwrap();
   assert!(!core.active());
   assert_eq!(core.next_host_call,1);
   assert_eq!(&core.memory(0).unwrap()[offset..offset+expected.len()],expected.as_slice());
   assert_eq!(core.tables[0].values,[Some(2),Some(2)]);
   assert_eq!(core.globals[0].value,Value::I32(fixture["global"].as_i64().unwrap()as i32));
   assert_eq!(core.data,serde_json::from_value::<Vec<Option<Vec<u8>>>>(fixture["dataSegments"].clone()).unwrap());
   let expected_elements=serde_json::from_value::<Vec<Option<Vec<Option<u32>>>>>(fixture["elementSegments"].clone()).unwrap();
   assert_eq!(core.elements,expected_elements);
   let mut copied=0;
   let (finished,heap)=observed_turn(&mut caller,&mut epoch,slice,|cx|owner.write_original_memory_step(&module,0,offset,&[99],&mut copied,cx));
   assert!(finished&&heap==(0,0));
   assert_eq!(owner.resources().unwrap().memory(0).unwrap()[offset],99);
   assert_eq!(image.memories[0].bytes[offset],expected[0]);
  }
  caller.cancel().cancel_now();
  let mut released=0;
  for _ in 0..100{
   if owner.terminal_is_empty(){break}
   let (_,heap)=observed_turn(&mut caller,&mut epoch,stock,|cx|owner.close_step(&module,cx));
   assert_eq!(heap,(caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes));
   released+=heap.1;
  }
  assert!(owner.terminal_is_empty());
  assert_eq!(born,released);
  caller.retire(stock);
 }
}
#[test]
fn original_core_opening_phases_actual_storage_and_paid_cancelled_retirement(){
 NOW.store(1000,Ordering::Release);let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../🏗️instance/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let bytes=binary();let module=Arc::new(CoreModule::parse(&bytes).unwrap());let(mut store,instance)=oracle(&bytes);let oracle_memory=instance.get_memory(&mut store,"memory").unwrap().data(&store).to_vec();let oracle_table=instance.get_table(&mut store,"table").unwrap().size(&store);let oracle_global=instance.get_global(&mut store,"value").unwrap().get(&mut store).i32().unwrap();
 let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock,clock,100000,100000);let mut epoch=1;let(owner,heap)=observed(||caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap());assert_eq!(heap,(0,0));let mut owner=owner.unwrap();let mut born=0;let mut denied=false;let phase_grant=RetainedCloneGrant{maximum_copy_bytes:1024,maximum_capacity_bytes:65535,..stock};
 for _ in 0..200{let(done,heap)=observed_turn(&mut caller,&mut epoch,phase_grant,|cx|{owner.advance(&module,cx)});assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),heap);born+=heap.0;assert!(!done);if let Some(core)=owner.core.as_ref(){if core.memories.first().is_some_and(|memory|memory.bytes.capacity()==0)&&owner.phase==Phase::Memories(0){let before=core.memories[0].bytes.as_ptr();let(done,heap)=observed_turn(&mut caller,&mut epoch,phase_grant,|cx|{owner.advance(&module,cx)});assert!(!done);assert_eq!(heap,(0,0));assert_eq!(owner.core.as_ref().unwrap().memories[0].bytes.as_ptr(),before);denied=true;break}}}assert!(denied);
 let funded=RetainedCloneGrant{maximum_copy_bytes:1024,maximum_capacity_bytes:65536,..stock};let mut phases=0;while !owner.resources_ready(){let(done,heap)=observed_turn(&mut caller,&mut epoch,funded,|cx|{owner.advance(&module,cx)});assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),heap);born+=heap.0;phases+=1;assert!(phases<200);if done{break}}
 let core=owner.resources().unwrap();assert_eq!(core.memory(0).unwrap(),oracle_memory);assert_eq!(core.tables[0].values.len()as u64,oracle_table);assert_eq!(core.globals[0].value,Value::I32(oracle_global));assert_eq!(&core.memory(0).unwrap()[fixture["activeOffset"].as_u64().unwrap()as usize..7],fixture["activeData"].as_str().unwrap().as_bytes());assert!(!core.active());assert!(phases>=64);caller.cancel().cancel_now();epoch+=1;assert!(caller.turn(funded,|cx|{cx.bind_original_source_epoch(epoch)?;owner.advance(&module,cx)}).unwrap());let mut released=0;for _ in 0..64{if owner.terminal_is_empty(){break}let(_,heap)=observed_turn(&mut caller,&mut epoch,stock,|cx|{owner.close_step(&module,cx)});assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),heap);released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(born,released);assert_eq!(observed(||drop(owner)).1,(0,0));caller.retire(stock);
}


#[test]
fn original_core_partial_cancel_and_foreign_source_preserve_actual_retained_owner(){
 NOW.store(1000,Ordering::Release);let module=Arc::new(CoreModule::parse(&binary()).unwrap());let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock,clock,100000,100000);let mut epoch=1;let mut owner=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap().unwrap();let mut born=0;for _ in 0..16{let(_,heap)=observed_turn(&mut caller,&mut epoch,stock,|cx|{owner.advance(&module,cx)});assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),heap);born+=heap.0;if owner.core.as_ref().unwrap().memories.first().is_some_and(|memory|memory.bytes.capacity()==65536){break}}let pointer=owner.core.as_ref().unwrap().memories[0].bytes.as_ptr();assert_eq!(owner.core.as_ref().unwrap().memories[0].bytes.len(),0);assert!(!owner.resources_ready());let mut foreign=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock,clock,100000,100000);let(result,heap)=observed(||foreign.turn(stock,|cx|{cx.bind_original_source_epoch(epoch+1)?;owner.advance(&module,cx)}));assert!(result.is_err());assert_eq!(heap,(0,0));foreign.retire(stock);caller.cancel().cancel_now();let(result,heap)=observed_turn(&mut caller,&mut epoch,stock,|cx|{owner.advance(&module,cx)});assert!(!result);assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),RetainedCloneProgress::default());assert_eq!(owner.core.as_ref().unwrap().memories[0].bytes.as_ptr(),pointer);assert_eq!(owner.core.as_ref().unwrap().memories[0].bytes.len(),0);let mut released=0;for _ in 0..80{if owner.terminal_is_empty(){break}let(_,heap)=observed_turn(&mut caller,&mut epoch,stock,|cx|{owner.close_step(&module,cx)});assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),heap);released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(born,released);assert_eq!(observed(||drop(owner)).1,(0,0));caller.retire(stock);
}



