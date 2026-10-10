//! 🧪️ Actual original gesture headers, aliases and private decisions obey independent grants.
use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;
fn drain<T:RetireOwned>(value:T)->(usize,usize){
 let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original gesture typed owner admission")));
 assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(mut born,mut released)=(0,0);
 for _ in 0..100000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(4096),maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy.max(4096)}else{release.max(4096)}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};
  for denied in[Some(RetainedCloneGrant{maximum_items:0,..grant}),(copy>0).then_some(RetainedCloneGrant{maximum_copy_bytes:copy.saturating_sub(1),..grant}),(grant.maximum_capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}),(release>0).then_some(RetainedCloneGrant{maximum_release_bytes:release.saturating_sub(1),..grant}),(grant.maximum_depth>0).then_some(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant})].into_iter().flatten(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(denied));if let Ok(step)=result{assert_eq!(step.progress(),Default::default());}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;
 }
 assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));(born,released)
}
#[test]
fn original_gesture_issuer_matches_neutral_admission_and_exact_allocator(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let units=|name:&str|row[name].as_u64().unwrap()as usize;let birth=RetainedCloneGrant{maximum_items:units("birthItems"),maximum_capacity_bytes:units("birthHeaderUnits")*SealedShared::<GestureSlot<()>>::birth_bytes(),maximum_depth:units("birthDepth"),..Default::default()};let handoff=RetainedCloneGrant{maximum_items:units("aliasItems"),maximum_copy_bytes:units("aliasHandleUnits")*size_of::<SealedShared<GestureSlot<()>>>(),maximum_depth:units("aliasDepth"),..Default::default()};
  let(prepared,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||reserve_live::<()>(row["available"].as_bool().unwrap(),birth,handoff));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(prepared.is_ok(),row["expected"]=="admitted");
  let Ok(prepared)=prepared else{continue};let(slot,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||GestureSlot::<()>::of("pane-a".into(),"document-revision".into(),None,Some("previous-press".into())));let held=heap.requested_bytes-heap.released_bytes;let((original,capture),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||issue_live(prepared,slot,17,handoff));assert_eq!((heap.requested_bytes,heap.released_bytes),(SealedShared::<GestureSlot<()>>::birth_bytes(),0));let mut born=heap.requested_bytes;assert_eq!(original.slot.identity(),capture.get()as*const GestureSlot<()>as usize);let(extra,released)=drain((original,capture));born+=extra;assert_eq!(held+born,released);eprintln!("[DEBUG] Original gesture issuer original={held} born={born} released={released} finalDrop=0");
 }
}
#[test]
fn original_gesture_issuer_cancellation_keeps_original_decision_and_live_alias(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cancellationCases"].as_array().unwrap(){for order in[false,true]{
  let birth=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<GestureSlot<()>>::birth_bytes(),maximum_depth:1,..Default::default()};
  let handoff=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<SealedShared<GestureSlot<()>>>(),maximum_depth:1,..Default::default()};
  let prepared=reserve_live::<()>(true,birth,handoff).unwrap();
  let(slot,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut slot=GestureSlot::<()>::of("pane-a".into(),"current-revision".into(),None,None);slot.state.get_mut().unwrap().closing=Some("original-decision".into());slot.state.get_mut().unwrap().driven=true;slot});
  let held=heap.requested_bytes-heap.released_bytes;
  let((mut original,capture),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||issue_live(prepared,slot,41,handoff));
  let mut born=heap.requested_bytes;assert_eq!(heap.released_bytes,0);let identity=original.slot.identity();let extracted=row["decisionExtracted"].as_bool().unwrap();
  original.settled=row["initiallySettled"].as_bool().unwrap();if extracted{original.decision=original.slot.take();}
  let pointer=if extracted{original.decision.as_ref().unwrap().closing.as_ref().unwrap().as_ptr()}else{original.slot.state.lock().unwrap().closing.as_ref().unwrap().as_ptr()};
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{for _ in 0..row["intentCount"].as_u64().unwrap(){original.request_cancellation();}});
  assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.settled,row["expectedSettled"].as_bool().unwrap());assert_eq!(original.decision.is_some(),extracted);assert_eq!(original.slot.identity(),identity);
  let retained=if extracted{original.decision.as_ref().unwrap().closing.as_ref().unwrap().as_ptr()}else{original.slot.state.lock().unwrap().closing.as_ref().unwrap().as_ptr()};assert_eq!(retained,pointer);
  let(births,releases)=if order{let(a,b)=drain(original);assert_eq!(capture.window(),"pane-a");assert_eq!(capture.state.lock().unwrap().closing.as_deref(),if extracted{None}else{Some("original-decision")});let(c,d)=drain(capture);(a+c,b+d)}else{let(a,b)=drain(capture);assert_eq!(original.slot.window(),"pane-a");let(c,d)=drain(original);(a+c,b+d)};
  born+=births;assert_eq!(held+born,releases);eprintln!("[DEBUG] Original gesture cancellation case={} aliasFirst={} original={held} born={born} released={releases} finalDrop=0",row["id"],!order);
 }}
 let(detached,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(GestureCapture::<()>::detached);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(born,released)=drain(detached);assert_eq!(born,released);
}
