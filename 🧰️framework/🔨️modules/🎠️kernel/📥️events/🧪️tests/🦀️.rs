//! 📥️ Every inbound Event is received from a genuine original source and retired through its finite caller.
use semio_framework::kernel::Event;
use semio_framework_value::DslValue;
use semio_framework_value_native_owned::owned::NativeReceive;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[path="../../../🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;
const BODY:usize=8192;
#[test]fn original_event_receive_matches_serde_and_closes_every_partial_owner(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for(row,valid)in law["cases"].as_array().unwrap().iter().map(|row|(row,true)).chain(law["invalid"].as_array().unwrap().iter().map(|row|(row,false))){for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()))){
let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let initial=bank.wallet().remaining();
  let(input,seed)=observe(||DslValue::from(row["input"].clone()));let source=driver::admit(&mut bank,input);
  let published=driver::lifecycle(<Event as NativeReceive<DslValue>>::receiver(),source,&mut bank,BODY,cut,|value:&Event|{
   let oracle:Event=serde_json::from_value(row["input"].clone()).unwrap();assert_eq!(*value,oracle);assert_eq!(serde_json::to_value(value).unwrap(),row["input"]);
  });
  match(cut,valid){
   (None,true)=>assert_eq!(published,Ok(true),"{}",row["variant"]),
   (None,false)=>{assert!(published.is_err(),"{}",row["variant"]);assert!(serde_json::from_value::<Event>(row["input"].clone()).is_err());},
   (Some(_),true)=>assert!(published.is_ok(),"{}",row["variant"]),
   (Some(_),false)=>{},
  }
  let left=bank.wallet().remaining();assert_eq!(initial.maximum_capacity_bytes-left.maximum_capacity_bytes+seed.0,initial.maximum_release_bytes-left.maximum_release_bytes+seed.1);
 }}
}
