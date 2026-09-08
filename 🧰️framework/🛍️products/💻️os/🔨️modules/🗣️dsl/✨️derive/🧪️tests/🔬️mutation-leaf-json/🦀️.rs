
use super::*;
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🔣️mutation-leaf-json/🧫️fixtures/🔣️.json")).unwrap()
}
fn authority(owner: &str) -> MutationSourceAuthority {
    MutationSourceAuthority { workspace_root: PathBuf::new(), mutation_root: PathBuf::new(), owner: owner.to_string(), expected_semantic_kind: None, source_path: PathBuf::new(), descriptor_path: PathBuf::new(), taxonomy_path: PathBuf::new() }
}
#[test]
fn parses_mutation_leaf_json_fixture() {
    let fixture = fixture();
    let authority = authority(fixture["authorityOwner"].as_str().unwrap());
    for vector in fixture["cases"].as_array().unwrap() {
        let result = parse_mutation_leaf_descriptor(vector["raw"].as_str().unwrap().as_bytes(), &authority);
        assert_eq!(result.is_ok(), vector["parserAccepted"].as_bool().unwrap(), "{}: {result:?}", vector["name"]);
        if let Err(error) = result {
            assert!(error.contains(vector["diagnostic"].as_str().unwrap()), "{}: {error}", vector["name"]);
        }
    }
}
#[test]
fn emits_all_core_descriptor_fields() {
    let fixture = fixture();
    let authority = authority(fixture["authorityOwner"].as_str().unwrap());
    let descriptor = parse_mutation_leaf_descriptor(fixture["cases"][0]["raw"].as_str().unwrap().as_bytes(), &authority).unwrap();
    let contract: syn::Path = syn::parse_str("::protocol").unwrap();
    let emitted = emit_mutation_leaf_descriptor(&contract, &descriptor).to_string();
    for field in
        ["schema_version", "owner", "semantic_kind", "display_name", "emoji", "aggregate_variant", "payload_schema", "text_opcode", "binary_tag", "invertibility", "diff_participation", "outcome_classes", "composition", "required_language_surfaces"]
    {
        assert!(emitted.contains(field), "missing {field}: {emitted}");
    }
    assert!(emitted.contains("MutationLeafDescriptor") && emitted.contains("ExplicitMutation") && emitted.contains("JsonSchema"));
}
