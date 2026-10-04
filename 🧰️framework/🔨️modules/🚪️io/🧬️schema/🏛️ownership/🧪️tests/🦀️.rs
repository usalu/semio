//! 🏛️ Neutral I/O vocabulary preserves shared literal values and controlled refusal causes.
use crate::{ArtifactDialect,ArtifactRef};
use semio_framework_value::{DslValue,FromValue,NativeDecodeControl,NativeEncodeControl,ToValue};
fn reference(value:&serde_json::Value)->ArtifactRef{
    ArtifactRef{artifact_id:value["artifactId"].as_str().unwrap().into(),dialect:ArtifactDialect{artifact_kind:value["dialect"]["artifactKind"].as_str().unwrap().into(),standard:value["dialect"]["standard"].as_str().unwrap().into(),subset:value["dialect"]["subset"].as_str().unwrap().into()}}
}
fn json(value:&DslValue)->serde_json::Value{
    match value{DslValue::String(text)=>serde_json::Value::String(text.clone()),DslValue::Object(entries)=>serde_json::Value::Object(entries.iter().map(|(key,value)|(key.clone(),json(value))).collect()),_=>panic!("reference wire consists of strings and objects")}
}
#[test]
fn neutral_vocabulary_preserves_shared_literal_identity_and_independent_json(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for value in fixture["references"].as_array().unwrap(){
        let expected=reference(value);let mut allow=|_|true;let mut control=NativeEncodeControl::new(16384,&mut allow);
        let native=expected.to_value_controlled(&mut control).unwrap();assert_eq!(json(&native),*value);
        assert_eq!(serde_json::to_value(&expected.dialect).unwrap(),value["dialect"]);
        let mut allow=|_|true;let mut control=NativeDecodeControl::new(16384,&mut allow);
        assert_eq!(ArtifactRef::from_value_controlled(&native,&mut control).unwrap(),expected);
    }
    eprintln!("[DEBUG] Three shared literal references preserve independent JSON and controlled native round trips");
}
#[test]
fn neutral_vocabulary_preserves_shared_controlled_refusal_categories(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🔗️reference/🧫️fixtures/🚦️refusals/🔣️.json")).unwrap();
    let identity=&fixture["identity"];let expected=ArtifactRef{artifact_id:identity["artifactId"].as_str().unwrap().into(),dialect:ArtifactDialect{artifact_kind:identity["artifactKind"].as_str().unwrap().into(),standard:identity["standard"].as_str().unwrap().into(),subset:identity["subset"].as_str().unwrap().into()}};
    let mut allow=|_|true;let mut control=NativeEncodeControl::new(16384,&mut allow);let native=expected.to_value_controlled(&mut control).unwrap();
    let mut deny=|_|false;let mut control=NativeDecodeControl::new(16384,&mut deny);let canceled=ArtifactRef::from_value_controlled(&native,&mut control).unwrap_err();
    let mut allow=|_|true;let mut control=NativeDecodeControl::new(0,&mut allow);let ownership=ArtifactRef::from_value_controlled(&native,&mut control).unwrap_err();
    let mut allow=|_|true;let mut control=NativeDecodeControl::new(16384,&mut allow);let invalid=ArtifactRef::from_value_controlled(&DslValue::Bool(false),&mut control).unwrap_err();
    for(error,case)in[canceled,ownership,invalid].into_iter().zip(fixture["cases"].as_array().unwrap()){assert_eq!(error.kind.as_str(),case["kind"].as_str().unwrap());}
    let mut allow=|_|true;let mut control=NativeDecodeControl::new(16384,&mut allow);assert_eq!(ArtifactRef::from_value_controlled(&native,&mut control).unwrap(),expected);
    eprintln!("[DEBUG] Shared initial cancellation, ownership admission, invalid shape and unrestricted literal identity retain their causes");
}
