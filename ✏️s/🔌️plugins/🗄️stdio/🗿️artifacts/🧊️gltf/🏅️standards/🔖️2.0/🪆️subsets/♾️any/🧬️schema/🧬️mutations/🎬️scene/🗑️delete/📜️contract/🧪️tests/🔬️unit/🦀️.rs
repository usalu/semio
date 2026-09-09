use crate::schema::mutations::contract_tests::{assert_laws, decode, scene_snapshot};
use crate::schema::mutations::delete_scene as mutation;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    assert_eq!(contract["id"], mutation::ID);
    let vectors = contract["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        let payload: mutation::GltfDeleteScenePayload = decode(&vector["payload"]);
        let base = scene_snapshot(&vector["base"]);
        let expected = scene_snapshot(&vector["after"]);
        assert_eq!(scene_snapshot(&vector["undo"]), base);
        assert_eq!(mutation::apply(&payload, &base).unwrap(), expected);
        assert_laws(&mutation::DeleteSceneMutation::Apply(payload.clone()), &base, &expected);
        for rejection in vector["rejections"].as_array().unwrap() {
            if let Some(payload) = rejection.get("payload") {
                let rejected = decode(payload);
                let rejected_base = rejection.get("base").map(scene_snapshot).unwrap_or_else(|| base.clone());
                assert_eq!(mutation::apply(&rejected, &rejected_base).unwrap_err().code, rejection["code"].as_str().unwrap());
            }
        }
    }
    println!("[DEBUG] delete_scene: {} canonical vectors verified through direct mutations, inverse restoration and the independent JSON oracle.", vectors.len());
}
