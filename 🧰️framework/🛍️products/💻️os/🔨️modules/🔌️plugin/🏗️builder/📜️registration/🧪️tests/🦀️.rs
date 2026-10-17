//! 🧪️ Registration: 2 tests covering every declared quota axis preserves none and explicit zero; descriptor registration deduplicates exact owned rows and consumes all fields.
use super::*;

#[test]
fn every_declared_quota_axis_preserves_none_and_explicit_zero() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/📦️units/🧪️registration.json")).unwrap();
    for row in rows.as_array().unwrap(){
        let field=row["field"].as_str().unwrap();
        macro_rules! axis {($field:ident)=>{if field==stringify!($field){let mut owner=PluginDescriptorRegistration::default();let mut base=QuotaSchema::default();base.$field=Some(row["base"].as_u64().unwrap());owner.quota(base);let mut incoming=QuotaSchema::default();incoming.$field=row["incoming"].as_u64();owner.quota(incoming);assert_eq!(owner.into_parts().quotas.$field,Some(row["expected"].as_u64().unwrap()));}}}
        axis!(memory_bytes);axis!(fuel_per_turn);axis!(turn_deadline_ms);axis!(tables);axis!(mailbox_len);axis!(message_bytes);axis!(outstanding_requests);axis!(timers);axis!(storage_bytes);axis!(network_bytes_per_min);axis!(ui_nodes);axis!(patch_bytes);axis!(patch_hz);axis!(blob_resident_bytes);axis!(gpu_ms_per_frame);axis!(background_ms_per_min);axis!(log_bytes_per_min);
    }
}

#[test]
fn descriptor_registration_deduplicates_exact_owned_rows_and_consumes_all_fields() {
    let mut owner=PluginDescriptorRegistration::default();
    let event=ActivationEvent::OnCommand{id:"command".into()};
    owner.activation(event.clone());owner.activation(event.clone());
    let request=CapabilityRequest{id:semio_framework::kernel::CapabilityId("request".into()),scope:"plugin".into(),reason:"fixture".into(),optional:false};
    owner.requests(request.clone());owner.requests(request.clone());
    let extension=ExtensionPointDeclaration{id:"point".into(),publisher_scope:"fixture".into(),allowed_modes:vec![ExecutionMode::Isolated],capability_allowance:Vec::new(),quota_ceiling:QuotaSchema::default(),payload_schema:semio_framework::kernel::SchemaId("fixture".into()),activation:event.clone()};
    owner.extension_point(extension.clone());owner.extension_point(extension.clone());
    let asset=AssetDeclaration{name:"asset".into(),media_type:semio_framework::MediaType{class:semio_framework::MediaClass::Text,form:semio_framework::MediaForm::Document},size_bytes:7,sha256:"digest".into()};
    owner.asset(asset.clone());owner.asset(asset.clone());owner.execution(ExecutionMode::Linked);
    let parts=owner.into_parts();
    assert_eq!(parts.activation_events,vec![event]);assert_eq!(parts.capability_requests,vec![request]);assert_eq!(parts.extension_points,vec![extension]);assert_eq!(parts.assets,vec![asset]);assert_eq!(parts.execution,ExecutionMode::Linked);assert_eq!(parts.quotas,QuotaSchema::default());
}
