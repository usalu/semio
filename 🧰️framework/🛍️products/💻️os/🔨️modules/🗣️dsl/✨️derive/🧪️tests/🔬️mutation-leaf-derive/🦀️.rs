
use super::*;
use sha2::Digest;

#[test]
fn parses_strict_mutation_leaf_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../✨️mutation-leaf-derive/🧫️fixtures/🔣️.json")).unwrap();
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
fn hashes_workspace_provenance_with_sha2_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🛂️mutation-source-authority/🧫️fixtures/🔣️.json")).unwrap();
    let (workspace, compiler_cwd, source) = mutation_source_authority_tests::materialize("valid", &fixture);
    let authority = mutation_source_authority(&source, &compiler_cwd).unwrap();
    let first = mutation_leaf_workspace_token(&authority).unwrap();
    let second = mutation_leaf_workspace_token(&authority).unwrap();
    let workspace = mutation_leaf_portable_path(&fs::canonicalize(workspace).unwrap()).unwrap();
    let taxonomy = mutation_authority_relative(&authority.workspace_root, &authority.taxonomy_path).unwrap();
    let mut input = b"semio.mutation-source-provenance/v1\0".to_vec();
    for value in [workspace.as_bytes(), taxonomy.as_bytes()] {
        input.extend_from_slice(&(value.len() as u64).to_be_bytes());
        input.extend_from_slice(value);
    }
    let expected: [u8; 32] = sha2::Sha256::digest(&input).into();
    assert_eq!(first, second);
    assert_eq!(first, expected);
    let (_, other_cwd, other_source) = mutation_source_authority_tests::materialize("valid", &fixture);
    let other = mutation_leaf_workspace_token(&mutation_source_authority(&other_source, &other_cwd).unwrap()).unwrap();
    assert_ne!(first, other);
}
