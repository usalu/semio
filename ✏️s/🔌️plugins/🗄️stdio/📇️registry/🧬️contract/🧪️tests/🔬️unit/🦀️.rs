use super::*;
use base64::Engine;

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
    println!("[DEBUG] Artifact assembly inline bytes={bytes}");
    assert!(bytes as u64 <= fixture["maximumInlineBytes"].as_u64().expect("assembly budget"));
}
