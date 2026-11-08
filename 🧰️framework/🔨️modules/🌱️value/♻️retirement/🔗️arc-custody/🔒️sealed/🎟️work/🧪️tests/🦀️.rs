use super::*;
#[test]
fn original_sealed_alias_return_and_unwrap_keep_actual_cpu_separate_from_denial(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let g:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
 let(root,_)=SealedShared::admit(law["value"].as_u64().unwrap(),SharedIssuer::owned(),g,&mut None).result.map_err(|(e,_)|e).unwrap();
 let(mut child,_)=crate::effect::run(g,|effect|root.try_duplicate(effect)).result.unwrap();
 let oracle=std::sync::Arc::new(law["value"].as_u64().unwrap());
 let oracle_child=std::sync::Arc::clone(&oracle);
 let mut actual=Vec::new();let mut cpu=u128::from(law["maximumCpuWork"].as_u64().unwrap());
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
 let((turn,attempts),heap)=allocation_testing::observe_retirement_allocations(||allocation_testing::refuse_one_allocation_on_this_thread(||SealedShared::admit(value,SharedIssuer::owned(),g,&mut None)));
 assert_eq!(attempts,1);assert_eq!(turn.work,u128::from(law["refusedBirthWork"].as_u64().unwrap()));assert_eq!(heap,(0,0));let(error,value)=turn.result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::AllocationFailed);assert_eq!(serde_json::to_value(value).unwrap(),law["value"]);
 let turn=SealedShared::admit(value,SharedIssuer::owned(),g,&mut None);assert_eq!(turn.work,u128::from(law["birthWork"].as_u64().unwrap()));assert_eq!(turn.work,u128::from(SealedShared::<u64>::birth_work_demand()));let(owner,birth)=turn.result.unwrap();assert_eq!(birth.retained_capacity_bytes,SealedShared::<u64>::birth_bytes());
 let turn=owner.try_unwrap(g);assert_eq!(turn.work,1);assert_eq!(serde_json::to_value(turn.result.unwrap().0).unwrap(),law["value"]);
}

fn spend_alias_work(bank:&mut RetainedCloneGrant,cpu:&mut u128,progress:RetainedCloneProgress,work:u128){
 assert!(progress.fits(*bank));*cpu=cpu.checked_sub(work).unwrap();bank.maximum_items-=progress.copied_items;bank.maximum_copy_bytes-=progress.copied_bytes;bank.maximum_capacity_bytes-=progress.retained_capacity_bytes;bank.maximum_release_bytes-=progress.released_bytes;
}
#[test]
fn original_shared_alias_admission_reports_actual_work_before_result_and_preserves_identity(){
 let law:serde_json::Value=serde_json::from_str(include_str!("alias.json")).unwrap();let mut bank:RetainedCloneGrant=serde_json::from_value(law["stock"].clone()).unwrap();let mut cpu=u128::from(law["maximumCpuWork"].as_u64().unwrap());
 let oracle=std::sync::Arc::new(law["text"].as_str().unwrap().to_owned());let input=law["text"].as_str().unwrap().to_owned();let pointer=input.as_ptr();let input_bytes=input.capacity();
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||SealedShared::admit(input,SharedIssuer::owned(),RetainedCloneGrant{maximum_items:1,..bank},&mut None));let work=turn.work;let(root,p)=turn.result.unwrap();assert_eq!(heap,(p.retained_capacity_bytes,0));spend_alias_work(&mut bank,&mut cpu,p,work);let mut born=heap.0;let mut freed=heap.1;let identity=root.identity();
 for g in [RetainedCloneGrant{maximum_items:0,..bank},RetainedCloneGrant{maximum_depth:0,..bank}]{
  let(turn,heap)=allocation_testing::observe_retirement_allocations(||crate::effect::run(g,|effect|root.try_duplicate(effect)));assert_eq!(turn.work,u128::from(law["expected"]["deniedWork"].as_u64().unwrap()));assert_eq!(heap,(0,0));assert_eq!(turn.result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(root.identity(),identity);assert_eq!(root.get().as_ptr(),pointer);
 }
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||crate::effect::run(RetainedCloneGrant{maximum_items:1,..bank},|effect|root.try_duplicate(effect)));let work=turn.work;assert_eq!(work,u128::from(law["expected"]["aliasWork"].as_u64().unwrap()));assert_eq!(work,u128::from(SealedShared::<String>::duplicate_alias_work_demand()));assert_eq!(heap,(0,0));let(mut child,p)=turn.result.unwrap();spend_alias_work(&mut bank,&mut cpu,p,work);let oracle_child=std::sync::Arc::clone(&oracle);assert_eq!(child.identity(),identity);assert_eq!(root.source.as_ref().unwrap().aliases(),std::sync::Arc::strong_count(&oracle));assert_eq!(serde_json::to_value(child.get()).unwrap(),law["text"]);
 let turn=child.return_borrowed_alias(&root,RetainedCloneGrant{maximum_items:1,..bank});let work=turn.work;let p=turn.result.unwrap();spend_alias_work(&mut bank,&mut cpu,p,work);drop(oracle_child);
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||crate::effect::run(RetainedCloneGrant{maximum_items:1,..bank},|effect|child.try_duplicate(effect)));assert_eq!(turn.work,u128::from(law["expected"]["closedWork"].as_u64().unwrap()));assert_eq!(heap,(0,0));assert_eq!(turn.result.unwrap_err().kind,ValueRefusalKind::InvalidValue);assert_eq!(root.source.as_ref().unwrap().aliases(),std::sync::Arc::strong_count(&oracle));assert!(child.terminal_is_empty());drop(child);
 let(turn,heap)=allocation_testing::observe_retirement_allocations(||root.try_unwrap(RetainedCloneGrant{maximum_items:1,..bank}));let work=turn.work;let(value,p)=turn.result.unwrap();assert_eq!(value.as_ptr(),pointer);assert_eq!(value,std::sync::Arc::try_unwrap(oracle).unwrap());assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));spend_alias_work(&mut bank,&mut cpu,p,work);born+=heap.0;freed+=heap.1;
 let mut close=retirement::controlled::ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if close.terminal_is_empty(){break}let d=close.next_demand(bank.maximum_copy_bytes).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(g.maximum_depth<=bank.maximum_depth&&bank.maximum_items>0&&cpu>0);let(turn,heap)=allocation_testing::observe_retirement_allocations(||crate::effect::settle(g,|effect|close.step(effect)));let work=turn.work;let p=turn.result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));spend_alias_work(&mut bank,&mut cpu,p,work);born+=heap.0;freed+=heap.1;}
 assert!(close.terminal_is_empty());assert_eq!(input_bytes+born,freed);assert!(cpu>0);
}

