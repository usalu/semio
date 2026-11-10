use crate::*;
use crate::driver;
use crate::owned::NativeReceiver;
#[derive(crate::NativeReceive,crate::RetireOwned)]#[value(crate="crate")]struct InitializerRow{value:u64}
fn initializer<T:crate::owned::NativeReceive<crate::DslValue>>(bank:&mut semio_framework_value_testing_admission::EffectBank){
 let(mut receiver,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||T::receiver());assert_eq!(heap,(0,0));
 let before=bank.wallet().remaining();driver::close::<DslValue,T,_>(&mut receiver,bank,4096);let after=bank.wallet().remaining();assert_eq!((before.maximum_capacity_bytes,before.maximum_release_bytes),(after.maximum_capacity_bytes,after.maximum_release_bytes));assert!(receiver.terminal_is_empty());
}
#[test]fn actual_concrete_and_derived_receiver_initializers_allocate_no_backing(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../../📐️.json")).unwrap();let mut bank=semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::Wallet::from_profile(&row["stock"]));
 initializer::<u64>(&mut bank);initializer::<String>(&mut bank);initializer::<Option<u64>>(&mut bank);initializer::<Vec<u64>>(&mut bank);initializer::<[u64;2]>(&mut bank);initializer::<InitializerRow>(&mut bank);initializer::<(u64,String)>(&mut bank);assert_eq!(row["pureInitializers"].as_array().unwrap().len(),7);
}
