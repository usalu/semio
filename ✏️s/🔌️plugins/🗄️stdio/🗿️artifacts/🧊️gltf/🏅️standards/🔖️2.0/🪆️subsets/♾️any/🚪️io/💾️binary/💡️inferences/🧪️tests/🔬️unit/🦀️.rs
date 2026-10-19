use crate::standards::v2_0::subsets::any::io::binary::inferences::*;

#[semio_framework_async_macros::async_test]
async fn deterministic_leaf_roundtrip() {
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
        value: semio_framework_value::DslValue::float(1.5),
    };
    let encoded = encode_gltf_inference_leaf_binary(&value).unwrap();
    assert_eq!(decode_gltf_inference_leaf_binary(&encoded).unwrap(), value);

    let integral = GltfInferenceLeafEnvelope { value: semio_framework_value::DslValue::float(1.0), ..value.clone() };
    let folded = GltfInferenceLeafEnvelope { value: semio_framework_value::DslValue::uint(1), ..value.clone() };
    assert_eq!(decode_gltf_inference_leaf_binary(&encode_gltf_inference_leaf_binary(&integral).unwrap()).unwrap(), folded);
}
