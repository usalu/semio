//! 🧪️ Original ephemeral request custody survives each denied birth and cancellation prefix.
use super::*;
use semio_framework_job::{Generation,OperationId,OriginalJobDecodeOwner,OriginalJobFixtureCaller,OriginalTurnFailure};
use store::ArtifactEphemeralOneItemPreparation;
use store::os_store::one_item::ArtifactStoreOneItemGrant as OneItemGrant;
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Unavailable;
impl ArtifactReplayPreparationFactory<bool,bool> for Unavailable{
 fn preparation_birth_demand(&self)->RetirementDemand{Default::default()}
 fn begin(&self,request:ArtifactReplayPreparationRequest<bool,bool>,_:RetainedCloneGrant)->Result<(Box<dyn semio_os_store_replay_operation::ArtifactReplayPreparation<bool,bool>>,RetainedCloneProgress),(ValueError,ArtifactReplayPreparationRequest<bool,bool>)>{Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"fixture has no semantic producer"),request))}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn closing(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.max(4096),maximum_items:2,maximum_depth:d.depth.max(1)+2,maximum_capacity_bytes:d.capacity_bytes+4096,maximum_release_bytes:d.release_bytes+4096,..grant(d)}}
fn close_clock()->Option<u64>{Some(1)}
fn close_turn(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,owner:&mut Preparation<bool,bool>,g:RetainedCloneGrant,controlled:bool)->(Result<RetainedCloneStep,String>,(usize,usize)){
 caller.retirement_turn_with_decoder(g,decoder,|cx|{
  let issued=cx.retained_grant();let body=issued.maximum_copy_bytes;
  let(outcome,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_effect(issued,|same,effect|{if controlled{same.with_native_retirement(|control|Ok(owner.close_step_controlled(body,effect,control)))}else{Ok(owner.close_step(body,effect))}}));
  let Some((poll,performed))=outcome.map_err(OriginalTurnFailure::from)? else{return Ok((Ok(RetainedCloneStep::Progress(Default::default())),(0,0)))};
  cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
  let poll=match poll{Ok(poll)=>poll,Err(error)=>return Ok((Err(format!("{error:?}")),(heap.requested_bytes,heap.released_bytes)))};
  let step=match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(|error|format!("{error:?}")),semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default()))};
  Ok((step,(heap.requested_bytes,heap.released_bytes)))
 }).unwrap()
}
#[test]
fn ephemeral_original_request_denials_and_cancellation_preserve_exact_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let stock=RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:4194304,maximum_capacity_bytes:4194304,maximum_release_bytes:4194304,maximum_depth:128};
 let factory=Factory{snapshots:Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<bool>::default()),mutations:Arc::new(semio_framework_value_factory::OwnedValueRetirementFactory::<bool>::default()),semantics:Arc::new(Unavailable)};
 for cut in law["cancelBeforeStages"].as_array().unwrap(){
  let registry=semio_framework_value_snapshot_read::SnapshotReadRegistryHandle::new();let root=Arc::new(law["source"].as_bool().unwrap());let lease=registry.try_issue(Arc::clone(&root)).unwrap();let pointer=Arc::as_ptr(&root);
  let request=store::ArtifactEphemeralOneItemPreparationRequest{operation:semio_framework_job::OperationId(7),generation:semio_framework_job::Generation(11),base:store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::TransientRead(SnapshotRead::new(root,lease))),mutation:true};
  let mut owner=factory.construct(request).map_err(|_|()).unwrap();
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(7),Generation(11),stock,close_clock,600000000,1000000);let mut decoder=caller.admitted_decoder();
  for _ in 0..cut.as_u64().unwrap(){
   let d=owner.demands(4096).unwrap();let funded=grant(d);let mut receiving=None;
   caller.turn_with_decoder(stock,&mut decoder,|cx|{
    let ran=cx.with_native_receiving(&mut receiving,|native,_|{
     for axis in law["deniedAxes"].as_array().unwrap(){let mut denied=funded;match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copyBytes" if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,"capacityBytes" if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,"depth"=>denied.maximum_depth=0,_=>continue};let before=owner.phase;let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.advance(OneItemGrant::from_retained(denied),native).unwrap());assert_eq!(step.ownership_progress(),Default::default());assert_eq!(owner.phase,before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
     let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.advance(OneItemGrant::from_retained(funded),native).unwrap());let receipt=step.ownership_progress();assert!(receipt.fits(funded));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
     Ok(())
    })?;
    assert!(ran,"the original receiving scope was admitted");Ok(())
   }).unwrap();
  }
  assert_eq!(owner.base.as_ref().unwrap().get()as*const bool,pointer);assert_eq!(serde_json::to_value(owner.base.as_ref().unwrap().get()).unwrap(),law["source"]);
  drop(registry);
  owner.begin_close();
  if owner.original.is_some(){
   let d=owner.close_demand(4096).unwrap();let before=owner.phase;
   let(missing,heap)=close_turn(&mut caller,&mut decoder,&mut owner,closing(d),false);assert!(missing.unwrap_err().contains(law["missingControl"].as_str().unwrap()));assert_eq!(owner.phase,before);assert_eq!((heap.0,heap.1),(0,0));
   let mut foreign=caller.admitted_decoder();let mut spent=None;caller.turn_with_decoder(stock,&mut foreign,|cx|{assert!(cx.with_native_receiving(&mut spent,|_,_|Ok(()))?);Ok(())}).unwrap();
   let(refused,heap)=close_turn(&mut caller,&mut foreign,&mut owner,closing(d),true);assert!(refused.unwrap_err().contains(law["foreignControl"].as_str().unwrap()));assert_eq!(owner.phase,before);assert_eq!((heap.0,heap.1),(0,0));
   while !foreign.terminal_is_empty(){caller.turn(stock,|cx|{foreign.close_on_source(cx)?;Ok(())}).unwrap();}
  }
  for turn in 0..100000{
   if owner.empty(){break;}assert!(turn<99999,"original ephemeral prefix failed to close");let d=owner.close_demand(4096).unwrap();let g=closing(d);
   let controlled=owner.requires_original_control();let(step,heap)=close_turn(&mut caller,&mut decoder,&mut owner,g,controlled);let step=step.unwrap();assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
  }
  assert_eq!(owner.empty(),law["terminal"].as_bool().unwrap());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  while !decoder.terminal_is_empty(){caller.turn(stock,|cx|{decoder.close_on_source(cx)?;Ok(())}).unwrap();}
  caller.retire(stock);
 }
}
