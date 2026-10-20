use semio_framework_test_fixtures::fixture_text;
use super::*;
#[test]
fn native_record_receive_borrows_original_named_members_without_payload_copy(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧩️record/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let entries=law["canonical"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_owned(),match pair[1].as_str(){Some(text)=>DslValue::String(text.to_owned()),None=>DslValue::Number(Number::Int(pair[1].as_i64().unwrap()))})).collect();let value=DslValue::Object(entries);let fields=[NativeRecordField{name:"id",ordinal:0},NativeRecordField{name:"name",ordinal:1},NativeRecordField{name:"value",ordinal:2}];assert_eq!(value.field_count().unwrap(),3);
 for index in 0..3{let((name,number),heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||{let field=fields.iter().find(|field|field.matches(value.field_key(index).unwrap())).unwrap();if field.ordinal==2{(field.name,Some(value.number_field(index).unwrap().as_i64().unwrap()))}else{let text=value.text_field(index).unwrap();let DslValue::Object(entries)=&value else{unreachable!()};let DslValue::String(actual)=&entries[index].1 else{unreachable!()};assert_eq!(std::ptr::from_ref(text),std::ptr::from_ref(actual));assert_eq!(text,law["expected"][field.name].as_str().unwrap());(field.name,None)}});assert_eq!(heap,(0,0));if let Some(number)=number{assert_eq!(number,law["expected"][name].as_i64().unwrap());}}
 assert!(value.field_key(3).is_err());assert!(value.text_field(0).is_err());
}
