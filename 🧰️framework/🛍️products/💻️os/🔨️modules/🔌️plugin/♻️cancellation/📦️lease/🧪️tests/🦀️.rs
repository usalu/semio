use super::*;
use std::mem::ManuallyDrop;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
struct Heap{requested_bytes:usize,released_bytes:usize}
fn close(grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll)->(semio_framework_value::RetainedWorkOutcome<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>,Heap){
 let stock=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128};
 let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(61),semio_framework_job::Generation(1),stock,||Some(1),1_000_000,1_000_000_000);
 let turn=caller.turn(stock,|cx|Ok(cx.with_original_effect(grant,|_,effect|{let(poll,heap)=observe(||run(effect));(poll,Heap{requested_bytes:heap.requested_bytes,released_bytes:heap.released_bytes})})?)).expect("original fixture effect turn");caller.retire(stock);
 match turn{None=>(semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0},Heap{requested_bytes:0,released_bytes:0}),Some(((semio_framework_value::RetainedWorkPoll::Complete(done),heap),_))=>(done,heap),Some(_)=>panic!("original close fixture never suspends")}
}
fn original_key(id:u64)->ToolOperationKey{ToolOperationKey{app_instance_id:7,document:ArtifactDocumentAuthority(7),operation_id:semio_framework_job::OperationId(id),base_revision:semio_framework_job::RevisionId(0),generation:semio_framework_job::Generation(0)}}
fn close_original(cursor:&mut ToolCancellationLeaseRetirement,canonical:&ToolCancellationHandle){
 for _ in 0..4096{
  if cursor.terminal_is_empty(){return;}
  let demand=cursor.retirement_demands(canonical,7).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  let mut denials=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denials.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denials.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denials.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}
  for denied in denials{let(step,heap)=close(denied,|effect|cursor.close_step(canonical,effect));assert_eq!(step.work,0);assert_eq!(step.result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let work=cursor.retirement_work_demand(7).unwrap();let(step,heap)=close(grant,|effect|cursor.close_step(canonical,effect));assert!(step.work<=work);let progress=step.result.unwrap().progress();assert!(progress.fits(grant));assert_eq!(progress.retained_capacity_bytes,heap.requested_bytes);assert_eq!(progress.released_bytes,heap.released_bytes);
 }
 panic!("original lease did not complete within its concrete children");
}
#[test]
fn original_cancellation_lease_detaches_scopes_and_returns_each_actual_child(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let canonical=ManuallyDrop::new(ToolCancellationHandle::default());let keyed=row["keyed"].as_bool().unwrap();let original=if keyed{canonical.begin_keyed(original_key(1)).unwrap()}else{canonical.begin(original_key(1)).unwrap()};let sibling=if row["operations"]==2{Some(canonical.begin_keyed(original_key(2)).unwrap())}else{None};
  if row["permit"].as_bool().unwrap(){let(permit,heap)=observe(||original.try_claim_publication().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(_,heap)=observe(||drop(permit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let original_claim=ManuallyDrop::new(original.publication_claim.clone());let mut source=Some(original);let frame=std::mem::size_of::<ToolCancellationLeaseRetirement>();let birth=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<ToolCancellationLease>(),maximum_capacity_bytes:frame,maximum_depth:1,..Default::default()};
  for denied in [RetainedCloneGrant{maximum_items:0,..birth},RetainedCloneGrant{maximum_copy_bytes:birth.maximum_copy_bytes-1,..birth},RetainedCloneGrant{maximum_capacity_bytes:frame-1,..birth},RetainedCloneGrant{maximum_depth:0,..birth}]{let(result,heap)=observe(||original_lease_cursor_admit(&mut source,denied));assert!(result.unwrap().0.is_none());assert!(source.is_some());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(result,heap)=observe(||original_lease_cursor_admit(&mut source,birth));let(owner,progress)=result.unwrap();assert!(source.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(frame,0));assert_eq!(progress.retained_capacity_bytes,frame);let mut frame_owner=owner;let cursor=frame_owner.as_mut().unwrap();
  let demand=cursor.retirement_demands(&canonical,7).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:demand.depth,..Default::default()};
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=close(denied,|effect|cursor.close_step(&canonical,effect));assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),Default::default());assert!(!cursor.detached);assert!(!original_claim.is_finished());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(step,heap)=close(grant,|effect|cursor.close_step(&canonical,effect));assert_eq!(step.work,1);assert!(step.result.is_ok());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(original_claim.is_finished());
  assert_eq!(canonical.active.load(std::sync::atomic::Ordering::Acquire),row["expectedActive"].as_u64().unwrap()as usize);let state=canonical.state.try_lock().unwrap();if keyed{assert!(state.keyed_operations.get(1).is_none());assert_eq!(state.keyed_documents.get(7).map_or(0,|scope|scope.active),row["expectedDocumentActive"].as_u64().unwrap()as usize);if sibling.is_some(){assert!(state.keyed_operations.get(2).is_some());}}else{assert!(state.get(ToolCancellationHandle::slot(ArtifactDocumentAuthority(7))).is_none());}drop(state);
  close_original(cursor,&canonical);
  let release=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:frame,maximum_depth:1,..Default::default()};for denied in [RetainedCloneGrant{maximum_items:0,..release},RetainedCloneGrant{maximum_release_bytes:frame-1,..release},RetainedCloneGrant{maximum_depth:0,..release}]{let(result,heap)=observe(||original_lease_cursor_release(&mut frame_owner,denied));assert_eq!(result.unwrap(),Default::default());assert!(frame_owner.is_some());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(result,heap)=observe(||original_lease_cursor_release(&mut frame_owner,release));assert_eq!(result.unwrap().released_bytes,frame);assert!(frame_owner.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,frame));
  let mut alias=Some(ManuallyDrop::into_inner(original_claim));let mut active=None;while alias.is_some()||active.is_some(){let demand=plugin_typed_owner_demand(&alias,&active,7).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let quote=semio_plugin_ownership::bounded_ownership::plugin_typed_owner_work_demand(&alias,&active,7).unwrap();let(step,heap)=close(grant,|effect|plugin_typed_owner_original(&mut alias,&mut active,effect));assert!(step.work<=quote);let progress=step.result.unwrap().progress();assert_eq!(progress.retained_capacity_bytes,heap.requested_bytes);assert_eq!(progress.released_bytes,heap.released_bytes);}
  if let Some(sibling)=sibling{close_original(&mut ToolCancellationLeaseRetirement::new(sibling),&canonical);}
  assert_eq!(canonical.active.load(std::sync::atomic::Ordering::Acquire),0);
 }
}

