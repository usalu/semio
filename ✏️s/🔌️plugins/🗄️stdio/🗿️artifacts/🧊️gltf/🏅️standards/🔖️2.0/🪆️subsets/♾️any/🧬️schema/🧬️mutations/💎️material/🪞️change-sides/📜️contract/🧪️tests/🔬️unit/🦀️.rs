use crate::schema::mutations::change_material_double_sided as mutation;
use crate::schema::mutations::contract_tests::{assert_laws, decode};
use crate::GltfSnapshot;

#[test]
fn canonical_vectors_execute_direct_mutation_and_codec_laws() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
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
        let mut direct = base.clone();
        mutation::apply(&mut direct, &payload).unwrap();
        assert_eq!(direct, expected);
        assert_eq!(mutation::validate(&payload, &direct).unwrap_err().code, "gltf.mutation.no-observable-change");
        assert_laws(&mutation::ChangeMaterialDoubleSidedMutation::Apply(payload.clone()), &base, &expected);
    }
    println!("[DEBUG] change_material_double_sided: {} canonical vectors verified through direct mutations, inverse restoration and the independent JSON oracle.", vectors.len());
}
