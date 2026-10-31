//! 🧯️ Funded diagnostic births preserve exact canonical wire and actual candidate custody.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn independent_wire(fault:&Fault)->serde_json::Value{
 let mut cursor=crate::FaultWireCursor::new();let mut target=[std::mem::MaybeUninit::uninit();8192];for _ in 0..8192{if cursor.is_complete(){break;}let d=cursor.advance_demands(fault,&target).unwrap();cursor.advance_one(fault,&mut target,grant(d)).result.unwrap();}assert!(cursor.is_complete());let actual=serde_json::from_slice(cursor.buffered_bytes(&target).unwrap()).unwrap();let d=cursor.retirement_demands();cursor.close_step(grant(d)).result.unwrap();assert!(cursor.terminal_is_empty());actual
}
#[test]
fn message_birth_denial_receipt_transfer_and_physical_close_match_shared_wire(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["messages"].as_array().unwrap(){
  let source=row["message"].as_str().unwrap();let identity=source.as_ptr();let d=FaultMessageOwner::construction_demand(source).unwrap();let g=grant(d);
  for axis in 0..4{let mut no=g;match axis{0=>no.maximum_items=0,1=>no.maximum_copy_bytes-=1,2=>no.maximum_capacity_bytes-=1,_=>no.maximum_depth-=1}let(result,heap)=observe(||FaultMessageOwner::construct(source,no));assert_eq!(result.work,law["cpuContract"]["deniedBirth"].as_u64().unwrap());assert!(result.result.is_err());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.as_ptr(),identity);}
  let(result,heap)=observe(||FaultMessageOwner::construct(source,g));assert_eq!(result.work,law["cpuContract"]["admittedBirth"].as_u64().unwrap());let(mut owner,p)=result.result.unwrap();assert!(!heap.overflowed);assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(p.copied_bytes,source.len()+"app.message".len());assert_eq!(independent_wire(owner.fault().unwrap()),row["expected"]);assert_eq!(source.as_ptr(),identity);
  let mut output=None;let take=owner.take_demand();let mut denied=grant(take);denied.maximum_depth=0;let(result,heap)=observe(||owner.take_step(&mut output,denied));assert_eq!(result.work,law["cpuContract"]["deniedTake"].as_u64().unwrap());assert_eq!(result.result.unwrap().progress(),Default::default());assert!(output.is_none());assert!(owner.fault().is_some());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let mut released=0;for _ in 0..8{if owner.terminal_is_empty(){break;}let d=owner.close_demand();let mut denied=grant(d);if d.release_bytes>0{denied.maximum_release_bytes-=1;let(step,heap)=observe(||owner.close_step(denied));assert_eq!(step.work,law["cpuContract"]["deniedClose"].as_u64().unwrap());assert_eq!(step.result.unwrap().progress(),Default::default());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(step,heap)=observe(||owner.close_step(grant(d)));assert_eq!(step.work,law["cpuContract"]["admittedClose"].as_u64().unwrap());let actual=step.result.unwrap().progress();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(actual.retained_capacity_bytes,actual.released_bytes));released+=actual.released_bytes;}
  assert!(owner.terminal_is_empty());assert_eq!(released,p.retained_capacity_bytes);let((),heap)=observe(||drop(owner));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
}

#[test]
fn successful_fault_take_moves_original_backings_without_payload_copy_and_closes_actual_output(){
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["messages"].as_array().unwrap(){
  let message=row["message"].as_str().unwrap();let d=FaultMessageOwner::construction_demand(message).unwrap();let(mut owner,_)=FaultMessageOwner::construct(message,grant(d)).result.unwrap();let text=owner.fault().unwrap().message.as_ptr();let scope=owner.fault().unwrap().scope.as_ref()as*const FaultScope;let mut output=None;let g=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let(result,h)=observe(||owner.take_step(&mut output,g));assert_eq!(result.work,law["cpuContract"]["admittedTake"].as_u64().unwrap());let actual=result.result.unwrap().progress();assert!(actual.fits(g));assert_eq!(actual.copied_items,1);assert_eq!(actual.copied_bytes,0);assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));assert!(owner.terminal_is_empty());let fault=output.unwrap();assert_eq!(fault.message.as_ptr(),text);assert_eq!(fault.scope.as_ref()as*const FaultScope,scope);assert_eq!(independent_wire(&fault),row["expected"]);let((),h)=observe(||drop(owner));assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));
  let(mut closing,h)=observe(||ControlledRetirement::retain(fault));assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));for _ in 0..4096{if closing.terminal_is_empty(){break}let d=closing.next_demand(8192).unwrap();let(result,h)=observe(||closing.step(grant(d)));let actual=result.unwrap().progress();assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(actual.retained_capacity_bytes,actual.released_bytes));}assert!(closing.terminal_is_empty());let((),h)=observe(||drop(closing));assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));
 }
}


#[test]
fn actual_occupied_receiver_and_terminal_queries_report_independent_cpu_without_hidden_effects(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in law["messages"].as_array().unwrap(){
  let source=row["message"].as_str().unwrap();let d=FaultMessageOwner::construction_demand(source).unwrap();let(mut first,_)=FaultMessageOwner::construct(source,grant(d)).result.unwrap();let(mut second,_)=FaultMessageOwner::construct(source,grant(d)).result.unwrap();let pointer=second.fault().unwrap().message.as_ptr();let mut occupied=None;let g=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};let transferred=first.take_step(&mut occupied,g);assert_eq!(transferred.work,law["cpuContract"]["admittedTake"].as_u64().unwrap());transferred.result.unwrap();
  let(turn,h)=observe(||second.take_step(&mut occupied,g));assert_eq!(turn.work,law["cpuContract"]["occupiedReceiver"].as_u64().unwrap());let error=turn.result.unwrap_err();assert_eq!(error.kind,ValueRefusalKind::InvariantViolated);assert_eq!(error.message,"Fault publication requires an empty receiver");assert_eq!(second.fault().unwrap().message.as_ptr(),pointer);assert_eq!(independent_wire(occupied.as_ref().unwrap()),row["expected"]);assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));
  let(turn,h)=observe(||first.close_step(g));assert_eq!(turn.work,law["cpuContract"]["cachedTerminal"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));drop(first);
  for _ in 0..8{if second.terminal_is_empty(){break}let d=second.close_demand();let(turn,h)=observe(||second.close_step(grant(d)));let p=turn.result.unwrap().progress();assert_eq!(turn.work,law["cpuContract"]["admittedClose"].as_u64().unwrap());assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}assert!(second.terminal_is_empty());drop(second);
  let mut closing=semio_framework_value::retirement::controlled::ControlledRetirement::retain(occupied);for _ in 0..4096{if closing.terminal_is_empty(){break}let d=closing.next_demand(8192).unwrap();let(result,h)=observe(||closing.step(grant(d)));let p=result.unwrap().progress();assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}assert!(closing.terminal_is_empty());let(_,h)=observe(||drop(closing));assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));
 }
}