#[test]
fn original_witness_selection_denial_does_not_attempt_lookup(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let denied_case=&fixture["witnessSelection"][0];let paid_case=&fixture["witnessSelection"][1];
 let canonical=ManuallyDrop::new(ToolCancellationHandle::default());
 let parent=canonical.app_scope.clone();
 let mut parts=CancellationLeaseParts::unpublished(None,[Some(parent.clone()),None,None],[None,None,None,None],Some(7));
 let full=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128};
 let first=close(full,|effect|parts.close_step(&canonical,&parent,effect)).0;assert_eq!(first.work,1);first.result.unwrap();
 WITNESS_LOOKUPS.with(|count|count.set(0));
 let stock=RetainedCloneGrant{maximum_items:1024,..full};
 let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(22),semio_framework_job::Generation(3),stock,||Some(1),100,10000);
 caller.turn(stock,|cx|cx.with_original_retirement(|cx|{let deadline=cx.deadline_us();cx.with_original_work_loan(0,deadline,|cx|{let demand=parts.retirement_demands(&canonical,&parent,full.maximum_copy_bytes)?;let work=parts.retirement_work_demand(full.maximum_copy_bytes)?;assert_eq!(work,denied_case["quote"].as_u64().unwrap());let step=semio_framework_job::close_original_effect_step(cx,demand,|effect|parts.close_step(&canonical,&parent,effect))?;assert_eq!(step.progress(),Default::default());assert_eq!(WITNESS_LOOKUPS.with(|count|count.get()),0);Ok(())})})).unwrap();
 caller.retire(stock);
 let denied=RetainedCloneGrant{maximum_items:0,..full};
 let turn=close(denied,|effect|parts.close_step(&canonical,&parent,effect)).0;assert_eq!(turn.work,0);turn.result.unwrap();
 assert_eq!(WITNESS_LOOKUPS.with(|count|count.get()),0);
 let paid=close(full,|effect|parts.close_step(&canonical,&parent,effect)).0;assert_eq!(paid.work,1);paid.result.unwrap();assert_eq!(WITNESS_LOOKUPS.with(|count|count.get()) as u64,paid_case["lookups"].as_u64().unwrap());
 for _ in 0..1000{if parts.terminal_is_empty(){break}close(full,|effect|parts.close_step(&canonical,&parent,effect)).0.result.unwrap();}
 assert!(parts.terminal_is_empty());
}

#[test]
fn original_blocked_token_reselects_a_live_alias(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert!(fixture["blockedReselect"].as_bool().unwrap());
 let canonical=ManuallyDrop::new(ToolCancellationHandle::default());let parent=canonical.app_scope.clone();
 let mut parts=CancellationLeaseParts::unpublished(None,[Some(parent.clone()),None,None],[None,None,None,None],None);
 let full=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128};
 close(full,|effect|parts.close_step(&canonical,&parent,effect)).0.result.unwrap();parts.token_witness[0]=TokenWitness::Close;
 let quote=parts.retirement_work_demand(full.maximum_copy_bytes).unwrap();let blocked=close(full,|effect|parts.close_step(&canonical,&parent,effect)).0;assert!(blocked.work<=quote);assert_eq!(blocked.work,2);blocked.result.unwrap();assert!(matches!(parts.token_witness[0],TokenWitness::Select));
 for _ in 0..1000{if parts.terminal_is_empty(){break}close(full,|effect|parts.close_step(&canonical,&parent,effect)).0.result.unwrap();}assert!(parts.terminal_is_empty());
}
