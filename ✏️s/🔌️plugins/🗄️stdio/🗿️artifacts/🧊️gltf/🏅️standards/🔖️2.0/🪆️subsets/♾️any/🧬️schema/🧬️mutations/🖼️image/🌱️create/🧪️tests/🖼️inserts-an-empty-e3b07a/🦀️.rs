//! 🖼️ `create-image` implementation case `🖼️inserts-an-empty-e3b07a`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🖼️image/🌱️create/🖼️inserts-an-empty-e3b07a/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<CreateImageMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "create-image");
    super::super::component::fixture_corpus_tests::assert_case("🖼️image/🌱️create/🖼️inserts-an-empty-e3b07a");
}
