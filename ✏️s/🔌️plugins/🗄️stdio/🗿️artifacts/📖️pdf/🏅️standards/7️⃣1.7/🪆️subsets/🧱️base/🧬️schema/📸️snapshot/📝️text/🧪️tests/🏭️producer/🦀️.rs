//! 🏭️ Independent scalar schema identity and both controlled allocation frontiers.
use super::*;
fn rows(spec:&dsl::RecordSpec)->serde_json::Value{serde_json::Value::Array(spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"shape":match field.shape{dsl::Shape::Text=>"Text",dsl::Shape::UInt=>"UInt",dsl::Shape::Bytes64=>"Bytes64",dsl::Shape::Bool=>"Bool",dsl::Shape::Value=>"Value",_=>panic!("scalar metadata shape")},"optional":field.optional})).collect())}
#[test]
fn sqlite_snapshot_scalar_schema_producer_matches_literal_fields_and_both_owned_frontiers(){
 let data:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏭️producer/🔣️.json")).unwrap();let maximum=data["maximumBytes"].as_u64().unwrap()as usize;let tiny=data["tinyMaximumBytes"].as_u64().unwrap()as usize;let count=data["fields"].as_array().unwrap().len();let at=data["cancelAt"].as_u64().unwrap()as usize;
 let expected=&data["fields"];assert_eq!(rows(&spec()),*expected);
 let exact=count*std::mem::size_of::<dsl::FieldSpec>()+expected.as_array().unwrap().iter().map(|field|field["key"].as_str().unwrap().len()).sum::<usize>();assert!(exact<=maximum);
 let mut accept=|_|true;let mut input=dsl::NativeDecodeControl::new(exact,&mut accept);input.begin_stage(5).unwrap();let decoded=spec_producer().decode(&mut input).unwrap();assert_eq!(rows(&decoded),*expected);assert_eq!(input.owned_bytes(),exact);input.step().unwrap();
 let mut accept=|_|true;let mut output=dsl::NativeEncodeControl::new(exact,&mut accept);output.begin_stage(5).unwrap();let encoded=spec_producer().encode(&mut output).unwrap();assert_eq!(rows(&encoded),*expected);assert_eq!(output.owned_bytes(),exact);output.step().unwrap();
 let mut accept=|_|true;let mut input=dsl::NativeDecodeControl::new(tiny,&mut accept);assert!(spec_producer().decode(&mut input).is_err());assert_eq!(input.owned_bytes(),0);
 let mut accept=|_|true;let mut output=dsl::NativeEncodeControl::new(tiny,&mut accept);assert!(spec_producer().encode(&mut output).is_err());assert_eq!(output.owned_bytes(),0);
 let mut accept=|_|true;assert!(spec_producer().decode(&mut dsl::NativeDecodeControl::new(exact-1,&mut accept)).is_err());let mut accept=|_|true;assert!(spec_producer().encode(&mut dsl::NativeEncodeControl::new(exact-1,&mut accept)).is_err());
 let mut cancel=|_|false;let mut input=dsl::NativeDecodeControl::new(maximum,&mut cancel);assert!(spec_producer().decode(&mut input).is_err());assert_eq!(input.owned_bytes(),0);
 let mut cancel=|_|false;let mut output=dsl::NativeEncodeControl::new(maximum,&mut cancel);assert!(spec_producer().encode(&mut output).is_err());assert_eq!(output.owned_bytes(),0);
 let mut reached=false;let mut cancel=|event:semio_framework_value::native_decoding::NativeDecodeProgress|if event.total==count&&event.completed==at{reached=true;false}else{true};assert!(spec_producer().decode(&mut dsl::NativeDecodeControl::new(maximum,&mut cancel)).is_err());assert!(reached);
 let mut reached=false;let mut cancel=|event:semio_framework_value::native_encoding::NativeEncodeProgress|if event.total==count&&event.completed==at{reached=true;false}else{true};assert!(spec_producer().encode(&mut dsl::NativeEncodeControl::new(maximum,&mut cancel)).is_err());assert!(reached);
 println!("scalar-schema-producer fields={count} exact-owned-bytes={exact} directions=2 refusal-frontiers=8");
}
