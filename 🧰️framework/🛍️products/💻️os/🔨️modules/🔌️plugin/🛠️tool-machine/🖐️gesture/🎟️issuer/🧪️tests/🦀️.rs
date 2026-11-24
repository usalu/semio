//! 🧪️ Actual original gesture headers, aliases and private decisions obey independent grants.
use semio_plugin_decl_views::gesture_contract::{GestureCapture, GestureSlot};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::shared_issuance::{SealedShared,SealedSharedBirth};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use crate::gesture_issuer::{GestureOperation,issue_live,publish_live,reserve_live};
const WORK:u64=1024;
fn bank(grant:RetainedCloneGrant)->EffectBank{EffectBank::new(Wallet::new(grant,WORK))}
fn funded()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:4*SealedShared::<GestureSlot<()>>::birth_bytes(),maximum_depth:4,..Default::default()}}
fn settle(grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome)->(Result<RetainedCloneStep,semio_framework_value::ValueError>,(usize,usize)){
 let mut bank=bank(grant);let((poll,_),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bank.run(grant,WORK,|effect|RetainedWorkPoll::Complete(run(effect))));let _=bank.take_admitted();
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("original gesture step stayed pending")};(turn.result,(heap.requested_bytes,heap.released_bytes))
}
#[path="../../../../../../../../🔨️modules/🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;
fn release_birth(birth:&mut SealedSharedBirth<GestureSlot<()>>){for _ in 0..64{if birth.terminal_is_empty(){return}let d=birth.next_close_demand();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(4096),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)};effect::with_effect(grant,birth.next_close_work_demand().max(1),&mut None,|effect|birth.close_step(4096,effect)).result.unwrap();}panic!("original gesture birth did not close")}
fn issue(slot:GestureSlot<()>,operation:u64)->((GestureOperation<()>,GestureCapture<()>),usize){
 let(mut slot,mut birth,mut published,mut issued)=(Some(slot),None,None,None);
 let(step,heap)=settle(funded(),|effect|reserve_live::<()>(true,&mut birth,effect));assert!(matches!(step.unwrap(),RetainedCloneStep::Complete(_)));let mut born=heap.0;assert_eq!(heap.1,0);
 let(step,heap)=settle(funded(),|effect|publish_live(birth.as_mut().unwrap(),&mut slot,&mut published,effect));assert!(matches!(step.unwrap(),RetainedCloneStep::Complete(_)));born+=heap.0;assert_eq!(heap.1,0);
 let(step,heap)=settle(funded(),|effect|issue_live(&mut published,operation,&mut issued,effect));assert!(matches!(step.unwrap(),RetainedCloneStep::Complete(_)));born+=heap.0;assert_eq!(heap.1,0);
 drop(birth);(issued.unwrap(),born)
}
#[doc(hidden)]
pub fn drain<T:RetireOwned>(value:T)->(usize,usize){drain_unreported(value,&mut 0)}
/// 🔒️ Drains one owner while crediting releases of backing the platform allocated unobserved, such as a lazily boxed mutex, against the caller's remaining `unreported` stock.
fn drain_unreported<T:RetireOwned>(value:T,unreported:&mut usize)->(usize,usize){
 let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original gesture typed owner admission")));
 assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(mut born,mut released)=(0,0);
 for _ in 0..100000{if owner.terminal_is_empty(){break;}let d=owner.next_demand(4096).unwrap();let work=owner.next_computational_work(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1<<10,maximum_copy_bytes:d.copy_bytes.max(1<<16),maximum_capacity_bytes:d.capacity_bytes.max(1<<20),maximum_release_bytes:d.release_bytes.max(1<<20),maximum_depth:d.depth.max(1<<10)};
  let(turn,heap)=effect::with_effect(grant,work,&mut None,|effect|semio_framework_trace::observe_heap_allocations_on_this_thread(||effect::settled(owner.step(4096,effect))));
  let step=turn.result.unwrap();assert!(step.progress().fits(grant));let progress=step.progress();let extra=heap.released_bytes.checked_sub(progress.released_bytes).expect("original gesture step released less than it reported");assert!(extra<=*unreported,"original gesture step released unreported backing");*unreported-=extra;assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);born+=heap.requested_bytes;released+=heap.released_bytes;
 }
 assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));(born,released)
}
#[test]
fn original_gesture_issuer_matches_neutral_admission_and_exact_allocator(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let reserve=SealedShared::<GestureSlot<()>>::prepare_birth_demand().unwrap();let publish=SealedSharedBirth::<GestureSlot<()>>::publication_demand();let duplicate=SealedShared::<GestureSlot<()>>::duplicate_copy_bytes();
 for row in fixture["cases"].as_array().unwrap(){let units=|name:&str|row[name].as_u64().unwrap()as usize;
  let birth_grant=RetainedCloneGrant{maximum_items:units("birthItems"),maximum_copy_bytes:reserve.copy_bytes,maximum_capacity_bytes:units("birthHeaderUnits")*reserve.capacity_bytes,maximum_depth:units("birthDepth")*reserve.depth,..Default::default()};
  let alias_grant=RetainedCloneGrant{maximum_items:units("aliasItems"),maximum_copy_bytes:units("aliasHandleUnits")*publish.copy_bytes.max(duplicate),maximum_capacity_bytes:publish.capacity_bytes,maximum_release_bytes:publish.release_bytes,maximum_depth:units("aliasDepth")*publish.depth.max(1),};
  let(slot,held_heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||GestureSlot::<()>::of("pane-a".into(),"document-revision".into(),None,Some("previous-press".into())));let held=held_heap.requested_bytes-held_heap.released_bytes;let(mut slot,mut birth,mut published,mut issued)=(Some(slot),None,None,None);
  let available=row["available"].as_bool().unwrap();
  let(reserved,reserve_heap)=settle(birth_grant,|effect|reserve_live::<()>(available,&mut birth,effect));
  let admitted=matches!(reserved,Ok(RetainedCloneStep::Complete(_)))&&birth.is_some();
  if !admitted{assert!(birth.is_none(),"{}",row["id"]);assert_eq!(reserve_heap,(0,0));assert_eq!(row["expected"],"pending");continue}
  assert_eq!(reserve_heap,(SealedShared::<GestureSlot<()>>::birth_bytes(),0));
  let(published_step,publish_heap)=settle(alias_grant,|effect|publish_live(birth.as_mut().unwrap(),&mut slot,&mut published,effect));
  let issued_step=if published.is_some(){let(step,heap)=settle(alias_grant,|effect|issue_live(&mut published,17,&mut issued,effect));assert_eq!(heap,(0,0));Some(step)}else{None};
  let live=matches!(published_step,Ok(RetainedCloneStep::Complete(_)))&&matches!(issued_step,Some(Ok(RetainedCloneStep::Complete(_))))&&issued.is_some();
  assert_eq!(live,row["expected"]=="admitted","{}",row["id"]);
  if !live{assert_eq!(publish_heap,(0,0));if published.is_none(){release_birth(birth.as_mut().unwrap());}drop(birth);let held=slot.take().map_or(0,|slot|drain(slot).0);let _=held;if let Some(published)=published.take(){drain(published);}continue}
  let(original,capture)=issued.unwrap();assert_eq!(original.slot.identity(),capture.get()as*const GestureSlot<()>as usize);let(extra,released)=drain((original,capture));drop(birth);assert_eq!(held+reserve_heap.0+publish_heap.0+extra,released);
 }
}
#[test]
fn original_gesture_issuer_cancellation_keeps_original_decision_and_live_alias(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cancellationCases"].as_array().unwrap(){for order in[false,true]{
  let(slot,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut slot=GestureSlot::<()>::of("pane-a".into(),"current-revision".into(),None,None);slot.state.get_mut().unwrap().closing=Some("original-decision".into());slot.state.get_mut().unwrap().driven=true;slot});
  let mut held=heap.requested_bytes-heap.released_bytes;
  let((mut original,capture),mut born)=issue(slot,41);
  let(_,lazy)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(original.slot.state.lock()));let mut unreported=lazy.requested_bytes-lazy.released_bytes;held+=unreported;
  let identity=original.slot.identity();let extracted=row["decisionExtracted"].as_bool().unwrap();
  original.settled=row["initiallySettled"].as_bool().unwrap();if extracted{original.decision=original.slot.take();}
  let pointer=if extracted{original.decision.as_ref().unwrap().closing.as_ref().unwrap().as_ptr()}else{original.slot.state.lock().unwrap().closing.as_ref().unwrap().as_ptr()};
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{for _ in 0..row["intentCount"].as_u64().unwrap(){original.request_cancellation();}});
  assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.settled,row["expectedSettled"].as_bool().unwrap());assert_eq!(original.decision.is_some(),extracted);assert_eq!(original.slot.identity(),identity);
  let retained=if extracted{original.decision.as_ref().unwrap().closing.as_ref().unwrap().as_ptr()}else{original.slot.state.lock().unwrap().closing.as_ref().unwrap().as_ptr()};assert_eq!(retained,pointer);
  let(births,releases)=if order{let(a,b)=drain_unreported(original,&mut unreported);assert_eq!(capture.window(),"pane-a");assert_eq!(capture.state.lock().unwrap().closing.as_deref(),if extracted{None}else{Some("original-decision")});let(c,d)=drain_unreported(capture,&mut unreported);(a+c,b+d)}else{let(a,b)=drain_unreported(capture,&mut unreported);assert_eq!(original.slot.window(),"pane-a");let(c,d)=drain_unreported(original,&mut unreported);(a+c,b+d)};
  assert_eq!(unreported,0);born+=births;assert_eq!(held+born,releases);
 }}
 let(detached,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(GestureCapture::<()>::detached);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(born,released)=drain(detached);assert_eq!(born,released);
}
