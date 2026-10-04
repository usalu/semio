//! 🔗️ Neutral reference fields and typed schema refusals remain separate from text terminals.
use super::*;
#[test]
fn artifact_reference_schema_refusals_retain_categories_and_literal_identity(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️refusals/🔣️.json")).unwrap();
    let mut deny=|_|false;let mut control=NativeDecodeControl::new(usize::MAX,&mut deny);
    let canceled=<ArtifactRef as DslField>::shape_controlled(&mut control).unwrap_err();
    let mut allow=|_|true;let mut control=NativeDecodeControl::new(0,&mut allow);let ownership=spec_controlled(&mut control).unwrap_err();
    let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Bool(false));let invalid=fields(&record).unwrap_err();
    for(error,case)in[canceled,ownership,invalid].into_iter().zip(fixture["cases"].as_array().unwrap()){assert_eq!(error.kind.as_str(),case["kind"].as_str().unwrap());}
    let identity=&fixture["identity"];let reference=ArtifactRef{artifact_id:identity["artifactId"].as_str().unwrap().into(),dialect:ArtifactDialect{artifact_kind:identity["artifactKind"].as_str().unwrap().into(),standard:identity["standard"].as_str().unwrap().into(),subset:identity["subset"].as_str().unwrap().into()}};
    let value=<ArtifactRef as DslField>::to_value(&reference);let mut allow=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(<ArtifactRef as DslField>::from_value_controlled(&value,&mut control).unwrap(),reference);
    let FieldValue::Record(record)=value else{panic!("literal reference record")};let strings=fields(&record).unwrap();let encoded=serde_json::to_string(&strings).unwrap();assert_eq!(serde_json::from_str::<Vec<String>>(&encoded).unwrap(),strings);
}
