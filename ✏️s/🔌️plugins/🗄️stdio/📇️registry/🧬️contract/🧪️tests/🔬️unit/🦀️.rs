use super::*;

#[test]
fn runtime_capability_standard_vectors_preserve_each_declared_owner() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪜️definition-hierarchy/🔣️.json")).unwrap();
    for row in corpus["runtimeCases"].as_array().unwrap() {
        let standards = row["standards"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>();
        assert_eq!(runtime_standard(standards.iter().copied(), row["identity"].as_str().unwrap(), row["category"].as_str().unwrap()).is_ok(), row["accepted"].as_bool().unwrap());
    }
}

#[test]
fn artifact_assembly_layout_and_identity_match_neutral_budget() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📦️assembly/🔣️.json")).expect("assembly fixture");
    let definition = ArtifactDefinition::stdio(fixture["artifact"].as_str().expect("artifact")).expect("definition");
    let assembly = definition_only_assembly("binary", definition).expect("definition-only assembly");
    assert_eq!(serde_json::to_value(assembly.definition().identity().as_str()).expect("identity oracle"), fixture["identity"]);
    let bytes = size_of::<ArtifactAssembly>();
    assert!(bytes as u64 <= fixture["maximumInlineBytes"].as_u64().expect("assembly budget"));
}
