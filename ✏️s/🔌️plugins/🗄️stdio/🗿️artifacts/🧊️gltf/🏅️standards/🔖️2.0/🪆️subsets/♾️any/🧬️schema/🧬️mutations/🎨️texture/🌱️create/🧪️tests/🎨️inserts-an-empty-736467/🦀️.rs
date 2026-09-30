//! 🎨️ `create-texture` implementation case `🎨️inserts-an-empty-736467`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🎨️texture/🌱️create/🎨️inserts-an-empty-736467/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateTextureMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-texture");
    super::super::component::fixture_corpus_tests::assert_case("🎨️texture/🌱️create/🎨️inserts-an-empty-736467");
}
