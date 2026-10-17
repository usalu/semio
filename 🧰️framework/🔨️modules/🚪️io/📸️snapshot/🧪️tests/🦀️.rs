//! 🧪️ Neutral consumer vectors compare native representations with an independent serde_json oracle.
use super::{ArtifactPack,ArtifactNativeSnapshot,NativeSnapshotInput,NativeSnapshotEncoding};
use semio_framework_value::{DslValue,ToValue,FromValue};
#[test]
fn neutral_snapshot_contracts_encode_firstparty_values_and_interaction_state_without_os(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for reference in fixture["valueCases"].as_array().unwrap(){
  let value=DslValue::from(reference);let encoded=value.encode_pack();
  assert_eq!(serde_json::Value::from(DslValue::decode_pack(&encoded).unwrap()),*reference);
  assert_ne!(encoded,serde_json::to_vec(reference).unwrap());
  let mut progress=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(1_000_000,&mut progress);
  let payload=value.encode_native_snapshot(NativeSnapshotEncoding::Text,&mut encode).unwrap();
  let semio_framework_io_schema::IoPayload::Text(text)=payload else{panic!("authored text face")};
  assert_eq!(serde_json::from_str::<serde_json::Value>(&text).unwrap(),*reference);
  let mut accepted=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(1_000_000,&mut accepted);
  let restored=DslValue::decode_native_snapshot(NativeSnapshotInput::Text(&text),&mut decode).unwrap();
  assert_eq!(serde_json::Value::from(restored),*reference);
 }
 for reference in fixture["interactionCases"].as_array().unwrap(){
  let state=semio_framework_replication_interaction::InteractionState::from_value(DslValue::from(reference)).unwrap();
  assert_eq!(serde_json::to_value(&state).unwrap(),*reference);
  let bytes=state.encode_pack();let decoded=semio_framework_replication_interaction::InteractionState::decode_pack(&bytes).unwrap();
  assert_eq!(serde_json::Value::from(decoded.to_value()),*reference);
 }
}
