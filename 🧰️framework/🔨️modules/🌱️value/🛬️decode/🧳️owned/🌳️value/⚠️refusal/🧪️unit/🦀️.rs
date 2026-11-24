 use super::*;
 #[test]fn refusal_variants_match_wire_schema_and_serde(){
  let schema:serde_json::Value=serde_json::from_str(include_str!("../../../../../⚠️refusal/🔁️codec/🧬️schema/🔣️.json")).unwrap();
  let expected=schema["$defs"]["kind"]["enum"].as_array().unwrap();
  assert_eq!(expected.len(),8);
  for (index,wire) in expected.iter().enumerate(){
   let NativeRecordSchema::Leaf(leaf)=variants(index).unwrap() else{panic!("expected leaf")};
   assert_eq!(serde_json::to_value(ValueRefusalKind::from_variant_index(index).unwrap()).unwrap(),*wire);
   assert_eq!(leaf.name,wire.as_str().unwrap());
   assert_eq!(leaf.ordinal,index as u32);
  }
  assert!(variants(expected.len()).is_none());
  assert!(ValueRefusalKind::from_variant_index(expected.len()).is_err());
 }
 #[test]fn allocation_progress_validates_all_presence_combinations(){
  use crate::owned::NativeReceiveTryMap;
  let schema:serde_json::Value=serde_json::from_str(crate::NATIVE_ALLOCATION_SCHEMA).unwrap();let variants=schema["$defs"]["allocationProgress"]["oneOf"].as_array().unwrap();
  for mask in 0..8{let wire=serde_json::json!({"priorAdmittedBytes":if mask&1!=0{Some(1)}else{None},"requestedBytes":if mask&2!=0{Some(2)}else{None},"maximumBytes":if mask&4!=0{Some(3)}else{None},"admittedBytes":4,"actualCapacityBytes":null});let number=|key:&str|wire[key].as_u64().map(|n|n as usize);let fields=NativeAllocationProgressFields{prior_admitted_bytes:number("priorAdmittedBytes"),requested_bytes:number("requestedBytes"),maximum_bytes:number("maximumBytes"),admitted_bytes:number("admittedBytes").unwrap(),actual_capacity_bytes:number("actualCapacityBytes")};let accepted=<crate::NativeAllocationProgress as NativeReceiveTryMap<NativeAllocationProgressFields>>::validate_native(&fields).is_ok();let schema_accepts=variants.iter().any(|branch|["priorAdmittedBytes","requestedBytes","maximumBytes"].into_iter().all(|key|match branch["properties"][key]["type"].as_str().unwrap(){"null"=>wire[key].is_null(),"integer"=>wire[key].as_u64().is_some(),_=>false}));assert_eq!(accepted,schema_accepts);if accepted{let actual=<crate::NativeAllocationProgress as NativeReceiveTryMap<NativeAllocationProgressFields>>::from_valid_native(fields);assert!(actual.request_is_complete());assert_eq!(actual.admitted_bytes,4);}}
 }
