//! 🧪️ Original metadata owns every admitted field through genuine native preparation and cancellation.
use super::*;
use semio_framework_value::ToValue;
#[test]
fn shared_semantic_assembly_preserves_original_slots_and_exact_funded_custody(){
 use super::super::assembly::{OriginalPublicationAssembly,OriginalPublicationInputs};
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/📦️assembly/🔣️.json")).unwrap();
 let maximum=law["maximumSemanticTurns"].as_u64().unwrap()as usize;
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
 for cut in 0..maximum{
  let mut owner=OriginalPublicationAssembly::<bool,bool>::default();
  let mut forward=Some(true);let mut post=Some(false);let mut inverse=Some(vec![false,true,false]);
  let mut authority=Some(Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(921),semio_framework_job::Generation(0),[0;32],3,4,semio_framework_os_spr::HybridLogicalTimestamp{actor:1,physical_ms:42,logical:2},"semantic:α".into(),None,None,None)));
  let mut mutations:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<bool>>>=Some(Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<bool>::default()));
  let mut snapshots:Option<Arc<dyn SnapshotRetirementFactory<bool>>>=Some(Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<bool>::default()));
  for _ in 0..cut{
   if owner.sealer().is_some(){break;}
   let(demand,actual)=heap(||owner.advance_demands(authority.as_ref(),grant.maximum_copy_bytes).unwrap());assert_eq!(actual,(0,0));assert!(permits(demand,grant));
   let original=(forward,post,inverse.as_ref().map(|value|(value.as_ptr(),value.len())),authority.as_ref().map(Arc::as_ptr));
   for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}]{
    if permits(demand,denied){continue;}
    let(turn,actual)=heap(||owner.advance(OriginalPublicationInputs{forward:&mut forward,post:&mut post,inverse:&mut inverse,authority:&mut authority,mutations:&mut mutations,snapshots:&mut snapshots},denied));
    assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap(),None);assert_eq!(actual,(0,0));assert_eq!(owner.advance_demands(authority.as_ref(),grant.maximum_copy_bytes).unwrap(),demand);
    assert_eq!((forward,post,inverse.as_ref().map(|value|(value.as_ptr(),value.len())),authority.as_ref().map(Arc::as_ptr)),original);
   }
   let(turn,actual)=heap(||owner.advance(OriginalPublicationInputs{forward:&mut forward,post:&mut post,inverse:&mut inverse,authority:&mut authority,mutations:&mut mutations,snapshots:&mut snapshots},grant));let receipt=turn.result.unwrap().expect("funded semantic frontier advances");assert!(receipt.fits(grant));assert!(turn.work>0);assert_eq!(actual,(receipt.retained_capacity_bytes,receipt.released_bytes));
  }
  if owner.sealer().is_some(){assert!(forward.is_none()&&post.is_none()&&inverse.is_none()&&authority.is_none()&&mutations.is_none()&&snapshots.is_none());}
  for _ in 0..maximum*128{
   if owner.terminal_is_empty(){break;}
   let(demand,actual)=heap(||owner.close_demands(grant.maximum_copy_bytes).unwrap().unwrap());assert_eq!(actual,(0,0));assert!(permits(demand,grant));
   let(turn,actual)=heap(||owner.close_step(grant));let receipt=turn.result.unwrap().progress();assert!(receipt.fits(grant));assert_eq!(actual,(receipt.retained_capacity_bytes,receipt.released_bytes));
  }
  assert!(owner.terminal_is_empty());assert_eq!(heap(||drop(owner)).1,(0,0));
  let mut born=0;let mut released=0;drive_original((forward,post,inverse,authority),grant,maximum*128,&mut born,&mut released);
  for factory in [mutations.map(|factory|->Arc<dyn semio_framework_value::FactoryRetirement>{factory}),snapshots.map(|factory|->Arc<dyn semio_framework_value::FactoryRetirement>{factory})].into_iter().flatten(){drive_original(FactoryAuthority::new(factory),grant,maximum*128,&mut born,&mut released);}
 }
}
fn heap<T>(action:impl FnOnce()->T)->(T,(usize,usize)){let(value,born,freed)=semio_framework_trace_memory_testing::observe_backing(action);(value,(born,freed))}
/// 🧮️ Drives one original owner under a fixed policy; a controlled retirement gates its turn on capacity, release and depth, so only those axes are denied one below their quote (its copy quote is an advisory payload price).
fn drive_original<T:RetireOwned>(original:T,grant:RetainedCloneGrant,maximum_turns:usize,born:&mut usize,released:&mut usize){
 let mut owner=ControlledRetirement::new(original).unwrap_or_else(|(error,_)|panic!("original metadata cleanup: {error}"));
 for _ in 0..maximum_turns{if owner.terminal_is_empty(){break;}let quote=|owner:&ControlledRetirement<T>|->RetirementDemand{RetirementDemand{copy_bytes:owner.next_copy_byte_demand().unwrap(),capacity_bytes:owner.next_capacity_byte_demand(grant.maximum_copy_bytes).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()}};let(demand,actual)=heap(||quote(&owner));assert_eq!(actual,(0,0));assert!(permits(demand,grant),"fixed original metadata closure policy admits {demand:?}");
  for(axis,denied)in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}].into_iter().enumerate(){if axis==1||(axis==2&&demand.capacity_bytes==0)||(axis==3&&demand.release_bytes==0)||(axis==4&&demand.depth==0){continue;}let(result,actual)=heap(||owner.step(denied));assert!(result.as_ref().map_or(true,|step|step.progress()==Default::default()),"axis {axis} progressed under {denied:?} for demand {demand:?}: {result:?}");assert_eq!(actual,(0,0));assert_eq!(quote(&owner),demand);}
  let(step,actual)=heap(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(actual,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_ne!(step.progress(),Default::default());*born+=actual.0;*released+=actual.1;}
 assert!(owner.terminal_is_empty());assert_eq!(heap(||drop(owner)).1,(0,0));
}
#[test]
fn original_native_edit_metadata_preserves_shared_actors_order_and_all_partial_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let maximum_turns=law["maximumTurns"].as_u64().unwrap()as usize;assert_eq!(law["terminalDropBytes"],0);let n=|axis:&str|law["grant"][axis].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")};
 for actor in law["actors"].as_array().unwrap(){for repeat in law["repeats"].as_array().unwrap(){for cancelled_at in law["cancelAt"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize)).chain([None]){
  let text=actor.as_str().unwrap().repeat(repeat.as_u64().unwrap()as usize);let clock=semio_os_spr::HybridLogicalTimestamp{actor:law["clock"][0].as_u64().unwrap(),physical_ms:law["clock"][1].as_u64().unwrap(),logical:law["clock"][2].as_u64().unwrap()};let authority=ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(919), semio_framework_job::Generation(0), [0;32], law["baseAppliedEditCount"].as_u64().unwrap()as usize, law["sequence"].as_i64().unwrap()as i32, clock, text.clone().into(), Some(law["line"].as_str().unwrap().into()), Some(law["group"].as_str().unwrap().into()), None);
  let mut material=Vec::new();material.extend_from_slice(&(text.len()as u64).to_le_bytes());material.extend_from_slice(text.as_bytes());material.extend_from_slice(&authority.next_sequence_number().to_le_bytes());for value in[clock.actor,clock.physical_ms,clock.logical]{material.extend_from_slice(&value.to_le_bytes());}let reference=blake3::hash(&material);let expected_id=format!("edit-{}",&reference.to_hex()[..16]);
  let(source,actual)=heap(||{let mut source=Vec::with_capacity(law["inverseCapacity"].as_u64().unwrap()as usize);source.extend(law["inverse"].as_array().unwrap().iter().map(|value|value.as_bool().unwrap()));source});let held=actual.0-actual.1;let original_pointer=source.as_ptr();let mut inverse=OriginalVectorCursor::admit(source,grant).result.unwrap_or_else(|(error,_)|panic!("original inverse admission: {error}")).0;let mut forward=Some(law["forward"].as_bool().unwrap());let demand=NativeEditMetadata::<bool>::constructor_demand();
  for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,actual)=heap(||NativeEditMetadata::<bool>::admit(&authority,denied));assert!(result.is_err());assert_eq!(actual,(0,0));assert_eq!(inverse.remaining().as_ptr(),original_pointer);}
  let(result,actual)=heap(||NativeEditMetadata::<bool>::admit(&authority,grant));let(mut owner,receipt)=result.unwrap();assert_eq!(actual,(receipt.retained_capacity_bytes,receipt.released_bytes));let pointer=std::ptr::from_ref(owner.as_ref());let mut born=actual.0;let mut released=actual.1;let mut turns=0;let mut output=None;
  while !owner.ready()&&cancelled_at!=Some(turns){assert!(turns<maximum_turns);let(demand,actual)=heap(||owner.next_demand(&authority,&inverse).unwrap());assert_eq!(actual,(0,0));assert!(permits(demand,grant),"original fixed metadata policy admits {demand:?}");let phase=owner.phase;let original=(inverse.remaining().as_ptr(),inverse.remaining().len(),forward);
   for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}]{if permits(demand,denied){continue;}let(receipt,actual)=heap(||owner.step(&authority,&mut forward,&mut inverse,denied).unwrap());assert_eq!(receipt,Default::default());assert_eq!(actual,(0,0));assert_eq!(owner.phase,phase);assert_eq!(std::ptr::from_ref(owner.as_ref()),pointer);assert_eq!((inverse.remaining().as_ptr(),inverse.remaining().len(),forward),original);assert_eq!(owner.next_demand(&authority,&inverse).unwrap(),demand);}
   let(receipt,actual)=heap(||owner.step(&authority,&mut forward,&mut inverse,grant).unwrap());assert!(receipt.fits(grant));assert_eq!(actual,(receipt.retained_capacity_bytes,receipt.released_bytes));assert_ne!(receipt,Default::default());born+=actual.0;released+=actual.1;turns+=1;
  }
  let cancelled=cancelled_at==Some(turns);if !cancelled{assert!(owner.ready());let(result,actual)=heap(||owner.take(grant).unwrap());let(original,receipt)=result.unwrap();assert_eq!(actual,(0,0));assert!(receipt.fits(grant));assert_eq!(original.id,expected_id);assert_eq!(original.actor.as_ref().unwrap().as_ptr(),authority.actor().as_ptr());assert_eq!(original.mutation_meta[0].author_id.as_ref().unwrap().0.as_ptr(),authority.actor().as_ptr());assert_eq!(serde_json::to_value(&original.forwards).unwrap(),serde_json::Value::Array(vec![law["forward"].clone()]));assert_eq!(serde_json::to_value(original.inverse.iter().copied().collect::<Vec<_>>()).unwrap(),law["inverse"]);let semantic=serde_json::to_value(original.to_value()).unwrap();assert_eq!(semantic["actor"],serde_json::Value::String(text.clone()));assert_eq!(semantic["mutationMeta"][0]["author_id"],serde_json::Value::String(text.clone()));assert_eq!(semantic["sequenceNumber"],law["sequence"]);assert_eq!(semantic["line"],law["line"]);assert_eq!(semantic["mutationMeta"][0]["group_id"],law["group"]);assert_eq!(semantic["mutationMeta"][0]["base_version"],law["baseAppliedEditCount"]);assert_eq!(semantic["id"],serde_json::Value::String(expected_id));output=Some(original);assert!(owner.terminal_is_empty());}
  drive_original(((owner,inverse),(forward,output)),grant,maximum_turns,&mut born,&mut released);assert_eq!(released,held+born);assert_eq!(authority.actor(),text);
 }}}
}
