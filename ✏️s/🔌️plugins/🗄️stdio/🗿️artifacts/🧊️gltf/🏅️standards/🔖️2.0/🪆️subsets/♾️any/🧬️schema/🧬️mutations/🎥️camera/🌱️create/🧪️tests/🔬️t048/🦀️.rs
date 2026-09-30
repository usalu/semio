//! 🔬️ `create-camera` implementation case `🔬️t048`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎥️camera/🌱️create/🔬️t048/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateCameraMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-camera");
    super::super::component::fixture_corpus_tests::assert_case("🎥️camera/🌱️create/🔬️t048");
}
