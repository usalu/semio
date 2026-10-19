//! 🧪️ The actual native Grant codec shares the complete portable authority and bounded loan vector.
use super::*;
use semio_framework_actor::{ActorId,OriginalActorTurnLoan,RetainedTurnInput,ShardId};

fn resources(value:&serde_json::Value)->semio_framework_actor::Budget{
 semio_framework_actor::Budget{fuel:value["fuel"].as_u64().unwrap(),wall_ms:u32::try_from(value["wallMs"].as_u64().unwrap()).unwrap(),memory_bytes:value["memoryBytes"].as_u64().unwrap(),ui_nodes:u32::try_from(value["uiNodes"].as_u64().unwrap()).unwrap(),mailbox_len:u16::try_from(value["mailboxLen"].as_u64().unwrap()).unwrap(),max_effects:u32::try_from(value["maxEffects"].as_u64().unwrap()).unwrap(),max_patch_bytes:u32::try_from(value["maxPatchBytes"].as_u64().unwrap()).unwrap(),max_frames:u32::try_from(value["maxFrames"].as_u64().unwrap()).unwrap()}
}

#[semio_framework_async_macros::async_test]
async fn original_shard_native_wire_preserves_one_source_and_its_distinct_bounded_loan(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let input:RetainedTurnInput=serde_json::from_value(law["input"].clone()).unwrap();
 let loan:OriginalActorTurnLoan=serde_json::from_value(law["loan"].clone()).unwrap();
 let before=resources(&law["dispatch"]["resourcesBefore"]);let after=resources(&law["dispatch"]["resourcesAfter"]);
 let grant=TurnGrant{actor:ActorId(71),shard:ShardId(0),budget:before,retained:input,loan,envelopes:Vec::new()};
 let expected=law["dispatch"]["singleAuthorityWire"]["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect::<Vec<_>>();
 let mut bytes=Vec::with_capacity(expected.len());let pointer=bytes.as_ptr();
 let (packed,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(super::super::ShardFrame::pack_encode_grant(&grant,ShardResourceBudget::from_original(&after),&mut bytes)));
 packed.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pointer,bytes.as_ptr());assert_eq!(bytes,expected);assert_eq!(bytes.len(),law["dispatch"]["singleAuthorityWire"]["byteLength"].as_u64().unwrap()as usize);
 let mut position=0;let decoded=super::super::ShardFrame::pack_decode(&bytes,&mut position).await.unwrap();assert_eq!(position,bytes.len());
 let super::super::ShardFrame::Grant{actor,budget,retained,retained_policy,work_fuel,deadline_us,envelopes}=decoded else{panic!("expected original Grant")};
 assert_eq!(actor,grant.actor);assert_eq!(budget,after);assert_eq!(retained,input);assert_eq!(retained_policy,loan.retained_policy);assert_eq!(work_fuel,loan.work_fuel);assert_eq!(deadline_us,loan.deadline_us);assert!(envelopes.is_empty());assert_eq!(grant.budget,before);assert_eq!(*grant.original_input(),input);assert_eq!(grant.loan,loan);
 eprintln!("[DEBUG] Actual native original Grant192 bytes equals neutral Python struct/Node Buffer vector; one source identity/clock/deadline/fuel and distinct bounded loan preserved through resource override, preallocated wire heap0");
}


#[semio_framework_async_macros::async_test]
async fn original_shard_raw_loan_refusal_precedes_all_wire_effects_and_envelope_ownership(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let input:RetainedTurnInput=serde_json::from_value(law["input"].clone()).unwrap();let budget=resources(&law["dispatch"]["resourcesAfter"]);
 let wire=law["dispatch"]["singleAuthorityWire"]["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect::<Vec<_>>();
 for row in law["loanCases"].as_array().unwrap(){
  let axis=row["axis"].as_str().unwrap();let admitted=row["admitted"].as_bool().unwrap();let mut value=law["loan"].clone();if value["retainedPolicy"].get(axis).is_some(){value["retainedPolicy"][axis]=row["value"].clone();}else if value.get(axis).is_some(){value[axis]=row["value"].clone();}let loan:OriginalActorTurnLoan=serde_json::from_value(value).unwrap();
  let frame=super::super::ShardFrame::Grant{actor:ActorId(71),budget,retained:input,retained_policy:loan.retained_policy,work_fuel:loan.work_fuel,deadline_us:loan.deadline_us,envelopes:Vec::new()};let mut bytes=Vec::with_capacity(256);bytes.extend_from_slice(&[11,13]);let pointer=bytes.as_ptr();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(frame.pack_encode(&mut bytes)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pointer,bytes.as_ptr());assert_eq!(result.is_ok(),admitted);
  if admitted{assert_eq!(&bytes[2..],wire.as_slice());continue;}let refusal=semio_framework_actor::pack::PackError::InvalidRetainedTurn("original grant semantic loan exceeds its source");assert_eq!(result.unwrap_err(),refusal);assert_eq!(bytes,[11,13]);
  let offset=match axis{"maximumItems"=>135,"maximumCopyBytes"=>143,"maximumCapacityBytes"=>151,"maximumReleaseBytes"=>159,"maximumDepth"=>167,"workFuel"=>175,"deadlineUs"=>183,_=>panic!("unknown original loan axis")};let mut incoming=wire.clone();incoming[offset..offset+8].copy_from_slice(&row["value"].as_u64().unwrap().to_le_bytes());let pointer=incoming.as_ptr();let mut position=0;
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(super::super::ShardFrame::pack_decode(&incoming,&mut position)));assert_eq!(result.unwrap_err(),refusal);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pointer,incoming.as_ptr());assert_eq!(position,191);
 }
 eprintln!("[DEBUG] Actual native raw Grant nine bounded-loan cases validate all5physical/work/deadline axes before any write or envelope decode; denial keeps original preallocated bytes and heap0, independent SQLite vectors");
}
