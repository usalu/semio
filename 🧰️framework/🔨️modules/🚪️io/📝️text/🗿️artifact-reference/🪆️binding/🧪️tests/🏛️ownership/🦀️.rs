//! 🔗️ Canonical IO ownership preserves literal records and cumulative typed controls.
use crate::{ArtifactRef,ArtifactDialect};
use semio_framework_dsl_record::{DslField,FieldValue,RecordValue,Shape};
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,ValueRefusalKind};
#[path="../../../../../../🗣️dsl/🧬️schema/🪆️binding/🏛️ownership/🧪️tests/🦀️.rs"]
mod oracle;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪆️binding/🔣️.json")).unwrap()}
fn reference(value:&serde_json::Value)->ArtifactRef{ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}}}
#[test]
fn artifact_reference_canonical_record_owner_preserves_fields_and_independent_json(){
 let fixture=fixture();for case in fixture["cases"].as_array().unwrap(){let reference=reference(&case["value"]);let Shape::Record(producer)=<ArtifactRef as DslField>::shape() else{panic!("literal record")};let spec=(producer.ordinary)();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(usize::MAX,&mut allow);let decoded=(producer.decoding)(&mut decoding).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(usize::MAX,&mut allow);let encoded=(producer.encoding)(&mut encoding).unwrap();assert_eq!(oracle::schema(&decoded),oracle::schema(&spec));assert_eq!(oracle::schema(&encoded),oracle::schema(&spec));assert!(decoding.owned_bytes()>0);assert!(encoding.owned_bytes()>0);let declared:Vec<_>=spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"shape":"Text","optional":field.optional})).collect();assert_eq!(serde_json::json!(declared),fixture["fields"]["ArtifactRef"]);
 let mut allow=|_|true;let mut output=NativeEncodeControl::new(usize::MAX,&mut allow);let record=<ArtifactRef as DslField>::to_record_controlled(&reference,&mut output).unwrap();assert_eq!(oracle::fields(&record),case["expectedFields"]);let json=serde_json::to_string(&oracle::fields(&record)).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&json).unwrap(),case["expectedFields"]);
 let mut allow=|_|true;let mut input=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(<ArtifactRef as DslField>::from_record_controlled(&record,&mut input).unwrap(),reference);let expected=reference.artifact_id.len()+reference.dialect.artifact_kind.len()+reference.dialect.standard.len()+reference.dialect.subset.len();assert_eq!(input.owned_bytes(),expected);println!("[DEBUG] canonical ArtifactRef binding case={} decodeRequested={}",case["id"],input.owned_bytes());}}
#[test]
fn artifact_reference_canonical_record_owner_refuses_without_uncontrolled_fallback(){
 let fixture=fixture();let reference=reference(&fixture["cases"][1]["value"]);let value=<ArtifactRef as DslField>::to_value(&reference);let Shape::Record(producer)=<ArtifactRef as DslField>::shape() else{unreachable!()};let mut deny=|_|false;let mut canceled=NativeDecodeControl::new(usize::MAX,&mut deny);assert_eq!(<ArtifactRef as DslField>::shape_controlled(&mut canceled).unwrap_err().kind.as_str(),fixture["controls"]["initialCancel"].as_str().unwrap());
 let mut allow=|_|true;let mut schema=NativeDecodeControl::new(0,&mut allow);assert_eq!((producer.decoding)(&mut schema).unwrap_err().kind.as_str(),fixture["controls"]["metadataZero"].as_str().unwrap());let mut allow=|_|true;let mut output=NativeEncodeControl::new(0,&mut allow);assert_eq!(<ArtifactRef as DslField>::to_value_controlled(&reference,&mut output).unwrap_err().kind.as_str(),fixture["controls"]["projection"].as_str().unwrap());let mut allow=|_|true;let mut input=NativeDecodeControl::new(0,&mut allow);assert_eq!(<ArtifactRef as DslField>::from_value_controlled(&value,&mut input).unwrap_err().kind.as_str(),fixture["controls"]["construction"].as_str().unwrap());
 let mut invalid=RecordValue::default();invalid.fields.insert(0,FieldValue::Bool(false));let mut allow=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(<ArtifactRef as DslField>::from_record_controlled(&invalid,&mut control).unwrap_err().kind,ValueRefusalKind::InvalidValue);println!("[DEBUG] canonical ArtifactRef schema/projection/construction/refusal controls retained");}

#[test]
fn artifact_reference_borrowed_schema_matches_the_literal_binding_corpus(){
 let expected=fixture();let spec=<ArtifactRef as semio_framework_dsl_record::BorrowedDslRecord>::RECORD;
 let actual:Vec<_>=spec.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"shape":match field.shape{semio_framework_dsl_record::BorrowedShape::Text=>"Text",_=>panic!("reference text field")},"optional":field.optional})).collect();
 assert_eq!(serde_json::json!(actual),expected["fields"]["ArtifactRef"]);
 assert!(matches!(<ArtifactRef as semio_framework_dsl_record::BorrowedDslField>::SHAPE,semio_framework_dsl_record::BorrowedShape::Record(_)));
}
