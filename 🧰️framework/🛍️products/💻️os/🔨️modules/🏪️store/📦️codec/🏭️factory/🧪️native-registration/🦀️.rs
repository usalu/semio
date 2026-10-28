//! 🧪️ Native identity registration accepts the actual source vocabulary without semantic Clone or cold mutation capabilities.
use super::*;
struct NativeOnlySnapshot;
impl pack::snapshot::ArtifactPackMetadata for NativeOnlySnapshot{
 const EXTENSION:&'static str="energy";
 fn native_record_spec_producer()->Option<semio_framework_dsl_record::BorrowedRecordSpecProducer>{None}
 fn native_pack_schema_hash()->[u8;32]{[0;32]}
}
impl ArtifactStoreSnapshot for NativeOnlySnapshot{}
#[test]
fn native_registration_keeps_only_declared_capabilities(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let codec=native_artifact_codec::<NativeOnlySnapshot>(fixture["schema"].as_str().unwrap());
 let actual=serde_json::json!({"schema":codec.schema,"extension":codec.extension,"reference":codec.reference.is_some(),"packSchemaHash":codec.pack_schema_hash});assert_eq!(actual,fixture);assert_eq!(codec.native_identity.snapshot_type(),Some(std::any::TypeId::of::<NativeOnlySnapshot>()));assert!(codec.reference().is_err());assert!(NativeOnlySnapshot::reference_snapshot_codec().is_none());
}
