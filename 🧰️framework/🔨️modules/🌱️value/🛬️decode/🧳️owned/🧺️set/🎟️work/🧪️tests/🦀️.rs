use crate::*;
use crate::driver;
use semio_framework_value_ordered::OrderedSet;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
#[test]fn actual_native_set_replaces_synchronous_controlled_decode_with_original_staged_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));
 let expected:std::collections::BTreeSet<String>=serde_json::from_value(law["members"].clone()).unwrap();assert_eq!(serde_json::to_value(&expected).unwrap(),law["expected"]);
 let source=driver::admit(&mut bank,DslValue::from(&law["members"]));let mut receiver=<OrderedSet as NativeReceive<DslValue>>::receiver();
 let output=driver::receive_faulted(&mut receiver,source.borrow(),&mut bank,BODY,None,Some(1)).unwrap().expect("uncut receive publishes");
 let actual:std::collections::BTreeSet<String>=output.iter().cloned().collect();assert_eq!(serde_json::to_value(actual).unwrap(),serde_json::to_value(expected).unwrap());
 driver::retire(output,&mut bank);driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
}
