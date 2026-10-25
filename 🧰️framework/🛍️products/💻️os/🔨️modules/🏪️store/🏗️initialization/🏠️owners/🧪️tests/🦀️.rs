//! 🧪️ Original shared semantic source identity, denial and actual catalog cancellation frontiers.
use super::*;
use semio_framework_os_store_replay_operation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationRequest};
#[test]
fn original_shared_semantic_source_constructs_one_paired_issuer_and_closes_all_actual_frontiers(){
 type Owners=DocumentStoreOwners<DemoSnapshot,DemoMutation>;
 type Factory=FixtureAuthoringFactory<DemoMutation>;
 type Disposer=ArtifactStoreCursorDisposer<DemoSnapshot,DemoMutation>;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔣️.json")).unwrap();
 let row=&fixture["grant"];let n=|key:&str|row[key].as_u64().unwrap()as usize;
 let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 let children=semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0};
 let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Owners::shared_semantic_source_demands::<DemoSnapshotRetirementFactory,DemoInitialSnapshotRetirementFactory,DemoMutationRetirementFactory,Disposer,Factory>(children,0).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let calls=std::cell::Cell::new(0);
 let construct=||{calls.set(calls.get()+1);(DemoSnapshotRetirementFactory,DemoInitialSnapshotRetirementFactory,DemoMutationRetirementFactory,Disposer::new(),FixtureAuthoringFactory::<DemoMutation>(std::marker::PhantomData))};
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant},RetainedCloneGrant{maximum_depth:demand.depth-1,..grant}]{
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Owners::admit_source_constructor_with_shared_semantic_preparation(denied,children,0,construct));
  let error=result.err().expect("one below source authority is refused");assert!(error.owners.is_none());assert_eq!(error.progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(calls.get(),0);
 }
 for cut in fixture["cancelCuts"].as_array().unwrap(){
  let limit=cut.as_u64().unwrap()as usize;
  let(admitted,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Owners::admit_source_constructor_with_shared_semantic_preparation(RetainedCloneGrant{maximum_release_bytes:0,..grant},children,0,construct));
  let(mut owners,progress)=admitted.unwrap_or_else(|_|panic!("original independent source issuance admits its quoted concrete source"));assert!(owners.original_semantic_issuer_is_shared());assert!(progress.fits(grant));assert_eq!(progress.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,0));
  let publication=Arc::as_ptr(owners.one_item_preparation.as_ref().unwrap())as*const();let replay=Arc::as_ptr(owners.replay_preparation.as_ref().unwrap())as*const();assert_eq!(publication,replay);
  let mut born=heap.requested_bytes;let mut released=0;
  for _ in 0..limit{if owners.constructor_is_complete(){break;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.admit_constructor(grant));let p=step.unwrap_or_else(|_|panic!("original fixed catalog ticket issuance"));assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert!(owners.original_semantic_issuer_is_shared());assert_eq!(Arc::as_ptr(owners.one_item_preparation.as_ref().unwrap())as*const(),publication);}
  let mut terminal=false;for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if owners.uninstalled_owners_terminal_is_empty(){terminal=true;break;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owners.close_uninstalled_owners_step(grant).unwrap());let p=step.progress();assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}
  assert!(terminal);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owners));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,fixture["terminalDropBytes"].as_u64().unwrap()as usize));assert_eq!(born,released);
 }
}
