//! 🎭️ `change-material-alpha-mode` implementation case `🎭️switches`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💎️material/🌫️change-alpha/🎭️switches/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use crate::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMaterialAlphaModeMutation as protocol::MutationKind<GltfSnapshot, crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation>>::SEMANTICS.kind, "change-material-alpha-mode");
    crate::standards::v2_0::subsets::any::schema::mutations::fixture_corpus_tests::assert_case("💎️material/🌫️change-alpha/🎭️switches");
}

#[semio_framework_async_macros::async_test]
async fn changes_only_alpha_mode_and_rejects_identity() {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.materials.push(Default::default());
    let payload = GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Mask };
    snapshot = protocol::apply_diff_cold(&plan(&payload, &snapshot).unwrap(), &snapshot).unwrap();
    assert_eq!(snapshot.document.materials[0].alpha_mode, GltfAlphaMode::Mask);
    assert!(plan(&payload, &snapshot).is_err());
}
