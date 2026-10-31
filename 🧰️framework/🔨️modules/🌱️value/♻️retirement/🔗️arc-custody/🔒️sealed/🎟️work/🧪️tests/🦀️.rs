use super::*;
#[test]
fn original_sealed_alias_return_and_unwrap_keep_actual_cpu_separate_from_denial(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let g:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
 let(root,_)=SealedShared::admit(law["value"].as_u64().unwrap(),SharedIssuer::owned(),g).result.map_err(|(e,_)|e).unwrap();
 let(mut child,_)=root.try_duplicate(g).result.unwrap();
 let oracle=std::sync::Arc::new(law["value"].as_u64().unwrap());
 let oracle_child=std::sync::Arc::clone(&oracle);
 let mut actual=Vec::new();let mut cpu=law["maximumCpuWork"].as_u64().unwrap();
 let turn=child.return_borrowed_alias(&root,RetainedCloneGrant{maximum_items:0,..g});cpu-=turn.work;assert_eq!(turn.result.unwrap(),Default::default());
 actual.push(serde_json::json!({"label":"deniedAlias","work":turn.work,"aliases":root.source.as_ref().unwrap().aliases()}));
 let turn=root.try_unwrap(g);cpu-=turn.work;let(error,root)=turn.result.err().unwrap();assert_eq!(error.kind,ValueRefusalKind::WorkLimit);
 actual.push(serde_json::json!({"label":"blockedUnwrap","work":turn.work,"aliases":root.source.as_ref().unwrap().aliases()}));
 assert_eq!(std::sync::Arc::strong_count(&oracle),root.source.as_ref().unwrap().aliases());
 let turn=child.return_borrowed_alias(&root,g);cpu-=turn.work;assert_eq!(turn.result.unwrap().copied_items,1);drop(oracle_child);
 actual.push(serde_json::json!({"label":"paidAliasReturn","work":turn.work,"aliases":root.source.as_ref().unwrap().aliases()}));
 assert_eq!(std::sync::Arc::strong_count(&oracle),root.source.as_ref().unwrap().aliases());
 let turn=child.return_borrowed_alias(&root,g);cpu-=turn.work;assert_eq!(turn.result.unwrap(),Default::default());
 actual.push(serde_json::json!({"label":"cachedAliasReturn","work":turn.work,"aliases":root.source.as_ref().unwrap().aliases()}));
 let turn=root.try_unwrap(RetainedCloneGrant{maximum_items:0,..g});cpu-=turn.work;let(_,root)=turn.result.err().unwrap();
 actual.push(serde_json::json!({"label":"deniedUnwrap","work":turn.work,"aliases":root.source.as_ref().unwrap().aliases()}));
 let turn=root.try_unwrap(g);cpu-=turn.work;let(value,p)=turn.result.map_err(|(e,_)|e).unwrap();
 assert_eq!(value,std::sync::Arc::try_unwrap(oracle).unwrap());assert_eq!(p.released_bytes,SealedShared::<u64>::birth_bytes());
 actual.push(serde_json::json!({"label":"paidUnwrap","work":turn.work,"value":value}));
 assert!(cpu>0);assert!(child.terminal_is_empty());assert_eq!(serde_json::Value::Array(actual),law["expected"]);
}

#[test]
fn original_sealed_birth_preserves_real_null_work_and_same_value(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let value=law["value"].as_u64().unwrap();let g:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
 let((turn,attempts),heap)=allocation_testing::observe_retirement_allocations(||allocation_testing::refuse_one_allocation_on_this_thread(||SealedShared::admit(value,SharedIssuer::owned(),g)));
 assert_eq!(attempts,1);assert_eq!(turn.work,1);assert_eq!(heap,(0,0));let(error,value)=turn.result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::AllocationFailed);assert_eq!(serde_json::to_value(value).unwrap(),law["value"]);
 let turn=SealedShared::admit(value,SharedIssuer::owned(),g);assert_eq!(turn.work,1);let(owner,birth)=turn.result.unwrap();assert_eq!(birth.retained_capacity_bytes,SealedShared::<u64>::birth_bytes());
 let turn=owner.try_unwrap(g);assert_eq!(turn.work,1);assert_eq!(serde_json::to_value(turn.result.unwrap().0).unwrap(),law["value"]);
}

