//! 🧪️ Original export sources survive interruption and every denied retirement currency.
use super::*;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

fn pump_returns(issuer:&mut store::ArtifactStore<DrawingSnapshot,crate::op::DrawingMutation>,active:&mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>){let demand=if let Some(owner)=active.as_ref(){let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();store::artifact_retirement_box_demands(owner,if copy>0{copy}else{release}).unwrap()}else{issuer.returned_snapshot_read_retirement_demand().unwrap()};if active.is_none()&&issuer.returned_snapshot_read_count()==0{return;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(progress,physical)=observe(||{if active.is_some(){store::artifact_retirement_box_close_step(active,grant).unwrap().progress()}else{let(owner,progress)=issuer.take_returned_snapshot_read_retirement(grant).unwrap();*active=owner;progress}});assert!(progress.fits(grant));assert_eq!(physical.requested_bytes,progress.retained_capacity_bytes);assert_eq!(physical.released_bytes,progress.released_bytes);}
#[semio_framework_async_macros::async_test]
async fn original_export_read_returns_under_exact_physical_grants(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🏪️stores/🧫️fixtures/🔣️.json")).unwrap();
 assert_eq!(corpus["readReturn"]["exportConsumer"]["identity"],"original-read");
 for format in ["png","svg","pdf","unsupported"]{
  let document=DrawingSnapshot{id:"original-export-source".into(),..Default::default()};
  let mut issuer=store::ArtifactStore::new(store::create_document_envelope::<DrawingSnapshot,crate::op::DrawingMutation>("drawing.drawing","original-export-source",document,None),protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
  issuer.install_document_store_owners_exact(crate::spr::drawing_document_store_owners());
  let read=issuer.snapshot_read().unwrap();let pointer=read.get()as*const DrawingSnapshot;
  let mut work=Work::new(read,Default::default(),Default::default());
  work.begin_close();let mut returned=None;let mut saw_read=false;let(mut requested,mut released)=(0,0);
  for _ in 0..100000{
   pump_returns(&mut issuer,&mut returned);if work.terminal_is_empty(){break;}
   if let Some(read)=work.read.as_ref(){assert_eq!(read.get()as*const DrawingSnapshot,pointer);saw_read=true;}
   let demand=work.close_demands(4096).unwrap();
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for axis in corpus["readReturn"]["exportConsumer"]["denied"].as_array().unwrap(){
    let axis=axis.as_str().unwrap();let mut denied=grant;
    match axis{"items"=>denied.maximum_items=0,"copy" if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,"capacity" if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,"release" if demand.release_bytes>0=>denied.maximum_release_bytes=demand.release_bytes-1,"depth" if demand.depth>0=>denied.maximum_depth=demand.depth-1,_=>continue}
    let(step,event)=observe(||work.close_step(denied));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
    assert!(matches!(step,InteractiveJobCloseStep::Blocked|InteractiveJobCloseStep::Refused(_))||matches!(step,InteractiveJobCloseStep::Pending{progress}if progress==Default::default()));
    if let Some(read)=work.read.as_ref(){assert_eq!(read.get()as*const DrawingSnapshot,pointer);}
   }
   let(step,event)=observe(||work.close_step(grant));let progress=match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>progress,other=>panic!("Original export refused exact grant: {other:?}")};
   assert!(progress.fits(grant));assert!(!event.overflowed);assert_eq!(event.requested_bytes,progress.retained_capacity_bytes);assert_eq!(event.released_bytes,progress.released_bytes);requested+=event.requested_bytes;released+=event.released_bytes;
  }
  assert!(saw_read&&work.terminal_is_empty());for _ in 0..100000{if returned.is_none()&&issuer.returned_snapshot_read_count()==0{break;}pump_returns(&mut issuer,&mut returned);}assert!(returned.is_none()&&issuer.returned_snapshot_read_count()==0);let(_,event)=observe(||drop(work));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
  for _ in 0..100000{if issuer.close_owned_terminal_is_empty(){break;}let demand=issuer.close_owned_demands(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};issuer.close_owned_step(grant).unwrap();}
  assert!(issuer.close_owned_terminal_is_empty());eprintln!("[DEBUG] Original export source {format} returned exact pointer with capacity={requested} release={released} and zero terminal Drop");
 }
}
