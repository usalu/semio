
use super::*;
use semio_framework_plugin::{ArtifactInferenceExecutionRequest, ArtifactInferenceServiceRegistry, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

#[semio_framework_async_macros::async_test]
async fn all_canonical_leaf_services_are_independently_registered() {
    let services = gltf_inference_services();
    let ids = services.iter().map(|service| service.metadata().inference_schema).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(services.len(), 67);
    assert_eq!(ids.len(), 67);
    assert!(ids.contains("s.stdio.gltf.inference.overall-size.v1"));
    let mut registry = ArtifactInferenceServiceRegistry::new();
    for service in services {
        registry.register(service).unwrap();
    }
}

#[semio_framework_async_macros::async_test]
async fn one_leaf_service_returns_its_id_bound_generic_envelope() {
    let snapshot_pack = <GltfSnapshot as store::ArtifactPack>::encode_pack(&GltfSnapshot::default());
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 1 };
    let dependencies = vec![("snapshot".into(), snapshot_pack.clone())];
    let request = ArtifactInferenceExecutionRequest {
        policy: b"gltf-test",
        budgets: &budgets,
        cancellation_id: "gltf-leaf",
        previous_state: None,
        requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
        canonical_payload: &snapshot_pack,
        dependencies: &dependencies,
    };
    let service = gltf_inference_services().into_iter().find(|service| service.metadata().inference_schema == "s.stdio.gltf.inference.overall-size.v1").unwrap();
    let execution = service.infer(&request).unwrap();
    let envelope = crate::io::inferences::binary::decode_gltf_inference_leaf_binary(&execution.canonical_payload).unwrap();
    assert_eq!(envelope.id, "s.stdio.gltf.inference.overall-size.v1");
    assert!(envelope.cache_key.contains(":p"));
    assert_eq!(envelope.dependency_hashes.len(), 1);
}
