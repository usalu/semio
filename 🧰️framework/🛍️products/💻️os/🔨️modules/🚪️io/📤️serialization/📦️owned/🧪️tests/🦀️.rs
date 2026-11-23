//! 🧪️ A registered owned serializer that refuses keeps the exact original read and options, and every closure drains under its own grant.
#![allow(clippy::all)]
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::ValueError;
use semio_framework_value_snapshot_read::{SnapshotRead, SnapshotReadRegistryHandle};
use std::sync::Arc;

use semio_framework_value_testing_allocator as _;
use semio_framework_value_testing_admission::{BankWallet,EffectBank,RetainedWorkPoll,with_original_invocation};
use semio_framework_value::{RetainedCloneProgress,RetainedWorkOutcome};
fn close_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:16}}

#[test]
fn registered_owned_serializer_rejections_preserve_exact_read_and_options(){
 use semio_os_io::io_mechanism::{IoEntry,IoEntryDirection,OwnedSerializerFactory,OwnedSerializerRequest,OwnedSerializerAdmission,OwnedSerializerRefusal};
 use semio_os_io::io_schema::{IoFidelity,IoPayload,IoOutcome,IoResult};
 use semio_framework_artifact_reference::{Dialect,StandardId,SubsetId};
 static CALLS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
 const NATIVE:Dialect=Dialect{artifact_kind:"test.owned-source",standard:StandardId("1"),subset:SubsetId::ANY};
 fn demand(_: &OwnedSerializerRequest)->Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand,ValueError>{Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:128,depth:1})}
 fn refuse(request:OwnedSerializerRequest,_:RetainedCloneGrant)->Result<OwnedSerializerAdmission,OwnedSerializerRefusal>{CALLS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);Err(OwnedSerializerRefusal{error:ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"fixture refuses original request after valid generic admission"),request,progress:Default::default()})}
 fn transport(_: &IoPayload,_: &mut semio_os_io::io_mechanism::IoRunControl<'_,'_>)->IoResult<IoPayload>{Ok(IoOutcome::clean(IoPayload::Binary(Vec::new())))}
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let source=Arc::new(73u32);let registry=SnapshotReadRegistryHandle::new();let revision=[7u8;32];assert!(registry.publish_authority(9,revision));
 for row in corpus["cases"].as_array().unwrap(){
  let foreign=Dialect{artifact_kind:match row["format"].as_str().unwrap(){"png"=>"png","svg"=>"svg",_=>"pdf"},standard:StandardId("1"),subset:SubsetId::ANY};
  let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("genuine captured source"));let read=SnapshotRead::new(source.clone(),lease).into_erased();
  let options=semio_framework_value::DslValue::String(format!("original-options-{}",row["id"]));let options_pointer=options.as_str().unwrap().as_ptr();
  let request=OwnedSerializerRequest{source:read,route:(semio_framework_artifact_reference::schema::ArtifactDialect::from(NATIVE),semio_framework_artifact_reference::schema::ArtifactDialect::from(foreign)),generation:if row["current"].as_bool().unwrap(){9}else{8},revision,options,children:None};
  let entry=IoEntry{from:NATIVE,into:foreign,fidelity:IoFidelity::Exact,direction:if row["direction"]=="export"{IoEntryDirection::Export}else{IoEntryDirection::Import},sniff:None,run:transport,owned_serializer:row["capability"].as_bool().unwrap().then_some(OwnedSerializerFactory{demand,admit:refuse})};
  let g=&row["grant"];let grant=RetainedCloneGrant{maximum_items:g["items"].as_u64().unwrap()as usize,maximum_copy_bytes:g["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:g["release"].as_u64().unwrap()as usize,maximum_depth:g["depth"].as_u64().unwrap()as usize};let calls=CALLS.load(std::sync::atomic::Ordering::Relaxed);
  let(missing,heap)=observe(||semio_os_io::io_mechanism::owned_serialization::io_begin_owned_serialization(request,grant));let missing=match missing{Err(refused)=>refused,Ok(_)=>panic!("unregistered original source route admitted")};assert_eq!(missing.error.kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner);assert_eq!(missing.progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(missing.request.source.get::<u32>().unwrap()as*const u32,Arc::as_ptr(&source));assert_eq!(missing.request.options.as_str().unwrap().as_ptr(),options_pointer);let request=missing.request;
  let(result,heap)=observe(||entry.begin_owned_serialization(request,grant));let refused=match result{Err(refused)=>refused,Ok(_)=>panic!("deliberately rejecting fixture admitted a job")};assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(refused.progress,Default::default());assert_eq!(refused.request.source.get::<u32>().unwrap()as*const u32,Arc::as_ptr(&source));assert_eq!(refused.request.options.as_str().unwrap().as_ptr(),options_pointer);assert_eq!(CALLS.load(std::sync::atomic::Ordering::Relaxed)-calls,row["factoryCalls"].as_u64().unwrap()as usize);
  let original=match refused.request.source.try_into_typed::<u32>(){Ok(read)=>read,Err(_)=>panic!("original type lost")};let witness=match original.try_return_to_registry_witness(){Ok(witness)=>witness,Err(_)=>panic!("exact source return refused")};assert!(!witness.terminal_is_empty());
  let mut bank=EffectBank::new(BankWallet::new(close_grant(),1<<30));
  let returned=bank.run_outcome(close_grant(),1<<10,|effect|registry.try_admit_one_returned::<u32,_>(effect,|owner,_|RetainedWorkOutcome{result:Ok(owner.take().map(|owner|(owner,RetainedCloneProgress{copied_items:1,..Default::default()}))),work:1})).result.ok().unwrap();bank.take_admitted();assert!(returned.0.is_some());drop(returned);assert!(witness.terminal_is_empty());drop(witness);
  let mut options_close=semio_framework_value::retirement::controlled::ControlledRetirement::new(refused.request.options).unwrap_or_else(|_|panic!("native options controlled authority"));
  for _ in 0..1000{if options_close.terminal_is_empty(){break;}let turn=bank.run_outcome(close_grant(),1<<10,|effect|{let mut turn=None;with_original_invocation(None,effect,&mut |effect|turn=Some(match options_close.step(65536,effect){RetainedWorkPoll::Complete(done)=>done,RetainedWorkPoll::Pending{..}=>panic!("options close stayed pending")})).unwrap();turn.expect("options close ran")});bank.take_admitted();turn.result.unwrap();}assert!(options_close.terminal_is_empty());
 }
}

