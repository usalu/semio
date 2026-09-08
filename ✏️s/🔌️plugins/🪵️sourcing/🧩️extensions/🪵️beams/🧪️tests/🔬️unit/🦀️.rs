
use super::*;

#[semio_framework_async_macros::async_test]
async fn bundle_contributes_module_for_sourcing_curation() {
    let manifest = bundle().manifest;
    assert_eq!(manifest.extension_id, EXTENSION_ID);
    assert_eq!(manifest.extends, "sourcing");
    assert_eq!(manifest.capabilities.len(), 0);
    assert_eq!(manifest.topic_contributions.len(), 1);
    let topic = &manifest.topic_contributions[0];
    assert_eq!(topic.topic, "sourcing.module");
    assert_eq!(topic.payload["appId"].as_str(), Some(HOST_APP_ID));
    assert_eq!(topic.payload["moduleId"].as_str(), Some("beams"));
    let typology_json = topic.payload["typologyJson"].as_str().unwrap();
    let kinds_json = topic.payload["kindsJson"].as_str().unwrap();
    assert!(semio_framework_os_kernel::json::from_json_str::<semio_s_artifact_sourcing_curation::schema::TypologyNode>(typology_json).is_ok());
    assert!(semio_framework_os_kernel::json::from_json_str::<Vec<semio_s_artifact_sourcing_curation::ObjectKind>>(kinds_json).is_ok());
}
