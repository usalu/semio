use super::*;

#[test]
fn native_codec_identity_matches_structural_graph_and_independent_blake3() {
    let spec = <GltfSnapshot as store::ArtifactPack>::record_spec().expect("GLTF typed pack record");
    let graph = store::os_pack::PackSchemaGraph::of(&spec);
    let bytes = graph.canonical_bytes();
    let independent = *blake3::hash(&bytes).as_bytes();
    let codec = native_codec();
    assert_eq!(codec.pack_schema_hash, independent);
    let hash = blake3::hash(&bytes).to_hex().to_string();
    let source: serde_json::Value = serde_json::from_str(ARTIFACT_DEFINITION_SCHEMA).unwrap();
    let binding = source["codecs"].as_array().unwrap().iter().find_map(|item| item.get("native_factory")).unwrap();
    assert_eq!(binding["pack_schema_hash"].as_str().unwrap(), hash);
    definition().unwrap();
}

#[test]
fn owned_semantic_identity_policy_matches_independent_serde_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📜️definition/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut source: serde_json::Value = serde_json::from_str(ARTIFACT_DEFINITION_SCHEMA).unwrap();
        let identity = row["identity"].as_str().unwrap();
        source[row["category"].as_str().unwrap()][0]["id"] = identity.into();
        let expected = ![".no-mutation.", ".set-snapshot.", ".set-"].iter().any(|fragment| identity.contains(fragment));
        let admitted = validate_definition_schema(&serde_json::to_string(&source).unwrap()).is_ok();
        assert_eq!(admitted, expected);
        assert_eq!(admitted, row["accepted"].as_bool().unwrap());
    }
    definition().unwrap();
    formats().unwrap();
    assert_eq!(contribution().definition_constraint, Some(ARTIFACT_DEFINITION_CONSTRAINT));
}
