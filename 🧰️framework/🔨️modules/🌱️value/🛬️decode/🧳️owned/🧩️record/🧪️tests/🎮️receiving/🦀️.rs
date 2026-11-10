use semio_framework_test_fixtures::fixture_text;
use crate::{Number,shared_issuance::SealedShared,owned::{NativeReceive,NativeReceiver},DslValue};
use crate::driver;
#[derive(crate::RetireOwned,crate::NativeReceive)]#[value(crate="crate",native_owned="crate")]struct FlatRow{id:SealedShared<String>,name:SealedShared<String>,value:i64}
#[test]fn native_flat_record_receive_composes_original_fields_and_every_prefix(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧩️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut finished=0;
 let input=||DslValue::Object(law["canonical"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_owned(),if let Some(text)=pair[1].as_str(){DslValue::String(text.to_owned())}else{DslValue::Number(Number::Int(pair[1].as_i64().unwrap()))})).collect());
 let cuts={let mut bank=driver::bank();let original=driver::admit(&mut bank,input());let mut receiver=<FlatRow as NativeReceive<DslValue>>::receiver();let mut trace=driver::Trace::default();let row=driver::receive_traced(&mut receiver,original.borrow(),&mut bank,4096,None,None,&mut trace).unwrap().expect("uncut receive publishes");driver::retire(row,&mut bank);driver::close(&mut receiver,&mut bank,4096);drop(receiver);driver::close_source(original,&mut bank);
  let total=trace.advance.len() as u64;let mut cuts=std::collections::BTreeSet::from([0u64,1,2]);let mut seen=std::collections::HashSet::new();for index in 1..trace.shapes.len(){if seen.insert((trace.shapes[index-1],trace.shapes[index])){cuts.extend([index as u64,index as u64+1])}}cuts.extend([total.saturating_sub(1),total,total+1]);cuts};
 for cut in cuts{
  let mut bank=driver::bank();let original=driver::admit(&mut bank,input());let foreign=driver::admit(&mut bank,input());
  let(mut receiver,heap)=crate::observe_retirement_allocations(<FlatRow as NativeReceive<DslValue>>::receiver);assert_eq!(heap,(0,0));
  if cut>0{driver::none(driver::receive_cut(&mut receiver,original.borrow(),&mut bank,4096,Some(1))).unwrap();if !receiver.output_ready(){assert!(receiver.demands(foreign.borrow(),4096,1<<20).is_err())}if cut>1{driver::none(driver::receive_cut(&mut receiver,original.borrow(),&mut bank,4096,Some(cut-1))).unwrap()}}
  if receiver.output_ready(){
   finished=cut;let row=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,4096,None).unwrap().expect("uncut receive publishes");
   assert_eq!(row.id.as_str(),law["expected"]["id"].as_str().unwrap());assert_eq!(row.name.as_str(),law["expected"]["name"].as_str().unwrap());assert_eq!(row.value,law["expected"]["value"].as_i64().unwrap());driver::retire(row,&mut bank);
  }
  driver::close(&mut receiver,&mut bank,4096);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(original,&mut bank);driver::close_source(foreign,&mut bank);
 }assert!(finished>0);
}