fn spend_alias_work(bank:&mut RetainedCloneGrant,cpu:&mut u64,progress:RetainedCloneProgress,work:u64){
 assert!(progress.fits(*bank));*cpu=cpu.checked_sub(work).unwrap();bank.maximum_items-=progress.copied_items;bank.maximum_copy_bytes-=progress.copied_bytes;bank.maximum_capacity_bytes-=progress.retained_capacity_bytes;bank.maximum_release_bytes-=progress.released_bytes;
}
#[test]
fn original_shared_alias_admission_reports_actual_work_before_result_and_preserves_identity(){
 let law:serde_json::Value=serde_json::from_str(include_str!("alias.json")).unwrap();let mut bank:RetainedCloneGrant=serde_json::from_value(law["stock"].clone()).unwrap();let mut cpu=law["maximumCpuWork"].as_u64().unwrap();
 let oracle=std::sync::Arc::new(law["text"].as_str().unwrap().to_owned());let input=law["text"].as_str().unwrap().to_owned();let pointer=input.as_ptr();let input_bytes=input.capacity();
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||SealedShared::admit(input,SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,..bank}));let work=turn.work;let(root,p)=turn.result.unwrap();assert_eq!(heap,(p.retained_capacity_bytes,0));spend_alias_work(&mut bank,&mut cpu,p,work);let mut born=heap.0;let mut freed=heap.1;let identity=root.identity();
 for g in [RetainedCloneGrant{maximum_items:0,..bank},RetainedCloneGrant{maximum_depth:0,..bank}]{
  let(turn,heap)=allocation_testing::observe_retirement_allocations(||root.try_duplicate(g));assert_eq!(turn.work,law["expected"]["deniedWork"].as_u64().unwrap());assert_eq!(heap,(0,0));assert_eq!(turn.result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(root.identity(),identity);assert_eq!(root.get().as_ptr(),pointer);
 }
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||root.try_duplicate(RetainedCloneGrant{maximum_items:1,..bank}));let work=turn.work;assert_eq!(work,law["expected"]["aliasWork"].as_u64().unwrap());assert_eq!(heap,(0,0));let(mut child,p)=turn.result.unwrap();spend_alias_work(&mut bank,&mut cpu,p,work);let oracle_child=std::sync::Arc::clone(&oracle);assert_eq!(child.identity(),identity);assert_eq!(root.source.as_ref().unwrap().aliases(),std::sync::Arc::strong_count(&oracle));assert_eq!(serde_json::to_value(child.get()).unwrap(),law["text"]);
 let turn=child.return_borrowed_alias(&root,RetainedCloneGrant{maximum_items:1,..bank});let work=turn.work;let p=turn.result.unwrap();spend_alias_work(&mut bank,&mut cpu,p,work);drop(oracle_child);
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||child.try_duplicate(RetainedCloneGrant{maximum_items:1,..bank}));assert_eq!(turn.work,law["expected"]["closedWork"].as_u64().unwrap());assert_eq!(heap,(0,0));assert_eq!(turn.result.unwrap_err().kind,ValueRefusalKind::InvalidValue);assert_eq!(root.source.as_ref().unwrap().aliases(),std::sync::Arc::strong_count(&oracle));assert!(child.terminal_is_empty());drop(child);
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||root.try_unwrap(RetainedCloneGrant{maximum_items:1,..bank}));let work=turn.work;let(value,p)=turn.result.unwrap();assert_eq!(value.as_ptr(),pointer);assert_eq!(value,std::sync::Arc::try_unwrap(oracle).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));spend_alias_work(&mut bank,&mut cpu,p,work);born+=heap.0;freed+=heap.1;
 let mut close=retirement::controlled::ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if close.terminal_is_empty(){break}let d=close.next_demand(bank.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(g.maximum_depth<=bank.maximum_depth&&bank.maximum_items>0&&cpu>0);let(result,heap)=allocation_testing::observe_retirement_allocations(||close.step(g));let work=close.last_turn_work();let p=result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));spend_alias_work(&mut bank,&mut cpu,p,work);born+=heap.0;freed+=heap.1;}
 assert!(close.terminal_is_empty());assert_eq!(input_bytes+born,freed);assert!(cpu>0);
}
