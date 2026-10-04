use super::*;
use base64::Engine;

#[test]
fn runtime_capability_standard_vectors_preserve_each_declared_owner() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪜️definition-hierarchy/🔣️.json")).unwrap();
    for row in corpus["runtimeCases"].as_array().unwrap() {
        let standards = row["standards"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(runtime_standard(standards.iter().copied(), row["identity"].as_str().unwrap(), row["category"].as_str().unwrap()).is_ok(), row["accepted"].as_bool().unwrap());
    }
}

#[test]
fn standard_base64_matches_the_reference_implementation() {
    for bytes in [b"".as_slice(), b"f", b"fo", b"foo", b"foobar", &[0, 127, 128, 255]] {
        assert_eq!(base64_standard(bytes), base64::engine::general_purpose::STANDARD.encode(bytes));
    }
}

#[test]
fn artifact_assembly_layout_and_identity_match_neutral_budget() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📦️assembly/🔣️.json")).expect("assembly fixture");
    let definition = ArtifactDefinition::stdio(fixture["artifact"].as_str().expect("artifact")).expect("definition");
    let assembly = definition_only_assembly("binary", definition).expect("definition-only assembly");
    assert_eq!(serde_json::to_value(assembly.definition().identity().as_str()).expect("identity oracle"), fixture["identity"]);
    let bytes = size_of::<ArtifactAssembly>();
    assert!(bytes as u64 <= fixture["maximumInlineBytes"].as_u64().expect("assembly budget"));
}

#[test]
fn addressed_table_contract_requires_the_complete_cell_address_and_allows_empty_text() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📊️addressed-table/🔣️.json")).expect("addressed table fixture");
    let definition = addressed_table_window_kind();
    let action = definition.actions.iter().find(|action| action.id == fixture["action"]).expect("fixture action");
    let required = action.args.iter().filter(|argument| argument.required).map(|argument| argument.id.as_str()).collect::<Vec<_>>();
    let expected = fixture["required"].as_array().expect("required argument ids").iter().map(|value| value.as_str().expect("argument id")).collect::<Vec<_>>();
    assert_eq!(required, expected);

    let args = semio_framework_pack_json::from_json_str(&fixture["valid"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("typed fixture args");
    assert_eq!(window_kit_required_index_argument(Some(&args), "row").expect("required row"), 1);
    assert_eq!(window_kit_required_index_argument(Some(&args), "column").expect("required column"), 2);
    assert_eq!(window_kit_required_text_argument(Some(&args), "value").expect("required value"), "");
    assert!(window_kit_required_text_argument(Some(&args), "missing").is_err());
}
