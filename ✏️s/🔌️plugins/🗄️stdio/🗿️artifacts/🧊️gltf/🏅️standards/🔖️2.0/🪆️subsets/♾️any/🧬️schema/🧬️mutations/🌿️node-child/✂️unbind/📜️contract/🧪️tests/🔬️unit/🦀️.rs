use crate::schema::mutations::contract_tests::{assert_laws, decode};
use crate::schema::mutations::unbind_node_child as mutation;
use crate::GltfSnapshot;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    assert_eq!(contract["id"], mutation::ID);
    let vectors = contract["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        let payload: mutation::GltfUnbindNodeChildPayload = decode(&vector["mutation"]);
        let base: GltfSnapshot = decode(&vector["base"]);
        let expected: GltfSnapshot = decode(&vector["after"]);
        assert_eq!(mutation::apply(&payload, &base).unwrap(), expected);
        assert_laws(&mutation::UnbindNodeChildMutation::Apply(payload.clone()), &base, &expected);
        let wire = &vector["wire"];
        let encoded: serde_json::Value = serde_json::from_str(&pack::to_json_string(&payload)).unwrap();
        assert_eq!(encoded, serde_json::from_str::<serde_json::Value>(wire["mutation"].as_str().unwrap()).unwrap());
        let malformed = wire["malformedPayload"].as_str().unwrap();
        assert!(pack::from_json_str::<mutation::GltfUnbindNodeChildPayload>(malformed).is_err());
        assert!(serde_json::from_str::<serde_json::Value>(malformed).is_err());
        for key in ["index", "reference"] {
            let rejected = decode(&vector["rejected"][key]);
            assert_eq!(mutation::validate(&rejected, &base).unwrap_err().code, "gltf.mutation.index-out-of-range");
        }
    }
    println!("[DEBUG] unbind_node_child: {} canonical vectors verified through direct mutations, inverse restoration and the independent JSON oracle.", vectors.len());
}
