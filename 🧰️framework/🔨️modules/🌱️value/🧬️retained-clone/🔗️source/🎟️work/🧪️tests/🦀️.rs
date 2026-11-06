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

#[test]fn paid_binding_check_obeys_neutral_vectors(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/binding.json")).unwrap();
 fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
 fn source(value:u64)->RetainedCloneSource<u64>{let mut admission=RetainedCloneSource::admission_owned(value,());let mut output=None;while !admission.terminal_is_empty(){let g=grant(admission.next_demand().unwrap());if admission.phase()=="ready"{admission.take_output(&mut output,g).result.unwrap();}else{let mut actual=None;admission.advance(g,&mut actual).result.unwrap();}}output.unwrap()}
 for row in rows.as_array().unwrap(){let mut left=source(1);let mut foreign=source(2);let mut binding=None;if row["bound"].as_bool().unwrap(){let turn=left.borrow().bind(&mut binding,grant(RetirementDemand{copy_bytes:RetainedCloneBinding::alias_copy_bytes(),depth:1,..Default::default()}));turn.result.unwrap();}
  let target=if row["matching"].as_bool().unwrap(){left.borrow()}else{foreign.borrow()};let mut g=grant(RetirementDemand{depth:1,..Default::default()});if !row["physical"].as_bool().unwrap(){g.maximum_items=0}let turn=target.check_binding_paid(&binding,row["work"].as_u64().unwrap(),g);assert_eq!(turn.work,row["reported"].as_u64().unwrap());let outcome=match turn.result{Err(_)=>"mismatch",Ok(None)=>"denied",Ok(Some(false))=>"unbound",Ok(Some(true))=>"matched"};assert_eq!(outcome,row["outcome"].as_str().unwrap());
  while binding.is_some(){let d=RetainedCloneBinding::close_demand(&binding,4096).unwrap();let mut actual=None;RetainedCloneBinding::close_one(&mut binding,grant(d),&mut actual).result.unwrap();}for s in [&mut left,&mut foreign]{while !s.terminal_is_empty(){let d=s.next_close_demand(4096).unwrap();let mut actual=None;s.close_step(grant(d),&mut actual).result.unwrap();}}
 }
}

#[test]fn paid_projection_obeys_neutral_vectors(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/projection.json")).unwrap();
 assert!(RetainedCloneRef::<(u64,u64)>::projection_work_demand(u64::MAX).is_err());
 fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
 for row in rows.as_array().unwrap(){let mut admission=RetainedCloneSource::admission_owned((3u64,7u64),());let mut output=None;while !admission.terminal_is_empty(){let g=grant(admission.next_demand().unwrap());if admission.phase()=="ready"{admission.take_output(&mut output,g).result.unwrap();}else{let mut actual=None;admission.advance(g,&mut actual).result.unwrap();}}let mut source=output.unwrap();let d=RetainedCloneRef::<(u64,u64)>::projection_demand::<u64>();assert_eq!(RetainedCloneRef::<(u64,u64)>::projection_work_demand(1).unwrap(),6);let mut g=grant(d);if !row["physical"].as_bool().unwrap(){g.maximum_items=0}let calls=std::cell::Cell::new(0);let fails=row["fails"].as_bool().unwrap();let turn=source.borrow().project_paid(2,1,row["work"].as_u64().unwrap(),g,|value|{calls.set(calls.get()+1);RetainedWorkOutcome{result:if fails{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture refusal"))}else{Ok(&value.1)},work:1}});assert_eq!(turn.work,row["reported"].as_u64().unwrap());let outcome=match turn.result{Err(_)=>"failure",Ok(None)=>"denied",Ok(Some((value,progress)))=>{assert_eq!(*value.get(),7);assert_eq!(progress.copied_bytes,d.copy_bytes);assert_eq!(progress.copied_items,row["copiedItems"].as_u64().unwrap()as usize);assert!(progress.fits(g));"projected"}};assert_eq!(outcome,row["outcome"].as_str().unwrap());assert_eq!(calls.get(),usize::from(outcome!="denied"));while !source.terminal_is_empty(){let d=source.next_close_demand(4096).unwrap();let mut actual=None;source.close_step(grant(d),&mut actual).result.unwrap();}}
}
