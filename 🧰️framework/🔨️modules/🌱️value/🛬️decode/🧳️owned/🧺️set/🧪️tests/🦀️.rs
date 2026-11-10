pub use semio_framework_value_native_owned::*;
use semio_framework_value_native_owned::owned::NativeReceive;
use semio_framework_value_ordered::OrderedSet;
#[path="../../../../🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;
const BODY:usize=32768;
#[test]fn original_set_receiving_matches_fixture_admits_every_insert_and_closes_every_partial_owner(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📐️.json")).unwrap();
 for case in law["cases"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap().iter().map(|v|Some(v.as_u64().unwrap())).chain([None]){
  let mut bank=driver::bank();let members=case["members"].as_array().unwrap();let source=driver::admit(&mut bank,DslValue::Array(members.iter().map(|m|DslValue::String(m.as_str().unwrap().into())).collect()));
  let published=driver::lifecycle(<OrderedSet as NativeReceive<DslValue>>::receiver(),source,&mut bank,BODY,cut,|set:&OrderedSet|{
   assert_eq!(serde_json::Value::Array(set.iter().map(|m|serde_json::json!(m)).collect()),case["expected"]);
   let oracle:std::collections::BTreeSet<&str>=members.iter().map(|m|m.as_str().unwrap()).collect();assert!(set.iter().map(String::as_str).eq(oracle.into_iter()));
  });
  if cut.is_none(){assert_eq!(published,Ok(true),"actual set preparation stalled")}else{assert!(published.is_ok(),"{published:?}")}
 }}
}
