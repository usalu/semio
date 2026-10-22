use crate::standards::v2_0::subsets::any::schema::mutations::contract_tests::{assert_laws, decode, scene_snapshot};
use crate::standards::v2_0::subsets::any::schema::mutations::delete_scene as mutation;
use semio_framework_test_fixtures::fixture_text;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🎬️scene/🗑️delete/📜️contract/🔣️.json")).unwrap();
    assert_eq!(contract["id"], mutation::ID);
    let vectors = contract["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        let payload: mutation::GltfDeleteScenePayload = decode(&vector["payload"]);
        let base = scene_snapshot(&vector["base"]);
        let expected = scene_snapshot(&vector["after"]);
        assert_eq!(scene_snapshot(&vector["undo"]), base);
        assert_eq!(protocol::apply_diff_cold(&mutation::plan(&payload, &base).unwrap(), &base).unwrap(), expected);
        assert_laws(&mutation::DeleteSceneMutation::Apply(payload.clone()), &base, &expected);
        for rejection in vector["rejections"].as_array().unwrap() {
            if let Some(payload) = rejection.get("payload") {
                let rejected = decode(payload);
                let rejected_base = rejection.get("base").map(scene_snapshot).unwrap_or_else(|| base.clone());
                assert_eq!(mutation::plan(&rejected, &rejected_base).unwrap_err().code, rejection["code"].as_str().unwrap());
            }
        }
    }
}
