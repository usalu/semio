//! 🖼️ `delete-image` implementation case `🖼️removes`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🖼️image/🗑️delete/🖼️removes/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<DeleteImageMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "delete-image");
    super::super::component::fixture_corpus_tests::assert_case("🖼️image/🗑️delete/🖼️removes");
}
