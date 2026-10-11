use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
#[test]
fn original_erased_read_return_retains_source_on_every_refusal_and_returns_the_same_registry(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["erasedReturn"];let g=&law["grant"];let grant=RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize};
 let registry=crate::os_store::SnapshotReadRegistryHandle::new();let identity=registry.identity();let source=Arc::new(law["source"].as_str().unwrap().to_owned());let pointer=Arc::as_ptr(&source);let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("original erased read admission"));let index=lease.index;let generation=lease.generation;let mut read=super::super::ErasedSnapshotRead::new(source.clone(),lease);drop(source);
 for axis in law["refusals"].as_array().unwrap(){let denied=match axis.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:0,..grant},"depth"=>RetainedCloneGrant{maximum_depth:0,..grant},"busy"=>grant,_=>unreachable!()};let locked=(axis=="busy").then(||registry.state.try_lock().unwrap());let(result,heap)=observe(||read.try_return_to_registry_witness(denied));let(error,same)=result.err().expect("original erased source must survive refusal");assert!(matches!(error.kind,ValueRefusalKind::WorkLimit|ValueRefusalKind::DepthLimit|ValueRefusalKind::OwnershipLimit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(same.get::<String>().unwrap()as*const _,pointer);assert_eq!(same.lease.as_ref().unwrap().registry.identity(),identity);assert_eq!((same.lease.as_ref().unwrap().index,same.lease.as_ref().unwrap().generation),(index,generation));read=same;drop(locked);}
 let(result,heap)=observe(||read.try_return_to_registry_witness(grant));let(witness,progress)=result.ok().unwrap();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(law["expectedRequestedBytes"].as_u64().unwrap()as usize,law["expectedReleasedBytes"].as_u64().unwrap()as usize));assert_eq!(witness.registry.identity(),identity);assert_eq!((witness.index,witness.generation),(index,generation));assert!(!witness.terminal_is_empty());
 let mut original=None;for _ in 0..super::super::SNAPSHOT_READ_LEASE_CAPACITY{if let Some(owner)=registry.try_admit_one_returned::<String,_>(grant,|owner,_|Ok((owner,RetainedCloneProgress{copied_items:1,..Default::default()}))).unwrap().0{original=Some(owner);break;}}let original=original.unwrap();assert_eq!(Arc::as_ptr(&original),pointer);assert_eq!(original.as_str(),law["source"].as_str().unwrap());assert!(witness.terminal_is_empty());drop(original);let mut retirement=SnapshotReadReturnRetirement::new(witness);for _ in 0..8{if retirement.terminal_is_empty(){break;}let demand=retirement.demands().unwrap();retirement.step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()}).unwrap();}assert!(retirement.terminal_is_empty());
 println!("[DEBUG] original erased read source/registry/generation survive independent grant and busy refusals; returned exact slot retains original payload until Store maintenance");
}
#[test]
fn original_returned_read_registry_alias_has_exact_physical_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let source=Arc::new(17u32);
  let((witness,mut other,backing),allocation)=observe(||{
   let registry=crate::os_store::SnapshotReadRegistryHandle::new();
   let backing=registry.state.try_lock().unwrap().slots.as_ptr();
   let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("real source admission"));
   let read=super::super::SnapshotRead::new(source.clone(),lease);
   let witness=read.return_to_registry_witness().expect("real source returned once");
   assert!(witness.terminal_is_empty());
   let other=(row["otherAliases"].as_u64().unwrap()!=0).then_some(registry);
   (witness,other,backing)
  });
  assert!(!allocation.overflowed);assert_eq!(allocation.released_bytes,0);let original_bytes=allocation.requested_bytes;
  let(mut cursor,allocation)=observe(||SnapshotReadReturnRetirement::new(witness));assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));
  let demand=cursor.demands().unwrap();assert_eq!(demand.release_bytes,snapshot_registry_frame_bytes().unwrap());
  if row["dropOtherBeforeRelease"].as_bool().unwrap(){let(_,allocation)=observe(||drop(other.take()));assert_eq!(allocation.released_bytes,0);}
  for grant in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes-1,maximum_depth:1,..Default::default()}]{
   let(step,allocation)=observe(||cursor.step(grant).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));assert!(cursor.registry.is_some());
  }
  let mut released=0;
  for _ in 0..8{
   if cursor.terminal_is_empty(){break;}
   let demand=cursor.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};
   let(step,allocation)=observe(||cursor.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,progress.released_bytes);released+=progress.released_bytes;
   if let Some(owner)=cursor.last_registry.as_ref(){let state=owner.state.try_lock().unwrap();if !state.slots.is_empty(){assert_eq!(state.slots.as_ptr(),backing);}}
  }
  assert!(cursor.terminal_is_empty());assert_eq!(released,if row["expectLastCustody"].as_bool().unwrap(){original_bytes}else{0});
  let(_,allocation)=observe(||drop(cursor));assert_eq!(allocation.released_bytes,0);
  if let Some(registry)=other.take(){
   let mut remaining=SnapshotReadReturnRetirement::new(SnapshotReadReturn{registry,index:0,generation:0});
   for _ in 0..8{if remaining.terminal_is_empty(){break;}let demand=remaining.demands().unwrap();remaining.step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()}).unwrap();}
   assert!(remaining.terminal_is_empty());
  }
  assert_eq!(*source,17);eprintln!("[DEBUG] Returned source witness {} original={original_bytes} released={released} terminalDrop=0",row["id"]);
 }
}

