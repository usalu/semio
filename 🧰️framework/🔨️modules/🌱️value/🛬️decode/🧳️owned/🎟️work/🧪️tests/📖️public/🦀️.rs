//! 📖️ Public native owning laws exercise canonical production ports from the same package target.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,RetainedCloneClose,RetainedCloneCursor,retained_clone_grant_admits};
pub use semio_framework_value_native_owned::retirement::RetireOwned;
pub use semio_framework_value_derive::{RetireOwned,NativeReceive,ToValue};
pub fn observe_retirement_allocations<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
#[path="../../../🌳️value/🧱️bulk/🎟️work/🧪️tests/🦀️.rs"]mod tree;
#[path="../../../🧩️record/🔎️schema/🎟️work/🧪️tests/🦀️.rs"]mod schema;
#[path="../../../🧩️record/📦️storage/🎟️work/🧪️tests/🦀️.rs"]mod storage;
#[path="../../../🌳️value/🔢️array/🎟️work/🧪️tests/🦀️.rs"]mod array;
#[path="../🦀️.rs"]mod scalar;
#[path="../../📤️take/🧪️tests/🦀️.rs"]mod take;
#[path="../../../🌳️value/🔀️variant/🎟️work/🧪️tests/🦀️.rs"]mod variant;
#[path="../../../🗂️map/🎟️work/🧪️tests/🦀️.rs"]mod map_work;
#[path="../../../🧺️set/🎟️work/🧪️tests/🦀️.rs"]mod set_work;
#[path="../../../🧩️record/🧪️tests/🎮️receiving/🦀️.rs"]mod record_receiving;

pub(crate) fn receive_native_work(control:&mut NativeDecodeControl<'_>,loan:RetainedCloneGrant,work_loan:source_fixture::WorkLoan<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{source_fixture::receive(control,loan,work_loan,operation)}

#[path="../../../📦️storage/🧪️tests/📖️initializers/🦀️.rs"]mod initializer_laws;

pub(crate) use semio_framework_value_testing_admission as source_fixture;

#[test]fn actual_native_batch_retains_real_move_work_when_the_next_owned_release_refuses(){
 let f:serde_json::Value=serde_json::from_str(include_str!("../../../📋️list/🧪️tests/🧫️fixtures/📐️.json")).unwrap();let law=&f["controlBatch"]["compositionLaw"];let mut bank=source_fixture::Wallet::from_profile(&f["stock"]);let mut storage=semio_framework_value_native_storage::NativeOwnedStorage::<u64>::empty();let g=bank.loan(1,4096);let mut marker=None;let(turn,h)=observe_retirement_allocations(||storage.reserve(g,&mut marker));assert_eq!(marker,Some(h.0));source_fixture::post_step(&mut bank,turn,g,h,|e|e.retained_progress()).unwrap();let pointer=storage.address();let mut input=Some(law["value"].as_u64().unwrap());let probe_grant=bank.loan(1,4096);assert_eq!(probe_grant.maximum_items,1);let mut callback=|_|true;let mut native=NativeDecodeControl::new(probe_grant.maximum_capacity_bytes,&mut callback);let mut zero=bank.work_loan(0);native.with_work_recipient(&mut zero,|native|{assert!(!native.prepare_work(1)?);Ok::<_,ValueError>(())}).unwrap();assert_eq!(native.actual_work(),0);drop(zero);let mut one=bank.work_loan(bank.work_remaining());native.with_work_recipient(&mut one,|native|{assert!(native.prepare_work(1)?);let bytes=input.as_ref().unwrap().to_le_bytes();assert!(std::str::from_utf8(&bytes).is_ok());native.commit_work(1)?;Ok::<_,ValueError>(())}).unwrap();drop(one);assert_eq!(native.actual_work(),law["independentProbeWork"].as_u64().unwrap());bank.post(Default::default(),native.actual_work(),probe_grant,(0,0));
 let denied=RetainedCloneGrant{maximum_items:0,..bank.loan(1,4096)};let empty=owned::NativeReceivingBatch::new(denied);let(turn,h)=observe_retirement_allocations(||storage.put_from(&mut input,denied));let turn=RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work};let result=empty.record(turn,true,false).err().unwrap();assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),Default::default());assert_eq!(h,(0,0));assert_eq!(input,Some(7));
 let g=bank.loan(law["items"].as_u64().unwrap()as usize,4096);let batch=owned::NativeReceivingBatch::new(g);let(turn,h)=observe_retirement_allocations(||storage.put_from(&mut input,batch.grant()));assert_eq!(h,(0,0));let batch=batch.record(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work},true,false).ok().unwrap();let(turn,h)=observe_retirement_allocations(||storage.release(batch.grant()));assert_eq!(h,(0,0));let result=batch.record(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work},false,false).err().unwrap();assert_eq!(result.work,law["workBeforeFailure"].as_u64().unwrap());let fault=result.result.unwrap_err();let p=fault.retained_progress();assert_eq!(p.copied_items,law["copiedItems"].as_u64().unwrap()as usize);assert_eq!(p.copied_bytes,law["copyBytes"].as_u64().unwrap()as usize);assert_eq!(fault.primary.as_ref().unwrap().kind,ValueRefusalKind::InvariantViolated);bank.post(p,result.work,g,(0,0));assert_eq!(storage.address(),pointer);assert_eq!(storage.get(),Some(&7));source_fixture::close_owned(fault,&mut bank,4096,10000);
 let mut output=None;let g=bank.loan(1,4096);let(turn,h)=observe_retirement_allocations(||storage.take_into(&mut output,g));source_fixture::post_step(&mut bank,turn,g,h,|e|e.retained_progress()).unwrap();assert_eq!(serde_json::to_value(output).unwrap(),law["value"]);source_fixture::close_owned(output.take().unwrap(),&mut bank,4096,10000);let g=bank.loan(1,4096);let(turn,h)=observe_retirement_allocations(||storage.release(g));source_fixture::post_step(&mut bank,turn,g,h,|e|e.retained_progress()).unwrap();assert!(storage.terminal_is_empty());
}
