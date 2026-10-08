
use super::*;

#[path = "../../../🧪️tests/🔬️unit/🔮️oracle/🦀️.rs"]
mod json_oracle;

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_module_for_sourcing_curation() {
    let manifest = bundle().into_manifest_cold().unwrap();
    assert_eq!(manifest.extension_id, EXTENSION_ID);
    assert_eq!(manifest.extends, "sourcing");
    assert_eq!(manifest.capabilities.len(), 0);
    assert_eq!(manifest.topic_contributions.len(), 1);
    let topic = &manifest.topic_contributions[0];
    assert_eq!(topic.topic, "sourcing.module");
    assert_eq!(topic.payload["appId"].as_str(), Some(HOST_APP_ID));
    assert_eq!(topic.payload["moduleId"].as_str(), Some("slabs"));
    let typology_json = topic.payload["typologyJson"].as_str().unwrap();
    let kinds_json = topic.payload["kindsJson"].as_str().unwrap();
    let typology = semio_framework_pack_json::from_json_str::<semio_s_artifact_sourcing_curation::schema::TypologyNode>(typology_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let kinds = semio_framework_pack_json::from_json_str::<Vec<semio_s_artifact_sourcing_curation::ObjectKind>>(kinds_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(typology, SlabsModule.typology());
    assert_eq!(kinds, SlabsModule.demo_kinds());
    json_oracle::verify(SlabsModule.module_id(), typology_json, kinds_json);
}
