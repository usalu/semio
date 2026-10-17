//! 🪞️ `change-material-double-sided` implementation case `🪞️makes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💎️material/🪞️change-sides/🪞️makes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMaterialDoubleSidedMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-material-double-sided");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("💎️material/🪞️change/🪞️makes");
}

#[semio_framework_async_macros::async_test]
async fn applies_and_rejects_identity() {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.materials.push(Default::default());
    let payload = GltfChangeMaterialDoubleSidedPayload { material: 0, double_sided: true };
    snapshot = protocol::apply_diff(&plan(&payload, &snapshot).unwrap(), &snapshot).unwrap();
    assert!(snapshot.document.materials[0].double_sided);
    assert!(plan(&payload, &snapshot).is_err());
}
