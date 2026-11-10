//! ⚠️ Refusal receivers survive denial and cancellation at every turn and close to terminal-empty through genuine funded effects.
use semio_framework_value_native_owned::*;
use semio_framework_value_native_owned::owned::NativeReceiver;
use semio_framework_value_native_owned::retirement::RetireOwned;
use semio_framework_value_native_owned::retained_clone::RetainedCloneSource;
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;
const BODY:usize=65536;
fn cuts<I:RetireOwned+Sync+'static,T:RetireOwned,R:NativeReceiver<I,T>>(wire:impl Fn()->I,receiver:impl Fn()->R,bound:u64,inspect:impl Fn(&T)){
 for cut in (0..bound).map(Some).chain([None]){let mut bank=driver::bank();let source=driver::admit(&mut bank,wire());let published=driver::lifecycle(receiver(),source,&mut bank,BODY,cut,&inspect);if cut.is_none(){assert_eq!(published,Ok(true))}else{assert!(matches!(published,Ok(false)|Ok(true)),"{cut:?}: {published:?}")}}
}
#[test]fn refusal_kind_receiver_denies_cancels_and_closes_at_every_turn(){
 cuts(||DslValue::String("ownershipLimit".into()),<ValueRefusalKind as owned::NativeReceive<DslValue>>::receiver,48,|kind:&ValueRefusalKind|assert_eq!(*kind,ValueRefusalKind::OwnershipLimit));
}
#[test]fn refusal_receiver_rejects_a_foreign_source_after_its_first_turn(){
 let mut bank=driver::bank();let source=driver::admit(&mut bank,DslValue::String("ownershipLimit".into()));let other=driver::admit(&mut bank,DslValue::String("ownershipLimit".into()));
 let mut receiver=<ValueRefusalKind as owned::NativeReceive<DslValue>>::receiver();
 driver::none(driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(2))).unwrap();
 assert!(receiver.demands(other.borrow(),BODY,1<<20).is_err());
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);driver::close_source(other,&mut bank);
}
#[test]fn invalid_allocation_progress_retains_actual_input_and_refusal_until_funded_close(){
 let wire=||DslValue::from(serde_json::json!({"priorAdmittedBytes":1,"requestedBytes":null,"maximumBytes":3,"admittedBytes":0,"actualCapacityBytes":null}));
 let mut bank=driver::bank();let source=driver::admit(&mut bank,wire());let other=driver::admit(&mut bank,wire());
 let mut receiver=<NativeAllocationProgress as owned::NativeReceive<DslValue>>::receiver();
 let mut settled=false;
 for _ in 0..50000{if receiver.validation_refusal().is_some(){settled=true;break}let step=driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,Some(1));assert_eq!(step,Ok(None));}
 assert!(settled);
 assert_eq!(receiver.validation_refusal().expect("actual validation refusal retained").kind,ValueRefusalKind::InvalidValue);
 assert!(receiver.retained_input().is_some());assert!(receiver.demands(other.borrow(),BODY,1<<20).is_err());
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);driver::close_source(other,&mut bank);
}
#[test]fn recursive_retirement_failure_cancels_and_closes_at_every_turn(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🚪️io/⏱️control/🧾️failure/🧪️tests/🧫️fixtures/♻️recursive-retirement.json")).unwrap();let wire=fixture["cause"]["value"].clone();
 cuts(||DslValue::from(wire.clone()),<OriginalRetirementFailure as owned::NativeReceive<DslValue>>::receiver,256,|_:&OriginalRetirementFailure|{});
}
