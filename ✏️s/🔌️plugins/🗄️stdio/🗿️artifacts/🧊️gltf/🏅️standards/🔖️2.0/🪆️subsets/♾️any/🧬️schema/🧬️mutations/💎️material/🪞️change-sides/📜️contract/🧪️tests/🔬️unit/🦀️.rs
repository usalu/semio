use crate::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided as mutation;
use crate::standards::v2_0::subsets::any::schema::mutations::contract_tests::{assert_laws, decode};
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../💎️material/🪞️change-sides/📜️contract/🔣️.json")).unwrap();
    assert_eq!(contract["id"], mutation::ID);
    let vectors = contract["vectors"].as_array().unwrap();
    assert!(!vectors.is_empty());
    for vector in vectors {
        let payload: mutation::GltfChangeMaterialDoubleSidedPayload = decode(&vector["mutation"]);
        assert_eq!(vector["base"]["material"], 0);
        assert_eq!(vector["after"]["material"], 0);
        assert_eq!(vector["undo"]["material"], 0);
        let mut base = GltfSnapshot::default();
        base.document.materials.push(Default::default());
        base.document.materials[0].double_sided = decode(&vector["base"]["doubleSided"]);
        let mut expected = base.clone();
        expected.document.materials[0].double_sided = decode(&vector["after"]["doubleSided"]);
        assert_eq!(base.document.materials[0].double_sided, decode::<bool>(&vector["undo"]["doubleSided"]));
        let direct = protocol::apply_diff_cold(&mutation::plan(&payload, &base).unwrap(), &base).unwrap();
        assert_eq!(direct, expected);
        assert_eq!(mutation::validate(&payload, &direct).unwrap_err().code, "gltf.mutation.no-observable-change");
        assert_laws(&mutation::ChangeMaterialDoubleSidedMutation::Apply(payload.clone()), &base, &expected);
    }
}
