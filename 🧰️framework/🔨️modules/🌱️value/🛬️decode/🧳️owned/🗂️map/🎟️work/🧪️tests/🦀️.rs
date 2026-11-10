use crate::*;
use crate::driver;
use semio_framework_value_ordered::OrderedMap;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
const BODY:usize=4096;
#[test]
fn actual_native_map_null_denials_replacement_and_cancel_prefixes_keep_every_owner_and_one_stock(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();let rows=law["rows"].as_array().unwrap();
 let expected:std::collections::BTreeMap<String,u64>=rows.iter().map(|v|(v[0].as_str().unwrap().to_owned(),v[1].as_u64().unwrap())).collect();
 for cut in law["cuts"].as_array().unwrap(){
  let cut=cut.as_u64().unwrap();let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));
  let source=driver::admit(&mut bank,DslValue::Object(rows.iter().map(|r|(r[0].as_str().unwrap().to_owned(),DslValue::Number(Number::UInt(r[1].as_u64().unwrap())))).collect()));
  let mut receiver=<OrderedMap<u64>as NativeReceive<DslValue>>::receiver();
  let received=driver::receive_faulted(&mut receiver,source.borrow(),&mut bank,BODY,Some(cut),Some(1)).unwrap();assert!(received.is_none());
  if Some(cut)==law["cuts"].as_array().unwrap().last().and_then(|last|last.as_u64()){assert!(receiver.output_ready(),"full native map case failed to complete within its original declared cut");}
  if receiver.output_ready(){let output=driver::receive_cut(&mut receiver,source.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");let actual:std::collections::BTreeMap<String,u64>=output.iter().map(|(k,v)|(k.to_owned(),*v)).collect();assert_eq!(serde_json::to_value(actual).unwrap(),serde_json::to_value(&expected).unwrap());driver::retire(output,&mut bank);}
  driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(source,&mut bank);
 }
}