#[test]
fn original_returned_read_registry_parallel_aliases_credit_exactly_one_frame(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let shares=fixture["parallelShares"].as_u64().unwrap()as usize;assert_eq!(shares,2);
 let source=Arc::new(31u32);
 let((left,right,identity),allocation)=observe(||{
  let registry=crate::os_store::SnapshotReadRegistryHandle::new();let identity=registry.identity();
  let read=||{let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("real parallel read admission"));super::super::SnapshotRead::new(source.clone(),lease).return_to_registry_witness().unwrap()};
  let left=read();let right=read();drop(registry);(left,right,identity)
 });
 assert!(!allocation.overflowed);assert_eq!(allocation.released_bytes,0);let original=allocation.requested_bytes;
 let barrier=std::sync::Barrier::new(shares);
 let drain=|witness|{
  let mut cursor=SnapshotReadReturnRetirement::new(witness);assert_eq!(cursor.registry.as_ref().unwrap().identity(),identity);barrier.wait();let mut released=0;let mut frame=false;
  for _ in 0..8{
   if cursor.terminal_is_empty(){break;}let demand=cursor.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};
   let(step,allocation)=observe(||cursor.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,progress.released_bytes);released+=progress.released_bytes;frame|=cursor.last_registry.is_some();
  }
  assert!(cursor.terminal_is_empty());let(_,allocation)=observe(||drop(cursor));assert_eq!(allocation.released_bytes,0);(released,frame)
 };
 let results=std::thread::scope(|scope|{let left=scope.spawn(||drain(left));let right=scope.spawn(||drain(right));[left.join().unwrap(),right.join().unwrap()]});
 assert_eq!(results.iter().map(|row|row.0).sum::<usize>(),original);assert_eq!(results.iter().filter(|row|row.1).count(),1);assert_eq!(*source,31);
 eprintln!("[DEBUG] Parallel original returned-read aliases frame-winners=1 original={original} physical={}",results.iter().map(|row|row.0).sum::<usize>());
}

