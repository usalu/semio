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

#[test]
fn packet_page_birth_waits_for_separate_original_turns() {
    use semio_framework_job_payload::{JobPayloadStep, JOB_PAYLOAD_PAGE_BYTES};
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/page-birth.json")).unwrap();
    for case in law["cases"].as_array().unwrap() {
        let operation = Operation::new(allocate_operation_id(), RevisionId(82), Generation(19), 71);
        let mut original = OriginalEnergyCaller::new(operation.operation, operation.generation);
        let mut job = EnergyJob::new(operation, crate::model::test_model_single_zone(), SimulationConfig::default()).unwrap();
        job.start_wire(EnergyWireKind::Preview, 1, JobPayloadStream::Preview).unwrap();
        for _ in 0..128 {
            let build = job.wire_build.as_ref().unwrap();
            if build.builder.as_ref().is_some_and(|builder| builder.is_initialized()) { break; }
            assert_eq!(original.turn(ENERGY_BATCH_GRANT, 8, u64::MAX, |cx| Ok(job.step_wire_build(cx))).unwrap(), Ok(false));
        }
        let builder = job.wire_build.as_ref().unwrap().builder.as_ref().unwrap();
        assert!(builder.is_initialized());
        assert_eq!(builder.staged_page_len(), None);
        for (index, turn) in case["turns"].as_array().unwrap().iter().enumerate() {
            let mut grant = ENERGY_BATCH_GRANT;
            let used = match turn["grant"].as_str().unwrap() {
                "used" => true,
                "short-capacity" => {
                    grant.maximum_capacity_bytes = JOB_PAYLOAD_PAGE_BYTES - 1;
                    false
                }
                "full" => false,
                other => panic!("Unknown packet page grant {other}"),
            };
            let advance = turn["advance"].as_bool().unwrap();
            let held = turn["held"].as_bool().unwrap();
            let staged = turn["staged"].as_bool().unwrap();
            let capacity = if staged { JOB_PAYLOAD_PAGE_BYTES } else { 0 };
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
                original.turn(grant, 8, u64::MAX, |cx| {
                    if used { cx.grant_payload_page(); }
                    let before = cx.retained_progress();
                    let fuel = cx.fuel_remaining();
                    let step = job.step_wire_build(cx);
                    let after = cx.retained_progress();
                    Ok((step, fuel - cx.fuel_remaining(), after.copied_items - before.copied_items,
                        after.retained_capacity_bytes - before.retained_capacity_bytes,
                        cx.payload_ledger().process_share_bytes(), cx.payload_page_granted()))
                })
            });
            assert_eq!(step.unwrap(), (Ok(false), u64::from(advance), usize::from(advance), capacity,
                if held { JOB_PAYLOAD_PAGE_BYTES } else { 0 }, used || advance), "{} turn {index}", case["id"]);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (capacity, 0));
            let build = job.wire_build.as_ref().unwrap();
            assert_eq!(build.field, 0);
            assert_eq!(build.builder.as_ref().unwrap().staged_page_len(), staged.then_some(0));
        }
        original.close_job(&mut job);
        drop(job);
        original.finish();
    }
}
