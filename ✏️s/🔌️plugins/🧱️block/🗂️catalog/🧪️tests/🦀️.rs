//! 🗂️ Plugin catalog agrees with the portable declaration through controlled Value and independent Serde.
use semio_framework_value::ToValue;
#[test]
fn catalog_matches_the_closed_portable_declaration(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("portable catalog declaration");assert_eq!(fixture["schemaVersion"],1);
 let spec=semio_s_plugin_block_catalog::artifact_kind();assert_eq!(spec.id,semio_s_plugin_block_catalog::ARTIFACT_ID);
 let mut calls=0usize;let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{calls+=1;event.owned_bytes<=65536&&(event.total==0||event.completed<=event.total)};
 let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut progress);let owned=spec.to_value_controlled(&mut control).expect("controlled catalog projection");drop(control);assert!(calls>0);
 let own=serde_json::Value::from(&owned);let independent=serde_json::to_value(&spec).expect("independent catalog projection");assert_eq!(own,fixture["artifactKind"]);assert_eq!(independent,fixture["artifactKind"]);assert_eq!(own,independent);
 eprintln!("[DEBUG] Block catalog: twelve fields and four label cells agree with controlled Value and Serde");
}
#[test]
fn catalog_projection_refuses_cancellation_and_unadmitted_ownership(){
 let spec=semio_s_plugin_block_catalog::artifact_kind();let mut canceled=|_:semio_framework_value::native_encoding::NativeEncodeProgress|false;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut canceled);assert!(spec.to_value_controlled(&mut control).is_err());
 let mut admitted=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;let mut control=semio_framework_value::NativeEncodeControl::new(1,&mut admitted);assert!(spec.to_value_controlled(&mut control).is_err());eprintln!("[DEBUG] Block catalog refuses canceled work and a one-byte budget");
}
