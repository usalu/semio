use super::*;
#[test]
fn source_constructor_records_actual_fixed_birth_and_keeps_one_bank_through_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut remaining:RetainedCloneGrant=serde_json::from_value(law["stock"].clone()).unwrap();
 let mut cpu=law["maximumCpuWork"].as_u64().unwrap();let mut allocation_actual=None;
 let oracle=std::sync::Arc::new(law["text"].as_str().unwrap().to_owned());
 let original=law["text"].as_str().unwrap().to_owned();let pointer=original.as_ptr();let input_bytes=original.capacity();let mut admission=RetainedCloneSource::admission_owned(original,());let d=admission.next_demand().unwrap();
 let g=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:d.capacity_bytes,maximum_depth:d.depth,..Default::default()};
 for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g},RetainedCloneGrant{maximum_depth:0,..g}]{
  let(turn,heap)=allocation_testing::observe_retirement_allocations(||admission.advance(&mut crate::RetainedAllocationEffect::new(denied,cpu,&mut allocation_actual)));assert_eq!(turn.work,law["expected"]["deniedCpu"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert!(allocation_actual.is_none());assert_eq!(admission.original_authority().unwrap().as_ptr(),pointer);
 }
 let(mut born,mut freed,mut birth_work)=(0,0,0u64);let mut output=None;
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if admission.terminal_is_empty(){break}let g=RetainedCloneGrant{maximum_items:remaining.maximum_items.min(1),..remaining};let(turn,heap)=allocation_testing::observe_retirement_allocations(||if admission.phase()=="ready"{admission.take_output(&mut output,g)}else{admission.advance(&mut crate::RetainedAllocationEffect::new(g,cpu,&mut allocation_actual))});cpu=cpu.checked_sub(turn.work).unwrap();birth_work+=turn.work;if let Some(actual)=allocation_actual.take(){assert_eq!(actual,heap.0)}let p=turn.result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(p.copied_bytes,0);remaining.maximum_items-=p.copied_items;remaining.maximum_copy_bytes-=p.copied_bytes;remaining.maximum_capacity_bytes-=p.retained_capacity_bytes;remaining.maximum_release_bytes-=p.released_bytes;born+=p.retained_capacity_bytes;freed+=p.released_bytes;}
 assert!(admission.terminal_is_empty());assert_eq!(birth_work,law["expected"]["birthCpu"].as_u64().unwrap());let mut source=output.unwrap();assert_eq!(source.borrow().get().as_ptr(),pointer);assert_eq!(source.borrow().get(),oracle.as_ref());
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){
  if source.terminal_is_empty(){break}
  let d=source.next_close_demand(remaining.maximum_copy_bytes).unwrap();assert!(d.depth>=law["expected"]["liveCloseMinimumDepth"].as_u64().unwrap()as usize,"a live original wrapper must quote its own terminal header clear");
  let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
  assert!(g.maximum_items<=remaining.maximum_items&&g.maximum_copy_bytes<=remaining.maximum_copy_bytes&&g.maximum_capacity_bytes<=remaining.maximum_capacity_bytes&&g.maximum_release_bytes<=remaining.maximum_release_bytes&&g.maximum_depth<=remaining.maximum_depth);
  let(turn,heap)=allocation_testing::observe_retirement_allocations(||source.close_step(g));cpu=cpu.checked_sub(turn.work).unwrap();let p=turn.result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
  remaining.maximum_items-=p.copied_items;remaining.maximum_copy_bytes-=p.copied_bytes;remaining.maximum_capacity_bytes-=p.retained_capacity_bytes;remaining.maximum_release_bytes-=p.released_bytes;born+=p.retained_capacity_bytes;freed+=p.released_bytes;
 }
 assert!(source.terminal_is_empty());assert_eq!(input_bytes+born,freed);assert!(cpu>0);
 let turn=source.close_step(Default::default());assert_eq!(turn.work,law["expected"]["cachedCloseCpu"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress(),Default::default());
 assert_eq!(allocation_testing::observe_retirement_allocations(||drop(source)).1,(0,0));
}
