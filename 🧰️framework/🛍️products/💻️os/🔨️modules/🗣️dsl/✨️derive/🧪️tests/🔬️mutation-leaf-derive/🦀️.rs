
use super::*;
use sha2::Digest;

#[test]
fn parses_strict_mutation_leaf_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/✨️mutation-leaf-derive/🔣️.json")).unwrap();
    for case in fixture["attributes"].as_array().unwrap() {
        let input: DeriveInput = match syn::parse_str(&format!("{} #[derive(MutationLeaf)] struct Probe;", case["attribute"].as_str().unwrap())) {
            Ok(input) => input,
            Err(_) => {
                assert!(!case["accepted"].as_bool().unwrap(), "{}", case["name"]);
                continue;
            }
        };
        let result = parse_mutation_leaf_attrs(&input);
        assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
        if let Some(diagnostic) = case["diagnostic"].as_str() {
            assert!(result.unwrap_err().to_string().contains(diagnostic), "{}", case["name"]);
        }
    }
}

#[test]
fn hashes_the_authority_content_with_a_sha2_oracle_and_never_the_workspace_path() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛂️mutation-source-authority/🔣️.json")).unwrap();
    let (_, compiler_cwd, source) = mutation_source_authority_tests::materialize("valid", &fixture);
    let authority = mutation_source_authority(&source, &compiler_cwd).unwrap();
    let first = mutation_leaf_workspace_token(&authority).unwrap();
    let taxonomy = mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path).unwrap();
    let content: [u8; 32] = sha2::Sha256::digest(fs::read(&authority.taxonomy_path).unwrap()).into();
    let mut input = b"semio.mutation-source-provenance/v2\0".to_vec();
    for value in [taxonomy.as_bytes(), content.as_slice()] {
        input.extend_from_slice(&(value.len() as u64).to_be_bytes());
        input.extend_from_slice(value);
    }
    let expected: [u8; 32] = sha2::Sha256::digest(&input).into();
    assert_eq!(first, expected);
    let (_, other_cwd, other_source) = mutation_source_authority_tests::materialize("valid", &fixture);
    let other = mutation_source_authority(&other_source, &other_cwd).unwrap();
    assert_eq!(first, mutation_leaf_workspace_token(&other).unwrap());
    let edited = fs::read_to_string(&other.taxonomy_path).unwrap().replace("\"mutationDomainOwners\":{}", "\"mutationDomainOwners\":{},\"unrelated\":0");
    fs::write(&other.taxonomy_path, edited).unwrap();
    assert_ne!(first, mutation_leaf_workspace_token(&other).unwrap());
}

#[test]
fn resolves_the_payload_schema_beside_the_descriptor_and_refuses_escapes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛂️mutation-source-authority/🔣️.json")).unwrap();
    let (_, compiler_cwd, source) = mutation_source_authority_tests::materialize("valid", &fixture);
    let authority = mutation_source_authority(&source, &compiler_cwd).unwrap();
    let leaf = authority.descriptor_path.parent().unwrap().to_path_buf();
    fs::create_dir_all(leaf.join("🧬️schema")).unwrap();
    fs::write(leaf.join("🧬️schema/🔣️.json"), r#"{"type":"object","properties":{}}"#).unwrap();
    assert_eq!(mutation_leaf_payload_schema_path(&authority, "🧬️schema/🔣️.json").unwrap(), leaf.join("🧬️schema/🔣️.json"));
    for rejected in ["", "/🧬️schema/🔣️.json", "../🔣️.json", "🧬️schema/../🔣️.json", "🧬️schema//🔣️.json", "🧬️schema\\🔣️.json", "🧬️schema/absent.json", "🦀️.rs#InsertPage"] {
        assert!(mutation_leaf_payload_schema_path(&authority, rejected).is_err(), "{rejected:?}");
    }
}
