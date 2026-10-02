use semio_framework_schema_state::{StateClass,parse_state_class_kebab};
use semio_framework_value::{DslValue,FromValue,ToValue,NativeDecodeControl,NativeEncodeControl};
#[derive(serde::Serialize,serde::Deserialize)]
enum ReferenceState { Artifact,Config,Presence,Transient }
#[test]
fn state_vectors_match_independent_serde_and_exact_kebab_authority(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🧱️neutrality/🔣️.json")).unwrap();
 assert_eq!(fixture["states"].as_array().unwrap().len(),4);
 for row in fixture["states"].as_array().unwrap(){let reference:ReferenceState=serde_json::from_value(row["wire"].clone()).unwrap();let state=parse_state_class_kebab(row["kebab"].as_str().unwrap()).unwrap();assert_eq!(serde_json::Value::from(&state.to_value()),serde_json::to_value(reference).unwrap());assert_eq!(StateClass::from_value(state.to_value()).unwrap(),state);let input=DslValue::String(row["wire"].as_str().unwrap().into());let mut accepted=|_|true;let mut decode=NativeDecodeControl::new(0,&mut accepted);assert_eq!(StateClass::from_value_controlled(&input,&mut decode).unwrap(),state);assert_eq!(decode.owned_bytes(),0);let mut accepted=|_|true;let mut encode=NativeEncodeControl::new(128,&mut accepted);assert_eq!(state.to_value_controlled(&mut encode).unwrap(),input);assert_eq!(encode.owned_bytes(),row["wire"].as_str().unwrap().len());}
 for row in fixture["retiredStates"].as_array().unwrap(){assert!(parse_state_class_kebab(row.as_str().unwrap()).is_none());assert!(serde_json::from_value::<ReferenceState>(row.clone()).is_err());assert!(StateClass::from_value(DslValue::String(row.as_str().unwrap().into())).is_err());}
}
#[test]
fn state_controlled_construction_refuses_cancellation_and_output_ceiling(){
 let mut refused=|_|false;let mut decode=NativeDecodeControl::new(0,&mut refused);assert!(StateClass::from_value_controlled(&DslValue::String("Artifact".into()),&mut decode).is_err());assert_eq!(decode.owned_bytes(),0);
 let mut accepted=|_|true;let mut encode=NativeEncodeControl::new(0,&mut accepted);assert!(StateClass::Artifact.to_value_controlled(&mut encode).is_err());assert_eq!(encode.owned_bytes(),0);
 let mut refused=|_|false;let mut encode=NativeEncodeControl::new(128,&mut refused);assert!(StateClass::Artifact.to_value_controlled(&mut encode).is_err());assert_eq!(encode.owned_bytes(),0);
}
