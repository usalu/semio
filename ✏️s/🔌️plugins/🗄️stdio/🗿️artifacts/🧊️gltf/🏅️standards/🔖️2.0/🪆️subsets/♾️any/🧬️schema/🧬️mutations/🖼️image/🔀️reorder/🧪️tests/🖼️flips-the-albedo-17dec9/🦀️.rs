//! 🖼️ `reorder-images` implementation case `🖼️flips`: the committed fixture bundle
//! `♾️any/🧫️fixtures/🧬️mutations/🖼️image/🔀️reorder/🖼️flips/` holds every corpus law, and the leaf keeps its language-neutral semantic identity.
use super::*;

#[test]
fn committed_case_holds_the_corpus_law() {
    assert_eq!(<ReorderImagesMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "reorder-images");
    super::super::component::fixture_corpus_tests::assert_case("🖼️image/🔀️reorder/🖼️flips");
}
