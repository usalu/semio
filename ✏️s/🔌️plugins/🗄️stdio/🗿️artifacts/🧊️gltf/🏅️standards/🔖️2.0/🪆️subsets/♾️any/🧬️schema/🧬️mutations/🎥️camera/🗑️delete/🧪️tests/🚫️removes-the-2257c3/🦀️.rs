//! 🚫️ `delete-camera` implementation case `🚫️removes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎥️camera/🗑️delete/🚫️removes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteCameraMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-camera");
    super::super::component::fixture_corpus_tests::assert_case("🎥️camera/🗑️delete/🚫️removes");
}
