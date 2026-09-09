use super::*;

#[semio_framework_async_macros::async_test]
async fn canonical_leaf_roundtrip_is_id_bound() {
    let value = GltfInferenceLeafEnvelope {
        id: "s.stdio.gltf.inference.overall-size.v1".into(),
        algorithm_version: 1,
        policy_hash: "policy".into(),
        dependency_hashes: vec!["geometry:1".into()],
        cache_key: "s.stdio.gltf.inference.overall-size.v1:geometry-v2".into(),
        validity: "valid".into(),
        quality: "exact".into(),
        diagnostic_ids: Vec::new(),
        provenance: vec!["scene-world".into()],
        value: dsl::DslValue::float(1.0),
    };
    let encoded = encode_gltf_inference_leaf_text(&value).unwrap();
    assert_eq!(decode_gltf_inference_leaf_text(&encoded).unwrap(), value);
}
