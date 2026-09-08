
use super::*;

#[semio_framework_async_macros::async_test]
async fn manifest_requires_exactly_one_fully_faceted_service_per_leaf() {
    let field_ids = GLTF_INFERENCE_FIELDS.iter().map(|field| field.id).collect::<std::collections::BTreeSet<_>>();
    let service_ids = GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS.iter().map(|descriptor| descriptor.id).collect::<std::collections::BTreeSet<_>>();
    let descriptors = gltf_artifact_inference_descriptors();
    let descriptor_ids = descriptors.iter().map(|descriptor| descriptor.id).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(field_ids.len(), 67);
    assert_eq!(service_ids.len(), 67);
    assert_eq!(descriptor_ids.len(), 67);
    assert_eq!(field_ids, service_ids);
    assert_eq!(service_ids, descriptor_ids);
    assert!(GLTF_INFERENCE_LEAF_SERVICE_DESCRIPTORS.iter().all(|descriptor| descriptor.algorithm_version == 1 && !descriptor.cache_key.is_empty()));
    assert!(!field_ids.iter().any(|id| id.contains("geometric-analysis") || id.ends_with(".bounds.v1")));
}
