use super::*;
use std::mem::ManuallyDrop;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn original_key(id:u64)->ToolOperationKey{ToolOperationKey{app_instance_id:7,document:ArtifactDocumentAuthority(7),operation_id:semio_framework_job::OperationId(id),base_revision:semio_framework_job::RevisionId(0),generation:semio_framework_job::Generation(0)}}
fn close_original(cursor:&mut ToolCancellationLeaseRetirement,canonical:&ToolCancellationHandle){
 for _ in 0..4096{
  if cursor.terminal_is_empty(){return;}
  let demand=cursor.retirement_demands(canonical,7).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
  let mut denials=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denials.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denials.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denials.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}
  for denied in denials{let(step,heap)=observe(||cursor.close_step(canonical,denied));assert_eq!(step.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(step,heap)=observe(||cursor.close_step(canonical,grant));let progress=step.unwrap().progress();assert!(progress.fits(grant));assert_eq!(progress.retained_capacity_bytes,heap.requested_bytes);assert_eq!(progress.released_bytes,heap.released_bytes);
 }
 panic!("original lease did not complete within its concrete children");
}
#[test]
fn original_cancellation_lease_detaches_scopes_and_returns_each_actual_child(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let canonical=ManuallyDrop::new(ToolCancellationHandle::default());let keyed=row["keyed"].as_bool().unwrap();let original=if keyed{canonical.begin_keyed(original_key(1)).unwrap()}else{canonical.begin(original_key(1)).unwrap()};let sibling=if row["operations"]==2{Some(canonical.begin_keyed(original_key(2)).unwrap())}else{None};
  if row["permit"].as_bool().unwrap(){let(permit,heap)=observe(||original.try_claim_publication().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(_,heap)=observe(||drop(permit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let original_claim=ManuallyDrop::new(original.publication_claim.clone());let mut source=Some(original);let frame=std::mem::size_of::<ToolCancellationLeaseRetirement>();let birth=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:std::mem::size_of::<ToolCancellationLease>(),maximum_capacity_bytes:frame,maximum_depth:1,..Default::default()};
  for denied in [RetainedCloneGrant{maximum_items:0,..birth},RetainedCloneGrant{maximum_copy_bytes:birth.maximum_copy_bytes-1,..birth},RetainedCloneGrant{maximum_capacity_bytes:frame-1,..birth},RetainedCloneGrant{maximum_depth:0,..birth}]{let(result,heap)=observe(||original_lease_cursor_admit(&mut source,denied));assert!(result.unwrap().0.is_none());assert!(source.is_some());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(result,heap)=observe(||original_lease_cursor_admit(&mut source,birth));let(owner,progress)=result.unwrap();assert!(source.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(frame,0));assert_eq!(progress.retained_capacity_bytes,frame);let mut frame_owner=owner;let cursor=frame_owner.as_mut().unwrap();
  let demand=cursor.retirement_demands(&canonical,7).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:demand.depth,..Default::default()};
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe(||cursor.close_step(&canonical,denied));assert_eq!(result.unwrap().progress(),Default::default());assert!(!cursor.detached);assert!(!original_claim.is_finished());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(step,heap)=observe(||cursor.close_step(&canonical,grant));assert!(step.is_ok());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(original_claim.is_finished());
  assert_eq!(canonical.active.load(std::sync::atomic::Ordering::Acquire),row["expectedActive"].as_u64().unwrap()as usize);let state=canonical.state.try_lock().unwrap();if keyed{assert!(state.keyed_operations.get(1).is_none());assert_eq!(state.keyed_documents.get(7).map_or(0,|scope|scope.active),row["expectedDocumentActive"].as_u64().unwrap()as usize);if sibling.is_some(){assert!(state.keyed_operations.get(2).is_some());}}else{assert!(state.get(ToolCancellationHandle::slot(ArtifactDocumentAuthority(7))).is_none());}drop(state);
  close_original(cursor,&canonical);
  let release=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:frame,maximum_depth:1,..Default::default()};for denied in [RetainedCloneGrant{maximum_items:0,..release},RetainedCloneGrant{maximum_release_bytes:frame-1,..release},RetainedCloneGrant{maximum_depth:0,..release}]{let(result,heap)=observe(||original_lease_cursor_release(&mut frame_owner,denied));assert_eq!(result.unwrap(),Default::default());assert!(frame_owner.is_some());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(result,heap)=observe(||original_lease_cursor_release(&mut frame_owner,release));assert_eq!(result.unwrap().released_bytes,frame);assert!(frame_owner.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,frame));
  let mut alias=Some(ManuallyDrop::into_inner(original_claim));let mut active=None;while alias.is_some()||active.is_some(){let demand=plugin_typed_owner_demand(&alias,&active,7).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=observe(||plugin_typed_owner_step(&mut alias,&mut active,grant));let progress=step.unwrap().progress();assert_eq!(progress.retained_capacity_bytes,heap.requested_bytes);assert_eq!(progress.released_bytes,heap.released_bytes);}
  if let Some(sibling)=sibling{close_original(&mut ToolCancellationLeaseRetirement::new(sibling),&canonical);}
  assert_eq!(canonical.active.load(std::sync::atomic::Ordering::Acquire),0);eprintln!("[DEBUG] original cancellation lease {} child custody completed; canonical owner remains unsupported",row["name"]);
 }
}
