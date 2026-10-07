//! 🧪️ Typed optional fields preserve three-way authored delta intent.
use super::*;
#[derive(Debug,PartialEq,semio_framework_dsl_record_derive::DslRecord)]
struct OptionalDelta { label:Option<Option<String>> }

#[test]
fn typed_optional_delta_matches_neutral_json_and_document_intent(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🪆️binding/🪆️optional/🧫️fixtures/🔣️.json")).unwrap();
 for invalid in corpus["invalidDocuments"].as_array().unwrap(){assert!(parse_exact(invalid.as_str().unwrap(),&OptionalDelta::__dsl_spec(),&ParseOptions::default()).is_err(),"invalid terminal input: {invalid}");}
 for sample in corpus["samples"].as_array().unwrap(){
  let expected=match sample["state"].as_str().unwrap(){"omitted"=>OptionalDelta{label:None},"clear"=>OptionalDelta{label:Some(None)},"replace"=>OptionalDelta{label:Some(Some(sample["json"]["label"].as_str().unwrap().into()))},_=>unreachable!()};
  let spec=OptionalDelta::__dsl_spec();let record=expected.__dsl_to_record();let text=print(&record,&spec,JoinMode::Inline);let parsed=parse(&text,&spec,&ParseOptions::default()).unwrap();
  assert_eq!(OptionalDelta::__dsl_from_record(&parsed).unwrap(),expected);
  let authored=parse_exact(sample["document"].as_str().unwrap(),&spec,&ParseOptions::default()).unwrap();assert_eq!(OptionalDelta::__dsl_from_record(&authored).unwrap(),expected);
  let mut accept=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut accept);let original=expected.label.clone().unwrap_or(None);let projected=original.to_value_controlled(&mut output).unwrap();
  let mut accept=|_|true;let mut input=NativeDecodeControl::new(usize::MAX,&mut accept);assert_eq!(<Option<String> as DslField>::from_value_controlled(&projected,&mut input).unwrap(),original);
  let mut accept=|_|true;let mut output=NativeEncodeControl::new(0,&mut accept);let document=print(&record,&spec,JoinMode::Document);assert_eq!(measure_print_borrowed(&expected,&OptionalDelta::RECORD,usize::MAX,&mut output).unwrap(),document.len());
  let mut stop=|_|false;let mut canceled=NativeEncodeControl::new(usize::MAX,&mut stop);assert!(original.to_value_controlled(&mut canceled).is_err());
  let borrowed=<Option<String> as BorrowedDslField>::SHAPE;let BorrowedShape::Block(make)=borrowed else{panic!("optional scalar must retain an explicit typed clear block")};let BorrowedShape::Record(make)=make()else{panic!("optional scalar must retain its authored value field")};let descriptor=make();assert_eq!(descriptor.fields.len(),1);assert_eq!(descriptor.fields[0].key,"value");assert!(descriptor.fields[0].optional);assert!(matches!(descriptor.fields[0].shape,BorrowedShape::Text));
  assert_eq!(<Option<String> as DslField>::from_value(&<Option<String> as DslField>::to_value(&expected.label.clone().unwrap_or(None))).unwrap(),expected.label.unwrap_or(None));
 }
}
