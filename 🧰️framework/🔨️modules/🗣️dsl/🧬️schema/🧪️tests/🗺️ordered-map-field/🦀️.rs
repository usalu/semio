//! 🧪️ A retained ordered dictionary field prints, binds and projects exactly like the `BTreeMap` oracle.
use semio_framework_dsl_record::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{retained_clone::ordered_map::RetainedOrderedMap,NativeDecodeControl,NativeEncodeControl};
use std::collections::BTreeMap;
#[derive(Debug,PartialEq,semio_framework_dsl_record_derive::DslRecord)]
struct Retained { weights:RetainedOrderedMap<String,f64> }
#[derive(Debug,PartialEq,semio_framework_dsl_record_derive::DslRecord)]
struct Oracle { weights:BTreeMap<String,f64> }

fn entries(sample:&serde_json::Value)->Vec<(String,f64)>{
 if let Some(count)=sample["generate"].as_u64(){return (0..count).rev().map(|index|(format!("k{index:03}"),index as f64)).collect();}
 sample["entries"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_string(),pair[1].as_f64().unwrap())).collect()
}

#[test]
fn retained_ordered_map_field_matches_the_btree_map_oracle(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🪆️binding/🗺️ordered-map/🔣️.json")).unwrap();
 for sample in corpus["samples"].as_array().unwrap(){
  let name=sample["name"].as_str().unwrap();let pairs=entries(sample);
  let retained=Retained{weights:pairs.iter().cloned().collect()};let oracle=Oracle{weights:pairs.iter().cloned().collect()};
  assert_eq!(retained.weights.len(),oracle.weights.len(),"{name}: length");
  assert!(retained.weights.iter().map(|(key,value)|(key.clone(),*value)).eq(oracle.weights.iter().map(|(key,value)|(key.clone(),*value))),"{name}: key order");
  let (retained_spec,oracle_spec)=(Retained::__dsl_spec(),Oracle::__dsl_spec());
  for mode in [JoinMode::Inline,JoinMode::Document]{
   let text=print(&retained.__dsl_to_record(),&retained_spec,mode);
   assert_eq!(text,print(&oracle.__dsl_to_record(),&oracle_spec,mode),"{name}: printed text");
   let parsed=parse(&text,&retained_spec,&ParseOptions::default()).unwrap();
   assert_eq!(Retained::__dsl_from_record(&parsed).unwrap(),retained,"{name}: text round trip");
  }
  let (projected,expected)=(DslField::to_value(&retained.weights),DslField::to_value(&oracle.weights));
  assert_eq!(format!("{projected:?}"),format!("{expected:?}"),"{name}: neutral value");
  let mut accept=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut accept);
  let controlled=DslField::to_value_controlled(&retained.weights,&mut output).unwrap();
  assert_eq!(format!("{controlled:?}"),format!("{expected:?}"),"{name}: controlled value");
  let mut accept=|_|true;let mut input=NativeDecodeControl::new(usize::MAX,&mut accept);
  assert_eq!(<RetainedOrderedMap<String,f64> as DslField>::from_value_controlled(&controlled,&mut input).unwrap(),retained.weights,"{name}: controlled decode");
  assert_eq!(<RetainedOrderedMap<String,f64> as DslField>::from_value(&projected).unwrap(),retained.weights,"{name}: decode");
  assert_eq!(format!("{:?}",<RetainedOrderedMap<String,f64> as DslField>::shape()),format!("{:?}",<BTreeMap<String,f64> as DslField>::shape()),"{name}: shape");
  let mut stop=|_|false;let mut canceled=NativeEncodeControl::new(usize::MAX,&mut stop);
  assert!(!pairs.is_empty()==DslField::to_value_controlled(&retained.weights,&mut canceled).is_err(),"{name}: cancellation");
 }
 assert!(matches!(<RetainedOrderedMap<String,f64> as BorrowedDslField>::SHAPE,BorrowedShape::Map(_)));
}
