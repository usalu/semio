use crate::{DslValue,NativeReceiver,owned::tree::NativeScalarReceiver};
use crate::driver::{self,Trace};
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
fn copied(bank:&EffectBank)->usize{bank.wallet().remaining().maximum_copy_bytes}
#[test]fn actual_native_owned_take_reports_its_own_work_and_no_work_for_denial_or_cached_output(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));
 let source=driver::admit(&mut bank,DslValue::Bool(law["value"].as_bool().unwrap()));let mut receiver=NativeScalarReceiver::<DslValue,bool>::new();let mut trace=Trace::default();
 while !receiver.output_ready(){driver::none(driver::receive_traced(&mut receiver,source.borrow(),&mut bank,BODY,Some(1),None,&mut trace)).unwrap()}
 assert_eq!(law["deniedWork"].as_u64().unwrap(),0);let before=copied(&bank);
 let value=driver::receive_traced(&mut receiver,source.borrow(),&mut bank,BODY,None,None,&mut trace).unwrap().expect("uncut receive publishes");
 assert_eq!(trace.take,u128::from(law["takeWork"].as_u64().unwrap()));assert_eq!(serde_json::to_value(value).unwrap(),law["value"]);assert_eq!(before-copied(&bank),law["copyBytes"].as_u64().unwrap()as usize);assert!(!receiver.output_ready());assert_eq!(law["cachedWork"].as_u64().unwrap(),0);
 driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
}
#[test]fn actual_native_sequence_owned_handoff_copies_no_retained_payload_and_keeps_denied_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));
 let values:Vec<bool>=serde_json::from_value(law["sequence"].clone()).unwrap();let source=driver::admit(&mut bank,DslValue::Array(values.iter().copied().map(DslValue::Bool).collect()));
 let mut receiver=<Vec<bool>as crate::NativeReceive<DslValue>>::receiver();let mut trace=Trace::default();
 while !receiver.output_ready(){driver::none(driver::receive_traced(&mut receiver,source.borrow(),&mut bank,BODY,Some(1),None,&mut trace)).unwrap()}
 let headers=2*std::mem::size_of::<Option<Vec<bool>>>();assert_eq!(receiver.take_demand().unwrap().copy_bytes,headers);let before=copied(&bank);
 let output=driver::receive_traced(&mut receiver,source.borrow(),&mut bank,BODY,None,None,&mut trace).unwrap().expect("uncut receive publishes");
 assert_eq!(trace.take,1);assert_eq!(before-copied(&bank),headers);assert_eq!(serde_json::to_value(&output).unwrap(),law["sequence"]);assert!(!receiver.output_ready());
 driver::retire(output,&mut bank);driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
}
