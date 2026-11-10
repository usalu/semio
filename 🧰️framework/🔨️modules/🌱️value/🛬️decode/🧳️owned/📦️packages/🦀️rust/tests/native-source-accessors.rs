//! ⏱️ Public source accessor work agrees with the neutral corpus and independent serde values.
pub use semio_framework_value_native_owned::*;
pub use semio_framework_value_native_owned::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[path="../../../../../🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
#[test]fn source_accessor_work_matches_neutral_cases_and_serde_values(){
 use crate::native_decoding::OriginalEffectReply;use crate::owned::tree::{NativeScalarQuery,NativeSourceScalar,NativeValueSource};
 let f:serde_json::Value=serde_json::from_str(include_str!("../../../🌳️value/📖️source/🧪️fixtures/⏱️access/🔣️.json")).unwrap();let mut bank=driver::bank();
 for row in f["cases"].as_array().unwrap(){
  let operation=row["operation"].as_str().unwrap();let index=row["index"].as_u64().unwrap_or(0) as usize;let expected_work=row["work"].as_u64().unwrap();
  let uint=|x:&serde_json::Value|DslValue::Number(Number::UInt(x.as_u64().unwrap()));
  let value=match operation{"Boolean"=>DslValue::Bool(row["value"].as_bool().unwrap()),"Number"=>uint(&row["value"]),"SequenceChild"=>DslValue::Array(row["value"].as_array().unwrap().iter().map(uint).collect()),"RecordChild"=>DslValue::Object(row["value"].as_object().unwrap().iter().map(|(k,v)|(k.clone(),uint(v))).collect()),_=>panic!("unknown fixture operation")};
  let query=match operation{"Boolean"=>Some(NativeScalarQuery::Boolean),"Number"=>Some(NativeScalarQuery::Number),_=>None};
  if let Some(query)=query{
   let grant=bank.loan(1,4096);
   let(turn,_,heap)=driver::funded(&mut bank,grant,expected_work.max(1),|effect|{let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|value.query_scalar(query,effect,reply).map(|_|()));let work=effect.performed_work();RetainedWorkOutcome{result:ran.and_then(|()|slot.take().expect("query replied")),work}});
   assert_eq!(turn.work,u128::from(expected_work));bank.post(RetainedCloneProgress::default(),grant,heap);
   match(operation,turn.result.unwrap()){("Boolean",NativeSourceScalar::Boolean(read))=>assert_eq!(read,row["expected"].as_bool().unwrap()),("Number",NativeSourceScalar::Number(read))=>assert_eq!(read.as_f64(),row["expected"].as_f64().unwrap()),(_,other)=>panic!("unexpected scalar {other:?}")}
  }else{
   assert_eq!(expected_work,0,"children are borrowed projections: their traversal is charged by the receiving action");
   let selected=match operation{"SequenceChild"=>value.sequence_child(index).unwrap(),_=>value.record_child(index).unwrap()};
   assert_eq!(selected.number().unwrap().as_f64(),row["expected"].as_f64().unwrap());
  }
 }
}
