use super::*;

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
        value: dsl::DslValue::float(1.5),
    };
    let encoded = encode_gltf_inference_leaf_binary(&value).unwrap();
    assert_eq!(decode_gltf_inference_leaf_binary(&encoded).unwrap(), value);

    // 🔢️ The envelope's payload is CANONICAL JSON (`canonical_number`, ECMA-262 `Number::toString`
    // as RFC 8785 mandates), so an INTEGRAL float is not a fixed point: `1.0` is written `1` and
    // `pack::json`'s lexer reads an integer literal back as an integer carrier. That folding is
    // the canonical form's own contract, and this asserts it rather than leaving it implicit --
    // which is why the round-trip law above is stated on a non-integral float.
    let integral = GltfInferenceLeafEnvelope { value: dsl::DslValue::float(1.0), ..value.clone() };
    let folded = GltfInferenceLeafEnvelope { value: dsl::DslValue::uint(1), ..value.clone() };
    assert_eq!(decode_gltf_inference_leaf_binary(&encode_gltf_inference_leaf_binary(&integral).unwrap()).unwrap(), folded);
}
