
#[cfg(test)]
mod operation_tests {
 use super::*;
 #[test]
 fn requested_operation_caps_are_data_and_never_replace_an_original_identity() {
  let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
  let caps: GuestRelayOperationCaps = serde_json::from_value(fixture["caps"].clone()).unwrap();
  for row in fixture["cases"].as_array().unwrap() {
   let request: GuestRelayOperationRequest = serde_json::from_value(row["request"].clone()).unwrap();
   let bounded = caps.bind_request(request);
   assert_eq!(bounded.is_ok(), row["valid"].as_bool().unwrap());
   if let Ok(actual) = bounded {
    assert_eq!(actual.operation_fuel,caps.operation_fuel);
    assert_eq!(actual.turn_fuel,caps.turn_fuel);
    assert_eq!(actual.maximum_lifetime_microseconds,caps.maximum_lifetime_microseconds.min(request.maximum_elapsed_microseconds));
    assert_eq!(actual.maximum_turn_microseconds,caps.maximum_turn_microseconds.min(actual.maximum_lifetime_microseconds));
    let semantic=caps.admitted_request(request).unwrap();
    assert_eq!(semantic.retained.maximum_depth, request.retained.maximum_depth.min(caps.operation.maximum_depth));
    assert_eq!(serde_json::to_value(actual).unwrap(), row["expected"]);
   }
  }
 }
}

#[test]
fn distinct_authenticated_operations_mint_distinct_original_sources_without_reusing_policy_identity() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let caps:super::GuestRelayOperationCaps=serde_json::from_value(fixture["caps"].clone()).unwrap();
 let request:super::GuestRelayOperationRequest=serde_json::from_value(fixture["cases"][0]["request"].clone()).unwrap();
 let first=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
 let second=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
 assert_ne!(first.policy().unwrap().root.operation,second.policy().unwrap().root.operation);
 assert_ne!(first.policy().unwrap().root.generation,second.policy().unwrap().root.generation);
 assert!(first.operation_terminal_is_empty().unwrap());
 assert!(second.operation_terminal_is_empty().unwrap());
}

#[test]
fn original_vm_port_is_scoped_nested_thread_bound_and_unwind_safe() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let caps:super::GuestRelayOperationCaps=serde_json::from_value(fixture["caps"].clone()).unwrap();
 let request:super::GuestRelayOperationRequest=serde_json::from_value(fixture["cases"][0]["request"].clone()).unwrap();
 let first=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
 let second=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
 let traces:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/scope.json")).unwrap();
 fn trace(driver:&super::GuestRelayWakeDriver,steps:&[serde_json::Value],index:&mut usize,actual:&mut Vec<bool>){while *index<steps.len(){let step=steps[*index].as_str().unwrap();*index+=1;match step{"enter"=>with_execution_scope(driver,0,||trace(driver,steps,index,actual)),"leave"=>return,"unwind"=>panic!("scope trace unwind"),"execute"=>actual.push(!current_execution_driver().is_null()),"foreignThread"=>assert!(std::thread::spawn(||current_execution_driver().is_null()).join().unwrap()),_=>unreachable!()}}}
 for row in traces["cases"].as_array().unwrap(){let steps=row["steps"].as_array().unwrap();let mut index=0;let mut actual=Vec::new();while index<steps.len(){let _=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||trace(&first,steps,&mut index,&mut actual)));}assert_eq!(serde_json::to_value(actual).unwrap(),row["allowed"]);assert!(current_execution_driver().is_null());}
 assert!(current_execution_driver().is_null());
 with_execution_scope(&first,0,||{
  assert_eq!(current_execution_driver(),&first as*const _);
  with_execution_scope(&second,0,||assert_eq!(current_execution_driver(),&second as*const _));
  assert_eq!(current_execution_driver(),&first as*const _);
  assert!(std::thread::spawn(||current_execution_driver().is_null()).join().unwrap());
 });
 assert!(current_execution_driver().is_null());
 let failure=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||with_execution_scope(&first,0,||panic!("scoped VM callback unwind"))));
 assert!(failure.is_err());assert!(current_execution_driver().is_null());
}

#[test]
fn scoped_vm_credit_narrows_after_real_worker_metadata_and_returns_the_actual_budget(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let caps:super::GuestRelayOperationCaps=serde_json::from_value(fixture["caps"].clone()).unwrap();let request:super::GuestRelayOperationRequest=serde_json::from_value(fixture["cases"][0]["request"].clone()).unwrap();let driver=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();let mut called=false;
 for _ in 0..32{let outcome=driver.dispatch_operation_step(true,false,|cx|{
  let before=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;cx.consume_fuel(1);
  super::GuestRelayWakeDriver::execute_current_original(cx,|budget,_|{called=true;assert!(budget.input.work.fuel<before.work.fuel);assert!(budget.input.grant.maximum_items<before.grant.maximum_items);Some(semio_framework_actor::OriginalGuestExecutionReceipt{budget,spent_fuel:Some(0),submitted:false})})?;Ok(())
 }).unwrap();if outcome{break}}
 assert!(called);for _ in 0..64{if driver.close_root_step().unwrap(){break}}assert!(driver.operation_terminal_is_empty().unwrap());
}

