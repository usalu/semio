//! 🧪️ Original ephemeral request custody survives each denied birth and cancellation prefix.
use super::*;
use store::ArtifactEphemeralOneItemPreparation;
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Unavailable;
impl ArtifactReplayPreparationFactory<bool,bool> for Unavailable{
 fn preparation_birth_demand(&self)->RetirementDemand{Default::default()}
 fn begin(&self,request:ArtifactReplayPreparationRequest<bool,bool>,_:RetainedCloneGrant)->Result<(Box<dyn semio_framework_os_store_replay_operation::ArtifactReplayPreparation<bool,bool>>,RetainedCloneProgress),(ValueError,ArtifactReplayPreparationRequest<bool,bool>)>{Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"fixture has no semantic producer"),request))}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
#[test]
fn ephemeral_original_request_denials_and_cancellation_preserve_exact_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let factory=Factory{snapshots:Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<bool>::default()),mutations:Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<bool>::default()),semantics:Arc::new(Unavailable)};
 for cut in law["cancelBeforeStages"].as_array().unwrap(){
  let registry=semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();let root=Arc::new(law["source"].as_bool().unwrap());let lease=registry.try_issue(Arc::clone(&root)).unwrap();let pointer=Arc::as_ptr(&root);
  let request=store::ArtifactEphemeralOneItemPreparationRequest{operation:semio_framework_job::OperationId(7),generation:semio_framework_job::Generation(11),base:store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::TransientRead(SnapshotRead::new(root,lease))),mutation:true};
  let mut owner=factory.construct(request).map_err(|_|()).unwrap();
  let mut callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut native=semio_framework_value::NativeDecodeControl::new_retained(&mut callback);native.install_retirement_recipient(&mut recipient).unwrap();
  for _ in 0..cut.as_u64().unwrap(){
   let d=owner.demands(4096).unwrap();let funded=grant(d);
   for axis in law["deniedAxes"].as_array().unwrap(){let mut denied=funded;match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copyBytes" if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,"capacityBytes" if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,"depth"=>denied.maximum_depth=0,_=>continue};let before=owner.phase;let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.advance(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(denied),&mut native).unwrap());assert_eq!(step.ownership_progress(),Default::default());assert_eq!(owner.phase,before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.advance(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(funded),&mut native).unwrap());let receipt=step.ownership_progress();assert!(receipt.fits(funded));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
  }
  assert_eq!(owner.base.as_ref().unwrap().get()as*const bool,pointer);assert_eq!(serde_json::to_value(owner.base.as_ref().unwrap().get()).unwrap(),law["source"]);
  drop(registry);
  owner.begin_close();
  if owner.original.is_some(){let d=owner.close_demand(4096).unwrap();let missing=owner.close_step(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(grant(d))).unwrap_err();assert_eq!(format!("{:?}",missing.kind),law["missingControl"]);}
  if owner.original.is_some(){
   let d=owner.close_demand(4096).unwrap();let g=grant(d);let before=owner.phase;
   let mut foreign_callback=|_:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|true;let mut foreign_recipient=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();let mut foreign=semio_framework_value::NativeDecodeControl::new_retained(&mut foreign_callback);foreign.install_retirement_recipient(&mut foreign_recipient).unwrap();
   let(error,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.with_retirement(g,|control|owner.close_step_controlled(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(g),control)).unwrap_err());assert_eq!(format!("{:?}",error.kind),law["foreignControl"]);assert_eq!(owner.phase,before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
  for turn in 0..100000{
   if owner.empty(){break;}assert!(turn<99999,"original ephemeral prefix failed to close");let d=owner.close_demand(4096).unwrap();let g=grant(d);
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||native.with_retirement(g,|control|owner.close_step_controlled(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(g),control)).unwrap());assert!(step.progress().fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
  }
  assert_eq!(owner.empty(),law["terminal"].as_bool().unwrap());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
}
