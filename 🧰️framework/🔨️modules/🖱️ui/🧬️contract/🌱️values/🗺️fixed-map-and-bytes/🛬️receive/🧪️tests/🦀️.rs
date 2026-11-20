//! 🧰️ Real paged collection construction matches independent wire decoding and conserves every physical owner.
use semio_ui_contract_values::{UiFixedBytes,UiFixedList,UiFixedMap,UiValue};
use semio_framework_value::DslValue;
use semio_framework_value_native_owned::owned::NativeReceive;
use semio_framework_value_testing_admission::{EffectBank,Wallet};
use serde::Serialize;
#[path="../../../../../../🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;
const BODY:usize=4096;
fn input(row:&serde_json::Value)->DslValue{if row["entries"].is_array(){DslValue::Object(row["entries"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_owned(),DslValue::from(pair[1].clone()))).collect())}else{DslValue::from(serde_json::from_str::<serde_json::Value>(row["wire"].as_str().unwrap()).unwrap())}}
/// 🔁️ Receives one case or cancels it after `cut` turns, then closes the receiver, the output and the genuine Source.
fn run<T:NativeReceive<DslValue>+Serialize>(row:&serde_json::Value,cut:Option<usize>,law:&serde_json::Value){
 let mut bank=EffectBank::new(Wallet::from_profile(&law["stock"]));let source=driver::admit(&mut bank,input(row));
 let published=driver::lifecycle(T::receiver(),source,&mut bank,BODY,cut.map(|cut|cut as u64),|value:&T|assert_eq!(numeric(serde_json::to_value(value).unwrap()),numeric(row["expected"].clone())));
 let accepted=row["accepted"].as_bool().unwrap_or(true);
 match(cut,accepted){(None,true)=>assert_eq!(published,Ok(true),"{row}"),(None,false)=>assert!(published.is_err(),"{row}"),(Some(_),true)=>assert!(published.is_ok(),"{row}"),(Some(_),false)=>{}}
 assert!(bank.wallet().remaining_work()>0);
}
/// 🔢️ JSON numbers compare by value: an integral float and an integer denote the same UI number.
fn numeric(value:serde_json::Value)->serde_json::Value{match value{serde_json::Value::Number(n)=>serde_json::json!(n.as_f64().unwrap()),serde_json::Value::Array(items)=>serde_json::Value::Array(items.into_iter().map(numeric).collect()),serde_json::Value::Object(entries)=>serde_json::Value::Object(entries.into_iter().map(|(k,v)|(k,numeric(v))).collect()),other=>other}}
fn cuts(law:&serde_json::Value)->impl Iterator<Item=Option<usize>>+'_{std::iter::once(None).chain(law["cancelPrefixes"].as_array().unwrap().iter().map(|cut|Some(cut.as_u64().unwrap()as usize)))}
#[test]
fn native_bounded_ui_collections_preserve_original_owners_on_refusal_and_cancellation(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let wire=row["wire"].as_str().unwrap();let accepted=row["accepted"].as_bool().unwrap();
  let oracle=match row["kind"].as_str().unwrap(){"map"=>serde_json::from_str::<UiFixedMap<u64>>(wire).map(|v|serde_json::to_value(v).unwrap()),"bytes"=>serde_json::from_str::<UiFixedBytes>(wire).map(|v|serde_json::to_value(v).unwrap()),_=>serde_json::from_str::<UiFixedList<u64>>(wire).map(|v|serde_json::to_value(v).unwrap())};
  assert_eq!(oracle.is_ok(),accepted);if let Ok(value)=oracle{assert_eq!(value,row["expected"])}
  for cut in cuts(&law){match row["kind"].as_str().unwrap(){"map"=>run::<UiFixedMap<u64>>(row,cut,&law),"bytes"=>run::<UiFixedBytes>(row,cut,&law),_=>run::<UiFixedList<u64>>(row,cut,&law)}}
 }
}
#[test]
fn native_recursive_ui_values_keep_scalar_and_empty_owners_without_global_arena_birth(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../🛬️receive/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap().iter().filter(|row|row["needsArena"]==false){
  if row["expected"].is_number(){let oracle:f64=serde_json::from_str(row["wire"].as_str().unwrap()).unwrap();assert_eq!(oracle,row["expected"].as_f64().unwrap());}else{let oracle:serde_json::Value=serde_json::from_str(row["wire"].as_str().unwrap()).unwrap();assert_eq!(oracle,row["expected"]);}
  for cut in cuts(&law){run::<UiValue>(row,cut,&law);}
 }
}