#[test]
fn original_read_return_refusals_preserve_payload_and_lease(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugin/♻️lifecycle/🏪️stores/🧫️fixtures/🔣️.json")).unwrap();
 let source=Arc::new(41u32);let registry=crate::os_store::SnapshotReadRegistryHandle::new();let identity=registry.identity();let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("actual original read slot"));let index=lease.index;let generation=lease.generation;let mut read=super::super::SnapshotRead::new(source.clone(),lease);
 for refusal in fixture["readReturn"]["refusals"].as_array().unwrap(){
  let refusal=refusal.as_str().unwrap();let mut saved_owner=None;let mut busy=None;
  match refusal{
   "busy"=>busy=Some(registry.state.try_lock().unwrap()),
   "stale"=>registry.lease_generations[index as usize].store(generation+1,std::sync::atomic::Ordering::Release),
   "already-returned"=>registry.lease_generations[index as usize].store(generation|(1u64<<63),std::sync::atomic::Ordering::Release),
   "changed-owner"=>{let mut state=registry.state.try_lock().unwrap();let slot=unsafe{state.slots[index as usize].assume_init_mut()};saved_owner=Some(std::mem::replace(&mut slot.owner,Arc::new(42u32)));},
   _=>panic!("unknown neutral read refusal"),
  }
  let(result,heap)=observe(||read.try_return_to_registry_witness());read=match result{Err(original)=>original,Ok(_)=>panic!("refused read transferred original custody")};assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(read.get(),source.as_ref());assert_eq!(Arc::as_ptr(read.owner.as_ref().unwrap()),Arc::as_ptr(&source));assert_eq!(read.lease.as_ref().unwrap().generation,generation);assert_eq!(read.lease.as_ref().unwrap().registry.identity(),identity);
  drop(busy);registry.lease_generations[index as usize].store(generation,std::sync::atomic::Ordering::Release);if let Some(original)=saved_owner{let mut state=registry.state.try_lock().unwrap();let slot=unsafe{state.slots[index as usize].assume_init_mut()};slot.owner=original;}
 }
 let(result,heap)=observe(||read.try_return_to_registry_witness());let witness=match result{Ok(witness)=>witness,Err(_)=>panic!("valid original return was refused")};assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(witness.registry.identity(),identity);assert!(!witness.terminal_is_empty());assert_eq!(registry.returned.load(std::sync::atomic::Ordering::Acquire),1);let guard=registry.try_take(index,generation).unwrap();drop(guard);assert!(witness.terminal_is_empty());drop(registry);
 let mut cursor=SnapshotReadReturnRetirement::new(witness);for _ in 0..8{if cursor.terminal_is_empty(){break;}let demand=cursor.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(result,heap)=observe(||cursor.step(grant));assert_eq!(result.unwrap().progress().released_bytes,heap.released_bytes);assert_eq!(heap.requested_bytes,0);}assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!(heap.released_bytes,0);assert_eq!(*source,41);
}

