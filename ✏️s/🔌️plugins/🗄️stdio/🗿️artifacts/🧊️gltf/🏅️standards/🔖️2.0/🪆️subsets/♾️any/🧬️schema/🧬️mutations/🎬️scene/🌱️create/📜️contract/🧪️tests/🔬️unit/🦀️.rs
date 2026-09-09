use crate::schema::mutations::contract_tests::{assert_laws, decode, scene_snapshot};
use crate::schema::mutations::create_scene as mutation;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    assert_eq!(contract["id"], mutation::ID);
    let vectors = contract["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        let payload: mutation::GltfCreateScenePayload = decode(&vector["payload"]);
        let base = scene_snapshot(&vector["base"]);
        let expected = scene_snapshot(&vector["after"]);
        assert_eq!(scene_snapshot(&vector["undo"]), base);
        assert_eq!(mutation::apply(&payload, &base).unwrap(), expected);
        assert_laws(&mutation::CreateSceneMutation::Apply(payload.clone()), &base, &expected);
        if let Some(malformed) = vector.get("malformedPayload") {
            let text = malformed["encoded"].as_str().unwrap();
            assert!(pack::from_json_str::<mutation::GltfCreateScenePayload>(text).is_err());
            assert!(serde_json::from_str::<serde_json::Value>(text).is_err());
        }
        for key in ["outOfRangePosition", "invalidDefaultReference"] {
            if let Some(rejection) = vector["rejections"].get(key) {
                let rejected = rejection.get("payload").map(decode).unwrap_or_else(|| payload.clone());
                let rejected_base = rejection.get("base").map(scene_snapshot).unwrap_or_else(|| base.clone());
                assert_eq!(mutation::apply(&rejected, &rejected_base).unwrap_err().code, rejection["code"].as_str().unwrap());
            }
        }
    }
    println!("[DEBUG] create_scene: {} canonical vectors verified through direct mutations, inverse restoration and the independent JSON oracle.", vectors.len());
}
