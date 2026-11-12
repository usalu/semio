//! 🧪️ Real original publication preserves source identity, finite stock and exact physical custody.
use semio_framework_job::*;
use semio_framework_job_publication::{JobPublicationKind,RetainedJobPublication};
use semio_framework_value_testing_allocator as _;
fn observed<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);assert!(!heap.overflowed);(value,(heap.requested_bytes,heap.released_bytes))}
fn grant(row:&serde_json::Value)->RetainedCloneGrant{let p=row.as_array().unwrap();RetainedCloneGrant{maximum_items:p[0].as_u64().unwrap()as usize,maximum_copy_bytes:p[1].as_u64().unwrap()as usize,maximum_capacity_bytes:p[2].as_u64().unwrap()as usize,maximum_release_bytes:p[3].as_u64().unwrap()as usize,maximum_depth:p[4].as_u64().unwrap()as usize}}
fn assert_physical(progress:RetainedCloneProgress,heap:(usize,usize),grant:RetainedCloneGrant){assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap)}
#[test]
fn original_source_publication_preserves_native_identity_paid_stages_and_every_cancellation_frontier(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let quantum=grant(&law["originalGrant"]);let stock=grant(&law["originalStock"]);let source:[u8;8193]=std::array::from_fn(|index|(index%251)as u8);let foreign=source;
 for extent in [0,3,20,321,8193]{for kind in [JobPublicationKind::Preview,JobPublicationKind::Checkpoint{applied_progress:7},JobPublicationKind::Fault]{for cut in [0,1,3,257,usize::MAX]{
  let source=&source[..extent];let(mut caller,heap)=observed(||OriginalJobFixtureCaller::admit(OperationId(99731),Generation(5),stock,||Some(0),u64::MAX,law["originalFuel"].as_u64().unwrap()));let mut born=heap.0;let mut released=heap.1;let(mut publication,heap)=observed(RetainedJobPublication::new);assert_eq!(heap,(0,0));let native_owner=std::ptr::addr_of!(publication);let source_pointer=source.as_ptr();let mut turns=0;let mut copied=0;let mut descriptor=None;
  while turns<cut&&descriptor.is_none(){
   let(demand,heap)=observed(||caller.turn(quantum,|cx|Ok(publication.advance_demands(kind,source,cx)?)).unwrap());assert_eq!(heap,(0,0));
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..quantum}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..quantum}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:0,..quantum}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..quantum})].into_iter().flatten(){
    let(result,heap)=observed(||caller.turn(denied,|cx|{assert!(publication.advance_from_source(kind,source,cx)?.is_none());Ok(())}));result.unwrap();assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(std::ptr::addr_of!(publication),native_owner);assert_eq!(source.as_ptr(),source_pointer);
   }
   if turns>0&&extent>0{let(result,heap)=observed(||caller.turn(quantum,|cx|{let result=publication.advance_from_source(kind,&foreign[..extent],cx)?;assert!(result.is_none());Ok(())}));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());}
   let(result,heap)=observed(||caller.turn(quantum,|cx|{cx.narrow_original_work_budget(StepBudget::new(1,cx.deadline_us(),quantum))?;if let Some(result)=publication.advance_from_source(kind,source,cx)?{descriptor=Some(result.into_descriptor());}Ok(())}));result.unwrap();let progress=caller.receipt();assert_physical(progress,heap,quantum);assert_eq!(progress.copied_items,1);if demand.copy_bytes==0{assert_eq!(progress.copied_bytes,law["metadataCopyBytes"].as_u64().unwrap()as usize);}copied+=progress.copied_bytes;assert!(copied<=extent);born+=heap.0;released+=heap.1;turns+=1;assert!(turns<law["maximumSourceTurns"].as_u64().unwrap()as usize);assert_eq!(source.as_ptr(),source_pointer);assert_eq!(std::ptr::addr_of!(publication),native_owner);
  }
  assert_eq!(publication.is_delivered(),descriptor.is_some());
  if let Some(mut descriptor)=descriptor{
   assert_eq!(copied,extent);let(view,heap)=observed(||publication.borrow_outcome(&descriptor).unwrap());assert_eq!(heap,(0,0));let payload=match view{JobOutcomeView::PreviewReady{payload,..}=>payload,JobOutcomeView::CheckpointReady{state,applied_progress,..}=>{assert_eq!(applied_progress,7);state},JobOutcomeView::Fault{detail,..}=>detail,_=>panic!("wrong original publication kind")};assert_eq!(payload.len(),extent);let pointer=payload as*const _;let mut bytes=Vec::new();for index in 0..payload.page_count(){bytes.extend_from_slice(payload.page(index).unwrap());}assert_eq!(serde_json::to_value(&bytes).unwrap(),serde_json::to_value(source).unwrap());assert_eq!(publication.published().unwrap()as*const _,pointer);
   let(turn,heap)=observed(||descriptor.acknowledge(RetainedCloneGrant{maximum_items:0,..quantum}));assert_eq!(heap,(0,0));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(!descriptor.is_acknowledged());
   caller.turn(quantum,|cx|{let turn=descriptor.acknowledge(RetainedCloneGrant{maximum_copy_bytes:0,..cx.retained_grant()});assert!(descriptor.is_acknowledged());assert_eq!(turn.result.as_ref().unwrap().progress(),RetainedCloneProgress{copied_items:1,..Default::default()});receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();let(_,heap)=observed(||drop(descriptor));assert_eq!(heap,(0,0));
  }
  assert_eq!(publication.retirement_work().unwrap(),if publication.terminal_is_empty(){law["closeWork"]["empty"].as_u64().unwrap()}else{law["closeWork"]["live"].as_u64().unwrap()});
  caller.cancel().cancel_now();let mut closes=0;while !publication.terminal_is_empty(){
   closes+=1;assert!(closes<law["maximumCloseTurns"].as_u64().unwrap()as usize,"original publication closure stalled");let(demand,heap)=observed(||publication.retirement_demands().unwrap());assert_eq!(heap,(0,0));
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..quantum}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..quantum}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..quantum})].into_iter().flatten(){let(turn,heap)=observed(||caller.with_effect(denied,|effect|publication.close_step(effect)).ok());assert_eq!(heap,(0,0));if let Some(turn)=turn{assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());}assert!(!publication.terminal_is_empty());}
   let quote=publication.retirement_work().unwrap();assert_eq!(quote,law["closeWork"]["live"].as_u64().unwrap());
   let fuel=caller.fuel_remaining();let(result,heap)=observed(||caller.turn(quantum,|cx|{let deadline=cx.deadline_us();cx.with_original_work_loan(law["closeWork"]["deniedBudget"].as_u64().unwrap(),deadline,|cx|{let demand=publication.retirement_demands()?;close_original_effect_step(cx,demand,|effect|semio_framework_value::RetainedWorkPoll::Complete(publication.close_step(effect))).map(|_|())})}));result.unwrap();assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());assert_eq!(caller.fuel_remaining(),fuel);assert_eq!(publication.retirement_work().unwrap(),quote);
   let(result,heap)=observed(||caller.turn(quantum,|cx|{let deadline=cx.deadline_us();cx.with_original_work_loan(quote,deadline,|cx|{let demand=publication.retirement_demands()?;close_original_effect_step(cx,demand,|effect|semio_framework_value::RetainedWorkPoll::Complete(publication.close_step(effect))).map(|_|())})}));result.unwrap();assert_eq!(fuel-caller.fuel_remaining(),quote);assert_physical(caller.receipt(),heap,quantum);born+=heap.0;released+=heap.1;
  }
  assert!(!publication.is_delivered());
  assert_eq!(publication.retirement_work().unwrap(),law["closeWork"]["empty"].as_u64().unwrap());
  let(_,heap)=observed(||drop(publication));assert_eq!(heap,(0,0));released+=caller.retire(stock);assert_eq!(born,released);assert_eq!(source.as_ptr(),source_pointer);
 }}}
}