#[test]
fn explicit_infrastructure_cannot_reuse_transport_or_cleanup_stock(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let request:super::GuestRelayOperationRequest=serde_json::from_value(fixture["cases"][0]["request"].clone()).unwrap();for row in fixture["refusals"].as_array().unwrap(){let mut value=fixture["caps"].clone();match row["field"].as_str().unwrap(){"missing"=>{value.as_object_mut().unwrap().remove("infrastructure");assert!(serde_json::from_value::<super::GuestRelayOperationCaps>(value).is_err());continue},"workFuel"=>value["infrastructure"]["workFuel"]=value["operationFuel"].clone(),"maximumCapacityBytes"=>value["infrastructure"]["retained"]["maximumCapacityBytes"]=value["operation"]["maximumCapacityBytes"].clone(),_=>unreachable!()}let caps:super::GuestRelayOperationCaps=serde_json::from_value(value).unwrap();assert!(caps.bind_request(request).is_err());}}

#[test]
fn actual_operation_keeps_one_thirty_second_source_across_multiple_two_second_turns(){
 static NOW:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1000);fn clock()->Option<u64>{Some(NOW.load(std::sync::atomic::Ordering::SeqCst))}let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let trace=&law["turnTrace"];let mut caps:super::GuestRelayOperationCaps=serde_json::from_value(law["caps"].clone()).unwrap();caps.maximum_turn_microseconds=trace["maximumTurnMicroseconds"].as_u64().unwrap();let mut request:super::GuestRelayOperationRequest=serde_json::from_value(law["cases"][0]["request"].clone()).unwrap();request.maximum_elapsed_microseconds=trace["requestedLifetimeMicroseconds"].as_u64().unwrap();NOW.store(trace["now"][0].as_u64().unwrap(),std::sync::atomic::Ordering::SeqCst);let driver=caps.issue(request,clock,Box::new(||false)).unwrap();let policy=driver.policy().unwrap();assert_eq!(policy.root.maximum_lifetime_microseconds,request.maximum_elapsed_microseconds);let mut previous=None;
 for(index,now)in trace["now"].as_array().unwrap().iter().enumerate(){NOW.store(now.as_u64().unwrap(),std::sync::atomic::Ordering::SeqCst);let mut ran=false;for _ in 0..64{if driver.dispatch_operation_step(false,false,|cx|{let input=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;assert_eq!((input.operation,input.generation),(policy.root.operation,policy.root.generation));assert_eq!(input.work.absolute_deadline_us,trace["absoluteDeadlineMicroseconds"].as_u64().unwrap());assert_eq!(cx.deadline_us(),trace["callbackDeadlinesMicroseconds"][index].as_u64().unwrap());cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;cx.consume_fuel(1);ran=true;Ok(())}).unwrap(){break}}assert!(ran);let remaining=driver.original_remaining().unwrap();if let Some(before)=previous{assert!(remaining.maximum_items<before);}previous=Some(remaining.maximum_items);}
 for _ in 0..256{if driver.close_root_step().unwrap(){break}}assert!(driver.operation_terminal_is_empty().unwrap());
}

#[test]
fn abandoned_source_cancels_actual_callbacks_and_still_accepts_paid_cleanup(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let caps:super::GuestRelayOperationCaps=serde_json::from_value(law["caps"].clone()).unwrap();let request:super::GuestRelayOperationRequest=serde_json::from_value(law["cases"][0]["request"].clone()).unwrap();let driver=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();
 let mut born=false;for _ in 0..64{if driver.dispatch_operation_step(false,false,|_|{born=true;Ok(())}).unwrap(){break}}assert!(born);driver.abandon_operation();assert!(driver.operation_abandoned());assert!(driver.original_cancelled_now().unwrap());let mut observed=false;
 for _ in 0..64{if driver.dispatch_operation_step(false,true,|cx|{observed=cx.is_cancelled();Ok(())}).unwrap(){break}}assert!(observed);for _ in 0..256{if driver.close_root_step().unwrap(){break}}assert!(driver.operation_terminal_is_empty().unwrap());
}

#[test]
fn original_vm_refinement_charges_fresh_work_under_the_issued_total(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/vm-refinement.json")).unwrap();
 for row in law.as_array().unwrap(){
  let number=|key|row[key].as_u64().unwrap();
  let result=refine_execution_work(number("issued"),number("observed"),number("performed").into(),number("redeemed"));
  assert_eq!(result.ok(),row["expected"].as_u64(),"{}",row["name"]);
 }
 assert!(refine_execution_work(u64::MAX,u64::MAX,u128::from(u64::MAX)+1,0).is_err());
}

