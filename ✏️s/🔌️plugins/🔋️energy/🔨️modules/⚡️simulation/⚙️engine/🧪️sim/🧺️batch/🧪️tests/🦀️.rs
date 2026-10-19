use super::*;
use crate::testing::OriginalEnergyCaller;
#[test]
fn original_batch_slots_and_foreign_roots_keep_exact_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){
  let operation=Operation::new(allocate_operation_id(),RevisionId(82),Generation(19),71);
  let mut original=OriginalEnergyCaller::new(operation.operation,operation.generation);
  let mut input=Some(EnergyJob::new(operation,crate::model::test_model_single_zone(),SimulationConfig::default()).unwrap());let mut output=None;
  if !case["pristine"].as_bool().unwrap(){original.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|Ok(input.as_mut().unwrap().step_input_identity(cx))).unwrap().unwrap();}
  let pointer=input.as_ref().unwrap().model.zones.as_ptr();let quote=size_of::<EnergyBatchRun>()+size_of::<Option<EnergyJob>>();let mut grant=ENERGY_BATCH_GRANT;grant.maximum_copy_bytes=quote;
  match case["grant"].as_str().unwrap(){"no-items"=>grant.maximum_items=0,"short-copy"=>grant.maximum_copy_bytes=quote-1,"no-depth"=>grant.maximum_depth=0,_=>{}}
  if case["cancelled"].as_bool().unwrap(){original.cancel();}
  let fuel=if case["work"].as_bool().unwrap(){4}else{0};
  let mut foreign=None;if !case["matchingIds"].as_bool().unwrap(){foreign=Some(OriginalEnergyCaller::new(operation.operation,Generation(20)));}
  let caller=foreign.as_mut().unwrap_or(&mut original);
  let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,fuel,u64::MAX,|cx|EnergyBatchRun::admit(&mut input,&mut output,cx)));
  let actual=match result{Ok(true)=>"admitted",Ok(false)=>"pending",Err(_)=>"refused"};assert_eq!(actual,case["expected"].as_str().unwrap(),"{}",case["id"]);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  if let Some(foreign)=foreign{foreign.finish();}
  if let Some(mut batch)=output{
   assert!(input.is_none());assert_eq!(batch.job.model.zones.as_ptr(),pointer);
   let mut foreign=OriginalEnergyCaller::new(operation.operation,operation.generation);
   let before=(batch.job.stage,batch.job.input_identity.processed_bytes());
   let (denied,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(ENERGY_BATCH_GRANT,4,u64::MAX,|cx|batch.step(cx)));
   assert!(denied.is_err());assert_eq!((batch.job.stage,batch.job.input_identity.processed_bytes()),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   assert!(foreign.turn(ENERGY_BATCH_GRANT,4,u64::MAX,|cx|batch.close_step(cx)).is_err());foreign.finish();
   for _ in 0..1_000_000{if batch.terminal_is_empty(){break}original.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|cx.with_original_retirement(|cx|batch.close_step(cx))).unwrap();}assert!(batch.terminal_is_empty());let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(batch));assert_eq!(heap.released_bytes,0);
  }else{assert_eq!(input.as_ref().unwrap().model.zones.as_ptr(),pointer);original.close_job(input.as_mut().unwrap());drop(input);}
  original.finish();
 }
}
