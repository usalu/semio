use super::*;
use semio_plugin::{ArtifactInferenceExecutionRequest, ArtifactInferenceServiceRegistry, WireArtifactInferenceBudget, WireArtifactInferenceCacheMode};

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
    let snapshot_pack = <GltfSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&GltfSnapshot::default());
    let budgets = WireArtifactInferenceBudget { allocation_bytes: 1_000_000, work_units: 1, recursion_depth: 1 };
    let dependencies = vec![("snapshot".into(), snapshot_pack.clone())];
    let request = ArtifactInferenceExecutionRequest { operation:17,generation:9,cancelled:false, retained:semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128},
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
    let envelope: serde_json::Value = serde_json::from_slice(execution.execution.as_ref().unwrap().canonical_payload.as_ref().unwrap()).unwrap();
    assert_eq!(envelope["id"], "s.stdio.gltf.inference.overall-size.v1");
    assert!(envelope["cacheKey"].as_str().unwrap().contains(":p"));
    assert_eq!(envelope["dependencyHashes"].as_array().unwrap().len(), 1);
}

#[test]
fn every_leaf_publishes_the_pack_bound_contract() {
    for service in gltf_inference_services() {
        let contract = service.metadata().payload.expect("every glTF leaf publishes a payload contract");
        assert_eq!(contract, GLTF_LEAF_INFERENCE_CONTRACT);
        assert_eq!(contract.artifact_binding.unwrap().encoding, semio_framework::INFERENCE_ARTIFACT_PACK_BYTES);
        serde_json::from_str::<serde_json::Value>(contract.input_schema).unwrap();
        serde_json::from_str::<serde_json::Value>(contract.output_schema).unwrap();
    }
}
