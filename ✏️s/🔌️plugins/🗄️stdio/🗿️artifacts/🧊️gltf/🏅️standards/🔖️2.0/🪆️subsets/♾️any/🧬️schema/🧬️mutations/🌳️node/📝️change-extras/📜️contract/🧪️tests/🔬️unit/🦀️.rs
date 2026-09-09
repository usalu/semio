use crate::schema::mutations::change_node_extra_data as mutation;
use crate::schema::mutations::contract_tests::{assert_laws, decode};
use crate::GltfSnapshot;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    assert_eq!(contract["id"], mutation::ID);
    let vectors = contract["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        let payload: mutation::GltfChangeNodeExtraDataPayload = decode(&vector["mutation"]);
        let base: GltfSnapshot = decode(&vector["base"]);
        let expected: GltfSnapshot = decode(&vector["after"]);
        assert_eq!(mutation::apply(&payload, &base).unwrap(), expected);
        assert_laws(&mutation::ChangeNodeExtraDataMutation::Apply(payload.clone()), &base, &expected);
        let mut invalid = payload.clone();
        invalid.node = base.document.nodes.len().try_into().unwrap();
        assert!(mutation::apply(&invalid, &base).is_err());
        assert!(mutation::apply(&payload, &expected).is_err());
    }
    println!("[DEBUG] change_node_extra_data: {} canonical vectors verified through direct mutations, inverse restoration and the independent JSON oracle.", vectors.len());
}
