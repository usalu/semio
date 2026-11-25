//! ⚖️ Original diagnostic allocation identity and independent JSON report laws.
use super::*;
 #[test]
 fn completion_fault_moves_original_without_heap_birth_and_matches_shared_json(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../📬️completion/⚠️fault/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
  for row in fixture["cases"].as_array().unwrap(){
   let mut original=Fault::new(FaultOrigin::App,semio_framework_diagnostic::FaultCode::new(row["code"].as_str().unwrap()),row["message"].as_str().unwrap());original.retryable=row["retryable"].as_bool().unwrap();original.params=Some(Box::new(semio_framework_diagnostic::FaultParams(vec![("path".into(),"drawing/clipboard".into()),("detail".into(),"original diagnostic parameter".into())])));original.scope.module=Some("draw".into());let message=original.message.as_ptr();let scope=(&*original.scope)as*const _;
   let(carrier,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ArtifactCompletionFault::new(original));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
   let mut bytes=[0;480];let len=carrier.framed_page_bytes(&mut bytes);let wire:serde_json::Value=serde_json::from_slice(&bytes[..len]).unwrap();
   let decoded=semio_framework_diagnostic::try_decode_fault_bytes(&bytes[..len]).expect("canonical diagnostic report");
   let frame_bytes=fixture["frameBytes"].as_u64().unwrap()as usize;
   assert!(len<=frame_bytes);
   let expected_code=if serde_json::to_string(row["code"].as_str().unwrap()).unwrap().len()-2<=fixture["codeEscapeBytes"].as_u64().unwrap()as usize{row["code"].as_str().unwrap()}else{"interactive-job.fault-capacity"};
   assert_eq!(decoded.code.as_str(),expected_code);
   assert_eq!(decoded.retained_progress,Default::default());
   assert_eq!(wire["retryable"],row["retryable"]);
   let escaped=|text:&str|serde_json::to_string(text).unwrap().len()-2;
   let message_limit=(fixture["messageEscapeBytes"].as_u64().unwrap()as usize).min(frame_bytes-len+escaped(wire["message"].as_str().unwrap()));
   let mut expected=String::new();for scalar in row["message"].as_str().unwrap().chars(){let mut candidate=expected.clone();candidate.push(scalar);if escaped(&candidate)>message_limit{break;}expected=candidate;}
   assert_eq!(decoded.message,expected);
   let(original,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||carrier.into_fault());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(original.message.as_ptr(),message);assert_eq!((&*original.scope)as*const _,scope);assert_eq!(original.message,row["message"].as_str().unwrap());
   let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original diagnostic declares exact retirement"));for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(event.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(event.released_bytes,step.progress().released_bytes);}assert!(owner.terminal_is_empty());
  }
 }
