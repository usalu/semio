use semio_framework_test_fixtures::fixture_text;
use super::*;
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate")]struct OriginalEmptyRecord{}
#[test]
fn original_empty_record_receiving_matches_serde_and_retains_every_cancelled_prefix(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🔗️sealed-shared-receiver/🧪️tests/🫙️empty/🧫️fixtures/🔣️.json")).unwrap();
 for case in law["cases"].as_array().unwrap(){
  let accepted=case["accepted"].as_bool().unwrap();assert_eq!(serde_json::from_value::<std::collections::BTreeMap<String,serde::de::IgnoredAny>>(case["input"].clone()).is_ok_and(|fields|fields.is_empty()),accepted);
  for cut in std::iter::once(None).chain(law["cancelAt"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()))){
   let mut bank=bank();let original=driver::admit(&mut bank,DslValue::from(&case["input"]));let(mut receiver,h)=crate::observe_retirement_allocations(<OriginalEmptyRecord as NativeReceive<DslValue>>::receiver);assert_eq!(h,(0,0));let mut refused=false;
   for _ in 0..cut.unwrap_or(10000){if receiver.output_ready(){break}if receiver.demands(original.borrow(),BODY,1<<20).is_err(){refused=true;break}if driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,Some(1)).is_err(){refused=true;break}}
   if cut.is_none(){assert_eq!(receiver.output_ready(),accepted);assert_eq!(refused,!accepted);}
   if receiver.output_ready(){let output=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");driver::retire(output,&mut bank);}
   driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());assert_eq!(crate::observe_retirement_allocations(||drop(receiver)).1,(0,0));driver::close_source(original,&mut bank);
  }
 }
}