#[test]
fn registered_owned_serializer_rejections_preserve_exact_read_and_options(){
 use crate::io::io_mechanism::{IoEntry,IoEntryDirection,OwnedSerializerFactory,OwnedSerializerRequest,OwnedSerializerAdmission,OwnedSerializerRefusal};
 use crate::io_schema::{IoFidelity,IoPayload,IoOutcome,IoResult};
 use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};
 static CALLS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
 const NATIVE:Dialect=Dialect{artifact_kind:"test.owned-source",standard:StandardId("1"),subset:SubsetId::ANY};
 fn demand(_: &OwnedSerializerRequest)->Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand,ValueError>{Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:128,depth:1})}
 fn refuse(request:OwnedSerializerRequest,_:RetainedCloneGrant)->Result<OwnedSerializerAdmission,OwnedSerializerRefusal>{CALLS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);Err(OwnedSerializerRefusal{error:ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"fixture refuses original request after valid generic admission"),request,progress:Default::default()})}
 fn transport(_: &IoPayload,_: &mut crate::io::io_mechanism::IoRunControl<'_,'_>)->IoResult<IoPayload>{Ok(IoOutcome::clean(IoPayload::Binary(Vec::new())))}
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🚪️io/📤️serialization/📦️owned/🧫️fixtures/🔣️.json")).unwrap();
 let source=Arc::new(73u32);let registry=crate::os_store::SnapshotReadRegistryHandle::new();let revision=[7u8;32];assert!(registry.publish_authority(9,revision));
 for row in corpus["cases"].as_array().unwrap(){
  let foreign=Dialect{artifact_kind:match row["format"].as_str().unwrap(){"png"=>"png","svg"=>"svg",_=>"pdf"},standard:StandardId("1"),subset:SubsetId::ANY};
  let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("genuine captured source"));let read=super::super::SnapshotRead::new(source.clone(),lease).into_erased();
  let options=semio_framework_value::DslValue::String(format!("original-options-{}",row["id"]));let options_pointer=options.as_str().unwrap().as_ptr();
  let request=OwnedSerializerRequest{source:read,route:(semio_framework_artifact_reference::ArtifactDialect::from(NATIVE),semio_framework_artifact_reference::ArtifactDialect::from(foreign)),generation:if row["current"].as_bool().unwrap(){9}else{8},revision,options,children:None};
  let entry=IoEntry{from:NATIVE,into:foreign,fidelity:IoFidelity::Exact,direction:if row["direction"]=="export"{IoEntryDirection::Export}else{IoEntryDirection::Import},sniff:None,run:transport,owned_serializer:row["capability"].as_bool().unwrap().then_some(OwnedSerializerFactory{demand,admit:refuse})};
  let g=&row["grant"];let grant=RetainedCloneGrant{maximum_items:g["items"].as_u64().unwrap()as usize,maximum_copy_bytes:g["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:g["release"].as_u64().unwrap()as usize,maximum_depth:g["depth"].as_u64().unwrap()as usize};let calls=CALLS.load(std::sync::atomic::Ordering::Relaxed);
  let(missing,heap)=observe(||crate::io::io_mechanism::io_begin_owned_serialization(request,grant));let missing=match missing{Err(refused)=>refused,Ok(_)=>panic!("unregistered original source route admitted")};assert_eq!(missing.error.kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner);assert_eq!(missing.progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(missing.request.source.get::<u32>().unwrap()as*const u32,Arc::as_ptr(&source));assert_eq!(missing.request.options.as_str().unwrap().as_ptr(),options_pointer);let request=missing.request;
  let(result,heap)=observe(||entry.begin_owned_serialization(request,grant));let refused=match result{Err(refused)=>refused,Ok(_)=>panic!("deliberately rejecting fixture admitted a job")};assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(refused.progress,Default::default());assert_eq!(refused.request.source.get::<u32>().unwrap()as*const u32,Arc::as_ptr(&source));assert_eq!(refused.request.options.as_str().unwrap().as_ptr(),options_pointer);assert_eq!(CALLS.load(std::sync::atomic::Ordering::Relaxed)-calls,row["factoryCalls"].as_u64().unwrap()as usize);
  let original=match refused.request.source.try_into_typed::<u32>(){Ok(read)=>read,Err(_)=>panic!("original type lost")};let witness=match original.try_return_to_registry_witness(){Ok(witness)=>witness,Err(_)=>panic!("exact source return refused")};let mut read_close=SnapshotReadReturnRetirement::new(witness);
  let birth=semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<u32>();let birth_grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth,maximum_depth:2,..Default::default()};
  let(returned,heap)=observe(||registry.try_admit_one_returned::<u32,_>(birth_grant,|owner,grant|semio_framework_value::retirement::shared::admit_shared_retirement(owner,grant,true)).unwrap());assert_eq!(heap.requested_bytes,returned.1.retained_capacity_bytes);assert_eq!(heap.released_bytes,returned.1.released_bytes);let mut slot_close=returned.0;assert!(slot_close.is_some());
  for _ in 0..1000{let Some(owner)=slot_close.as_ref()else{break;};let demand=crate::os_store::artifact_retirement_box_demands(owner,4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=observe(||crate::os_store::artifact_retirement_box_close_step(&mut slot_close,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(slot_close.is_none());
  for _ in 0..16{if read_close.terminal_is_empty(){break;}let demand=read_close.demands().unwrap();read_close.step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()}).unwrap();}assert!(read_close.terminal_is_empty());
  let mut options_close=semio_framework_value::retirement::controlled::ControlledRetirement::new(refused.request.options).unwrap_or_else(|_|panic!("native options controlled authority"));
  for _ in 0..1000{if options_close.terminal_is_empty(){break;}let copy=options_close.next_copy_byte_demand().unwrap();options_close.step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:options_close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:options_close.next_release_byte_demand().unwrap(),maximum_depth:options_close.next_depth_demand().unwrap()}).unwrap();}assert!(options_close.terminal_is_empty());
  eprintln!("[DEBUG] Registered serializer rejection {} retained exact source/options with zero boundary heap and calls={}",row["id"],row["factoryCalls"]);
 }
}