#[test]
fn original_publication_refuses_another_real_caller_before_first_page_and_for_empty_sources(){
 fn clock()->Option<u64>{Some(0)}
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let quantum=grant(&law["originalGrant"]);let stock=grant(&law["originalStock"]);let bytes=[7u8,11,13];
 for extent in [0,bytes.len()]{
  let source=&bytes[..extent];let mut caller=OriginalJobFixtureCaller::admit(OperationId(99732),Generation(5),stock,clock,u64::MAX,law["originalFuel"].as_u64().unwrap());let mut foreign=OriginalJobFixtureCaller::admit(OperationId(99732),Generation(5),stock,clock,u64::MAX,law["originalFuel"].as_u64().unwrap());let mut publication=RetainedJobPublication::new();
  for _ in 0..3{
   caller.turn(quantum,|cx|{assert!(publication.advance_from_source(JobPublicationKind::Preview,source,cx)?.is_none());Ok(())}).unwrap();
   let fuel=foreign.fuel_remaining();let(result,heap)=observed(||foreign.turn(quantum,|cx|{publication.advance_from_source(JobPublicationKind::Preview,source,cx)?;Ok(())}));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(foreign.receipt(),Default::default());assert_eq!(foreign.fuel_remaining(),fuel);
  }
  let mut descriptor=None;for turn in 0..2000{if descriptor.is_some(){break}caller.turn(quantum,|cx|{if let Some(outcome)=publication.advance_from_source(JobPublicationKind::Preview,source,cx)?{descriptor=Some(outcome.into_descriptor())}Ok(())}).unwrap();assert!(turn<1999);}
  let mut descriptor=descriptor.unwrap();assert!(matches!(publication.borrow_outcome(&descriptor).unwrap(),JobOutcomeView::PreviewReady{..}));
  caller.turn(quantum,|cx|{receive_original_retained_work(cx,descriptor.acknowledge(cx.retained_grant()))?;Ok(())}).unwrap();drop(descriptor);
  for turn in 0..10000{if publication.terminal_is_empty(){break}caller.turn(quantum,|cx|{close_original_effect_step(cx,publication.retirement_demands()?,|effect|semio_framework_value::RetainedWorkPoll::Complete(publication.close_step(effect)))?;Ok(())}).unwrap();assert!(turn<9999);}
  assert!(publication.terminal_is_empty());caller.retire(stock);foreign.retire(stock);
 }
}
