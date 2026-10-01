//! 🎭️ `change-material-alpha-mode` implementation case `🎭️switches`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/💎️material/🌫️change-alpha/🎭️switches/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ChangeMaterialAlphaModeMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-material-alpha-mode");
    super::super::component::fixture_corpus_tests::assert_case("💎️material/🌫️change-alpha/🎭️switches");
}

#[semio_framework_async_macros::async_test]
async fn changes_only_alpha_mode_and_rejects_identity() {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.materials.push(Default::default());
    let payload = GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Mask };
    apply(&mut snapshot, &payload).unwrap();
    assert_eq!(snapshot.document.materials[0].alpha_mode, GltfAlphaMode::Mask);
    assert!(apply(&mut snapshot, &payload).is_err());
}
