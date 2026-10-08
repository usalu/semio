//! 🏷️ Required single variants preserve the actual inline native owner.
use super::*;
#[derive(Debug,PartialEq,serde::Deserialize,semio_framework_dsl_record_derive::DslEnum)]
#[serde(tag="kind",rename_all="lowercase",deny_unknown_fields)]
enum InlineVariant { Rect { width:f64 }, Text { text:String } }
#[derive(Debug,PartialEq,serde::Deserialize,semio_framework_dsl_record_derive::DslRecord)]
#[serde(deny_unknown_fields)]
struct InlineStatement { #[dsl(statements)] statement:InlineVariant }

#[test]
fn required_inline_statement_roundtrips_the_native_owner_and_refuses_zero_or_two(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../✨️derive/🧫️fixtures/🏷️required-inline/🔣️.json")).unwrap();
 for row in corpus["valid"].as_array().unwrap(){
  let value=&row["statement"];let expected=InlineStatement{statement:match value["kind"].as_str().unwrap(){"rect"=>InlineVariant::Rect{width:value["width"].as_f64().unwrap()},"text"=>InlineVariant::Text{text:value["text"].as_str().unwrap().into()},_=>unreachable!()}};
  assert_eq!(serde_json::from_value::<InlineStatement>(row.clone()).unwrap(),expected);
  let spec=InlineStatement::__dsl_spec();let record=expected.__dsl_to_record();let document=print(&record,&spec,JoinMode::Document);
  assert_eq!(InlineStatement::__dsl_from_record(&parse_exact(&document,&spec,&ParseOptions::default()).unwrap()).unwrap(),expected);
  let mut accept=|_|true;let mut control=NativeEncodeControl::new(usize::MAX,&mut accept);let encoded=expected.__dsl_to_record_controlled(&mut control).unwrap();assert_eq!(encoded,record);
  let mut accept=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut accept);assert_eq!(InlineStatement::__dsl_from_record_controlled(&record,&mut control).unwrap(),expected);
  let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);assert_eq!(measure_print_borrowed(&expected,&InlineStatement::RECORD,usize::MAX,&mut control).unwrap(),document.len());
  assert!(matches!(expected.projection_view(&[0]).unwrap(),native_encoding::FieldProjectionView::Statements(1)));
  let mut stop=|_|false;let mut control=NativeEncodeControl::new(usize::MAX,&mut stop);assert!(expected.__dsl_to_record_controlled(&mut control).is_err());
  let mut stop=|_|false;let mut control=NativeDecodeControl::new(usize::MAX,&mut stop);assert!(InlineStatement::__dsl_from_record_controlled(&record,&mut control).is_err());
  let id=spec.fields[0].id;for statements in [Vec::new(),match record.get(id).unwrap(){FieldValue::Statements(items)=>vec![items[0].clone(),items[0].clone()],_=>panic!("tagged native field")}]{let mut invalid=RecordValue::default();invalid.fields.insert(id,FieldValue::Statements(statements));assert!(InlineStatement::__dsl_from_record(&invalid).is_err());let mut accept=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut accept);assert!(InlineStatement::__dsl_from_record_controlled(&invalid,&mut control).is_err());}
 }
 for row in corpus["invalid"].as_array().unwrap(){assert!(serde_json::from_value::<InlineStatement>(row.clone()).is_err());}
 eprintln!("[DEBUG] required inline variant original native owner roundtrips, borrowed measurement, controlled cancellation and exact one-value refusal agree with independent serde interpretation of ordinary neutral examples");
}