#[test]
fn original_registry_alias_close_requires_funded_atomic_last_custody(){
 use crate::os_store::SnapshotReadRegistryAliasRetirement as Alias;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🔒️registry/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let mut others=Vec::with_capacity(row["otherAliases"].as_u64().unwrap()as usize);
  let(registry,birth)=observe(crate::os_store::SnapshotReadRegistryHandle::new);assert!(!birth.overflowed);assert_eq!(birth.released_bytes,0);let identity=registry.identity();
  for _ in 0..row["otherAliases"].as_u64().unwrap(){others.push(Some(Alias::new(registry.clone())));}let mut original=Some(Alias::new(registry));
  let demand=crate::os_store::snapshot_registry_alias_demands(&original).unwrap();assert_eq!(demand.release_bytes,snapshot_registry_frame_bytes().unwrap(),"{} must fund its possible actual last header before atomic transfer",row["id"]);
  for grant in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes-1,maximum_depth:1,..Default::default()}]{
   let(step,heap)=observe(||crate::os_store::snapshot_registry_alias_close_step(&mut original,grant).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(original.as_ref().unwrap().identity(),identity);
  }
  let close=|slot:&mut Option<Alias>|{let(mut released,mut winners)=(0,0);for _ in 0..fixture["maximumSteps"].as_u64().unwrap(){if slot.is_none(){break;}let frame=slot.as_ref().unwrap().frame_byte_demand().unwrap();let demand=crate::os_store::snapshot_registry_alias_demands(slot).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,heap)=observe(||crate::os_store::snapshot_registry_alias_close_step(slot,grant).unwrap());assert_eq!(heap.requested_bytes,0);assert_eq!(step.progress().released_bytes,heap.released_bytes);assert!(step.progress().fits(grant));winners+=usize::from(frame!=0&&heap.released_bytes==frame);released+=heap.released_bytes;}assert!(slot.is_none());(released,winners)};
  let(mut released,mut winners)=(0,0);if row["closeOthersFirst"].as_bool().unwrap(){for other in &mut others{let result=close(other);released+=result.0;winners+=result.1;}}
  let result=close(&mut original);released+=result.0;winners+=result.1;for other in &mut others{let result=close(other);released+=result.0;winners+=result.1;}
  assert_eq!((released,winners),(birth.requested_bytes,1));eprintln!("[DEBUG] Original registry alias {} original={} physical={released} header-winners={winners} refused identity retained",row["id"],birth.requested_bytes);
 }
}

