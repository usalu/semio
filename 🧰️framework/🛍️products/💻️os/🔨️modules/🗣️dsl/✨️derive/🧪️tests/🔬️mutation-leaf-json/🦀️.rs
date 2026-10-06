
use super::*;
fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️mutation-leaf-json/🔣️.json")).unwrap()
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
#[test]
fn a_withdraw_only_descriptor_answers_no_input_schema() {
    let fixture = fixture();
    let authority = authority(fixture["authorityOwner"].as_str().unwrap());
    let raw = |name: &str| fixture["cases"].as_array().unwrap().iter().find(|vector| vector["name"] == name).unwrap()["raw"].as_str().unwrap().as_bytes().to_vec();
    let editable = parse_mutation_leaf_descriptor(&raw("valid-full"), &authority).unwrap();
    let marked = parse_mutation_leaf_descriptor(&raw("valid-editable-true"), &authority).unwrap();
    let withdrawn = parse_mutation_leaf_descriptor(&raw("valid-withdraw-only"), &authority).unwrap();
    assert!(editable.editable && marked.editable && !withdrawn.editable);
    let contract: syn::Path = syn::parse_str("::protocol").unwrap();
    let plain = MutationLeafAttrs { contract: contract.clone(), payload: None, input_schema: None };
    let name: syn::Ident = syn::parse_str("SetSnapshot").unwrap();
    assert!(mutation_leaf_withdraw_only(&name, &editable, &plain).unwrap().is_none());
    let emitted = mutation_leaf_withdraw_only(&name, &withdrawn, &plain).unwrap().unwrap().to_string();
    assert!(emitted.contains("input_schema") && emitted.contains("None"), "{emitted}");
    assert!(emitted.contains("with_input_value") && emitted.contains("InvalidValue") && emitted.contains("SetSnapshot") && emitted.contains("is withdraw-only"), "a withdraw-only leaf refuses every edited payload: {emitted}");
    assert!(!emitted.contains("from_input_value"), "a withdraw-only leaf still decodes from its payload: {emitted}");
    let wrapped = MutationLeafAttrs { contract: contract.clone(), payload: Some(syn::parse_str("Apply").unwrap()), input_schema: None };
    let instanced = MutationLeafAttrs { contract, payload: None, input_schema: Some(syn::parse_str("schema_at_path").unwrap()) };
    assert!(mutation_leaf_withdraw_only(&name, &withdrawn, &wrapped).is_err() && mutation_leaf_withdraw_only(&name, &withdrawn, &instanced).is_err());
}
