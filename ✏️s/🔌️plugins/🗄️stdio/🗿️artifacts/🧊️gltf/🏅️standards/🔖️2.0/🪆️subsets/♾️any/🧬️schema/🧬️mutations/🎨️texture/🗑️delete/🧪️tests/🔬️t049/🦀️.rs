//! 🔬️ `delete-texture` implementation case `🔬️t049`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎨️texture/🗑️delete/🔬️t049/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteTextureMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-texture");
    super::super::component::fixture_corpus_tests::assert_case("🎨️texture/🗑️delete/🔬️t049");
}
