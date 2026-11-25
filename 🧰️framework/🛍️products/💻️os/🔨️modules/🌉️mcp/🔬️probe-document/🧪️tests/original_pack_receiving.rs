//! 🔁️ Original probe pack receiving retains every cancelled owner and releases exactly what it births.

use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_testing_allocator as _;
use semio_os_mcp_probe_document::probe_document::ProbeSnapshot;
use std::cell::Cell;

#[path = "../../../🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🪙️close/🦀️.rs"]
mod original_close;
use original_close::{close_decode, retire, ConstructionStock, WorkStock};

#[test]
fn original_probe_pack_receiving_retains_every_cancelled_owner_and_exact_allocations(){
 use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, ValueError};
 use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl, NativeDecodeRetirementRecipient, OriginalConstructionAuthority, OriginalWorkAuthority};
 let vectors:serde_json::Value=serde_json::from_str(fixture_text!("../../../../../../../🔨️modules/🚪️io/⏱️control/🛫️snapshot/🧪️tests/🧫️fixtures/📦️receiving.json")).unwrap();
 let policy=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:65536,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:64};
 let mut successes=0;
 for row in vectors["cases"].as_array().unwrap(){for cutoff in (0..=16).chain(std::iter::once(usize::MAX)){
  let input=row["text"].as_str().unwrap().as_bytes();let pointer=input.as_ptr();let allowance=Cell::new(cutoff);let events=Cell::new(0);
  let mut original=|_|{let next=events.get()+1;events.set(next);next<=allowance.get()};
  let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut original);native.install_retirement_recipient(&mut recipient).unwrap();
  let mut work=OriginalWorkAuthority::new(WorkStock(1<<30));let mut construction=OriginalConstructionAuthority::new(ConstructionStock::new(policy));
  let mut slot=None;let mut terminal_born=0;let mut terminal_released=0;
  let observed=work.with_loan(&mut(),|work|construction.with_loan(&mut(),|construction|native.with_work_recipient(work,|native|native.with_construction_recipient(construction,|native|{
   let mut owner=semio_os_kernel::NativeSnapshotDecodeOwner::new(native,policy);
   let(turn,born,released)=semio_framework_trace_memory_testing::observe_backing(||<ProbeSnapshot as semio_os_kernel::ArtifactPackReceiving>::receive_pack(input,&mut slot,&mut owner));let accepted=owner.progress();assert!(accepted.fits(policy));drop(owner);assert_eq!(input.as_ptr(),pointer);allowance.set(usize::MAX);
   if turn.result.is_ok(){successes+=1;let snapshot=slot.take().expect("accepted pack leaves its snapshot");assert_eq!(serde_json::to_value(&snapshot.0).unwrap(),row["expected"]);let(_,born,released)=semio_framework_trace_memory_testing::observe_backing(||retire(snapshot,policy));terminal_born+=born;terminal_released+=released;}
   for _ in 0..4096{if !native.has_retirement_owner(){break}let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||close_decode(native,policy).result.unwrap());assert!(step.progress().fits(policy));terminal_born+=born;terminal_released+=released;}assert!(!native.has_retirement_owner());
   Ok::<_,ValueError>((accepted,born,released))
  })))).unwrap().unwrap().unwrap();
  let(accepted,born,released)=observed;assert!(accepted.retained_capacity_bytes<=born);assert_eq!(released+terminal_released,born+terminal_born,"every retained and transient birth is released");
 }}
 assert!(successes>=vectors["cases"].as_array().unwrap().len());
 for axis in ["items","capacity","depth"]{let mut denied=policy;match axis{"items"=>denied.maximum_items=0,"capacity"=>denied.maximum_capacity_bytes=0,_=>denied.maximum_depth=0};let mut yes=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut native=NativeDecodeControl::new(1048576,&mut yes);native.install_retirement_recipient(&mut recipient).unwrap();let mut work=OriginalWorkAuthority::new(WorkStock(1<<30));let mut construction=OriginalConstructionAuthority::new(ConstructionStock::new(denied));let mut slot=None;
  work.with_loan(&mut(),|work|construction.with_loan(&mut(),|construction|native.with_work_recipient(work,|native|native.with_construction_recipient(construction,|native|{let mut owner=semio_os_kernel::NativeSnapshotDecodeOwner::new(native,denied);let(turn,born,released)=semio_framework_trace_memory_testing::observe_backing(||<ProbeSnapshot as semio_os_kernel::ArtifactPackReceiving>::receive_pack(b"null",&mut slot,&mut owner));assert!(turn.result.is_err());assert_eq!(born,released);assert_eq!(owner.progress(),RetainedCloneProgress::default());drop(owner);assert!(!native.has_retirement_owner());Ok::<_,ValueError>(())})))).unwrap().unwrap().unwrap();}
}
