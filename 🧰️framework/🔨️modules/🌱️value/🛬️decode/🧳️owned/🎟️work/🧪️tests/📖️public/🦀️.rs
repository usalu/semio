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

#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]pub(crate) mod driver;
#[path="../../../📦️storage/🧪️tests/📖️initializers/🦀️.rs"]mod initializer_laws;

#[test]fn actual_native_batch_retains_real_move_work_when_the_next_owned_release_refuses(){
 use semio_framework_value_testing_admission::{EffectBank,Wallet};
 let f:serde_json::Value=serde_json::from_str(include_str!("../../../📋️list/🧪️tests/🧫️fixtures/📐️.json")).unwrap();let law=&f["controlBatch"]["compositionLaw"];let mut bank=EffectBank::new(Wallet::from_profile(&f["stock"]));let mut storage=semio_framework_value_native_storage::NativeOwnedStorage::<u64>::empty();
 let g=bank.loan(1,4096);let(turn,marker,h)=driver::funded(&mut bank,g,1,|effect|{let poll=storage.reserve_empty(effect);match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{..}=>panic!("synchronous reserve stayed pending")}});assert_eq!(marker,Some(h.0));bank.post(turn.result.unwrap().progress(),g,h);
 let pointer=storage.address();let mut input=Some(law["value"].as_u64().unwrap());let probe_grant=bank.loan(1,4096);assert_eq!(probe_grant.maximum_items,1);
 let(zero,_,_)=driver::funded(&mut bank,probe_grant,0,|effect|{assert!(!effect.performed(1).unwrap());RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()}});assert_eq!(zero.work,1);assert_eq!(bank.take_admitted(),Some(false));
 let fuel=bank.wallet().remaining_work();let(probe,_,_)=driver::funded(&mut bank,probe_grant,fuel,|effect|{assert!(effect.performed(1).unwrap());let bytes=input.as_ref().unwrap().to_le_bytes();assert!(std::str::from_utf8(&bytes).is_ok());RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()}});assert_eq!(probe.work,u128::from(law["independentProbeWork"].as_u64().unwrap()));
 let denied=RetainedCloneGrant{maximum_items:0,..bank.loan(1,4096)};let empty=owned::NativeReceivingBatch::new(denied);let(turn,_,h)=driver::funded(&mut bank,denied,1,|effect|storage.put_from(&mut input,effect));let turn=RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work};let result=empty.record(turn,true,false).err().unwrap();assert_eq!(result.work,0);assert_eq!(result.result.unwrap().progress(),Default::default());assert_eq!(h,(0,0));assert_eq!(input,Some(7));
 let g=bank.loan(law["items"].as_u64().unwrap()as usize,4096);let batch=owned::NativeReceivingBatch::new(g);let(turn,_,h)=driver::funded(&mut bank,batch.grant(),1,|effect|storage.put_from(&mut input,effect));assert_eq!(h,(0,0));let batch=batch.record(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work},true,false).ok().unwrap();
 let(turn,_,h)=driver::funded(&mut bank,batch.grant(),1,|effect|storage.release(effect));assert_eq!(h,(0,0));let result=batch.record(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work},false,false).err().unwrap();assert_eq!(result.work,u128::from(law["workBeforeFailure"].as_u64().unwrap()));let fault=result.result.unwrap_err();let p=fault.retained_progress();assert_eq!(p.copied_items,law["copiedItems"].as_u64().unwrap()as usize);assert_eq!(p.copied_bytes,law["copyBytes"].as_u64().unwrap()as usize);assert_eq!(fault.primary.as_ref().unwrap().kind,ValueRefusalKind::InvariantViolated);bank.post(p,g,(0,0));assert_eq!(storage.address(),pointer);assert_eq!(storage.get(),Some(&7));driver::retire(fault,&mut bank);
 let mut output=None;let g=bank.loan(1,4096);let(turn,_,h)=driver::funded(&mut bank,g,1,|effect|storage.take_into(&mut output,effect));bank.post(turn.result.unwrap().progress(),g,h);assert_eq!(serde_json::to_value(&output).unwrap(),law["value"]);driver::retire(output.take().unwrap(),&mut bank);let g=bank.loan(1,4096);let(turn,_,h)=driver::funded(&mut bank,g,1,|effect|storage.release(effect));bank.post(turn.result.unwrap().progress(),g,h);assert!(storage.terminal_is_empty());
}