#[test]
fn original_registry_parallel_aliases_match_independent_arc_custody(){
 use crate::os_store::{SnapshotReadRegistryAliasRetirement as Alias,SnapshotReadRegistryHandle as Handle};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🔒️registry/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let shares=fixture["parallelShares"].as_u64().unwrap()as usize;assert_eq!(shares,2);let maximum=fixture["maximumSteps"].as_u64().unwrap();
 let((left,right),birth)=observe(||{let registry=Handle::new();let right=Alias::new(registry.clone());(Alias::new(registry),right)});assert!(!birth.overflowed);assert_eq!(birth.released_bytes,0);
 let barrier=std::sync::Barrier::new(shares);let drain=|owner:Alias|{let mut slot=Some(owner);barrier.wait();let(mut released,mut winners)=(0,0);for _ in 0..maximum{let Some(owner)=slot.as_ref()else{break;};let frame=owner.frame_byte_demand().unwrap();let demand=crate::os_store::snapshot_registry_alias_demands(&slot).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,heap)=observe(||crate::os_store::snapshot_registry_alias_close_step(&mut slot,grant).unwrap());assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);assert!(step.progress().fits(grant));released+=heap.released_bytes;winners+=usize::from(frame!=0&&heap.released_bytes==frame);}assert!(slot.is_none());let(_,heap)=observe(||drop(slot));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));(released,winners)};
 let actual=std::thread::scope(|scope|{let left=scope.spawn(||drain(left));let right=scope.spawn(||drain(right));[left.join().unwrap(),right.join().unwrap()]});let actual=(actual.iter().map(|row|row.0).sum::<usize>(),actual.iter().map(|row|row.1).sum::<usize>());assert_eq!(actual,(birth.requested_bytes,1));
 let((left,right),oracle_birth)=observe(||{let registry=Arc::new(super::super::SnapshotReadLeaseRegistry::new());let right=registry.clone();(registry,right)});assert!(!oracle_birth.overflowed);assert_eq!(oracle_birth.released_bytes,0);assert_eq!(oracle_birth.requested_bytes,birth.requested_bytes);
 let barrier=std::sync::Barrier::new(shares);let oracle=|alias:Arc<super::super::SnapshotReadLeaseRegistry>|{barrier.wait();let(last,header)=observe(||Arc::into_inner(alias));assert_eq!(header.requested_bytes,0);let Some(registry)=last else{assert_eq!(header.released_bytes,0);return(0,0);};assert_eq!(header.released_bytes,snapshot_registry_frame_bytes().unwrap());let demand=registry.empty_backing_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,backing)=observe(||registry.close_empty_backing_step(grant).unwrap());assert_eq!(backing.requested_bytes,0);assert_eq!(step.progress().released_bytes,backing.released_bytes);assert!(step.progress().fits(grant));let(_,terminal)=observe(||drop(registry));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));(header.released_bytes+backing.released_bytes,1)};
 let expected=std::thread::scope(|scope|{let left=scope.spawn(||oracle(left));let right=scope.spawn(||oracle(right));[left.join().unwrap(),right.join().unwrap()]});let expected=(expected.iter().map(|row|row.0).sum::<usize>(),expected.iter().map(|row|row.1).sum::<usize>());assert_eq!(actual,expected);eprintln!("[DEBUG] Parallel original registry aliases actual={actual:?} independent-Arc={expected:?} terminalDrop=0");
}
#[test]
fn original_registry_last_custody_preserves_unretired_slot_on_refusal(){
 use crate::os_store::{SnapshotReadRegistryAliasRetirement as Alias,SnapshotReadRegistryHandle as Handle};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🔒️registry/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let source=Arc::new(fixture["occupiedSlotValue"].as_u64().unwrap()as u32);let pointer=Arc::as_ptr(&source);
 let(registry,birth)=observe(Handle::new);assert!(!birth.overflowed);assert_eq!(birth.released_bytes,0);let identity=registry.identity();let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("original occupied-slot admission"));let index=lease.index;let generation=lease.generation;drop(lease);let mut slot=Some(Alias::new(registry));
 let demand=crate::os_store::snapshot_registry_alias_demands(&slot).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,heap)=observe(||crate::os_store::snapshot_registry_alias_close_step(&mut slot,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,demand.release_bytes));assert_eq!(heap.released_bytes,step.progress().released_bytes);let mut released=heap.released_bytes;let registry=slot.as_ref().unwrap();assert_eq!(registry.identity(),identity);assert_eq!(registry.frame_byte_demand().unwrap(),0);assert!(registry.contains(index,generation));
 let demand=crate::os_store::snapshot_registry_alias_demands(&slot).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(result,heap)=observe(||crate::os_store::snapshot_registry_alias_close_step(&mut slot,grant));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let registry=slot.as_ref().unwrap();assert_eq!(registry.identity(),identity);assert!(registry.contains(index,generation));
 let(owner,heap)=observe(||registry.try_take(index,generation).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.downcast_ref::<u32>().unwrap()as*const u32,pointer);let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 for _ in 0..fixture["maximumSteps"].as_u64().unwrap(){if slot.is_none(){break;}let demand=crate::os_store::snapshot_registry_alias_demands(&slot).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,heap)=observe(||crate::os_store::snapshot_registry_alias_close_step(&mut slot,grant).unwrap());assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);assert!(step.progress().fits(grant));released+=heap.released_bytes;}assert!(slot.is_none());assert_eq!(released,birth.requested_bytes);assert_eq!(*source,fixture["occupiedSlotValue"].as_u64().unwrap()as u32);eprintln!("[DEBUG] Original occupied registry retains exact slot on refusal; original={} physical={released} source preserved",birth.requested_bytes);
}