#[test]
fn native_retirement_receives_scoped_execution_after_its_actual_metadata(){
 use semio_framework_job::{OriginalNativeRetirement,OriginalTurnFailure,StepContext};
 use semio_framework_value::RetainedCloneProgress;
 use std::sync::atomic::{AtomicU64,Ordering};
 static ISSUED_WORK:AtomicU64=AtomicU64::new(0);
 struct VmParent{empty:bool}
 impl OriginalNativeRetirement for VmParent{
  fn close_requires_execution(&self)->bool{!self.empty}
  fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
   let before=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?;
   cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})?;
   cx.consume_fuel(1);
   assert_eq!(cx.original_work_units_spent(),1);
   assert_eq!(cx.reserved_return_work(),0);
   let expected=ISSUED_WORK.load(Ordering::Relaxed)-1;
   assert!(expected>cx.fuel_remaining());
   super::GuestRelayWakeDriver::execute_current_original(cx,|budget,_|{
    assert_eq!(budget.input.grant.maximum_items,before.grant.maximum_items-1);
    assert_eq!(budget.input.work.fuel,expected);
    Some(semio_framework_actor::OriginalGuestExecutionReceipt{budget,spent_fuel:Some(0),submitted:false})
   })?;
   self.empty=true;
   Ok(true)
  }
  fn terminal_is_empty(&self)->bool{self.empty}
 }
 impl Drop for VmParent{fn drop(&mut self){assert!(self.empty)}}
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let caps:super::GuestRelayOperationCaps=serde_json::from_value(law["caps"].clone()).unwrap();let request:super::GuestRelayOperationRequest=serde_json::from_value(law["cases"][0]["request"].clone()).unwrap();let driver=caps.issue(request,||Some(1000),Box::new(||false)).unwrap();let mut input=Some(VmParent{empty:false});let mut lease=None;
 for _ in 0..64{driver.dispatch_admission_step(false,|_,cx|super::GuestRelayAdmissionCall{result:cx.admit_original_native(&mut input,&mut lease).map(|_|()),execution:None}).unwrap();if lease.is_some(){break}}assert!(input.is_none());drop(lease);
 let mut closed=false;
 for _ in 0..512{
  ISSUED_WORK.store(driver.original.try_lock().unwrap().root.fuel_remaining(),Ordering::Relaxed);
  if driver.close_root_step().unwrap(){closed=true;break}
 }
 assert!(closed);
 assert!(driver.operation_terminal_is_empty().unwrap());
}

#[test]
fn exhausted_items_refusal_names_the_dimension_and_every_amount(){
 let error=items_exhausted(ItemsLedger{issued:48_000_000,remaining:164_000,guest_held:100_000,reserve:64_000,grant:4096},None);
 assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::WorkLimit);
 assert_eq!(error.message,"semantic operation currency exhausted with cleanup reserve retained: dimension=retained items, issued=48000000, spent=47836000, remaining=164000, held by guest=100000, cleanup reserve=64000, turn grant=4096, spendable=0");
 let overdrawn=items_exhausted(ItemsLedger{issued:10,remaining:20,guest_held:30,reserve:40,grant:0},None);
 assert!(overdrawn.message.contains("spent=0")&&overdrawn.message.contains("spendable=0")&&overdrawn.message.contains("turn grant=0"));
}

#[test]
fn settled_debits_and_live_reserve_shortages_are_distinct(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/reserve.json")).unwrap();
 fn grant(axis:&str,amount:usize)->RetainedCloneGrant{
  let mut grant=RetainedCloneGrant::default();
  *match axis{"items"=>&mut grant.maximum_items,"copy"=>&mut grant.maximum_copy_bytes,"capacity"=>&mut grant.maximum_capacity_bytes,"release"=>&mut grant.maximum_release_bytes,"depth"=>&mut grant.maximum_depth,_=>unreachable!()}=amount;
  grant
 }
 for row in fixture["cases"].as_array().unwrap(){
  let axis=row["axis"].as_str().unwrap();
  let available=grant(axis,row["available"].as_u64().unwrap() as usize);
  let reserved=grant(axis,row["reserved"].as_u64().unwrap() as usize);
  let actual=consumable_remaining(available,reserved);
  let live=remaining_beside_reserve(available,reserved,"cleanup");
  assert_eq!(actual.as_ref().err().map(|error|error.kind.as_str()),row["static"].as_str(),"{axis}");
  assert_eq!(live.as_ref().err().map(|error|error.kind.as_str()),row["runtime"].as_str(),"{axis}");
  if let Err(error)=live{assert!(error.message.contains(axis),"{axis}: {}",error.message)}
 }
}