#[test]
fn resident_original_handback_preserves_actual_owner_and_work(){
 struct Work;impl OriginalWorkRecipient for Work{type Context<'a,'b>=() where 'b:'a;fn validate_context<'a,'b>(&self,_:&())->Result<(),ValueError> where 'b:'a{Ok(())}fn source_turn<'a,'b>(&self,_:&())->Option<OriginalSourceTurnIdentity> where 'b:'a{None}fn admits<'r,'a,'b>(&mut self,_:&mut (),q:OriginalWorkQuote<'r>,_:u64,_:u64)->OriginalWorkQuoteComplete<'r> where 'b:'a{q.complete(Ok(true))}fn settle_partial<'r,'a,'b>(&mut self,_:&mut (),c:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 'b:'a{c.complete(Ok(()))}fn recover_partial<'r,'a,'b>(&mut self,_:&mut (),c:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 'b:'a{c.pending()}}
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/handback.json")).unwrap();assert_eq!(law[0]["entered"],false);assert_eq!(law[3]["transferred"],true);let mut authority=OriginalWorkAuthority::new(Work);authority.with_loan(&mut (),|loan|{let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let(mut action,mut receipt,mut actual)=(None,None,None);assert_eq!(work.prepare_for_action(1000,&mut action).unwrap(),Some(true));work.issue_performed_for_action(action,&mut receipt).unwrap();let grant=RetainedCloneGrant{maximum_items:100,maximum_copy_bytes:100000,maximum_capacity_bytes:100000,maximum_release_bytes:100000,maximum_depth:100};let mut input=Some(String::from("original handback"));let pointer=input.as_ref().unwrap().as_ptr();let(mut sealed,mut elected,mut output)=(None,None,None);{let mut effect=RetainedAllocationEffect::new(grant,1000,&mut actual,receipt.as_mut().unwrap());let turn=SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut sealed,&mut effect);let born=turn.result.unwrap().progress();assert!(input.is_none()&&sealed.is_some());assert_eq!(*effect.actual(),Some(born.retained_capacity_bytes));*effect.actual()=None;let before=effect.performed_work();{let left=effect.remaining_work_units().unwrap();let mut denied=effect.reborrow(RetainedCloneGrant{maximum_items:0,..grant},left).unwrap();let turn=SealedShared::try_owner_into(&mut sealed,&mut elected,&mut denied);assert!(turn.result.unwrap().is_none());assert_eq!(turn.work,0)}assert_eq!(effect.performed_work(),before);assert!(sealed.is_some()&&elected.is_none());let turn=SealedShared::try_owner_into(&mut sealed,&mut elected,&mut effect);assert!(turn.result.unwrap().is_some());assert_eq!(turn.work,1);assert!(sealed.is_none()&&elected.is_some());let demand=elected.as_ref().unwrap().take_value_demand().unwrap();let before=effect.performed_work();{let left=effect.remaining_work_units().unwrap();let mut denied=effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-std::mem::size_of::<Option<String>>(),..grant},left).unwrap();let turn=elected.as_mut().unwrap().take_value_step(&mut output,&mut denied);assert!(turn.result.unwrap().is_none());assert_eq!(turn.work,0)}assert_eq!(effect.performed_work(),before);assert!(output.is_none());let turn=elected.as_mut().unwrap().take_value_step(&mut output,&mut effect);let returned=turn.result.unwrap().unwrap();assert_eq!(returned.copied_bytes,demand.copy_bytes);assert_eq!(turn.work,4);assert_eq!(returned.released_bytes,born.retained_capacity_bytes);assert!(elected.as_ref().unwrap().terminal_is_empty());assert_eq!(output.as_ref().unwrap().as_ptr(),pointer)}work.claim_performed_for_action(action,&mut receipt).unwrap();let mut partial=None;assert!(work.settle_partial_for_action_into(action,&mut partial).unwrap());let total=partial.take().unwrap().total();let mut proof=None;assert!(work.commit_for_action(total,action,&mut proof).unwrap());work.finish_for_action(&mut proof,&mut action).unwrap();assert_eq!(output.take().unwrap(),"original handback")}).unwrap();assert!(!authority.has_live_actions());
}
